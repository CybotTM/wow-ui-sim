//! Startup publication and values for small 12.0.0 enum families.

#![cfg(feature = "retail-12-0-0")]

use super::super::*;

const ENUM_VALUES: &[(&str, &[(&str, i64)])] = &[
    (
        "AccountTransType",
        &[
            ("HouseInitiativeFavor", 66),
            ("TransmogOutfitCollection", 65),
        ],
    ),
    (
        "AccountTransTypeMeta",
        &[("MaxValue", 66), ("NumValues", 67)],
    ),
    (
        "CraftingReagentItemFlag",
        &[("TooltipShowsAsStatModifications", 1)],
    ),
    (
        "CraftingReagentItemFlagMeta",
        &[("MaxValue", 1), ("MinValue", 1)],
    ),
    ("CurrencyDestroyReason", &[("CraftingOrderReagent", 16)]),
    (
        "CurrencyDestroyReasonMeta",
        &[("MaxValue", 16), ("NumValues", 17)],
    ),
    ("CurrencySource", &[("InitiativeReward", 67)]),
    ("CurrencySourceMeta", &[("MaxValue", 67), ("NumValues", 68)]),
    (
        "EditModeAuraFrameSystemIndices",
        &[("ExternalDefensivesFrame", 3)],
    ),
    (
        "EditModeAuraFrameSystemIndicesMeta",
        &[("MaxValue", 3), ("NumValues", 3)],
    ),
    ("EditModeCooldownViewerSetting", &[("BarWidthScale", 11)]),
    (
        "EditModeCooldownViewerSettingMeta",
        &[("MaxValue", 11), ("NumValues", 12)],
    ),
    (
        "GameRule",
        &[
            ("EjJourneysDisabled", 156),
            ("PvPInitialRatingOverride", 190),
        ],
    ),
    ("GameRuleMeta", &[("MaxValue", 190), ("NumValues", 140)]),
    ("HousingDecorActionFlags", &[("PreviewDecor", 2_048)]),
    (
        "HousingDecorActionFlagsMeta",
        &[("MaxValue", 2_048), ("NumValues", 13)],
    ),
    ("HousingItemToastType", &[("House", 4)]),
    (
        "HousingItemToastTypeMeta",
        &[("MaxValue", 4), ("NumValues", 5)],
    ),
    ("MapIconUIWidgetSetType", &[("AdventureMapDetails", 2)]),
    (
        "MapIconUIWidgetSetTypeMeta",
        &[("MaxValue", 2), ("NumValues", 3)],
    ),
    ("SurveyDeliveryMoment", &[("MythicPlusCompleted", 4)]),
    (
        "SurveyDeliveryMomentMeta",
        &[("MaxValue", 4), ("NumValues", 5)],
    ),
];

#[test]
fn test_patch_12_0_0_small_enum_values() {
    let env = WowLuaEnv::new().unwrap();
    let expected_namespaces = ENUM_VALUES
        .iter()
        .map(|(namespace, values)| {
            let expected_values = values
                .iter()
                .map(|(name, value)| format!("[{name:?}] = {value}"))
                .collect::<Vec<_>>()
                .join(",\n                    ");
            format!(
                "[{namespace:?}] = {{\n                    {expected_values}\n                }}"
            )
        })
        .collect::<Vec<_>>()
        .join(",\n                ");
    let script = format!(
        r#"
            local expected = {{
                {expected_namespaces}
            }}
            for namespace_name, expected_values in pairs(expected) do
                local namespace = Enum[namespace_name]
                if type(namespace) ~= "table" then
                    return namespace_name .. ":namespace=" .. type(namespace)
                end
                for name, value in pairs(expected_values) do
                    local actual = namespace[name]
                    if type(actual) ~= "number" then
                        return namespace_name .. "." .. name .. ":type=" .. type(actual)
                    end
                    if actual ~= value then
                        return namespace_name .. "." .. name .. ":value=" .. tostring(actual)
                    end
                end
            end
            return "ok"
        "#,
        expected_namespaces = expected_namespaces,
    );
    let result: String = env.eval(&script).unwrap();
    assert_eq!(
        result, "ok",
        "small 12.0.0 enum namespaces did not match the source register"
    );
}
