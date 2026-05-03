//! Load smoke for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use crate::common::panel_fixtures::recorded_lua_errors;

const ROOT: &str = "Blizzard_ArchaeologyUI";

#[test]
fn archaeology_ui_loads_cleanly_with_no_recorded_lua_errors() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, loaded| {
        assert!(
            loaded.iter().any(|name| name == ROOT),
            "{ROOT} must be present in its dependency closure. Loaded set: {loaded:?}"
        );

        let errors = recorded_lua_errors(env);
        assert!(
            errors.is_empty(),
            "{ROOT} must settle without recorded Lua errors after startup events:\n  {}",
            errors.join("\n  ")
        );
    });
}
