//! Original match consumers; completion state exists only in Retail/PTR builds.
#![cfg(any(feature = "client-retail", feature = "client-ptr",))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn load_original_consumer(addon: &str) -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let (env, loaded) =
        crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(&ui, &[addon], &[]);
    assert!(loaded.iter().any(|name| name == addon));
    eprintln!("CF-2 original consumer cache: {}", ui.display());
    env
}

fn create_match_event_fixture(env: &WowLuaEnv) {
    env.exec(
        r#"
        local info = debug.getinfo(PVPMatchResultsMixin.OnEvent, 'S')
        local suffix = 'Blizzard_PVPMatch/PVPMatchResults.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must invoke the original cached event consumer')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)
        -- Real event-registration output fixture; no substituted handler or API.
        Cf2MatchResults = CreateFrame('Frame', nil, UIParent)
        Mixin(Cf2MatchResults, PVPMatchResultsMixin)
        Cf2MatchResults:SetScript('OnEvent', Cf2MatchResults.OnEvent)
        Cf2MatchResults:RegisterEvent('PLAYER_ENTERING_WORLD')
        "#,
    )
    .expect("original match event provenance and frame fixture");
}

fn assert_enter_world_registration(env: &WowLuaEnv, expected: bool) {
    let errors_before = env.state().borrow().lua_errors.len();
    // Invoke the unchanged original handler directly so swallowed dispatch errors
    // cannot masquerade as a successful condition branch.
    env.exec("Cf2MatchResults:OnEvent('PLAYER_ENTERING_WORLD')")
        .expect("original entering-world condition must consume supplied match state");
    let registered: bool = env
        .eval("return Cf2MatchResults:IsEventRegistered('PVP_MATCH_COMPLETE')")
        .unwrap();
    assert_eq!(registered, expected, "post-match event subscription");
    let rewards_registered: bool = env
        .eval("return Cf2MatchResults:IsEventRegistered('POST_MATCH_ITEM_REWARD_UPDATE')")
        .unwrap();
    assert_eq!(rewards_registered, expected, "reward event subscription");
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

#[test]
fn original_match_results_enter_world_tracks_inactive_active_and_completed_match() {
    let env = load_original_consumer("Blizzard_PVPMatch");
    {
        let mut sim = env.state().borrow_mut();
        sim.pvp_match_active = false;
        sim.pvp_catalog.match_completed = false;
    }
    create_match_event_fixture(&env);
    assert_enter_world_registration(&env, false);
    env.exec("Cf2MatchResults:UnregisterAllEvents()")
        .expect("reset real registration output before active snapshot");
    {
        let mut sim = env.state().borrow_mut();
        sim.pvp_match_active = true;
        sim.pvp_catalog.match_completed = false;
    }
    assert_enter_world_registration(&env, true);
    env.exec("Cf2MatchResults:UnregisterAllEvents()")
        .expect("reset real registration output between match snapshots");
    {
        let mut sim = env.state().borrow_mut();
        sim.pvp_match_active = false;
        sim.pvp_catalog.match_completed = true;
    }
    assert_enter_world_registration(&env, true);
}
