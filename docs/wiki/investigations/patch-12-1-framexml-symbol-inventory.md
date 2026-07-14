# Patch 12.1 FrameXML Symbol Inventory

Exhaustive status inventory for the local 12.1 FrameXML API snapshot. This page prevents the broad 12.1 audit from silently treating parsed global APIs as the whole source scope.

## Content

**Scope:** 320 added entries and 112 removed entries from `/tmp/warcraft_12_1_framexml.json` (432 entries total). `MacroFrame_SaveMacro` and `PlayerChoiceToggle_TryShow` occur in both source lists, so the snapshot represents 430 distinct names.

**Status rule:** Each entry is classified independently. `implemented` and `best-effort` rows name their modeled behavior and focused coverage; remaining `exception-requested` rows still require strict unsafe/impossible re-triage. Vendor presence alone is not focused behavioral coverage.

### Added symbols

| Symbol | Status | Reason |
|---|---|---|
| `AddBehavioralMessagingTrayToStatusFrames` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `AddFriendFrame_Show` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `AddGMChatStatusFrameToStatusFrames` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `AddTicketStatusFrameToStatusFrames` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `AddWowSurveyStatusFrameToStatusFrames` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `AlliedRacesFrame_TryShow` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `AnchorUtil.ApplyFlowLayout` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ApplySecureDelegatesToTable` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ArchaeologyFrame_ToggleUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ArcheologyDigsiteProgressBar_OnSurveyCast` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ArdenwealdGardening_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ArtifactFrame_OnTraitsRefunded` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `AuraUtil.GetAuraBorderColor` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `AuraUtil.GetUnitAuras` | best-effort | Delegates to existing state-backed aura collection; focused 12.1 bridge test. |
| `AuraUtil.IsValidFilterString` | best-effort | Rust validator accepts nonempty `HELPFUL`, `HARMFUL`, `RAID`, `INCLUDE_NAME_PLATE_ONLY`, and `PLAYER` token combinations; focused 12.1 bridge test. |
| `AzeriteEmpoweredItemUI_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `AzeriteEssenceUI_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `BattlefieldMap_ToggleUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `BehavioralMessaging_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `BehavioralMessagingTray_OnNotification` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `Blizzard_HousingCatalogUtil.AddDecorEntryTooltipTrackingText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `Blizzard_HousingCatalogUtil.TrackHousingDecorID` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `BNet_GetBattleTagComponents` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `BNet_GetBattleTagSelf` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `BNet_GetBroadcastTextSelf` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `BNet_GetFriendLevelRank` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `BNet_IsFriendLevelEqualOrHigher` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `BoostTutorial_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ChallengeModeCompleteBanner_OnChallengeModeCompleted` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ChatAdditionalColor_OpenColorPicker` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ChatFrameUtil.DiscordNameColorize` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ChatFrameUtil.FormatDiscordMessage` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ChatFrameUtil.GetNameForDiscordMessage` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CheckActiveStoreForFree` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatAudioAlertUtil.EnumerateInterruptCastInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatAudioAlertUtil.EnumerateInterruptCastSuccessInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatAudioAlertUtil.EnumerateSayCombatEndInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatAudioAlertUtil.EnumerateSayCombatStartInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatAudioAlertUtil.EnumeratetWhenTargetDiesInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatAudioAlertUtil.GetInterruptCastInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatAudioAlertUtil.GetInterruptCastSuccessInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatAudioAlertUtil.GetSayCombatEndInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatAudioAlertUtil.GetSayCombatStartInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatAudioAlertUtil.GetWhenTargetDiesInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CombatText_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CompactUnitFrame_GetOptionDispelIndicatorOverlayAnimation` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CompactUnitFrame_GetOptionDispelIndicatorOverlayType` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CompactUnitFrameLayoutTemplates_LayoutFrameElement` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CompactUnitFrameUtil.ApplyConfig` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CompactUnitFrameUtil.GenerateNewConfig` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ConfirmDisenchantRollDialog_Show` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ConfirmLootRollDialog_Show` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ConfirmTalentWipeDialog_Show` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ContributionCollectionFrame_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CooldownManagerLayout_GetGroupBuffVisualAlerts` | best-effort | Delegates to existing C_UnitAuras group-buff preference state; focused 12.1 bridge test. |
| `CooldownManagerLayout_GetHiddenGroupBuffs` | best-effort | Delegates to existing C_UnitAuras group-buff preference state; focused 12.1 bridge test. |
| `CooldownManagerLayout_SetGroupBuffVisualAlerts` | best-effort | Delegates to existing C_UnitAuras group-buff preference state; focused 12.1 bridge test. |
| `CooldownManagerLayout_SetHiddenGroupBuffs` | best-effort | Delegates to existing C_UnitAuras group-buff preference state; focused 12.1 bridge test. |
| `CooldownViewer_MarkAuraCacheDirty` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CooldownViewerContextMenu_AddAlertEntryButton` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CooldownViewerContextMenu_AddNewAlertButton` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CooldownViewerDraggedItem_Clear` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CooldownViewerDraggedItem_Pickup` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CooldownViewerDraggedItem_SetIsLegalTarget` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CooldownViewerUtil.AddSoundAlertRadio` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CooldownViewerUtil.BuildSoundMenus` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CooldownViewerUtil.GetSoundTypeSoundKit` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CooldownViewerUtil.GetSoundTypeText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `CovenantCallings_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `DebugTools_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `DifficultyUtil.GetCreatureDifficultyColor` | best-effort | 12.1 post-load dynamic delegate to the authoritative vendor global; preserves arguments, both return values, and later hotfix replacement. |
| `DifficultyUtil.GetDifficultyColor` | best-effort | 12.1 post-load dynamic delegate to the authoritative vendor global; preserves arguments, both return values, and later hotfix replacement. |
| `DifficultyUtil.GetQuestDifficultyColor` | best-effort | 12.1 post-load dynamic delegate to the authoritative vendor global; preserves arguments, both return values, and later hotfix replacement. |
| `DifficultyUtil.GetRelativeDifficultyColor` | best-effort | 12.1 post-load dynamic delegate to the authoritative vendor global; focused test covers `(10, 15)`, two-return fidelity, and hot-swap behavior. |
| `DifficultyUtil.GetScalingQuestDifficultyColor` | best-effort | 12.1 post-load dynamic delegate to the authoritative vendor global; preserves arguments, both return values, and later hotfix replacement. |
| `EditModeManagerFrame_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `EncounterJournal_SetTabVisibe` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `EventTrace_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ExpansionTrial_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FadingFrame_CopyTextScalingTime` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FadingFrame_GetTextScalingMinHeight` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FadingFrame_InitSlot` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FadingFrame_SetTextScaling` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FadingFrame_StartTextScaling` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FadingFrame_StopTextScaling` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FadingFrame_UpdateTextScaling` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.BuildCharacterClassDisplayText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.BuildCharacterLevelDisplayText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.BuildCharacterNameDisplayText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.BuildFriendNameDisplayText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.BuildLocationDisplayText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.BuildTooltipBroadcastText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GameStateUsesFactions` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetBattleNetFriendGameAccountInfoIfExactlyOneDirectInviteTargetExists` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetBattleNetFriendInviteInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetBattleNetFriendInviteTypeLabel` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetBattleNetFriendPartyInviteRestrictionText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetBattleNetFriendPartyInviteRestriction` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetFormattedCharacterName` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetFriendAccountNameText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetFriendNameColorForFriendType` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetFriendNameDisplayColor` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetFriendNameOfflineDisplayColor` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetGameAccountPartyInviteRestriction` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetLastOnlineText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetRegionName` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.GetRelativeTimeText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.HasMultipleGameAccounts` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.InviteOrRequestToJoin` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.IsPlayingDifferentWoWProject` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.IsPlayingSameWoWProject` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.IsPlayingWoW` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.IsRequestInviteType` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.IsTitleFriend` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `FriendsListUtil.ShouldShowRichPresenceOnly` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GameMenuFrame_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GameMenuFrame_IsShown` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GameMenuFrame_Show` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GameRulesUtil.IsPlayerAtEffectiveMaxLevel` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetBottomManagedFrameContainer` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetChatAdditionalColor` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetDiscordUserCommunityLink` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetDiscordUserLink` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetGarrisonMissionFrameNameForFollowerType` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetGarrisonTypeForFollowerType` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetPlayerBottomManagedFrameContainer` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetRightManagedFrameContainer` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetTimeSinceLastQuestProgress` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetTimeStringFromSeconds` | best-effort | Classified as snapshot cross-flavor contamination: the only PTR definition is in `Mists/UIParent.lua`, loaded exclusively by `Blizzard_UIParent_Mists.toc`; PTR tests verify absence during initialization, post-load compatibility, and settled mainline startup. |
| `GetUIPanelLayoutAttribute` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GetUIPanelLayoutFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GMChatFrame_OnWhisperFromGM` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GossipConfirmDialog_Show` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlDiscord_Loaded_OnEvent` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlDiscord_Loaded_OnLoad` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlDiscord_SetGuildSettingsCheckboxes` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlRankDiscord_OnLoad` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlUI_Discord_HideAll` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlUI_Discord_Update` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlUI_DiscordFrame_OnLoad` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlUI_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlUI_OnShow` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlUI_SetupDiscord` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlUI_SetupSelected` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlUI_Setup` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlUI_Show` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `GuildControlUI_UnlinkDiscord` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HandleQuestSessionInviteToPartyConfirmation` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HelpFrame_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HideAuctionHouseFrame` | best-effort | Classified as snapshot/runtime mismatch: no definition exists in the local PTR Blizzard sources; focused PTR runtime test loads `Blizzard_AuctionHouseUI` and verifies the wrapper remains absent rather than inventing behavior. |
| `HideBarberShopFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HideBlackMarketFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HideGarrisonMissionFrames` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HideGarrisonShipyardFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HideGossipFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HideGuildBankFrame` | best-effort | Classified as snapshot/runtime mismatch: no definition exists in local PTR Blizzard Lua sources; focused PTR test explicitly loads `Blizzard_GuildBankUI` and verifies the wrapper remains absent. |
| `HideInstanceBootDialog` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HideInstanceLockDialog` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HideItemUpgradeFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HideProfessionsCustomerOrdersFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HideSummonConfirmationDialogs` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HouseFinderFrame_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HousingBulletinBoardFrame_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HousingControls_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HousingFramesUtil.IsBlueprintCollectionAvailable` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HousingFramesUtil.IsBlueprintOperationInProgress` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HousingFramesUtil.ShowBlueprintExport` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HousingFramesUtil.ShowBlueprintImport` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HousingFramesUtil.ShowBlueprintRoomExport` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HousingFramesUtil.TryOpenBlueprintCollection` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `HybridMinimap_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InputBoxInstructions_OnEnter` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InputBoxInstructions_OnLeave` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InputBoxInstructions_ShowTooltipIfTruncated` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InputUtil.CursorOnUpdate` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InputUtil.CursorUpdate` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InputUtil.GetCursorDelta` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InputUtil.IsMouseOver` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InputUtil.ShowInspectCursor` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InterfaceUtil.GetScreenHeightScale` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InterfaceUtil.GetScreenWidthScale` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `InterpolatorUtil.GetSmoothProgressChange` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `IslandsPartyPoseFrame_TryShow` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `IsMouseoverCastSupported` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `IsSummonConfirmationDialogVisible` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `IsTypeAdditionalChatColor` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ItemUtil.DisplayEquipSlotTooltip` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ItemUtil.GetEmptyEquipSlotTooltipForSlotName` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ItemUtil.GetEmptyEquipSlotTooltip` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ItemUtil.GetEquipSlotTexture` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ItemUtil.GetValidatedItemLocation` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `Kiosk_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `KioskFrame_HandlePlayerEnteringWorld` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `LandingSoulbinds_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `LFGListApplicationViewer_OpenEditMode` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `LFGListApplicationViewerRemoveEntryButton_OnClick` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `LoadAddOnWithErrorHandling` | implemented | 12.1 wrapper in `src/ptr/compat_bootstrap.lua`; focused delegation test in `global_functions.rs`. |
| `LocaleUtil.GetLocaleDisplayName` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `LocalizePlayerFrame_zhCN` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `LocalizePlayerFrame_zhTW` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `LootFrame_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `MacroFrame_SaveMacro` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ManageFramePositions` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `MenuUtil.CreateHighlightButton` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `MovePad_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.CreateNarrationInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.GetCheckboxContext` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.MakeIndexInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.MakeNarrationStringForMoney` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.MakeNarrationStringFromIndexInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.MakeNarrationStringFromInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.MakeNarrationString` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.NarrateCurrentScreen` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.RegionToNarrationInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.ResolveForwardedRegion` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.SetStaticDescription` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.SetStaticName` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.ShouldBeEnabled` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NarrationUtil.ShouldRegionNavigationSkipTooltips` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `NPE_InitializeIfLoaded` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `OpacityFrame_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `OpenEncounterJournalToJourney` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `OpenOrderHallTalentUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `OpenPlayerSpellsToGlyphTarget` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `PhotoSharingFrame_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `PingUtil.SendMacroPing` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `PingUtil.TogglePingTarget` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `PlayerChoiceFrame_TryShow` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `PlayerChoiceToggle_TryShow` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `PVPUI_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `RaidWarningUtil.AddMessage` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `RaidWarningUtil.ClearBossEmotes` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `RaidWarningUtil.UpdateCenterScreenAnchors` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `RecentAlliesUtil.GetBestSocialUIPresenceTypeForStateData` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `RecruitAFriendFrameSocialInitializeAADC` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `RegionUtil.GetTopLeftMost` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `RegionUtil.SortByTopLeft` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `RegisterGameMenuEscHandler` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `RegisterPlayerInteraction` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ReportFrame_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ResetDiscordSettings` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `RestoreGMChatFrameSession` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SetGhostFrameShown` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SetPlayerInteractionConditions` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SettingsPanel_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SetUIPanelLayoutAttribute` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShakeFrameRandom` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShakeFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShouldDisplaySpellCooldown` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowAchievementFrameForAchievement` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowAdventureMapFrameForFollowerType` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowArtifactFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowArtifactRelicForgeFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowAuctionHouseFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowBarberShopFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowBlackMarketFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowChallengesKeystoneFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowFlightMapFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowGarrisonCapacitiveDisplayFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowGarrisonMissionFrameForFollowerType` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowGarrisonRecruiterFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowGarrisonShipyardFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowGuildBankFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowHeirloomsJournalToClosestUpgradeablePage` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowInstanceBootDialog` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowInstanceLockDialog` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowItemSocketingFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowItemUpgradeFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowMatchCelebrationPartyPoseFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowPendingPlayerChoiceResponseUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowPerksProgramFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowProfessionEquipmentHelpTip` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowProfessionsCustomerOrdersFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowProfessionsFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowQuestSessionGroupInviteConfirmation` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowQuestSessionGroupInviteReceivedConfirmation` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowRemixArtifactFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowRuneforgeFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowSummonConfirmationDialog` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ShowTaxiMapFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SimpleCheckout_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIContactsFrameInitializeAADC` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.AddSeparatorToTooltip` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.GetBattleNetFriendTagInterestsUIOrder` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.GetBattleNetFriendTagRoleUIOrder` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.GetBlockedName` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.GetIconForPresenceType` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.GetLabelForBattleNetFriendTag` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.GetLabelForPresenceType` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.GetPresenceTypeForBattleNetAccountInfo` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.GetPresenceTypeSelf` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.InitializeUserScaledDropdownButton` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.InitializeUserScaledDropdownMainTitle` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.InitializeUserScaledDropdownTitle` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SocialUIUtil.SetBattleNetPresenceFromSocialUIPresence` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SoulbindViewer_LoadUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SpellFlyout_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `SplashFrame_EscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `StoreEscapePressed` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `StringUtil.JoinAlternatingConditionalColor` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `TextureUtil.AnimateTexCoords` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `TimeUtil.BetterDate` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `TimeUtil.GetRecentTimeDate` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ToggleRAFPanel` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `ToggleSocialUI` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `TryShowAnimaDiversionFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `TryShowCovenantPreviewFrame` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `UIModeUtil.CreateExtendedBlocklist` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `UIModeUtil.CreateModifiedBlocklist` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `UIModeUtil.IsModeActive` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `UIModeUtil.RegisterMode` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `UIModeUtil.SetModeActive` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `UnitPopupSharedUtil.IsFriendshipUpgrade` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `UpdateQuestAcceptLogFullDialog` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `VisualAlert_GetTypeTemplate` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `VisualAlert_GetTypeText` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `VisualAlertData_ForEach` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `VisualAlerts_RegisterAll` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `WarfrontsPartyPoseFrame_TryShow` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |
| `WowSurveyStatusFrame_OnSurveyDelivered` | exception-requested | FrameXML ownership, load-on-demand timing, and behavior not individually modeled/tested. |

### Removed symbols

| Symbol | Status | Reason |
|---|---|---|
| `AddFrameLock` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `AnimatedShine_OnUpdate` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `AnimateTexCoords` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `BattleTagInviteFrame_Show` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `BetterDate` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `BFAMissionFrame_EscapePressed` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `BoostTutorial_AttemptLoad` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `BuildColoredListString` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `BuildIconArray` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `BuildListString` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `BuildMultilineTooltip` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `BuildNewLineListString` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ButtonPulse_OnUpdate` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ClassTrainerFrame_Hide` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ClassTrainerFrame_Show` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `CloseCalendarMenus` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `CommunitiesFrame_IsEnabled` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `CompactUnitFrame_GetOptionDisplayOnlyHealerPowerBars` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `CompactUnitFrame_GetOptionDisplayPowerBar` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `CompactUnitFrame_GetOptionShowDispelIndicatorOverlay` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ConvertRGBtoColorString` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `DisplayTypeUnassignedSupported` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `EventUtil.AreVariablesLoaded` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ExpansionTrial_CheckLoadUI` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `FriendsFrame_CloseQuickJoinHelpTip` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `FriendsFrameAddFriendButton_OnClick` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `FriendsFriends_InitButton` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `FriendsFriends_SetSelection` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `FriendsFriendsButton_SetSelected` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `FriendsFriendsFrame_Close` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetDungeonNameWithDifficulty` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetNotchHeight` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetScaledCursorDelta` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetScaledCursorPositionForFrame` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetScaledCursorPosition` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetScreenHeightScale` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetScreenWidthScale` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetSmoothProgressChange` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetSocialColoredName` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetSortedSelfResurrectOptions` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `GetUIParentOffset` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `HelpPlatesSupported` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `HousingControlsUtil.CanActivateHousingControls` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `IsFrameLockActive` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `IsFrameSmartShown` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `IsLevelAtEffectiveMaxLevel` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `IsPlayerAtEffectiveMaxLevel` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `KeyBindingFrame_LoadUI` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `LocalizePlayerFrame` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `LocalizezhCN` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `LocalizezhTW` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `MacroFrame_SaveMacro` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `MajorFactions_LoadUI` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `MouseIsOver` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `NPE_CheckTutorials` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `NPETutorial_AttemptToBegin` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `OpenAchievementFrameToAchievement` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `OrderHallMissionFrame_EscapePressed` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `OrderHallTalentFrame_EscapePressed` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `OutfitterUI_LoadUI` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `PingUtil.GetContextualPingTypeForUnit` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `PlayerChoiceToggle_TryShow` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `QuickJoin_JoinQueueButtonOnClick` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidBossEmoteFrame_OnEvent` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidBossEmoteFrame_OnLoad` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidBrowser_IsEmpowered` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidNotice_AddMessage` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidNotice_ClearSlot` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidNotice_Clear` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidNotice_FadeInit` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidNotice_OnUpdate` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidNotice_SetSlot` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidNotice_UpdateSlot` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidWarningFrame_OnEvent` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RaidWarningFrame_OnLoad` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RecentTimeDate` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RefreshAuras` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RegisterNewFrameLock` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `RemoveFrameLock` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ReverseQuestObjective` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `SetDesaturation` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `SetFrameLock` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ShowResurrectRequest` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `SmartHide` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `SmartShow` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `TalentFrame_LoadUI` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `TargetFrame_UpdateBuffAnchor` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `TargetFrame_UpdateDebuffAnchor` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ToggleLFGFrame` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ToggleRafPanel` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ToggleRaidBrowser` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `ToggleWoWHackCharacterUI` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `TokenFrame_LoadUI` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `TrialAccountCapReached_Inform` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UIDoFramesIntersect` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UIParent_ManageFramePositions` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UIParent_OnEvent` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UIParent_OnHide` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UIParent_OnLoad` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UIParent_OnShow` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UIParent_Shared_OnEvent` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UIParent_Shared_OnLoad` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UIParent_UpdateTopFramePositions` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UIParentLoadAddOn` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UnitHasMana` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UpdateFrameLock` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `UpdateUIElementsForClientScene` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `WorldFrame_OnLoad` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `WorldFrame_OnUpdate` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `WoWHackSpellsUI_LoadUI` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `getglobal` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |
| `setglobal` | exception-requested | Strict removal requires current Blizzard UI call-site/lifecycle proof; do not hide a legacy symbol solely from this snapshot. |

## Sources

- `/tmp/warcraft_12_1_framexml.json` — exact local added/removed symbol arrays.
- `src/ptr/compat_bootstrap.lua` — implemented `LoadAddOnWithErrorHandling` wrapper and post-load dynamic `DifficultyUtil` color delegates.
- `src/loader/tests/wow_api_globals/global_functions.rs` — focused `LoadAddOnWithErrorHandling` wrapper regression.
- `src/ptr/compat_bootstrap.rs` — focused reset, argument/return, hot-swap, missing-global, and preservation coverage for the five `DifficultyUtil` delegates.
- `tests/blizzard_frame_xml_util_loads.rs` — full PTR Game UI lifecycle/threshold proof and older-retail non-exposure proof.

## See Also

- [[patch-12-1-api-audit]] — patch-level implementation and exception matrix.
- [[client-profiles]] — cumulative retail epoch feature gating.
- [[lua-api]] — runtime Lua surface context.
