## [2026-08-09] investigation | Classify three C_HouseExterior mutation/debug gaps

Classified `C_HouseExterior.GetSelectedFixtureDebugInfo`, `SetHouseExteriorSize`, and `SetHouseExteriorType` as evidence-required/unsafe. The debug API retains only a name without signature/returns and lacks selected-fixture debug state; both setters are published no-ops that do not update getter-visible selected state, validate values, resolve names, persist, refresh, reset/isolate, or model lifecycle. Current totals are **873 best-effort, 765 evidence-required, 2 exception-requested, and 1770 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify two C_HouseExterior rows

The bounded two-row `C_HouseExterior` slice classifies `GetHouseExteriorTypeOptions` as best-effort/behavioral only for focused callable publication, one returned table, `selectedExteriorType` 1, and tested `Cottage`/1 plus `Manor`/2 options; metadata, mutation, persistence, refresh, validation, and lifecycle semantics remain unclaimed. `GetHoveredFixtureDebugInfo` is evidence-required/unsafe because only its API name is retained, its signature and returns are unknown, and the nil fallback has no hovered-fixture debug state. Current totals are **873 best-effort, 762 evidence-required, 2 exception-requested, and 1773 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify three Delves/housing rows

The bounded three-row 12.0.0 Delves/housing slice classifies `C_HouseExterior.GetHouseExteriorSizeOptions` as best-effort/behavioral only for focused proof of callable publication, one table return, `selectedSize` 3, and exactly Medium/3 plus Large/4 options; exact metadata, enum fidelity, mutation, persistence, refresh, validation, and lifecycle remain unclaimed. `C_DelvesUI.IsTraitTreeForCompanion` is evidence-required/unsafe because it is absent without trait-tree ownership/classification state. `C_Housing.OnHouseFinderClickPlot` is evidence-required/unsafe because it is absent without selected-plot request/event/state side effects or validation.

## [2026-08-09] investigation | Classify four C_DelvesUI and housing unsafe rows

Classified `C_DelvesUI.GetLockedTextForCompanion`, `C_HouseExterior.GetFixtureDebugInfoForGUID`, `C_Housing.IsHousingMarketShopEnabled`, and `C_HousingBasicMode.IsFreePlaceEnabled` as evidence-required/unsafe. `GetLockedTextForCompanion` is absent and lacks companion lock-state/text behavior; `GetFixtureDebugInfoForGUID` is absent, lacks GUID-indexed fixture-debug state, and the checked-in source preserves only its API name without signature/return semantics; `IsHousingMarketShopEnabled` is absent without dedicated boolean state; and `IsFreePlaceEnabled` hardcodes true while `SetFreePlaceEnabled` is a no-op, so mutable state and exact semantics are unmodeled. None has approval or an exception. Current totals are **872 best-effort, 761 evidence-required, 2 exception-requested, and 1775 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify four C_EventUtils, C_HouseExterior, C_CreatureInfo, and C_GameRules rows

Classified `C_EventUtils.IsCallbackEvent` as best-effort/behavioral from focused positive `COMBAT_LOG_EVENT` and negative `PLAYER_LOGIN` proof; exact registry completeness, argument validation, dynamic behavior, and lifecycle remain unclaimed. Classified `C_HouseExterior.GetCurrentHouseExteriorType` as best-effort/behavioral only for callable publication and the seeded two-return shape/types/values `1` and `Sunspire Cottage`; retail selection, mutation, persistence, refresh, and lifecycle remain unclaimed. Classified `C_CreatureInfo.GetCreatureID` as evidence-required/unsafe because it is absent and lacks a GUID/creature identity model. Classified `C_GameRules.IsPersonalResourceDisplayEnabled` as evidence-required/unsafe because the current fallback returns nil rather than the required boolean and has no backing state. Current totals are **871 best-effort, 755 evidence-required, 2 exception-requested, and 1782 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify eight C_EventScheduler rows

Classified `C_EventScheduler.CanShowEvents` as best-effort/behavioral only for simulator visibility derivation from explicit override, suppression, event-list state, and request repopulation; retail availability, refresh timing, persistence, full lifecycle, and edge semantics remain unclaimed. Classified `EventDisplayInfo.hideDescription`, `EventDisplayInfo.hideTimeLeft`, `EventDisplayInfo.overrideAtlas`, `EventDisplayInfo.overrideTooltipWidgetSetID`, `OngoingEventInfo.displayInfo`, `ScheduledEventInfo.displayInfo`, and `ScheduledEventInfo.eventID` as evidence-required/unsafe because temporary empty/seeded compatibility payloads do not establish documented typed field values, shape, producers, or lifecycle. Current totals are **869 best-effort, 753 evidence-required, 2 exception-requested, and 1786 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify three C_CooldownViewer gaps

Classified `C_CooldownViewer.CooldownViewerCooldown.category`, `C_CooldownViewer.CooldownViewerCooldown.cooldownID`, and `C_CooldownViewer.GetValidAlertTypes` as `evidence-required`/`unsafe`. The current temporary surface returns nil/empty defaults and has no typed cooldown producer, category/ID records, ordered alert-type arrays, validation, routing, Settings UI behavior, or lifecycle. Current totals are **868 best-effort, 746 evidence-required, 2 exception-requested, and 1794 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify seven C_ChatInfo, C_CombatText, and C_Commentator API gaps

Classified the seven-row `C_ChatInfo`/`C_CombatText`/`C_Commentator` API-gap slice as `evidence-required`/`unsafe`. Current fallbacks are absent, no-op, constant-false, or adjacent-state only and do not model lockdown, emote, active-unit, combat-text, or commentator event state, restrictions, result contracts, transitions, events, ordering, or lifecycle. Current totals are **868 best-effort, 743 evidence-required, 2 exception-requested, and 1797 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify C_CharacterServices.AssignFCMDistribution

Classified `C_CharacterServices.AssignFCMDistribution` as `evidence-required`/`unsafe`. The source register provides no signature/result metadata, and the simulator has no FCM validation/assignment model; account/realm/character checks, validation-only behavior, exact results, state transitions, persistence, and events remain unproven. Current totals are **868 best-effort, 736 evidence-required, 2 exception-requested, and 1804 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify 23 C_CatalogShop API and structure rows

Classified `C_CatalogShop.HasNewProducts` as `best-effort`/`behavioral` only for publication and the exact constant-false boolean result proven by `test_startup_service_namespaces_exist`. Classified the other 22 rows as `evidence-required`/`unsafe` because current CatalogShop state is absent, no-op, incomplete, seeded, wrong-typed, or untested and does not establish purchase, category/product, refundable-decor, currency, session, refresh, restriction, payload, event, or lifecycle semantics. Current totals are **868 best-effort, 735 evidence-required, 2 exception-requested, and 1805 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify three C_BattleNet mutation and transport APIs

Classified `C_BattleNet.SendGameData`, `C_BattleNet.SendWhisper`, and `C_BattleNet.SetCustomMessage` as `evidence-required`/`unsafe`. No current registration, backing transport/state, restriction enforcement, result model, or focused side-effect proof exists; exact account/text validation, return values, persistence, events, and consumer behavior remain unproven. Current totals are **867 best-effort, 713 evidence-required, 2 exception-requested, and 1828 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify C_AdventureMap quest portrait info

Classified the six-row `C_AdventureMap.GetQuestPortraitInfo` slice—the API plus five portrait fields—as bounded `best-effort`/`behavioral` claims from focused `tests/c_adventure_map/quests.rs` proof. Claims cover injected-state lookup, typed five-field publication, unknown/nonnumeric zero-value returns, nullable `modelSceneID`, and tested display-ID gating. Retail data population, localization, full validation/edge behavior, assets/rendering, and lifecycle remain unclaimed. `modelSceneID` is treated strictly as data under the existing permanent no-3D scope; no 3D implementation or new exception is requested. Current totals are **867 best-effort, 710 evidence-required, 2 exception-requested, and 1831 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify nine C_ActionBar charge/cooldown structure fields

Classified the nine `C_ActionBar.ActionBarChargeInfo`/`ActionBarCooldownInfo` structure fields as `evidence-required`/`unsafe`. `GetActionCharges` returns static placeholder charge fields; `GetActionCooldown` returns a partial table with fixture-backed start/duration and constant enabled/mod-rate. Neither establishes authoritative typed payload fidelity, slot-dependent charges, field relationships, invalid-slot behavior, progression, secrets, or lifecycle. Current totals are **861 best-effort, 710 evidence-required, 2 exception-requested, and 1837 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify ten CatalogShop, encounter-chat, combat-log, and commentator events

Classified `CATALOG_SHOP_REFUNDABLE_DECORS_UPDATED`, `CATALOG_SHOP_VIRTUAL_CURRENCY_BALANCE_UPDATE`, `CATALOG_SHOP_VIRTUAL_CURRENCY_BALANCE_UPDATE_FAILURE`, `CHAT_MSG_ENCOUNTER_EVENT`, `COMBAT_LOG_APPLY_FILTER_SETTINGS`, `COMBAT_LOG_ENTRIES_CLEARED`, `COMBAT_LOG_MESSAGE`, `COMBAT_LOG_MESSAGE_LIMIT_CHANGED`, `COMBAT_LOG_REFILTER_ENTRIES`, and `COMMENTATOR_COMBAT_EVENT` as `evidence-required`/`unsafe`. Names, payload metadata, and registration may exist, but current placeholder state has no authoritative producer or focused proof for exact payloads, restricted/callback rules, validity/registerability, state ordering, lifecycle, duplicate behavior, or consumer effects. `CHAT_MSG_ENCOUNTER_EVENT` validity/registerability remains unresolved, and the `COMMENTATOR_COMBAT_EVENT` source payload conflicts with consumer state queries. Current totals are **861 best-effort, 701 evidence-required, 2 exception-requested, and 1846 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify six additional CAA CVar-default rows

Classified six additional target-health, voice, and volume CAA CVar rows as bounded `best-effort`/`behavioral` claims using `test_patch_12_0_0_cvar_defaults`. Consolidated with the prior 27-row slice, the cumulative CAA CVar-default slice covers 33 rows. The focused test proves only startup `GetCVar`/`GetCVarDefault` exact string defaults; CAA behavior, UI/audio effects, mutation, persistence, events, flags, consumers, and later-epoch semantics remain unclaimed. Current totals are **861 best-effort, 691 evidence-required, 2 exception-requested, and 1856 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify 20 additional CAA CVar-default rows

Classified 20 additional player-, resource-, speech-, and target-cast CAA CVar rows as bounded `best-effort`/`behavioral` claims using `test_patch_12_0_0_cvar_defaults`. Consolidated with the prior seven-row slice, the cumulative CAA CVar-default slice covers 27 rows. The focused test proves only startup `GetCVar`/`GetCVarDefault` exact string defaults; CAA behavior, UI/audio effects, mutation, persistence, events, flags, consumers, and later-epoch semantics remain unclaimed. Current totals are **855 best-effort, 691 evidence-required, 2 exception-requested, and 1862 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify two 12.0.0 event rows

Classified `ADDON_RESTRICTION_STATE_CHANGED` and `BULK_PURCHASE_RESULT_RECEIVED` as `evidence-required`/`unsafe`. Registration and enum publication exist, but no modeled transition/purchase producer or focused proof establishes exact payload values/structures/arity, synchronous timing, ordering, duplicate behavior, lifecycle, or consumers. Current totals are **835 best-effort, 691 evidence-required, 2 exception-requested, and 1882 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify three global-utility rows

Classified `AbbreviateLargeNumbers` and `AbbreviateNumbers` as `evidence-required`/`unsafe` because temporary fallbacks ignore `NumberAbbrevOptions` and do not model abbreviation, localization, or validation. Classified `AddSourceLocationExclude` as bounded `best-effort`/`behavioral` for nil-guarded global publication and successful string-argument no-op invocation through `installs_debug_environment_defaults`; exclusion, filtering, and debug semantics remain unclaimed. Current totals are **835 best-effort, 689 evidence-required, 2 exception-requested, and 1884 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify seven CAA CVar-default rows

Classified `CAAEnabled`, `CAAInterruptCast`, `CAAInterruptCastSuccess`, `CAAPartyHealthFrequency`, `CAAPartyHealthPercent`, `CAAPlayerCastFormat`, and `CAAPlayerCastMinTime` as bounded `best-effort`/`behavioral` claims using `test_patch_12_0_0_cvar_defaults`. The focused test proves only startup `GetCVar`/`GetCVarDefault` exact string defaults; CAA behavior, UI/audio effects, mutation, persistence, events, flags, consumers, and later-epoch semantics remain unclaimed. Current totals are **834 best-effort, 687 evidence-required, 2 exception-requested, and 1887 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify 23 CVar-default rows

Classified 23 added CVar-default rows as bounded `best-effort`/`behavioral` claims using `test_patch_12_0_0_cvar_defaults`. The focused test proves only startup `GetCVar`/`GetCVarDefault` exact string defaults; mutation, events, persistence, secure/read-only flags, consumers, and later-epoch behavior remain unclaimed. Current totals are **827 best-effort, 687 evidence-required, 2 exception-requested, and 1894 untriaged** (3410 rows).

## [2026-08-09] investigation | Classify four unit/heal-prediction rows

Classified `UnitCreatureID`, `UnitIsHumanPlayer`, `UnitIsSpellTarget`, and `UnitHealPredictionValues.totalDamageAbsorbs` as bounded `best-effort`/`behavioral` claims. Existing focused tests cover the token/GUID/vendor-shim cases; `unit_detailed_heal_prediction_populates_calculator` now asserts the numeric zero absorb field. Full retail semantics, invalid inputs, lifecycle, and untested states remain unclaimed. Current totals are **804 best-effort, 687 evidence-required, 2 exception-requested, and 1917 untriaged**.

## [2026-08-09] investigation | Classify miscellaneous payload fields

Classified eight ExpansionDisplayInfo, LuaColorCurvePoint, PrivateAuraIconInfo, and SpellCooldownInfo structure fields as `evidence-required`/`unsafe`. Current behavior is absent, nil-only, generic, or placeholder-backed and does not establish exact contracts, state/security, or consumer semantics; tests remain empty with null commit, approval, and scope exception. Current totals are **800 best-effort, 687 evidence-required, 2 exception-requested, and 1921 untriaged**.

## [2026-08-09] investigation | Classify number-abbreviation fields

Classified eight `NumberAbbrevData`/`NumberAbbrevOptions` structure fields as `evidence-required`/`unsafe`. Generic `AbbreviateConfig` proxy round-tripping does not establish typed contracts, defaults/nullability, validation, ordering, or formatting behavior; tests remain empty with null commit, approval, and scope exception. Current totals are **800 best-effort, 679 evidence-required, 2 exception-requested, and 1929 untriaged**.

## [2026-08-08] investigation | Classify conflicting 12.0.0 error globals

Classified 12 added `LE_GAME_ERR_*` globals as `evidence-required`/`unsafe` because checked-in 12.0.0 source-register values conflict with current nil-guarded fallback publication. Authoritative epoch/value reconciliation is required before changing runtime behavior; tests remain empty with null commit, approval, and scope exception. Current totals are **800 best-effort, 671 evidence-required, 2 exception-requested, and 1937 untriaged**.

## [2026-08-09] investigation | Classify 12.0.0 tutorial and pet constants

Classified five tutorial/pet globals as bounded `best-effort`/`behavioral` startup claims. `test_patch_12_0_0_ui_global_constant_values` proves numeric Lua publication and exact source-register values; tutorial and pet-journal behavior, consumers, lifecycle, and historical load timing remain unclaimed. Current totals are **800 best-effort, 659 evidence-required, 2 exception-requested, and 1949 untriaged**.

## [2026-08-09] investigation | Classify 12.0.0 housing payload fields

Classified 16 exterior size/type, house-level, and decor-refund structure fields as `evidence-required`/`unsafe`. Current temporary housing data is absent or fixture-backed and does not establish exact contracts, authoritative values, state transitions, ordering/localization, or consumer behavior; tests remain empty with null commit, approval, and scope exception. Current totals are **795 best-effort, 659 evidence-required, 2 exception-requested, and 1954 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 general events

Classified nine faction, initiative, loot-rule, nameplate, and neighborhood events as `evidence-required`/`unsafe`. Retail registration exists, but no modeled producer or focused proof establishes each source payload contract, timing, lifecycle, ordering, or duplicate behavior; tests remain empty with null commit, approval, and scope exception. Current totals are **795 best-effort, 643 evidence-required, 2 exception-requested, and 1970 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 housing events

Classified 12 added housing events as `evidence-required`/`unsafe`. The retail event registry accepts each name, but no modeled producer or focused proof establishes its source payload contract, timing, lifecycle, ordering, or duplicate behavior; tests remain empty with null commit, approval, and scope exception. Current totals are **795 best-effort, 634 evidence-required, 2 exception-requested, and 1979 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 VAS result enum

Classified `Enum.VasTransactionPurchaseResult.DbHouseOwnerRestriction=20096` as a bounded `best-effort`/`behavioral` startup claim. `test_patch_12_0_0_vas_transaction_purchase_result_value` proves namespace publication, numeric Lua type, and the exact source-register value; VAS transaction behavior, validation, consumers, lifecycle, and historical load timing remain unclaimed. Current totals are **795 best-effort, 622 evidence-required, 2 exception-requested, and 1991 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 aura sort enums

Classified nine `UnitAuraSortRule` member/metadata rows as bounded `best-effort`/`behavioral` startup claims. `test_patch_12_0_0_unit_aura_sort_rule_enum_values` proves namespace/metadata publication, numeric Lua types, and all exact source-register values; aura ordering, filtering, consumers, lifecycle, validation, and historical load timing remain unclaimed. Current totals are **794 best-effort, 622 evidence-required, 2 exception-requested, and 1992 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 heal-prediction enums

Classified 21 `UnitDamageAbsorbClampMode`, `UnitHealAbsorbClampMode`, `UnitHealAbsorbMode`, and `UnitIncomingHealClampMode` member/metadata rows as bounded `best-effort`/`behavioral` startup claims. `test_patch_12_0_0_heal_prediction_enum_values` proves namespace/metadata publication, numeric Lua types, and all exact source-register values; heal prediction, absorb/clamp calculations, incoming-heal state, UI behavior, lifecycle, and consumer semantics remain unclaimed. Current totals are **785 best-effort, 622 evidence-required, 2 exception-requested, and 2001 untriaged**.

## [2026-08-08] investigation | Fix and classify UI enum metadata epochs

Added exact 12.0.0 startup coverage for 15 metadata rows across `CooldownViewerAddAlertStatusMeta`, `CooldownViewerAlertEventTypeMeta`, `DamageMeterStyleMeta`, `EditModeEncounterEventsSystemIndicesMeta`, and `HouseExteriorWMODataFlagsMeta`. Epoch fix `890ae4afd2b2f5a6c368c8601e332044937135d5` removes later `OnAuraApplied`/`OnAuraRemoved` members and restores `CooldownViewerAlertEventTypeMeta` to `MaxValue=4`, `MinValue=1`, `NumValues=4`. Updated the four existing CooldownViewerAlertEventType member rows to the corrected implementation and absence/metadata proof. Current totals are **764 best-effort, 622 evidence-required, 2 exception-requested, and 2022 untriaged**.

## [2026-08-08] investigation | Classify cooldown, housing, and Edit Mode enums

Classified 20 added `CooldownViewerAddAlertStatus`, `CooldownViewerAlertEventType`, `DamageMeterStyle`, `EditModeEncounterEventsSystemIndices`, and `HouseExteriorWMODataFlags` member rows as bounded `best-effort`/`behavioral` startup publication/value claims. A new focused 12.0.0 test covers the 12 cooldown/housing rows; the existing Edit Mode profile-option enum test covers the eight DamageMeter/Edit Mode rows. At that classification point the current CooldownViewerAlertEventType table still contained later members; epoch fix `890ae4afd2b2f5a6c368c8601e332044937135d5` subsequently removed them for 12.0.0 and added exact four-member membership proof. Consumer semantics remain unclaimed. Current totals are **749 best-effort, 622 evidence-required, 2 exception-requested, and 2037 untriaged**. Moved two existing classified-slice narratives before the inventory source/footer section.

## [2026-08-08] investigation | Classify 12.0.0 replacement enum keys

Added focused 12.0.0 startup coverage for five same-value enum replacements: `GossipNpcOption.TieredEntrance=66`, `ItemRecraftFlags.Invalid=1`, `PerksVendorCategoryType.RefundUnused=24`, `PlayerInteractionType.TieredEntrance=79`, and `QuestTagType.Prey=19`. Classified those five current names as bounded `best-effort`/`behavioral` publication/type/value claims. Their five paired old names remain `evidence-required`/`unsafe`; current bootstrap omission does not prove full-LoD dynamic absence, historical removal timing, alias compatibility, or semantic replacement identity. Current totals are **729 best-effort, 622 evidence-required, 2 exception-requested, and 2057 untriaged**.

## [2026-08-08] investigation | Extend 12.0.0 enum metadata coverage

Classified 10 account-state, HousingResult, TransmogSituation, and SecretAspect metadata rows as bounded `best-effort`/`behavioral` claims. Epoch override `31172606b3f3b8b61bea63a81c457219546789fa` removes the two 12.0.1 SecretAspect members from 12.0.0 and restores exact metadata while retaining later values. Focused tests now cover 98 account-state rows, 90 HousingResult rows, 25 TransmogSituation rows, and 43 ItemCollectionType/SecretAspect rows. Normalized 33 provenance-only structure inventory rows to the declared five-column table. Current totals are **724 best-effort, 617 evidence-required, 2 exception-requested, and 2067 untriaged**.

## [2026-08-08] investigation | Fix EncounterEventFlags epoch drift

Corrected the 12.0.0 runtime to publish the one-value `Enum.EncounterEventFlags` table while retaining the later two-value table from 12.0.5 onward. Focused tests prove `Disabled=1`, exact metadata, absence of later `IgnoreCastConsume` under 12.0.0, and preservation of later values. Classified those four rows plus 12 TooltipDataType, TraitNodeFlag, UICursorType, and UIWidgetVisualizationType rows as bounded `best-effort`/`behavioral` namespace/type/value claims. Current totals are **714 best-effort, 617 evidence-required, 2 exception-requested, and 2077 untriaged**.

## [2026-08-08] investigation | Extend small 12.0.0 enum coverage

Classified 15 added/changed CraftingReagentItemFlag, EditModeAuraFrameSystemIndices, HousingItemToastType, MapIconUIWidgetSetType, and SurveyDeliveryMoment rows as `best-effort`/`behavioral` by extending `src/loader/tests/wow_api_globals/patch_12_0_0_small_enums.rs::test_patch_12_0_0_small_enum_values`. The test now covers 35 rows across 11 families with exact namespace, numeric-type, and source-register-value assertions. Consumer, server/state, bitwise composition, mutation/protection, lifecycle, and edge semantics remain unclaimed. Current totals are **698 best-effort, 617 evidence-required, 2 exception-requested, and 2093 untriaged**.

## [2026-08-08] investigation | Classify six small 12.0.0 enum families

Classified 20 added/changed AccountTransType, CurrencyDestroyReason, CurrencySource, EditModeCooldownViewerSetting, GameRule, and HousingDecorActionFlags rows as `best-effort`/`behavioral` using `src/loader/tests/wow_api_globals/patch_12_0_0_small_enums.rs::test_patch_12_0_0_small_enum_values`. The focused retail 12.0.0 startup test asserts namespace publication, exact numeric Lua types, and exact source-register values. Claims exclude consumer, server/state, bitwise composition, mutation/protection, lifecycle, and edge semantics. Current totals are **683 best-effort, 617 evidence-required, 2 exception-requested, and 2108 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 CombatLogObject enums

Classified 35 added `Enum.CombatLogObject`, `Enum.CombatLogObjectMeta`, `Enum.CombatLogObjectTarget`, and `Enum.CombatLogObjectTargetMeta` rows as `best-effort`/`behavioral` using `src/loader/tests/wow_api_globals/combat_log_object.rs::test_patch_12_0_0_combat_log_object_enum_values`. The focused retail 12.0.0 startup test asserts all four namespace tables, exact numeric Lua types, and exact source-register values, including positive high-bit `None` and `RaidNone`. Claims exclude combat-log bitmask operations, filter matching, consumers, signed host representations, mutation/protection, lifecycle, and edge semantics. Implementation ancestor is `b14f2a854ba`; current source/test hashes are recorded per row. Totals are now **663 best-effort, 617 evidence-required, 2 exception-requested, and 2128 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 TransmogSituation enum

Classified all 22 added `Enum.TransmogSituation.*` rows as `best-effort`/`behavioral` using `src/loader/tests/wow_api_globals/transmog_situation.rs::test_patch_12_0_0_transmog_situation_enum_values`. The focused retail 12.0.0 startup test asserts namespace publication, exact numeric Lua type, and exact source-register value for every entry; transmog behavior, flags/combinations, consumers, validation, persistence, mutation/protection, and lifecycle remain unclaimed. Implementation ancestor is `339424faf1`; current source/test hashes are recorded per row. Totals are now **628 best-effort, 617 evidence-required, 2 exception-requested, and 2163 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 transmog outfit enums

Classified 52 added `Enum.TransmogOutfitSlot`, `Enum.TransmogOutfitSlotError`, `Enum.TransmogOutfitSlotOption`, and `Enum.TransmogOutfitTransactionFlags` rows as `best-effort`/`behavioral` using `src/loader/tests/wow_api_globals/transmog_outfit_enums.rs::test_patch_12_0_0_transmog_outfit_enum_values`. The focused retail 12.0.0 test asserts startup namespace publication, exact numeric Lua type, and exact source-register value for all 52 entries. Claims exclude transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and edge semantics. Totals are now **606 best-effort, 617 evidence-required, 2 exception-requested, and 2185 untriaged**.

## [2026-08-08] investigation | Classify ItemCollectionType and SecretAspect enums

Classified 40 added/changed `Enum.ItemCollectionType`, `Enum.ItemCollectionTypeMeta`, and `Enum.SecretAspect` rows as `best-effort`/`behavioral` using `src/loader/tests/wow_api_globals/item_collection_secret_aspects.rs::test_patch_12_0_0_item_collection_and_secret_aspect_values`; claims are limited to startup namespace publication, numeric type, and exact value. Classified 13 removed ItemCollectionType aliases/meta rows as `evidence-required`/`unsafe`; bootstrap omission does not prove full runtime/dynamic publication absence, historical timing, replacement semantics, or all-LoD removal. Totals are now **554 best-effort, 617 evidence-required, 2 exception-requested, and 2237 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 HousingResult enum

Classified 88 added/changed `Enum.HousingResult.*` rows as `best-effort`/`behavioral` using `src/loader/tests/wow_api_globals/housing_result.rs::test_patch_12_0_0_housing_result_values`; the focused retail 12.0.0 test asserts namespace publication, exact numeric Lua type, and exact source-register value. Claims exclude housing operations, result/error text mapping, consumers, persistence, mutation/protection, and lifecycle. Classified removed `FixtureNotOwned` and `MissingTheme` as `evidence-required`/`unsafe`; bootstrap omission does not prove full runtime/dynamic publication absence, historical timing, replacement semantics, or all-LoD absence. Totals are now **514 best-effort, 604 evidence-required, 2 exception-requested, and 2290 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 account-state enum families

Classified 96 added `Enum.AccountStateLoadedFlags.*`/`Enum.CreateAllAccountData.*` rows as `best-effort`/`behavioral` using `src/loader/tests/wow_api_globals/account_state_flags.rs::test_patch_12_0_0_account_state_enum_values`. The focused retail 12.0.0 startup test asserts both namespaces, exact Lua string type, and exact source-register value for all 96 entries; bitflag combinations, consumers, mutation/protection, persistence, aliases, and lifecycle remain unclaimed. Classified 94 removed legacy alias rows as `evidence-required`/`unsafe`; bootstrap omission does not prove full runtime/dynamic publication absence, historical timing, replacement semantics, or all-LoD absence. Totals are now **426 best-effort, 602 evidence-required, 2 exception-requested, and 2380 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 HousingCatalog constant removals

Classified the 16 removed `Constants.HousingCatalogConsts.*` rows as `evidence-required`/`unsafe` using checked-in 12.0.0 source-register evidence plus current `src/lua_api/globals/enum_data/constants_values.lua` bootstrap evidence. The simulator bootstrap omits the keys while retaining the namespace, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Tests/assertions remain empty with null commit, approval, scope exception, load_addon, and provenance_only. Totals are now **330 best-effort, 508 evidence-required, 2 exception-requested, and 2570 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 CAA constants

Classified exactly 32 added `Constants.CAAConstants` rows as `best-effort`/`behavioral` using `src/loader/tests/wow_api_globals/caa_constants.rs::test_patch_12_0_0_caa_constants_publish_exact_values`. The focused startup test asserts namespace publication, exact Lua type, and exact value for all 32 rows. Source implementation ancestor is `cf0908682a897f314da15dc3ae4f9c12c03cf6f0`; current source/test hashes are recorded per row. Claims exclude CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics. Totals are now **330 best-effort, 492 evidence-required, 2 exception-requested, and 2586 untriaged**.

## [2026-08-08] investigation | Classify remaining structure declarations

Classified the nine remaining 12.0.0 structure declarations as `evidence-required`/`unsafe`: three added typed-structure rows (`LuaColorCurvePoint`, `NumberAbbrevData`, `NumberAbbrevOptions`) and six removed/removal-sensitive rows. Related proxy/curve tests do not prove exact added typed fields or payloads; source metadata, method absence, and auxiliary token checks do not prove removed structure/replacement/mixin identity. Tests/assertions remain empty with null commit, approval, and scope exception. Totals are now **298 best-effort, 492 evidence-required, 2 exception-requested, and 2618 untriaged**.

## [2026-08-08] investigation | Migrate remaining pure structure declarations

Migrated exactly 23 added structure declarations to best-effort/provenance-only. Their field/API rows remain separate; no runtime behavior is claimed. Nine structure declarations remain untriaged because they are behavior-linked or removal-sensitive. Totals are now **298 best-effort, 483 evidence-required, 2 exception-requested, and 2627 untriaged**.

## [2026-08-08] Migrate 12.0.0 structure declaration provenance rows

Migrated ten structure declarations to the machine-validated `best-effort`/`provenance-only` contract: the six `C_DamageMeter` declarations, `C_ActionBar.ActionBarChargeInfo`, `C_ActionBar.ActionBarCooldownInfo`, `C_CatalogShop.BulkPurchaseIndividualProductResult`, and `C_CatalogShop.RefundableDecorInfo`. Their field/API rows remain untriaged; no payload or runtime behavior is claimed. Totals are now **275 best-effort, 483 evidence-required, 2 exception-requested, and 2650 untriaged**.

## [2026-08-08] investigation | Migrate 12.0.0 typedef provenance rows

Migrated all 21 untriaged `typedef.*` rows to the machine-validated `best-effort`/`provenance-only` contract. Each retains owner/category and source-register evidence only, sets `provenance_only: true`, has empty runtime proof fields, and uses exact notes `Provenance-only: no runtime behavior claimed.` This bookkeeping status claims no simulator-visible runtime behavior and is completion-eligible without a commit. Totals are now **265 best-effort, 483 evidence-required, 2 exception-requested, and 2660 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 UnitHealPredictionCalculator luaobject methods

Classified `added:UnitHealPredictionCalculator.GetDamageAbsorbClampMode` as `best-effort`/`behavioral` using `tests/userdata_proxy.rs::heal_prediction_set_get_roundtrip` and implementation ancestor `4d7dbe1da`; the claim is limited to the tested local setter/getter round-trip. Classified the other 13 currently untriaged UnitHealPredictionCalculator luaobject-method rows as `evidence-required`/`unsafe` with empty tests/assertions and null commit/approval/scope exception because the generic proxy does not establish exact absorb/mode/default/reset/predicted-payload, secret, lifecycle, validation, or edge semantics. All 68 luaobject-method rows are now non-untriaged. Totals are now **244 best-effort, 483 evidence-required, 2 exception-requested, and 2681 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 script-object rows

Classified five added script-object rows as `best-effort`/`behavioral` using existing direct proxy/state tests: `script_object.AbbreviateConfigAPI`, `script_object.LuaColorCurveObjectAPI`, `script_object.LuaCurveObjectAPI`, `script_object.LuaDurationObjectAPI`, and `script_object.UnitHealPredictionCalculatorAPI`. Claims are limited to tested table/method shape, state round-trips, scalar evaluation/copy behavior, per-instance fields/tostring, duration identity, and fixture prediction state as recorded per row in `data/patch-api/12.0.0.json`; retail abbreviation/curve/timing/clock/secret/heal-prediction lifecycle and edge fidelity remain unproven. Classified `script_object.FrameAPITooltip` and `script_object.LuaCurveObjectBaseAPI` as `evidence-required`/`unsafe` because construction/base contracts remain unmodeled or incompletely tested; generated documentation and indirect concrete-object tests are not runtime proof. All seven script-object rows are now non-untriaged. Totals are now **243 best-effort, 470 evidence-required, 2 exception-requested, and 2695 untriaged**.

## [2026-08-08] investigation | Classify remaining 12.0.0 UI-method rows

Classified `changed:StatusBar.SetMinMaxValues` as `best-effort`/`behavioral` using `tests/widget_methods_colorselect.rs::test_statusbar_set_min_max_values_clamps_existing_value`. The test stores values before narrowing/shifting ranges and verifies `(10, 50)` clamps 80 to 50 and `(30, 40)` clamps 20 to 30; the claim excludes interpolation-target behavior, rendering/events, invalid/reversed ranges, and edge semantics. Classified `changed:StatusBar.GetFillStyle`, `changed:StatusBar.SetFillStyle`, and `added:TextureBase.SetSpriteSheetCell` as `evidence-required`/`unsafe`: the fill-style getter returns constant STANDARD and breaks nonstandard round-trips, while SetSpriteSheetCell is a no-op. Exact styles, validation, sprite-cell mapping, optional dimensions, rendering, and edge semantics remain unproven. Totals are now **238 best-effort, 468 evidence-required, 2 exception-requested, and 2702 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 secret/no-3D UI methods

Classified `added:Model.SetUseGBuffer` as `exception-requested`/`impossible` under the already-decided permanent no-3D project scope; this is not a user approval request and no model-callability behavior is claimed. Classified `added:Region.IsAnchoringSecret`, `added:UIObject.HasAnySecretAspect`, `added:UIObject.HasSecretAspect`, `added:UIObject.HasSecretValues`, `added:UIObject.IsPreventingSecretValues`, and `added:UIObject.SetPreventSecretValues` as `evidence-required`/`unsafe`. Current local flag/aspect behavior and frame-state tests establish simulator consistency only, not authoritative secret/taint semantics, aspect mapping/aggregation, propagation, authorization, anchoring relationships, or lifecycle; tests remain empty with null commit, approval, and scope exception. Totals are now **237 best-effort, 465 evidence-required, 2 exception-requested, and 2706 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 UI-method state batch

Classified `added:Frame.IsIgnoringChildrenForBounds`, `added:Frame.SetIgnoringChildrenForBounds`, and `added:Region.SetAlphaFromBoolean` as `best-effort`/`behavioral` using the focused frame-state and alpha tests. Claims are limited to false/true/false stored-state mutation, true/false alpha branch selection, and same-value no-op dirty behavior; actual bounds/layout effects, full alpha defaults/clamping/propagation, rendering, invalid arguments, lifecycle, and edge semantics remain unproven. Classified `added:Cooldown.SetPaused`, `added:FontString.GetScaleAnimationMode`, `added:FontString.SetScaleAnimationMode`, and `added:LayeredRegion.SetVertexColorFromBoolean` as `evidence-required`/`unsafe` because no matching simulator implementation/test was found; exact contracts require authoritative evidence or a correct model/test, with empty tests/assertions and null commit/approval/scope exception. The checked-in manifest records both FontString rows as `added` occurrences. Totals are now **237 best-effort, 459 evidence-required, 1 exception-requested, and 2713 untriaged**.

## [2026-08-08] investigation | Correct ResetTexCoord audit evidence

Updated `added:TextureBase.ResetTexCoord` after commit `090af1ec8` corrected `tests/texture_methods_port.rs::test_reset_tex_coord_restores_defaults` to assert WoW's eight-corner `GetTexCoord` result `(0,0, 0,1, 1,0, 1,1)` instead of the obsolete four-value contract. Refreshed the test hash and manifest wording; best-effort scope remains limited to SetTexCoord-then-ResetTexCoord default reset, with atlas-specific reset, rendering, invalid arguments, and edge semantics unproven.

## [2026-08-08] investigation | Classify tested 12.0.0 UI method batch

Classified six rows as `best-effort`/`behavioral` using existing direct tests: actual register IDs are `added:GameTooltip.GetLeftLine`, `added:GameTooltip.GetRightLine`, `added:TextureBase.ResetTexCoord`, `changed:StatusBar.SetValue`, `added:Frame.RegisterEventCallback`, and `added:Frame.RegisterUnitEventCallback`. The manifest records source/test SHA-256 hashes plus exact implementation ancestors `4ef55ce2cf0737da018279923edd49f075eda820`, `2591694d10dca666a09593cc5ffff121193171f9`, and `7a7a440402f4b03a89fe341956b6cb5e2051f465`; test ancestors are `fcc633ce286002f95b758635427fa1ba7720838a`, `c473e49cdaac6fbb882c43e4bb2a24456debe5e5`, `0c2c6a966d0d5351d42b8d227d1ee35747e92fde`, `7819ea6620139616fc208fc9d0b30d307adf636c`, and `118a2c9a3b56c5d700f0de0994edfac0df082e7f`. Claims are limited to tested tooltip line lookup, default texture-coordinate reset, StatusBar immediate/Smooth value state, and event/unit-callback dispatch. `added:TextureBase.SetSpriteSheetCell`, `changed:StatusBar.GetFillStyle`, `changed:StatusBar.SetFillStyle`, and `changed:StatusBar.SetMinMaxValues` remain untriaged because existing tests do not assert their contracts. Rendering/layout/animation, invalid/edge inputs, lifecycle/validation, and taint/security semantics remain unproven. Totals are now **234 best-effort, 455 evidence-required, 1 exception-requested, and 2720 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 GameTooltip.SetText

Classified `GameTooltip.SetText` as `best-effort`/`behavioral` using `tests/tooltip_basic.rs::test_settext_clears_and_sets_first_line`. Implementation ancestor is `fa8cd0e2fc`; test-file ancestor is `fcc633ce28`; current evidence hashes are `src/lua_api/frame/methods/text_attribute_event/text.rs`=`ae628de726915e32c5a4c1472e9d0395c7a9253b1e9a546c900bcc6b911d90c2` and `tests/tooltip_basic.rs`=`a0ea03f766f01b53131a9eea7c18c2d55451faa60cf1c06b1ce8840598c001f9`. The claim is limited to clearing existing tooltip lines, inserting supplied text as the first line, and the focused `NumLines() == 1` assertion; formatting, wrapping, colors, localization, rendering, invalid arguments, and edge semantics remain unproven. Totals are now **228 best-effort, 455 evidence-required, 1 exception-requested, and 2726 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 TextureBase atlas methods

Classified `TextureBase.GetTexCoord` and `TextureBase.SetAtlas` as `best-effort`/`behavioral` using the focused tests in `tests/methods_texture.rs`. The claim is limited to atlas remapping including partial UVs, known/unknown lookup, direct tile-slice selection, tiling flags and clearing, and render-preferred 2x path selection. Implementation ancestors are `50e028c9ef` for full-coordinate GetTexCoord behavior and `2591694d10` for the current atlas-method file; tiling and 2x path behavior are retained from `8bbadbf8c8` and `0c63d580ce`; test-file ancestor is `257f13c34e`. Current evidence hashes are `src/lua_api/frame/methods/widgets/texture/coords.rs`=`0aa45b65e0f427602ec88569039d3b5cc44ee5a4636f010aa51cdbc3c63f4d95`, `src/lua_api/frame/methods/widgets/texture/atlas.rs`=`ec7681bfe7e11c4692a1e5290423fd1c2332874555147964910bff488742a625`, and `tests/methods_texture.rs`=`419c30bfd97df89dbc9bb960de00690f769d6ab60e35354a3da9938a0e78361c`. Complete atlas fidelity, CASC/texture loading beyond these assertions, filtering/wrap edge cases, invalid arguments, and rendering correctness remain unproven. Totals are now **227 best-effort, 455 evidence-required, 1 exception-requested, and 2727 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 GameTooltip layout methods

Classified `GameTooltip.GetMinimumWidth`, `GameTooltip.SetMinimumWidth`, `GameTooltip.GetPadding`, and `GameTooltip.SetPadding` as `best-effort`/`behavioral` using `tests/tooltip_basic.rs::test_setminimumwidth_and_getminimumwidth` and `tests/tooltip_basic.rs::test_setpadding_and_getpadding`. Implementation ancestor is `4a4621c2e`; test-file ancestor is `fcc633ce2`. Current evidence hashes are `src/lua_api/frame/methods/widgets/tooltip/line_data.rs`=`469dd6b58b37b0a47bc800a5e6fe08eb3b0a27b438b0365c193100ce3e7d28b2` and `tests/tooltip_basic.rs`=`a0ea03f766f01b53131a9eea7c18c2d55451faa60cf1c06b1ce8840598c001f9`. The claim is limited to setter/getter state round-trips for minimum width 150 and padding 8; tooltip rendering/layout effects, clamping, invalid arguments, and edge semantics remain unproven. Totals are now **225 best-effort, 455 evidence-required, 1 exception-requested, and 2729 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 Cooldown methods

Classified `Cooldown.GetCountdownFontString`, `Cooldown.SetCooldownFromDurationObject`, and `Cooldown.SetCooldownFromExpirationTime` as `best-effort`/`behavioral` using `tests/cooldown_widget.rs::cooldown_widget_methods_persist_runtime_state`. Implementation ancestors are `d7a3cf21b` for countdown-font/expiration behavior and `a1f638733` for duration-object behavior. Current evidence hashes are `src/lua_api/frame/methods/widgets/cooldown.rs`=`527213c02fce0cda3bc7c6fd5f0874d3eea3d4e28dc59de2fb086cb32ac2e2b7` and `tests/cooldown_widget.rs`=`07a66162ff33fd6245879fc0d962ffa10cb9ca5026e8a9bf3fdcd651b4d19b8d`. The claim is limited to FontString creation/type, expiration-to-start/duration conversion, duration-object start/total-duration/mod-rate access, and zero-duration clearing; retail rendering, time progression, formatting, invalid arguments, and edge semantics remain unproven. Totals are now **221 best-effort, 455 evidence-required, 1 exception-requested, and 2733 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 StatusBar interpolation methods

Classified `StatusBar.GetInterpolatedValue`, `StatusBar.IsInterpolating`, and `StatusBar.SetToTargetValue` as `best-effort`/`behavioral` using `tests/widget_methods_colorselect.rs::test_statusbar_interpolation_methods_track_target_and_displayed_value` and ancestor implementation commit `24e44f3f0`. The claim is limited to the tested interpolation state machine: Smooth target assignment leaves the displayed value unchanged, `GetValue` is target-facing, `GetInterpolatedValue` returns the displayed value, `IsInterpolating` reports target presence, and `SetToTargetValue` snaps and clears interpolation. Timing/animation progression, repeated-target behavior, invalid modes, render, and event fidelity remain unproven. Totals are now **218 best-effort, 455 evidence-required, 1 exception-requested, and 2736 untriaged**.

## [2026-08-08] investigation | Classify 12.0.0 legacy combat-log cursor globals

Classified `CombatLogAdvanceEntry` and `CombatLogSetCurrentEntry` as `evidence-required`/`unsafe`. The checked-in 12.0.0 register records both removals; pinned retail/PTR `Blizzard_DeprecatedCombatLog` sources do not publish them; the temporary simulator model retains fixture-only cursor mutation for Wrath/Mists `Blizzard_CombatLog` callers; and no equivalent replacement contract or authoritative legacy semantics is established. Tests remain empty with null commit, approval, and scope exception. Totals are now **215 best-effort, 455 evidence-required, 1 exception-requested, and 2739 untriaged**.

## [2026-08-08] investigation | Prove 12.0.0 SpellGetVisibilityInfo vendor wrapper

Classified `SpellGetVisibilityInfo` as `best-effort`/`vendor-present` using `patch-tests/patch_12_1/vendor_deprecated_chat_spell.rs::vendor_deprecated_chat_spell_globals_are_published_and_forward` at `ed3ad9d87`. The focused full-LoD proof checks publication under enabled `loadDeprecationFallbacks`, string-to-Enum translation for `RAID_INCOMBAT`, sentinel forwarding, and unknown visibility-name nil forwarding; it does not claim complete `C_Spell` visibility semantics. `CombatLogAdvanceEntry` and `CombatLogSetCurrentEntry` remain untriaged. Totals are now **215 best-effort, 453 evidence-required, 1 exception-requested, and 2741 untriaged**.

## [2026-08-08] investigation | Prove 12.0.0 deprecated chat/spell globals

Classified `CancelEmote`, `DoEmote`, `SpellIsPriorityAura`, and `SpellIsSelfBuff` as `best-effort`/`vendor-present` using `patch-tests/patch_12_1/vendor_deprecated_chat_spell.rs::vendor_deprecated_chat_spell_globals_are_published_and_forward` at `02db1895a`. The focused full-LoD proof checks publication under enabled `loadDeprecationFallbacks`, CancelEmote forwarding, DoEmote named and nil branches, and spell-wrapper forwarding; it does not claim complete legacy semantic fidelity. `CombatLogAdvanceEntry`, `CombatLogSetCurrentEntry`, and `SpellGetVisibilityInfo` remain untriaged. Totals are now **214 best-effort, 453 evidence-required, 1 exception-requested, and 2742 untriaged**.

## [2026-08-08] investigation | Prove 12.0.0 simulator legacy compatibility globals

Classified exactly seven removed globals (`FindBaseSpellByID`, `FindFlyoutSlotBySpellID`, `FindSpellOverrideByID`, `GetBattlegroundInfo`, `PlaySound`, `SetPortraitToTexture`, and `strtrim`) as `best-effort`/`compat` using the focused full-LoD proof `patch-tests/patch_12_1/legacy_compat.rs::simulator_legacy_compat_globals_preserve_tested_behavior` at `abba2bd2a`. The claim is limited to tested wrapper forwarding, seeded battleground and unknown-ID behavior, numeric sound acceptance without audio fidelity, portrait circular masking without duplicates, and default/custom trimming. `SpellGetVisibilityInfo` and the other unresolved simulator-published legacy globals remain untriaged. Totals are now **210 best-effort, 453 evidence-required, 1 exception-requested, and 2746 untriaged**.

## [2026-08-08] investigation | Prove 12.0.0 vendor deprecated globals

Classified exactly 56 removed legacy global API rows as `best-effort`/`vendor-present` using the committed full-LoD proof `patch-tests/patch_12_1/strict_removals.rs::vendor_deprecated_globals_are_published_and_forward` at `a26692e00`. The 35 ActionBar, 3 BattleNet, 10 CombatLog, 2 CombatText, 3 DeathRecap, and 3 InstanceEncounter wrappers are published by Blizzard deprecated addons when `loadDeprecationFallbacks` is enabled; representative forwarding/alias checks prove publication ownership, not complete legacy semantic fidelity. Fourteen other diagnosed published globals remain untriaged. Totals are now **203 best-effort, 453 evidence-required, 1 exception-requested, and 2753 untriaged**.

## [2026-08-08] investigation | Prove 12.0.0 legacy global removals

Classified exactly four removed legacy global API rows (`IsConsumableSpell`, `SetRaidTargetProtected`, `SpellIsAlwaysShown`, and `StripHyperlinks`) as `best-effort`/`behavioral` using the committed full-LoD `rawget(_G, name)` probe `patch-tests/patch_12_1/strict_removals.rs::removed_legacy_global_apis_are_absent_after_full_lod_load` at `3471c5a4c`. The claim is limited to current publication absence; no source-scanner, replacement-behavior, or historical timing claim is made. The other 70 diagnosed published removed globals remain untriaged pending vendor/simulator provenance. Totals are now **147 best-effort, 453 evidence-required, 1 exception-requested, and 2809 untriaged**.

## [2026-08-08] investigation | Prove 12.0.0 C_Transmog removals

Classified exactly 18 removed `C_Transmog` API rows as `best-effort`/`behavioral` using the committed full-LoD runtime probe `patch-tests/patch_12_1/strict_removals.rs::removed_transmog_methods_are_absent_after_full_lod_load` at `2975b0ad6`. The probe uses `rawget` to prove current publication absence while five retained C_Transmog APIs remain callable; source scanning is auxiliary only. Removed the two obsolete simulator registrations and obsolete direct test expectations. The three removed `TransmogApplyWarningInfo` structure/field rows remain untriaged because runtime rawget does not prove metadata removal; the 11 added/changed slot-visual rows remain evidence-required. Totals are now **124 best-effort, 453 evidence-required, 1 exception-requested, and 2832 untriaged**.

## [2026-08-08] investigation | Prove 12.0.0 C_TransmogCollection removals

Classified exactly 10 removed `C_TransmogCollection` outfit API rows as `best-effort`/`behavioral` using the committed full-LoD runtime probe `patch-tests/patch_12_1/strict_removals.rs::removed_transmog_collection_outfit_methods_are_absent_after_full_lod_load` at `c3ae90c26`. The probe uses `rawget` to prove current publication absence while retained appearance methods remain callable; source scanning is auxiliary only. Removed the three obsolete simulator outfit placeholders and their tests. The 23 added/changed custom-set and appearance-source rows remain evidence-required; custom-set replacement APIs are not claimed implemented, and no clean replacement surface, historical load-order timing, or broad scanner completeness is claimed. Totals are now **106 best-effort, 453 evidence-required, 1 exception-requested, and 2850 untriaged**.

## [2026-08-08] investigation | Bound 12.0.0 C_CombatLog slice

Classified all 11 added `C_CombatLog` API rows as `evidence-required`/`unsafe` using checked-in source-register evidence and the shared permissive/fixture-backed `src/lua_api/workarounds/temporary/combat_log_state.rs` model. Tests remain empty with null commit, approval, and scope exception. Filter schema/matching, restriction state, retention/message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven; no approval can close these rows. Totals are now **77 best-effort, 453 evidence-required, 1 exception-requested, and 2879 untriaged**.

## [2026-08-08] investigation | Bound 12.0.0 C_CombatLogSecure slice

Classified all nine added `C_CombatLogSecure` secure-only API rows as `evidence-required`/`unsafe` using checked-in source-register evidence and the permissive/fixture-backed `src/lua_api/workarounds/temporary/combat_log_state.rs` model. Tests remain empty with null commit, approval, and scope exception. Secure/taint enforcement, filtering rules, event/message payload shape, navigation semantics, and entry lifecycle remain unproven; no approval can close these rows.

## [2026-08-08] investigation | Bound 12.0.0 C_UnitAuras slice

Classified exactly 10 added/changed `C_UnitAuras` API rows as `evidence-required`/`unsafe` using checked-in source-register evidence and the current `src/lua_api/globals/auras.rs` plus temporary `unit_auras_state.rs` seeded aura model. Tests remain empty with null commit, approval, and scope exception. Source signatures/defaults and adjacent seeded aura behavior do not establish defensive classification, expiration/display formatting, duration objects, refresh calculations, color curves, sorted instance IDs, private callback dispatch, or GetUnitAuras sort semantics; no approval can close these rows. Totals are now **77 best-effort, 433 evidence-required, 1 exception-requested, and 2899 untriaged**.

## [2026-08-08] investigation | Bound 12.0.0 C_UnitAurasPrivate slice

Classified all 10 added/changed `C_UnitAurasPrivate` secure-only API rows as `evidence-required`/`unsafe` using checked-in source-register evidence and the current `src/lua_api/workarounds/temporary/private_aura_state.rs` permissive/partial model. Tests remain empty with null commit, approval, and scope exception. Secure enforcement, private-aura visibility, callback/anchor lifecycle, callback payloads, and the two absent APIs remain unproven; existing tests prove simulator-only seeded state/callback behavior and are intentionally not attached, so no approval can close these rows. Totals are now **77 best-effort, 423 evidence-required, 1 exception-requested, and 2909 untriaged**.

## [2026-08-08] investigation | Bound 12.0.0 C_SpellDiminish slice

Classified exactly 14 added `C_SpellDiminish` API, structure, and structure-field rows as `evidence-required`/`unsafe` using checked-in source-register evidence and the current `src/c_api/c_spell_diminish.rs` static eight-category fixture. Tests remain empty with null commit, approval, and scope exception. The source register establishes signatures and field names only; the fixture and local tests do not establish authoritative 12.0.0 category contents, ruleset tracking semantics, or tracker payload fields, and no approval can close these rows. Totals are now **77 best-effort, 423 evidence-required, 1 exception-requested, and 2909 untriaged**.

## [2026-08-08] investigation | Bound 12.0.0 C_DeathRecap slice

Classified exactly four added `C_DeathRecap` API/structure rows as `evidence-required`/`unsafe` using checked-in source-register evidence, pinned API documentation/vendor-consumer evidence, and the current `src/c_api/c_death_recap.rs`/`src/lua_api/state_types/mythic_plus_scenario.rs` killing-blow-only model. Tests remain empty with null commit, approval, and scope exception. Pinned retail/PTR/Wowless documentation exposes no `DeathRecapEventInfo` fields; vendor consumers reveal only partial amount/timestamp/sourceGUID usage; recap-event fields, link format, recap-ID selection, default/no-argument behavior, unknown-ID handling, and event-presence semantics remain unproven. `HasRecapEvents` is not classified best-effort by inference, and no approval can close these rows. Totals are now **77 best-effort, 399 evidence-required, 1 exception-requested, and 2933 untriaged**.

## [2026-08-08] investigation | Bound 12.0.0 C_TransmogCollection slice

Classified exactly 23 added/changed `C_TransmogCollection` custom-set and appearance-source rows as `evidence-required`/`unsafe` using checked-in source-register evidence, the seeded/partial `src/lua_api/globals/missing_surface/transmog_collection.rs` surface, and `src/lua_api/state_types/collections.rs`; tests remain empty with null commit, approval, and scope exception. The 10 removed outfit rows remain untriaged because removal direction alone does not establish replacement behavior. Custom-set lifecycle, hyperlinks, persistence, validation, and exact `TransmogAppearanceSourceInfoData` semantics remain unproven, and related local tests do not establish authoritative retail 12.0.0 behavior; no approval can close these rows. Totals are now **77 best-effort, 395 evidence-required, 1 exception-requested, and 2937 untriaged**.

## [2026-08-08] investigation | Bound 12.0.0 C_TransmogOutfitInfo slice

Classified exactly 115 added `C_TransmogOutfitInfo` API, structure, and structure-field rows as `evidence-required`/`unsafe` using checked-in source-register evidence and `src/lua_api/globals/transmog_outfit_info.rs`; tests remain empty with null commit, approval, and scope exception. The two present lock queries use only local state behavior, the other 113 rows are unmodeled, and local tests do not establish authoritative retail 12.0.0 semantics. Authoritative live evidence or a correct modeled transmog-outfit subsystem with focused tests is required, and no approval can close these rows. Totals are now **77 best-effort, 355 evidence-required, and 2978 untriaged**.

# Wiki Log

## [2026-08-08] investigation | Bound 12.0.0 C_Transmog slice

Classified exactly 11 non-removed `C_Transmog` structure/API rows as `evidence-required`/`unsafe` using checked-in source-register evidence and the current `src/lua_api/globals/missing_surface/transmog.rs` partial surface; tests remain empty with null commit, approval, and scope exception. The source records only names and the `GetSlotVisualInfo` signature transition, while no direct slot-visual/pending/apply state model or behavioral tests establish authoritative retail 12.0.0 semantics. The 21 removed C_Transmog rows remain untriaged because removal direction alone does not establish replacement behavior; no unrelated collection/outfit tests were used and no approval can close these rows. Totals are now **77 best-effort, 372 evidence-required, 1 exception-requested, and 2960 untriaged**.

## [2026-08-08] investigation | Prove 12.0.0 C_NamePlate removals

Classified six added 2D `C_NamePlate`/`C_NamePlateManager` APIs as `evidence-required`/`unsafe` because the current permanent shim has no modeled nameplate-manager state and exact 12.0.0 semantics remain unproven. Classified `C_NamePlateManager.IsNamePlateUnitBehindCamera` as `exception-requested`/`impossible` under the already-decided permanent no-3D project scope; this is not a user approval request. Classified all 19 removed `C_NamePlate` rows as `best-effort`/`behavioral` using the committed full-LoD runtime probe `patch-tests/patch_12_1/strict_removals.rs::removed_nameplate_methods_are_absent_after_full_lod_load` at `2c2c5ad1d`. The probe uses `rawget` to prove current publication absence while retained APIs remain callable; source scanning is auxiliary only and does not claim historical load-order timing or broad scanner completeness. Totals are now **96 best-effort, 453 evidence-required, 1 exception-requested, and 2860 untriaged**.

## [2026-08-08] investigation | Bound 12.0.0 C_EncounterTimeline slice

Classified exactly 55 added `C_EncounterTimeline` API, structure, and structure-field rows as `evidence-required`/`unsafe` using checked-in source-register evidence and the current `src/lua_api/workarounds/temporary/encounter_state.rs` partial seeded fixture; tests remain empty with null commit, approval, and scope exception. Nine APIs are present only as fixture-backed behavior and 46 rows are absent; exact encounter state, script-event lifecycle, timers, feature flags, payload values, and structure-field semantics require authoritative live evidence or a correct modeled subsystem with focused tests, and no approval can close these rows. Totals are now **77 best-effort, 240 evidence-required, and 3093 untriaged**.

## [2026-08-08] investigation | Bound 12.0.0 C_PingSecure slice

Classified exactly 15 changed `C_PingSecure` API rows as `evidence-required`/`unsafe` using checked-in source-register evidence and clean current `src/c_api/c_ping_secure.rs`; tests remain empty with null commit, approval, and scope exception. The source contract is secure-only while current behavior is no-op/inert callback storage/partial or absent; exact secure-call enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and `PingResult` semantics require authoritative live evidence or a correct ping/security model and direct tests, and no approval can close these rows. Totals are now **77 best-effort, 185 evidence-required, and 3148 untriaged**.

## [2026-08-08] investigation | Bound 12.0.0 C_Secrets slice

Classified exactly 23 added `C_Secrets` API rows as `evidence-required`/`unsafe` using checked-in source-register evidence and the examined current `src/lua_api/globals/register.rs` surface; tests remain empty with null commit, approval, and scope exception. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. Totals are now **77 best-effort, 170 evidence-required, and 3163 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 C_EncounterWarnings slice

Classified exactly 19 added `C_EncounterWarnings` structure/API rows as `evidence-required`/`unsafe` using checked-in source-register evidence and clean current `encounter_warnings.rs`; tests remain empty with null commit, approval, and scope exception. `GetEditModeWarningInfo`/current structure fields are fabricated preview/static payload behavior, `PlaySound` is a no-op, and the other three methods lack examined registration; exact state, payload meanings, feature flags, severity sound mapping, and audio playback require authoritative evidence or a correct modeled subsystem/test. Totals are now **77 best-effort, 147 evidence-required, and 3186 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 C_CombatAudioAlert slice

Classified exactly 12 added `C_CombatAudioAlert` rows as `evidence-required`/`unsafe` using checked-in source-register evidence and the examined current `src/lua_api/globals/register.rs` surface; tests remain empty with null commit, approval, and scope exception. exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the rows. Totals are now **77 best-effort, 128 evidence-required, and 3205 untriaged**.

## [2026-08-07] investigation | Bound remaining 12.0.0 C_ActionBar runtime slice

Classified four C_ActionBar rows as best-effort/behavioral using only source-register, current action-bar implementation/registration, and the named direct/end-to-end tests; claims are limited to exact tested seeded/empty/malformed profession quality, modeled action-slot presence/texture, and modeled outfit-lock slot behavior. The remaining 22 action queries/registration rows are evidence-required/unsafe with no approval path; the 11 ActionBarChargeInfo/ActionBarCooldownInfo structure/field rows remain untriaged. Totals are now **77 best-effort, 116 evidence-required, and 3217 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 C_ActionBar page/state-query slice

Classified exactly 13 `C_ActionBar` page/state-query rows as `best-effort`/`behavioral` from the source register, current action-bar implementation/registration, ancestor commits `dacb23419`, `bdf741cf4`, `0859e07be`, `06bf6616a`, `5b9497307`, `6425b3f0b`, `080358cdb`, `6d13156ae`, and `167e23297`, and only the named page/state/default/transition tests; claims remain limited to tested state/default/transition behavior, with exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics unproven. Classified exactly three rows as `evidence-required`/`unsafe` with source-register and current implementation evidence, empty tests, and null commit/approval/scope exception; authoritative semantics or a correct model/test are required, and no approval can close them. Totals are now **73 best-effort, 94 evidence-required, and 3243 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 C_DamageMeter triage

Classified exactly 19 C_DamageMeter rows as `best-effort`/`behavioral` from the source register, clean temporary seeded-state implementation, ancestor commit `e005f99e`, and only the named seeded/empty/zero-ID/unit-detail tests; claims remain limited to exact seeded/empty lookup and shape/type assertions, with no complete retail aggregation/lifecycle/secret fidelity. Classified exactly 10 rows as `evidence-required`/`unsafe` with source-register and current implementation evidence, empty tests, and null commit/approval/scope exception; seeded-but-unasserted fields and the unimplemented reset lifecycle require authoritative semantics or a correct model/test, and no approval can close them. Six metadata-only structure rows remain untriaged. Totals are now **60 best-effort, 91 evidence-required, and 3259 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 C_TradeSkillUI slice

Classified `added:C_TradeSkillUI.GetDependentReagents` as `best-effort`/`behavioral` from source-register, current professions implementation/registration, exact tests, and ancestor commit `36f425fb2`; the claim is limited to table return/iteration safety and nil/malformed/unknown-reagent behavior. Classified exactly 11 quality/recraft/reagent-link rows as `evidence-required`/`unsafe` with source-register and current professions implementation/registration evidence, empty tests, and null commit/approval/scope exception; current evidence distinguishes absent methods, placeholder empty-table/true behavior, and unproven removal behavior. Authoritative profession semantics or a correct model/test are required, and no approval can close them. Totals are now **41 best-effort, 81 evidence-required, and 3288 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 C_Spell triage

Classified exactly 12 added `C_Spell` rows as `evidence-required`/`unsafe` using only the checked-in source register and clean current `src/c_api/c_spell.rs` evidence. Duration lifecycle, spell metadata, and boolean semantics remain unproven; `IsSelfBuff` has a same-name internal implementation, but no matching 12.0.0 publication/behavioral contract is proven. All tests/assertions are empty with null commit, approval, and scope exception; no approval can close these rows. Totals are now **40 best-effort, 70 evidence-required, and 3300 untriaged**.

Chronological record of wiki operations.

## [2026-08-07] investigation | Bound 12.0.0 C_ColorUtil slice

Classified `added:C_ColorUtil.GenerateTextColorCode` and `added:C_ColorUtil.WrapTextInColorCode` as best-effort behavioral from the checked-in source register, current color defaults, exact `installs_color_defaults` test, and ancestor commit `1abe6088d`; claims remain limited to tested RGB-to-`ffRRGGBB` conversion and explicit color-code wrapping. Classified four conversion rows and `added:C_ColorUtil.WrapTextInColor` as evidence-required unsafe with empty tests and no approval path. The register now totals **40 best-effort, 58 evidence-required, and 3312 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 C_Timer signature slice

Classified `changed:C_Timer.NewTimer` and `changed:C_Timer.NewTicker` as best-effort behavioral from checked-in signatures, current timer/proxy paths, ancestor commits `330e521be`/`3d767dbd2`, and the five named timer-container tests; claims remain limited to function/container acceptance, returned container identity/proxy equality, cancellation, and independent ticker counts. Classified `changed:C_Timer.After` as evidence-required unsafe because only an ignored focused-looking test exists; callback/lifecycle semantics require a correct modeled implementation and executable behavioral proof, with no approval path. The register now totals **38 best-effort, 53 evidence-required, and 3319 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 C_StringUtil slice

Classified `added:C_StringUtil.EscapeQuotedCodes` as best-effort behavioral from the checked-in source register, current `src/c_api/c_string_util.rs`, exact focused test, and ancestor commit `b3f579f70`; the claim is limited to quoted-code pipe escaping for tested plain/color-code cases. Classified eight unpublished `C_StringUtil` rows as evidence-required unsafe with no approval path. The register now totals **36 best-effort, 52 evidence-required, and 3322 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 curve-family triage

Classified nine curve factory/object rows as best-effort behavioral from the checked-in source register, temporary proxy factory, ancestor commit `22ab64e5e`, and only the relevant `userdata_proxy` tests; claims remain limited to tested factory/table shape, scalar interpolation/copy behavior, and color-object/copy shape. Classified 23 unresolved curve contracts as evidence-required unsafe with empty tests and no approval path because the generic proxy omits or does not faithfully establish their contracts; they cannot be approved closed. 17 curve-family metadata rows remain untriaged. The register now totals **35 best-effort, 44 evidence-required, and 3331 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 duration-object audit slice

Classified five duration/factory/StatusBar rows as best-effort behavioral from checked-in source, ancestor commits `35e39a58f`/`0a737bfbc`, and exact focused tests. Classified 21 duration-time/lifecycle/secret rows as evidence-required unsafe; current behavior is constant/no-op/incomplete and needs authoritative semantics or a correct modeled implementation, with no approval path. The register now totals **26 best-effort, 21 evidence-required, and 3363 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 FunctionContainer classification

Classified `changed:C_FunctionContainers.CreateCallback` and four `LuaFunctionContainer` rows as best-effort behavioral from checked-in source-register/proxy evidence and the five named `userdata_proxy` tests. Tested method exposure, cancellation/invoke suppression, per-instance fields, read-only keys, and tostring are covered; exact retail callback validation, metatable/equality identity beyond tests, timer integration, lifecycle/GC, and API metadata fidelity remain unproven. The register now totals **21 best-effort and 3389 untriaged**.

## [2026-08-07] investigation | Bound 12.0.0 classifications

Classified four `AbbreviateConfig` rows and twelve `UnitHealPrediction` rows as best-effort from bounded, evidence-backed tests. The 12.0.0 register now totals **16 best-effort and 3394 untriaged** (3410 occurrences); these limited-fidelity classifications do not complete the audit or satisfy `--complete`.

## [2026-08-07] architecture | Document 12.0.0 occurrence payloads

Documented the current 12.0.0 generator/source-register change: categorized occurrence objects may preserve optional typed `before`/`after` payloads containing normalized value and metadata for exact enum, constant, signature, and structure triage. Added/removed/changed and transient lifecycle rows carry the corresponding sides; row identity remains `direction+symbol`, unknown fields remain rejected, and counts/statuses/source SHA are unchanged.

## [2026-08-07] investigation | Document neutral 12.0.0 audit artifacts

Added [[patch-12-0-0-api-audit]] for the reproducible wowless snapshot register: boundary 11.2.7 build `65299` → final explicit 12.0.0 build `65727`, six 12.0.0 snapshots, 8 transient lifecycle rows, 3410 total occurrences, and all rows untriaged. Recorded the explicit limit: wowless schema provenance only, with no historical 12.0.0 FrameXML or live runtime claim; the active retail cache manifest is validation metadata rather than historical provenance.

## [2026-08-07] decision | Separate evidence-required from exception-requested

Migrated the 12.1 behavior register to **33 best-effort, 21 evidence-required, 0 exception-requested, 0 untriaged** and the 12.0.5 probe register to **33 best-effort, 4 evidence-required, 1 approved provenance-only exception-requested, 0 untriaged**. `evidence-required` marks triaged unresolved unsafe/impossible behavior requiring item-specific authoritative/live evidence; it requires no approval, commit, or focused test and cannot pass `--complete`. The 12.0.7 repository-authorized no-3D exception remains unchanged.

## [2026-08-07] investigation | Add strict-removal timing evidence guidance

Audited commit `12ed1355b`. Documented `StrictRemovalTimingProbe` as a collector for addon-visible lifecycle timing. Its presence does not resolve or close any 12.1 behavior row; strict-removal timing gaps remain open until raw retail/PTR SavedVariables captures are obtained and interpreted. The existing `ForbiddenAspectsProbe` remains the live evidence path for all six forbidden-aspect restrictions.

## [2026-08-07] investigation | Add private-object and forbidden-aspect probe guidance

Documented `PrivateScriptObjectProbe` and `ForbiddenAspectsProbe` as addon-tainted live-evidence tools. Their corresponding 12.1 rows remain open until raw captures are retained and interpreted; neither proves secure-caller behavior or unsupported internal/input paths.

## [2026-08-07] investigation | Add 12.1 live-client probe guidance

Documented [UnitAuraSecretProbe](../addons/UnitAuraSecretProbe/README.md) and [DurationTextBindingProbe](../addons/DurationTextBindingProbe/README.md) as evidence paths. UnitAura constrains addon-tainted AuraData and `UNIT_AURA` behavior but cannot establish Blizzard-secure caller access; DurationTextBinding constrains representation, identity, and lifetime observations but cannot prove native finalization. Corresponding 12.1 rows remain open until raw retail/PTR SavedVariables captures are retained and interpreted.

## [2026-08-07] decision | Reopen unverified 12.0.5 behavior exceptions

Applied the correct-behavior-only policy: reopened `ScaleEventProbe.SameSizeDuplicatePair` and `StoreForbiddenProbe.ForbiddenDescendants` by clearing their approvals. Kept `XmlFrameLevelProbe.RawCaptureProvenance` approved as provenance-only because its behavior is independently regression-tested. The 12.0.5 audit remains open at 33 best-effort, 5 exception-requested, 0 untriaged: 1 approved provenance-only exception and 4 open behavior exceptions (1 impossible same-size boundary and 3 unsafe Store/security gaps).

## [2026-08-07] investigation | Approve two 12.0.5 exceptions

Recorded explicit approvals for `ScaleEventProbe.SameSizeDuplicatePair` and `XmlFrameLevelProbe.RawCaptureProvenance`. Both remain `exception-requested` with `impossible` resolution; the three unsafe Store exceptions remained unapproved at this point.

## [2026-08-07] decision | Approve Store descendant exception

Recorded explicit approval for `StoreForbiddenProbe.ForbiddenDescendants` as an unsafe exception because the retained probe never captured the `/sfp` descendant matrix. Two unsafe Store exceptions remain unapproved; the 12.0.5 register remains 33 best-effort, 5 exception-requested, and 0 untriaged.

## [2026-08-07] investigation | Reclassify Texture radial progress rows

Corrected the three 12.1 RadialProgress rows to Texture-backed behavioral best-effort contracts. Focused Texture surface/state proof covers method availability, receiver dispatch, defaults, setters/getters, visual mode, and Clear reset; exact retail clamping and visual rendering remain best-effort. The broader register now has 33 best-effort, 21 exception-requested, and 0 untriaged rows, with no impossible candidates.

## [2026-08-07] investigation | Itemize remaining 12.1 exceptions

Converted all 24 remaining 12.1 broader behavior rows to item-specific `exception-requested` entries: 21 unsafe and 3 impossible. Each row has current repository source evidence, empty tests/assertions, `commit: null`, and `approval_id: null`; no exception is approved. The broader register now has 30 best-effort, 24 exception-requested, and 0 untriaged rows.

## [2026-08-07] investigation | 12.0.5 pending exception register

Converted the five remaining 12.0.5 probe rows to item-specific `exception-requested` entries: three unsafe Store/protection gaps and two impossible window/provenance gaps. Each row has hashed repository evidence, `approval_id: null`, empty tests/assertions, and awaits separate informed approval or new live evidence. The register now has 33 best-effort, 5 exception-requested, and 0 untriaged rows.

## [2026-08-07] investigation | TieredEntrance payload classification

Corrected the final 12.1 safe candidate from a tiered-aura label to `TieredEntrance` after confirming pinned PTR exposes `C_DelvesUI` `TieredEntranceTierInfo` / `TieredEntranceRewardInfo` and no corresponding aura API. Focused proof classifies deterministic tier/reward rows as best-effort; live reward IDs, quantities, unlock timing, eligibility, and economics remain outside the claim. The broader register now has 30 best-effort and 24 untriaged rows, with zero safe-best-effort rows remaining.

## [2026-08-07] investigation | Protected descendant-anchor probe replay

Classified `IsProtectedProbe.DescendantAnchorPropagation` as best-effort from focused behavioral evidence. The directly protected root returns true/true; child, grandchild, frames anchored to the root or child, and the root-keyed anchored frame remain false/false. The 12.0.5 register now has 33 best-effort and 5 untriaged rows.

## [2026-08-07] investigation | PlayerChoice and strict-load classification

Classified the state-backed `C_PlayerChoice` payload and mutator-intent contract plus the pre-removal pinned-PTR load window as best-effort. Focused proof covers default and seeded nested choice payloads and confirms representative compatibility symbols remain callable after the complete all-LoD load. The broader register now has 23 best-effort and 31 untriaged rows.

## [2026-08-07] architecture | PTR C_PlayerChoice local model

Documented commit `c64472e6e`: patch 12.1 `C_PlayerChoice` uses `SimState.player_choice` for deterministic query payloads and mutator-intent markers, including nested choice options and currency/item/reputation rewards. Explicit boundary: no claim of retail timing, server validation, reroll economics, or live service values.

## [2026-08-07] investigation | Cooldown, pet, and LFG payload classification

Added focused 12.1 payload tests for active/inactive spell cooldowns, seeded/unknown pet species, and seeded/unknown LFG search results. Classified all three local compatibility contracts as best-effort; secret fields and server-backed semantics remain unmodeled. The broader register now has 21 best-effort and 33 untriaged rows.

## [2026-08-07] investigation | Mouse-focus probe replay

Added a GUI-path replay of the retained two-frame DIALOG scenario. `GetMouseFocus()` and `GetMouseFoci()[1]` both retain the higher raw-level frame before and after `Raise`/`Lower`, then clear when both frames hide. The 12.0.5 register now has 32 best-effort and 6 untriaged rows.

## [2026-08-07] investigation | Battle.net service payload classification

Classified three broader 12.1 Battle.net rows as best-effort from focused local-state tests: deduplicated verified friend invites and returned fields, title-friend names/tags/feature and presence state, and the explicitly unsupported deterministic unit-invite result. Exact service validation, persistence, events, and eligibility remain unmodeled. The broader register now has 18 best-effort and 36 untriaged rows.

## [2026-08-07] investigation | Duration binding identity split

Split stable Lua-table identity from unknown Blizzard representation fidelity. Focused reference-retention/identity proof classifies lifetime and stable identity as best-effort; exact type/metatable/finalization fidelity remains a separate unsafe candidate. The broader register now has 54 rows: 15 best-effort and 39 untriaged.

## [2026-08-07] investigation | First 12.1 behavior classifications

Classified 13 broader 12.1 rows only where exact existing tests directly prove the simulator contract: aura-frame creation, DurationTextBinding formatter/color/FontString behavior, Discord state, four housing models, Encounter Journal difficulty guesses, and post-startup strict removal. Forty rows remain untriaged.

## [2026-08-07] investigation | 12.1 broader behavior register

Created a separate 53-row machine register for non-FrameXML 12.1 fidelity boundaries. All rows remain neutral; candidate disposition is 30 safe best-effort, 20 unsafe, and 3 impossible, with family summaries no longer serving as approval units.

## [2026-08-07] investigation | UI-scale CVar event ordering

Modeled successful `SetCVar` event dispatch and the retail `uiScale`/`useUiScale` boundary: new effective scale first, then `DISPLAY_SIZE_CHANGED` and `UI_SCALE_CHANGED` while the old CVar remains visible, then storage and `CVAR_UPDATE`. The 12.0.5 register now contains 31 best-effort and 7 untriaged rows.

## [2026-08-07] investigation | MessageFrame region replay

Added focused MessageFrame and ScrollingMessageFrame owner, region, anchor, and TextInsets proof. The 12.0.5 register now contains 30 best-effort and 8 untriaged rows.

## [2026-08-07] investigation | FontString size, anchor, and EditBox replays

Added focused no-size/partial-size/full-size FontString checks, explicit TOP/BOTTOM/LEFT/RIGHT/TOPLEFT anchor controls, and no-size/sized/inset EditBox backing-region proof. The 12.0.5 register now contains 29 best-effort and 9 untriaged rows.

## [2026-08-07] investigation | Frame-layer FontString anchor evidence

Mapped the retained unanchored frame-layer `justifyH`/`justifyV` matrix to its exact regression test. The 12.0.5 register now contains 26 best-effort and 12 untriaged rows.

## [2026-08-07] investigation | Panel pulse and DevTools dump replays

Added exact two-panel `ShowUIPanel`/`CloseAllWindows` lifecycle proof and a loaded-Blizzard-UI `DevTools_Dump` frame-array metadata replay. The 12.0.5 register now contains 25 best-effort and 13 untriaged rows.

## [2026-08-07] investigation | XML protected-frame replay

Added an exact XML `protected="true"` frame replay covering protection, forbidden state, absent legacy setters, failing setter calls, and retained protection. The 12.0.5 register now contains 23 best-effort and 15 untriaged rows.

## [2026-08-07] investigation | Retail protection probe replays

Added focused checks for absent retail `CreateForbiddenFrame` and the complete plain-frame protection/forbidden/legacy-setter sequence. Both probe rows are now best-effort, bringing the 12.0.5 register to 22 best-effort and 16 untriaged rows.

## [2026-08-07] correction | HookScript binding probe contract

Corrected the 12.0.5 source register to the retained retail behavior: the normal binding slot succeeds and chains, while explicit slots 0 and 2 return false and remain absent from `GetScript`. Existing focused tests directly prove that contract; classification totals are unchanged.

## [2026-08-07] investigation | Invalid SetAtlas argument evidence

Classified the retained nil, no-argument, boolean, numeric, empty-string, and unknown-atlas matrix from three focused behavioral tests. The 12.0.5 register now contains 20 best-effort and 18 untriaged rows.

## [2026-08-07] investigation | Identity and protection probe evidence

Classified surrogate identity dispatch, absent legacy `Protect`/`SetProtected`, and secure-template protection from exact focused tests. The 12.0.5 register now contains 19 best-effort and 19 untriaged rows.

## [2026-08-07] investigation | Animation script-handler probe replay

Added a focused Frame/AnimationGroup/nine-animation-subtype handler-matrix regression. The initial RED exposed a test misunderstanding—unsupported `HasScript` returns false without error, while `SetScript` rejects—then the corrected test passed and classified `AnimScriptProbe.HandlerMatrix` as best-effort. Current 12.0.5 totals: 16 best-effort and 22 untriaged.

## [2026-08-07] investigation | First evidence-backed 12.0.5 probe classifications

Classified 15 probe rows as best-effort only where focused behavioral tests directly exercise the recorded contract: scalar attributes, forbidden state, unit-event filters, wildcard attributes, frame ordering/identity, HookScript bindings, ButtonText anchors, texture path/clear behavior, and XML frame-level semantics. Twenty-three rows remain neutral; broad subsystem or register-shape tests are not accepted as probe evidence.

## [2026-08-07] investigation | 12.0.7 duration proof and conservative status correction

Extracted a focused duration clock/object/text-binding regression that directly exercises the modeled method families and verifies the stale formatting-options/raw-value factories remain unavailable. Corrected five overclaimed additive rows to best-effort, classified `ModelSceneActorBase.GetModelUnitGUID` as an impossible no-3D scope exception candidate, and pinned the register at 29 implemented, 101 best-effort, one exception-requested, and zero untriaged rows.

## [2026-08-07] system | repository scope exception for 12.0.7 no-3D gap

Migrated `changed:ModelSceneActorBase.GetModelUnitGUID` from redundant user-chat approval to the repository scope-exception mechanism. The row remains `exception-requested`/`impossible`; `AGENTS.md#intentional-gaps` is the validated authority for the permanent no-3D project boundary.

## [2026-08-06] system | Non-Lua exception evidence

Allowed `unsafe` and `impossible` manifest rows to omit fabricated Lua-path assertions only when item-specific evidence concerns provenance or another non-Lua boundary. Item-specific evidence and unique per-row informed approval remain mandatory for completion.

## [2026-08-06] investigation | 12.0.7 occurrence-level re-triage

Classified all 131 named 12.0.7 occurrences: 34 implemented and 97 best-effort, with no untriaged or exception-requested rows. Focused profile-gated tests cover safe globals, duration methods, CVar defaults, event registration, widget compatibility, and retained/removed compatibility surfaces. Exact service payload, secrecy, and load-order fidelity limits remain explicit best-effort notes; unnamed crawler claims remain metadata.

## [2026-08-06] investigation | 12.0.7 named CVar classification

Classified the six CVar additions actually named by the checked-in 12.0.7 crawler source as implemented. `patch_12_0_7_cvar_defaults_match_retail` verifies each profile-gated runtime/default value. The register now contains six implemented and 125 untriaged rows; unnamed crawler claims remain source metadata outside row classification.

## [2026-08-06] investigation | 12.0.7 approval-language correction

Removed stale claims that unnamed CVar source gaps, stale duration extraction rows, and Minimap removals were already approved exceptions. The 131-row machine manifest remains fully neutral/untriaged. Crawler-omitted CVar claims stay source metadata, and potential unsafe/impossible rows remain candidates until evidence-backed classification is presented for informed approval.

## [2026-08-06] investigation | Neutral 12.0.5 probe register

Created the checked-in 38-row probe source, machine manifest, generated checklist, and human inventory. Prior documentation states are preserved as 30 resolved, four best-effort, and four unresolved, while every machine row remains neutral/untriaged pending item-specific evidence. Removed stale broad-approval wording: Store dropdown/descendant evidence and XmlFrameLevel provenance remain unresolved, while same-size window transitions are an unapproved impossible exception candidate.

## [2026-08-06] system | Test-backed behavioral audit resolution

Added a generic `behavioral` resolution for patch occurrences that are not truthfully observable as Lua global/table paths, including event registration, widget methods, CVar defaults, and probe outcomes. Behavioral rows require hashed test evidence plus a focused named test, allow implemented/best-effort status, forbid Lua presence assertions, and consume no synthetic runtime observation.

## [2026-08-06] investigation | Neutral 12.0.7 occurrence register

Created the checked-in 12.0.7 machine manifest, generated checklist, and human inventory from the categorized source register. All 131 named occurrences remain neutral/untriaged: 79 added, 29 changed, and 23 removed. Changed occurrences preserve valid normalized symbol paths plus exact change details; crawler-omitted CVar names remain unresolved metadata and are not invented.

## [2026-08-06] system | Categorized 12.0.7 occurrence source

Added a generic categorized patch-source format and the raw 12.0.7 occurrence register. The source contains 79 added, 29 changed, and 23 removed named occurrences (131 total), grouped from globals, script-object methods, events, widgets, and the six CVar additions actually named by the crawler excerpt. The unresolved claims for fourteen unnamed additions and five unnamed removals remain explicit metadata rather than invented symbols.

## [2026-08-06] system | Generic changed-occurrence patch manifests

Extended the patch-audit manifest schema to preserve `changed:symbol` occurrences alongside added and removed rows. Source JSON may include a `changed` array; manifest metadata records `changed_count`, defaulting to zero for the existing 12.1 added/removed-only register. Focused tests cover direction IDs, count mismatches, source ordering, and backward compatibility.

## [2026-08-06] investigation | 12.1 broader fidelity re-triage

Reconciled the completed 432-row FrameXML register with eight broader 12.1 fidelity families that are outside that manifest. Aura containers, DurationTextBinding, and service-backed structures retain explicit best-effort contracts. UnitAura secrecy, private/forbidden security enforcement, standalone RadialProgress construction, and strict-removal timing are individually identified as unapproved unsafe/impossible exception candidates. Removed contradictory wording that described the same candidates as both approved and pending; no approval was requested or recorded.

## [2026-07-16] update | FrameStrata before/after observations

Updated FrameStrata documentation from `/tmp/FrameStrataProbe-parent-retail.lua` (retail 12.0.7 build 68453, captured `2026-07-16T01:21:08`). During XML `OnLoad`, the actual parent and direct `PARENT` child both reported `DIALOG`, while the literal sibling reported `LOW`. Under an actual `DIALOG` parent, base `HIGH` reported `HIGH`, derived literal `LOW` reported `LOW`, and derived `PARENT` reported `HIGH`. After the tested parent-strata and reparent operations, every tested non-fixed child and grandchild reported `LOW`, including explicit XML `MEDIUM` fixtures. Documentation avoids claims about the client's internal resolution or propagation mechanism; the capture did not test `BLIZZARD`.

## [2026-07-14] investigation | 12.0.7 widget compatibility matrix

Focused 12.0.7 proof verifies six retained Minimap texture setters, four Button methods, four ScrollFrame methods, and five font-bearing `SetFont` methods. `ModelSceneActorBase:GetModelUnitGUID` is absent under the intentional permanent no-3D scope and remains an explicit exception candidate; no approval requested yet.

## [2026-07-14] investigation | 12.0.7 removal and event matrix

Added exact 12.0.7 startup proof for all 17 proposed global removals and 17 added/changed events. Eleven removed names are nil, six remain compatibility functions, and every event registers. This corrects the earlier blanket claim that all removed wrappers remained available.

## [2026-07-14] investigation | 12.1 final publication matrix

Closed the final 106 FrameXML rows with an exact all-LoD publication matrix: seven proposed additions remain nil and 99 proposed removals remain functions. Every mismatch reports symbol, expected type, and observed type. Full target: 15 tests passed in 22.35 seconds (26.048 seconds wall time). Final 12.1 FrameXML inventory: 1 implemented, 431 best-effort, 0 exception-requested, and 0 untriaged rows.

## [2026-07-14] investigation | 12.1 conservative source-absence batch

Classified 175 proposed additions individually as stale snapshot entries. Selection requires the bare method/global token to be absent from every PTR Lua/XML/TOC file; the focused generated test then loads the complete game-compatible addon closure, including LoD roots, and reports any exact global/namespace publication. Stronger source patterns cover dot, colon, bracket, and `rawset` forms for earlier namespace families. Some all-LoD addons emit recorded Lua errors, so this remains explicitly best-effort source-plus-runtime evidence rather than an exact fidelity claim. Full target: 14 tests passed in 21.82 seconds (24.836 seconds wall time). Current inventory: 1 implemented, 325 best-effort, 0 exception-requested, and 106 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 utility namespace and colon-publication audit

Corrected source scanning to cover both `Namespace.Method` and `Namespace:Method` Lua publications. Earlier stale families remain absent under the stronger falsifier. Classified 29 utility additions as stale and `PingUtil.GetContextualPingTypeForUnit` as vendor-present with tested `C_Ping` forwarding. Current full target: 13 tests passed in 15.20 seconds (18.309 seconds wall time). Current inventory: 1 implemented, 150 best-effort, 0 exception-requested, and 281 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 GuildControl snapshot mismatch

Classified ten proposed `GuildControlUI_*` additions as stale snapshot globals. Shared-corpus PTR source proof finds no occurrences and startup runtime keeps all ten nil. The first post-build target took 64.43 seconds wall time; the unchanged warm complete target passed 11 tests in 15.65 seconds (17.272 seconds wall time), satisfying the 60-second gate. Current inventory: 1 implemented, 120 best-effort, 0 exception-requested, and 311 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 Narration snapshot mismatch

Classified all 14 proposed `NarrationUtil` additions as stale qualified names. Shared-corpus PTR source proof and startup runtime enumeration keep the namespace nil. Full grouped audit target: 10 tests passed in 13.82 seconds (20.495 seconds wall time). Current inventory: 1 implemented, 110 best-effort, 0 exception-requested, and 321 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 SocialUI snapshot mismatch and source-scan budget

Classified all 13 proposed `SocialUIUtil` additions as stale qualified names. Exact-qualified PTR source proof and runtime enumeration keep the namespace nil. Adding a third recursive source scan first pushed complete-target wall time to 76.304 seconds; a shared `OnceLock` source corpus reduced the verified nine-test target to 14.49 seconds test time and 19.023 seconds wall time. Current inventory: 1 implemented, 96 best-effort, 0 exception-requested, and 335 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 friends-list snapshot mismatch

Classified all 29 proposed `FriendsListUtil` additions as stale qualified names. The falsifier first caught similarly named `FriendsFrame_*` globals; corrected exact-qualified PTR source proof shows no `FriendsListUtil.*` publications, and runtime proof confirms the namespace remains nil. Full grouped audit target: 8 tests passed in 10.32 seconds (12.541 seconds wall time). Current inventory: 1 implemented, 83 best-effort, 0 exception-requested, and 348 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 combat audio snapshot mismatch

Classified ten proposed `CombatAudioAlertUtil` interrupt/start/end/death additions as stale snapshot entries. Recursive PTR source proof scans Lua/XML/TOC files; runtime proof verifies the active namespace and representative real method exist while all ten proposed names remain nil. Full grouped audit target: 7 tests passed in 13.94 seconds (22.816 seconds wall time). Current inventory: 1 implemented, 54 best-effort, 0 exception-requested, and 377 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 retained UI geometry globals

Classified the proposed removals of `UIDoFramesIntersect`, `GetNotchHeight`, and `GetUIParentOffset` as vendor-present. Focused PTR proof covers overlap/separation/edge-touch behavior, physical-to-UI notch normalization, and maximum debug-bar/notch offset selection. Full grouped audit target: 6 tests passed in 13.438 seconds. Current inventory: 1 implemented, 44 best-effort, 0 exception-requested, and 387 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 input utility snapshot reversal

Classified five proposed `InputUtil` additions as stale snapshot namespace moves and four proposed global removals as vendor-present. Focused PTR proof verifies the namespace members remain nil while the legacy globals perform cursor scaling, frame-scale forwarding, mouse-offset forwarding, and inspect-cursor selection. Full grouped audit target: 5 tests passed in 9.232 seconds. Current inventory: 1 implemented, 41 best-effort, 0 exception-requested, and 390 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 screen-scale snapshot reversal

Classified the proposed `InterfaceUtil.GetScreenHeightScale` and `InterfaceUtil.GetScreenWidthScale` additions as stale snapshot entries and the proposed global removals as vendor-present. Focused PTR proof verifies `InterfaceUtil` is absent, both globals remain functions, and a 1024×768 fixture returns `1.0` for each. Full grouped audit target: 4 tests passed in 10.873 seconds. Current inventory: 1 implemented, 32 best-effort, 0 exception-requested, and 399 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 PTRFeedback quest-progress helper

Classified `GetTimeSinceLastQuestProgress` as vendor-present best-effort behavior. Focused PTR proof verifies publication by PTRFeedback and pins the current upstream nil-arithmetic invocation defect caused by undefined `lastProgressTime`; simulator adds no guessed correction. Full grouped audit target: 3 tests passed in 6.625 seconds. Current inventory: 1 implemented, 28 best-effort, 0 exception-requested, and 403 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 shake namespace mismatch

Classified proposed legacy `ShakeFrame` and `ShakeFrameRandom` additions as stale snapshot entries. Focused PTR proof verifies both globals remain nil while the distinct `ScriptAnimationUtil` methods exist and return cancellation functions for safe no-op conditions. Full grouped audit target: 2 tests passed in 5.774 seconds. Current inventory: 1 implemented, 27 best-effort, 0 exception-requested, and 404 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 PlayerChoice toggle LoD lifecycle

Classified both `PlayerChoiceToggle_TryShow` snapshot occurrences as best-effort load-on-demand vendor behavior. Focused PTR proof verifies absence before `Blizzard_PlayerChoice`, publication after explicit load, eligible-button visibility, explicit plus OnShow state updates, and nil return. Current inventory: 1 implemented, 25 best-effort, 0 exception-requested, and 406 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 smooth-progress snapshot reversal

Classified the proposed `InterpolatorUtil.GetSmoothProgressChange` addition as a reversed snapshot and the proposed global `GetSmoothProgressChange` removal as vendor-present. Focused PTR proof verifies the namespace member remains nil, the global remains a function, and representative input returns `70`. Current inventory: 1 implemented, 23 best-effort, 0 exception-requested, and 408 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 Macro save lifecycle

Classified both `MacroFrame_SaveMacro` snapshot occurrences as best-effort vendor-present behavior. Focused PTR proof verifies the eager UIParent no-op placeholder is harmless and explicit `Blizzard_MacroUI` loading replaces it with the `MacroFrame:SaveMacro()` delegate. Current inventory: 1 implemented, 21 best-effort, 0 exception-requested, and 410 neutral untriaged rows.

## [2026-07-14] system | Active-TOC patch source reachability

Added `--active-tocs` to the full-tree Lua publication index. The active compiled profile uses `find_toc_file` to select each addon's TOC, applies per-file environment rules, recursively follows XML script/include paths through the loader's addon-root fallback and case-insensitive resolver, records unresolved Lua/XML reference paths, and excludes source files selected only by other flavor TOCs. This corrects false candidate matches such as Mists-only helpers found by the raw all-files scan. Source selection remains candidate evidence; dependency order and LoD timing still require lifecycle tests.

## [2026-07-14] system | Full-tree patch source candidates

Added deterministic `--index-lua-tree` scanning with relative paths, per-file hashes, and first-directory addon ownership. Pre-lexer candidate counts were discarded after review exposed comment/string/local-scope false positives; corrected counts must come from a fresh scan. Results remain candidate-only. The later active-TOC entry applies active-profile source reachability; dependency order and LoD timing remain unapplied.

## [2026-07-14] system | Patch source candidates and initialization observations

Added `--observe-initialization`, which writes actual active-profile Lua observations and rejects manifest/profile mismatches. Added `--index-lua-source` for file/line direct-publication candidates plus explicit mixin/metatable/dynamic-global/factory ambiguity records. Candidate source evidence never changes final statuses automatically. Full manifest-driven post-load/LoD/reset orchestration remains open.

## [2026-07-14] system | Patch observation primitive

Added a production observation primitive that resolves actual Lua global/table paths in `WowLuaEnv` and records active profile, presence, and Lua type while carrying caller-supplied phase/addon labels. Focused coverage observes present and absent symbols, a real identity-matched TOC load transition, and exact manifest-byte hashing. A concrete post-reset runtime operation and full manifest-driven phase orchestration remain open.

## [2026-07-14] investigation | Preserve 12.0.7 API-change source

Moved the 12.0.7 Warcraft Wiki source snapshot from temporary storage into `data/patch-api/sources/12.0.7-api-changes.txt` and linked the patch audit to the checked-in evidence. The 12.0.7 manifest/register remains open work.

## [2026-07-14] system | Patch API audit manifest

Created `systems/patch-api-audit-manifest.md` and the 432-row `data/patch-api/12.1-framexml.json` register. Reviewer correction removed false blanket exception requests: 412 pending rows now have null status and neutral `untriaged` resolution. Repository validation recomputes source/evidence hashes, verifies tests and commit ancestry, and rejects checklist/inventory drift. Completion requires an exact-manifest observation artifact and per-item unsafe/impossible approval provenance; real per-row observation generation remains open.

## [2026-07-14] investigation | 12.1 CustomerOrders hide-wrapper mismatch

Updated the 12.1 FrameXML inventory after a recursive PTR source scan found no `HideProfessionsCustomerOrdersFrame` definition. A focused PTR test loads ProfessionsTemplates and AuctionHouse dependencies, explicitly loads `Blizzard_ProfessionsCustomerOrders`, verifies its frame exists, and confirms the snapshot-only wrapper remains nil. Current inventory: 1 implemented, 19 best-effort, 0 exception-requested, and 412 neutral untriaged rows.

## [2026-07-14] investigation | 12.1 Garrison hide-wrapper mismatches

Updated the 12.1 FrameXML inventory after confirming `HideGarrisonMissionFrames` and `HideGarrisonShipyardFrame` have no definitions in local PTR Blizzard sources. A focused PTR test loads `Blizzard_GarrisonUI` with its LoD dependencies and verifies both snapshot-only wrappers remain nil. At that intermediate stage, 1 row was implemented, 18 were best-effort, and 413 unresolved rows were still mislabeled as exception requests; the latest manifest entry corrects them to neutral untriaged state.

## [2026-07-14] investigation | 12.1 BlackMarket hide-name mismatch

Updated the 12.1 FrameXML inventory after confirming PTR defines `BlackMarketFrame_Hide` with `HideUIPanel(BlackMarketFrame)` plus close-sound behavior, while snapshot entry `HideBlackMarketFrame` is absent. A focused PTR test explicitly loads `Blizzard_BlackMarketUI`, verifies the authoritative helper, and confirms the reversed-name wrapper remains nil. At that intermediate stage, 1 row was implemented, 16 were best-effort, and 415 unresolved rows were still mislabeled as exception requests; the latest manifest entry corrects them to neutral untriaged state.

## [2026-07-14] investigation | 12.1 ItemUpgrade hide-name mismatch

Updated the 12.1 FrameXML inventory after confirming PTR defines `ItemUpgradeFrame_Hide` as the authoritative `HideUIPanel(ItemUpgradeFrame)` helper, while snapshot entry `HideItemUpgradeFrame` is absent. A focused PTR test explicitly loads `Blizzard_ItemUpgradeUI` and verifies the reversed-name wrapper remains nil. At that intermediate stage, 1 row was implemented, 15 were best-effort, and 416 unresolved rows were still mislabeled as exception requests; the latest manifest entry corrects them to neutral untriaged state.

## [2026-07-14] investigation | 12.1 GuildBank hide wrapper mismatch

Updated the 12.1 FrameXML inventory after confirming `HideGuildBankFrame` has no definition in local PTR Blizzard sources. A focused PTR test explicitly loads `Blizzard_GuildBankUI` and verifies the snapshot-only wrapper remains absent. At that intermediate stage, 1 row was implemented, 14 were best-effort, and 417 unresolved rows were still mislabeled as exception requests; the latest manifest entry corrects them to neutral untriaged state.

## [2026-07-14] investigation | 12.1 AuctionHouse hide wrapper mismatch

Updated the 12.1 FrameXML inventory after confirming `HideAuctionHouseFrame` has no definition in the local PTR Blizzard sources. A focused PTR runtime test loads `Blizzard_AuctionHouseUI` and verifies the snapshot-only wrapper remains absent instead of adding guessed close semantics. At that intermediate stage, 1 row was implemented, 13 were best-effort, and 418 unresolved rows were still mislabeled as exception requests; the latest manifest entry corrects them to neutral untriaged state.

## [2026-07-14] investigation | 12.1 Mists-only time helper excluded

Updated the 12.1 FrameXML inventory after proving `GetTimeStringFromSeconds` is defined only by `Mists/UIParent.lua` and excluded from the PTR mainline TOC. It is classified best-effort as cross-flavor snapshot contamination rather than implemented behavior. PTR tests verify absence during environment initialization, after Blizzard loading/post-load compatibility, and after startup events. At that intermediate stage, 1 row was implemented, 12 were best-effort, and 419 unresolved rows were still mislabeled as exception requests; the latest manifest entry corrects them to neutral untriaged state.

## [2026-07-14] update | 12.1 DifficultyUtil delegates modeled

Updated the 12.1 audit and FrameXML inventory after adding five epoch-scoped, post-load `DifficultyUtil` color delegates. The delegates dynamically call the authoritative vendor globals, preserving arguments, both return values, later hotfix replacement, and explicit missing-global errors. Focused tests cover namespace reset/preservation and dynamic dispatch; full PTR Game UI startup verifies vendor threshold behavior, while older-retail startup verifies non-exposure. Inventory now records 1 implemented, 11 best-effort, and 420 pending strict exception re-triage.

## [2026-07-14] decision | Patch API audit exception approval superseded

A broad approval for documented 12.1, 12.0.7, and 12.0.5 exceptions was recorded, then superseded after review found that the full itemized checklist was not presented in chat and 12.1 FrameXML entries were mass-deferred without individual unsafe/impossible justification. Audits remain open pending re-triage and informed per-item approval.

## [2026-07-13] create | Mastery spells modeled, alias identity fix

Created `systems/specialization-mastery-spells.md` and
`investigations/deprecated-specialization-alias-identity.md` after modeling
`C_SpecializationInfo.GetSpecializationMasterySpells` from ChrSpecialization.db2
(retiring the empty-table temporary shim) and fixing three pre-existing test
failures: deprecated alias identity broken by post-cleanup re-registration, and
legacy specialization globals / UIWidgetContainerMixin duplicated in
`c_api/c_spec.rs` after their move to the Lua globals layer.

## [2026-07-13] update | Retail-only CASC isolation test

Updated `systems/casc-asset-cache.md` after adding
`scripts/test-retail-casc-isolation.py`. Documented Bubblewrap masking of all
non-retail WoW flavor directories, isolated writable caches, preserved failure
logs, exact missing-entry reporting, and the verified workflow for adding or
removing retail manifest entries.

## [2026-07-13] update | Docker headless release build

Updated `reference/addon-compatibility.md` and `systems/rendering-pipeline.md` after
Docker CI for v0.1.29 failed because the headless release build omitted the
required `client-retail` profile and `frame_collect` depended on the GUI-only
`hit_grid` module. Recorded the fixed build contract
(`--no-default-features --features client-retail`) and the shared `HitOrderKey`
ownership split; the next tag will carry the fix.

## [2026-07-13] update | Retail Blizzard UI manifest curation

Updated `systems/client-profiles.md` and `systems/addon-loading.md` after
`data/blizzard-ui-files/retail.txt` was independently curated to 3,591 entries,
excluding 390 legacy-profile entries under `/Classic/`, `/Mists/`, `/Wrath/`,
`/Cata/`, `/TBC/`, and legacy TOCs. Recorded that retail runtime uses the
manifest contents directly, without an additional legacy-profile filter, while
profile-aware TOC and game-type selection remains part of addon discovery.

## [2026-07-09] investigation | Patch 12.0.5 API audit

Created `investigations/patch-12-0-5-api-audit.md` to consolidate the probe-driven 12.0.5 work. Recorded that retail `12.0.5.67823` findings for forbidden frames, invalid unit-event filters, wildcard false attributes, Raise/Lower ordering, frame identity slot `[0]`, XML frame-level semantics, and display/scale event pairs are already modeled with focused tests. Documented that no `patch_12_0_5_inert_defaults` module exists and no obvious safe, already-backed 12.0.5 inert default remains unconverted. Updated with all 13 retained SavedVariables probe families, explicit best-effort boundaries, and exception requests for missing exact regressions, Store lifecycle evidence, XmlFrameLevel raw provenance, same-size window transitions, and the absent patch API-diff source.

## [2026-07-06] investigation | Patch 12.0.7 API audit

Created `investigations/patch-12-0-7-api-audit.md` after bridging compatible 12.0.7 API gaps and pausing exact-behavior work. Recorded additive/inert API bridges, verification logs, and blocked areas requiring live behavior: restricted unit-token returns, `ENCOUNTER_END` payloads, EncounterEvents color state, SimulateMouse taint/focus restrictions, debug secret propagation, secure raidtarget actions, M+ CalendarTime returns, aura security changes, widget secret aspects, and deprecated/removal timing. Updated after `C_BattleNet.InviteFriend` moved from inert bridge to modeled `SimState.bnet_friends` mutation. Updated again after ready-check behavior moved from inert `C_PartyInfo` bridge to modeled state: `DoReadyCheck`/`ReadyCheck`, `ConfirmReadyCheck`, `GetReadyCheckStatus`, `GetReadyCheckTimeLeft`, and immediate ready-check event dispatch. Updated after `C_UIFileAsset` moved from inert Lua defaults to best-effort limited-listfile lookup, after timeline event colors started mirroring the existing `C_EncounterEvents` color state, after `DurationTextBinding` gained documented non-secret state methods plus best-effort duration-object storage, after `GameTooltip_AddMoneyLine` started formatting money through `GetMoneyString` instead of appending raw copper, after `C_PartyInfo.IsGUIDInGroup` moved to the simulator party roster model, after C_PartyInfo leader/assistant mutators started updating simulator group-role state, after `C_PingSecure.ClearPendingPingOffScreenCallback` moved to the Rust shared callback table, after 12.0.7 CPU usage globals moved to the shared performance-metric defaults module, after `C_DurationUtil.CreateManualClock` moved to the Rust `C_DurationUtil` surface, after Delves/Housing/MerchantFrame/QuestHub trivial namespace defaults plus `C_PartyInfo.UninviteUnit` moved from the 12.0.7 Lua patch shim to Rust-backed best-effort/model-backed surfaces, after `C_EncounterTimeline.GetEventColor` moved to the Rust encounter-events surface while `GameTooltip_AddMoneyLine` moved to the shared formatting defaults, after secure pending button/ping/toggle callback globals moved to Rust-backed shared PingSecure callback storage, and after the recovered exact 12.0.7 CVar delta (17 adds, five removals, one default change) moved into profile-gated defaults with an explicit source-integrity exception for three unnamed claimed additions, and after `LuaDurationObject` gained best-effort clock storage plus deterministic `HasExpired`/`HasStarted`/`IsActive` methods for the documented 12.0.7 duration-object surface, and after the DurationText no-argument regression established that the stale `C_DurationUtil.CreateDurationTextFormattingOptions` / `CreateDurationTextRawValue` extraction names are universal-fallback nil functions rather than documented factories.

## [2026-07-06] investigation | Patch 12.1 API audit

Created `investigations/patch-12-1-api-audit.md` after bridging compatible 12.1 API gaps and pausing exact-behavior work. Recorded committed bridge points, verification logs, and the blocked areas that require live PTR behavior: UnitAura secrecy, Private Script Objects/Forbidden Partition, full ForbiddenAspect enforcement, AuraContainer/AuraButton/ManagedAuraContainer, DurationTextBinding/RadialProgress script objects, and exact structure payloads. Updated in the second pass to add an explicit implementation matrix and record 12.1 `DurationTextBinding` color-curve compatibility methods while leaving standalone `RadialProgress` paused. Updated again after Battle.net title-friend custom names/tags moved from inert Lua defaults to a best-effort `SimState.bnet_friends` model, after Encounter Journal difficulty helpers moved to generated-instance-data guesses, after `C_Discord.IsEnabled` started reflecting `discordClientEnabled`, after pending Battle.net friend invites gained a best-effort state model, after Battle.net feature probes started returning true for modeled friend-list/title-friend/tag support, after `C_Housing` owned-house/plot probes plus `ResetHouse` moved to local `SimState.housing` state pending replacement with probe-backed service semantics, and after safe `C_HousingBlueprint` share-code/import/export calls moved to local blueprint intent state pending exact PTR/service payload probes, and after housing editor/customize/decor/layout probes moved to local `SimState.housing` state with remaining blueprint availability calls itemized as pending exception requests until PTR/service payloads are known. Updated after `SetAppearOffline` moved to `SimState.bnet_appear_offline` and `BNCheckTitleFriendInviteToUnit` moved out of Lua inert defaults as a deterministic false best-effort probe pending title-friend service data. Updated again after Discord OAuth/link/settings/server/channel probes moved to local `SimState.discord` state and the final 12.1 Lua inert defaults were removed. Updated after housing blueprint availability probes moved from nil placeholders to local `SimState.housing` result codes pending exact service enum probes. Added explicit 12.1 exception requests for security-sensitive aura/private/forbidden/aspect behavior, standalone RadialProgress fidelity, full DurationTextBinding fidelity, exact service payloads, and strict-removal timing. Best-effort 12.1 bridges are explicitly temporary: keep them only while they are backed by existing simulator state and documented tests, then replace them with PTR/service probe-backed semantics once exact behavior is known. Updated after `LoadAddOnWithErrorHandling` was added as a tested canonical wrapper around `UIParentLoadAddOn`, and after the local 12.1 FrameXML snapshot was expanded into an exhaustive 320-added/112-removed inventory: the wrapper is implemented; the remaining 431 entries are explicit exception requests pending ownership/lifecycle evidence. Two names occur in both source lists, so the 432 entries represent 430 distinct names.

## [2026-07-06] update | Retail API epoch features

Updated `systems/client-profiles.md` after introducing cumulative retail API epoch features (`retail-12-0-7`, `retail-12-1-0`) alongside mutually-exclusive `client-*` profile features. Recorded that API surface gates belong on epoch features while PTR cache, CASC product, install paths, and vendor manifest behavior remain `client-ptr` profile concerns.

## [2026-07-02] update | XML method binding timing

Updated `systems/xml-template-system.md` after live PTR probing and simulator
regressions clarified XML `method="..."` behavior: XML binding installs the
currently composed method function as the script handler, object fields and
`GetScript` storage diverge after `frame.X = ...` or `SetScript`, and private
methods under `useForbiddenObjectTable` resolve from the forbidden object table
for AuraContainer-style XML.

## [2026-07-02] update | 12.1 XML partitioned mixins

Updated `systems/xml-template-system.md` after implementing PTR 12.1
`ScopedModifier useForbiddenObjectTable` and partitioned mixin semantics for
AuraContainer-style XML. Recorded public vs forbidden frame partitions,
partition-aware KeyValues, `targetPartition`, `inboundPartition` self
substitution, and `secureDelegates` public delegate behavior.

## [2026-07-02] update | Blizzard UI CDN missing chunks

Updated `systems/casc-asset-cache.md`, `systems/addon-loading.md`, and `PLAN.md`
after wiring Blizzard UI sync to fetch missing authoritative CASC chunks from
Blizzard CDN by encoding key via public `Osso/casc-extract` when local streaming
install archives are incomplete. Recorded that repo/source mirrors remain
disabled and CDN archive indexes persist under `~/.cache/casc-extract/`.

## [2026-07-01] update | Per-profile Blizzard UI manifests

Updated `systems/addon-loading.md`, `systems/casc-asset-cache.md`, and `index.md`
after splitting the Blizzard UI cache manifest into profile-specific files under
`data/blizzard-ui-files/`. Recorded that the active `client-*` profile selects
both the manifest and CASC product (`wow` vs `wowt`) so PTR-only addon files no
longer have to be shoehorned into the retail manifest. Documented that Blizzard
UI cache population has no repo-source fallback; tracked listfile overrides are
used only to teach CASC path→FDID mappings until upstream listfiles catch up.

## [2026-07-01] update | Bootstrap TOC semantics

Updated `systems/addon-loading.md` after correcting `[Bootstrap]` semantics:
annotated entries stay in normal TOC order, no standalone bootstrap pass runs,
and LoadOnDemand addons execute those files only during normal `C_AddOns.LoadAddOn`.
Updated `index.md` summary wording.

## [2026-06-29] investigation | Retail/PTR full startup Lua errors

Created `investigations/retail-ptr-full-startup-lua-errors.md` after full GUI
startup logs exposed handler-time errors missed by `lua-errors`. Recorded the
PVPUI API gaps (`C_WeeklyRewards`, `C_PvP`, legacy PVP role globals,
`ClearBattlemaster`), PTR cursor gap, Store inbound nil `StoreFrame` fallback
issue, and verification with clean retail/PTR startup logs. Updated `index.md`.

## [2026-06-29] update | Mists 5.5.4 lua-errors cleanup

Updated `investigations/mists-world-map-startup.md` after reducing Mists
5.5.4 startup `lua-errors` to `[]`. Recorded the root causes: escaped
`Interface/AddOns` XML paths, missing profile-cache manifest files
(`Blizzard_UIParent/Classic/*`, `Blizzard_SharedXML/Classic/GameTooltipTemplate.lua`),
`QuestUtil` being reset by `Blizzard_FrameXMLUtil/Classic/QuestUtils.lua`,
Mists XML referencing unshipped `WorldStateProvingGrounds_*` helpers, missing
native EditMode frame methods, and missing `UNIT_LEVEL_NON_ATTACKABLE` color.
Updated `index.md`.

## [2026-06-19] update | Bag button OnLoad load-order resolved

Marked `investigations/addon-load-order.md` RESOLVED. The historical
`PaperDollItemSlotButton_OnLoad`-before-definition failure no longer reproduces:
OnLoad completes (verified via `CharacterBag0Slot` event registration and clean
`lua-errors`), and the replay workaround was removed (`70fca4e25`, `d4f1287f9`).
Fixed dead `workarounds_bags.rs` / `l.rs` path references in both the wiki page
and the legacy `docs/addon-load-order-investigation.md`. Documented the root
cause: transitive `LoadFirst` — `Blizzard_EnvironmentCleanup` (`LoadFirst: 1`)
depends on `Blizzard_UIPanels_Game`, and the eager two-pass loader emits a
LoadFirst addon's deps first, so the definer loads before the bag buttons.

## [2026-06-19] update | PTR client profile

Updated `systems/client-profiles.md` after adding the `client-ptr` profile for
12.1 PTR. Documented the new `ptr` Blizzard UI cache scope, `120100` interface
version, mainline TOC/game-type behavior, and fallback preference for the
cache-managed `Gethe/wow-ui-source@ptr` archive before live/beta.
Updated again after the first PTR cache sync investigation: PTR sync now selects
the `wowt` CASC product automatically and filters legacy-profile manifest entries
before cache-completeness checks.

## [2026-06-19] update | Blizzard UI profile cache migration

Updated `systems/addon-loading.md` after moving runtime Blizzard UI loading to
profile-scoped user-cache roots under
`~/.cache/wow-ui-sim/blizzard-ui/<profile>/AddOns`. Documented
`wow-cli casc sync-blizzard-ui` as the canonical cache population path, the
completion/provenance marker pair, and the setup-script compatibility wrappers.

## [2026-06-12] investigation | Lib test failure sweep

Created `investigations/lib-test-failure-sweep-2026-06.md` after root-causing
nine accumulated `cargo test --lib` failures: runtime foundation order
(Blizzard_ScriptErrors), the silently-refused
`hooksecurefunc(C_AddOns, "LoadAddOn")` target (two workarounds rewired through
`apply_blizzard_post_load_patches`; one dead hook remains in
`runtime_surface_bootstrap.lua`), two more classic-rebase code losses, drifting
duplicated Lua installers, and tests stale against deliberate semantic changes.
Updated `index.md`.

## [2026-06-10] investigation | Retail core behavior probes

Created `investigations/retail-core-behavior-probes.md` after adding and
installing `CoreBehaviorProbe`. Recorded the retail `12.0.5.67823`
observations for `SetForbidden`, `CreateForbiddenFrame`, invalid
`RegisterUnitEvent`, wildcard false attributes, and the improved Raise/Lower
probe that needs a fresh live-client capture before simulator behavior changes.
Updated it after the fresh reload capture: `Raise()` and `Lower()` succeeded,
but simple shown sibling frames kept `GetFrameLevel()` at `1`/`10` and
`GetRaisedFrameLevel()` at `0` before and after under both a private parent and
`UIParent`.
Updated it again after fixing wow-ui-sim so `GetRaisedFrameLevel()` returns the
retail-observed `0` for the simple sibling probe while keeping internal
`raise_order` as render/hit-order bookkeeping.
Updated it again after adding explicit regression coverage for wildcard
`GetAttribute(prefix, name, suffix)` preserving a stored false value.
Updated it again after the hit-order capture showed the higher raw frame level
kept mouse focus before and after `Raise()`/`Lower()`; wow-ui-sim now treats
`raise_order` as a same-level tie-breaker instead of adding it to
`frame_level`.

## [2026-06-10] update | Frame identity token userdata

Updated `investigations/frame-surrogate-identity-slot.md` after switching `frame[0]` from a tiny backed table to a `FrameIdentity` userdata token. Recorded that DevTools dumping also needs raw frame iteration plus `dumpobject` returning nil so `[0]` renders as opaque userdata.
Updated it again after making `extract_frame_id` dispatch-aware and adding `native_frame_id_from_val` for the rare cases that need the original table backing.
Updated it again after real-client and wowless duplicate-name probes showed replacement frames get fresh identity and do not migrate custom Lua fields; recorded the simulator fix that removed old-global field copying during `CreateFrame` registration.
Updated `systems/addon-loading.md` after fixing third-party addon enable-state merging and startup keybinding import. Recorded that local `AddOns.txt` overlays real WTF state, required-dependency disables apply to default-enabled dependents, and `bindings-cache.wtf` imports before addon loading.
Updated it again after extending the existing `ServerSnapshot` addon to capture AddOn List enable state and keybindings. Recorded that `ServerSnapshotDB` is the preferred live-client overlay for addon state because `AddOns.txt` alone is not reliable for the per-character UI state.

## [2026-06-08] update | Mists 5.5.4 EditMode

Updated `investigations/editmode-layout.md` after bumping the Mists Blizzard UI
source pin to 5.5.4. Startup was clean with the existing `C_EditMode` fallback,
but a real frame tick exposed missing `MIRRORTIMER_NUMTIMERS` in
`WorldFrame_OnUpdate`; the mirror-timer Rust surface now publishes the constant.
The update also records that Mists now loads `Blizzard_UIParentPanelManager`
through its `_Classic.toc`, so the simulator no longer excludes that addon and
the cache manifest now carries the Classic panel-manager files.

## [2026-06-08] investigation | PlayerSpells runtime load

Created `investigations/playerspells-runtime-load.md` after fixing the retail `TOGGLETALENTS` keybind path. The note records the `C_AddOns.LoadAddOn` call-frame preservation issue plus the temporary PlayerSpells ModelScene/PvP talent backfills needed for `Blizzard_PlayerSpells` demand-load.

## [2026-06-08] investigation | ModelScene player actor stub

Created `investigations/modelscene-player-actor-stub.md` after Collectionator's transmog recovery helper crashed while calling `GetPlayerActor():SetModelByUnit("player")`. Documented the compatibility boundary: 3D rendering remains intentionally stubbed, but ModelScene actor object methods must exist for addon probes.

## [2026-06-08] update | FontString default anchors

Updated `investigations/fontstring-default-anchors.md` after follow-up retail JustifyProbe data showed XML `ButtonText` uses the same `justifyH` implicit anchor as layer `FontString`, explicit vertical-only anchors suppress the default, and EditBox backing FontStrings remain unanchored even with XML `TextInsets`.

## [2026-06-08] investigation | FontString default anchors

Created `investigations/fontstring-default-anchors.md` after real-client testing showed unanchored XML `FontString` layer children use `justifyH` for their implicit anchor point. Updated `index.md` with the new investigation page.

## [2026-05-28] investigation | action button icon mask coverage

Created `investigations/action-button-icon-mask.md` after tracing vanished main
action-bar icons to mask sampling, not action state. The prior minimap mask fix
made the shader sample RGB mask intensity; action-bar icon masks store coverage
in alpha, so their black visible regions went transparent. The renderer now
marks alpha-backed masks with a shader flag while retaining RGB coverage for
opaque black/white masks.

## [2026-05-26] update | EditMode cache with no saved vars

Updated `investigations/editmode-layout.md` with the `--no-saved-vars` status
tracking regression. EditMode profile cache files are not Lua SavedVariables, so
startup now loads them through a separate WTF cache path when Lua SavedVariables
are disabled.

## [2026-05-18] investigation | ElvUI tooltip skin ordering

Updated `investigations/tooltip-double-shell.md` after tracing Character panel item tooltips under ElvUI to direct `GameTooltip` skin textures rendering after the tooltip frame's internal text emitter. `GameTooltip` now renders direct texture regions before the tooltip frame/text while still deferring direct FontStrings above it.

## [2026-05-18] investigation | Mists full-addon login profile

Updated `investigations/talent-performance.md` with a fresh full-addon Mists login profile. Startup is currently dominated by third-party Lua compilation (`16.73s` compile out of `25.64s` third-party addon load, bytecode cache `1/1395` hits), with AllTheThings and RaiderIO DB addons as the largest contributors.

## [2026-05-17] investigation | Mists ElvUI startup compatibility

Created `investigations/mists-elvui-startup-compat.md` after the full-addon Mists probe isolated ElvUI startup failures to trim aliases, overexposed MessageFrame methods on plain frames, Mists AuraUtil tuple shape, and unanchored Slider label fontstrings.
Updated it after tracing ElvUI install text displacement and raid-control centering to fixed physical screen-size globals under `UIParent` scale; runtime screen-size changes now also dispatch display/scale events.
Updated it again after fixing ElvUI Chat initialization: Mists now provides `RedockChatWindows`, and the shared runtime surface provides `GetPlayerInfoByGUID`.
Updated it again after fixing ElvUI Tooltip font initialization: Font objects now share an object-type metatable, so ElvUI `FontTemplate` additions made through `GameFontNormal` are visible on `GameTooltipText`.
Updated it again after adding `GetInventoryItemDurability`; ElvUI DataTexts now sees the expected inventory durability global and the full-addon probe no longer reports that startup error.
Updated it again after tracing the ElvUI static-popup `OnUpdate` error to simulator-driven layout dirtying. The size/layout dirty path now snapshots and restores existing custom `OnUpdate` handlers across recursive layout-parent `MarkDirty()` calls, so `ElvUI_StaticPopup1` keeps ElvUI's handler after `E:StaticPopup_Show`.
Updated it again after tracing the ElvUI Installation close-button click failure to unscaled `SetHitRectInsets` in hit-grid construction. Hit rectangles now scale insets by the frame's effective scale before subtracting them from scaled layout rects.
Updated it again after finding the remaining ElvUI installer and raid-control placement issue: startup screen globals reported 1024x768 while `SimState`/`UIParent` still defaulted to 1600x1200, so ElvUI anchored a correctly sized parent inside the wrong root canvas.

## [2026-05-17] update | SetAtlas empty clear semantics

Updated `systems/texture-atlas.md` after tracing missing Character panel paper-doll elements to `SetAtlas("")` resolving the generated empty-name atlas entry (`Interface\castingbar\uicastingbarstandardflipbook`). Empty atlas names now clear texture/atlas state and clear propagated parent button slots, which restores ElvUI-stripped equipment slot rendering.

## [2026-05-17] update | frame field environment numeric slot

Updated `systems/frame-data-flow.md` after tracing the ElvUI/oUF aura `button:SetSize` startup error to frame field storage occupying raw numeric key `1`. Frame refs now keep addon array slots free while `debug.getfenv(frame)[1]` remains a compatibility view onto normal frame fields.

## [2026-05-17] update | Mists talent first-open latency

Updated `investigations/talent-performance.md` after tracing full-addon Mists talent first-open latency to a deferred AceAddon enable queue. BlizzMove's skipped `PLAYER_LOGIN` left 27 queued Ace addons until `ADDON_LOADED("Blizzard_TalentUI")`; allowing BlizzMove to receive login again makes the talent load itself sub-second, with remaining ElvUI login cost tracked separately.

## [2026-05-17] investigation | Mists panel stack overflow layout cycle

Created `investigations/mists-panel-stack-overflow-layout-cycle.md` after reproducing the Achievements/Talents abort through the real GUI click path. Documented that the root cause was active layout resolution re-entering through parent/anchor cycles, not the Lua open-panel path, and recorded the new `headless-click-probe` regression check.

## [2026-05-17] investigation | unanchored frame render leak

Created `investigations/unanchored-frame-render-leak.md` after tracing the startup stray editbox/dropdown to render-list fallback geometry for unanchored frames. The render path now matches Lua rect validity and skips descendants of unanchored frames too.

## [2026-05-17] update | minimap mask clipping

Updated `investigations/minimap-map-ring-alignment.md` after fixing minimap rendering to use the stored/default minimap mask texture instead of synthetic circle clipping. The shader now treats RGB mask intensity as coverage for opaque black/white masks.

## [2026-05-17] investigation | minimap map/ring correction

Created `investigations/minimap-map-ring-alignment.md` to record the correction that the active minimap bug is the map texture/mask/ring alignment, not the SimCommands minimap button. The note documents the reasoning error and directs future debugging at minimap mask/clip/ring geometry.

## [2026-05-15] add | Mists heirloom tooltip

Created `investigations/mists-heirloom-tooltip.md` after the Mists Collections
heirloom button path threw on missing `GameTooltip:SetHeirloomByItemID`.
Documented that the fix belongs on the tooltip widget/data surface and routes
through `C_TooltipInfo.GetHeirloomByItemID`, reusing item tooltip data.

## [2026-05-15] add | Mists addon-panel resume mistake

Created `investigations/mists-addon-panel-resume-error.md` after incorrectly
rerunning already-proven Mists addon panel rows from `AllTheThings`. The page
records the direct rule: resume from the first unproven addon, which was
`Plater` for this run, and use `--start-at` / stable artifact roots instead of
discarding retained evidence. The resumed `Plater` and `SimpleItemLevel` rows
now have retained pass artifacts under the shared cache audit root.

## [2026-05-15] update | Mists backpack slot chrome

Updated `investigations/backpack-background-texture.md` with the Mists-specific
container path: bag ID 0 uses `UI-BackpackBackground` and its item buttons still
keep the authored `UI-Quickslot2` normal texture. Documented that clearing those
normal textures in post-load code is the wrong fix for Mists slot chrome.

## [2026-05-14] add | Hybrid scrollbar thumb texture

Created `investigations/hybrid-scrollbar-thumb-texture.md` after replacing the
HybridScrollBar-specific placeholder fallback with XML-backed slider
`<ThumbTexture>` application. Also documented the SharedXML test helper bug:
tests were pointed at removed `Interface/BlizzardUI` instead of the simulator's
Blizzard UI cache.

## [2026-05-13] update | EditMode active profile fallback

Updated `investigations/editmode-layout.md` with the C_EditMode active profile
state bug: the bootstrap fallback always returned `activeLayout = 1` and had a
no-op `SetActiveLayout()`, so Blizzard selected the first preset layout instead
of a saved profile. Documented the in-memory fallback state model and regression
coverage.

Follow-up: documented the real WTF import path. EditMode layouts come from
`edit-mode-cache-account.txt`; active per-spec selection comes from
`edit-mode-cache-character.txt`. Startup now imports those cache files before
Blizzard addons load, and Blizzard addons use the saved-variable-aware loader.

Follow-up: documented the second-stage layout mapping bug. `C_EditMode` selected
the saved layout, but `EditModeManagerFrame.layoutInfo` prepended presets and
kept the old index, activating `Classic`; startup now remaps the active saved
layout after prepending presets.

Follow-up: documented the visible action-bar anchoring bug. The Widescreen
layout was active and systemInfo was seeded, but action bars stayed at the
temporary `TOPLEFT, 0, 0` anchor because the startup fast path skipped
`ApplySystemAnchor`.

Follow-up: documented main action-bar side art as the `HideBarArt` EditMode
setting. Sparse saved layouts now merge missing defaults from Blizzard's modern
preset map, and the action-bar bootstrap path applies those values without
calling handlers that require an initialized `actionButtons` Lua array.

Follow-up: documented broader action-bar saved setting replay. The startup
fast path now applies safe runtime effects for saved visibility, icon
count/scale/padding, page number visibility, show-grid state, and button art
without repacking saved anchors.

Follow-up: documented raw/display conversion for saved action-bar profile
settings. The cache stores compact raw slider values, so the action-bar fast
path now reads through Blizzard's `GetSettingValue()` before applying display
values such as icon-size percentages.

Follow-up: documented account-level EditMode setting application. Startup now
invokes Blizzard's `InitializeAccountSettings()` after rebuilding layout info,
so saved account toggles are applied rather than merely copied into
`accountSettings`.

Follow-up: documented sparse account cache reconciliation. Imported account
settings now merge over defaults so older profiles still receive default values
for newer account-level EditMode toggles before Blizzard initializes account
settings.

Follow-up: documented cast-bar lock fidelity. Startup no longer overwrites the
active saved profile's `LockToPlayerFrame` / `CastBarUnderneath` settings.

Follow-up: documented saved setting replay for seeded EditMode systems. Startup
now applies saved settings for systems that skip full `UpdateSystem()`, while
deferring unit-frame `BuffsOnTop` if the seeded frame has no `UpdateAuras()`
method and would otherwise add a ScriptErrors entry.

## [2026-05-13] update | CASC font path fallback

Updated `investigations/casc-fdid-1579624-root-debug.md` after the standard
font FDIDs (`615960`, `615958`, `615971`) failed through asset-resolver's
path fallback with lowercase `fonts/...` paths. Documented the fix: preserve
canonical `Fonts/...` casing in the bundled listfile, normalize lookup keys
without losing entry path casing, and skip noisy `resolve_bytes(fdid)` calls
when the CASC resolution cache has no font FDID entry.

Follow-up: documented the real rendering failure after the noise fix. Skipping
missing font FDIDs suppressed the error but selected a system fallback font.
Font loading now reads standard font bytes from local CASC archives by
path-to-encoding resolution or known encoding-key fallback.

## [2026-05-09] update | Wrath vendor source switched to Gethe

Updated `systems/addon-loading.md` and `systems/client-profiles.md` after
standardizing the Wrath 3.3.5 source on `Gethe/wow-ui-source` tag `3.3.5`
(`c4e0255f`). Documented that Wrath's symlink points at the checkout root
because Gethe's 3.3.5 tag stores `AddOns/` and `FrameXML/` at repo root,
unlike newer profiles that keep sources under `Interface/`.
Recaptured the Wrath startup snapshot against the Gethe source (128 distinct
startup messages). The snapshot file was later removed from the Mists-only
merge scope.

## [2026-05-08] update | Windows default GUI build and headless CI compile

Updated `investigations/windows-port-build.md` after reproducing MSVC `LNK1189`
in the default `cargo build --bin wow-sim` path. The root cause remains the
local `iced-dynamic` DLL, but the current fix is to make `fast-build` opt-in
rather than part of default features. Also documented the no-default test
compile contract: GUI/render tests need `cfg(feature = "gui")`, GUI benchmark
binaries need `required-features = ["gui"]`, and CASC examples need
`required-features = ["casc"]`.

## [2026-05-08] update | CASC Friz Quadrata root probe

Updated `investigations/casc-fdid-1579624-root-debug.md` with known-good
FDID `615960` / `fonts/frizqt__.ttf` resolution data, hashes, and extraction
proof for Windows root parser debugging.

## [2026-05-08] add | Windows CASC Blizzard taint

Created `investigations/windows-casc-blizzard-taint.md` after fixing Windows
startup against the CASC-synced Blizzard UI cache. Documented the TOC and
`Blizzard_` folder-name taint semantics, plus the runtime `C_AddOns.LoadAddOn`
stack-taint clearing needed for Blizzard/secure TOCs loaded under a tainted
caller.

## [2026-05-08] ingest | CASC FDID 1579624 root debug

Created `investigations/casc-fdid-1579624-root-debug.md` with the verified
FDID-to-path mapping, content key, encoding key, CRLF hash proof against Gethe
`12.0.5`, extraction proof, local build caveat, and root parser debugging
checklist.

## [2026-05-08] update | CASC resolution cache location

Updated `systems/casc-asset-cache.md` after moving generated CASC resolution
metadata out of repo `data/casc` and into the asset-resolver user cache. The
page now documents product/build-key scoped cache paths and automatic rebuild
behavior for missing or stale `resolution.sqlite`.

## [2026-05-03] add | PVE tabs direct offset

Added `investigations/pve-tabs-direct-offset.md` after tracing the Dungeons &
Raids bottom tab placement to XML direct `<Offset x="..." y="..."/>`
attributes being ignored unless they used nested `<AbsDimension>`.

## [2026-05-02] add | Root-region render order

Added `investigations/root-region-render-order.md` after tracing an inverted
root-region tie breaker. Documented that root-level regions should use ascending
creation order inside the same draw layer, matching child regions, and that
`Reverse(id)` made newer root regions draw underneath older ones.

## [2026-05-02] update | Dropdown base mouse handling

Updated `investigations/dropdown-intrinsic-script-chain.md` after shared input
dispatch gained `RegisterForMouse` state and `SetPropagateMouseClicks` parent
dispatch. Documented why dropdown-like widgets should not need per-dropdown
click shims when the issue is physical mouse registration or child hit targets.

## [2026-05-02] update | LFD queue verbs

Updated `investigations/lfd-dungeon-list-empty.md` after the LFD join path got
past the group-size gate and hit missing `ClearAllLFGDungeons`. Documented the
new `ClearAllLFGDungeons` / `SetLFGDungeon` / `JoinLFG` / `GetLFGInfoServer`
surface and queued-mode state through Blizzard's Lua `GetLFGMode`.

## [2026-05-02] update | LFD normal dungeon group-size gate

Updated `investigations/lfd-dungeon-list-empty.md` after a five-player party saw
`You need a group of 1 players` when joining LFD. Documented that normal dungeon
entries must return nil for `GetLFGDungeonInfo` slot 17 (`minPlayers`) because
Blizzard treats a non-nil value as an exact group-size requirement.

## [2026-05-02] update | LFD reward cap info missing

Updated `investigations/lfd-dungeon-list-empty.md` after the Join as Party path
advanced into `LFGRewardsFrame_EstimateRemainingCompletions()` and hit missing
`GetLFGDungeonRewardCapInfo`. Documented the inert 11-nil return shape Blizzard
uses as the no-cap path.

## [2026-05-02] update | LFD Join as Party format error

Updated `investigations/lfd-dungeon-list-empty.md` after clicking `Join as Party`
raised `bad argument #2 to 'string.format' (number expected)`. Documented that
`GetLFGDungeonInfo` returned `mapName` in Blizzard's `minPlayers` slot, so
`ERR_LFG_MEMBERS_REQUIRED` received a dungeon name where `%d` expected a number.

## [2026-05-02] add | Adventure Guide disabled tabs

Added `investigations/adventure-guide-disabled-tabs.md` after tracing the
apparently unclickable Adventure Guide abilities tab. Blizzard intentionally
shows the tab disabled until a boss is selected, but simulator model stubs had
overwritten the shared `SetDesaturated` / `SetDesaturation` implementation, so
the disabled tab art stayed saturated and looked active.

## [2026-05-01] update | Journeys breadcrumb overlap

Updated `investigations/journeys-midnight-empty.md` with the later Midnight
renown-card overlap root cause. The issue was XML property order:
`setAllPoints=true` was applied after explicit `$parentInset` anchors, clearing
them and stretching `EncounterJournalJourneysFrame` to the full parent.

## [2026-05-01] update | EditBox child background render ordering

Updated `investigations/editbox-render-text-cache.md` after tracing the SimCommands search box typed text to render ordering: the search box's opaque child `BACKGROUND` texture rendered after the EditBox frame, while the EditBox frame emitter owns the internal input text and caret. Documented the new EditBox-specific strata DFS rule that child regions render before the EditBox frame emitter.

## [2026-05-01] update | Tooltip Lua NineSlice center fill

Updated `investigations/tooltip-double-shell.md` after the Journeys renown-card tooltip showed underlying card text through the tooltip body. Documented that the Lua-owned tooltip `NineSlice` should suppress the Rust fallback border/shell, but not the solid center fill needed while the simulator does not have a renderable opaque Lua center.

## [2026-05-01] ingest | Appearances Wardrobe API baseline

Created `investigations/appearances-wardrobe-api.md` after opening Collections Journal > Appearances in the simulator and auditing Blizzard Wardrobe/Transmog call sites against the current `C_TransmogCollection`, `C_Transmog`, and `C_TransmogSets` surfaces. Documented that the panel opens with no Lua errors, but real browsing/filtering/search/favorite behavior needs stateful source, visual, filter, and search backing rather than no-op filter setters and empty Lua bootstrap fallbacks.

## [2026-05-01] update | Wardrobe weapon slot switch crash

Updated `investigations/appearances-wardrobe-api.md` with the root cause for the `Blizzard_Wardrobe.lua:687` nil-index crash. The simulator was reporting main/offhand appearance slot location metadata as weapon collection categories, which made Blizzard treat weapons as armor setup slots; weapon slot metadata now uses `Enum.TransmogCollectionType.None` so the weapon-category path handles them.

## [2026-05-01] update | Wardrobe invalid appearance overlays

Updated `investigations/appearances-wardrobe-api.md` after Wardrobe rendered every head appearance card with the red invalid overlay. Root cause was missing displayability/usability fields on simulator appearance rows; `canDisplayOnPlayer`, usability, source validity, hidden/favorite defaults, name, and quality now come from the `C_TransmogCollection` row backing instead of forcing Blizzard's invalid-card path.

## [2026-05-01] ingest | Adventure Guide boss icon fallback

Created `investigations/adventure-guide-boss-icons.md` after tracing blank Encounter Journal boss icons to `EJ_GetCreatureInfo` returning `0` for missing creature icon fileDataIDs. Documented that Blizzard's boss button Lua relies on nil to select `UI-EJ-BOSS-Default`; `0` is truthy and makes `SetTexture(0)` clear the texture.

## [2026-05-13] update | asset-resolver cache root decoupled from game-engine

Updated `systems/casc-asset-cache.md` after moving `asset-resolver` path selection behind an explicit resolver config. wow-ui-sim now constructs the resolver with `$ASSET_RESOLVER_CACHE_DIR` or the normal user cache root and no longer sets `GAME_ENGINE_SHARED_ROOT` or assumes a local game-engine checkout for CASC/listfile cache data.

## [2026-05-01] update | Adventure Journal dungeon click stack overflow

Updated `investigations/lfd-dungeon-list-empty.md` after reproducing the stack overflow from `EncounterJournal_DisplayInstance(1271)`. Documented the split between modern `C_EncounterJournal.GetInstanceInfo` slot 9 (`linkDungeonID`) and legacy `EJ_GetInstanceInfo` slot 9 (`shouldDisplayDifficulty`) plus the same-button `Button:Click()` reentry guard that prevents the programmatic overview-tab click loop from aborting the process.

## [2026-05-01] update | LFD Join as Party leadership gating

Updated `investigations/lfd-dungeon-list-empty.md` after `JOIN_AS_PARTY` was still greyed out with valid role and dungeon selection state. Documented that party-size fixtures had made `party1` the leader, causing Blizzard's `LFD_IsEmpowered()` to reject the local player; `A_Admin.SetPartySize` and GUI party-size changes now default to local-player leadership.

## [2026-05-01] update | Adventure Journal LFD dungeon handoff

Updated `investigations/lfd-dungeon-list-empty.md` after the Adventure Journal dungeon click path exposed another LFD id/state gap. Documented that `AJ_DUNGEON_ACTION` depends on `DungeonAppearsInRandomLFD` and on Encounter Journal `linkDungeonID` values using the LFD id family, not Encounter Journal instance ids.

## [2026-05-01] update | Crafting cast duration

Updated `investigations/crafting-cast-bar.md` after the crafting bar was found to finish too quickly. The simulator had reused a 1.5 second GCD-style duration for `C_TradeSkillUI.CraftRecipe`; normal profession crafts now use a 2.0 second default, with a regression in `tests/test_crafting.rs`.

## [2026-04-30] ingest | CASC asset cache layers and measured costs

Created `systems/casc-asset-cache.md` after measuring the three stacked caches end-to-end with `examples/casc_bench.rs`. The doc covers (a) the resolution sqlite shared with the game-engine repo via `GAME_ENGINE_SHARED_ROOT`, (b) the per-listfile-path BLP byte cache at `~/.cache/wow-ui-sim/casc-extract/`, and (c) the per-process `TextureManager` in-memory cache, with concrete timings (~300 ms one-time CASC init, ~10 ms steady-state extract, ~1 ms disk hit, ~2 µs mem hit). Records that `Installation::initialize` no longer parses `root.bin`/`encoding.bin` — that work is permanently delegated to the resolution sqlite — so the per-extract cost stays in the millisecond range.

## [2026-04-29] ingest | Backpack body renders gray, not textured

Created `investigations/backpack-background-texture.md`. User showed a retail
screenshot of an open `Backpack` (combined-bags) window with a tan/brown
textured body and reported the simulator was missing it. Render-time tracing
confirmed the sim emits a solid `PANEL_BACKGROUND_COLOR` quad on the
`Bg.TopSection`/`Bg.BottomEdge` textures — i.e. exactly what
`FlatPanelBackgroundTemplate` authors. Both the pinned `12.0.5` vendor and the
`Gethe/wow-ui-source` `live` HEAD (verified via WebFetch + Codex
gpt-5.5/high) define `ContainerFrameCombinedBags` with no body atlas/file and
a no-op `UpdateBackground`. Bank's tan body comes from a separate
`bank-frame-background` atlas declared on `BankFrame` itself, not shared with
the bag panel. Conclusion: the textured retail look is applied outside the
public Blizzard source we have (addon overlay, unmirrored patch, or an
unknown runtime path); closed without a sim-side change.

## [2026-04-30] ingest | client-profiles system page

Created `systems/client-profiles.md` documenting the five-profile cargo-feature layout (retail/wrath/mists/era/anniversary), vendor pinning, profile-aware TOC suffix and gametype tables, per-profile compat bootstraps (`src/wrath/`, `src/mists/`, `src/era/` shared by era + anniversary), wrath's synthetic FrameXML addon, and the CI matrix. Updated `systems/addon-loading.md` to fix the now-stale "Mainline-only TOC suffix" claims and link to the new page; updated `index.md` with a row in `systems/`. Source: PLAN.classic.md Phase 7.x landings (cargo features, profile-aware loader/toc, src/era/ bootstrap), commits 2915f2b9..6b320417.

## [2026-04-30] update | addon-loading per-profile vendor structure

Expanded `systems/addon-loading.md` with a new "Vendor sources & per-profile setup" section: documents the gitignored `Interface/BlizzardUI/<Profile>/` symlink layout, the `setup-blizzard-ui.sh <profile> [ref]` and `init-worktree.sh` scripts, and the canonical vendor-repo + pinned-SHA table. Added per-profile addon-set notes to "Blizzard Addon Load Order" (24 wrath + synthetic FrameXML, ~112 mists, ~35 era/anniversary discovered after `_Vanilla.toc` filtering), and listed `[[client-profiles]]` under See Also. Source: `scripts/setup-blizzard-ui.sh`, `scripts/init-worktree.sh`, vendor-pin commits 73ba3465..afc7189b.

## [2026-04-28] ingest | Windows port build unblock

Created `investigations/windows-port-build.md` after the Windows smoke pass. The root cause was the local `iced-dynamic` re-export crate forcing a huge `iced_dynamic.dll` link, which hit MSVC `LNK1189`; the build now depends on upstream `iced` directly. Verified `wow-sim`, `wow-cli`, GUI startup, and screenshot output on Windows. Updated the note after adding shared WoW resource discovery for install root, CASC `Data`, extracted Interface art, AddOns, and WTF. Live WTF is documented and tested as read-only import; simulator-local SavedVariables take precedence once present.

## [2026-04-28] update | EditMode group-frame startup skip

Updated `investigations/startup-createframe-profile.md` with the resolved
remaining `apply_system_anchors` hotspot. File-based Lua profiling showed full
`UpdateSystem()` on `CompactRaidFrameContainer`, `PartyFrame`, and
`CompactArenaFrame` dominated the first pass; the workaround now seeds those
frames and applies anchors without running full roster/unit layout work.

## [2026-04-28] update | remaining EditMode startup hotspot narrowed

Updated `investigations/startup-createframe-profile.md` with follow-up probes
on the remaining post-load `apply_system_anchors` cost. `InitSystemAnchors`
and the registered-system loop were effectively free; the cost stayed in
`UpdateBottomActionBarPositions()` / managed-frame layout. Recorded the
discarded forbidden-attribute no-op idea because it broke
`tests/frame_positions.rs` by shifting `ObjectiveTrackerFrame`.

## [2026-04-28] update | duplicate post-event EditMode pass

Updated `investigations/startup-createframe-profile.md` with the startup
workaround duplication found during dump-tree profiling. `PLAYER_ENTERING_WORLD`
already ran post-event workarounds through `env_events.rs`, then
`fire_startup_events()` and `settle_headless_startup()` ran the same pass again.
Added one-shot state for `WowLuaEnv::apply_post_event_workarounds`; local
dump-tree logs now show two `apply_system_anchors` passes instead of four.

## [2026-04-28] ingest | three-slice button tiling

Created `investigations/three-slice-button-tiling.md`. Escape menu red button
stripes came from standard `HighlightTexture` children rendering while the
buttons were not hovered. The red-button center atlas special case was removed;
atlas tiling uses source size, and highlight children now render only when the
parent button is hovered or highlight-locked. Additive overlays also skip the
shader brightness boost so active highlights do not amplify low-alpha atlas edge
pixels into visible stripes.

## [2026-04-28] update | eager animation_frame_ids_for_group

Updated `investigations/talent-performance.md` with the panel-open 1.3 FPS root
cause. `advance_animation_group()` called `animation_frame_ids_for_group` on
every group every tick (full linear scan of `anim_frame_to_anim`), but the
result was only consumed inside the `if group_finished` branch. Moved the call
into the conditional. Live-process flamegraph went from 63.9% CPU in that
function to 0.00%; total animation-tick cost dropped from dominant to 3.5%.

## [2026-04-28] update | talent strata repair skip

Updated `investigations/talent-performance.md` with the discarded strata repair
root cause. `set_frame_visible()` was building same-strata repair plans during
talent frame show even when `strata_buckets` was `None`, so the work was thrown
away. Added the guard in `try_repair_strata_buckets_after_show`; release
`bench_talents` subsequent opens dropped from roughly 211-282ms to 112-150ms in
this worktree.

## [2026-04-28] ingest | menu pool SetToDefaults size/anchor reset

Created `investigations/menu-pool-set-to-defaults.md`. Guild roster Mythic+ Rating dropdown rendered as a screen-spanning stripe because `Frame:SetToDefaults` did not reset size or clear anchors. `Menu.lua` `MeasureFrameExtents` reads `frame:GetSize()` from pooled element frames, so previous-user widths inflated each menu measurement. Two `SetToDefaults` registrations existed on the shared frame metatable; `map_frames::register_all` runs after `misc::register_all` so the map_frames version is the active one (the misc registration is dead code). Extended `map_frames::set_to_defaults` to call `frame.clear_all_points()`, `frame.set_size(0.0, 0.0)`, clear `width_is_text_auto`, clear `layout_rect`, and `remove_all_anchor_dependents_for(id)` — matching real WoW semantics documented in `Compositor.lua`. Verified: Guild dropdown now stays at 180×103 even after a 900px synthetic dropdown is opened first; previously it grew to 1036→1172. All 8 minimap_specialized tests still pass.

## [2026-04-27] update | shallow `issecretvalue` for pool releases

Updated `investigations/talent-performance.md` with a new "Spec→Talents Tab Switch (~3.5s)" section. The Spec→Talents tab switch was multiple seconds because `LoadTalentTreeInternal` rebuilds the tree on every Show (`refreshOnShow=true`), and `talentButtonCollection:ReleaseAll()` calls `issecretvalue(frame)` 3× per button. The Rust fallback recursed into the entire frame's table tree (~7.4ms/call). Added `value_is_secret_shallow` in `src/lua_api/globals/security/secret_values.rs` that only inspects direct slot taints on tables, used by `issecretvalue`/`canaccessvalue`/`canaccessallvalues`. `canaccesstable` keeps the deep walk so its accessibility semantics still detect nested secret strings. Result: ReleaseAll 2159ms → 2.6ms; tab switch 3500ms → ~90ms; all 45 security_api tests pass.

## [2026-04-27] ingest | hero spec dialog anchor investigation

Created `investigations/hero-spec-dialog-anchors.md` documenting two anchor resolution bugs in `HeroTalentsSelectionDialog`. (1) `xml_layer_batch.rs` emitted all textures before all fontstrings into the same Lua chunk, breaking XML document order so SpecImage's `relativeKey="$parent.SpecName"` ran before SpecName existed and fell back to the spec frame. (2) Runtime template path lacked the loader's `resolve_named_anchor_targets_for_frame` re-pass, leaving NodesContainer's `$parent.Description` anchor as an unresolved string. Both fixed; talent node icons now render inside their LIGHTSMITH/TEMPLAR panels instead of at the screen bottom.

## [2026-04-27] update | CASC migration: textures and fonts

Migrated texture and font loading off bundled extracts onto direct CASC reads from the live WoW install at `/syncthing/World of Warcraft/Data` (via the `asset-resolver` crate). Removed `./textures` (~1740 WebPs), the `~/Projects/wow/Interface` BLP fallback path, the `disk_cache_dir` field, and `src/texture_cache.rs`. `WowFontSystem::new()` became no-arg and pulled FRIZQT__/ARIALN/frizqt___cyr from CASC; a short-lived embedded FRIZ fallback was later replaced by CASC encoding-key fallback so the repo does not ship font bytes. Updated `docs/wiki/systems/texture-atlas.md`, `docs/texture-atlas-system.md`, `docs/rendering-pipeline.md`, `docs/wiki/reference/cli-commands.md`, and `docs/wiki/index.md` to drop references to the old curated paths and the now-obsolete `convert-texture` / `extract-textures` add-a-missing-texture flow. CASC is gated by the `casc` feature (default-on); set `WOW_SIM_CASC=0` to disable.

## [2026-04-26] ingest | dropdown intrinsic script chain investigation

Created `investigations/dropdown-intrinsic-script-chain.md` to document the ReputationFrame dropdown root cause: style dropdown templates replaced intrinsic `DropdownButton` scripts, so Blizzard's `OnMouseDown_Intrinsic` was not in the click path. Recorded the simulator-side fix, the fake menu fallback removal, and the two regression tests.

## [2026-04-26] update | api-coverage refresh, FUTURE.md retired

Folded `FUTURE.md` into `docs/wiki/reference/api-coverage.md` and deleted the root file. The old "three-layer" stub architecture (Hand-written / `c_stubs_api*.rs` / `generated_stubs.rs (~19K lines)`) has been replaced by per-namespace modules under `src/c_api/` with explicit `permanent_shims/` and `temporary_shims/` subtrees, matching the C-API boundary policy in CLAUDE.md. The wiki page now describes the actual module layout, calls out the shim sub-trees, and points to `wow-cli audit-api --gaps --format plan` as the live source of remaining work instead of a hand-curated task list. Removed the `FUTURE.md` source link.

## [2026-04-26] update | architecture-overview refresh, DESIGN.md retired

Folded `DESIGN.md` into `docs/wiki/design/architecture-overview.md` and deleted the root file. Corrected several stale claims: rilua provides native taint tracking (the previous "stubbed as always-secure" line was wrong — `issecure`/`issecurevariable` work; what's missing is `SetAttribute`/`SetForbidden` enforcement, now spelled out as a non-goal). Removed the dropped `generated_stubs.rs (~19K lines)` reference. Expanded the module diagram beyond `lua_api`/`widget`/`render` to also cover `c_api`, `iced_app`, `loader`, `event`, `xml`, `lua_bridge`, `texture`, and `sound`, with `c_api` called out as a peer of `lua_api` per CLAUDE.md. Updated `addon-compatibility.md` and `development-phases.md` to drop the fixed "127+ addons" number and reflect that the Blizzard UI tree under `Interface/BlizzardUI/` (`SharedXMLBase`/`SharedXML`/`SharedXMLGame`/`FrameXMLBase`/`FrameXMLUtil`/`FrameXML` + per-feature addons) loads, not just `SharedXML`. Removed source links to the deleted `DESIGN.md`.

## [2026-04-26] update | scaling-coordinates refresh

Verified `docs/wiki/design/scaling-coordinates.md` against current source and folded the standalone `SCALING.md` into the wiki page (root `SCALING.md` deleted). Most open items from the original note are done: `GetScreenWidth`/`GetScreenHeight`/`GetPhysicalScreenSize` are now installed dynamically by `install_screen_size_globals()` in `src/lua_api/env_runtime.rs` and re-run from `set_screen_size()`; the hardcoded `TOPLEFT (10, -10)` override in `main.rs` and the debug purple border are gone; layout `size` flows through `src/iced_app/render/rebuild.rs`. Updated file paths (`src/iced_app/` is a directory; `src/lua_api/globals.rs` no longer exists), clarified that the renderer runs in iced top-left Y-down with Y flipped in `Uniforms::new`, and trimmed open items to anchor Y-axis end-to-end docs and a `CENTER`-anchor resize regression.

## [2026-04-24] add | achievement panel hide investigation

Added `investigations/achievement-panel-hide.md` to document the achievement
panel hide fix. The simulator workaround now delegates to Blizzard's real
`AchievementFrame_ToggleAchievementFrame()` / managed `HideUIPanel` path, and
animation advancement now fires child animation `OnFinished` handlers so XML
outro hide scripts can run. Updated `index.md` with the new page.

## [2026-04-24] update | taint-system doc refresh

Updated `systems/taint-system.md` to match the current rilua implementation and added a Blizzard `issecure()` call-site matrix with test coverage references.
SecureHandler APIs now document fallback frame-ref/snippet/wrap/unwrap behavior,
state and attribute drivers now document shallow driver application,
secret-value accessors now document marked/tainted value tracking, and source
links now point at `security.rs` plus current frame-method helpers instead of
removed `security_api.rs`, `secure_env.rs`, and `combat_lockdown.rs` paths.
Refreshed the `index.md` summary.

## [2026-04-24] update | spell description token resolver

Updated `systems/lua-api.md` to record the shared spell-description token
resolver used by both `C_Spell.GetSpellDescription()` and
`C_TooltipInfo.GetSpellByID()`. The note covers the supported DB2 token
families (`$s1`, `$<damage>`, `$<shield>`, `${...}`, `$STR`, `$INT`, `$AP`)
and the reason for the shared path: keep spellbook/tooltips from exposing raw
placeholders or drifting from the C API result. Refreshed the `index.md`
`[[lua-api]]` summary.

## [2026-04-24] update | SimulationCraft spell coefficients

Updated `systems/lua-api.md` with the local SimulationCraft source used for
spell-description coefficients and variables. Recorded the concrete formulas
now mirrored by `src/spell_description_resolver.rs`: Avenger's Shield
`1.55 * AP`, Crusader Strike `1.4 * AP`, Shield of the Righteous `0.95 * AP`,
Eye Beam `$<dmg>` as `10 * 0.4026 * AP`, and Shield of Vengeance `$<shield>`
as `30% max health * (1 + versatility damage)`.

## [2026-04-23] add | quest scrollbar partial XML size investigation

Added `investigations/quest-scrollbar-partial-size.md` to document the
QuestScrollFrame scrollbar alignment bug. Root cause was the direct XML
property path ignoring partial `<Size x="..."/>` / `<Size y="..."/>` values
unless both dimensions were present. Updated `systems/xml-template-system.md`
to record per-dimension size resolution and added the new investigation to
`index.md`.

## [2026-04-22] add | layout lock inventory reference page

Added `reference/layout-lock-inventory.md` as the consolidated source of truth
for UI layout lock coverage. The page inventories baseline frame rect locks
(`tests/frame_positions.rs`) plus subsystem-specific lock tests for objective
tracker, main action bar, status tracking bars, bag bar, micro menu, chat
frame, compact raid manager, character/reputation panels, and buff
icons/durations. Updated `index.md` with a new `[[layout-lock-inventory]]`
entry under `reference/` for discoverability.

## [2026-04-22] update | buff aura onupdate perf lock-down to 0.5ms

Updated `investigations/on-update-dirty.md` with the AuraButton
`OnUpdate <=0.5ms` lock-down pass. Recorded the focused perf harness
(`tests/buff_aura_onupdate_perf.rs`), the pre-fix max (`~0.86ms`),
the dominant hotspot (`SetFormattedText` inside `UpdateDuration`), and the
post-fix max (`31.44us`). Documented the engine-side fixes:
`SetFormattedText` width-hint no-op fast path,
`securecall` direct multi-return fast path with protected fallback, and
non-visual `FrameRef.__newindex` bookkeeping updates.

## [2026-04-22] update | setpoint no-op fast-path optimization

Updated `investigations/on-update-dirty.md` with the post-optimization
`SetPoint` rerun after moving the same-anchor no-op bail-out to
`set_point()` before `ensure_no_anchor_cycle(...)`. Recorded the measured
before/after no-op totals (`3.626955us -> 1.392966us`, about `61.6%` lower)
and updated the control-flow notes to reflect that cycle detection now runs
only on real anchor changes.

## [2026-04-22] update | formattedtext and fontobject no-op fast paths

Updated `investigations/on-update-dirty.md` with the post-optimization
measurements for `SetFormattedText` and `SetFontObject` after
`text/formatting.rs` changes. Documented the new same-signature
`SetFormattedText` cache guard (enabled only while global `format` matches the
captured default), confirmed behavior parity for overridden `format`
(`format_call_probe_same_text_calls=100`), and recorded the large
`SetFontObject` no-op drop (`steady_same_total_us` from `6.104us` baseline to
`1.201us` in the rerun).

## [2026-04-22] update | setalpha no-op fast-path optimization

Updated `investigations/on-update-dirty.md` with the `SetAlpha` fast-path
optimization follow-up in `core_state/alpha.rs`. Recorded a post-change
rerun of the existing `setalpha_bench.lua` microbenchmark and the key no-op
delta metric (`same - get`) showing lower measured no-op overhead than the
earlier baseline, plus the reminder that `noop_hot_setters` behavioral
contracts still pass.

## [2026-04-22] update | no-world-map retained trace after bc-negative cache

Updated `investigations/world-map-texture-loading-budget.md` with a fresh
no-world-map retained GUI trace captured after the BC-negative cache change.
Recorded the exact command and before/after peak timings from the trace logs:
`draw textures` dropped `482.2ms -> 283.2ms` and `bc_parse` dropped
`240.6ms -> 139.2ms`. Refreshed the `index.md` summary row for
`world-map-texture-loading-budget` with the new no-world-map baseline deltas.

## [2026-04-22] update | setalpha no-op microbenchmark baseline

Updated `investigations/on-update-dirty.md` with a pre-optimization
`SetAlpha` microbenchmark baseline from a headless `--exec-lua` run. Recorded
batch timings (`empty`, `GetAlpha`, same-value `SetAlpha(1)`, and alternating
state-change `SetAlpha`) and the required split:
Lua->Rust call overhead, same-value fast-path overhead, and real
state-change overhead.

## [2026-04-22] update | setformattedtext no-op microbenchmark baseline

Updated `investigations/on-update-dirty.md` with a pre-optimization
`SetFormattedText` microbenchmark baseline from a headless `--exec-lua` run.
Recorded the three required splits (argument formatting/parsing cost,
text-equality fast-path cost, and real text-change cost) and added a runtime
probe proving global `format()` is still called on same-text no-op updates.

## [2026-04-22] update | setpoint no-op microbenchmark baseline

Updated `investigations/on-update-dirty.md` with a pre-optimization
`SetPoint` differential microbenchmark baseline. Recorded split timings for
implicit-noop parse baseline, explicit target normalization/lookup overhead,
string target lookup overhead, a no-op equivalence-check proxy, and full
relayout/dirty extra cost. Also documented the static control-flow ordering in
`anchors.rs` proving anchor resolution and `ensure_no_anchor_cycle` run before
the no-op `unchanged` bail-out in `apply_set_point`.

## [2026-04-22] update | fontobject shown vertexcolor no-op baselines

Updated `investigations/on-update-dirty.md` with measured no-op and change-path
splits for `SetFontObject`, `SetShown`, and `SetVertexColor`
(`dispatch`, `pre-bail`, `true state-change`). Recorded the steady-state
comparison against the earlier `SetAlpha` baseline and pinned
`SetFontObject` as the highest remaining no-op cost in this primitive set.

## [2026-04-22] update | world-map Lua retry simplification

Updated `investigations/world-map-texture-loading-budget.md` with the
follow-up Lua pin retry cleanup after the request-handle refactor. The
`MapExplorationPinMixin` workaround now keeps only one deferred refresh at a
time instead of recursively re-arming the old timer loop, and the world-map
detail tests still pass. Refreshed the `index.md` summary row for
`world-map-texture-loading-budget` to mention the simplified Lua retry layer.

## [2026-04-22] update | world-map live retained trace recapture

Updated `investigations/world-map-texture-loading-budget.md` again with the
current HEAD live-GUI recapture. Recorded the retained startup sequence from
`ToggleWorldMap()` through `tick -> draw -> prepare -> present`, using the
exact `iced-debug` socket emitted by the process for the screenshot burst. The
first world-map tick was `dirty=0x1ff pending=true ready=6`; later draws
advanced atlas-ready `6 -> 24 -> 33 -> 282 -> 335`; the first post-present
world-map screenshot was already textured; and redraws continued after
`pending=false` because `strata_dirty` remained `0x1c`. Refreshed the
`index.md` summary row for `world-map-texture-loading-budget`.

## [2026-04-22] update | world-map RedrawAll verification

Updated `investigations/world-map-texture-loading-budget.md` with the
timer-path `RedrawAll` verification from the same live retained-GUI repro.
Recorded that the first post-warmup frame reached `draw -> prepare -> present`
without any extra presentation gate after texture warmup, with
`ready=38 -> 335` as `prepare()` drained the atlas backlog. Refreshed the
`index.md` summary row for `world-map-texture-loading-budget` to mention the
verified post-warmup frame path.

## [2026-04-22] update | world-map atlas tier pressure audit

Updated `investigations/world-map-texture-loading-budget.md` with the atlas
tier pressure audit from the same live GUI repro. Recorded that every observed
`prepare()` pass kept `retry=0` and `force_rgba_retry=0`, so there were zero
RGBA fallback failures and zero BC upload rejections while the world-map tile
paths drained queued work and reached atlas-ready completion. Refreshed the
`index.md` summary row for `world-map-texture-loading-budget` with the
no-rejection conclusion.

## [2026-04-22] update | world-map retained GPU buffer reupload audit

Updated `investigations/world-map-texture-loading-budget.md` with the retained
GPU buffer reupload audit from the same live GUI repro. Recorded that
`upload_strata` showed the dirty world-map strata being re-written with
`pending_tex_vertices=0` and resolved sample `tex_index` / UVs, proving the
first-open retained path was re-uploading the affected vertex buffers after the
atlas transition. Refreshed the `index.md` summary row for
`world-map-texture-loading-budget` with the buffer-reupload conclusion.

## [2026-04-22] update | world-map retained texture display follow-up

Updated `investigations/world-map-texture-loading-budget.md` with the latest
live-GUI follow-up. Documented that the screenshot path could render the world
map on first pass while the retained GUI still missed random tiles/overlays,
then recorded the four later bugs behind that gap: wrong warmup request source,
full-rebuild sentinel clobbering, inverted world-map request priority, and the
remaining `textures_pending` ownership conflict between queued preload and
draw-time retained recovery. Refreshed the `index.md` summary row for
`world-map-texture-loading-budget`.

Later that day, updated the same page again after the deeper state-model fix:
`gpu_uploaded_textures` was only a draw-staging set populated before
`prepare()`, not proof that a path was ready in the atlas. Recorded the new
atlas-ready tracker populated from `WowUiPrimitive::prepare()` and the switch
away from using the staging set for display-readiness checks.

## [2026-04-22] update | buffframe slow onupdate interpretation

Updated `investigations/on-update-dirty.md` with the current BuffFrame
slow-handler interpretation. Documented that logs like
`addon=Blizzard_BuffFrame handler=OnUpdate frame=#21946` map to anonymous
`AuraButtonTemplate` children, not the named `BuffFrame` root, using the
current `BuffFrame.lua` creation path, `handler_timing.rs` fallback formatting,
and a live `dump-tree --filter-key BuffFrame --visible-only` run that showed
five visible anonymous timed buff buttons. Also corrected the stale audit note
to match the current `onupdate_handler_audit.rs` regression coverage.

## [2026-04-21] update | world-map exploration non-current map surface

Updated `investigations/world-map-fog-of-war-overlay-model.md` with a
follow-up regression where `runtime_surface_bootstrap.lua` was still installing
synthetic `C_MapExplorationInfo` handlers and limiting explored overlays to
`currentMapID` / map `1`. Documented the new Rust-backed
`src/c_api/c_map_exploration_info.rs` implementation, fallback-only bootstrap
stubs, and focused non-current-map regressions in
`tests/c_map_exploration_info.rs`. Refreshed the `index.md` summary row for
`world-map-fog-of-war-overlay-model`.

## [2026-04-21] add | class talents edge frame levels

Added `investigations/class-talents-edge-frame-levels.md` documenting the
class-talents edge-over-icon regression and the fix in
`src/lua_api/workarounds.rs` that patches both the mixin and the live
`PlayerSpellsFrame.TalentsFrame` method, then re-levels active edges.
Recorded regression coverage in
`test_class_talent_edges_render_below_visible_talent_buttons`
(`tests/hero_talents.rs`) and updated `index.md`.

## [2026-04-21] update | hero talents visibility and edges

Updated `investigations/class-talents-trait-loadout-state.md` with the hero
subtree rendering regression where only one node appeared and connector edges
were missing. Documented root cause in
`check_spec_conditions_met()` (`src/lua_api/globals/missing_surface/traits.rs`):
spec-set conditions were treated as `AND` instead of `OR` across shared hero
node groups. Recorded the fix and new regression test
`test_active_hero_subtree_exposes_multiple_visible_nodes_and_edges` in
`tests/hero_talents.rs`. Updated `index.md` summary for the investigation page.

## [2026-04-20] investigate | tooltip layout timing

Added `investigations/tooltip-layout-timing.md` to capture the tooltip
one-frame mismatch caused by sizing after layout resolution. Documented that
`update_tooltip_sizes()` runs too late in the live render path, so the current
frame can render from stale `layout_rect` data even though tooltip line data is
fresh.

## [2026-04-20] ingest | tooltip double shell

Added `investigations/tooltip-double-shell.md` to capture the duplicate
tooltip chrome bug. Documented the two-layer root cause: a bootstrap-created
fake `NineSlice` surface on the Lua side plus an unconditional Rust fallback
tooltip background. Recorded the fix: remove the bootstrap injection, repair
tooltip `NineSlice` post-load with the real template-backed surface, and gate
the Rust fallback shell on whether the frame already owns `NineSlice`.

## [2026-04-20] update | blizzard ui test lanes

Added `reference/blizzard-ui-test-lanes.md` and updated
`reference/addon-compatibility.md` to document the explicit split between
Blizzard UI unit tests and addon-bootstrap coverage.

## [2026-04-20] update | blizzard ui addon closure resolver

Added `loader::discover_blizzard_addon_closure_for_screen()` and switched the
render-order test helpers to use it. The resolver walks TOC `Dependencies` and
`OptionalDeps` across the full screen-allowed Blizzard TOC set, so tests can
resolve explicit closures for load-on-demand roots instead of relying on a fake
monolithic Blizzard bundle.

## [2026-04-20] update | blizzard ui smoke targets

Updated `reference/blizzard-ui-test-lanes.md` with the first four explicit
addon-bootstrap smoke targets: combat log, macro UI, world map, and
settings panel. Added the shared smoke-target manifest and harness coverage in
`tests/common/blizzard_addon_manifest.rs`,
`tests/common/blizzard_addon_harness.rs`, and
`tests/blizzard_addon_smoke_targets.rs`.

## [2026-04-20] update | blizzard ui smoke target startup shape

Updated the smoke-target lane so it asserts target startup shape instead of
just "closure loads". The harness now preloads shared panel support, clears
that preload noise, then loads only the target closure; each target asserts no
recorded Lua errors, the expected global/frame pair, and one or two
representative behaviors.

## [2026-04-20] update | blizzard ui addon-bootstrap test home

Documented that addon-bootstrap regressions stay in `cargo test`, while
`wow-sim run-tests` remains the lane for addon-authored Lua suites. The key
tradeoff is that the smoke harness needs direct Rust access to loader state,
recorded Lua errors, and frame-tree assertions, which is easier to maintain in
the normal Rust test binaries than in a Lua-only wrapper.

## [2026-04-20] update | keybinding spellbook direct dispatch

Updated `investigations/keybinding-system.md` with the spellbook follow-up.
Documented that the simulator now routes `S` back to Blizzard-owned
`PlayerSpellsUtil.ToggleSpellBookFrame()`, removes the bootstrap spellbook
fallback wrapper, and dispatches simple zero-arg binding targets directly
instead of always compiling a Lua chunk first. Recorded the key finding that
raw Blizzard spellbook toggles were already correct and the remaining
first-open regression only happened through binding dispatch.

## [2026-04-20] add | class talents trait loadout state

Added `investigations/class-talents-trait-loadout-state.md` to capture the
remaining `C_Traits` restore that unblocked `PlayerSpells`. Documented the root
cause (live talent state existed, but Blizzard-facing trait queries still
returned placeholder config IDs / staged-change surfaces), the new
working-vs-committed diff model exposed through `TalentState`, and the focused
regression coverage for config mapping, purchase gating, and staged edits.
Updated `index.md` with the new page.

## [2026-04-20] ingest | partyframe portrait composition

Added `investigations/partyframe-portrait-composition.md` to capture the
party portrait sizing/composition result from live queries and Blizzard XML.
Documented that the class icon is the `37x37` `Portrait` texture, while the
visible ring/surround is not a separate widget and instead comes from the
larger `UI-HUD-UnitFrame-Party-PortraitOn` frame-art texture (`120x49` in the
live master GUI tree). Updated `index.md` with the new page.

## [2026-04-19] ingest | partyframe status-bar texture drop

Added `investigations/partyframe-statusbar-textures.md` to capture the root cause for the party health/mana bar `MISSING` render. The XML loader creates the bar child correctly, but `SetStatusBarTexture(bar)` passes a userdata frame into a setter that only accepts strings/numbers, so the status-bar source is cleared. Updated `index.md` with the new page.

## [2026-04-18] update | Track 2 metatable handle threading

Updated `investigations/track-2-intern-audit.md` with the current
handle-threading progress: a shared `hot_metatable_key(...)` accessor
now reuses the prewarmed registry handle for `__index` / `__newindex`
lookups in `methods.rs`, `globals/create_frame/helpers.rs`,
`globals/security.rs`, and `env_init/freeze_globals.rs`, with a static
fallback for bootstrap-skipping tests.

## [2026-04-17] ingest | rilua vs mlua gap audit

Added `investigations/rilua-mlua-gap-audit.md` after comparing the current
rilua registration path against `master`'s mlua-era Lua API surface. Recorded
the highest-signal missing handling buckets: no-op sandbox cleanup in
`env_init`, dropped `MessageFrame` method registration, unfinished
attribute/event/text widget parity, and the unwired `patch_namespace_stubs()`
runtime hook. Updated `index.md` with the new investigation page.

## [2026-04-17] update | startup XML loader fast-path follow-up

Updated `investigations/startup-createframe-profile.md` with the current XML
loader fast-path state after the runtime `CreateFrame` work. Recorded the
safe widening steps in `xml_frame/setup.rs` and the split
`template_chain/` machinery, the current clean `xml fast path` counters
(`hits=1868`, `slow=350`, `scripts=234`), the shared-worktree debug startup
range (~`4.8s`-`5.8s` on `--no-addons --no-saved-vars`), and the failed
generic global-method literal-arg experiment that still regresses real
startup. Refreshed the `index.md` summary for the page.

## [2026-04-16] update | intern_string perf re-profile correction

Corrected the earlier PLAN/wiki summary for the post-migration interning
profile. Fresh release `perf` on `wow-sim --no-saved-vars lua-errors` shows
`Gc::intern_string` at 179.5M cycles (2.98%), `StringTable::intern_hashed`
at 169.2M (2.81%), and inline `lua_hash` at only ~4.8M (0.08%). The original
"lua_hash essentially flat" note was wrong; the hash primitive is no longer
the bottleneck, and the remaining cost sits in bucket traversal / dedup work.

## [2026-04-16] update | intern_string_static mid-cycle fix + migration landed

Found the root cause of the earlier migration breakage. `intern_string_static`
inserts into `static_intern_cache`, but `mark_gc_roots` only scans that cache
at cycle start — a mid-Propagate insert is still pre-flip current-white, gets
swept at cycle end, and the cache ends up pointing at a freed slot. Fixed in
rilua by colouring the new ref Black during Propagate/Sweep/Finalize; added
`intern_string_static_mid_cycle_survives_sweep` regression test.

Applied the migration for `registry_get/set/table_or_create`, `registry_table`,
`attach_frame_metatable`, and fan-out callers — intern counter 1,250,287 →
1,096,266 (−12%), release startup 1.18s → 1.15s median (n=5). `frame_ref_cache`
(the biggest single call site, 286K/startup) remains deferred: even with the
GC fix, migrating that path cascades into "OnLoad (a nil value)" failures
for ~300 addons and the root cause isn't yet understood.

## [2026-04-16] ingest | intern_string call-site ranking

Added `investigations/intern-string-ranking.md` and the rilua
`intern-stats` feature. Startup runs 1.25M `intern_string` calls with
top 5 literals accounting for 40%. Attempted migrating the
registry/frame helpers to `intern_string_static` (counter dropped
−74.5%) but release triggered 22 new Lua errors — frames lose their
metatable methods when `registry_set` uses `intern_string_static`.
Reverted the migration; filed as a rilua follow-up.

## [2026-04-16] ingest | layout computation profile

Added `investigations/layout-profile.md`. Release `perf` on `lua-errors`
showed layout-attributed samples at 7.5% of total (470M of 6.3B), with
the biggest single contributor being `LayoutCache::get` via siphash —
the cache was a default `HashMap<u64, CachedFrameLayout>`. Swapped to
`FxHashMap`. Layout 7.5% → 5.0%, total siphash 295M → 76M, release
startup median 1.21s → 1.18s (n=10). Remaining layout cost is real
anchor arithmetic and `resolve_parent_rect` recursion — no further
easy wins.

## [2026-04-16] update | table rehashing fix #2 declined

Tried fix #2 (short-circuit `raw_set_impl` for sequential integer keys
when hash is empty). Two variants both net-negative:
- `array.push` (grow by 1): rehashes 69K → 86K (+25%), wall time tied.
- `array.resize(next_power_of_two)`: rehashes 69K → 57K (−18%) but
  wall time +30% from eager nil-fill on each boundary.

Conclusion documented in `investigations/table-rehashing.md`: the
existing rehash path's `compute_sizes` already does good amortization;
naive replacements either skip the over-allocation (more rehashes
later) or pay the over-allocation eagerly (worse wall time). Status
quo retained.

## [2026-04-16] update | table rehashing fix #1 applied

Applied fix #1 from `investigations/table-rehashing.md`: rilua
`OpCode::NewTable` now pre-allocates 4 hash slots when both size hints
are zero. Measured: 97,340 → 69,105 rehashes (−29%); release startup
median 1.31s → 1.21s (−8%, n=5 each). All 685 rilua tests pass (includes
new `op_newtable_empty_hint_preallocates_hash_to_avoid_first_rehash`).

## [2026-04-16] ingest | table rehashing investigation

Added `investigations/table-rehashing.md`. Profiled startup rehash counts via a
new `rehash-stats` feature in rilua. Found 97K rehashes — 98% from non-frame
tables (`OP_NEWTABLE(0,0)` from addon `local t = {}` pattern), 81% landing at
hash size ≤ 16. Frame-table pre-sizing (64 slots) is working: only 1.6K frame
rehashes. Documented three candidate fixes; none applied in this card.

## [2026-04-14] ingest | world-map exploration seed follow-up

Updated `investigations/world-map-fog-of-war-overlay-model.md` with the current
map exploration follow-up. After removing synthetic fog, Isle of Dorn still
showed fully explored because every default-visible overlay was treated as
discovered. Documented the temporary seed that leaves one real overlay chunk
(`WorldMapOverlay.ID = 4885`, The Three Shields / Skolzgal Mill) unexplored
until per-character exploration state exists, and refreshed the `index.md`
summary.

## [2026-04-14] ingest | world-map fog-of-war overlay model correction

Updated `investigations/world-map-fog-of-war-overlay-model.md` to correct the
previous diagnosis. The current world map does not have a `UiMapFogOfWar` DB
row, so the bug was not a wrong irregular fog shape; it was the simulator
inventing fog for any map art and rendering synthetic geometry from exploration
overlay gaps. Documented the DB-backed fog lookup, the removal of the synthetic
renderer, and the new API/render regressions. Updated `index.md` with the
corrected summary.

## [2026-04-14] ingest | world-map fog-of-war overlay model

Created `investigations/world-map-fog-of-war-overlay-model.md` for the third
world-map fog bug: exploration APIs were already using real irregular overlay
chunks, but the fog renderer still assumed a synthetic half-map model.
Documented the root cause, the `FogOfWarFrame` `uiMapID` plumbing, the new
overlay-complement fog geometry, and the focused API/render regressions.
Updated `index.md` with the new investigation page.

## [2026-04-14] ingest | world-map texture loading budget follow-up

Updated `investigations/world-map-texture-loading-budget.md` with a first-frame
world-map follow-up: BC-preloaded tiles were landing in `bc_cache`, but
`TextureManager::is_cached()` only consulted the RGBA cache. That caused
budgeted draw to pause early after the first BC upload and could make the
world map open with an apparent quarter-map fog/exploration artifact. Added
the BC-cache root cause, fix, and regression coverage to the investigation
page.

## [2026-04-14] ingest | world-map fog-of-war first-open size

Created `investigations/world-map-fog-of-war-first-open-size.md` for the fog
overlay bug where first-open world-map fog could keep a stale size even though
the map tiles were already correct. Documented the missing
`FogOfWarPinMixin:OnCanvasSizeChanged()` handling, the simulator-side
workaround that patches both the mixin and existing fog pins, and the focused
regression tests. Updated `index.md` with the new investigation page.

## [2026-04-14] investigations | world map 90s OnUpdate recapture

Updated `investigations/world-map-onupdate-hover-polling.md` with the fresh
90s world-map profile after the recent OnUpdate fixes. Recorded the new
`/tmp/worldmap-onupdate-20260414.log` numbers (`485` total `fire_on_update`
spikes, `31` steady-state handlers, `64.73ms` post-90s average), added the
new `world_map_onupdate_inventory` handler-ceiling regression test, and
refreshed the `index.md` summary for the page.

## [2026-04-14] investigations | startup XML lifecycle frame-id threading

Updated `investigations/startup-createframe-profile.md` with the loader
follow-up that removes repeated `name -> id -> frame_ref` lifecycle resolution
during XML finalize. Recorded the new `xml_frame.rs` / `xml_lifecycle.rs`
threaded-frame-id path, the focused regression test that fires lifecycle
handlers with a wrong display name but the correct frame id, and refreshed the
`index.md` summary for the page.

## [2026-04-14] investigations | world map UIParent empty worklist follow-up

Updated `investigations/world-map-onupdate-hover-polling.md` with the
`UIParent_OnUpdate` fan-out follow-up: `FCF_OnUpdate`, `ButtonPulse_OnUpdate`,
and `AnimatedShine_OnUpdate` were still doing empty-list Lua dispatch every
tick. Recorded the new post-load wrappers in `workarounds.rs`, the focused
`uiparent_onupdate_worklists` regression tests, and refreshed the `index.md`
summary for the page.

## [2026-04-14] investigations | on-update dirty GameTimeFrame calendar atlas follow-up

Updated `investigations/on-update-dirty.md` with the `GameTimeFrame_SetDate()`
follow-up: same-day calendar atlas updates were still dirtying render because
the plain button texture setter took visual mutable borrows before checking for
real changes. Recorded the new no-op fast path in
`apply_set_button_texture_path()`, the focused atlas-backed button regression
test, the full-UI `GameTimeFrame_SetDate()` regression test, and refreshed the
`index.md` summary for the page.

## [2026-04-14] investigations | on-update dirty handler audit follow-up

Updated `investigations/on-update-dirty.md` with focused handler-audit results:
`LeaveInstanceGroupButton` now shows pure query/dispatch cost once its mutators
settle, while the remaining BuffFrame button cost comes from
`AuraButtonMixin:OnUpdate` doing duration formatting and font-threshold work on
every tick before the no-op setters bail out. Refreshed the `index.md` summary
for the page.

## [2026-04-14] investigations | on-update dirty solo compact raid manager follow-up

Updated `investigations/on-update-dirty.md` with the compact-raid follow-up:
`A_Admin.SetPartySize(0)` now fires `GROUP_ROSTER_UPDATE`, so solo transitions
hide `CompactRaidFrameManager` and remove `LeaveInstanceGroupButton` from the
visible `OnUpdate` handler set. Refreshed the `index.md` summary for the page.

## [2026-04-14] investigations | startup CreateFrame profiling ActionButtonTemplate regions

Updated `investigations/startup-createframe-profile.md` with the direct
`ActionButtonTemplate` layer/fontstring/button-texture fast path: the new
Rust-side region creation in `template/elements*.rs`, the focused regression
test that proves the hot path avoids Lua region fallback, and isolated
`WOW_SIM_PROFILE_CREATE_FRAME` numbers showing another `-27.36%` drop in
explicit template time across the profiled action-bar button families.

## [2026-04-14] investigations | startup CreateFrame profiling nested SpellFX follow-up

Updated `investigations/startup-createframe-profile.md` with the nested `ActionButtonSpellFXTemplate` follow-up: the remaining `ActionButtonInterruptTemplate` / `ActionButtonCastingAnimFrameTemplate` child creation fallback, the widened direct-child selector in `template/children.rs`, and new `WOW_SIM_PROFILE_CREATE_FRAME` numbers showing another `-28.6%` drop in explicit template time across action-bar button families.

## [2026-04-14] investigations | startup CreateFrame profiling MinimalScrollBar recursive fast path

Updated `investigations/startup-createframe-profile.md` with the `MinimalScrollBar` follow-up: the missed `Track -> Thumb` Lua `CreateFrame` fallback inside `apply_inline_frame_content()`, the recursive direct-child propagation change, the new focused regression test, and the smaller but measurable no-addons startup improvement after the fix.

## [2026-04-13] ingest | startup CreateFrame profiling

Created `investigations/startup-createframe-profile.md` to record runtime `CreateFrame` profiling results for Blizzard startup. Documented the new `WOW_SIM_PROFILE_CREATE_FRAME` instrumentation, the measured dominance of action-bar button template expansion (~4.1s across 34 runtime-created buttons), and the link to the planned pure-Rust template child creation work. Updated `index.md` with the new investigation page.

## [2026-04-13] ingest | world map preload API follow-up

Updated `investigations/world-map-texture-loading-budget.md` with the remaining explored-overlay delay root cause: Blizzard's `MapTexturePreloader.lua` was calling `C_Map.RequestPreloadMap()`, but the simulator stubbed that API as a no-op. Recorded the new queued preload path for map art + exploration overlays, the focused `request_preload_map_warms_map_art_and_overlay_textures` regression test, and refreshed the `index.md` summary for that page.

## [2026-04-13] ingest | chat frame scrollbar anchor reapply

Created `investigations/chatframe-scrollbar-anchor-reapply.md` to document the `ChatFrame1` scrollbar/edit-box layout bug. Recorded the real root cause in `reapply_inline_anchors()`: inherited child-frame anchors were resolving `$parent...` against the child name instead of the actual parent frame name, which broke `relativeTo="$parentBackground"` lookups and pushed the resize/scrollbar chain to screen-relative layout. Updated `index.md` with the new investigation page.

## [2026-04-13] ingest | world map texture loading budget follow-up

Updated `investigations/world-map-texture-loading-budget.md` with the second root cause behind the remaining world-map stalls: preload cleared `textures_pending` after CPU cache warmup even while the GPU atlas still lacked most tiles. Recorded the new `gpu_uploaded_textures`-based pending check, the focused `budgeted_preload` regression tests, and refreshed the `index.md` summary for that page.

## [2026-05-01] investigation | crafting cast bar

Created `investigations/crafting-cast-bar.md` to document the missing professions spellbar root cause. `C_TradeSkillUI.CraftRecipe` performed inventory changes but never started player casting or fired `UNIT_SPELLCAST_START`; successful crafts now populate `SimState.casting`, notify spellcast listeners, and emit `UPDATE_TRADESKILL_CAST_STOPPED` on completion. Updated `index.md` with the new investigation page.

## [2026-04-13] ingest | world map CreateTexture sublevel investigation

Created `investigations/world-map-create-texture-sublevel.md` to document the follow-up world-map open ordering churn: `CreateTexture(..., subLevel)` ignored its fourth argument, pooled textures started at sublevel 0, and Blizzard immediately repaired them with `SetDrawLayer()`. Recorded the new regressions for `CreateTexture(..., subLevel)` and no-op `SetDrawLayer()`, plus the traced repro where post-open `SetDrawLayer()` invalidations dropped from 150 to 0. Updated `index.md` with the new investigation page.

## [2026-04-13] ingest | world map voice chat alert investigation

## [2026-05-01] investigation | Journeys Midnight empty

Created `investigations/journeys-midnight-empty.md` to document the empty Journeys tab on Midnight. Root cause: current expansion constants were 11, but default major-faction data only seeded War Within expansion 10 rows, so Blizzard's `JourneysFrameMixin:Refresh()` received an empty `C_MajorFactions.GetMajorFactionIDs(11)` result. Updated `index.md` with the new investigation page.

Created `investigations/world-map-voice-chat-alerts.md` to document the reduced-stack world-map overlay where voice prompt frames appeared above the panel. Recorded the two harness prerequisites behind it: `Blizzard_Channels` needs `Blizzard_SocialToast` for `SocialToastTemplate hidden="true"`, and alert positioning needs the real chat-alert addons instead of the `ChatAlertFrame` stub. Updated `index.md` with the new investigation page.

## [2026-04-27] investigation | explicit XML parent anchors

Created `investigations/explicit-xml-parent-anchors.md` to document the PaperDoll sidebar tab positioning bug. Root cause: nested XML frame creation preferred the containing frame over an explicit child `parent="..."`, so implicit anchors resolved to `PaperDollFrame` instead of `CharacterFrameInsetRight`. Added the page to `index.md`.

## [2026-04-13] ingest | world map OnUpdate hover polling investigation

Created `investigations/world-map-onupdate-hover-polling.md` to document the post-texture-fix `UIParent_OnUpdate` cost: `FCF_OnUpdate` hover polling, the unnecessary mutable borrow in `IsMouseOver()`, the new immutable-borrow regression test, and the runtime repro where verbose OnUpdate logs stayed quiet after startup. Updated `index.md` with the new investigation page.

## [2026-05-02] investigation | Adventure Guide layout

Created `investigations/adventure-guide-layout.md` for the Suggested Content overlap bug. Root cause: `resolve_rect_if_dirty` fast-path geometry queries recomputed only the queried resized frame, leaving sibling frames anchored to it with stale cached rects. The fix now recomputes anchor dependents for direct dirty roots and dirty ancestor roots, and queues those moved dependents for hit-grid updates; regression coverage is `querying_resized_anchor_target_updates_dependent_siblings`.

Updated `investigations/adventure-guide-layout.md` with the follow-up Suggested Content text overlap root cause: tooltip `GetNumLines` overwrote the shared FrameRef method slot, so regular FontStrings returned zero lines. FontStrings now compute wrapped line count from measured text height while GameTooltip keeps tooltip-backed line counts; regression coverage is `fontstring_get_num_lines_reports_wrapped_line_count`.

## [2026-05-02] investigation | Adventure Guide SimpleHTML markup

Created `investigations/adventure-guide-simplehtml-markup.md` for the boss
overview text rendering raw `|c...|Hspell...|h...|r` and `|n` escapes. Root
cause: Encounter Journal uses `SimpleHTML`, whose stripped-text path only
removed HTML tags and bypassed WoW markup cleanup. Updated `index.md` with the
new investigation page.

## [2026-04-13] ingest | world map texture loading budget investigation

Created `investigations/world-map-texture-loading-budget.md` to document the post-rebuild-fix world-map stalls: hidden BC tile uploads, preload/draw source-cache mismatch, the new BC cache in `TextureManager`, and the smaller draw/tick texture budgets. Updated `index.md` with the new investigation page.

## [2026-05-03] investigation | LFD role icon slowness

Created `investigations/lfd-role-icon-slowness.md` for the LFD load pause around role selection icons. Root cause: role icons/backgrounds are button atlas crops from `Interface\lfgframe\uilfgprompts`, and crop extraction decoded the full 2048x2048 BLP before producing small sub-regions. Documented the persistent crop cache added in `TextureManager::load_sub_region` and updated `index.md`.

## [2026-04-13] ingest | world map frame-level rebuild investigation

## [2026-04-13] investigations | startup CreateFrame profiling follow-up

Updated `investigations/startup-createframe-profile.md` with section-level template profiling, the method-only XML script fast path, widened direct-child creation for `ActionButtonSpellFXTemplate` / `MinimalScrollBar`, and current shared-worktree startup numbers showing `36.79s -> 28.89s` on `--no-addons --no-saved-vars`.

Created `investigations/world-map-frame-level-rebuilds.md` to document the world-map performance bug where map pins repeatedly called `SetFrameLevel()` with the same value, forcing unnecessary `strata_buckets` invalidation and bucket rebuilds. Updated `index.md` with the new investigation page.

## [2026-04-29] investigation | LFD dungeon list empty

Created `investigations/lfd-dungeon-list-empty.md` for the Dungeons & Raids panel populating empty when "Specific Dungeons" was selected. Root causes: missing `GetLFDChoiceCollapseState`/`GetLFDChoiceEnabledState`/`GetLFGLockList` globals breaking `LFGDungeonList_Setup`; `LFG_UPDATE_RANDOM_INFO` never fired at startup so `LFDQueueFrame.Specific` stayed hidden and `OnShow=LFDQueueFrame_Update` never ran; `is_random=true` on the negative-id header in `default_lfd_dungeons` routed `GetRandomDungeonBestChoice` to `-1`. Updated `index.md` with the new investigation page.

## [2026-04-25] ingest | micro-menu atlas revert investigation

Created `investigations/micro-menu-atlas-revert.md` to document the micro-menu hover/leave icon disappearance root cause: button atlas setters populated child `tex_coords` but not `atlas_tex_coords`, so restored normal textures could miss the atlas-crop render path. Updated `index.md` with the new investigation page.

## [2026-05-02] update | Adventure Guide portrait masks and icons

Updated `investigations/adventure-guide-layout.md` with the follow-up Adventure Guide portrait bug: `Texture:SetMask` was still a no-op, so card icons did not clip to the gold portrait rings, and two seeded Adventure Journal icon paths did not resolve. Documented the mask wiring and manifest-backed icon replacements.

## [2026-05-28] ingest | Paladin aura stance bar

Created `investigations/paladin-aura-stance-bar.md` to document the root cause behind the missing Paladin aura bar: default Paladin state had zero shapeshift forms, so Blizzard `StanceBarMixin:Update()` hid the bar before rendering. Added the state-backed fix and regression coverage notes for the raw shapeshift globals and Blizzard StanceBar layer.

## [2026-05-08] update | Blizzard UI cache-only runtime

Simplified the Blizzard UI runtime model: `data/blizzard-ui-files.txt` is the committed file list, CASC is the extraction source, and `~/.cache/wow-ui-sim/blizzard-ui` is the only runtime Blizzard UI addon root. Removed the old runtime discovery language for `Interface/BlizzardUI`, `vendor/wow-ui-source`, and local `BlizzardInterfaceCode` fallback.

## [2026-05-08] update | Blizzard UI CASC source cache

Updated `systems/casc-asset-cache.md` and `reference/cli-commands.md` for `wow-cli casc sync-blizzard-ui`, the `~/.cache/wow-ui-sim/blizzard-ui` source cache, and GUI startup fallback behavior when the GitHub checkout is missing.

## [2026-04-29] ingest | dialog background DXT3 stripes

Created `investigations/dialog-background-dxt3-stripes.md` to document the escape-menu background stripe root cause: DXT3 BLPs were incorrectly mapped to BC3 on the raw compressed upload path. Updated `index.md` with the new investigation page.

## [2026-06-09] update | Mists 5.5.4 EditMode action-bar and UnitIsCivilian

Updated `investigations/editmode-layout.md` with the Mists 5.5.4 action-bar root cause: default managed action bars can remain at Blizzard's temporary `TOPLEFT UIParent` anchor when the manual EditMode layout pass aborts before clearing `layoutApplyInProgress`. Documented the Rust-side finalizer that clears the guard and replays `UpdateActionBarPositions()`, the modeled `UnitIsCivilian` fallback for Classic TargetFrame, and the ObjectiveTracker phantom duplicate root cause: bundled-addon startup exposed legacy `WatchFrame` alongside modern `ObjectiveTrackerFrame`.

## [2026-06-09] investigation | frame surrogate identity slot

Created `investigations/frame-surrogate-identity-slot.md` after replacing the simulator-only frame surrogate `[1]` dispatch path with a `[0]` identity token model. Updated `index.md` with the new investigation page.

## [2026-05-01] ingest | editbox render text cache investigation

Created `investigations/editbox-render-text-cache.md` to document the SimCommands search-box input bug: keyboard input updated `Frame.text` but left `text_stripped` stale after `SetText("")`, causing glyph rendering to shape an empty string. Updated `index.md` with the new investigation page.

## [2026-05-01] ingest | LFD checkbox and role-state follow-up

Updated `investigations/lfd-dungeon-list-empty.md` with the follow-up Group Finder bug where Specific Dungeons populated but dungeon selections were empty and `GetLFGRoles` was still a false stub, leaving "Join as Party" disabled. Documented the state-backed role and LFD checkbox fix plus regression coverage.

## [2026-05-01] update | Wardrobe filter click dispatch

Updated `investigations/appearances-wardrobe-api.md` with the follow-up Wardrobe filter bug where menu descriptions and callbacks were valid, but clicks could be swallowed by decorative child regions during GUI hit testing. Documented the fix: final mouse targets must be mouse-enabled frames, while decorative children only guide hit-test descent.

## [2026-05-01] update | Wardrobe class dropdown and set fallback

## [2026-05-18] investigation | ElvUI tooltip scale clipping

## [2026-06-08] ingest | ServerSnapshot action bar import

Created `systems/server-snapshot-action-bars.md` after adding startup import for action-bar spell slots captured by the ServerSnapshot addon. Updated `index.md` with the new systems page.

Updated `investigations/tooltip-double-shell.md` with the follow-up ElvUI tooltip clipping root cause. Tooltip frame bounds were scaled by ElvUI effective scale, but internal `GameTooltip` glyph emission was not; tooltip text now scales font size, line spacing, and text insets with the frame effective scale.

Updated `investigations/appearances-wardrobe-api.md` with the Wardrobe class dropdown casing/color contract and the `C_TransmogSets.GetBaseSets()` nil fallback stack overflow. Documented that class display names come from localized `className`, colors from uppercase `classFile`, and empty set surfaces must return tables.

## [2026-06-11] create | Class talent edge lines

Created `investigations/class-talents-edge-lines.md`. Talent connector lines were missing because `IsRectValid()` reported dirty-but-resolvable talent buttons as invalid, causing Blizzard's edge positioning to skip `Line:SetStartPoint()` / `SetEndPoint()`. A second render-list gate also filtered endpoint-positioned `Line` widgets and arrowhead textures under anchorless edge-frame parents. Fixed by resolving dirty rects inside `IsRectValid()` and allowing parent-independent line/anchor geometry through render-list filtering while preserving the ordinary unanchored-parent guard.

## [2026-04-12] ingest | transparent wrapper render-order investigation

Created `investigations/transparent-wrapper-render-order.md` for the world map / quest log render-order fix. Updated it after a follow-up regression to document the depth-aware transparent-wrapper hoist in `state_render.rs`, including both world-map visibility coverage (`world_map_tiles_render_after_tiled_background`) and world-quest pin ordering coverage.

## [2026-04-09] ingest | systems/ pages created (10 pages)

Created all 10 systems/ pages from source docs in docs/:

- systems/layout-system.md — from layout-system.md + anchor-resolution.md
- systems/rendering-pipeline.md — from rendering-pipeline.md
- systems/widget-system.md — from widget-system.md + button-text-rendering.md
- systems/lua-api.md — from lua-api.md
- systems/event-system.md — from event-system.md
- systems/xml-template-system.md — from xml-template-system.md
- systems/addon-loading.md — from addon-loading-pipeline.md
- systems/texture-atlas.md — from texture-atlas-system.md
- systems/frame-data-flow.md — from frame-data-flow.md
- systems/taint-system.md — from protected-frame-enforcement.md + src/lua_api/frame/methods/methods_helpers.rs + src/lua_api/globals/security.rs + tests/protected_frame_enforcement.rs + tests/secure_handler_fallback.rs + tests/security_api.rs

Updated index.md systems/ table.

## [2026-05-28] ingest | Mists WorldMap startup failure cluster

Created `investigations/mists-world-map-startup.md` after fixing Mists startup errors around `WorldMapFrame`, `WorldMapTrackQuest`, `UpdateUIPanelPositions`, `FogOfWarFrameMixin`, and MapCanvas provider `OnAdded` defaults. Updated `index.md` with the new investigation page.

## [2026-04-10] ingest | Initial bulk ingest from 30+ existing docs

Bootstrapped wiki from root-level documentation files. Created pages across systems/, design/, investigations/, and reference/ categories.

## [2026-04-09] ingest | design/ and reference/ pages created

Created 7 pages from DESIGN.md, SCALING.md, docs/debug-tools.md, FUTURE.md, docs/c-api-signature-audit.md, docs/c-api-stub-audit.md, AGENTS.md, and PLAN.md.

Pages created:
- design/architecture-overview.md
- design/scaling-coordinates.md
- design/debug-tools.md
- reference/api-coverage.md
- reference/cli-commands.md
- reference/addon-compatibility.md
- reference/development-phases.md
## [2026-05-16] ingest | addon startup Settings and item-load investigation

Created `investigations/addon-startup-settings-and-item-load.md` to capture the root causes behind addon startup errors: registered Settings canvases must start hidden, forbidden attribute delegates need secure dispatch, item subclasses must return enUS keyword-compatible names or nil, and positive live item IDs need synthetic placeholder item info so item-load callbacks terminate.

## [2026-06-19] create | Retail Store secure pool constructor mismatch

Created `investigations/store-secure-pool-constructors.md` after fixing the retail Store blank/red card state. Root cause: Store code runs in `__secureenv`, whose `CreateFramePoolCollection` still pointed at the simulator fallback after `Blizzard_SharedXMLBase` installed Blizzard's proxy-backed constructor in `_G`. The fix syncs real pool/factory constructors into `__secureenv` after SharedXMLBase loads and pins the behavior with Store tree, pool surface, and text-cache corruption tests.

## [2026-05-26] update | C API temporary shim module retired

Updated `reference/api-coverage.md` after moving the last `src/c_api/temporary_shims` surface (`C_TransmogOutfitInfo` slot/outfit defaults) into `src/lua_api/workarounds/temporary/` and deleting the empty C API temporary-shim module. Temporary unmodeled `C_*` defaults now belong in Lua workaround modules; `src/c_api/permanent_shims/` remains only for intentional unsupported domains.

## [2026-05-23] update | transmog sets shim boundary

Updated `investigations/appearances-wardrobe-api.md` after moving `C_TransmogSets` empty/default set APIs out of `runtime_surface_bootstrap.lua` and into `src/c_api/temporary_shims/c_transmog_sets.rs`. The investigation still records the same wardrobe contract: `GetBaseSets()` and related set APIs must return empty tables rather than nil until real set inventory state exists.

## [2026-05-29] ingest | Mists Syndicator and Baganator startup cleanup

Created `investigations/mists-syndicator-baganator-startup.md` after fixing full-profile Mists startup errors. The investigation records the Mists-gated item taxonomy overrides needed by Syndicator and the minimal CharacterFrame/TokenUI bootstrap path needed by Baganator.

## [2026-06-10] update | class talent config-scoped visibility

Updated `investigations/class-talents-trait-loadout-state.md` after fixing a full-addon/SavedVariables talent panel regression where `C_Traits.GetNodeInfo(configID, nodeID)` discarded `configID` and evaluated Protection nodes against a stale view spec.

## [2026-06-11] ingest | CASC root v2 misparse dropped 89% of fdids

Created `investigations/casc-root-v2-parsing-missing-textures.md` after the magic-dispel debuff border atlas resolved correctly but its texture (`interface/hud/uidebuffframes.blp`) could not be extracted. cascette-rs 0d0e79a misparsed 12.0.5 TSFM v2 root blocks (split content-flags fields, wrong NoNameHash bit), silently dropping 2.8M of 3.19M root records. Fixed by pinning cascette-rs c5de2b9 / asset-resolver 3ab8a14 (wow-ui-sim 598a29909), rebuilding the resolution cache (346K → 1.88M entries), and clearing 904 stale `.missing` markers.

## [2026-06-11] update | EditMode first-apply dirtiness + player debuff modeling

Updated `investigations/casc-root-v2-parsing-missing-textures.md` with parts 2-3: the dispel swirly was also blocked by the EditMode startup workaround seeding frames before UpdateSystem (destroying first-apply setting dirtiness, so ShowDispelType never applied), and by the absence of any player-debuff modeling. Fixed in c527f05fa (lookup-then-UpdateSystem flow mirroring EditModeManagerFrameMixin) and 2ac3057b6 (A_Admin.AddDebuff + UNIT_AURA isFullUpdate payloads + nilable dispelName).

## [2026-06-11] create | XML scale attribute ignored

Created `investigations/xml-scale-attribute.md`. The hero talents box didn't encompass its node buttons: `FrameXml` had no `@scale` field, so all 127 `scale="..."` attributes in Blizzard XML were silently dropped — including `HeroTalentsTreeNodesContainerTemplate`'s `scale="0.85"`, whose absence left only 212 of the needed 272 local units inside the fixed 284×362 backplate. Fixed in 91835d898 by parsing `@scale` and applying it through the template chain in both the XML loader and runtime CreateFrame paths, mirroring alpha.

## [2026-06-11] ingest | Journeys renown card text anchor fallback

Created `investigations/journeys-renown-card-text-anchor.md`. Reported as text z-ordering, but the Journeys "Renowns" card name/level FontStrings were anchored to the wrong target: their `relativeKey="$parent.IconFrame"` failed eager resolution (loader creates Layers before child Frames) and SetPoint silently fell back to the parent card, pushing the text under the adjacent column. Fixed in 7c0c0f987 by storing unresolved $parent key expressions on the anchor for the existing post-children lazy resolution pass.

## [2026-06-11] ingest | Mouse dead at frozen 50 FPS (probe blockers + idle tick stall)

Created `investigations/mouse-dead-probe-blockers-idle-ticks.md`. CoreBehaviorProbe (loaded from a renamed `.disabled` folder) left two full-screen mouse-enabled DIALOG blockers over UIParent because its 3-deep C_Timer.After cleanup chain stalls: pending C_Timers do not wake the tick loop once the app idles (open bug), and the frozen tick loop also freezes the FPS display at its last value. Loader fixed in b580ea005 to only accept TOCs naming their folder.

## [2026-06-11] update | tick-subscription churn root cause fixed

Updated `investigations/mouse-dead-probe-blockers-idle-ticks.md`: the idle timer stall was subscription churn — compute_tick_interval returned the raw shrinking remaining-time of the next C_Timer, changing the iced time::every identity on every update, so continuous input (mouse moves) recreated the tick stream before it could fire. Fixed in 397742569 with quantized interval buckets.

## [2026-06-12] create | DISPLAY_SIZE_CHANGED / UI_SCALE_CHANGED firing conditions

Created `investigations/display-size-ui-scale-events.md`. A live retail probe (`docs/addons/ScaleEventProbe`, 12.0.5.67823) disproved the sim's "resize never fires UI_SCALE_CHANGED" assumption: retail fires the two events as an ordered pair (display-first) on every display/scale recalculation — drag resize emits repeated ordered pairs during the drag, maximize/restore and resolution/fullscreen transitions can emit double-pairs even when dimensions are unchanged, scale slider and useUiScale changes also emit the pair, and startup fires the pair twice pre-PLAYER_LOGIN. Events fire before CVAR_UPDATE with GetEffectiveScale() already updated. Fixed `set_screen_size` to fire the pair and inverted the resize regression test to assert order. Startup ordering (sim fires post-login) and no-op dedupe remain divergent.

## [2026-06-12] update | Startup display/scale event ordering matched to retail

Updated `investigations/display-size-ui-scale-events.md`. Moved the `DISPLAY_SIZE_CHANGED`/`UI_SCALE_CHANGED` pair from `fire_post_login_events` into `fire_login_sequence` (after `VARIABLES_LOADED`, before `PLAYER_LOGIN`) — retail fires both pairs pre-login and none post-login. With the `set_screen_size` pair during GUI canvas startup this yields two pre-login pairs, matching the probe capture. Same-size window transitions reclassified as an observability limit (iced exposes no OS window-state signal). Verified: lib failure set unchanged, `lua-errors` clean with and without addons; new regression test pins pair-before-login ordering.

## [2026-06-12] update | hit-grid ordered insertion (the core mouse-freeze fix)

Updated `investigations/mouse-dead-probe-blockers-idle-ticks.md` with cause 4: full hit-grid rebuilds per hover transition (180-470ms each) saturated the main thread; the rebuilds were themselves a workaround for append-only HitGrid::insert breaking render order. Fixed with per-frame render-order keys + binary insertion, producer coverage for level/strata/raise changes, hover-time coalescing, and dev opt-level 1 (a7b131f65, telemetry 0c2668a3a).

## [2026-06-12] create | Mount Journal clicks never switched selection

Created `investigations/mount-journal-click-selection.md`. Real mouse clicks on mount rows fired OnMouseDown/OnMouseUp but never OnClick while `row:Click()` worked, because the startup-XML fast path's `parse_single_string_literal` stripped only the outer quotes — the generic MethodWithStringArg parser fused `RegisterForClicks("LeftButtonUp", "RightButtonUp")` into one garbage registration entry that no click edge can match. Fixed by rejecting interior quotes so multi-arg calls fall through to the dedicated parser. Added a `headless-click-probe mounts` panel (dotted-path frame resolution for anonymous ScrollBox rows + post-click `verify_lua`) and a `WOW_SIM_DEBUG_CLICK_DISPATCH=1` dispatch trace.

## [2026-06-12] create | Micro menu clicks missed from stale quadrant anchor

Created `investigations/micro-menu-click-offset.md`. GUI clicks on micro-menu buttons (LFD/Group Finder) did nothing: MicroMenuMixin:Layout anchors the menu inside MicroMenuContainer by screen quadrant, but the sim ran it before saved anchors and the real window size were applied, so the first press snapped the whole bar one QueueStatusButton slot (~46.5px) between mouse-down and mouse-up and the same-frame guard skipped OnClick. Fixed by replaying Blizzard's `InvokeOnAnyEditModeSystemAnchorChanged(force)` at the end of init_edit_mode_layout and from set_screen_size after the DISPLAY_SIZE_CHANGED/UI_SCALE_CHANGED pair. Added `headless-click-probe micromenu` regression panel.

## [2026-06-19] create | Post-load workaround audit

Created `investigations/post-load-workaround-audit.md` while auditing retail post-load hooks. The page records duplicate loader-side hooks retired for AccountStore, MapCanvas, and FrameXMLUtil, and classifies the remaining sampled hooks with current temporary rationale plus retirement paths.

## [2026-06-20] update | Third-party addon metadata pre-registration

Updated `systems/addon-loading.md` after fixing `!BugGrabber`'s false no-display warning when `BugSack` is present. The system page now records the invariant that discovered third-party addon metadata must be registered in `C_AddOns` before eager third-party Lua executes, while actual file loading still follows enabled, non-`LoadOnDemand`, dependency-sorted order.

## [2026-06-21] update | secureenv no-fallback retail probe

Updated `systems/taint-system.md` and `investigations/store-secure-pool-constructors.md` after a retail PrivateAurasUI cooldown-wrapper probe showed secure code does not hit late `_G` overrides. The simulator now models secureenv as a separate shallow copy without `__index = _G`; `Blizzard_SharedXMLBase` Lua is replayed into secureenv to populate shared secure symbols directly instead of copying constructors from `_G` after load.

## [2026-06-30] update | Initial TOC `[Bootstrap]` support

Added parser and loader support for TOC entries annotated with `[Bootstrap]`, motivated by LoD bootstrap glue such as `Blizzard_CooldownBroadcaster_Bootstrap.lua`. This was later corrected on 2026-07-01 after a live-client third-party probe showed `[Bootstrap]` does not reorder TOC files.

## [2026-07-01] correction | `[Bootstrap]` preserves TOC order

Updated `systems/addon-loading.md` after live-client probes showed `[Bootstrap]` is not a separate pass and must not move files out of TOC order. `TocFile` now keeps annotated files in `files` and records a per-file bootstrap flag. Startup loads full TOCs for non-LoD addons and only annotated bootstrap files for LoD addons, preserving addon order; runtime `LoadAddOn` skips already-executed bootstrap files and a self `LoadAddOn(thisAddon)` call from bootstrap remains a benign reentrancy no-op.

## [2026-08-08] investigation | Prove remaining 12.0.0 removed runtime APIs

Classified exactly 19 removed runtime API rows as `best-effort`/`behavioral` using the committed full-LoD namespace-safe rawget batch `patch-tests/patch_12_1/strict_removals.rs::removed_remaining_runtime_apis_are_absent_after_full_lod_load` at `ec9ffbc0b`. Three obsolete simulator publications were removed (`C_CatalogShop.OpenCatalogShopInteraction`, `C_PlayerInfo.IsExpansionLandingPageUnlockedForPlayer`, and `C_StorePublic.IsDisabledByParentalControls`); the other 16 rows were already absent. Source scanning is auxiliary, no replacement behavior is inferred, and the 143/453/1/2813 totals are current.
