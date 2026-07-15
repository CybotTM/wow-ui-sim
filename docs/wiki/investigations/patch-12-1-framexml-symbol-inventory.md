# Patch 12.1 FrameXML Symbol Inventory

Exhaustive human-readable status inventory for the local 12.1 FrameXML API snapshot. The machine SSOT is `data/patch-api/12.1-framexml.json`; its generated compact checklist is `docs/generated/patch-12-1-framexml-checklist.md`. This page prevents the broad 12.1 audit from silently treating parsed global APIs as the whole source scope.

## Content

**Scope:** 320 added entries and 112 removed entries from `/tmp/warcraft_12_1_framexml.json` (432 entries total). `MacroFrame_SaveMacro` and `PlayerChoiceToggle_TryShow` occur in both source lists, so the snapshot represents 430 distinct names.

**Status rule:** Each entry is classified independently. `implemented` and `best-effort` rows name their modeled behavior and focused coverage. `untriaged` is neutral draft state, not an exception request. Only individually proven unsafe/impossible rows may become `exception-requested`. Vendor presence alone is not focused behavioral coverage.

**Current totals:** 1 implemented, 96 best-effort, 0 exception-requested, 335 untriaged.

### Added symbols

| Symbol | Status | Reason |
|---|---|---|
| `AddBehavioralMessagingTrayToStatusFrames` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AddFriendFrame_Show` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AddGMChatStatusFrameToStatusFrames` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AddTicketStatusFrameToStatusFrames` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AddWowSurveyStatusFrameToStatusFrames` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AlliedRacesFrame_TryShow` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AnchorUtil.ApplyFlowLayout` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ApplySecureDelegatesToTable` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ArchaeologyFrame_ToggleUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ArcheologyDigsiteProgressBar_OnSurveyCast` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ArdenwealdGardening_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ArtifactFrame_OnTraitsRefunded` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AuraUtil.GetAuraBorderColor` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AuraUtil.GetUnitAuras` | best-effort | Delegates to existing state-backed aura collection; focused 12.1 bridge test. |
| `AuraUtil.IsValidFilterString` | best-effort | Rust validator accepts nonempty `HELPFUL`, `HARMFUL`, `RAID`, `INCLUDE_NAME_PLATE_ONLY`, and `PLAYER` token combinations; focused 12.1 bridge test. |
| `AzeriteEmpoweredItemUI_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AzeriteEssenceUI_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BattlefieldMap_ToggleUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BehavioralMessaging_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BehavioralMessagingTray_OnNotification` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `Blizzard_HousingCatalogUtil.AddDecorEntryTooltipTrackingText` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `Blizzard_HousingCatalogUtil.TrackHousingDecorID` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BNet_GetBattleTagComponents` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BNet_GetBattleTagSelf` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BNet_GetBroadcastTextSelf` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BNet_GetFriendLevelRank` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BNet_IsFriendLevelEqualOrHigher` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BoostTutorial_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ChallengeModeCompleteBanner_OnChallengeModeCompleted` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ChatAdditionalColor_OpenColorPicker` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ChatFrameUtil.DiscordNameColorize` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ChatFrameUtil.FormatDiscordMessage` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ChatFrameUtil.GetNameForDiscordMessage` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CheckActiveStoreForFree` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CombatAudioAlertUtil.EnumerateInterruptCastInfo` | best-effort | Stale snapshot addition: recursive PTR source proof finds no publication, and the active runtime namespace leaves this method nil. |
| `CombatAudioAlertUtil.EnumerateInterruptCastSuccessInfo` | best-effort | Stale snapshot addition: recursive PTR source proof finds no publication, and the active runtime namespace leaves this method nil. |
| `CombatAudioAlertUtil.EnumerateSayCombatEndInfo` | best-effort | Stale snapshot addition: recursive PTR source proof finds no publication, and the active runtime namespace leaves this method nil. |
| `CombatAudioAlertUtil.EnumerateSayCombatStartInfo` | best-effort | Stale snapshot addition: recursive PTR source proof finds no publication, and the active runtime namespace leaves this method nil. |
| `CombatAudioAlertUtil.EnumeratetWhenTargetDiesInfo` | best-effort | Stale snapshot addition: recursive PTR source proof finds no publication, and the active runtime namespace leaves this method nil. |
| `CombatAudioAlertUtil.GetInterruptCastInfo` | best-effort | Stale snapshot addition: recursive PTR source proof finds no publication, and the active runtime namespace leaves this method nil. |
| `CombatAudioAlertUtil.GetInterruptCastSuccessInfo` | best-effort | Stale snapshot addition: recursive PTR source proof finds no publication, and the active runtime namespace leaves this method nil. |
| `CombatAudioAlertUtil.GetSayCombatEndInfo` | best-effort | Stale snapshot addition: recursive PTR source proof finds no publication, and the active runtime namespace leaves this method nil. |
| `CombatAudioAlertUtil.GetSayCombatStartInfo` | best-effort | Stale snapshot addition: recursive PTR source proof finds no publication, and the active runtime namespace leaves this method nil. |
| `CombatAudioAlertUtil.GetWhenTargetDiesInfo` | best-effort | Stale snapshot addition: recursive PTR source proof finds no publication, and the active runtime namespace leaves this method nil. |
| `CombatText_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CompactUnitFrame_GetOptionDispelIndicatorOverlayAnimation` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CompactUnitFrame_GetOptionDispelIndicatorOverlayType` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CompactUnitFrameLayoutTemplates_LayoutFrameElement` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CompactUnitFrameUtil.ApplyConfig` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CompactUnitFrameUtil.GenerateNewConfig` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ConfirmDisenchantRollDialog_Show` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ConfirmLootRollDialog_Show` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ConfirmTalentWipeDialog_Show` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ContributionCollectionFrame_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CooldownManagerLayout_GetGroupBuffVisualAlerts` | best-effort | Delegates to existing C_UnitAuras group-buff preference state; focused 12.1 bridge test. |
| `CooldownManagerLayout_GetHiddenGroupBuffs` | best-effort | Delegates to existing C_UnitAuras group-buff preference state; focused 12.1 bridge test. |
| `CooldownManagerLayout_SetGroupBuffVisualAlerts` | best-effort | Delegates to existing C_UnitAuras group-buff preference state; focused 12.1 bridge test. |
| `CooldownManagerLayout_SetHiddenGroupBuffs` | best-effort | Delegates to existing C_UnitAuras group-buff preference state; focused 12.1 bridge test. |
| `CooldownViewer_MarkAuraCacheDirty` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CooldownViewerContextMenu_AddAlertEntryButton` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CooldownViewerContextMenu_AddNewAlertButton` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CooldownViewerDraggedItem_Clear` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CooldownViewerDraggedItem_Pickup` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CooldownViewerDraggedItem_SetIsLegalTarget` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CooldownViewerUtil.AddSoundAlertRadio` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CooldownViewerUtil.BuildSoundMenus` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CooldownViewerUtil.GetSoundTypeSoundKit` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CooldownViewerUtil.GetSoundTypeText` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CovenantCallings_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `DebugTools_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `DifficultyUtil.GetCreatureDifficultyColor` | best-effort | 12.1 post-load dynamic delegate to the authoritative vendor global; preserves arguments, both return values, and later hotfix replacement. |
| `DifficultyUtil.GetDifficultyColor` | best-effort | 12.1 post-load dynamic delegate to the authoritative vendor global; preserves arguments, both return values, and later hotfix replacement. |
| `DifficultyUtil.GetQuestDifficultyColor` | best-effort | 12.1 post-load dynamic delegate to the authoritative vendor global; preserves arguments, both return values, and later hotfix replacement. |
| `DifficultyUtil.GetRelativeDifficultyColor` | best-effort | 12.1 post-load dynamic delegate to the authoritative vendor global; focused test covers `(10, 15)`, two-return fidelity, and hot-swap behavior. |
| `DifficultyUtil.GetScalingQuestDifficultyColor` | best-effort | 12.1 post-load dynamic delegate to the authoritative vendor global; preserves arguments, both return values, and later hotfix replacement. |
| `EditModeManagerFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `EncounterJournal_SetTabVisibe` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `EventTrace_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ExpansionTrial_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FadingFrame_CopyTextScalingTime` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FadingFrame_GetTextScalingMinHeight` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FadingFrame_InitSlot` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FadingFrame_SetTextScaling` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FadingFrame_StartTextScaling` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FadingFrame_StopTextScaling` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FadingFrame_UpdateTextScaling` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FriendsListUtil.BuildCharacterClassDisplayText` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.BuildCharacterLevelDisplayText` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.BuildCharacterNameDisplayText` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.BuildFriendNameDisplayText` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.BuildLocationDisplayText` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.BuildTooltipBroadcastText` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GameStateUsesFactions` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetBattleNetFriendGameAccountInfoIfExactlyOneDirectInviteTargetExists` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetBattleNetFriendInviteInfo` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetBattleNetFriendInviteTypeLabel` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetBattleNetFriendPartyInviteRestrictionText` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetBattleNetFriendPartyInviteRestriction` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetFormattedCharacterName` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetFriendAccountNameText` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetFriendNameColorForFriendType` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetFriendNameDisplayColor` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetFriendNameOfflineDisplayColor` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetGameAccountPartyInviteRestriction` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetLastOnlineText` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetRegionName` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.GetRelativeTimeText` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.HasMultipleGameAccounts` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.InviteOrRequestToJoin` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.IsPlayingDifferentWoWProject` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.IsPlayingSameWoWProject` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.IsPlayingWoW` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.IsRequestInviteType` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.IsTitleFriend` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `FriendsListUtil.ShouldShowRichPresenceOnly` | best-effort | Stale snapshot addition: recursive PTR proof finds no qualified publication, and `FriendsListUtil` remains nil after startup. |
| `GameMenuFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GameMenuFrame_IsShown` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GameMenuFrame_Show` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GameRulesUtil.IsPlayerAtEffectiveMaxLevel` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetBottomManagedFrameContainer` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetChatAdditionalColor` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetDiscordUserCommunityLink` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetDiscordUserLink` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetGarrisonMissionFrameNameForFollowerType` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetGarrisonTypeForFollowerType` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetPlayerBottomManagedFrameContainer` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetRightManagedFrameContainer` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetTimeSinceLastQuestProgress` | best-effort | Vendor-present in PTRFeedback. Focused proof pins publication and the current upstream nil-arithmetic invocation defect from undefined `lastProgressTime`; no guessed correction is added. |
| `GetTimeStringFromSeconds` | best-effort | Classified as snapshot cross-flavor contamination: the only PTR definition is in `Mists/UIParent.lua`, loaded exclusively by `Blizzard_UIParent_Mists.toc`; PTR tests verify absence during initialization, post-load compatibility, and settled mainline startup. |
| `GetUIPanelLayoutAttribute` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetUIPanelLayoutFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GMChatFrame_OnWhisperFromGM` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GossipConfirmDialog_Show` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlDiscord_Loaded_OnEvent` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlDiscord_Loaded_OnLoad` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlDiscord_SetGuildSettingsCheckboxes` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlRankDiscord_OnLoad` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlUI_Discord_HideAll` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlUI_Discord_Update` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlUI_DiscordFrame_OnLoad` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlUI_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlUI_OnShow` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlUI_SetupDiscord` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlUI_SetupSelected` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlUI_Setup` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlUI_Show` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GuildControlUI_UnlinkDiscord` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HandleQuestSessionInviteToPartyConfirmation` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HelpFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HideAuctionHouseFrame` | best-effort | Classified as snapshot/runtime mismatch: no definition exists in the local PTR Blizzard sources; focused PTR runtime test loads `Blizzard_AuctionHouseUI` and verifies the wrapper remains absent rather than inventing behavior. |
| `HideBarberShopFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HideBlackMarketFrame` | best-effort | Classified as reversed-name snapshot mismatch: PTR defines `BlackMarketFrame_Hide` with panel-hide and close-sound behavior, not `HideBlackMarketFrame`; focused PTR test explicitly loads `Blizzard_BlackMarketUI`, verifies the authoritative helper, and confirms the reversed wrapper remains absent. |
| `HideGarrisonMissionFrames` | best-effort | Classified as snapshot/runtime mismatch: no definition exists in local PTR Blizzard Lua sources; focused PTR test loads `Blizzard_GarrisonUI` with its LoD dependencies and verifies the wrapper remains absent. |
| `HideGarrisonShipyardFrame` | best-effort | Classified as snapshot/runtime mismatch: no definition exists in local PTR Blizzard Lua sources; focused PTR test loads `Blizzard_GarrisonUI` with its LoD dependencies and verifies the wrapper remains absent. |
| `HideGossipFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HideGuildBankFrame` | best-effort | Classified as snapshot/runtime mismatch: no definition exists in local PTR Blizzard Lua sources; focused PTR test explicitly loads `Blizzard_GuildBankUI` and verifies the wrapper remains absent. |
| `HideInstanceBootDialog` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HideInstanceLockDialog` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HideItemUpgradeFrame` | best-effort | Classified as reversed-name snapshot mismatch: PTR defines and uses `ItemUpgradeFrame_Hide`, not `HideItemUpgradeFrame`; focused PTR test explicitly loads `Blizzard_ItemUpgradeUI` and verifies the reversed wrapper remains absent. |
| `HideProfessionsCustomerOrdersFrame` | best-effort | Classified as snapshot/runtime mismatch: recursive local PTR source scan finds no definition; focused PTR test loads the required ProfessionsTemplates/AuctionHouse dependencies plus `Blizzard_ProfessionsCustomerOrders`, verifies the frame exists, and confirms the wrapper remains absent. |
| `HideSummonConfirmationDialogs` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HouseFinderFrame_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HousingBulletinBoardFrame_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HousingControls_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HousingFramesUtil.IsBlueprintCollectionAvailable` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HousingFramesUtil.IsBlueprintOperationInProgress` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HousingFramesUtil.ShowBlueprintExport` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HousingFramesUtil.ShowBlueprintImport` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HousingFramesUtil.ShowBlueprintRoomExport` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HousingFramesUtil.TryOpenBlueprintCollection` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HybridMinimap_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `InputBoxInstructions_OnEnter` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `InputBoxInstructions_OnLeave` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `InputBoxInstructions_ShowTooltipIfTruncated` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `InputUtil.CursorOnUpdate` | best-effort | Stale snapshot namespace move: focused PTR proof keeps the authoritative global behavior and verifies `InputUtil.CursorOnUpdate` remains nil. |
| `InputUtil.CursorUpdate` | best-effort | Stale snapshot namespace move: focused PTR proof keeps the authoritative global behavior and verifies `InputUtil.CursorUpdate` remains nil. |
| `InputUtil.GetCursorDelta` | best-effort | Stale snapshot namespace move: focused PTR proof keeps the authoritative global behavior and verifies `InputUtil.GetCursorDelta` remains nil. |
| `InputUtil.IsMouseOver` | best-effort | Stale snapshot namespace move: focused PTR proof keeps the authoritative global behavior and verifies `InputUtil.IsMouseOver` remains nil. |
| `InputUtil.ShowInspectCursor` | best-effort | Stale snapshot namespace move: focused PTR proof keeps the authoritative global behavior and verifies `InputUtil.ShowInspectCursor` remains nil. |
| `InterfaceUtil.GetScreenHeightScale` | best-effort | Stale snapshot: active PTR has no `InterfaceUtil`; global height-scale helper remains authoritative and returns `1.0` at 768 pixels. |
| `InterfaceUtil.GetScreenWidthScale` | best-effort | Stale snapshot: active PTR has no `InterfaceUtil`; global width-scale helper remains authoritative and returns `1.0` at 1024 pixels. |
| `InterpolatorUtil.GetSmoothProgressChange` | best-effort | Reversed snapshot: PTR UIParent exports global `GetSmoothProgressChange`, not this namespace member; focused PTR proof confirms the member remains nil while the global computes the expected value. |
| `IslandsPartyPoseFrame_TryShow` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `IsMouseoverCastSupported` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `IsSummonConfirmationDialogVisible` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `IsTypeAdditionalChatColor` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ItemUtil.DisplayEquipSlotTooltip` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ItemUtil.GetEmptyEquipSlotTooltipForSlotName` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ItemUtil.GetEmptyEquipSlotTooltip` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ItemUtil.GetEquipSlotTexture` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ItemUtil.GetValidatedItemLocation` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `Kiosk_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `KioskFrame_HandlePlayerEnteringWorld` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `LandingSoulbinds_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `LFGListApplicationViewer_OpenEditMode` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `LFGListApplicationViewerRemoveEntryButton_OnClick` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `LoadAddOnWithErrorHandling` | implemented | 12.1 wrapper in `src/ptr/compat_bootstrap.lua`; focused delegation test in `global_functions.rs`. |
| `LocaleUtil.GetLocaleDisplayName` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `LocalizePlayerFrame_zhCN` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `LocalizePlayerFrame_zhTW` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `LootFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `MacroFrame_SaveMacro` | best-effort | PTR lifecycle proof verifies the eager UIParent no-op placeholder is harmless, then explicit `Blizzard_MacroUI` loading replaces it with the `MacroFrame:SaveMacro()` delegate. |
| `ManageFramePositions` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `MenuUtil.CreateHighlightButton` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `MovePad_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.CreateNarrationInfo` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.GetCheckboxContext` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.MakeIndexInfo` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.MakeNarrationStringForMoney` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.MakeNarrationStringFromIndexInfo` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.MakeNarrationStringFromInfo` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.MakeNarrationString` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.NarrateCurrentScreen` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.RegionToNarrationInfo` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.ResolveForwardedRegion` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.SetStaticDescription` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.SetStaticName` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.ShouldBeEnabled` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NarrationUtil.ShouldRegionNavigationSkipTooltips` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NPE_InitializeIfLoaded` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `OpacityFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `OpenEncounterJournalToJourney` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `OpenOrderHallTalentUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `OpenPlayerSpellsToGlyphTarget` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `PhotoSharingFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `PingUtil.SendMacroPing` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `PingUtil.TogglePingTarget` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `PlayerChoiceFrame_TryShow` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `PlayerChoiceToggle_TryShow` | best-effort | PTR LoD behavior: absent before `Blizzard_PlayerChoice`, present afterward; focused proof verifies eligible button visibility, explicit plus OnShow updates, and nil return. |
| `PVPUI_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidWarningUtil.AddMessage` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidWarningUtil.ClearBossEmotes` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidWarningUtil.UpdateCenterScreenAnchors` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RecentAlliesUtil.GetBestSocialUIPresenceTypeForStateData` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RecruitAFriendFrameSocialInitializeAADC` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RegionUtil.GetTopLeftMost` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RegionUtil.SortByTopLeft` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RegisterGameMenuEscHandler` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RegisterPlayerInteraction` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ReportFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ResetDiscordSettings` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RestoreGMChatFrameSession` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SetGhostFrameShown` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SetPlayerInteractionConditions` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SettingsPanel_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SetUIPanelLayoutAttribute` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShakeFrameRandom` | best-effort | Stale snapshot: active PTR leaves the legacy global nil and publishes distinct `ScriptAnimationUtil.ShakeFrameRandom` behavior; focused no-op proof returns a cancellation function. |
| `ShakeFrame` | best-effort | Stale snapshot: active PTR leaves the legacy global nil and publishes distinct `ScriptAnimationUtil.ShakeFrame` behavior; focused locked-region proof returns a cancellation function. |
| `ShouldDisplaySpellCooldown` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowAchievementFrameForAchievement` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowAdventureMapFrameForFollowerType` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowArtifactFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowArtifactRelicForgeFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowAuctionHouseFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowBarberShopFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowBlackMarketFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowChallengesKeystoneFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowFlightMapFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowGarrisonCapacitiveDisplayFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowGarrisonMissionFrameForFollowerType` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowGarrisonRecruiterFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowGarrisonShipyardFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowGuildBankFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowHeirloomsJournalToClosestUpgradeablePage` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowInstanceBootDialog` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowInstanceLockDialog` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowItemSocketingFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowItemUpgradeFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowMatchCelebrationPartyPoseFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowPendingPlayerChoiceResponseUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowPerksProgramFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowProfessionEquipmentHelpTip` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowProfessionsCustomerOrdersFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowProfessionsFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowQuestSessionGroupInviteConfirmation` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowQuestSessionGroupInviteReceivedConfirmation` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowRemixArtifactFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowRuneforgeFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowSummonConfirmationDialog` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowTaxiMapFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SimpleCheckout_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SocialUIContactsFrameInitializeAADC` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SocialUIUtil.AddSeparatorToTooltip` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.GetBattleNetFriendTagInterestsUIOrder` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.GetBattleNetFriendTagRoleUIOrder` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.GetBlockedName` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.GetIconForPresenceType` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.GetLabelForBattleNetFriendTag` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.GetLabelForPresenceType` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.GetPresenceTypeForBattleNetAccountInfo` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.GetPresenceTypeSelf` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.InitializeUserScaledDropdownButton` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.InitializeUserScaledDropdownMainTitle` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.InitializeUserScaledDropdownTitle` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SocialUIUtil.SetBattleNetPresenceFromSocialUIPresence` | best-effort | Stale snapshot addition: exact-qualified PTR source proof finds no publication, and `SocialUIUtil` remains nil after startup. |
| `SoulbindViewer_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SpellFlyout_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SplashFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `StoreEscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `StringUtil.JoinAlternatingConditionalColor` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `TextureUtil.AnimateTexCoords` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `TimeUtil.BetterDate` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `TimeUtil.GetRecentTimeDate` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ToggleRAFPanel` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ToggleSocialUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `TryShowAnimaDiversionFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `TryShowCovenantPreviewFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIModeUtil.CreateExtendedBlocklist` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIModeUtil.CreateModifiedBlocklist` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIModeUtil.IsModeActive` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIModeUtil.RegisterMode` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIModeUtil.SetModeActive` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UnitPopupSharedUtil.IsFriendshipUpgrade` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UpdateQuestAcceptLogFullDialog` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `VisualAlert_GetTypeTemplate` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `VisualAlert_GetTypeText` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `VisualAlertData_ForEach` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `VisualAlerts_RegisterAll` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `WarfrontsPartyPoseFrame_TryShow` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `WowSurveyStatusFrame_OnSurveyDelivered` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |

### Removed symbols

| Symbol | Status | Reason |
|---|---|---|
| `AddFrameLock` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AnimatedShine_OnUpdate` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `AnimateTexCoords` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BattleTagInviteFrame_Show` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BetterDate` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BFAMissionFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BoostTutorial_AttemptLoad` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BuildColoredListString` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BuildIconArray` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BuildListString` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BuildMultilineTooltip` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `BuildNewLineListString` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ButtonPulse_OnUpdate` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ClassTrainerFrame_Hide` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ClassTrainerFrame_Show` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CloseCalendarMenus` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CommunitiesFrame_IsEnabled` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CompactUnitFrame_GetOptionDisplayOnlyHealerPowerBars` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CompactUnitFrame_GetOptionDisplayPowerBar` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `CompactUnitFrame_GetOptionShowDispelIndicatorOverlay` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ConvertRGBtoColorString` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `DisplayTypeUnassignedSupported` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `EventUtil.AreVariablesLoaded` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ExpansionTrial_CheckLoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FriendsFrame_CloseQuickJoinHelpTip` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FriendsFrameAddFriendButton_OnClick` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FriendsFriends_InitButton` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FriendsFriends_SetSelection` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FriendsFriendsButton_SetSelected` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `FriendsFriendsFrame_Close` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetDungeonNameWithDifficulty` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetNotchHeight` | best-effort | Vendor-present global: focused PTR proof verifies the function remains published and exercises its UI geometry behavior. |
| `GetScaledCursorDelta` | best-effort | Vendor-present global: focused PTR proof verifies the function remains published and exercises its cursor behavior. |
| `GetScaledCursorPositionForFrame` | best-effort | Vendor-present global: focused PTR proof verifies the function remains published and exercises its cursor behavior. |
| `GetScaledCursorPosition` | best-effort | Vendor-present global: focused PTR proof verifies the function remains published and exercises its cursor behavior. |
| `GetScreenHeightScale` | best-effort | Vendor-present despite snapshot removal; focused PTR proof verifies function publication and `1.0` at the 768-pixel fixture height. |
| `GetScreenWidthScale` | best-effort | Vendor-present despite snapshot removal; focused PTR proof verifies function publication and `1.0` at the 1024-pixel fixture width. |
| `GetSmoothProgressChange` | best-effort | Vendor-present despite the snapshot removal: PTR UIParent retains this global function; focused PTR proof verifies representative input `(100, 0, 100, 1)` returns `70`. |
| `GetSocialColoredName` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetSortedSelfResurrectOptions` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `GetUIParentOffset` | best-effort | Vendor-present global: focused PTR proof verifies the function remains published and exercises its UI geometry behavior. |
| `HelpPlatesSupported` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `HousingControlsUtil.CanActivateHousingControls` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `IsFrameLockActive` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `IsFrameSmartShown` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `IsLevelAtEffectiveMaxLevel` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `IsPlayerAtEffectiveMaxLevel` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `KeyBindingFrame_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `LocalizePlayerFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `LocalizezhCN` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `LocalizezhTW` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `MacroFrame_SaveMacro` | best-effort | PTR lifecycle proof verifies the eager UIParent no-op placeholder is harmless, then explicit `Blizzard_MacroUI` loading replaces it with the `MacroFrame:SaveMacro()` delegate. |
| `MajorFactions_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `MouseIsOver` | best-effort | Vendor-present global: focused PTR proof verifies the function remains published and exercises its cursor behavior. |
| `NPE_CheckTutorials` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `NPETutorial_AttemptToBegin` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `OpenAchievementFrameToAchievement` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `OrderHallMissionFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `OrderHallTalentFrame_EscapePressed` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `OutfitterUI_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `PingUtil.GetContextualPingTypeForUnit` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `PlayerChoiceToggle_TryShow` | best-effort | Retained PTR LoD behavior despite snapshot removal: absent before `Blizzard_PlayerChoice`, published afterward with focused button-state proof. |
| `QuickJoin_JoinQueueButtonOnClick` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidBossEmoteFrame_OnEvent` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidBossEmoteFrame_OnLoad` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidBrowser_IsEmpowered` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidNotice_AddMessage` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidNotice_ClearSlot` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidNotice_Clear` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidNotice_FadeInit` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidNotice_OnUpdate` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidNotice_SetSlot` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidNotice_UpdateSlot` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidWarningFrame_OnEvent` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RaidWarningFrame_OnLoad` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RecentTimeDate` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RefreshAuras` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RegisterNewFrameLock` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `RemoveFrameLock` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ReverseQuestObjective` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SetDesaturation` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SetFrameLock` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ShowResurrectRequest` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SmartHide` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `SmartShow` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `TalentFrame_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `TargetFrame_UpdateBuffAnchor` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `TargetFrame_UpdateDebuffAnchor` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ToggleLFGFrame` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ToggleRafPanel` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ToggleRaidBrowser` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `ToggleWoWHackCharacterUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `TokenFrame_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `TrialAccountCapReached_Inform` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIDoFramesIntersect` | best-effort | Vendor-present global: focused PTR proof verifies the function remains published and exercises its UI geometry behavior. |
| `UIParent_ManageFramePositions` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIParent_OnEvent` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIParent_OnHide` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIParent_OnLoad` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIParent_OnShow` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIParent_Shared_OnEvent` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIParent_Shared_OnLoad` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIParent_UpdateTopFramePositions` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UIParentLoadAddOn` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UnitHasMana` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UpdateFrameLock` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `UpdateUIElementsForClientScene` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `WorldFrame_OnLoad` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `WorldFrame_OnUpdate` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `WoWHackSpellsUI_LoadUI` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `getglobal` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |
| `setglobal` | untriaged | Awaiting per-item source, reachability, lifecycle, and behavior triage; no exception requested. |

## Sources

- [Raw 12.1 FrameXML patch list](../../../data/patch-api/sources/12.1-framexml.json) — checked-in exact added/removed symbol arrays.
- [12.1 FrameXML audit register](../../../data/patch-api/12.1-framexml.json) — per-row machine SSOT and lifecycle/evidence metadata.
- [Generated compact checklist](../../generated/patch-12-1-framexml-checklist.md) — one itemized line per patch-list occurrence.
- [PTR compatibility bootstrap](../../../src/ptr/compat_bootstrap.lua) — implemented `LoadAddOnWithErrorHandling` wrapper and post-load dynamic `DifficultyUtil` color delegates.
- [Global-function regressions](../../../src/loader/tests/wow_api_globals/global_functions.rs) — focused `LoadAddOnWithErrorHandling` wrapper regression.
- [PTR compatibility tests](../../../src/ptr/compat_bootstrap.rs) — focused reset, argument/return, hot-swap, missing-global, and preservation coverage for the five `DifficultyUtil` delegates.
- [FrameXMLUtil lifecycle tests](../../../tests/blizzard_frame_xml_util_loads.rs) — full PTR Game UI lifecycle/threshold proof and older-retail non-exposure proof.
- [Garrison UI tests](../../../tests/blizzard_garrison_ui_loads.rs) — recursive PTR Garrison source scan plus explicit LoD addon-load proof for the two absent Garrison hide wrappers.
- [Customer Orders tests](../../../tests/blizzard_professions_customer_orders_loads.rs) — recursive PTR source scan plus explicit dependency/addon-load proof for the absent CustomerOrders hide wrapper.
- [Input utility snapshot proof](../../../patch-tests/patch_12_1/input_util.rs) — focused PTR publication and cursor-behavior proof for the stale namespace move and retained globals.
- [UI geometry snapshot proof](../../../patch-tests/patch_12_1/ui_geometry.rs) — focused PTR intersection, notch normalization, and UIParent offset proof for three retained globals.
- [Combat audio snapshot proof](../../../patch-tests/patch_12_1/combat_audio.rs) — recursive PTR source scan and runtime absence proof for ten stale proposed methods.
- [Friends list snapshot proof](../../../patch-tests/patch_12_1/friends_list.rs) — exact-qualified PTR source scan and runtime absence proof for 29 stale proposed methods.
- [Social UI snapshot proof](../../../patch-tests/patch_12_1/social_ui.rs) — exact-qualified PTR source scan and runtime absence proof for 13 stale proposed methods.

## See Also

- [[patch-api-audit-manifest]] — manifest schema, validation, lifecycle assertions, and checklist generation.
- [[patch-12-1-api-audit]] — patch-level implementation and exception matrix.
- [[client-profiles]] — cumulative retail epoch feature gating.
- [[lua-api]] — runtime Lua surface context.
