use super::{
    assert_ptr_source_omits_qualified_methods, assert_ptr_source_omits_tokens,
    load_full_game_ui_with_all_lod, ptr_source_files,
};

const REMOVED_NAMEPLATE_METHODS: &[&str] = &[
    "GetNamePlateEnemyClickThrough",
    "GetNamePlateEnemyPreferredClickInsets",
    "GetNamePlateEnemySize",
    "GetNamePlateFriendlyClickThrough",
    "GetNamePlateFriendlyPreferredClickInsets",
    "GetNamePlateFriendlySize",
    "GetNamePlateSelfClickThrough",
    "GetNamePlateSelfPreferredClickInsets",
    "GetNamePlateSelfSize",
    "GetNumNamePlateMotionTypes",
    "SetNamePlateEnemyClickThrough",
    "SetNamePlateEnemyPreferredClickInsets",
    "SetNamePlateEnemySize",
    "SetNamePlateFriendlyClickThrough",
    "SetNamePlateFriendlyPreferredClickInsets",
    "SetNamePlateFriendlySize",
    "SetNamePlateSelfClickThrough",
    "SetNamePlateSelfPreferredClickInsets",
    "SetNamePlateSelfSize",
];

const REMOVED_TRANSMOG_COLLECTION_OUTFIT_METHODS: &[&str] = &[
    "DeleteOutfit",
    "GetItemTransmogInfoListFromOutfitHyperlink",
    "GetNumMaxOutfits",
    "GetOutfitHyperlinkFromItemTransmogInfoList",
    "GetOutfitInfo",
    "GetOutfitItemTransmogInfoList",
    "GetOutfits",
    "ModifyOutfit",
    "NewOutfit",
    "RenameOutfit",
];

const SNAPSHOT_ONLY_SYMBOLS: &[&str] = &[
    "AddBehavioralMessagingTrayToStatusFrames",
    "AddFriendFrame_Show",
    "AddGMChatStatusFrameToStatusFrames",
    "AddTicketStatusFrameToStatusFrames",
    "AddWowSurveyStatusFrameToStatusFrames",
    "AlliedRacesFrame_TryShow",
    "AnchorUtil.ApplyFlowLayout",
    "ApplySecureDelegatesToTable",
    "ArchaeologyFrame_ToggleUI",
    "ArcheologyDigsiteProgressBar_OnSurveyCast",
    "ArdenwealdGardening_LoadUI",
    "ArtifactFrame_OnTraitsRefunded",
    "AuraUtil.GetAuraBorderColor",
    "AzeriteEmpoweredItemUI_LoadUI",
    "AzeriteEssenceUI_LoadUI",
    "BattlefieldMap_ToggleUI",
    "BehavioralMessaging_LoadUI",
    "BehavioralMessagingTray_OnNotification",
    "Blizzard_HousingCatalogUtil.AddDecorEntryTooltipTrackingText",
    "Blizzard_HousingCatalogUtil.TrackHousingDecorID",
    "BNet_GetBattleTagComponents",
    "BNet_GetBattleTagSelf",
    "BNet_GetBroadcastTextSelf",
    "BNet_GetFriendLevelRank",
    "BNet_IsFriendLevelEqualOrHigher",
    "BoostTutorial_LoadUI",
    "ChallengeModeCompleteBanner_OnChallengeModeCompleted",
    "ChatAdditionalColor_OpenColorPicker",
    "CheckActiveStoreForFree",
    "CombatText_LoadUI",
    "CompactUnitFrame_GetOptionDispelIndicatorOverlayAnimation",
    "CompactUnitFrame_GetOptionDispelIndicatorOverlayType",
    "CompactUnitFrameUtil.GenerateNewConfig",
    "ConfirmDisenchantRollDialog_Show",
    "ConfirmLootRollDialog_Show",
    "ConfirmTalentWipeDialog_Show",
    "ContributionCollectionFrame_LoadUI",
    "CooldownViewer_MarkAuraCacheDirty",
    "CooldownViewerContextMenu_AddAlertEntryButton",
    "CooldownViewerContextMenu_AddNewAlertButton",
    "CooldownViewerDraggedItem_Clear",
    "CooldownViewerDraggedItem_Pickup",
    "CooldownViewerDraggedItem_SetIsLegalTarget",
    "CovenantCallings_LoadUI",
    "DebugTools_LoadUI",
    "EditModeManagerFrame_EscapePressed",
    "EncounterJournal_SetTabVisibe",
    "EventTrace_LoadUI",
    "ExpansionTrial_LoadUI",
    "FadingFrame_CopyTextScalingTime",
    "FadingFrame_GetTextScalingMinHeight",
    "FadingFrame_InitSlot",
    "FadingFrame_SetTextScaling",
    "FadingFrame_StartTextScaling",
    "FadingFrame_StopTextScaling",
    "FadingFrame_UpdateTextScaling",
    "GameMenuFrame_EscapePressed",
    "GameMenuFrame_IsShown",
    "GameMenuFrame_Show",
    "GetBottomManagedFrameContainer",
    "GetChatAdditionalColor",
    "GetDiscordUserCommunityLink",
    "GetDiscordUserLink",
    "GetGarrisonMissionFrameNameForFollowerType",
    "GetGarrisonTypeForFollowerType",
    "GetPlayerBottomManagedFrameContainer",
    "GetRightManagedFrameContainer",
    "GetUIPanelLayoutAttribute",
    "GetUIPanelLayoutFrame",
    "GMChatFrame_OnWhisperFromGM",
    "GossipConfirmDialog_Show",
    "GuildControlDiscord_Loaded_OnEvent",
    "GuildControlDiscord_Loaded_OnLoad",
    "GuildControlDiscord_SetGuildSettingsCheckboxes",
    "GuildControlRankDiscord_OnLoad",
    "HandleQuestSessionInviteToPartyConfirmation",
    "HelpFrame_EscapePressed",
    "HideBarberShopFrame",
    "HideGossipFrame",
    "HideInstanceBootDialog",
    "HideInstanceLockDialog",
    "HideSummonConfirmationDialogs",
    "HouseFinderFrame_LoadUI",
    "HousingBulletinBoardFrame_LoadUI",
    "HousingControls_LoadUI",
    "HybridMinimap_LoadUI",
    "InputBoxInstructions_OnEnter",
    "InputBoxInstructions_OnLeave",
    "InputBoxInstructions_ShowTooltipIfTruncated",
    "IslandsPartyPoseFrame_TryShow",
    "IsMouseoverCastSupported",
    "IsSummonConfirmationDialogVisible",
    "IsTypeAdditionalChatColor",
    "Kiosk_LoadUI",
    "KioskFrame_HandlePlayerEnteringWorld",
    "LandingSoulbinds_LoadUI",
    "LFGListApplicationViewer_OpenEditMode",
    "LFGListApplicationViewerRemoveEntryButton_OnClick",
    "LocaleUtil.GetLocaleDisplayName",
    "LocalizePlayerFrame_zhCN",
    "LocalizePlayerFrame_zhTW",
    "LootFrame_EscapePressed",
    "MenuUtil.CreateHighlightButton",
    "MovePad_LoadUI",
    "NPE_InitializeIfLoaded",
    "OpacityFrame_EscapePressed",
    "OpenEncounterJournalToJourney",
    "OpenOrderHallTalentUI",
    "OpenPlayerSpellsToGlyphTarget",
    "PhotoSharingFrame_EscapePressed",
    "PlayerChoiceFrame_TryShow",
    "PVPUI_LoadUI",
    "RaidWarningUtil.UpdateCenterScreenAnchors",
    "RecentAlliesUtil.GetBestSocialUIPresenceTypeForStateData",
    "RecruitAFriendFrameSocialInitializeAADC",
    "RegisterGameMenuEscHandler",
    "RegisterPlayerInteraction",
    "ReportFrame_EscapePressed",
    "ResetDiscordSettings",
    "RestoreGMChatFrameSession",
    "SetGhostFrameShown",
    "SetPlayerInteractionConditions",
    "SettingsPanel_EscapePressed",
    "SetUIPanelLayoutAttribute",
    "ShouldDisplaySpellCooldown",
    "ShowAchievementFrameForAchievement",
    "ShowAdventureMapFrameForFollowerType",
    "ShowArtifactFrame",
    "ShowArtifactRelicForgeFrame",
    "ShowAuctionHouseFrame",
    "ShowBarberShopFrame",
    "ShowBlackMarketFrame",
    "ShowChallengesKeystoneFrame",
    "ShowFlightMapFrame",
    "ShowGarrisonCapacitiveDisplayFrame",
    "ShowGarrisonMissionFrameForFollowerType",
    "ShowGarrisonRecruiterFrame",
    "ShowGarrisonShipyardFrame",
    "ShowGuildBankFrame",
    "ShowHeirloomsJournalToClosestUpgradeablePage",
    "ShowInstanceBootDialog",
    "ShowInstanceLockDialog",
    "ShowItemSocketingFrame",
    "ShowItemUpgradeFrame",
    "ShowMatchCelebrationPartyPoseFrame",
    "ShowPendingPlayerChoiceResponseUI",
    "ShowPerksProgramFrame",
    "ShowProfessionEquipmentHelpTip",
    "ShowProfessionsCustomerOrdersFrame",
    "ShowProfessionsFrame",
    "ShowQuestSessionGroupInviteConfirmation",
    "ShowQuestSessionGroupInviteReceivedConfirmation",
    "ShowRemixArtifactFrame",
    "ShowRuneforgeFrame",
    "ShowSummonConfirmationDialog",
    "ShowTaxiMapFrame",
    "SimpleCheckout_EscapePressed",
    "SocialUIContactsFrameInitializeAADC",
    "SoulbindViewer_LoadUI",
    "SpellFlyout_EscapePressed",
    "SplashFrame_EscapePressed",
    "StoreEscapePressed",
    "StringUtil.JoinAlternatingConditionalColor",
    "ToggleRAFPanel",
    "ToggleSocialUI",
    "TryShowAnimaDiversionFrame",
    "TryShowCovenantPreviewFrame",
    "UnitPopupSharedUtil.IsFriendshipUpgrade",
    "UpdateQuestAcceptLogFullDialog",
    "VisualAlert_GetTypeTemplate",
    "VisualAlert_GetTypeText",
    "VisualAlertData_ForEach",
    "VisualAlerts_RegisterAll",
    "WarfrontsPartyPoseFrame_TryShow",
    "WowSurveyStatusFrame_OnSurveyDelivered",
];

fn assert_nameplate_source_omits_table_and_dynamic_publications() {
    for (path, source) in ptr_source_files() {
        for method in REMOVED_NAMEPLATE_METHODS {
            let table_literal_patterns = [
                format!("{method} ="),
                format!("[\"{method}\"] ="),
                format!("['{method}'] ="),
            ];
            let dynamic_name_patterns = [format!("\"{method}\""), format!("'{method}'")];

            for pattern in table_literal_patterns
                .iter()
                .chain(dynamic_name_patterns.iter())
            {
                assert!(
                    !source.contains(pattern),
                    "removed NamePlate method {method} appears in table/dynamic source pattern {pattern} in {}",
                    path.display(),
                );
            }
        }

        assert!(
            !source
                .lines()
                .any(|line| { line.contains("C_NamePlate[") && line.contains("] =") }),
            "dynamic C_NamePlate publication appears in {}",
            path.display(),
        );
    }
}

/// Proves removed NamePlate methods stay absent while retained APIs remain callable.
#[test]
fn removed_nameplate_methods_are_absent_after_full_lod_load() {
    // Source checks only falsify; the full-LoD runtime probe below is the proof.
    assert_ptr_source_omits_qualified_methods("C_NamePlate", REMOVED_NAMEPLATE_METHODS);
    assert_nameplate_source_omits_table_and_dynamic_publications();

    let env = load_full_game_ui_with_all_lod();
    let (removed_published, retained_non_functions): (String, String) = env
        .eval(
            r#"
            local removed = {
                "GetNamePlateEnemyClickThrough",
                "GetNamePlateEnemyPreferredClickInsets",
                "GetNamePlateEnemySize",
                "GetNamePlateFriendlyClickThrough",
                "GetNamePlateFriendlyPreferredClickInsets",
                "GetNamePlateFriendlySize",
                "GetNamePlateSelfClickThrough",
                "GetNamePlateSelfPreferredClickInsets",
                "GetNamePlateSelfSize",
                "GetNumNamePlateMotionTypes",
                "SetNamePlateEnemyClickThrough",
                "SetNamePlateEnemyPreferredClickInsets",
                "SetNamePlateEnemySize",
                "SetNamePlateFriendlyClickThrough",
                "SetNamePlateFriendlyPreferredClickInsets",
                "SetNamePlateFriendlySize",
                "SetNamePlateSelfClickThrough",
                "SetNamePlateSelfPreferredClickInsets",
                "SetNamePlateSelfSize",
            }
            local retained = {
                "GetNamePlateForUnit",
                "GetNamePlates",
                "SetNamePlateSize",
            }
            local removedPublished = {}
            for _, name in ipairs(removed) do
                if rawget(C_NamePlate, name) ~= nil then
                    table.insert(removedPublished, name)
                end
            end
            local retainedNonFunctions = {}
            for _, name in ipairs(retained) do
                if type(rawget(C_NamePlate, name)) ~= "function" then
                    table.insert(retainedNonFunctions, name)
                end
            end
            return table.concat(removedPublished, ","),
                table.concat(retainedNonFunctions, ",")
            "#,
        )
        .expect("C_NamePlate runtime probe succeeds");

    assert_eq!(
        removed_published, "",
        "removed C_NamePlate methods were published"
    );
    assert_eq!(
        retained_non_functions, "",
        "retained C_NamePlate methods are not callable functions",
    );
}

/// Proves removed TransmogCollection outfit methods stay absent while appearance queries remain callable.
#[test]
fn removed_transmog_collection_outfit_methods_are_absent_after_full_lod_load() {
    // Source checks only falsify; the full-LoD runtime probe below is the proof.
    assert_ptr_source_omits_qualified_methods(
        "C_TransmogCollection",
        REMOVED_TRANSMOG_COLLECTION_OUTFIT_METHODS,
    );

    let env = load_full_game_ui_with_all_lod();
    let (removed_published, retained_non_functions): (String, String) = env
        .eval(
            r#"
            local removed = {
                "DeleteOutfit",
                "GetItemTransmogInfoListFromOutfitHyperlink",
                "GetNumMaxOutfits",
                "GetOutfitHyperlinkFromItemTransmogInfoList",
                "GetOutfitInfo",
                "GetOutfitItemTransmogInfoList",
                "GetOutfits",
                "ModifyOutfit",
                "NewOutfit",
                "RenameOutfit",
            }
            local retained = {
                "GetAppearanceSources",
                "GetAllAppearanceSources",
            }
            local removedPublished = {}
            for _, name in ipairs(removed) do
                if rawget(C_TransmogCollection, name) ~= nil then
                    table.insert(removedPublished, name)
                end
            end
            local retainedNonFunctions = {}
            for _, name in ipairs(retained) do
                if type(rawget(C_TransmogCollection, name)) ~= "function" then
                    table.insert(retainedNonFunctions, name)
                end
            end
            return table.concat(removedPublished, ","),
                table.concat(retainedNonFunctions, ",")
            "#,
        )
        .expect("C_TransmogCollection runtime probe succeeds");

    assert_eq!(
        removed_published, "",
        "removed C_TransmogCollection outfit methods were published"
    );
    assert_eq!(
        retained_non_functions, "",
        "retained C_TransmogCollection appearance methods are not callable functions",
    );
}

/// Proves conservative source-absent additions remain absent after PTR startup.
#[test]
fn source_absent_additions_remain_absent() {
    assert_ptr_source_omits_tokens(SNAPSHOT_ONLY_SYMBOLS);

    let env = load_full_game_ui_with_all_lod();
    let published_symbols: String = env
        .eval(
            r#"
            local names = {
                "AddBehavioralMessagingTrayToStatusFrames",
                "AddFriendFrame_Show",
                "AddGMChatStatusFrameToStatusFrames",
                "AddTicketStatusFrameToStatusFrames",
                "AddWowSurveyStatusFrameToStatusFrames",
                "AlliedRacesFrame_TryShow",
                "AnchorUtil.ApplyFlowLayout",
                "ApplySecureDelegatesToTable",
                "ArchaeologyFrame_ToggleUI",
                "ArcheologyDigsiteProgressBar_OnSurveyCast",
                "ArdenwealdGardening_LoadUI",
                "ArtifactFrame_OnTraitsRefunded",
                "AuraUtil.GetAuraBorderColor",
                "AzeriteEmpoweredItemUI_LoadUI",
                "AzeriteEssenceUI_LoadUI",
                "BattlefieldMap_ToggleUI",
                "BehavioralMessaging_LoadUI",
                "BehavioralMessagingTray_OnNotification",
                "Blizzard_HousingCatalogUtil.AddDecorEntryTooltipTrackingText",
                "Blizzard_HousingCatalogUtil.TrackHousingDecorID",
                "BNet_GetBattleTagComponents",
                "BNet_GetBattleTagSelf",
                "BNet_GetBroadcastTextSelf",
                "BNet_GetFriendLevelRank",
                "BNet_IsFriendLevelEqualOrHigher",
                "BoostTutorial_LoadUI",
                "ChallengeModeCompleteBanner_OnChallengeModeCompleted",
                "ChatAdditionalColor_OpenColorPicker",
                "CheckActiveStoreForFree",
                "CombatText_LoadUI",
                "CompactUnitFrame_GetOptionDispelIndicatorOverlayAnimation",
                "CompactUnitFrame_GetOptionDispelIndicatorOverlayType",
                "CompactUnitFrameUtil.GenerateNewConfig",
                "ConfirmDisenchantRollDialog_Show",
                "ConfirmLootRollDialog_Show",
                "ConfirmTalentWipeDialog_Show",
                "ContributionCollectionFrame_LoadUI",
                "CooldownViewer_MarkAuraCacheDirty",
                "CooldownViewerContextMenu_AddAlertEntryButton",
                "CooldownViewerContextMenu_AddNewAlertButton",
                "CooldownViewerDraggedItem_Clear",
                "CooldownViewerDraggedItem_Pickup",
                "CooldownViewerDraggedItem_SetIsLegalTarget",
                "CovenantCallings_LoadUI",
                "DebugTools_LoadUI",
                "EditModeManagerFrame_EscapePressed",
                "EncounterJournal_SetTabVisibe",
                "EventTrace_LoadUI",
                "ExpansionTrial_LoadUI",
                "FadingFrame_CopyTextScalingTime",
                "FadingFrame_GetTextScalingMinHeight",
                "FadingFrame_InitSlot",
                "FadingFrame_SetTextScaling",
                "FadingFrame_StartTextScaling",
                "FadingFrame_StopTextScaling",
                "FadingFrame_UpdateTextScaling",
                "GameMenuFrame_EscapePressed",
                "GameMenuFrame_IsShown",
                "GameMenuFrame_Show",
                "GetBottomManagedFrameContainer",
                "GetChatAdditionalColor",
                "GetDiscordUserCommunityLink",
                "GetDiscordUserLink",
                "GetGarrisonMissionFrameNameForFollowerType",
                "GetGarrisonTypeForFollowerType",
                "GetPlayerBottomManagedFrameContainer",
                "GetRightManagedFrameContainer",
                "GetUIPanelLayoutAttribute",
                "GetUIPanelLayoutFrame",
                "GMChatFrame_OnWhisperFromGM",
                "GossipConfirmDialog_Show",
                "GuildControlDiscord_Loaded_OnEvent",
                "GuildControlDiscord_Loaded_OnLoad",
                "GuildControlDiscord_SetGuildSettingsCheckboxes",
                "GuildControlRankDiscord_OnLoad",
                "HandleQuestSessionInviteToPartyConfirmation",
                "HelpFrame_EscapePressed",
                "HideBarberShopFrame",
                "HideGossipFrame",
                "HideInstanceBootDialog",
                "HideInstanceLockDialog",
                "HideSummonConfirmationDialogs",
                "HouseFinderFrame_LoadUI",
                "HousingBulletinBoardFrame_LoadUI",
                "HousingControls_LoadUI",
                "HybridMinimap_LoadUI",
                "InputBoxInstructions_OnEnter",
                "InputBoxInstructions_OnLeave",
                "InputBoxInstructions_ShowTooltipIfTruncated",
                "IslandsPartyPoseFrame_TryShow",
                "IsMouseoverCastSupported",
                "IsSummonConfirmationDialogVisible",
                "IsTypeAdditionalChatColor",
                "Kiosk_LoadUI",
                "KioskFrame_HandlePlayerEnteringWorld",
                "LandingSoulbinds_LoadUI",
                "LFGListApplicationViewer_OpenEditMode",
                "LFGListApplicationViewerRemoveEntryButton_OnClick",
                "LocaleUtil.GetLocaleDisplayName",
                "LocalizePlayerFrame_zhCN",
                "LocalizePlayerFrame_zhTW",
                "LootFrame_EscapePressed",
                "MenuUtil.CreateHighlightButton",
                "MovePad_LoadUI",
                "NPE_InitializeIfLoaded",
                "OpacityFrame_EscapePressed",
                "OpenEncounterJournalToJourney",
                "OpenOrderHallTalentUI",
                "OpenPlayerSpellsToGlyphTarget",
                "PhotoSharingFrame_EscapePressed",
                "PlayerChoiceFrame_TryShow",
                "PVPUI_LoadUI",
                "RaidWarningUtil.UpdateCenterScreenAnchors",
                "RecentAlliesUtil.GetBestSocialUIPresenceTypeForStateData",
                "RecruitAFriendFrameSocialInitializeAADC",
                "RegisterGameMenuEscHandler",
                "RegisterPlayerInteraction",
                "ReportFrame_EscapePressed",
                "ResetDiscordSettings",
                "RestoreGMChatFrameSession",
                "SetGhostFrameShown",
                "SetPlayerInteractionConditions",
                "SettingsPanel_EscapePressed",
                "SetUIPanelLayoutAttribute",
                "ShouldDisplaySpellCooldown",
                "ShowAchievementFrameForAchievement",
                "ShowAdventureMapFrameForFollowerType",
                "ShowArtifactFrame",
                "ShowArtifactRelicForgeFrame",
                "ShowAuctionHouseFrame",
                "ShowBarberShopFrame",
                "ShowBlackMarketFrame",
                "ShowChallengesKeystoneFrame",
                "ShowFlightMapFrame",
                "ShowGarrisonCapacitiveDisplayFrame",
                "ShowGarrisonMissionFrameForFollowerType",
                "ShowGarrisonRecruiterFrame",
                "ShowGarrisonShipyardFrame",
                "ShowGuildBankFrame",
                "ShowHeirloomsJournalToClosestUpgradeablePage",
                "ShowInstanceBootDialog",
                "ShowInstanceLockDialog",
                "ShowItemSocketingFrame",
                "ShowItemUpgradeFrame",
                "ShowMatchCelebrationPartyPoseFrame",
                "ShowPendingPlayerChoiceResponseUI",
                "ShowPerksProgramFrame",
                "ShowProfessionEquipmentHelpTip",
                "ShowProfessionsCustomerOrdersFrame",
                "ShowProfessionsFrame",
                "ShowQuestSessionGroupInviteConfirmation",
                "ShowQuestSessionGroupInviteReceivedConfirmation",
                "ShowRemixArtifactFrame",
                "ShowRuneforgeFrame",
                "ShowSummonConfirmationDialog",
                "ShowTaxiMapFrame",
                "SimpleCheckout_EscapePressed",
                "SocialUIContactsFrameInitializeAADC",
                "SoulbindViewer_LoadUI",
                "SpellFlyout_EscapePressed",
                "SplashFrame_EscapePressed",
                "StoreEscapePressed",
                "StringUtil.JoinAlternatingConditionalColor",
                "ToggleRAFPanel",
                "ToggleSocialUI",
                "TryShowAnimaDiversionFrame",
                "TryShowCovenantPreviewFrame",
                "UnitPopupSharedUtil.IsFriendshipUpgrade",
                "UpdateQuestAcceptLogFullDialog",
                "VisualAlert_GetTypeTemplate",
                "VisualAlert_GetTypeText",
                "VisualAlertData_ForEach",
                "VisualAlerts_RegisterAll",
                "WarfrontsPartyPoseFrame_TryShow",
                "WowSurveyStatusFrame_OnSurveyDelivered",
            }
            local published = {}
            for _, name in ipairs(names) do
                local namespaceName, methodName = string.match(name, "^([^.]+)%.(.+)$")
                local value
                if namespaceName then
                    local namespace = _G[namespaceName]
                    value = namespace and namespace[methodName]
                else
                    value = _G[name]
                end
                if value ~= nil then
                    table.insert(published, name)
                end
            end
            return table.concat(published, ",")
            "#,
        )
        .expect("source-absent runtime probe succeeds");

    assert_eq!(published_symbols, "", "unexpected PTR publications");
}
