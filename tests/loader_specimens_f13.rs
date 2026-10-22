//! Additive, test-owned loader contracts. These do not replace native consumers.

use std::path::Path;
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

#[test]
fn empty_lod_toc_registers_only_after_explicit_load_without_errors() {
    let root = tempfile::tempdir().expect("create specimen directory");
    let dir = root.path().join("F13Empty");
    std::fs::create_dir(&dir).expect("create addon directory");
    let path = dir.join("F13Empty.toc");
    std::fs::write(&path, "## Title: F13 Empty
## LoadOnDemand: 1
")
        .expect("write empty TOC specimen");
    let toc = TocFile::from_file(&path).expect("parse specimen");
    assert!(toc.files.is_empty());
    assert!(toc.is_load_on_demand());
    let env = WowLuaEnv::new().expect("create environment");
    env.state().borrow_mut().addon_base_paths = vec![root.path().to_path_buf()];
    let before: bool = env.eval("return C_AddOns.IsAddOnLoaded('F13Empty')").unwrap();
    assert!(!before);
    load_addon(&env.loader_env(), &path).expect("load empty specimen");
    let after: bool = env.eval("return C_AddOns.IsAddOnLoaded('F13Empty')").unwrap();
    assert!(after);
    assert!(env.state().borrow().lua_errors.is_empty());
}

#[test]
fn both_scope_empty_addon_is_discovered_on_game_and_glue() {
    let root = tempfile::tempdir().expect("create specimen directory");
    let dir = root.path().join("Blizzard_F13Scope");
    std::fs::create_dir(&dir).expect("create addon directory");
    std::fs::write(
        dir.join("Blizzard_F13Scope.toc"),
        "## Title: F13 Scope
## AllowLoad: Both
",
    )
    .expect("write scope specimen");
    for screen in [ScreenKind::Game, ScreenKind::Login, ScreenKind::CharacterSelect] {
        let discovered = discover_blizzard_addons_for_screen(root.path(), screen);
        assert_eq!(discovered.len(), 1);
        assert_eq!(discovered[0].0, "Blizzard_F13Scope");
        let env = WowLuaEnv::new().expect("create environment");
        env.set_screen_mode(screen);
        load_addon(&env.loader_env(), &discovered[0].1).expect("load scope specimen");
        let loaded: bool = env
            .eval("return C_AddOns.IsAddOnLoaded('Blizzard_F13Scope')")
            .unwrap();
        assert!(loaded);
        assert!(env.state().borrow().lua_errors.is_empty());
    }
}

#[test]
fn repeated_singular_dependencies_preserve_order() {
    let toc = TocFile::parse(
        Path::new("F13Dependencies"),
        "## Dep: First
## Dep: Second
",
    );
    assert_eq!(toc.dependencies(), vec!["First".to_string(), "Second".to_string()]);
}

#[test]
fn global_only_file_annotation_is_not_a_global_fenv_override() {
    let toc = TocFile::parse(
        Path::new("F13Environment"),
        "Diagnostic.lua [AllowLoadEnvironment Global]
",
    );
    assert_eq!(toc.files, [Path::new("Diagnostic.lua")]);
    assert_eq!(toc.file_use_secure_env(0), None);
    assert_eq!(toc.file_allow_load_environment(0), Some(false));
}
