mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::{
    fire_one_on_update_tick, fire_startup_events_for_screen, process_pending_timers,
};
use wow_ui_sim::toc::TocFile;

const TEST_ADDONS: &[&str] = &["Wowless", "WowlessData"];

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn addons_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/AddOns")
}

fn scan_game_addons() -> Vec<(String, PathBuf)> {
    let mut addons = Vec::new();
    let base_path = addons_dir();
    let Ok(entries) = std::fs::read_dir(base_path) else {
        return addons;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name.starts_with('.') || TEST_ADDONS.contains(&name) {
            continue;
        }
        let Some(toc_path) = find_toc_file(&path) else {
            continue;
        };
        let Ok(toc) = TocFile::from_file(&toc_path) else {
            continue;
        };
        if toc.allows_screen(ScreenKind::Game)
            && !toc.is_ptr_only()
            && !toc.is_game_type_restricted()
        {
            addons.push((name.to_string(), toc_path));
        }
    }

    wow_ui_sim::loader::sort_addons_by_dependencies(&mut addons);
    addons
}

fn load_game_screen() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![
            PathBuf::from("./Interface/BlizzardUI"),
            PathBuf::from("./Interface/AddOns"),
        ];
    }
    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let blizzard = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    for (name, toc_path) in &blizzard {
        if let Err(err) = load_addon(&env.loader_env(), toc_path) {
            panic!("[load {name}] FAILED: {err}");
        }
    }

    let addons = scan_game_addons();
    for (name, toc_path) in &addons {
        if let Err(err) = load_addon(&env.loader_env(), toc_path) {
            panic!("[addon {name}] FAILED: {err}");
        }
    }

    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, ScreenKind::Game);
    env
}

#[test]
fn game_boot_has_no_unexpected_lua_errors() {
    test_timeout! {
        let env = load_game_screen();

        let errors = env.state().borrow().lua_errors.clone();
        assert!(
            errors.is_empty(),
            "game boot still has lua errors: {errors:#?}"
        );
    }
}

#[test]
fn game_boot_lua_errors_pipeline_finishes() {
    test_timeout! {
        let env = load_game_screen();
        env.apply_post_event_workarounds();
        env.state().borrow_mut().widgets.rebuild_anchor_index();
        process_pending_timers(&env);
        fire_one_on_update_tick(&env);
        let _ = wow_ui_sim::lua_api::globals::global_frames::hide_runtime_hidden_frames(&*env.rilua());
        let errors = env.state().borrow().lua_errors.clone();
        assert!(
            errors.is_empty(),
            "game boot settle pipeline still has lua errors: {errors:#?}"
        );
    }
}
