use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_lod_loads_without_declared_dependencies() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let is_loaded: bool = env
                    .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_AzeriteRespecUI")"#)
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
