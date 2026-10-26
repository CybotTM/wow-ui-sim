//! Era/Anniversary stock startup consumers, without third-party addons or saved variables.
#![cfg(any(feature = "client-era", feature = "client-anniversary"))]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn load_stock_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    let ui = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    crate::common::load_profile_framexml(&env);
    for (name, toc) in
        wow_ui_sim::loader::discover_blizzard_addons_for_screen(&ui, ScreenKind::Game)
    {
        wow_ui_sim::loader::load_addon(&env.loader_env(), &toc)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
    }
    env.apply_post_load_workarounds();
    env
}

#[test]
fn classic_original_startup_timers_complete_without_lua_errors() {
    let env = load_stock_ui();
    wow_ui_sim::startup::fire_startup_events_for_screen(&env, ScreenKind::Game);
    let before = env.state().borrow().lua_errors.len();
    let fired = env.process_timers().unwrap();
    assert!(fired > 0, "the original startup must schedule callbacks");
    let errors = env.state().borrow().lua_errors[before..].to_vec();
    assert!(
        errors.is_empty(),
        "Original startup timer errors: {errors:#?}"
    );
}
