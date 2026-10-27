//! Specialization events reach the selected Forever micro-menu consumer.
#![cfg(feature = "client-wowforever")]

use wow_ui_sim::loader::{
    StartupAddonLoadKind, discover_blizzard_startup_addons_for_screen, load_startup_addon,
};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn load_game_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("fresh environment");
    let ui = wow_ui_sim::client_profile::blizzard_ui_addons_dir();
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    wow_ui_sim::xml::register_intrinsic_templates();
    if let Some(toc) = wow_ui_sim::client_profile::blizzard_ui_framexml_toc() {
        load_startup_addon(&env.loader_env(), &toc, StartupAddonLoadKind::Full, None)
            .expect("load FrameXML");
        env.fire_event_with_args("ADDON_LOADED", &[env.lua_string("FrameXML")])
            .expect("dispatch FrameXML load");
    }
    for addon in discover_blizzard_startup_addons_for_screen(&ui, ScreenKind::Game) {
        load_startup_addon(&env.loader_env(), &addon.toc_path, addon.kind, None)
            .expect("load startup addon");
        if addon.kind == StartupAddonLoadKind::Full {
            env.fire_event_with_args("ADDON_LOADED", &[env.lua_string(&addon.name)])
                .expect("dispatch addon load");
        }
        if addon.name == "Blizzard_EnvironmentCleanup" {
            env.restore_post_cleanup_globals();
        }
    }
    env.sync_string_metatable_to_global_string();
    env.sync_addon_names_to_lua();
    env.apply_post_load_workarounds();
    wow_ui_sim::startup::fire_startup_events(&env);
    wow_ui_sim::startup::process_pending_timers(&env);
    wow_ui_sim::startup::fire_one_on_update_tick(&env);
    env
}

fn check_desired(desired: bool) {
    let env = load_game_ui();
    env.state().borrow_mut().war_mode.desired = desired;
    env.exec(
        r#"
        assert(PlayerSpellsMicroButton and PlayerSpellsMicroButton:IsEventRegistered("PLAYER_SPECIALIZATION_CHANGED"))
        local source = debug.getinfo(PlayerSpellsMicroButton.GetAnyPvpTalentAlert, "S").source
        assert(string.find(source, "MainMenuBarMicroButtons.lua", 1, true), source)
        observedConsumerCalls = 0
        hooksecurefunc(PlayerSpellsMicroButton, "GetAnyPvpTalentAlert", function()
            observedConsumerCalls = observedConsumerCalls + 1
        end)
        observedBattlefieldQueries = 0
        hooksecurefunc(PVPUtil, "IsInActiveBattlefield", function()
            observedBattlefieldQueries = observedBattlefieldQueries + 1
        end)
        "#,
    )
    .expect("observe the original selected micro-menu consumer");
    env.exec("A_Admin.SetSpec(2)")
        .expect("change specialization");
    crate::common::panel_fixtures::clear_recorded_lua_errors(&env);
    env.fire_event_with_args("PLAYER_SPECIALIZATION_CHANGED", &[env.lua_string("player")])
        .expect("dispatch specialization change");
    env.fire_on_update(0.016)
        .expect("evaluate dirty micro buttons");
    let errors = crate::common::panel_fixtures::recorded_lua_errors(&env);
    assert!(
        errors.is_empty(),
        "specialization refresh errors: {errors:?}"
    );
    let calls: i32 = env
        .eval("return observedConsumerCalls")
        .expect("consumer count");
    assert!(
        calls > 0,
        "specialization refresh must reach the original consumer"
    );
    let queries: i32 = env
        .eval("return observedBattlefieldQueries")
        .expect("query count");
    if desired {
        assert_eq!(queries, 0, "desired mode short-circuits battlefield lookup");
    } else {
        assert!(queries > 0, "disabled mode reaches battlefield lookup");
    }
}

#[test]
fn forever_specialization_refresh_war_mode_disabled() {
    check_desired(false);
}

#[test]
fn forever_specialization_refresh_war_mode_enabled() {
    check_desired(true);
}
