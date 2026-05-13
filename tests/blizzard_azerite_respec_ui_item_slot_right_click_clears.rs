use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_item_slot_right_click_clears_existing_slot() {
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

                        local previousLocation = { itemID = 158041, bagID = 0, slotIndex = 8 }
                        local respecMixinEnv = debug.getfenv(AzeriteRespecMixin.SetRespecItem)
                        local slotMixinEnv = debug.getfenv(AzeriteRespecItemSlotMixin.OnClick)
                        local originalClearCursor = slotMixinEnv.ClearCursor
                        local originalSetRespecItem = AzeriteRespecFrame.SetRespecItem

                        _G.__azerite_respec_right_click_set_arg = "not-called"
                        _G.__azerite_respec_right_click_unlocked_location = nil
                        _G.__azerite_respec_right_click_clear_cursor_count = 0

                        respecMixinEnv.Item = {
                            CreateFromItemLocation = function(_, location)
                                return {
                                    UnlockItem = function()
                                        _G.__azerite_respec_right_click_unlocked_location = location
                                    end,
                                }
                            end,
                        }

                        slotMixinEnv.ClearCursor = function()
                            _G.__azerite_respec_right_click_clear_cursor_count =
                                _G.__azerite_respec_right_click_clear_cursor_count + 1
                            return originalClearCursor()
                        end

                        AzeriteRespecFrame.SetRespecItem = function(self, location)
                            _G.__azerite_respec_right_click_set_arg = location
                            return originalSetRespecItem(self, location)
                        end

                        AzeriteRespecFrame.respecItemLocation = previousLocation
                        AzeriteRespecFrame.ItemSlot.Icon:Show()
                        AzeriteRespecFrame.ItemSlot.GlowOverlay:Show()

                        AzeriteRespecFrame.ItemSlot:OnClick("RightButton")

                        expect(_G.__azerite_respec_right_click_set_arg == nil,
                            "right click should call SetRespecItem(nil)")
                        expect(AzeriteRespecFrame.respecItemLocation == nil,
                            "right click should clear respecItemLocation")
                        expect(_G.__azerite_respec_right_click_unlocked_location == previousLocation,
                            "right click should unlock the previous slot item")
                        expect(not AzeriteRespecFrame.ItemSlot.Icon:IsShown(),
                            "right click should hide the item slot icon")
                        expect(_G.__azerite_respec_right_click_clear_cursor_count == 1,
                            "right click should run ClearCursor once even without a cursor item")

                        return table.concat(failures, "\n")
                        "#,
                    )
                    .expect("AzeriteRespec right-click clear probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` right-click clear mismatches:\n{failures}"
                );

                let cursor_is_empty = env.state().borrow().cursor_item.is_none();
                assert!(
                    cursor_is_empty,
                    "`{ROOT}` right click with no cursor item should leave the Rust cursor empty"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking right-click clear:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
