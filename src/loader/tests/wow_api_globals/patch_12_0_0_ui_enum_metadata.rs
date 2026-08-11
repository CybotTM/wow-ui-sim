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
fn test_patch_12_0_0_vas_transaction_purchase_result_value() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                local namespace = Enum.VasTransactionPurchaseResult
                if type(namespace) ~= "table" then
                    return "namespace=" .. type(namespace)
                end
                local expected_values = {
                    DbHouseOwnerRestriction = 20096,
                    EndDbErrors = 20097,
                }
                for name, expected_value in pairs(expected_values) do
                    local value = namespace[name]
                    if type(value) ~= "number" then
                        return name .. ":type=" .. type(value)
                    end
                    if value ~= expected_value then
                        return name .. ":value=" .. tostring(value)
                    end
                end

                local meta = Enum.VasTransactionPurchaseResultMeta
                if type(meta) ~= "table" then
                    return "meta=" .. type(meta)
                end
                local expected_metadata = {
                    MaxValue = 20097,
                    NumValues = 145,
                }
                for name, expected_value in pairs(expected_metadata) do
                    local value = meta[name]
                    if type(value) ~= "number" then
                        return "meta." .. name .. ":type=" .. type(value)
                    end
                    if value ~= expected_value then
                        return "meta." .. name .. ":value=" .. tostring(value)
                    end
                end
                return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "retail 12.0.0 VasTransactionPurchaseResult values and metadata did not match the source register"
    );
}

#[test]
fn test_patch_12_0_0_ui_global_constant_values() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                local expected = {
                    LE_FRAME_TUTORIAL_JOURNEYS_TAB = 163,
                    LE_FRAME_TUTORIAL_LINK_TRANSMOG_CUSTOM_SET = 110,
                    LE_FRAME_TUTORIAL_TRANSMOG_CUSTOM_SET_DROPDOWN = 38,
                    LE_PET_JOURNAL_FILTER_TYPE_BATTLE_PETS = 3,
                    LE_PET_JOURNAL_FILTER_TYPE_NON_COMBAT_PETS = 4,
                }
                for name, expected_value in pairs(expected) do
                    local value = _G[name]
                    if type(value) ~= "number" then
                        return name .. ":type=" .. type(value)
                    end
                    if value ~= expected_value then
                        return name .. ":value=" .. tostring(value)
                    end
                end
                return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "retail 12.0.0 UI global constants did not match the source register"
    );
}

#[test]
fn test_patch_12_0_0_catalog_and_combat_log_constant_values() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                local function check_constant(namespace, name, expected_type, expected_value)
                    local value = namespace[name]
                    if type(value) ~= expected_type then
                        return name .. ":type=" .. type(value)
                    end
                    if value ~= expected_value then
                        return name .. ":value=" .. tostring(value)
                    end
                end

                local catalog = Constants.CatalogShopVirtualCurrencyConstants
                local error_message = check_constant(
                    catalog, "HEARTHSTEEL_VC_CURRENCY_CODE", "string", "XVV")
                if error_message then return error_message end
                error_message = check_constant(
                    catalog, "TRADERS_TENDER_VC_CURRENCY_CODE", "string", "XWP")
                if error_message then return error_message end

                local message_limits = Constants.CombatLogMessageLimits
                error_message = check_constant(
                    message_limits, "CombatLogDefaultMessageLimit", "number", 300)
                if error_message then return error_message end
                error_message = check_constant(
                    message_limits, "CombatLogMaximumMessageLimit", "number", 1000)
                if error_message then return error_message end

                local object_masks = Constants.CombatLogObjectMasks
                local expected_masks = {
                    COMBATLOG_OBJECT_AFFILIATION_MASK = 15,
                    COMBATLOG_OBJECT_CONTROL_MASK = 768,
                    COMBATLOG_OBJECT_REACTION_MASK = 240,
                    COMBATLOG_OBJECT_SPECIAL_MASK = -65536,
                    COMBATLOG_OBJECT_TYPE_MASK = 64512,
                }
                for name, expected_value in pairs(expected_masks) do
                    error_message = check_constant(
                        object_masks, name, "number", expected_value)
                    if error_message then return error_message end
                end

                return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "retail 12.0.0 catalog and combat-log constants did not match the source register"
    );
}

#[test]
fn test_patch_12_0_0_cvar_defaults() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                local expected = {
                    CAAEnabled = "0",
                    CAAInterruptCast = "0",
                    CAAInterruptCastSuccess = "0",
                    CAAPartyHealthFrequency = "0",
                    CAAPartyHealthPercent = "0",
                    CAAPlayerCastFormat = "4",
                    CAAPlayerCastMinTime = "1.500000",
                    CAAPlayerCastMode = "0",
                    CAAPlayerCastThrottle = "0.000000",
                    CAAPlayerHealthFormat = "1",
                    CAAPlayerHealthPercent = "0",
                    CAAPlayerHealthThrottle = "0.000000",
                    CAAResource1Formats = "",
                    CAAResource1Percents = "",
                    CAAResource1Throttle = "0.000000",
                    CAAResource2Formats = "",
                    CAAResource2Percents = "",
                    CAAResource2Throttle = "0.000000",
                    CAASayCombatEnd = "1",
                    CAASayCombatStart = "1",
                    CAASayIfTargeted = "",
                    CAASayTargetName = "1",
                    CAASpeed = "0",
                    CAATargetCastFormat = "0",
                    CAATargetCastMinTime = "1.500000",
                    CAATargetCastMode = "0",
                    CAATargetCastThrottle = "0.000000",
                    CAATargetDeathBehavior = "0",
                    CAATargetHealthFormat = "3",
                    CAATargetHealthPercent = "2",
                    CAATargetHealthThrottle = "0.000000",
                    CAAVoice = "0",
                    CAAVolume = "100",
                    Sound_EnableEncounterWarningsSounds = "1",
                    Sound_EncounterWarningsVolume = "1.000000",
                    WorldTextCritScreenY_v2 = "0.0275",
                    WorldTextGravity_v2 = "0.500000",
                    WorldTextMinAlpha_v2 = "0.500000",
                    WorldTextNonRandomZ_v2 = "2.5",
                    WorldTextRampDuration_v2 = "1.000000",
                    WorldTextRampPowCrit_v2 = "8.000000",
                    WorldTextRampPow_v2 = "1.900000",
                    WorldTextRandomXY_v2 = "0.0",
                    WorldTextRandomZMax_v2 = "1.5",
                    WorldTextRandomZMin_v2 = "0.8",
                    WorldTextScale_v2 = "1.000000",
                    WorldTextScreenY_v2 = "0.015",
                    WorldTextStartPosRandomness_v2 = "1.0",
                    addonChatRestrictionsForced = "0",
                    alwaysShowRuneIcons = "0",
                    auctionSortByBuyoutPrice = "0",
                    auctionSortByUnitPrice = "0",
                    chatBubblesRaid = "0",
                    combatWarningsEnabled = "1",
                    damageMeterEnabled = "0",
                    disableSuggestedLevelActivityFilter = "0",
                }
                for name, expected_value in pairs(expected) do
                    local value = GetCVar(name)
                    if value ~= expected_value then
                        return name .. ":value=" .. tostring(value)
                    end
                    local default_value = GetCVarDefault(name)
                    if default_value ~= expected_value then
                        return name .. ":default=" .. tostring(default_value)
                    end
                end
                return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "retail 12.0.0 CVar defaults did not match the source register"
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
