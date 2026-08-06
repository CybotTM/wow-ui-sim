# Patch 12.0.7 Occurrence Inventory
Occurrence-level register for the committed 12.0.7 API-change source. Every named occurrence has an evidence-backed classification; crawler-omitted unnamed claims remain source metadata and are not invented.
## Content
- **Source:** `data/patch-api/sources/12.0.7-register.json`
- **Source SHA-256:** `389e3b19174bf77c3646028f764cf186ccfe1b7dddaca2a3b3fcba75e3bdec60`
- **Target:** retail build `12.0.7`
- **Rows:** 131 total — 34 implemented, 97 best-effort, 0 exception-requested, 0 untriaged
- **Directions:** 79 added, 29 changed, 23 removed

| Symbol | Status | Category | Direction | Note |
|---|---|---|---|---|
| `C_BattleNet.InviteFriend` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_Container.CalculateTotalNumberOfFreeBagSlots` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_DelvesUI.GetDelveEntranceTitleString` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_DelvesUI.GetWorldTierDifficultyForActivePlayer` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_DurationUtil.CreateDurationTextBinding` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_DurationUtil.CreateManualClock` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_EncounterTimeline.GetEventColor` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_HousingCatalog.GetCatalogCategoryAndSubcategoryNames` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_HousingCustomizeMode.RoomConnectionSupportsDoorType` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_HousingLayout.CanSetViewedFloor` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_MerchantFrame.GetMerchantCurrencies` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_PartyInfo.ConfirmReadyCheck` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_PartyInfo.DemoteAssistant` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_PartyInfo.DoReadyCheck` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_PartyInfo.IsGUIDInGroup` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_PartyInfo.PromoteToAssistant` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_PartyInfo.PromoteToLeader` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_PartyInfo.SetEveryoneIsAssistant` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_PartyInfo.UninviteUnit` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_PingSecure.ClearPendingPingOffScreenCallback` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_PingSecure.SetPendingPingOffScreenCallback` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_QuestHub.GetDragonridingRacesForAreaPOI` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_UIFileAsset.GetFileID` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_UIFileAsset.IsKnownFile` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `C_UIFileAsset.IsLooseFile` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `GetEventCPUUsage` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `GetFunctionCPUUsage` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `GetScriptCPUUsage` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `GetSecurePendingButtonCallback` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `GetSecurePendingPingOffScreenCallback` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `GetSecurePendingToggleRunCallback` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `GameTooltip_AddMoneyLine` | best-effort | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `SetSecurePendingButtonCallback` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `SetSecurePendingPingOffScreenCallback` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `SetSecurePendingToggleRunCallback` | implemented | global-api | added | Behavior is covered by the profile-gated safe bridge test; exact Blizzard service/payload fidelity is best-effort where noted. |
| `DurationClock.GetTime` | implemented | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationManualClock.AdvanceTime` | implemented | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationManualClock.ResetTime` | implemented | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationManualClock.RewindTime` | implemented | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationManualClock.SetTime` | implemented | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationObject.GetClock` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationObject.HasExpired` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationObject.HasStarted` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationObject.IsActive` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationObject.SetClock` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.CanFormatText` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.CanUpdateFontString` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.Disable` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.Enable` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.GetDuration` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.GetExpiredText` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.GetFontString` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.GetFormattedText` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.GetTimeModifier` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.GetUpdateInterval` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.GetZeroDurationText` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.IsEnabled` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.SetDuration` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.SetExpiredText` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.SetFontString` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.SetTimeModifier` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.SetUpdateInterval` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextBinding.SetZeroDurationText` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextFormattingOptions.GetAddRemainingText` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextFormattingOptions.GetDurationType` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextFormattingOptions.SetAddRemainingText` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextFormattingOptions.SetDurationType` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextRawValue.GetMilliseconds` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextRawValue.GetSeconds` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextRawValue.SetMilliseconds` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `DurationTextRawValue.SetSeconds` | best-effort | script-object-method | added | Duration clock methods are directly state-tested; duration object/text binding methods are compatibility best-effort. |
| `ENCOUNTER_TIMELINE_EVENT_COLOR_CHANGED` | best-effort | event | added | Registration is tested; live payload/firing semantics remain best-effort. |
| `URL_TEXTURE_REQUEST_RESULT` | best-effort | event | added | Registration is tested; live payload/firing semantics remain best-effort. |
| `assistedCombatReduceHighlights` | implemented | cvar | added | Implemented profile-gated default 1; crawler-omitted CVar claims remain outside occurrence rows. |
| `developerLogFilterDebug` | implemented | cvar | added | Implemented profile-gated default 0; crawler-omitted CVar claims remain outside occurrence rows. |
| `developerLogFilterError` | implemented | cvar | added | Implemented profile-gated default 1; crawler-omitted CVar claims remain outside occurrence rows. |
| `developerLogFilterFatal` | implemented | cvar | added | Implemented profile-gated default 1; crawler-omitted CVar claims remain outside occurrence rows. |
| `developerLogFilterNormal` | implemented | cvar | added | Implemented profile-gated default 1; crawler-omitted CVar claims remain outside occurrence rows. |
| `developerLogFilterSpam` | implemented | cvar | added | Implemented profile-gated default 0; crawler-omitted CVar claims remain outside occurrence rows. |
| `Button.GetButtonState` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `Button.IsEnabled` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `Button.SetButtonState` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `Button.SetEnabled` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `EditBox.SetFont` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `Font.SetFont` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `FontString.SetFont` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `MessageFrame.SetFont` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `ModelSceneActorBase.GetModelUnitGUID` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `ScrollFrame.GetHorizontalScroll` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `ScrollFrame.GetVerticalScroll` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `ScrollFrame.SetHorizontalScroll` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `ScrollFrame.SetVerticalScroll` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `SimpleHTML.SetFont` | best-effort | widget-method | changed | Compatibility presence is tested; exact changed secret/aspect semantics remain best-effort. |
| `CHAT_MSG_COMBAT_FACTION_CHANGE` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CHAT_MSG_COMBAT_HONOR_GAIN` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CHAT_MSG_COMBAT_MISC_INFO` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CHAT_MSG_COMBAT_XP_GAIN` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CHAT_MSG_CURRENCY` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CHAT_MSG_FILTERED` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CHAT_MSG_LOOT` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CHAT_MSG_MONEY` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CHAT_MSG_RESTRICTED` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CLUB_MEMBER_ADDED` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CLUB_MEMBER_PRESENCE_UPDATED` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CLUB_MEMBER_REMOVED` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CLUB_MEMBER_ROLE_UPDATED` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `CLUB_MEMBER_UPDATED` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `ENCOUNTER_END` | best-effort | event | changed | Registration is tested; changed payload and secret-lockdown semantics remain best-effort. |
| `BNInviteFriend` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `C_ClickBindings.GetStringFromModifiers` | best-effort | global-api | removed | Patch list marks removal, but simulator retains a compatibility function to support loaded UI. |
| `C_ClickBindings.MakeModifiers` | best-effort | global-api | removed | Patch list marks removal, but simulator retains a compatibility function to support loaded UI. |
| `C_Spell.GetMawPowerBorderAtlasBySpellID` | best-effort | global-api | removed | Patch list marks removal, but simulator retains a compatibility function to support loaded UI. |
| `ConfirmReadyCheck` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `DemoteAssistant` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `DoReadyCheck` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `GetMerchantCurrencies` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `IsGUIDInGroup` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `PromoteToAssistant` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `PromoteToLeader` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `SetEveryoneIsAssistant` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `UninviteUnit` | best-effort | global-api | removed | Patch list marks removal, but simulator retains a compatibility function to support loaded UI. |
| `GetAutoCompletePresenceID` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `GetAutoCompleteResults` | best-effort | global-api | removed | Patch list marks removal, but simulator retains a compatibility function to support loaded UI. |
| `GetAutoCompleteRealms` | best-effort | global-api | removed | Patch list marks removal, but simulator retains a compatibility function to support loaded UI. |
| `IsRecognizedName` | best-effort | global-api | removed | Startup removal behavior is tested; exact Blizzard load-order timing remains best-effort. |
| `Minimap.SetBlipTexture` | best-effort | widget-method | removed | Patch list marks removal, but method remains as compatibility surface; exact vendor lifecycle semantics are best-effort. |
| `Minimap.SetCorpsePOIArrowTexture` | best-effort | widget-method | removed | Patch list marks removal, but method remains as compatibility surface; exact vendor lifecycle semantics are best-effort. |
| `Minimap.SetIconTexture` | best-effort | widget-method | removed | Patch list marks removal, but method remains as compatibility surface; exact vendor lifecycle semantics are best-effort. |
| `Minimap.SetPOIArrowTexture` | best-effort | widget-method | removed | Patch list marks removal, but method remains as compatibility surface; exact vendor lifecycle semantics are best-effort. |
| `Minimap.SetPlayerTexture` | best-effort | widget-method | removed | Patch list marks removal, but method remains as compatibility surface; exact vendor lifecycle semantics are best-effort. |
| `Minimap.SetStaticPOIArrowTexture` | best-effort | widget-method | removed | Patch list marks removal, but method remains as compatibility surface; exact vendor lifecycle semantics are best-effort. |
