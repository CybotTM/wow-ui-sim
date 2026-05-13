use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_update_money_reads_cost_and_player_money() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                {
                    let mut state = env.state().borrow_mut();
                    state.azerite_empowered.respec_cost = 50_000;
                    state.player.money = 100_000;
                }
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                env.exec(
                    r#"
                    local mixinEnv = debug.getfenv(AzeriteRespecMixin.UpdateMoney)
                    _G.__azerite_respec_money_frame_colors = {}
                    local originalSetMoneyFrameColor = mixinEnv.SetMoneyFrameColor or SetMoneyFrameColor
                    mixinEnv.SetMoneyFrameColor = function(frameName, color)
                        table.insert(_G.__azerite_respec_money_frame_colors, color)
                        return originalSetMoneyFrameColor(frameName, color)
                    end
                    AzeriteRespecFrame.respecItemLocation = { itemID = 158041 }
                    "#,
                )
                .expect("money-color recorder should install cleanly");

                env.fire_event("PLAYER_MONEY")
                    .expect("PLAYER_MONEY should dispatch cleanly");
                let (displayed_money, first_color): (i64, String) = env
                    .eval(
                        r#"
                        return AzeriteRespecFrame.ButtonFrame.MoneyFrame.lastArgMoney,
                            _G.__azerite_respec_money_frame_colors[1] or ""
                        "#,
                    )
                    .expect("first money update should be readable");
                assert_eq!(displayed_money, 50_000);
                assert_eq!(first_color, "white");

                env.state().borrow_mut().player.money = 25_000;
                env.fire_event("PLAYER_MONEY")
                    .expect("second PLAYER_MONEY should dispatch cleanly");
                let (second_color, button_enabled): (String, bool) = env
                    .eval(
                        r#"
                        return _G.__azerite_respec_money_frame_colors[2] or "",
                            AzeriteRespecFrame.ButtonFrame.AzeriteRespecButton:IsEnabled()
                        "#,
                    )
                    .expect("second money update should be readable");
                assert_eq!(second_color, "red");
                assert!(
                    !button_enabled,
                    "`{ROOT}` should disable the respec button when player money is below cost"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking money updates:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
