//! Focused retail 12.0.0 Edit Mode account/aura setting enum values.

#![cfg(feature = "retail-12-0-0")]

use super::super::*;

#[test]
fn test_patch_12_0_0_edit_mode_account_aura_settings_enum_values() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                local expected = {
                    ["Enum.EditModeAccountSetting"] = {
                        ShowPersonalResourceDisplay = 29,
                        ShowEncounterEvents = 30,
                        ShowDamageMeter = 31,
                        ShowExternalDefensives = 32,
                    },
                    ["Enum.EditModeAuraFrameSetting"] = {
                        VisibleSetting = 8,
                        Opacity = 9,
                        ShowDispelType = 10,
                    },
                }
                for enum_name, members in pairs(expected) do
                    local enum = Enum[enum_name:match("Enum%.(.+)")]
                    if type(enum) ~= "table" then
                        return enum_name .. ": expected table"
                    end
                    for name, expected_value in pairs(members) do
                        local value = enum[name]
                        if type(value) ~= "number" or value ~= expected_value then
                            return enum_name .. "." .. name .. ": expected "
                                .. tostring(expected_value) .. ", got " .. tostring(value)
                        end
                    end
                end
                return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "retail 12.0.0 Edit Mode account/aura enum mismatch: {result}"
    );
}
