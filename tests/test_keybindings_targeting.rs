//! Integration tests for keybinding dispatch — targeting tests.
//!
//! Covers TargetFrame visibility, F1–F6 party/enemy targeting keybinds.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::globals::global_frames;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::call_global_if_present(env, "RequestTimePlayed");
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
        "PLAYER_LEAVING_WORLD",
    ] {
        let _ = env.fire_event(event);
    }
}

fn frame_is_shown(env: &WowLuaEnv, frame_name: &str) -> bool {
    let code = format!("return {frame_name} ~= nil and {frame_name}:IsShown() == true");
    env.eval::<bool>(&code).unwrap_or(false)
}

/// Check whether a global frame exists.
fn frame_exists(env: &WowLuaEnv, frame_name: &str) -> bool {
    let code = format!("return {frame_name} ~= nil");
    env.eval::<bool>(&code).unwrap_or(false)
}

fn install_test_error_handler(env: &WowLuaEnv) {
    common::install_error_collector(env, "__test_errors");
}

fn drain_test_errors(env: &WowLuaEnv) -> Vec<String> {
    common::drain_string_table(env, "__test_errors")
}

/// Create environment with ALL Blizzard addons (including Blizzard_UnitFrame).
fn setup_full_env() -> common::LockedEnv {
    common::lock_env(|| {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        let ui = blizzard_ui_dir();
        {
            let mut state = env.state().borrow_mut();
            state.addon_base_paths = vec![ui.clone()];
        }

        let addons = discover_blizzard_addons(&ui);
        for (name, toc_path) in &addons {
            if let Err(e) = load_addon(&env.loader_env(), toc_path) {
                eprintln!("[load {name}] FAILED: {e}");
            }
        }
        env.apply_post_load_workarounds();
        fire_startup_events(&env);
        env.apply_post_event_workarounds();
        let _ = global_frames::hide_runtime_hidden_frames(&*env.rilua());
        env
    })
}

// ── Target frame visibility tests (full addon load including Blizzard_UnitFrame) ──

#[test]
fn target_frame_shown_after_targeting() {
    test_timeout! {
        let env = setup_full_env();
        install_test_error_handler(&env);

        assert!(
            frame_exists(&env, "TargetFrame"),
            "TargetFrame should exist after full addon load"
        );

        // TargetFrame starts hidden (hide_runtime_hidden_frames) or via startup;
        // ensure it's hidden before testing
        if frame_is_shown(&env, "TargetFrame") {
            env.exec("TargetFrame:Hide()").unwrap();
        }

        // F1 = target self → TargetFrame should show
        env.send_key_press("F1", None).expect("F1 keybind failed");
        let _ = drain_test_errors(&env); // non-fatal errors from TargetFrame:Update()
        assert!(
            frame_is_shown(&env, "TargetFrame"),
            "TargetFrame should be shown after targeting self with F1"
        );

        // ESCAPE = clear target → TargetFrame should hide
        env.send_key_press("ESCAPE", None).expect("ESCAPE keybind failed");
        let _ = drain_test_errors(&env);
        assert!(
            !frame_is_shown(&env, "TargetFrame"),
            "TargetFrame should be hidden after clearing target with ESCAPE"
        );
    }
}

#[test]
fn target_frame_shown_for_enemy() {
    test_timeout! {
        let env = setup_full_env();
        install_test_error_handler(&env);

        if frame_is_shown(&env, "TargetFrame") {
            env.exec("TargetFrame:Hide()").unwrap();
        }

        // TAB = target nearest enemy → TargetFrame should show
        env.send_key_press("TAB", None).expect("TAB keybind failed");
        let _ = drain_test_errors(&env); // non-fatal errors from TargetFrame:Update()
        assert!(
            frame_is_shown(&env, "TargetFrame"),
            "TargetFrame should be shown after targeting enemy with TAB"
        );
    }
}

// ── F2–F5 → TargetUnit('party1')–('party4') ─────────────────────────────

#[test]
fn keybind_f2_targets_party1() {
    test_timeout! {
        let env = setup_full_env();
        install_test_error_handler(&env);
        if frame_is_shown(&env, "TargetFrame") {
            env.exec("TargetFrame:Hide()").unwrap();
        }
        env.send_key_press("F2", None).expect("F2 keybind failed");
        let _ = drain_test_errors(&env);
        assert!(
            frame_is_shown(&env, "TargetFrame"),
            "TargetFrame should be shown after targeting party1 with F2"
        );
    }
}

#[test]
fn keybind_f3_targets_party2() {
    test_timeout! {
        let env = setup_full_env();
        install_test_error_handler(&env);
        if frame_is_shown(&env, "TargetFrame") {
            env.exec("TargetFrame:Hide()").unwrap();
        }
        env.send_key_press("F3", None).expect("F3 keybind failed");
        let _ = drain_test_errors(&env);
        assert!(
            frame_is_shown(&env, "TargetFrame"),
            "TargetFrame should be shown after targeting party2 with F3"
        );
    }
}

#[test]
fn keybind_f4_targets_party3() {
    test_timeout! {
        let env = setup_full_env();
        install_test_error_handler(&env);
        if frame_is_shown(&env, "TargetFrame") {
            env.exec("TargetFrame:Hide()").unwrap();
        }
        env.send_key_press("F4", None).expect("F4 keybind failed");
        let _ = drain_test_errors(&env);
        assert!(
            frame_is_shown(&env, "TargetFrame"),
            "TargetFrame should be shown after targeting party3 with F4"
        );
    }
}

#[test]
fn keybind_f5_targets_party4() {
    test_timeout! {
        let env = setup_full_env();
        install_test_error_handler(&env);
        if frame_is_shown(&env, "TargetFrame") {
            env.exec("TargetFrame:Hide()").unwrap();
        }
        env.send_key_press("F5", None).expect("F5 keybind failed");
        let _ = drain_test_errors(&env);
        assert!(
            frame_is_shown(&env, "TargetFrame"),
            "TargetFrame should be shown after targeting party4 with F5"
        );
    }
}

// ── F6 → TargetUnit('enemy1') ────────────────────────────────────────────

#[test]
fn keybind_f6_targets_enemy() {
    test_timeout! {
        let env = setup_full_env();
        install_test_error_handler(&env);
        if frame_is_shown(&env, "TargetFrame") {
            env.exec("TargetFrame:Hide()").unwrap();
        }
        env.send_key_press("F6", None).expect("F6 keybind failed");
        let _ = drain_test_errors(&env);
        assert!(
            frame_is_shown(&env, "TargetFrame"),
            "TargetFrame should be shown after targeting enemy with F6"
        );
    }
}
