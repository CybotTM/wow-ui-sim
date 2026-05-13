use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_set_respec_item_nil_clears_slot_and_unlocks_previous_item() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(
                        r#"
                        local failures = {}
                        local function expect(condition, message)
                            if not condition then
                                table.insert(failures, message)
                            end
                        end

                        local previousLocation = { itemID = 158041, bagID = 0, slotIndex = 4 }
                        local mixinEnv = debug.getfenv(AzeriteRespecMixin.SetRespecItem)
                        _G.__azerite_respec_unlocked_location = nil
                        mixinEnv.Item = {
                            CreateFromItemLocation = function(_, location)
                                return {
                                    UnlockItem = function()
                                        _G.__azerite_respec_unlocked_location = location
                                    end,
                                }
                            end,
                        }

                        AzeriteRespecFrame.respecItemLocation = previousLocation
                        AzeriteRespecFrame.ItemSlot.Icon:Show()
                        AzeriteRespecFrame.ItemSlot.GlowOverlay:Show()

                        AzeriteRespecFrame:SetRespecItem(nil)

                        expect(AzeriteRespecFrame.respecItemLocation == nil,
                            "respecItemLocation must clear")
                        expect(_G.__azerite_respec_unlocked_location == previousLocation,
                            "previous item location must be unlocked")
                        expect(not AzeriteRespecFrame.ItemSlot.Icon:IsShown(),
                            "ItemSlot.Icon must hide")

                        return table.concat(failures, "\n")
                        "#,
                    )
                    .expect("SetRespecItem(nil) probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` SetRespecItem(nil) cleanup mismatches:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking SetRespecItem(nil):\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
