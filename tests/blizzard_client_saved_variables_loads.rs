#![cfg(feature = "client-retail")]
use std::path::PathBuf;

use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

fn client_saved_variables_toc() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_ClientSavedVariables/Blizzard_ClientSavedVariables.toc")
}

#[test]
fn blizzard_client_saved_variables_appears_in_game_discovery() {
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);

    let discovered = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_ClientSavedVariables");
    assert!(
        discovered,
        "Blizzard_ClientSavedVariables should appear in Game-screen discovery (non-LOD addon)"
    );
}

#[test]
fn blizzard_client_saved_variables_toc_declares_per_character_globals() {
    let toc =
        TocFile::from_file(&client_saved_variables_toc()).expect("TOC should parse from file");

    let mut declared = toc.saved_variables_per_character();
    declared.sort();
    assert_eq!(
        declared,
        vec![
            "CHANNELPULLOUT_FADEFRAMES".to_string(),
            "DISPLAYED_COMMUNITIES_INVITATIONS".to_string(),
        ],
        "Blizzard_ClientSavedVariables TOC should declare exactly the two per-character globals \
         consumed by Blizzard_Communities and Blizzard_MicroMenu"
    );

    assert!(
        toc.saved_variables().is_empty(),
        "Blizzard_ClientSavedVariables should declare no shared SavedVariables"
    );
}

#[test]
fn blizzard_client_saved_variables_loads_without_errors() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }
    wow_ui_sim::xml::register_intrinsic_templates();

    load_addon(&env.loader_env(), &client_saved_variables_toc())
        .expect("Blizzard_ClientSavedVariables should load via Rust loader");

    let load_errors: Vec<String> = env.state().borrow().lua_errors.clone();
    assert!(
        load_errors.is_empty(),
        "Blizzard_ClientSavedVariables emitted Lua errors during load:\n  {}",
        load_errors.join("\n  ")
    );
}
