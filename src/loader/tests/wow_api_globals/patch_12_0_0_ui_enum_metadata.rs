//! Focused retail 12.0.0 metadata for UI-facing enum families.

#![cfg(feature = "retail-12-0-0")]

use super::super::*;

#[test]
fn test_patch_12_0_0_ui_enum_metadata_values() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                local expected = {
                    CooldownViewerAddAlertStatusMeta = {
                        MaxValue = 3,
                        MinValue = 0,
                        NumValues = 4,
                    },
                    CooldownViewerAlertEventTypeMeta = {
                        MaxValue = 4,
                        MinValue = 1,
                        NumValues = 4,
                    },
                    DamageMeterStyleMeta = {
                        MaxValue = 3,
                        MinValue = 0,
                        NumValues = 4,
                    },
                    EditModeEncounterEventsSystemIndicesMeta = {
                        MaxValue = 4,
                        MinValue = 1,
                        NumValues = 4,
                    },
                    HouseExteriorWMODataFlagsMeta = {
                        MaxValue = 4,
                        MinValue = 0,
                        NumValues = 4,
                    },
                }
                for namespace_name, expected_values in pairs(expected) do
                    local namespace = Enum[namespace_name]
                    if type(namespace) ~= "table" then
                        return namespace_name .. ":namespace=" .. type(namespace)
                    end
                    for name, expected_value in pairs(expected_values) do
                        local value = namespace[name]
                        if type(value) ~= "number" then
                            return namespace_name .. "." .. name .. ":type=" .. type(value)
                        end
                        if value ~= expected_value then
                            return namespace_name .. "." .. name .. ":value=" .. tostring(value)
                        end
                    end
                end
                return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "retail 12.0.0 UI enum metadata did not match the source register"
    );
}

#[test]
fn test_patch_12_0_0_heal_prediction_enum_values() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                local expected = {
                    UnitDamageAbsorbClampMode = {
                        MissingHealth = 0,
                        MissingHealthWithoutIncomingHeals = 1,
                        MaximumHealth = 2,
                    },
                    UnitDamageAbsorbClampModeMeta = {
                        MaxValue = 2,
                        MinValue = 0,
                        NumValues = 3,
                    },
                    UnitHealAbsorbClampMode = {
                        CurrentHealth = 0,
                        MaximumHealth = 1,
                    },
                    UnitHealAbsorbClampModeMeta = {
                        MaxValue = 1,
                        MinValue = 0,
                        NumValues = 2,
                    },
                    UnitHealAbsorbMode = {
                        ReducedByIncomingHeals = 0,
                        Total = 1,
                    },
                    UnitHealAbsorbModeMeta = {
                        MaxValue = 1,
                        MinValue = 0,
                        NumValues = 2,
                    },
                    UnitIncomingHealClampMode = {
                        MissingHealth = 0,
                        MaximumHealth = 1,
                    },
                    UnitIncomingHealClampModeMeta = {
                        MaxValue = 1,
                        MinValue = 0,
                        NumValues = 2,
                    },
                }
                for namespace_name, expected_values in pairs(expected) do
                    local namespace = Enum[namespace_name]
                    if type(namespace) ~= "table" then
                        return namespace_name .. ":namespace=" .. type(namespace)
                    end
                    for name, expected_value in pairs(expected_values) do
                        local value = namespace[name]
                        if type(value) ~= "number" then
                            return namespace_name .. "." .. name .. ":type=" .. type(value)
                        end
                        if value ~= expected_value then
                            return namespace_name .. "." .. name .. ":value=" .. tostring(value)
                        end
                    end
                end
                return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "retail 12.0.0 heal-prediction enum values did not match the source register"
    );
}

#[test]
fn test_patch_12_0_0_unit_aura_sort_rule_enum_values() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                local expected = {
                    UnitAuraSortRule = {
                        Default = 1,
                        Expiration = 3,
                        ExpirationOnly = 4,
                        Name = 5,
                        NameOnly = 6,
                        Unsorted = 0,
                    },
                    UnitAuraSortRuleMeta = {
                        MaxValue = 6,
                        MinValue = 0,
                        NumValues = 7,
                    },
                }
                for namespace_name, expected_values in pairs(expected) do
                    local namespace = Enum[namespace_name]
                    if type(namespace) ~= "table" then
                        return namespace_name .. ":namespace=" .. type(namespace)
                    end
                    for name, expected_value in pairs(expected_values) do
                        local value = namespace[name]
                        if type(value) ~= "number" then
                            return namespace_name .. "." .. name .. ":type=" .. type(value)
                        end
                        if value ~= expected_value then
                            return namespace_name .. "." .. name .. ":value=" .. tostring(value)
                        end
                    end
                end
                return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "retail 12.0.0 UnitAuraSortRule values did not match the source register"
    );
}

#[test]
fn test_patch_12_0_0_cooldown_alert_event_type_omits_later_members() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                local event_type = Enum.CooldownViewerAlertEventType
                if rawget(event_type, "OnAuraApplied") ~= nil then
                    return "OnAuraApplied:present"
                end
                if rawget(event_type, "OnAuraRemoved") ~= nil then
                    return "OnAuraRemoved:present"
                end
                return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "retail 12.0.0 exposed cooldown alert event types from a later epoch"
    );
}
