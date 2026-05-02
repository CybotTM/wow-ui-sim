//! Load smoke for `Blizzard_AdventureMap`.
//!
//! TOC reference (`Interface/BlizzardUI/Blizzard_AdventureMap/
//! Blizzard_AdventureMap.toc`):
//!
//! ```text
//! ## Title: Blizzard Adventure Map UI
//! ## RequiredDep: Blizzard_GarrisonTemplates, Blizzard_MapCanvas, Blizzard_SharedMapDataProviders
//! ## LoadOnDemand: 1
//! ```

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use crate::common::panel_fixtures::{blizzard_ui_dir, recorded_lua_errors};
use std::path::PathBuf;
use wow_ui_sim::toc::TocFile;

const ROOT: &str = "Blizzard_AdventureMap";
const ROOT_TOC_FILE: &str = "Blizzard_AdventureMap.toc";
const REQUIRED_DEPS: &[&str] = &[
    "Blizzard_GarrisonTemplates",
    "Blizzard_MapCanvas",
    "Blizzard_SharedMapDataProviders",
];

#[test]
fn adventure_map_loads_with_dependency_closure_and_no_lua_errors() {
    assert_toc_declares_required_dependencies();

    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, loaded| {
        assert_loaded(loaded, ROOT);
        for dependency in REQUIRED_DEPS {
            assert_loaded(loaded, dependency);
        }

        let errors = recorded_lua_errors(env);
        assert!(
            errors.is_empty(),
            "`{ROOT}` dependency-closure load must emit zero recorded Lua errors after the \
             startup-shape harness ticks startup events. Got:\n  {}",
            errors.join("\n  ")
        );
    });
}

fn assert_toc_declares_required_dependencies() {
    let toc = load_root_toc();
    let dependencies = toc.dependencies();

    assert_eq!(
        dependencies, REQUIRED_DEPS,
        "`{ROOT}` TOC dependency contract changed. Expected required deps: {REQUIRED_DEPS:?}; \
         parsed deps: {dependencies:?}"
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

fn assert_loaded(loaded: &[String], addon: &str) {
    assert!(
        loaded.iter().any(|name| name == addon),
        "`{addon}` must be present in the `{ROOT}` dependency closure. Loaded set: {loaded:?}"
    );
}
