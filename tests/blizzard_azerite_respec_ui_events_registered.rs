use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_onload_registers_money_and_selection_events() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let (money_registered, selection_registered): (bool, bool) = env
                    .eval(
                        r#"
                        return AzeriteRespecFrame:IsEventRegistered("PLAYER_MONEY"),
                            AzeriteRespecFrame:IsEventRegistered("AZERITE_EMPOWERED_ITEM_SELECTION_UPDATED")
                        "#,
                    )
                    .expect("AzeriteRespecFrame event registration state should be readable");
                assert!(
                    money_registered,
                    "`{ROOT}` should register PLAYER_MONEY during AzeriteRespecMixin:OnLoad"
                );
                assert!(
                    selection_registered,
                    "`{ROOT}` should register AZERITE_EMPOWERED_ITEM_SELECTION_UPDATED during OnLoad"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking registered events:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
