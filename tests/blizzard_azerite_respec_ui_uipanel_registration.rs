use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_registers_uipanel_window_metadata() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let (exists, area, pushable, has_show_failed_func, show_failed_matches): (
                    bool,
                    String,
                    i64,
                    bool,
                    bool,
                ) = env
                    .eval(
                        r#"
                        local entry = UIPanelWindows["AzeriteRespecFrame"]
                        return type(entry) == "table",
                            entry and entry.area or "",
                            entry and entry.pushable or -1,
                            entry and type(entry.showFailedFunc) == "function" or false,
                            entry and entry.showFailedFunc == C_AzeriteEmpoweredItem.CloseAzeriteEmpoweredItemRespec or false
                        "#,
                    )
                    .expect("AzeriteRespecFrame UIPanelWindows metadata should be readable");

                assert!(exists, "`{ROOT}` should register AzeriteRespecFrame");
                assert_eq!(area, "left");
                assert_eq!(pushable, 3);
                assert!(
                    has_show_failed_func,
                    "`{ROOT}` should publish a callable showFailedFunc"
                );
                assert!(
                    show_failed_matches,
                    "showFailedFunc should resolve to C_AzeriteEmpoweredItem.CloseAzeriteEmpoweredItemRespec"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors during UIPanelWindows registration:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
