use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_selection_event_clears_only_matching_slot() {
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

                        local itemA = { itemID = 170001, bagID = 0, slotIndex = 7 }
                        local itemB = { itemID = 170002, bagID = 0, slotIndex = 8 }
                        function itemA:IsEqualTo(other)
                            _G.__azerite_respec_last_compared_location = other
                            return self == other
                        end

                        local originalSetRespecItem = AzeriteRespecFrame.SetRespecItem
                        AzeriteRespecFrame.SetRespecItem = function(self, location)
                            _G.__azerite_respec_set_respec_item_calls =
                                (_G.__azerite_respec_set_respec_item_calls or 0) + 1
                            _G.__azerite_respec_set_respec_item_arg = location
                            return originalSetRespecItem(self, location)
                        end

                        local mixinEnv = debug.getfenv(AzeriteRespecMixin.SetRespecItem)
                        mixinEnv.Item = {
                            CreateFromItemLocation = function()
                                return {
                                    UnlockItem = function() end,
                                }
                            end,
                        }

                        AzeriteRespecFrame.UpdateMoney = function() end
                        AzeriteRespecFrame.UpdateAzeriteRespecButtonState = function() end
                        AzeriteRespecFrame.ItemSlot.RefreshIcon = function() end
                        AzeriteRespecFrame.ItemSlot.RefreshTooltip = function() end

                        AzeriteRespecFrame.respecItemLocation = itemA
                        FireEvent("AZERITE_EMPOWERED_ITEM_SELECTION_UPDATED", itemB)

                        expect(_G.__azerite_respec_last_compared_location == itemB,
                            "selection event should compare the slotted item against item B")
                        expect(_G.__azerite_respec_set_respec_item_calls == nil,
                            "non-matching selection update should not clear the slot")
                        expect(AzeriteRespecFrame.respecItemLocation == itemA,
                            "non-matching selection update should leave item A in the slot")

                        FireEvent("AZERITE_EMPOWERED_ITEM_SELECTION_UPDATED", itemA)

                        expect(_G.__azerite_respec_last_compared_location == itemA,
                            "matching selection event should compare the slotted item against item A")
                        expect(_G.__azerite_respec_set_respec_item_calls == 1,
                            "matching selection update should call SetRespecItem exactly once")
                        expect(_G.__azerite_respec_set_respec_item_arg == nil,
                            "matching selection update should call SetRespecItem(nil)")
                        expect(AzeriteRespecFrame.respecItemLocation == nil,
                            "matching selection update should clear the respec slot")

                        return table.concat(failures, "\n")
                        "#,
                    )
                    .expect("AzeriteRespec selection event probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` selection event mismatches:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking selection events:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
