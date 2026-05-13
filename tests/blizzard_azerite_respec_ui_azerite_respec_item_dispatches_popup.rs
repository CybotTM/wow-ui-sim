use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";
const ITEM_LINK: &str = "|cffa335ee|Hitem:19019::::::::|h[Thunderfury]|h|r";

#[test]
fn blizzard_azerite_respec_ui_azerite_respec_item_dispatches_regular_and_expensive_popups() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(&format!(
                        r#"
                        local failures = {{}}
                        local function expect(condition, message)
                            if not condition then
                                table.insert(failures, message)
                            end
                        end

                        local mixinEnv = debug.getfenv(AzeriteRespecMixin.AzeriteRespecItem)
                        _G.__azerite_respec_popup_calls = {{}}
                        local respecLocation = {{ itemID = 19019, bagID = 0, slotIndex = 7 }}

                        mixinEnv.Item = {{
                            CreateFromItemLocation = function(_, location)
                                expect(location == respecLocation,
                                    "AzeriteRespecItem should read the frame's respec item location")
                                return {{
                                    GetItemLink = function()
                                        return "{ITEM_LINK}"
                                    end,
                                }}
                            end,
                        }}
                        mixinEnv.StaticPopup_Show = function(which, textArg1, textArg2, data)
                            table.insert(_G.__azerite_respec_popup_calls, {{
                                which = which,
                                textArg1 = textArg1,
                                textArg2 = textArg2,
                                data = data,
                            }})
                        end

                        AzeriteRespecFrame.respecItemLocation = respecLocation
                        AzeriteRespecFrame.respecCost = 5000
                        AzeriteRespecFrame:AzeriteRespecItem()

                        local regular = _G.__azerite_respec_popup_calls[1]
                        expect(regular.which == "CONFIRM_AZERITE_EMPOWERED_RESPEC",
                            "low respec cost should use the regular confirmation popup")
                        expect(regular.textArg1 == "{ITEM_LINK}",
                            "regular confirmation should pass the item link as textArg1")
                        expect(regular.textArg2 == nil,
                            "regular confirmation should leave textArg2 nil")
                        expect(regular.data.empoweredItemLocation == respecLocation,
                            "regular confirmation data should carry the respec item location")
                        expect(regular.data.respecCost == 5000,
                            "regular confirmation data should carry the 5000 copper cost")

                        AzeriteRespecFrame.respecCost = 20000000
                        AzeriteRespecFrame:AzeriteRespecItem()

                        local expensive = _G.__azerite_respec_popup_calls[2]
                        expect(expensive.which == "CONFIRM_AZERITE_EMPOWERED_RESPEC_EXPENSIVE",
                            "high respec cost should use the expensive confirmation popup")
                        expect(expensive.textArg1 == "{ITEM_LINK}",
                            "expensive confirmation should pass the item link as textArg1")
                        expect(expensive.textArg2 == nil,
                            "expensive confirmation should leave textArg2 nil")
                        expect(expensive.data.empoweredItemLocation == respecLocation,
                            "expensive confirmation data should carry the respec item location")
                        expect(expensive.data.respecCost == 20000000,
                            "expensive confirmation data should carry the high copper cost")

                        return table.concat(failures, "\n")
                        "#
                    ))
                    .expect("AzeriteRespecItem popup probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` AzeriteRespecItem popup mismatches:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking AzeriteRespecItem popups:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
