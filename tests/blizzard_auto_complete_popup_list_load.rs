use crate::common;

use std::path::PathBuf;

use common::blizzard_addon_harness::{
    with_blizzard_addon_glue_smoke_shape, with_blizzard_addon_smoke_shape,
};
use common::panel_fixtures::blizzard_ui_dir;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const ROOT_TOC_FILE: &str = "Blizzard_AutoCompletePopupList.toc";

#[test]
fn blizzard_auto_complete_popup_list_lod_loads_without_ingestion_errors() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AutoCompletePopupList")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let is_loaded: bool = env
                    .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_AutoCompletePopupList")"#)
                    .expect("C_AddOns.IsAddOnLoaded should return");
                assert!(
                    is_loaded,
                    "`{ROOT}` should be marked loaded after LoadAddOn"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors during load:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

#[test]
fn blizzard_auto_complete_popup_list_allowload_both_loads_in_game_and_glue_scopes() {
    assert_toc_allows_game_and_glue_scopes();

    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert_loaded_cleanly_in_scope(env, loaded, "game");
            });

            with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert_loaded_cleanly_in_scope(env, loaded, "glue");
            });
        });
    });
}

fn assert_toc_allows_game_and_glue_scopes() {
    let toc = load_root_toc();
    assert!(
        toc.allows_screen(ScreenKind::Game),
        "`{ROOT}` has `## AllowLoad: Both`, so game-scope discovery must include it"
    );
    assert!(
        toc.allows_screen(ScreenKind::CharacterSelect),
        "`{ROOT}` has `## AllowLoad: Both`, so glue-scope discovery must include it"
    );
}

fn load_root_toc() -> TocFile {
    let toc_path = root_toc_path();
    TocFile::from_file(&toc_path).unwrap_or_else(|err| {
        panic!(
            "TOC at `{}` MUST parse cleanly before the AllowLoad contract can be checked: {err}",
            toc_path.display()
        )
    })
}

fn root_toc_path() -> PathBuf {
    blizzard_ui_dir().join(ROOT).join(ROOT_TOC_FILE)
}

fn assert_loaded_cleanly_in_scope(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    loaded: &[String],
    scope: &str,
) {
    assert!(
        loaded.iter().any(|name| name == ROOT),
        "`{ROOT}` must load in the {scope} closure. Loaded set: {loaded:?}"
    );

    let is_loaded: bool = env
        .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_AutoCompletePopupList")"#)
        .unwrap_or_else(|err| panic!("C_AddOns.IsAddOnLoaded must run in {scope}: {err}"));
    assert!(
        is_loaded,
        "`{ROOT}` must be reported loaded in the {scope} scope"
    );

    let errors = recorded_lua_errors(env);
    assert!(
        errors.is_empty(),
        "`{ROOT}` emitted Lua errors in the {scope} scope:\n{}",
        errors.join("\n")
    );
}
