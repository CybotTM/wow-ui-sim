use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_exposes_mixin_tables_and_methods() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let missing: Vec<String> = env
                    .eval(
                        r#"
                        local missing = {}

                        local function expectTable(name)
                            if type(_G[name]) ~= "table" then
                                table.insert(missing, name .. " table")
                                return nil
                            end
                            return _G[name]
                        end

                        local function expectMethods(mixinName, methodNames)
                            local mixin = expectTable(mixinName)
                            if not mixin then
                                return
                            end

                            for _, methodName in ipairs(methodNames) do
                                if type(mixin[methodName]) ~= "function" then
                                    table.insert(missing, mixinName .. "." .. methodName)
                                end
                            end
                        end

                        expectMethods("AzeriteRespecMixin", {
                            "OnLoad",
                            "OnEvent",
                            "OnShow",
                            "OnHide",
                            "UpdateMoney",
                            "GetRespecItemLocation",
                            "AzeriteRespecItem",
                            "UpdateAzeriteRespecButtonState",
                            "SetRespecItem",
                        })

                        expectMethods("AzeriteRespecItemSlotMixin", {
                            "OnLoad",
                            "RefreshIcon",
                            "RefreshTooltip",
                            "OnClick",
                            "OnDragStart",
                            "OnReceiveDrag",
                            "OnMouseEnter",
                            "OnMouseLeave",
                        })

                        expectMethods("AzeriteRespecButtonMixin", {
                            "OnMouseEnter",
                            "OnMouseLeave",
                        })

                        return missing
                        "#,
                    )
                    .expect("Azerite Respec mixin surface should be inspectable");
                assert!(
                    missing.is_empty(),
                    "`{ROOT}` missing mixin surface entries: {missing:?}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while exposing mixins:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
