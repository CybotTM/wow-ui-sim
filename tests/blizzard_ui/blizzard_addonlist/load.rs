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

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use crate::common::panel_fixtures::recorded_lua_errors;

const ROOT: &str = "Blizzard_AddOnList";
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

fn assert_loaded(loaded: &[String], addon: &str) {
    assert!(
        loaded.iter().any(|name| name == addon),
        "`{addon}` must appear in the loaded addon set for the closure rooted at `{ROOT}`. \
         Loaded set: {loaded:?}"
    );
}
