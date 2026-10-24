//! Retail/PTR original-consumer contract; Forever has no completion input yet.
//! The epoch gate is required by SimState.pvp_catalog, not a passing alternate path.
#![cfg(feature = "retail-12-0-0")]

use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_original_match_consumer() -> WowLuaEnv {
    assert!(matches!(ACTIVE, ClientProfile::Retail | ClientProfile::Ptr));
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let (env, loaded) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &ui,
        &["Blizzard_PVPMatch"],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_PVPMatch"));
    eprintln!("CF-2 selected {ACTIVE:?} consumer cache: {}", ui.display());
    env.exec(
        r#"
        assert(PVPMatchResults, 'selected XML must instantiate the original results frame')
        local handler = PVPMatchResults:GetScript('OnEvent')
        assert(handler == PVPMatchResultsMixin.OnEvent and handler == PVPMatchResults.OnEvent,
            'selected XML must retain the installed original OnEvent method')
        local info = debug.getinfo(handler, 'S')
        local suffix = 'Blizzard_PVPMatch/PVPMatchResults.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must invoke the original cached event consumer')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)
        "#,
    )
    .expect("selected XML and original match event provenance");
    env
}

fn assert_enter_world_registration(env: &WowLuaEnv, expected: bool) {
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec("PVPMatchResults:UnregisterAllEvents()")
        .expect("reset real subscriptions before every snapshot");
    // Call the unchanged closure directly: dispatch must not swallow the RED error.
    env.exec("PVPMatchResults:GetScript('OnEvent')(PVPMatchResults, 'PLAYER_ENTERING_WORLD')")
        .expect("original entering-world consumer must read supplied completion state");
    for event in [
        "PVP_MATCH_COMPLETE",
        "TREASURE_PICKER_CACHE_FLUSH",
        "LFG_ROLE_CHECK_DECLINED",
        "POST_MATCH_ITEM_REWARD_UPDATE",
        "POST_MATCH_CURRENCY_REWARD_UPDATE",
        "LFG_ROLE_CHECK_SHOW",
        "LFG_READY_CHECK_DECLINED",
        "LFG_READY_CHECK_SHOW",
    ] {
        let registered: bool = env
            .eval(&format!(
                "return PVPMatchResults:IsEventRegistered('{event}')"
            ))
            .unwrap();
        assert_eq!(registered, expected, "{event} subscription");
    }
    let shown: bool = env.eval("return PVPMatchResults:IsShown()").unwrap();
    assert!(
        !shown,
        "entering world subscribes without displaying results"
    );
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

#[test]
fn original_match_results_enter_world_tracks_inactive_active_and_completed_match() {
    let env = load_original_match_consumer();
    // One selector must reject both constants: true fails snapshot 1; false fails 3.
    for (active, completed, expected) in [
        (false, false, false),
        (true, false, true),
        (false, true, true),
        (false, false, false),
    ] {
        {
            let mut sim = env.state().borrow_mut();
            sim.pvp_match_active = active;
            sim.pvp_catalog.match_completed = completed;
        }
        assert_enter_world_registration(&env, expected);
    }
}
