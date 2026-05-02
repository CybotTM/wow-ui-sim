//! Load smoke for `Blizzard_ActionStatus`.
//!
//! TOC reference (`Interface/BlizzardUI/Blizzard_ActionStatus/
//! Blizzard_ActionStatus.toc`):
//!
//! ```text
//! ## Title: Blizzard_ActionStatus
//! ## DefaultState: enabled
//! ## OptionalDep: Blizzard_FrameXML, Blizzard_GlueXML
//! ## AllowLoad: Both
//! ```

use crate::common::blizzard_addon_harness::{
    with_blizzard_addon_glue_smoke_shape, with_blizzard_addon_smoke_shape,
};
use crate::common::panel_fixtures::{blizzard_ui_dir, recorded_lua_errors};
use std::path::PathBuf;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

const ROOT: &str = "Blizzard_ActionStatus";
const ROOT_TOC_FILE: &str = "Blizzard_ActionStatus.toc";
const RAW_OPTIONAL_DEP_LINE: &str = "## OptionalDep: Blizzard_FrameXML, Blizzard_GlueXML";

#[test]
fn action_status_loads_with_no_required_deps_and_no_lua_errors() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
        assert_loaded(loaded, "game");

        assert_toc_declares_no_required_deps_and_only_raw_optional_deps();

        assert_no_lua_errors(env, "game");
    });
}

#[test]
fn action_status_allowload_both_loads_in_game_and_glue_screens() {
    assert_toc_allows_game_and_glue_screens();

    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
        assert_loaded(loaded, "game");
        assert_in_glue(env, false, "game");
        assert_no_lua_errors(env, "game");
    });

    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, loaded| {
        assert_loaded(loaded, "glue");
        assert_in_glue(env, true, "glue");
        assert_no_lua_errors(env, "glue");
    });
}

fn assert_toc_declares_no_required_deps_and_only_raw_optional_deps() {
    let toc = load_root_toc();

    assert!(
        toc.dependencies().is_empty(),
        "`{ROOT}` must declare no required dependencies. The TOC only lists the singular \
         optional dependency line `{RAW_OPTIONAL_DEP_LINE}`."
    );

    let toc_path = root_toc_path();
    let raw_toc = std::fs::read_to_string(&toc_path).unwrap_or_else(|err| {
        panic!(
            "TOC at `{}` MUST be readable for raw OptionalDep verification: {err}",
            toc_path.display()
        )
    });

    assert!(
        raw_toc.contains(RAW_OPTIONAL_DEP_LINE),
        "`{ROOT}` must keep its only listed dependencies optional via the raw singular TOC line \
         `{RAW_OPTIONAL_DEP_LINE}`. Raw TOC:\n{raw_toc}"
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

fn assert_loaded(loaded: &[String], screen: &str) {
    assert!(
        loaded.iter().any(|name| name == ROOT),
        "`{ROOT}` must load in the {screen} closure. Loaded set: {loaded:?}"
    );
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

fn assert_no_lua_errors(env: &wow_ui_sim::lua_api::WowLuaEnv, screen: &str) {
    let errors = recorded_lua_errors(env);
    assert!(
        errors.is_empty(),
        "`{ROOT}` must emit zero recorded Lua errors in the {screen} closure. Got:\n  {}",
        errors.join("\n  ")
    );
}
