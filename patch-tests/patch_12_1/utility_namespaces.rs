use super::{assert_ptr_source_omits_qualified_symbols, load_game_ui_without_player_choice};

const SNAPSHOT_ONLY_SYMBOLS: &[&str] = &[
    "ChatFrameUtil.DiscordNameColorize",
    "ChatFrameUtil.FormatDiscordMessage",
    "ChatFrameUtil.GetNameForDiscordMessage",
    "CooldownViewerUtil.AddSoundAlertRadio",
    "CooldownViewerUtil.BuildSoundMenus",
    "CooldownViewerUtil.GetSoundTypeSoundKit",
    "CooldownViewerUtil.GetSoundTypeText",
    "HousingFramesUtil.IsBlueprintCollectionAvailable",
    "HousingFramesUtil.IsBlueprintOperationInProgress",
    "HousingFramesUtil.ShowBlueprintExport",
    "HousingFramesUtil.ShowBlueprintImport",
    "HousingFramesUtil.ShowBlueprintRoomExport",
    "HousingFramesUtil.TryOpenBlueprintCollection",
    "ItemUtil.DisplayEquipSlotTooltip",
    "ItemUtil.GetEmptyEquipSlotTooltipForSlotName",
    "ItemUtil.GetEmptyEquipSlotTooltip",
    "ItemUtil.GetEquipSlotTexture",
    "ItemUtil.GetValidatedItemLocation",
    "PingUtil.SendMacroPing",
    "PingUtil.TogglePingTarget",
    "RegionUtil.GetTopLeftMost",
    "RegionUtil.SortByTopLeft",
    "TimeUtil.BetterDate",
    "TimeUtil.GetRecentTimeDate",
    "UIModeUtil.CreateExtendedBlocklist",
    "UIModeUtil.CreateModifiedBlocklist",
    "UIModeUtil.IsModeActive",
    "UIModeUtil.RegisterMode",
    "UIModeUtil.SetModeActive",
];

/// Proves proposed utility methods do not exist in current PTR source or runtime.
#[test]
fn snapshot_only_utility_methods_remain_absent() {
    assert_ptr_source_omits_qualified_symbols(SNAPSHOT_ONLY_SYMBOLS);

    let env = load_game_ui_without_player_choice();
    let published_symbols: String = env
        .eval(
            r#"
            local names = {
                "ChatFrameUtil.DiscordNameColorize",
                "ChatFrameUtil.FormatDiscordMessage",
                "ChatFrameUtil.GetNameForDiscordMessage",
                "CooldownViewerUtil.AddSoundAlertRadio",
                "CooldownViewerUtil.BuildSoundMenus",
                "CooldownViewerUtil.GetSoundTypeSoundKit",
                "CooldownViewerUtil.GetSoundTypeText",
                "HousingFramesUtil.IsBlueprintCollectionAvailable",
                "HousingFramesUtil.IsBlueprintOperationInProgress",
                "HousingFramesUtil.ShowBlueprintExport",
                "HousingFramesUtil.ShowBlueprintImport",
                "HousingFramesUtil.ShowBlueprintRoomExport",
                "HousingFramesUtil.TryOpenBlueprintCollection",
                "ItemUtil.DisplayEquipSlotTooltip",
                "ItemUtil.GetEmptyEquipSlotTooltipForSlotName",
                "ItemUtil.GetEmptyEquipSlotTooltip",
                "ItemUtil.GetEquipSlotTexture",
                "ItemUtil.GetValidatedItemLocation",
                "PingUtil.SendMacroPing",
                "PingUtil.TogglePingTarget",
                "RegionUtil.GetTopLeftMost",
                "RegionUtil.SortByTopLeft",
                "TimeUtil.BetterDate",
                "TimeUtil.GetRecentTimeDate",
                "UIModeUtil.CreateExtendedBlocklist",
                "UIModeUtil.CreateModifiedBlocklist",
                "UIModeUtil.IsModeActive",
                "UIModeUtil.RegisterMode",
                "UIModeUtil.SetModeActive",
            }
            local published = {}
            for _, name in ipairs(names) do
                local namespaceName, methodName = string.match(name, "^([^.]+)%.(.+)$")
                local namespace = _G[namespaceName]
                if namespace ~= nil and namespace[methodName] ~= nil then
                    table.insert(published, name)
                end
            end
            return table.concat(published, ",")
            "#,
        )
        .expect("utility namespace runtime probe succeeds");

    assert_eq!(published_symbols, "", "unexpected PTR publications");
}

/// Proves the proposed PingUtil removal is reversed and still delegates to C_Ping.
#[test]
fn contextual_ping_helper_remains_vendor_present() {
    let env = load_game_ui_without_player_choice();
    let (helper_type, forwarded_guid, result): (String, String, i32) = env
        .eval(
            r#"
            local forwardedGUID
            local original = C_Ping.GetContextualPingTypeForUnit
            C_Ping.GetContextualPingTypeForUnit = function(guid)
                forwardedGUID = guid
                return 17
            end
            local result = PingUtil:GetContextualPingTypeForUnit("Player-1-TEST")
            C_Ping.GetContextualPingTypeForUnit = original
            return type(PingUtil.GetContextualPingTypeForUnit), forwardedGUID, result
            "#,
        )
        .expect("contextual ping helper behavior probe succeeds");

    assert_eq!(helper_type, "function");
    assert_eq!(forwarded_guid, "Player-1-TEST");
    assert_eq!(result, 17);
}
