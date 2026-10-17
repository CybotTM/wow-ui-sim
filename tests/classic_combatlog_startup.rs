//! R3-unmasked CombatLog history boundary, exercised through original startup.
#![cfg(any(feature = "client-era", feature = "client-anniversary"))]

use wow_ui_sim::loader::{discover_blizzard_startup_addons_for_screen, load_startup_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;

#[test]
fn classic_a_original_login_refilter_has_no_fabricated_history_entry() {
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    wow_ui_sim::xml::register_intrinsic_templates();
    for addon in discover_blizzard_startup_addons_for_screen(&ui, ScreenKind::Game) {
        load_startup_addon(&env.loader_env(), &addon.toc_path, addon.kind, None)
            .unwrap_or_else(|error| panic!("load {}: {error}", addon.name));
    }
    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, ScreenKind::Game);
    env.exec("assert(C_AddOns.IsAddOnLoaded('Blizzard_CombatLog')); assert(type(Blizzard_CombatLog_RefilterUpdate) == 'function')")
        .expect("original UIParent PLAYER_LOGIN loads the combat log");
    let history_errors: Vec<_> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|error| error.contains("string.sub") || error.contains("RefilterUpdate"))
        .cloned()
        .collect();
    assert!(history_errors.is_empty(), "{history_errors:?}");
    env.exec(
        r#"
        assert(C_CombatLog.GetEntryCount() == 0)
        assert(C_CombatLog.GetCurrentEntryInfo() == nil)
        assert(C_CombatLog.SeekToNewestEntry() == false)
        assert(C_CombatLog.SeekToPreviousEntry() == false)
        C_CombatLog._state.currentEntry = 37
        assert(C_CombatLog.GetCurrentEntryInfo() == nil)
        Blizzard_CombatLog_Refilter()
        assert(CombatLogUpdateFrame.refiltering == false)
        assert(CombatLogUpdateFrame:GetScript('OnUpdate') == nil)
        assert(not CombatLogQuickButtonFrameProgressBar:IsShown())
        "#,
    )
    .expect("empty Classic history never fabricates a valid event from a cursor; populated replay is policy-blocked");
}
