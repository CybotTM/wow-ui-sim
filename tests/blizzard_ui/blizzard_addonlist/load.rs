//! Load smoke for `Blizzard_AddOnList`.
//!
//! TOC reference (`Interface/BlizzardUI/Blizzard_AddOnList/
//! Blizzard_AddOnList.toc`):
//!
//! ```text
//! ## Title: Blizzard_AddOnList
//! ## DefaultState: enabled
//! ## Dependencies: Blizzard_SharedXML
//! ## AllowLoad: Both
//! ## SavedVariablesMachine: g_addonCategoriesCollapsed
//! ```

use std::path::PathBuf;

use crate::common::blizzard_addon_harness::{
    with_blizzard_addon_glue_smoke_shape, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{blizzard_ui_dir, recorded_lua_errors};
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

const ROOT: &str = "Blizzard_AddOnList";
const ROOT_TOC_FILE: &str = "Blizzard_AddOnList.toc";
const REQUIRED_DEPS: &[&str] = &["Blizzard_SharedXML"];

#[test]
fn addon_list_loads_with_dependency_closure_and_no_lua_errors() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, loaded| {
        assert_loaded(loaded, ROOT);
        for dependency in REQUIRED_DEPS {
            assert_loaded(loaded, dependency);
        }

        let errors = recorded_lua_errors(env);
        assert!(
            errors.is_empty(),
            "`{ROOT}` dependency-closure load must emit zero recorded Lua errors after the \
             startup-shape harness clears the panel baseline. Got:\n  {}",
            errors.join("\n  ")
        );
    });
}

#[test]
fn addon_list_allowload_both_selects_game_and_glue_branches() {
    assert_toc_allows_game_and_glue_screens();

    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, loaded| {
        assert_loaded(loaded, ROOT);
        assert_in_glue(env, false, "game");
        assert_game_branch(env);
    });

    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, loaded| {
        assert_loaded(loaded, ROOT);
        assert_in_glue(env, true, "glue");
        assert_glue_branch(env);
    });
}

fn assert_loaded(loaded: &[String], addon: &str) {
    assert!(
        loaded.iter().any(|name| name == addon),
        "`{addon}` must appear in the loaded addon set for the closure rooted at `{ROOT}`. \
         Loaded set: {loaded:?}"
    );
}

fn assert_toc_allows_game_and_glue_screens() {
    let toc = load_root_toc();

    assert!(
        toc.allows_screen(ScreenKind::Game),
        "`{ROOT}` has `## AllowLoad: Both`, so the game-screen closure must discover it"
    );
    assert!(
        toc.allows_screen(ScreenKind::CharacterSelect),
        "`{ROOT}` has `## AllowLoad: Both`, so the glue-screen closure must discover it"
    );
}

fn load_root_toc() -> TocFile {
    let toc_path = root_toc_path();
    TocFile::from_file(&toc_path).unwrap_or_else(|err| {
        panic!(
            "TOC at `{}` MUST parse cleanly before the load contract can be checked: {err}",
            toc_path.display()
        )
    })
}

fn root_toc_path() -> PathBuf {
    blizzard_ui_dir().join(ROOT).join(ROOT_TOC_FILE)
}

fn assert_in_glue(env: &wow_ui_sim::lua_api::WowLuaEnv, expected: bool, screen: &str) {
    let in_glue: bool = env
        .eval("return InGlue()")
        .unwrap_or_else(|err| panic!("InGlue probe must run cleanly in {screen} screen: {err}"));

    assert_eq!(
        in_glue, expected,
        "test harness must execute the {screen} branch with the expected InGlue() value"
    );
}

fn assert_game_branch(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (has_panel_window, tooltip_is_game_tooltip): (bool, bool) = env
        .eval(
            r#"
            local panel = UIPanelWindows and UIPanelWindows["AddonList"]
            return panel ~= nil
                and panel.area == "center"
                and panel.pushable == 0
                and panel.whileDead == 1,
                AddonTooltip == GameTooltip
            "#,
        )
        .expect("game branch globals must be probeable after loading Blizzard_AddOnList");

    assert!(
        has_panel_window,
        "`{ROOT}` game branch must register `UIPanelWindows[\"AddonList\"]`"
    );
    assert!(
        tooltip_is_game_tooltip,
        "`{ROOT}` game branch must assign `AddonTooltip = GameTooltip`"
    );
}

fn assert_glue_branch(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (addon_dialog_name, tooltip_is_glue_tooltip): (String, bool) = env
        .eval(
            r#"
            return AddonDialog:GetName(), AddonTooltip == GlueTooltip
            "#,
        )
        .expect("glue branch globals must be probeable after loading Blizzard_AddOnList");

    assert_eq!(
        addon_dialog_name, "AddonDialog",
        "`{ROOT}` glue branch must register the `AddonDialog` frame"
    );
    assert!(
        tooltip_is_glue_tooltip,
        "`{ROOT}` glue branch must assign `AddonTooltip = GlueTooltip`"
    );
}
