use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_show_failed_func_routes_to_close_respec() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let fired: i64 = env
                    .eval(
                        r#"
                        local fired = 0
                        local listener = CreateFrame("Frame")
                        listener:RegisterEvent("AZERITE_EMPOWERED_ITEM_RESPEC_CLOSE")
                        listener:SetScript("OnEvent", function()
                            fired = fired + 1
                        end)

                        UIPanelWindows["AzeriteRespecFrame"].showFailedFunc()

                        return fired
                        "#,
                    )
                    .expect("AzeriteRespecFrame showFailedFunc should run");
                assert_eq!(
                    fired, 1,
                    "`{ROOT}` showFailedFunc should fire the respec-close event"
                );

                let last_close_request = env
                    .state()
                    .borrow()
                    .azerite_empowered
                    .last_close_request
                    .clone();
                assert!(
                    last_close_request.is_none(),
                    "`{ROOT}` showFailedFunc should call CloseAzeriteEmpoweredItemRespec with no item location"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking showFailedFunc routing:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
