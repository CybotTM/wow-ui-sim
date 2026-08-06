# Patch 12.0.7 Occurrence Inventory

Occurrence-level register for the committed 12.0.7 API-change source. Six named CVar rows are implemented from exact profile-gated default evidence; the remaining rows stay `untriaged` until item-specific evidence supports `implemented`, `best-effort`, or an individually justified `exception-requested` status.

## Content

- **Source:** `data/patch-api/sources/12.0.7-register.json`
- **Source SHA-256:** `389e3b19174bf77c3646028f764cf186ccfe1b7dddaca2a3b3fcba75e3bdec60`
- **Target:** retail build `12.0.7`
- **Rows:** 131 total — 6 implemented, 0 best-effort, 0 exception-requested, 125 untriaged
- **Directions:** 79 added, 29 changed, 23 removed

| Symbol | Status | Category | Direction | Note |
|---|---|---|---|---|
| `C_BattleNet.InviteFriend` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_Container.CalculateTotalNumberOfFreeBagSlots` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_DelvesUI.GetDelveEntranceTitleString` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_DelvesUI.GetWorldTierDifficultyForActivePlayer` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_DurationUtil.CreateDurationTextBinding` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_DurationUtil.CreateManualClock` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_EncounterTimeline.GetEventColor` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_HousingCatalog.GetCatalogCategoryAndSubcategoryNames` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_HousingCustomizeMode.RoomConnectionSupportsDoorType` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_HousingLayout.CanSetViewedFloor` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_MerchantFrame.GetMerchantCurrencies` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_PartyInfo.ConfirmReadyCheck` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_PartyInfo.DemoteAssistant` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_PartyInfo.DoReadyCheck` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_PartyInfo.IsGUIDInGroup` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_PartyInfo.PromoteToAssistant` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_PartyInfo.PromoteToLeader` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_PartyInfo.SetEveryoneIsAssistant` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_PartyInfo.UninviteUnit` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_PingSecure.ClearPendingPingOffScreenCallback` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_PingSecure.SetPendingPingOffScreenCallback` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_QuestHub.GetDragonridingRacesForAreaPOI` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_UIFileAsset.GetFileID` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_UIFileAsset.IsKnownFile` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_UIFileAsset.IsLooseFile` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GetEventCPUUsage` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GetFunctionCPUUsage` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GetScriptCPUUsage` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GetSecurePendingButtonCallback` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GetSecurePendingPingOffScreenCallback` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GetSecurePendingToggleRunCallback` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GameTooltip_AddMoneyLine` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `SetSecurePendingButtonCallback` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `SetSecurePendingPingOffScreenCallback` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `SetSecurePendingToggleRunCallback` | untriaged | global-api | added | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `DurationClock.GetTime` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationManualClock.AdvanceTime` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationManualClock.ResetTime` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationManualClock.RewindTime` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationManualClock.SetTime` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationObject.GetClock` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationObject.HasExpired` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationObject.HasStarted` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationObject.IsActive` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationObject.SetClock` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.CanFormatText` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.CanUpdateFontString` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.Disable` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.Enable` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.GetDuration` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.GetExpiredText` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.GetFontString` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.GetFormattedText` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.GetTimeModifier` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.GetUpdateInterval` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.GetZeroDurationText` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.IsEnabled` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.SetDuration` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.SetExpiredText` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.SetFontString` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.SetTimeModifier` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.SetUpdateInterval` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextBinding.SetZeroDurationText` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextFormattingOptions.GetAddRemainingText` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextFormattingOptions.GetDurationType` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextFormattingOptions.SetAddRemainingText` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextFormattingOptions.SetDurationType` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextRawValue.GetMilliseconds` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextRawValue.GetSeconds` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextRawValue.SetMilliseconds` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `DurationTextRawValue.SetSeconds` | untriaged | script-object-method | added | Pending occurrence-level evidence and classification for 12.0.7 script-object-method occurrence. |
| `ENCOUNTER_TIMELINE_EVENT_COLOR_CHANGED` | untriaged | event | added | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `URL_TEXTURE_REQUEST_RESULT` | untriaged | event | added | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `assistedCombatReduceHighlights` | implemented | cvar | added | Profile-gated default `1` verified by `patch_12_0_7_cvar_defaults_match_retail`. |
| `developerLogFilterDebug` | implemented | cvar | added | Profile-gated default `0` verified by `patch_12_0_7_cvar_defaults_match_retail`. |
| `developerLogFilterError` | implemented | cvar | added | Profile-gated default `1` verified by `patch_12_0_7_cvar_defaults_match_retail`. |
| `developerLogFilterFatal` | implemented | cvar | added | Profile-gated default `1` verified by `patch_12_0_7_cvar_defaults_match_retail`. |
| `developerLogFilterNormal` | implemented | cvar | added | Profile-gated default `1` verified by `patch_12_0_7_cvar_defaults_match_retail`. |
| `developerLogFilterSpam` | implemented | cvar | added | Profile-gated default `0` verified by `patch_12_0_7_cvar_defaults_match_retail`. |
| `Button.GetButtonState` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `Button.IsEnabled` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `Button.SetButtonState` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `Button.SetEnabled` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `EditBox.SetFont` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `Font.SetFont` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `FontString.SetFont` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `MessageFrame.SetFont` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `ModelSceneActorBase.GetModelUnitGUID` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `ScrollFrame.GetHorizontalScroll` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `ScrollFrame.GetVerticalScroll` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `ScrollFrame.SetHorizontalScroll` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `ScrollFrame.SetVerticalScroll` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `SimpleHTML.SetFont` | untriaged | widget-method | changed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `CHAT_MSG_COMBAT_FACTION_CHANGE` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CHAT_MSG_COMBAT_HONOR_GAIN` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CHAT_MSG_COMBAT_MISC_INFO` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CHAT_MSG_COMBAT_XP_GAIN` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CHAT_MSG_CURRENCY` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CHAT_MSG_FILTERED` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CHAT_MSG_LOOT` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CHAT_MSG_MONEY` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CHAT_MSG_RESTRICTED` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CLUB_MEMBER_ADDED` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CLUB_MEMBER_PRESENCE_UPDATED` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CLUB_MEMBER_REMOVED` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CLUB_MEMBER_ROLE_UPDATED` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `CLUB_MEMBER_UPDATED` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `ENCOUNTER_END` | untriaged | event | changed | Pending occurrence-level evidence and classification for 12.0.7 event occurrence. |
| `BNInviteFriend` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_ClickBindings.GetStringFromModifiers` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_ClickBindings.MakeModifiers` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `C_Spell.GetMawPowerBorderAtlasBySpellID` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `ConfirmReadyCheck` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `DemoteAssistant` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `DoReadyCheck` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GetMerchantCurrencies` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `IsGUIDInGroup` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `PromoteToAssistant` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `PromoteToLeader` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `SetEveryoneIsAssistant` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `UninviteUnit` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GetAutoCompletePresenceID` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GetAutoCompleteResults` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `GetAutoCompleteRealms` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `IsRecognizedName` | untriaged | global-api | removed | Pending occurrence-level evidence and classification for 12.0.7 global-api occurrence. |
| `Minimap.SetBlipTexture` | untriaged | widget-method | removed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `Minimap.SetCorpsePOIArrowTexture` | untriaged | widget-method | removed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `Minimap.SetIconTexture` | untriaged | widget-method | removed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `Minimap.SetPOIArrowTexture` | untriaged | widget-method | removed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `Minimap.SetPlayerTexture` | untriaged | widget-method | removed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |
| `Minimap.SetStaticPOIArrowTexture` | untriaged | widget-method | removed | Pending occurrence-level evidence and classification for 12.0.7 widget-method occurrence. |

## Sources

- `data/patch-api/sources/12.0.7-register.json` — committed occurrence source and category metadata.
- `data/blizzard-ui-files/retail.txt` — target retail cache manifest.

## See Also

- [[patch-12-0-7-api-audit]] — broader 12.0.7 audit context.
- [[patch-api-audit-manifest]] — register schema and validation contract.
