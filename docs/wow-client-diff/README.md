# WoW Client Diff — Task Reference

Discovery data collected 2026-02-23 by running `WowDiscovery` and `WowBehaviorTest` addons against
live WoW client, then diffing against what the simulator exposes.

Source files in this directory:
- `WowDiscovery.lua` — addon that dumps live client API surface
- `WowBehaviorTest.lua` — addon that records actual values for edge-case behaviors
- `diff_summary.txt` — counts at a glance
- `diff_methods_missing.txt` / `diff_methods_extra.txt` — frame method gaps
- `diff_c_functions_missing.txt` / `diff_c_functions_extra.txt` — C_* function gaps
- `diff_c_namespaces_missing.txt` / `diff_c_namespaces_extra.txt` — C_* namespace gaps
- `diff_enums_missing.txt` / `diff_enums_missing_values.txt` / `diff_enums_extra.txt` — enum gaps
- `diff_constants_missing.txt` / `diff_constants_wrong.txt` / `diff_constants_extra.txt` — constant gaps
- `diff_global_functions_missing.txt` / `diff_global_functions_extra.txt` — global function gaps

Summary of gaps:

| Category | Missing | Extra |
|---|---|---|
| Frame methods | 1382 | 3810 |
| C_* functions | 208 | 152 |
| Global functions | 310 | 182 |
| Enums | 1430 | 2 |
| Constants | 1364 (72 wrong value) | 12 |
| C_* namespaces | 6 | 11 |

## Weekly Reconciliation Cycle

Run a full `diff_*.txt` reconciliation pass once per week, and also before any broad API-surface
implementation sprint that touches more than one of these categories:

- frame methods
- C_* APIs / namespaces
- global functions
- constants
- enums

### Pass Gate

Do not start or close a reconciliation pass unless every still-open gap category has an explicit
owner recorded for that pass. "No owner yet" is itself a blocker, not a pass result.

Each pass must leave behind:

- the pass date
- the exact `diff_*.txt` files reviewed
- the owner list for every open gap class
- any tasks opened or updated from the diff review
- the unresolved gaps carried forward to the next pass, with owners

### Owner List

Use the code-area owner list below when assigning responsibility for a pass:

| Gap Class | Owner Surface |
|---|---|
| Methods | `src/lua_api/frame/methods/`, `src/lua_api/frame/method_registry/`, `src/lua_api/frame/metatable.rs` |
| C_* APIs / namespaces | `src/lua_api/globals/c_*.rs`, `src/lua_api/globals/c_stubs_api*.rs`, `src/lua_api/globals/generated_stubs.rs` |
| Globals | `src/lua_api/globals/*.rs`, `src/lua_api/globals_legacy.rs`, `src/lua_api/globals/strings/` |
| Constants | `src/lua_api/globals/constants_api.rs`, `src/lua_api/globals/c_stubs_api_extra.rs`, `src/lua_api/globals/enum_data/missing_constants.lua` |
| Enums | `src/lua_api/globals/enum_api.rs`, `src/lua_api/globals/enum_data/`, `src/lua_api/globals/enum_data/missing_enums.lua` |

---

## Section 1: Behavior Divergences

One test failed out of 161. All passing tests record the confirmed live client values below —
useful as regression anchors.

**The one failure:**

- `animation.SetToFinalAlpha` — `GetToFinalAlpha` method is nil on the sim's Alpha animation.
  Live client has it. Implement `SetToFinalAlpha` / `GetToFinalAlpha` on Alpha animation objects.

**Notable confirmed behaviors (already correct in sim, keep as-is):**

- `SetPoint('CENTER')` with no relativeTo: returns `relativeTo_name = "nil"` and `is_parent = false`
  (nil, not the parent — this is intentional in live WoW)
- `SetAllPoints` creates exactly 2 anchors: TOPLEFT and BOTTOMRIGHT (in that canonical order)
- `GetPoint(0)` returns 0 values; `GetPoint(1)` returns 5 values (1-based indexing confirmed)
- Anchor canonical order for all 9 points: TOPLEFT, TOP, TOPRIGHT, LEFT, CENTER, RIGHT, BOTTOMLEFT, BOTTOM, BOTTOMRIGHT
- `RegisterEvent` returns `true` (boolean), 1 return value
- `RegisterEvent` is idempotent — registering twice then unregistering once fully unregisters
- `SetScale(0)` and `SetScale(-1)` both produce error: `"Frame:SetScale(): Scale must be > 0"`
- Effective alpha chain: 0.5 * 0.8 * 0.5 = 0.2 (confirmed multiplicative)
- Effective scale chain: 2.0 * 3.0 = 6.0 (confirmed multiplicative)
- `Raise()` does NOT change frame level (before=1, after=1)
- `Lower()` DOES change frame level — sets to parent's level - 1 (e.g. parent=6, child goes from 7 to 5)
- `SetIgnoreParentAlpha`: effective alpha before=0.5019, after=1 (confirmed working)
- `SetIgnoreParentScale`: effective scale before=2, after=1 (confirmed working)
- `GetScript` after `HookScript`: returns a wrapper function, not the original (same_as_original=false)
- `SetScript` after `HookScript`: the hook does NOT survive — only the new SetScript handler runs
- `HookScript` with no existing handler: creates a handler, does not error (ok=true, called=true)
- Scroll: negative scroll NOT clamped — `SetVerticalScroll(-50)` returns -50.0 (not 0)
- Scroll: out-of-range scroll NOT clamped — `SetHorizontalScroll(999)` returns 999.0
- Tooltip: `NumLines` returns 0 even after `AddLine` calls (counter not updated in our sim — investigate)
- Tooltip: `AddDoubleLine` counts as 0 lines in NumLines (same issue)
- Tooltip: `GetLine` returns text via named FontString regions, not positional returns
- `RegisterAllEvents`: does NOT make `IsEventRegistered("PLAYER_LOGIN")` return true
- `Mixin(t, nil)` errors: `"Usage: local outObject = Mixin(object, ...)\nLua Taint: ..."`
- `EditBox` auto focus default is `true` (not false)
- `frame.alpha_effective` with SetAlpha(0.5): `own=0.5019..` (float precision — `0.5` stored as nearest float)
- `frame.scale_effective`: parent=2.0, child=0.5 → effective=1.0 (2*0.5=1, not 0.5)
- `Mixin` overwrites frame methods — `GetName` on a frame with a mixin that has `GetName` returns the mixin value

**Behavior to fix — `tooltip.NumLines` / `tooltip.AddLine` / `tooltip.AddDoubleLine`:**

Live client: `NumLines` returns 0 before `SetOwner`. After `ClearLines`, returns 0. After `AddLine`x2,
test result shows `count=0` — this is the test checking NumLines with no owner set. The
`multiple_SetOwner` test confirms that second `SetOwner` clears lines (count=0). The issue is the
sim's `NumLines` may not be tracking correctly.

---

## Section 2: Missing Frame Methods

Source: `diff_methods_missing.txt` (1382 entries total).

The file contains methods that exist in live WoW but not in the sim. Many are on every widget type —
that indicates the sim already has them but does NOT have them type-specifically registered
(see Section 3 for the flip side).

**Methods that appear on all or most widget types — genuine engine methods to add:**

All Frame-type widgets are missing this cluster of methods (present on Frame, Button, CheckButton,
EditBox, ScrollFrame, Slider, StatusBar, GameTooltip, MessageFrame, ColorSelect, Cooldown, Model,
PlayerModel, SimpleHTML):

| Method | Category |
|---|---|
| `AbortDrag` | Input |
| `CanChangeAttribute` | Attribute |
| `CanPropagateMouseClicks` / `CanPropagateMouseMotion` | Input |
| `ClearAlphaGradient` / `SetAlphaGradient` / `HasAlphaGradient` / `GetAlphaGradient` | Visual |
| `ClearAttribute` / `SetAttributeNoHandler` / `ExecuteAttribute` | Attribute |
| `ClearParentKey` | Hierarchy |
| `CollapsesLayout` / `SetCollapsesLayout` | Layout |
| `CreateLine` | Drawing |
| `DesaturateHierarchy` | Visual |
| `DisableDrawLayer` / `EnableDrawLayer` | Visual |
| `DoesHyperlinkPropagateToParent` / `SetHyperlinkPropagateToParent` | Input |
| `EnableGamePadButton` / `EnableGamePadStick` / `IsGamePadButtonEnabled` / `IsGamePadStickEnabled` | Input |
| `GetAnimationGroups` | Animation |
| `GetBoundsRect` | Layout |
| `GetClampRectInsets` / `SetClampRectInsets` | Layout |
| `GetDontSavePosition` | Persistence |
| `GetEffectivelyFlattensRenderLayers` / `GetFlattensRenderLayers` | Visual |
| `GetHighestFrameLevel` / `GetRaisedFrameLevel` | Level |
| `GetHitRectInsets` / `SetHitRectInsets` | Input |
| `GetPointByName` | Anchor |
| `GetWindow` / `SetWindow` | Window |
| `HasAnySecretAspect` / `HasSecretAspect` / `HasSecretValues` / `IsPreventingSecretValues` / `SetPreventSecretValues` | Security |
| `InterceptStartDrag` | Input |
| `IsAnchoringRestricted` / `IsAnchoringSecret` | Anchor |
| `IsCollapsed` / `IsDragging` / `IsFrameBuffer` / `IsHighlightLocked` / `IsMouseMotionFocus` / `IsProtected` | State |
| `IsIgnoringChildrenForBounds` / `SetIgnoringChildrenForBounds` | Layout |
| `IsUsingParentLevel` / `SetUsingParentLevel` | Level |
| `Lower` / `Raise` | Level |
| `RegisterEventCallback` / `RegisterUnitEventCallback` | Event |
| `RotateTextures` | Visual |
| `SetCollapsesLayout` | Layout |
| `SetHighlightLocked` | State |
| `SetIsFrameBuffer` | Visual |
| `SetPointsOffset` | Anchor |
| `SetPropagateMouseClicks` / `SetPropagateMouseMotion` | Input |
| `SetToDefaults` | Misc |
| `ShouldButtonPassThrough` | Input |
| `StartMoving` / `StartSizing` / `StopMovingOrSizing` | Layout |

**Button-specific missing methods (not on Frame):**
`SetATTTooltip`, `StartATTCoroutine` — likely addon-injected (ATT = All the Things). Skip.
`SetFormattedText` — legitimate, also on CheckButton/EditBox/FontString.
`SetDisabledAtlas`, `SetHighlightAtlas`, `SetNormalAtlas`, `SetPushedAtlas` — atlas variants for button state textures.
`ClearDisabledTexture`, `ClearHighlightTexture`, `ClearNormalTexture`, `ClearPushedTexture` — clear state texture slots.
`SetHighlightLocked` / `IsHighlightLocked` — lock highlight state.

**CheckButton-specific:**
`GetDisabledCheckedTexture` / `SetDisabledCheckedTexture` — checked state for disabled buttons.

**Cooldown-specific (Cooldown widget is largely unimplemented):**
`GetCooldownDisplayDuration`, `GetCountdownFontString`, `GetDrawBling`, `GetDrawEdge`, `GetDrawSwipe`,
`GetEdgeScale`, `GetHideCountdownNumbers`, `GetMinimumCountdownDuration`, `GetUseAuraDisplayTime`,
`IsPaused`, `Pause`, `Resume`, `SetCooldownFromDurationObject`, `SetCooldownFromExpirationTime`,
`SetCooldownUNIX`, `SetCountdownAbbrevThreshold`, `SetCountdownFont`, `SetEdgeColor`,
`SetMinimumCountdownDuration`, `SetPaused`, `SetTexCoordRange`, `SetUseAuraDisplayTime`

**FontString-specific:**
`CalculateScreenAreaFromCharacterSpan`, `ClearText`, `FindCharacterIndexAtCoordinate`,
`GetFieldSize`, `GetFontHeight`, `GetLineHeight`, `GetScaleAnimationMode`, `GetTextScale`,
`OnColorsUpdated`, `SetFixedColor`, `SetFontHeight`, `SetNonSpaceWrap`, `SetScaleAnimationMode`,
`SetTextHeight`, `SetTextScale`, `SetTextToFit`, `SetVertexColorFromBoolean`, `CanNonSpaceWrap`

**Texture-specific:**
`ClearVertexOffsets`, `GetDesaturation`, `GetTextureFileID`, `GetTextureFilePath`, `GetVertexOffset`,
`IsBlockingLoadRequested`, `ResetTexCoord`, `SetBlockingLoadsRequested`, `SetMask`, `SetSpriteSheetCell`,
`SetVertexColorFromBoolean`, `SetVertexOffset`

**GameTooltip-specific:**
`AddAtlas`, `AddFontStrings`, `AddTexture`, `ClearPadding`, `CopyTooltip`, `GetCustomLineSpacing`,
`GetLeftLine`, `GetMinimumWidth`, `GetRightLine`, `IsOwned`, `SetAllowShowWithNoLines`,
`SetAnchorType`, `SetCustomLineSpacing`, `SetCustomWordWrapMinWidth`, `SetFrameStack`,
`SetObjectTooltipPosition`, `SetShrinkToFitWrapped`

**Model-specific (Model widget largely unimplemented):**
`AdvanceTime`, `ClearFog`, `ClearTransform`, `GetCameraDistance`, `GetCameraFacing`,
`GetCameraPosition`, `GetCameraRoll`, `GetCameraTarget`, `GetDesaturation`, `GetFogColor`,
`GetFogFar`, `GetFogNear`, `GetLight`, `GetModelAlpha`, `GetModelDrawLayer`, `GetModelFileID`,
`GetPaused`, `GetPitch`, `GetRoll`, `GetShadowEffect`, `GetViewInsets`, `GetViewTranslation`,
`GetWorldScale`, `HasAttachmentPoints`, `HasCustomCamera`, `IsUsingModelCenterToTransform`,
`MakeCurrentCameraCustom`, `ReplaceIconTexture`, `SetCameraDistance` (etc.), `SetCustomCamera`,
`SetFogColor` (etc.), `SetGlow`, `SetGradientMask`, `SetModelAlpha`, `SetModelDrawLayer`,
`SetParticlesEnabled`, `SetPaused`, `SetPitch`, `SetRoll`, `SetShadowEffect`, `SetTransform`,
`SetUseGBuffer`, `SetViewInsets`, `SetViewTranslation`, `TransformCameraSpaceToModelSpace`,
`UseModelCenterToTransform`

**PlayerModel adds over Model:**
`ApplySpellVisualKit`, `CanSetUnit`, `FreezeAnimation`, `GetDisplayInfo`, `GetDoBlend`,
`GetKeepModelOnHide`, `HasAnimation`, `PlayAnimKit`, `SetBarberShopAlternateForm`,
`SetDoBlend`, `SetItem`, `SetItemAppearance`, `SetKeepModelOnHide`, `StopAnimKit`, `ZeroCachedCenterXY`

---

## Section 3: Extra Frame Methods

Source: `diff_methods_extra.txt` (3810 entries total, 350 unique method names across 17 widget types).

**Root cause**: The sim registers all methods on all widget types instead of per-type. For example,
`GetScrollChild` (a ScrollFrame method) is also registered on Button, Frame, Cooldown, etc. Live WoW
only exposes each method on the widget types that actually have it.

**Methods per type with the most extras (sim registering wrong-type methods):**

| Widget Type | Extra Methods |
|---|---|
| Minimap | 350 |
| FontString | 263 |
| Texture | 247 |
| Frame | 229 |
| ColorSelect | 225 |
| SimpleHTML | 220 |
| ScrollFrame | 220 |
| Model | 216 |
| GameTooltip | 216 |
| StatusBar | 212 |
| Slider | 212 |
| Cooldown | 210 |
| MessageFrame | 209 |
| PlayerModel | 207 |
| Button | 196 |
| CheckButton | 192 |
| EditBox | 186 |

**Examples of cross-type pollution visible in the extra list:**
- `Button:GetScrollChild`, `Button:GetVerticalScroll` — ScrollFrame methods on Button
- `Button:GetCooldownDuration`, `Button:GetFillStyle` — Cooldown/StatusBar methods on Button
- `Button:GetModel`, `Button:GetCamDistanceScale` — Model methods on Button
- `Button:AddLine`, `Button:GetNumLines` — GameTooltip/MessageFrame methods on Button
- `Button:GetChecked`, `Button:GetCheckedTexture` — CheckButton methods on Button

**Task**: Audit the Lua API registration (`src/lua_api/`) to ensure methods are only registered on
the widget types that actually have them. The `wowless/data/products/wow/uiobjects.yaml` file has
the authoritative per-type method lists.

---

## Section 4: Missing C_* Functions

Source: `diff_c_functions_missing.txt` (208 entries). Filtered for real engine functions (not addon pollution).

**High-priority namespaces with many missing functions:**

**C_TooltipInfo** (76 functions missing — entire namespace absent):
`GetAction`, `GetInventoryItem`, `GetSpellBookItem`, `GetUnitAura`, `GetUnitBuff`, `GetUnitDebuff`,
`GetMerchantItem`, `GetQuestItem`, `GetLootItem`, `GetBagItemChild`, `GetCurrencyToken`, etc.
The entire `C_TooltipInfo` namespace needs to be added as stubs.

**C_PingSecure** (14 functions — secure ping system):
`CreateFrame`, `SendPing`, `GetTargetWorldPing`, `SetPingPinFrameAddedCallback`, etc.

**C_WoWLabsMatchmaking** (16 functions) and **C_WowLabsDataManager** (6 functions):
WoW Labs / plunderstorm matchmaking system.

**C_UnitAurasPrivate** (12 functions):
`GetAllPrivateAuras`, `GetAuraDataBySlot`, `AnchorPrivateAura`, `SetPrivateAuraAnchorAddedCallback`, etc.

**C_PetBattles** (46 functions — entire namespace largely absent):
`GetAbilityInfo`, `GetHealth`, `GetLevel`, `GetName`, `GetNumPets`, `ChangePet`, etc.

**C_EncounterEvents** / **C_EncounterTimeline** / **C_EncounterWarnings** (12 functions):
Encounter event/warning system.

**C_CombatLog** (10 functions):
`AddEventFilter`, `AdvanceEntry`, `GetCurrentEntryInfo`, `GetEntryCount`, etc.

**C_CombatLogSecure** (10 functions):
Secure combat log access.

**C_MacOptions** (9 functions):
Mac-specific options. Can be stubbed as returning false/nil.

**Smaller gaps (1-3 functions each):**
- `C_ActionBar.GetActionCount`, `C_ActionBar.GetCurrentActionBarByClass`
- `C_Calendar.CloseCalendar`, `C_Calendar.GetDate`, `C_Calendar.GetMaxDate`
- `C_ChallengeMode.GetCompletionInfo`
- `C_Console.GetAllCommands`, `C_Console.GetColorFromType`
- `C_EditMode.ConvertLayoutInfoToHyperlink`
- `C_EncounterJournal.GetEncounterInfo`, `C_EncounterJournal.GetInstanceInfo`
- `C_Guild.GetGuildInfo`, `C_Guild.GetMemberInfo`, `C_Guild.GetNumMembers`, `C_Guild.IsInGuild`
- `C_LFGList.IsSquelched`
- `C_Macro.GetMacroInfo`, `C_Macro.GetNumMacros`
- `C_Mail.GetNumItems`, `C_Mail.HasNewMail`
- `C_Minimap.SetPlayerTexture`, `C_Minimap.SetTrackingFilterByFilterIndex`
- `C_MountJournal.Summon`
- `C_MythicPlus.GetOverallDungeonScore`, `C_MythicPlus.GetSeasonInfo`
- `C_NamePlate.GetNamePlateEnemySize` / `GetNamePlateFriendlySize` / `GetNamePlateSelfSize` / `GetTargetClampingInsets` / `SetNamePlate*`
- `C_Navigation.GetDestination`, `C_Navigation.IsAutoFollowEnabled`, `C_Navigation.SetAutoFollowEnabled`
- `C_PlayerChoice.GetNumPlayerChoices`, `C_PlayerChoice.GetPlayerChoiceInfo`, `C_PlayerChoice.GetPlayerChoiceOptionInfo`
- `C_PlayerInfo.GetContentDifficultyQualityForPlayer`, `C_PlayerInfo.IsExpansionLandingPageUnlockedForPlayer`
- `C_Reputation.GetFactionInfo`
- `C_Sound.PlaySoundFile`
- `C_Transmog.GetAppliedSourceID`, `C_Transmog.GetSlotInfo`
- `C_TransmogCollection.GetNumMaxOutfits`, `C_TransmogCollection.GetOutfits`, etc.
- `C_Who.GetWhoInfo`, `C_Who.SendWho`, `C_Who.SetWhoToUi`

**Missing C_* namespaces** (6 entire namespaces have no stub at all):
`C_CinematicList`, `C_CombatLogSecure`, `C_Console`, `C_GMTicketInfo`, `C_Guild`, `C_Login`,
`C_MacOptions`, `C_PingSecure`, `C_PrivateAuras`, `C_UnitAurasPrivate`, `C_Who`

Wait — that's 11 entries in `diff_c_namespaces_missing.txt`. Actually the summary says 6 missing
namespaces; the file may list both namespace-level and function-level mismatches. The actual
`diff_c_namespaces_missing.txt` lists: `C_CombatLogSecure`, `C_Console`, `C_GMTicketInfo`,
`C_Guild`, `C_Login`, `C_MacOptions`, `C_PingSecure`, `C_PrivateAuras`, `C_UnitAurasPrivate`, `C_Who`,
`C_CinematicList`.

---

## Section 5: Missing Enums

**Count**: 1430 enum names missing (the file also lists `Meta` variants, which are companion tables).

**Full list with values**: `diff_enums_missing_values.txt` — use this for bulk import.

**Extra enums** (2 that shouldn't be there):
- `ExpansionLandingPageType`
- `TransmogOutfitFlags`

**Task**: Bulk-import the 1430 missing enums from `diff_enums_missing_values.txt`. The file format
is `EnumName.VALUE=number` — parse it to generate the Lua globals. This is mechanical work and can
be done in one batch. Remove `ExpansionLandingPageType` and `TransmogOutfitFlags` if they cause issues
with addon code.

---

## Section 6: Missing Constants

**Count**: 1364 missing constants, 72 with wrong values.

**Wrong values** (72 constants — fix these first, they cause silent behavior bugs):

Source: `diff_constants_wrong.txt`. Key examples:

| Constant | WoW Value | Sim Value |
|---|---|---|
| `LE_EXPANSION_LEVEL_CURRENT` | 11 | 10 |
| `LE_LFG_CATEGORY_LFR` | 2 | 4 |
| `LE_LFG_CATEGORY_RF` | 3 | 2 |
| `LE_LFG_CATEGORY_SCENARIO` | 4 | 3 |
| `LE_WORLD_ELAPSED_TIMER_TYPE_CHALLENGE_MODE` | 1 | 0 |
| `LE_WORLD_ELAPSED_TIMER_TYPE_PROVING_GROUND` | 2 | 1 |
| `MAX_CHARACTER_NAME_BYTES` | 305 | 100 |
| `MAX_COMMUNITY_NAME_LENGTH` | 12 | 20 |
| `MAX_COMMUNITY_NAME_LENGTH_NO_CHANNEL` | 24 | 15 |
| `LE_AUTOCOMPLETE_PRIORITY_*` | All off by +1 | sim values are -1 from WoW |
| `ITEM_MOD_*` | Format strings with `%c%s` | sim has plain strings |
| `LE_GAME_ERR_*` | Integer error codes | sim has English strings |
| `RAID_BOSSES` | "Bosses" | "Raid Bosses" |
| `SPELL_FAILED_NOT_READY` | "Not yet recovered" | "Spell is not ready" |

**Note on `LE_AUTOCOMPLETE_PRIORITY_*`**: All 6 values are off by +1 — sim uses 0-based, WoW uses 1-based.

**Note on `LE_GAME_ERR_*` constants**: WoW stores these as integer error codes, not English strings.
The sim appears to have substituted English strings instead. These need to be numeric IDs.

**Missing constants**: 1364 total. The `diff_constants_missing.txt` file has them with their WoW values.
Includes item quality color codes (`ITEM_EPIC_COLOR_CODE`, etc.), many `LE_GAME_ERR_*` numeric codes,
`LE_CHARACTER_UNDELETE_RESULT_*`, `LE_CHARACTER_UPGRADE_RESULT_*`, and hundreds of others.

**Task**: Fix the 72 wrong values first (higher bug impact), then bulk-add the 1364 missing ones.

---

## Section 7: Extra Stubs to Remove or Investigate

**Extra C_* namespaces** (11 namespaces the sim has that live WoW does NOT):

Source: `diff_c_namespaces_extra.txt`:
- `C_AccountServices`
- `C_ArrowCalloutManager`
- `C_CatalogShop`
- `C_DelvesUI`
- `C_EncounterEvents`
- `C_EncounterTimeline`
- `C_EncounterWarnings`
- `C_Housing` / `C_HousingCatalog` / `C_HousingLayout` / `C_HousingPhotoSharing`

Wait — that's more than 11. The `diff_c_namespaces_extra.txt` lists functions in extra namespaces,
while `diff_c_functions_extra.txt` has 152 entries. The 11 extra namespaces from the summary count
are: `C_AccountServices`, `C_ArrowCalloutManager`, `C_CatalogShop`, `C_DelvesUI`,
`C_EncounterEvents`, `C_EncounterTimeline`, `C_EncounterWarnings`, `C_Housing`,
`C_HousingCatalog`, `C_HousingLayout`, `C_HousingPhotoSharing`.

These are namespaces where the sim registered functions that exist under different namespaces in live
WoW, or which were renamed/removed. Check each against `diff_c_functions_extra.txt` to see if they
should be removed, renamed, or merged.

**Extra enums** (2):
- `ExpansionLandingPageType` — verify if this is a sim invention or was renamed in live WoW
- `TransmogOutfitFlags` — same check

**Extra global functions** (182 in `diff_global_functions_extra.txt`):
Most are addon-injected globals from loaded third-party addons (Angleur, Details, ATT, Auctionator,
DejunkBindings, etc.) that leaked into the discovery scan. The real sim extras are a smaller set —
look for non-addon-prefixed names. Examples of likely real extras: `GetNumTotemSlots`, `GetWorldDeltaSeconds`,
`FindSpellByName`, `FormatTooltipNumber`, `GetPlayerInfo`, `ResolvePrefixedChannelName`.

**Extra C_* functions** (152 in `diff_c_functions_extra.txt`):
These are functions the sim registered under C_* namespaces that don't exist in live WoW. Many are
under the extra namespaces above. Review to determine if they're renamed, moved to different namespaces,
or genuinely absent from live WoW and should be removed.
