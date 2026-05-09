//! Regression coverage for Encounter Journal tab interactions.
//!
//! Ensures known nil-shape regressions do not reappear when opening/clicking
//! Encounter Journal tabs from the micro menu.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

fn install_test_error_handler(env: &WowLuaEnv) {
    common::install_error_collector(env, "__test_errors");
}

fn drain_test_errors(env: &WowLuaEnv) -> Vec<String> {
    common::drain_string_table(env, "__test_errors")
}

fn setup_full_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    for (name, toc_path) in &addons {
        if let Err(e) = load_addon(&env.loader_env(), toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }

    env.apply_post_load_workarounds();
    install_test_error_handler(&env);
    fire_startup_events(&env);
    drain_test_errors(&env);
    env
}

fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    let _ = env.fire_edit_mode_layouts_updated();
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }
    let _ = env.fire_on_update(0.016);
    let _ = env.process_timers();
}

fn click_named(env: &WowLuaEnv, name: &str) -> Vec<String> {
    let script = format!(
        r#"
        local btn = _G["{name}"]
        if btn then
            local onclick = btn:GetScript("OnClick")
            if onclick then
                pcall(onclick, btn, "LeftButton", false)
            end
        end
    "#
    );
    let mut errors = Vec::new();
    if let Err(err) = env.exec(&script) {
        errors.push(format!("[{name}] {err}"));
    }
    errors.extend(
        drain_test_errors(env)
            .into_iter()
            .map(|err| format!("[{name}] {err}")),
    );
    errors
}

#[test]
fn encounter_journal_tabs_do_not_raise_known_lua_errors() {
    test_timeout! {
        let env = setup_full_ui();

        let mut errors = Vec::new();
        for frame_name in [
            "EJMicroButton",
            "EncounterJournalMonthlyActivitiesTab",
            "EncounterJournalLootJournalTab",
            "EncounterJournalJourneysTab",
            "EncounterJournalLootJournalTab",
        ] {
            errors.extend(click_named(&env, frame_name));
            let _ = env.fire_on_update(0.016);
            let _ = env.process_timers();
            errors.extend(drain_test_errors(&env));
        }

        let signatures = [
            "Blizzard_MonthlyActivities.lua:716",
            "Blizzard_MonthlyActivities.lua:888",
            "Blizzard_EncounterJournal.lua:2789",
            "Blizzard_EncounterJournal.lua:2882",
            "Blizzard_EncounterJournal.lua:2920",
        ];

        let matched: Vec<String> = errors
            .iter()
            .filter(|err| signatures.iter().any(|sig| err.contains(sig)))
            .cloned()
            .collect();

        assert!(
            matched.is_empty(),
            "Encounter Journal regressions detected:\n  {}",
            matched.join("\n  ")
        );
    }
}
