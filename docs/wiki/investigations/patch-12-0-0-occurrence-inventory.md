# Patch 12.0.0 Occurrence Inventory
Occurrence-level register derived from explicit wowless retail snapshots. 885 rows carry evidence-backed best-effort classifications, including 56 deprecated-wrapper vendor-present rows, five chat/spell vendor-present wrappers, seven simulator compat rows, three StatusBar interpolation rows, three Cooldown rows (`GetCountdownFontString`, `SetCooldownFromDurationObject`, and `SetCooldownFromExpirationTime`), four GameTooltip layout rows (`GetMinimumWidth`, `SetMinimumWidth`, `GetPadding`, and `SetPadding`), one GameTooltip content row (`SetText`), two GameTooltip line rows (`GetLeftLine` and `GetRightLine`), one TextureBase reset row (`ResetTexCoord`), two TextureBase atlas rows (`GetTexCoord` and `SetAtlas`), one StatusBar value/clamping row (`SetMinMaxValues`), one StatusBar value row (`SetValue`), and two Frame event callback rows (`RegisterEventCallback` and `RegisterUnitEventCallback`), plus 32 CAA constant publication/value rows, 90 HousingResult enum publication/value rows, 98 account-state enum publication/value rows, and 43 ItemCollectionType/SecretAspect enum publication/value rows, plus 52 transmog outfit enum publication/value rows, 25 TransmogSituation enum publication/value rows, and 35 CombatLogObject enum publication/value rows, plus 47 AccountTransType, CraftingReagentItemFlag, CurrencyDestroyReason, CurrencySource, EditModeAuraFrameSystemIndices, EditModeCooldownViewerSetting, GameRule, HousingDecorActionFlags, HousingItemToastType, MapIconUIWidgetSetType, SurveyDeliveryMoment, TooltipDataType, TraitNodeFlag, UICursorType, and UIWidgetVisualizationType enum publication/value rows, five GossipNpcOption, ItemRecraftFlags, PerksVendorCategoryType, PlayerInteractionType, and QuestTagType replacement-key publication/value rows, 20 CooldownViewerAddAlertStatus, CooldownViewerAlertEventType, DamageMeterStyle, EditModeEncounterEventsSystemIndices, and HouseExteriorWMODataFlags member rows and 15 matching metadata rows—including the epoch-corrected four-member CooldownViewerAlertEventType table—and four epoch-specific EncounterEventFlags rows, plus 21 UnitDamageAbsorbClampMode, UnitHealAbsorbClampMode, UnitHealAbsorbMode, and UnitIncomingHealClampMode member/metadata rows and nine UnitAuraSortRule member/metadata rows plus `VasTransactionPurchaseResult.DbHouseOwnerRestriction=20096`; 16 removed `Constants.HousingCatalogConsts` rows and 94 removed account-state enum alias rows and five same-value renamed enum aliases are evidence-required/unsafe because bootstrap omission while retaining the namespace does not prove full runtime or dynamic publication, historical load order, replacement semantics, or exact 12.0.0 removal; five script-object rows are best-effort proxy/state classifications and two script-object rows are evidence-required/unsafe; 21 typedef rows and 33 structure declarations are provenance-only best-effort source metadata with no runtime behavior claimed; the nine remaining behavior-linked or removal-sensitive structure declarations are evidence-required/unsafe because source metadata or method absence does not prove their exact runtime contracts or removal/replacement identity; the UnitHealPredictionCalculator luaobject-method slice adds one best-effort getter and 13 evidence-required/unsafe rows, and all 68 luaobject-method rows are now non-untriaged; `StatusBar.GetFillStyle`, `StatusBar.SetFillStyle`, and `TextureBase.SetSpriteSheetCell` are evidence-required/unsafe because current behavior is constant/round-trip-broken or a no-op and exact contracts remain unproven; 622 rows carry evidence-required unsafe classifications, including the two removed legacy cursor globals `CombatLogAdvanceEntry` and `CombatLogSetCurrentEntry`, whose pinned retail/PTR deprecated addons do not publish them while the temporary simulator model retains fixture-only Wrath/Mists caller compatibility; two rows are already-decided permanent-scope exceptions, including Model.SetUseGBuffer under the permanent no-3D scope; the bounded C_UnitAurasPrivate slice classifies all 10 added/changed secure-only API rows as evidence-required/unsafe using the permissive/partial `src/lua_api/workarounds/temporary/private_aura_state.rs` model; secure enforcement, private-aura visibility, callback/anchor lifecycle, callback payloads, and the two absent APIs remain unproven, so tests remain empty and no approval can close these rows. The bounded C_SpellDiminish slice adds 14 evidence-required unsafe rows whose static eight-category fixture and local tests do not establish authoritative 12.0.0 category contents, ruleset tracking semantics, or tracker payload fields; the remaining 2022 rows are neutral pending evidence-backed classification, including the nine-row `C_ActionBar` charge/cooldown structure-field slice, classified as evidence-required/unsafe. The bounded C_ActionBar slice covers 17 best-effort behavioral rows and 25 evidence-required unsafe rows; four newly best-effort claims are limited to exact tested seeded/empty/malformed profession quality, modeled action-slot presence/texture, and modeled outfit-lock slot behavior, while 22 unresolved action queries/registration rows remain evidence-required with no approval path. The bounded C_TradeSkillUI slice classifies GetDependentReagents as best-effort behavioral only for table return/iteration safety and nil/malformed/unknown-reagent behavior; exact retail dependency semantics remain unproven. Its eleven quality/recraft/reagent-link rows remain evidence-required unsafe: current evidence distinguishes absent methods, placeholder empty-table/true behavior, and unproven removal behavior; authoritative profession semantics or a correct model/test are required and no approval can close them. The twelve-row C_Spell slice covers duration-object lifecycle, spell metadata/display, and boolean contracts without focused proof; authoritative evidence or correct models are required, and no approval can close these rows. The nine curve-family best-effort rows are limited to tested factory/table shape, scalar interpolation/copy behavior, and color-object/copy shape; 23 unresolved curve contracts remain evidence-required and cannot be approved closed. The C_StringUtil slice contains one best-effort behavioral row limited to quoted-code pipe escaping for tested plain/color-code cases and eight evidence-required unsafe rows because the current model does not publish them; authoritative semantics or correct implementations are required and no approval can close them. The bounded C_Timer slice classifies NewTimer/NewTicker as best-effort behavioral, limited to function/container acceptance, returned container identity/proxy equality, cancellation, and independent ticker counts; exact scheduling, lifecycle, GC, and edge semantics remain unproven. C_Timer.After is evidence-required unsafe because its only focused-looking test is ignored; callback/lifecycle semantics require a correct modeled implementation and executable behavioral proof, and no approval can close it. The bounded C_ColorUtil slice adds two best-effort behavioral rows limited to the tested RGB-to-ffRRGGBB code and explicit color-code text wrapping; five conversion/wrapper rows remain evidence-required unsafe because current behavior is absent, placeholder/identity/max-channel, or lacks focused proof; edge/secret/localization/clamping semantics remain unproven. The bounded C_DamageMeter slice classifies exactly 19 best-effort behavioral rows, limited to exact seeded/empty/shape/type/lookup assertions; 10 seeded-but-unasserted/reset rows remain evidence-required unsafe with no approval path, six structure declarations are provenance-only source metadata with no payload behavior claimed, and no complete retail aggregation/lifecycle/secret fidelity is claimed. The bounded C_CombatAudioAlert slice classifies exactly 12 added API rows as evidence-required/unsafe using checked-in source-register evidence and the examined current `src/lua_api/globals/register.rs` surface; tests remain empty with null commit, approval, and scope exception. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close these rows. The bounded `C_EncounterWarnings` slice classifies exactly 19 added structure/API rows as evidence-required/unsafe using checked-in source-register evidence and the clean current `src/lua_api/globals/missing_surface/encounter_warnings.rs`; tests remain empty with null commit, approval, and scope exception. `GetEditModeWarningInfo`/current structure fields are limited to fabricated preview/static payload behavior, `PlaySound` is a no-op, and `GetSoundKitForSeverity`, `IsFeatureAvailable`, and `IsFeatureEnabled` have no examined registration; exact encounter state, payload field meanings, feature availability/enabling, severity sound mapping, and audio playback require authoritative evidence or a correct modeled subsystem/test, and no approval can close these rows. The bounded `C_Secrets` slice classifies exactly 23 added API rows as evidence-required/unsafe using checked-in source-register evidence and the examined current `src/lua_api/globals/register.rs` surface; tests remain empty with null commit, approval, and scope exception. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. The bounded `C_PingSecure` slice classifies exactly 15 changed API rows as evidence-required/unsafe using checked-in source-register evidence and clean current `src/c_api/c_ping_secure.rs`; tests remain empty with null commit, approval, and scope exception. The source contract is secure-only while current behavior is no-op/inert callback storage/partial or absent; exact secure-call enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and `PingResult` semantics require authoritative live evidence or a correct ping/security model and direct tests, and no approval can close these rows. The bounded `C_EncounterTimeline` slice classifies exactly 55 added API, structure, and structure-field rows as evidence-required/unsafe using checked-in source-register evidence and the current `src/lua_api/workarounds/temporary/encounter_state.rs` partial seeded fixture; tests remain empty with null commit, approval, and scope exception. The nine present APIs are fixture-backed and 46 rows are absent; exact encounter state, script-event lifecycle, timers, feature flags, payload values, and structure-field semantics require authoritative live evidence or a correct modeled subsystem with focused tests, and no approval can close these rows. The bounded `C_TransmogOutfitInfo` slice classifies exactly 115 added API, structure, and structure-field rows as evidence-required/unsafe using checked-in source-register evidence and `src/lua_api/globals/transmog_outfit_info.rs`; the two present lock queries use only local state behavior, the other 113 rows are unmodeled, and local tests do not establish authoritative retail 12.0.0 semantics. Authoritative live evidence or a correct modeled transmog-outfit subsystem with focused tests is required, and no approval can close these rows. The bounded `C_NamePlate` slice classifies six added 2D size/hit-test/state APIs as evidence-required/unsafe because the current permanent shim has no modeled nameplate-manager state and exact 12.0.0 semantics remain unproven. `C_NamePlateManager.IsNamePlateUnitBehindCamera` is exception-requested/impossible under the already-decided permanent no-3D project scope; this is not a user approval request. The 19 removed C_NamePlate rows are best-effort/behavioral only for current full-LoD rawget publication absence and retained-method callability; source scanning is auxiliary, with no historical load-order or broad scanner claim. The bounded `C_TransmogCollection` slice classifies exactly 23 added/changed custom-set and appearance-source rows as evidence-required/unsafe using checked-in source-register evidence, the seeded/partial `src/lua_api/globals/missing_surface/transmog_collection.rs` surface, and `src/lua_api/state_types/collections.rs`; all 10 removed outfit rows are best-effort/behavioral only for the committed full-LoD rawget publication-absence and retained-appearance-method callability proof. The three obsolete simulator outfit placeholders and their tests were removed; custom-set lifecycle, hyperlink, persistence, validation, and exact `TransmogAppearanceSourceInfoData` semantics remain unproven, and this removal proof does not claim custom-set replacement APIs are implemented, a clean replacement surface, historical load-order timing, or broad source-scanner completeness. Related local tests do not establish authoritative retail 12.0.0 behavior for the 23 added/changed rows. The source covers wowless schema provenance, not historical FrameXML or live runtime behavior; the audit remains open.
The bounded 12.0.0 miscellaneous payload-field slice classifies eight ExpansionDisplayInfo, LuaColorCurvePoint, PrivateAuraIconInfo, and SpellCooldownInfo fields as evidence-required/unsafe. Current behavior is absent, nil-only, generic, or placeholder-backed and does not establish exact contracts, state/security, or consumer semantics.
The bounded 12.0.0 unit/heal-prediction slice classifies four added rows as best-effort/behavioral from focused tests; full retail semantics remain unclaimed.
The bounded 23-row 12.0.0 CVar-default slice classifies the added encounter-warning, WorldText, chat-restriction, rune-icon, auction-sort, chat-bubble, combat-warning, damage-meter, and suggested-level-filter CVars as best-effort/behavioral using `test_patch_12_0_0_cvar_defaults`. The focused test proves only startup `GetCVar`/`GetCVarDefault` exact string defaults; mutation, events, persistence, secure/read-only flags, consumers, and later-epoch behavior remain unclaimed.
The bounded three-row `C_CooldownViewer` gap classifies `CooldownViewerCooldown.category`, `CooldownViewerCooldown.cooldownID`, and `GetValidAlertTypes` as evidence-required/unsafe because the current temporary surface returns nil/empty defaults and has no typed cooldown producer, category/ID records, ordered alert-type arrays, validation, routing, Settings UI behavior, or lifecycle.

The bounded eight-row `C_EventScheduler` slice classifies `C_EventScheduler.CanShowEvents` as best-effort/behavioral only for simulator visibility derivation from explicit override, suppression, event-list state, and request repopulation; retail availability, refresh timing, persistence, full lifecycle, and edge semantics remain unclaimed. `EventDisplayInfo.hideDescription`, `EventDisplayInfo.hideTimeLeft`, `EventDisplayInfo.overrideAtlas`, `EventDisplayInfo.overrideTooltipWidgetSetID`, `OngoingEventInfo.displayInfo`, `ScheduledEventInfo.displayInfo`, and `ScheduledEventInfo.eventID` are evidence-required/unsafe because temporary empty/seeded compatibility payloads do not establish documented typed field values, shape, producers, or lifecycle.

The bounded four-row API slice classifies `C_EventUtils.IsCallbackEvent` and `C_HouseExterior.GetCurrentHouseExteriorType` as best-effort/behavioral from focused positive/negative callback-event and seeded two-return exterior-type tests. Exact callback registry completeness, argument validation, housing mutation, persistence, refresh, and lifecycle semantics remain unclaimed. `C_CreatureInfo.GetCreatureID` and `C_GameRules.IsPersonalResourceDisplayEnabled` are evidence-required/unsafe because the first is absent without a creature identity model and the second falls through to a nil-returning temporary function instead of its required boolean contract.

The bounded four-row Delves/housing gap classifies `C_DelvesUI.GetLockedTextForCompanion`, `C_HouseExterior.GetFixtureDebugInfoForGUID`, `C_Housing.IsHousingMarketShopEnabled`, and `C_HousingBasicMode.IsFreePlaceEnabled` as evidence-required/unsafe. The first three are absent without their required companion-lock, fixture-debug, or market-shop backing systems; the fixture-debug source also lacks a retained signature. `IsFreePlaceEnabled` hardcodes true while its setter is a no-op, so state transitions and exact semantics are not modeled.

The bounded three-row Delves/housing slice classifies `C_HouseExterior.GetHouseExteriorSizeOptions` as best-effort/behavioral only for its tested seeded selectedSize 3 and Medium/3 plus Large/4 option table. Exact option metadata, enum fidelity, mutation, persistence, refresh, validation, and lifecycle remain unclaimed. `C_DelvesUI.IsTraitTreeForCompanion` and `C_Housing.OnHouseFinderClickPlot` are evidence-required/unsafe because both are absent without trait-tree ownership or selected-plot interaction models.

The bounded two-row exterior-type/debug slice classifies `C_HouseExterior.GetHouseExteriorTypeOptions` as best-effort/behavioral only for its tested selectedExteriorType 1 and Sunspire Cottage/1 plus Sunspire Manor/2 option table. Exact option metadata, selection mutation, persistence, refresh, validation, and lifecycle remain unclaimed. `C_HouseExterior.GetHoveredFixtureDebugInfo` is evidence-required/unsafe because the retained source has no signature and the temporary nil fallback has no hovered-fixture debug payload or state.

The bounded three-row exterior mutation/debug gap classifies `C_HouseExterior.GetSelectedFixtureDebugInfo`, `C_HouseExterior.SetHouseExteriorSize`, and `C_HouseExterior.SetHouseExteriorType` as evidence-required/unsafe. The selected-debug API is absent with no retained signature or payload model; both setters are no-ops that do not update getter-visible state, validate values, resolve names, persist, refresh, reset, or model lifecycle behavior.

The bounded three-row housing placement/cart gap classifies `C_HousingBasicMode.SetFreePlaceEnabled`, `C_HousingBasicMode.StartPlacingPreviewDecor`, and `C_HousingCatalog.DeletePreviewCartDecor` as evidence-required/unsafe. All three are no-ops without mutable free-placement state, preview-placement/decor/bundle state, or observable cart deletion; validation, repeated calls, requests/events, persistence, reset/isolation, and lifecycle remain unmodeled.

The bounded four-row housing catalog slice classifies `C_HousingCatalog.GetBundleInfo` and `C_HousingCatalog.GetCartSizeLimit` as best-effort/behavioral only for tested seeded bundle lookup/viewed-state mutation and callable one-number value 20. Exact retail bundle schema, unknown-ID/clone behavior, pricing, dynamic enforcement, validation, persistence, refresh, consumers, and lifecycle remain unclaimed. `GetCatalogEntryRefundTimeStampByRecordID` and `HasFeaturedEntries` are evidence-required/unsafe because the former always returns nil without keyed refund state and the latter hardcodes true instead of deriving from catalog contents.

The bounded two-field `HousingBundleInfo` slice classifies `canPreview` as best-effort/behavioral only for tested seeded bundle 5001 boolean true publication; dynamic preview eligibility, other bundles, mutation, validation, persistence, refresh, and lifecycle remain unclaimed. `originalPrice` is evidence-required/unsafe because the fixture publishes nil only and has no modeled numeric original price, discount, or currency semantics.

The bounded three-field `HousingCatalogEntryInfo` slice classifies `isUniqueTrophy` and `itemID` as best-effort/behavioral only for tested seeded entry 1001 boolean false and numeric 1001 publication. Exact trophy classification, nullable/missing item IDs, other entries, authoritative item data, validation, mutation, persistence, refresh, and lifecycle remain unclaimed. `dyeIDs` is evidence-required/unsafe because catalog payloads omit the required numeric array and have no dye-ID state.

The bounded eleven-field `HousingPreviewItemData` slice classifies `bundleCatalogShopProductID`, `decorGUID`, `decorID`, `icon`, `id`, `isBundleChild`, `isBundleParent`, `name`, `price`, `productID`, and `salePrice` as evidence-required/unsafe. No typed preview-item payload producer exists; related catalog/decor/bundle fixture fields do not establish these specific fields, nullability, identities, relationships, pricing, validation, event/preview-list production, refresh, persistence, or lifecycle.

The bounded four-row preview-cart/refund slice classifies `C_HousingCatalog.IsPreviewCartItemShown`, `PromotePreviewDecor`, and `SetPreviewCartItemShown` as best-effort/behavioral only for tested in-memory unknown=false, boolean setter round-trip, successful promotion, and shown-state mutation. GUID/decor validation, failures, repeated calls, events/refresh, persistence, and lifecycle remain unclaimed. `RequestHousingMarketRefundInfo` is evidence-required/unsafe because the no-op has no refund request/list state or update-event lifecycle.

The bounded six-row housing preview-state slice classifies `C_HousingDecor.EnterPreviewState`, `ExitPreviewState`, `GetNumPreviewDecor`, and `IsPreviewState` as best-effort/behavioral only for tested temporary false/0 → true/1 → false/0 transitions. Actual preview-decor cardinality, placement/cleanup, events, persistence, refresh, and retail lifecycle remain unclaimed. `C_HousingCustomizeMode.IsHouseExteriorDoorHovered` and `C_HousingDecor.IsModeDisabledForPreviewState` are evidence-required/unsafe because exterior-door hover state is absent and mode-disabled behavior is constant false without mode-aware state.

The bounded 33-row 12.0.0 CAA CVar-default slice classifies the existing 27 CAA CVars plus six additional target-health, voice, and volume CAA CVars as best-effort/behavioral using `test_patch_12_0_0_cvar_defaults`. The focused test proves only startup `GetCVar`/`GetCVarDefault` exact string defaults; CAA behavior, UI/audio effects, mutation, persistence, events, flags, consumers, and later-epoch semantics remain unclaimed.

The bounded six-row `C_AdventureMap.GetQuestPortraitInfo` slice classifies the API plus five portrait fields as best-effort/behavioral from focused `tests/c_adventure_map/quests.rs` proof. Claims cover injected-state lookup, typed five-field publication, unknown/nonnumeric zero-value returns, nullable `modelSceneID`, and tested display-ID gating. Retail data population, localization, full validation/edge behavior, assets/rendering, and lifecycle remain unclaimed. `modelSceneID` is treated strictly as data under the existing permanent no-3D scope; no 3D implementation or new exception is requested.

The bounded nine-row `C_ActionBar` charge/cooldown structure-field slice classifies all `ActionBarChargeInfo`/`ActionBarCooldownInfo` field rows as evidence-required/unsafe. `GetActionCharges` returns static placeholder charge fields; `GetActionCooldown` returns a partial table with fixture-backed start/duration and constant enabled/mod-rate. Neither establishes authoritative typed payload fidelity, slot-dependent charges, field relationships, invalid-slot behavior, progression, secrets, or lifecycle; no approval can close these rows.

The bounded 12.0.0 number-abbreviation field slice classifies eight `NumberAbbrevData`/`NumberAbbrevOptions` fields as evidence-required/unsafe. Generic `AbbreviateConfig` proxy round-tripping does not establish typed contracts, defaults/nullability, validation, ordering, or formatting behavior.
The bounded three-row global-utility slice classifies `AbbreviateLargeNumbers` and `AbbreviateNumbers` as evidence-required/unsafe because temporary fallbacks ignore `NumberAbbrevOptions` and do not model abbreviation, localization, or validation. `AddSourceLocationExclude` is best-effort/behavioral only for nil-guarded global publication and successful string-argument no-op invocation through `installs_debug_environment_defaults`; exclusion, filtering, and debug semantics remain unclaimed.
The bounded two-event slice classifies `ADDON_RESTRICTION_STATE_CHANGED` and `BULK_PURCHASE_RESULT_RECEIVED` as evidence-required/unsafe. Registration and enum publication exist, but no modeled transition/purchase producer or focused proof establishes exact payload values/structures/arity, synchronous timing, ordering, duplicate behavior, lifecycle, or consumers.
The bounded ten-event CatalogShop/encounter-chat/combat-log/commentator slice classifies `CATALOG_SHOP_REFUNDABLE_DECORS_UPDATED`, `CATALOG_SHOP_VIRTUAL_CURRENCY_BALANCE_UPDATE`, `CATALOG_SHOP_VIRTUAL_CURRENCY_BALANCE_UPDATE_FAILURE`, `CHAT_MSG_ENCOUNTER_EVENT`, `COMBAT_LOG_APPLY_FILTER_SETTINGS`, `COMBAT_LOG_ENTRIES_CLEARED`, `COMBAT_LOG_MESSAGE`, `COMBAT_LOG_MESSAGE_LIMIT_CHANGED`, `COMBAT_LOG_REFILTER_ENTRIES`, and `COMMENTATOR_COMBAT_EVENT` as evidence-required/unsafe. Names, payload metadata, and registration may exist, but current placeholder state has no authoritative producer or focused proof for exact payloads, restricted/callback rules, validity/registerability, state ordering, lifecycle, duplicate behavior, or consumer effects. `CHAT_MSG_ENCOUNTER_EVENT` validity/registerability remains unresolved, and the `COMMENTATOR_COMBAT_EVENT` source payload conflicts with consumer state queries.
The bounded three-row `C_BattleNet` mutation/transport slice classifies `SendGameData`, `SendWhisper`, and `SetCustomMessage` as evidence-required/unsafe because the current simulator has no registration, backing transport/state, restriction enforcement, result model, or focused side-effect proof; exact account/text validation, return values, persistence, events, and consumer behavior remain unproven.

The bounded one-row `C_CharacterServices.AssignFCMDistribution` slice classifies the API as evidence-required/unsafe because the source register provides no signature/result metadata and the simulator has no FCM validation/assignment model. Account/realm/character checks, validation-only behavior, exact results, state transitions, persistence, and events remain unproven.
The bounded 23-row `C_CatalogShop` API/structure slice classifies `HasNewProducts` as best-effort/behavioral only for publication and the exact constant-false boolean result proven by `test_startup_service_namespaces_exist`; the other 22 rows are evidence-required/unsafe because current CatalogShop state is absent, no-op, incomplete, seeded, wrong-typed, or untested and does not establish purchase, category/product, refundable-decor, currency, session, refresh, restriction, payload, event, or lifecycle semantics.
The bounded 12.0.0 tutorial/pet global-constant slice classifies five added globals as best-effort/behavioral. Focused startup proof establishes numeric publication and exact values only; tutorial and pet-journal behavior, consumers, lifecycle, and historical load timing remain unclaimed.
The bounded 12.0.0 conflicting-global slice classifies 12 LE_GAME_ERR globals as evidence-required/unsafe because source-register values conflict with current fallback publication; authoritative epoch/value reconciliation is required before implementation.
The bounded 12.0.0 housing-payload-field slice classifies 16 exterior size/type, house-level, and decor-refund structure fields as evidence-required/unsafe. Current temporary data is absent or fixture-backed and does not establish exact contracts, authoritative values, state transitions, ordering/localization, or consumer behavior.
The bounded 12.0.0 general-event slice classifies nine added faction, initiative, loot-rule, nameplate, and neighborhood events as evidence-required/unsafe. Retail registration exists, but no modeled producer or focused proof establishes source payload contracts, timing, lifecycle, ordering, or duplicate behavior.
The bounded 12.0.0 housing-event slice classifies 12 added events as evidence-required/unsafe. The retail event registry accepts each name, but no modeled producer or focused proof establishes source payload contracts, timing, lifecycle, ordering, or duplicate behavior. Registration alone cannot close these rows.
The bounded 12.0.0 legacy-global removal slice classifies exactly four removed globals (`IsConsumableSpell`, `SetRaidTargetProtected`, `SpellIsAlwaysShown`, and `StripHyperlinks`) as best-effort/behavioral from one full-LoD `rawget(_G, name)` probe; the vendor-present slice classifies exactly 56 deprecated wrapper globals as best-effort/vendor-present from one full-LoD publication and representative forwarding/alias proof, while five chat/spell wrappers are covered separately by focused forwarding tests; the two diagnosed legacy cursor globals (`CombatLogAdvanceEntry` and `CombatLogSetCurrentEntry`) are now evidence-required/unsafe: pinned retail/PTR deprecated combat-log addons do not publish them, while the temporary simulator model retains fixture-only cursor behavior for Wrath/Mists callers; no replacement contract or authoritative legacy semantics is established. No replacement-behavior or historical timing claim is made. The bounded 12.0.0 `C_UnitAurasPrivate` slice covers one added and nine changed secure-only API rows. Source-register evidence establishes secure-only and two return shapes only; current temporary behavior is permissive/partial and existing tests prove simulator-only seeded state/callback behavior rather than retail security or private-aura semantics. The audit remains open.
The bounded 12.0.0 `C_UnitAuras` slice covers nine added and one changed API row. Source-register evidence establishes signatures/defaults only; current aura lookup/state behavior is adjacent seeded functionality, not proof of defensive classification, expiration/display formatting, duration objects, refresh calculations, color curves, sorted instance IDs, private callback dispatch, or GetUnitAuras sort semantics. The audit remains open.
The bounded 12.0.0 `C_CombatLogSecure` slice classifies exactly nine added secure-only API rows as evidence-required/unsafe using checked-in source-register evidence and the permissive/fixture-backed `src/lua_api/workarounds/temporary/combat_log_state.rs` model; tests remain empty with null commit, approval, and scope exception. Secure/taint enforcement, filtering rules, event/message payload shape, navigation semantics, and entry lifecycle remain unproven, and no approval can close these rows. The audit remains open.

## Content
- **Source:** `data/patch-api/sources/12.0.0-register.json`
- **Source SHA-256:** `6f26d194d0c3f721b3a071217cf69714f1278950512369272298735bdf44c863`
- **Boundary:** retail 11.2.7 build 65299 → final explicit retail 12.0.0 build 65727
- **Rows:** 3410 total — 0 implemented, 885 best-effort, 786 evidence-required, 2 exception-requested, 1737 untriaged
- **Directions:** 2554 added, 313 changed, 543 removed
- **Limit:** no historical 12.0.0 FrameXML tree or live SavedVariables capture is claimed.

Source occurrence objects preserve optional typed `before`/`after` JSON payloads with normalized `category`, `value`, and `metadata` fields for exact enum, constant, signature, and structure triage. Added rows carry `after`, removed rows `before`, changed rows both, and transient add/remove rows the corresponding side. Row identity remains `direction+symbol`; unknown occurrence fields are rejected. These payloads do not change the 3410-row register or the 2554/313/543 direction counts.

| Symbol | Status | Category | Direction | Note |
|---|---|---|---|---|
| `ADDON_RESTRICTION_STATE_CHANGED` | evidence-required | event | added | Evidence required: no modeled addon-restriction transition producer or focused proof establishes exact enum payload values, synchronous timing, ordering, duplicate suppression, or lifecycle behavior. |
| `AbbreviateConfig` | best-effort | luaobject | added | Best-effort behavioral evidence covers table/method shape, round-trip storage, per-instance isolation, read-only keys, and tostring; exact arrayof NumberAbbrevData structure fidelity is not established. |
| `AbbreviateConfig.GetAbbreviateNumberData` | best-effort | luaobject-method | added | Best-effort behavioral evidence covers table/method shape, round-trip storage, per-instance isolation, read-only keys, and tostring; exact arrayof NumberAbbrevData structure fidelity is not established. |
| `AbbreviateConfig.SetAbbreviateNumberData` | best-effort | luaobject-method | added | Best-effort behavioral evidence covers table/method shape, round-trip storage, per-instance isolation, read-only keys, and tostring; exact arrayof NumberAbbrevData structure fidelity is not established. |
| `AbbreviateLargeNumbers` | evidence-required | api | added | Evidence required: Current temporary fallback ignores NumberAbbrevOptions and returns a floored numeric string; abbreviation thresholds, localization, options, validation, and edge semantics are unmodeled. |
| `AbbreviateNumbers` | evidence-required | api | added | Evidence required: Current temporary fallback ignores NumberAbbrevOptions and returns a direct string conversion; abbreviation thresholds, localization, options, validation, and edge semantics are unmodeled. |
| `AddSourceLocationExclude` | best-effort | api | added | Best-effort behavioral evidence is limited to global publication and successful string-argument no-op invocation. Source-location exclusion storage, filtering, debug output effects, validation, and lifecycle semantics are not claimed. |
| `BULK_PURCHASE_RESULT_RECEIVED` | evidence-required | event | added | Evidence required: no modeled bulk-purchase producer or focused proof establishes exact result values, product-result structures, payload arity, consumer behavior, synchronous timing, ordering, or lifecycle semantics. |
| `CAAEnabled` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAInterruptCast` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAInterruptCastSuccess` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAPartyHealthFrequency` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAPartyHealthPercent` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAPlayerCastFormat` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAPlayerCastMinTime` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAPlayerCastMode` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAPlayerCastThrottle` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAPlayerHealthFormat` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAPlayerHealthPercent` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAPlayerHealthThrottle` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAResource1Formats` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAResource1Percents` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAResource1Throttle` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAResource2Formats` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAResource2Percents` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAResource2Throttle` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAASayCombatEnd` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAASayCombatStart` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAASayIfTargeted` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAASayTargetName` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAASpeed` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAATargetCastFormat` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAATargetCastMinTime` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAATargetCastMode` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAATargetCastThrottle` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAATargetDeathBehavior` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAATargetHealthFormat` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAATargetHealthPercent` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAATargetHealthThrottle` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAVoice` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CAAVolume` | best-effort | cvar | added | Best-effort behavioral evidence is limited to current startup/default CVar publication and exact 12.0.0 string value. CAA behavior, UI/audio effects, mutation, persistence, events, secure/read-only flags, consumers, and later-epoch semantics are not claimed. |
| `CATALOG_SHOP_REFUNDABLE_DECORS_UPDATED` | evidence-required | event | added | Evidence required: no modeled refundable-decor refresh producer or focused proof establishes zero-payload dispatch, state-update ordering, lifecycle, duplicate behavior, or CatalogShop consumer effects. |
| `CATALOG_SHOP_VIRTUAL_CURRENCY_BALANCE_UPDATE` | evidence-required | event | added | Evidence required: no modeled virtual-currency balance producer or focused proof establishes exact string payloads, success ordering, lifecycle, duplicate behavior, or CatalogShop consumers. |
| `CATALOG_SHOP_VIRTUAL_CURRENCY_BALANCE_UPDATE_FAILURE` | evidence-required | event | added | Evidence required: no modeled virtual-currency failure producer or focused proof establishes exact currencyCode delivery, failure conditions, ordering, lifecycle, duplicate behavior, or consumers. |
| `CHAT_MSG_ENCOUNTER_EVENT` | evidence-required | event | added | Evidence required: authoritative validity/registerability behavior and exact 17-field producer semantics are unresolved; no focused proof establishes dispatch, payload values/types, timing, or lifecycle. |
| `COMBAT_LOG_APPLY_FILTER_SETTINGS` | evidence-required | event | added | Evidence required: no modeled filter-settings producer or focused proof establishes restricted callback registration, exact table payload, state mutation, dispatch timing, ordering, lifecycle, or consumer effects. |
| `COMBAT_LOG_ENTRIES_CLEARED` | evidence-required | event | added | Evidence required: no modeled clear-event producer or focused proof establishes zero-payload dispatch, timing, ordering, repeated clears, empty-state behavior, or lifecycle. |
| `COMBAT_LOG_MESSAGE` | evidence-required | event | added | Evidence required: no modeled combat-message producer or focused proof establishes exact message/RGB/order payloads, synchronous dispatch, callback rules, ordering, or lifecycle. |
| `COMBAT_LOG_MESSAGE_LIMIT_CHANGED` | evidence-required | event | added | Evidence required: no modeled message-limit event producer or focused proof establishes exact numeric payload, timing, repeated/invalid updates, ordering, lifecycle, or consumers. |
| `COMBAT_LOG_REFILTER_ENTRIES` | evidence-required | event | added | Evidence required: no modeled refilter producer or focused proof establishes restricted callback registration, zero-payload dispatch, callback ordering, duplicate behavior, lifecycle, or actual refilter effects. |
| `COMMENTATOR_COMBAT_EVENT` | evidence-required | event | added | Evidence required: no modeled commentator combat-event producer/state or focused proof resolves the zero-payload source contract against consumer state queries, dispatch timing, ordering, repeated events, or lifecycle. |
| `C_ActionBar.ActionBarChargeInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_ActionBar.ActionBarChargeInfo.chargeModRate` | evidence-required | structure-field | added | Evidence required: static ActionBarChargeInfo.chargeModRate publication does not establish authoritative slot-dependent charge values, field relationships, cooldown progression, validation, secret behavior, or lifecycle semantics. |
| `C_ActionBar.ActionBarChargeInfo.cooldownDuration` | evidence-required | structure-field | added | Evidence required: static ActionBarChargeInfo.cooldownDuration publication does not establish authoritative slot-dependent charge values, field relationships, cooldown progression, validation, secret behavior, or lifecycle semantics. |
| `C_ActionBar.ActionBarChargeInfo.cooldownStartTime` | evidence-required | structure-field | added | Evidence required: static ActionBarChargeInfo.cooldownStartTime publication does not establish authoritative slot-dependent charge values, field relationships, cooldown progression, validation, secret behavior, or lifecycle semantics. |
| `C_ActionBar.ActionBarChargeInfo.currentCharges` | evidence-required | structure-field | added | Evidence required: static ActionBarChargeInfo.currentCharges publication does not establish authoritative slot-dependent charge values, field relationships, cooldown progression, validation, secret behavior, or lifecycle semantics. |
| `C_ActionBar.ActionBarChargeInfo.maxCharges` | evidence-required | structure-field | added | Evidence required: static ActionBarChargeInfo.maxCharges publication does not establish authoritative slot-dependent charge values, field relationships, cooldown progression, validation, secret behavior, or lifecycle semantics. |
| `C_ActionBar.ActionBarCooldownInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_ActionBar.ActionBarCooldownInfo.duration` | evidence-required | structure-field | added | Evidence required: current ActionBarCooldownInfo.duration publication does not establish complete typed payload fidelity, authoritative cooldown/enabled/mod-rate values, invalid-slot behavior, progression, secret behavior, or lifecycle semantics. |
| `C_ActionBar.ActionBarCooldownInfo.isEnabled` | evidence-required | structure-field | added | Evidence required: current ActionBarCooldownInfo.isEnabled publication does not establish complete typed payload fidelity, authoritative cooldown/enabled/mod-rate values, invalid-slot behavior, progression, secret behavior, or lifecycle semantics. |
| `C_ActionBar.ActionBarCooldownInfo.modRate` | evidence-required | structure-field | added | Evidence required: current ActionBarCooldownInfo.modRate publication does not establish complete typed payload fidelity, authoritative cooldown/enabled/mod-rate values, invalid-slot behavior, progression, secret behavior, or lifecycle semantics. |
| `C_ActionBar.ActionBarCooldownInfo.startTime` | evidence-required | structure-field | added | Evidence required: current ActionBarCooldownInfo.startTime publication does not establish complete typed payload fidelity, authoritative cooldown/enabled/mod-rate values, invalid-slot behavior, progression, secret behavior, or lifecycle semantics. |
| `C_ActionBar.GetActionAutocast` | evidence-required | api | added | Current action-bar implementation and registration do not publish this method (absent). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetActionBarPage` | best-effort | api | added | Best-effort behavioral evidence is limited to cold-start page 1, ActionBar_PageUp advancing to page 2, ACTIONBAR_PAGE_CHANGED dispatch, valid page 6, and wrap to page 1. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.GetActionChargeDuration` | evidence-required | api | added | Current implementation returns a fresh default LuaDurationObject without reading the action slot (default duration behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetActionCharges` | evidence-required | api | added | Current implementation returns a partial zero/default ActionBarChargeInfo-shaped table: current/max charges and cooldown fields are zero, with modRate 1. Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetActionCooldown` | evidence-required | api | added | Current implementation returns a partial ActionBarCooldownInfo-shaped table: spell-slot cooldown start/duration are modeled, while isEnabled=true and modRate=1 are constant defaults. Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetActionCooldownDuration` | evidence-required | api | added | Current implementation returns a fresh default LuaDurationObject without reading the action slot (default duration behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetActionDisplayCount` | evidence-required | api | added | Current implementation returns constant nil for every input (no-op/absent-value behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetActionLossOfControlCooldown` | evidence-required | api | added | Current implementation returns constant startTime=0 and duration=0 for every input (default/no-op behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetActionLossOfControlCooldownDuration` | evidence-required | api | added | Current implementation returns a fresh default LuaDurationObject without reading the action slot (default duration behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetActionText` | evidence-required | api | added | Current implementation returns constant nil for every input (no-op/absent-value behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetActionTexture` | best-effort | api | added | Best-effort behavioral evidence is limited to modeled action-slot texture lookup for the tested seeded spell and empty transition. Broader source-type, file-ID, item/spell, gear-state, secure, and UI-lifecycle fidelity remain unproven. |
| `C_ActionBar.GetActionUseCount` | evidence-required | api | added | Current implementation returns constant 0 for every input (default behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetBonusBarIndex` | best-effort | api | added | Best-effort behavioral evidence is limited to the state-backed active bonus index value 5. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.GetBonusBarOffset` | evidence-required | api | added | Current GetBonusBarOffset behavior is derived compatibility behavior without focused direct proof; authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetExtraBarIndex` | evidence-required | api | added | Current GetExtraBarIndex behavior is a constant compatibility value without focused direct proof; authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetMultiCastBarIndex` | evidence-required | api | added | Current GetMultiCastBarIndex behavior is a constant compatibility value without focused direct proof; authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.GetOverrideBarIndex` | best-effort | api | added | Best-effort behavioral evidence is limited to the inactive default override index 14. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.GetOverrideBarSkin` | best-effort | api | added | Best-effort behavioral evidence is limited to seeded override skin 1 during the skinned override transition. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.GetProfessionQualityInfo` | best-effort | api | added | Best-effort behavioral evidence is limited to seeded/empty/malformed profession-quality inputs: missing slot nil, the modeled state table fields, and malformed input nil. Broader CraftingQualityInfo source-type, file-ID, secure, and UI-lifecycle fidelity remain unproven. |
| `C_ActionBar.GetTempShapeshiftBarIndex` | best-effort | api | added | Best-effort behavioral evidence is limited to inactive default index 1 and the state-backed active index value 9. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.GetVehicleBarIndex` | best-effort | api | added | Best-effort behavioral evidence is limited to the state-backed vehicle index value 7 and inactive default index 12. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.HasAction` | best-effort | api | added | Best-effort behavioral evidence is limited to modeled action-slot presence for the tested empty and seeded spell paths. Broader source-type, item/spell, gear-state, secure, and UI-lifecycle fidelity remain unproven. |
| `C_ActionBar.HasBonusActionBar` | best-effort | api | added | Best-effort behavioral evidence is limited to the seeded bonus-bar flag and state-backed index transition. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.HasExtraActionBar` | best-effort | api | added | Best-effort behavioral evidence is limited to the seeded extra-action flag driving bar visibility and icon round-trip through update events. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.HasOverrideActionBar` | best-effort | api | added | Best-effort behavioral evidence is limited to the seeded override-bar flag and skinned override transition. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.HasRangeRequirements` | evidence-required | api | added | Current action-bar implementation and registration do not publish this method (absent). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.HasTempShapeshiftActionBar` | best-effort | api | added | Best-effort behavioral evidence is limited to the seeded temp-shapeshift flag and state-backed index transition. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.HasVehicleActionBar` | best-effort | api | added | Best-effort behavioral evidence is limited to the seeded vehicle-bar flag and skinned vehicle transition/update-skin dispatch. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.IsActionInRange` | evidence-required | api | added | Current action-bar implementation and registration do not publish this method (absent). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.IsAttackAction` | evidence-required | api | added | Current implementation returns constant false for every input (default predicate behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.IsAutoRepeatAction` | evidence-required | api | added | Current implementation returns constant false for every input (default predicate behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.IsConsumableAction` | evidence-required | api | added | Current implementation returns constant false for every input (default predicate behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.IsCurrentAction` | evidence-required | api | added | Current implementation provides a partial casting/action-slot model: it compares the active casting spell to the slot action and otherwise returns false. Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.IsEquippedAction` | evidence-required | api | added | Current implementation returns constant false for every input (default predicate behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.IsEquippedGearOutfitAction` | best-effort | api | added | Best-effort behavioral evidence is limited to modeled equipped-gear outfit slot behavior as exercised by the locked/unlocked action-button overlay. Broader gear state, source-type, secure, and UI-lifecycle fidelity remain unproven. |
| `C_ActionBar.IsItemAction` | evidence-required | api | added | Current implementation returns constant false for every input (default predicate behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.IsPossessBarVisible` | best-effort | api | added | Best-effort behavioral evidence is limited to the seeded possess-bar flag driving show/hide round-trip through UPDATE_POSSESS_BAR. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.IsStackableAction` | evidence-required | api | added | Current implementation returns constant false for every input (default predicate behavior). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.IsUsableAction` | evidence-required | api | added | Current implementation provides a partial action-slot presence model: it reports whether a spell/outfit slot exists and always returns isLackingResources=false. Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.RegisterActionUIButton` | evidence-required | api | added | Current implementation partially records button/action identity for simulator refresh dispatch but ignores the cooldown-frame argument; cooldown and lifecycle semantics are unproven. Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_ActionBar.SetActionBarPage` | best-effort | api | added | Best-effort behavioral evidence is limited to valid page 6 storage and the ACTIONBAR_PAGE_CHANGED transition exercised by the named page-change test. Exact retail paging, vehicle/override/bonus precedence, skins, secure state, and lifecycle semantics remain unproven. |
| `C_ActionBar.UnregisterActionUIButton` | evidence-required | api | added | Current action-bar implementation and registration do not publish this method (absent). Authoritative semantics or a correct model/test are required, and no approval can close the row. |
| `C_AdventureMap.AdventureMapQuestPortraitInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_AdventureMap.AdventureMapQuestPortraitInfo.modelSceneID` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to numeric-or-nil modelSceneID publication from injected portrait state. ModelScene rendering, camera, actors, and visual behavior remain outside scope; retail data population and edge semantics are unclaimed. |
| `C_AdventureMap.AdventureMapQuestPortraitInfo.mountPortraitDisplayID` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to required numeric mountPortraitDisplayID publication from injected portrait state and the tested zero value. Display-ID validity, mount selection, rendering, and edge semantics are unclaimed. |
| `C_AdventureMap.AdventureMapQuestPortraitInfo.name` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to required string name publication from injected portrait state. Retail data production, localization, missing-field behavior, and lifecycle semantics are unclaimed. |
| `C_AdventureMap.AdventureMapQuestPortraitInfo.portraitDisplayID` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to required numeric portraitDisplayID publication, exact seeded value, and tested zero-sentinel consumer gating. Display-ID validity, asset loading, rendering, and lifecycle semantics are unclaimed. |
| `C_AdventureMap.AdventureMapQuestPortraitInfo.text` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to required string text publication from injected portrait state. Retail data production, localization, empty/missing-field behavior, and lifecycle semantics are unclaimed. |
| `C_AdventureMap.GetQuestPortraitInfo` | best-effort | api | added | Best-effort behavioral evidence covers current injected-state lookup, five-field return shape, unknown/nonnumeric zero-value returns, nullable modelSceneID, and tested portrait-display consumer gating. Retail data population, full validation/edge semantics, and ModelScene rendering remain unclaimed. |
| `C_BattleNet.SendGameData` | evidence-required | api | added | Evidence required: no modeled Battle.net game-data transport or focused proof establishes namespace publication, exact SendAddonMessageResult outcomes, account/payload validation, throttling, side effects, or legacy forwarding behavior. |
| `C_BattleNet.SendWhisper` | evidence-required | api | added | Evidence required: no modeled Battle.net whisper transport or focused proof establishes publication, exact boolean results, account/text validation, restricted/macro behavior, chat events, failures, or side effects. |
| `C_BattleNet.SetCustomMessage` | evidence-required | api | added | Evidence required: no modeled custom-message state or focused proof establishes publication, assignment/clearing, exact success results, restrictions, persistence, validation, events, or UI-visible side effects. |
| `C_CatalogShop.BulkPurchaseIndividualProductResult` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_CatalogShop.BulkPurchaseIndividualProductResult.entitlementId` | evidence-required | structure-field | added | Evidence required: no modeled BulkPurchaseIndividualProductResult producer establishes required string entitlementId publication, value preservation, event array placement, validation, or lifecycle semantics. |
| `C_CatalogShop.BulkPurchaseIndividualProductResult.externalTransactionId` | evidence-required | structure-field | added | Evidence required: no modeled BulkPurchaseIndividualProductResult producer establishes required string externalTransactionId publication, value preservation, event array placement, validation, or lifecycle semantics. |
| `C_CatalogShop.BulkPurchaseIndividualProductResult.parentProductId` | evidence-required | structure-field | added | Evidence required: no modeled BulkPurchaseIndividualProductResult producer establishes nullable numeric parentProductId publication, absent/present values, event array placement, consumer behavior, or lifecycle semantics. |
| `C_CatalogShop.BulkPurchaseIndividualProductResult.recordId` | evidence-required | structure-field | added | Evidence required: no modeled BulkPurchaseIndividualProductResult producer establishes required numeric recordId publication, exact values, event array placement, validation, or lifecycle semantics. |
| `C_CatalogShop.BulkPurchaseIndividualProductResult.status` | evidence-required | structure-field | added | Evidence required: no modeled BulkPurchaseIndividualProductResult producer establishes required Enum.SimpleOrderStatus publication, exact values, event array placement, validation, or lifecycle semantics. |
| `C_CatalogShop.BulkPurchaseProducts` | evidence-required | api | added | Evidence required: no modeled bulk-purchase transaction establishes numeric-array validation, restricted invocation, exact boolean result, entitlement/currency state mutation, result events, or failure semantics. |
| `C_CatalogShop.CatalogShopCategoryInfo.showPersistentRefundButton` | evidence-required | structure-field | added | Evidence required: seeded constant-false category data does not establish required boolean field fidelity, category-dependent values, persistent-refund button behavior, or retail catalog lifecycle semantics. |
| `C_CatalogShop.CatalogShopProductInfo.consumableQuantity` | evidence-required | structure-field | added | Evidence required: current product payload omits nullable consumableQuantity and does not establish absent/populated numeric values, bundle behavior, consumer display, or lifecycle semantics. |
| `C_CatalogShop.ConfirmHousingPurchase` | evidence-required | api | added | Evidence required: current no-op does not establish product-array validation, purchase transitions, currency/entitlement mutation, success/failure behavior, events, or UI updates. |
| `C_CatalogShop.GetFirstCategoryByProductID` | evidence-required | api | added | Evidence required: temporary three-product mapping does not establish general nullable category lookup, category ordering, bundle behavior, invalid inputs, payload fidelity, or retail catalog semantics. |
| `C_CatalogShop.GetNewProducts` | evidence-required | api | added | Evidence required: no new-product ID state or API implementation establishes numeric-array output, viewed/refresh transitions, persistence, empty behavior, or consumers. |
| `C_CatalogShop.GetProductIDsForCategory` | evidence-required | api | added | Evidence required: seeded category arrays and weak shape-only coverage do not establish authoritative ordered membership, numeric element types, unknown-category behavior, or retail catalog lifecycle semantics. |
| `C_CatalogShop.GetRefundableDecors` | evidence-required | api | added | Evidence required: empty placeholder results ignore optional filtering and do not establish RefundableDecorInfo payloads, minimum remaining time, refresh lifecycle, or consumer behavior. |
| `C_CatalogShop.GetVirtualCurrencyBalance` | evidence-required | api | added | Evidence required: current numeric zero return violates the nullable-string contract and no currency-code keyed balance state, refresh lifecycle, or consumer behavior is modeled. |
| `C_CatalogShop.HasNewProducts` | best-effort | api | added | Best-effort behavioral evidence is limited to function publication and exact constant-false boolean result. New-product detection, viewed/refresh transitions, persistence, notification events, and consumer badge behavior remain unclaimed. |
| `C_CatalogShop.OpenCatalogShopInteractionFromHouse` | evidence-required | api | added | Evidence required: constant seeded session ID and refresh events do not establish house-bound session identity, lifecycle, validation, ordering, repeated calls, or purchase state. |
| `C_CatalogShop.OpenCatalogShopInteractionFromShop` | evidence-required | api | added | Evidence required: constant seeded session ID and refresh events do not establish shop session identity, lifecycle, validation, ordering, repeated calls, or purchase state. |
| `C_CatalogShop.RefreshRefundableDecors` | evidence-required | api | added | Evidence required: no refresh implementation/state establishes updated refundable results, event ordering, repeated/empty/failure behavior, or consumer effects. |
| `C_CatalogShop.RefreshVirtualCurrencyBalance` | evidence-required | api | added | Evidence required: current no-op does not establish currency-code validation, balance refresh state, success/failure events, ordering, repeated calls, or consumers. |
| `C_CatalogShop.RefundableDecorInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_CatalogShop.RefundableDecorInfo.decorGUID` | evidence-required | structure-field | added | Evidence required: no RefundableDecorInfo producer establishes required WOWGUID/string decorGUID publication, exact values, filtering, refresh behavior, or consumer semantics. |
| `C_CatalogShop.RefundableDecorInfo.standaloneDecorProductID` | evidence-required | structure-field | added | Evidence required: no RefundableDecorInfo producer establishes required numeric standaloneDecorProductID publication, exact values, filtering, refresh behavior, or consumer semantics. |
| `C_CatalogShop.RefundableDecorInfo.timeRemainingSeconds` | evidence-required | structure-field | added | Evidence required: no RefundableDecorInfo producer establishes required numeric timeRemainingSeconds publication, countdown/sorting behavior, filtering, refresh lifecycle, or consumer semantics. |
| `C_CatalogShop.StartHousingVCPurchaseConfirmation` | evidence-required | api | added | Evidence required: no housing virtual-currency confirmation state establishes product validation, session creation, balance/top-up transitions, events, repeated/invalid calls, or consumers. |
| `C_CharacterServices.AssignFCMDistribution` | evidence-required | api | added | Evidence required: authoritative signature/result metadata is incomplete and no modeled free-character-move validation/assignment state establishes realm/account/character checks, validation-only behavior, exact results, transitions, persistence, or events. |
| `C_ChatInfo.CancelEmote` | evidence-required | api | added | Evidence required: no modeled active-emote state or focused proof establishes cancellation transitions, repeated/no-active behavior, events, validation, or consumer effects. |
| `C_ChatInfo.InChatMessagingLockdown` | evidence-required | api | added | Evidence required: no modeled chat-lockdown state or focused proof establishes boolean restriction results, nullable reason enum, encounter/PvP/keystone transitions, ordering, or reset behavior. |
| `C_ChatInfo.PerformEmote` | evidence-required | api | added | Evidence required: no modeled emote state or focused proof establishes argument/default handling, exact success/failure, valid/invalid emotes, target/suppression behavior, restrictions, movement errors, or events. |
| `C_ColorUtil.ConvertHSLToHSV` | evidence-required | api | added | Current C_ColorUtil conversion behavior is absent or placeholder/identity/max-channel only; authoritative semantics or a correct modeled implementation/test are required, and no approval can close the row. |
| `C_ColorUtil.ConvertHSVToHSL` | evidence-required | api | added | Current C_ColorUtil conversion behavior is absent or placeholder/identity/max-channel only; authoritative semantics or a correct modeled implementation/test are required, and no approval can close the row. |
| `C_ColorUtil.ConvertHSVToRGB` | evidence-required | api | added | Current C_ColorUtil conversion behavior is absent or placeholder/identity/max-channel only; authoritative semantics or a correct modeled implementation/test are required, and no approval can close the row. |
| `C_ColorUtil.ConvertRGBToHSV` | evidence-required | api | added | Current C_ColorUtil conversion behavior is absent or placeholder/identity/max-channel only; authoritative semantics or a correct modeled implementation/test are required, and no approval can close the row. |
| `C_ColorUtil.GenerateTextColorCode` | best-effort | api | added | Best-effort behavioral evidence is limited to the tested RGB-to-ffRRGGBB code and explicit color-code text wrapping; edge/secret/localization/clamping semantics remain unproven. |
| `C_ColorUtil.WrapTextInColor` | evidence-required | api | added | WrapTextInColor lacks focused executable proof; authoritative semantics or a correct modeled implementation/test are required, and no approval can close the row. |
| `C_ColorUtil.WrapTextInColorCode` | best-effort | api | added | Best-effort behavioral evidence is limited to the tested RGB-to-ffRRGGBB code and explicit color-code text wrapping; edge/secret/localization/clamping semantics remain unproven. |
| `C_CombatAudioAlert.GetFormatSetting` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.GetSpeakerSpeed` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.GetSpeakerVolume` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.GetSpecSetting` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.GetThrottle` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.IsEnabled` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.SetFormatSetting` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.SetSpeakerSpeed` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.SetSpeakerVolume` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.SetSpecSetting` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.SetThrottle` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatAudioAlert.SpeakText` | evidence-required | api | added | The examined current registration surface does not register/model C_CombatAudioAlert or this method; this is not an exhaustive lexical/runtime absence claim. Exact combat-audio settings, speech scheduling/audio output, and enable/throttle semantics require authoritative evidence or a correct modeled subsystem, and no approval can close the row. |
| `C_CombatLog.ApplyFilterSettings` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLog.AreFilteredEventsEnabled` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLog.ClearEntries` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLog.DoesObjectMatchFilter` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLog.GetEntryRetentionTime` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLog.GetMessageLimit` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLog.IsCombatLogRestricted` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLog.RefilterEntries` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLog.SetEntryRetentionTime` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLog.SetFilteredEventsEnabled` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLog.SetMessageLimit` | evidence-required | api | added | Current C_CombatLog behavior is a shared permissive/fixture-backed temporary model, not authoritative 12.0.0 behavior. The source establishes signatures only; filter schema/matching, restriction state, retention and message-limit bounds, clear/refilter lifecycle, and entry semantics remain unproven. Existing tests prove simulator fixture behavior and are intentionally not attached; no approval can close this row. |
| `C_CombatLogSecure.AddEventFilter` | evidence-required | api | added | secure-only API; permissive temporary model and secure/filter/payload/navigation/lifecycle semantics remain unproven. |
| `C_CombatLogSecure.ClearEventFilters` | evidence-required | api | added | secure-only API; permissive temporary model and secure/filter/payload/navigation/lifecycle semantics remain unproven. |
| `C_CombatLogSecure.CreateCombatLogMessage` | evidence-required | api | added | secure-only API; permissive temporary model and secure/filter/payload/navigation/lifecycle semantics remain unproven. |
| `C_CombatLogSecure.GetCurrentEntryInfo` | evidence-required | api | added | secure-only API; permissive temporary model and secure/filter/payload/navigation/lifecycle semantics remain unproven. |
| `C_CombatLogSecure.GetCurrentEventInfo` | evidence-required | api | added | secure-only API; permissive temporary model and secure/filter/payload/navigation/lifecycle semantics remain unproven. |
| `C_CombatLogSecure.GetEntryCount` | evidence-required | api | added | secure-only API; permissive temporary model and secure/filter/payload/navigation/lifecycle semantics remain unproven. |
| `C_CombatLogSecure.SeekToNewestEntry` | evidence-required | api | added | secure-only API; permissive temporary model and secure/filter/payload/navigation/lifecycle semantics remain unproven. |
| `C_CombatLogSecure.SeekToPreviousEntry` | evidence-required | api | added | secure-only API; permissive temporary model and secure/filter/payload/navigation/lifecycle semantics remain unproven. |
| `C_CombatLogSecure.ShouldShowCurrentEntry` | evidence-required | api | added | secure-only API; permissive temporary model and secure/filter/payload/navigation/lifecycle semantics remain unproven. |
| `C_CombatText.GetActiveUnit` | evidence-required | api | added | Evidence required: no C_CombatText namespace/state or focused proof establishes no-value behavior, valid unit round-trip, invalid/secret/declassified-unit handling, or consumer routing. |
| `C_CombatText.GetCurrentEventInfo` | evidence-required | api | added | Evidence required: source metadata lacks result semantics and no modeled combat-text event state establishes no-event behavior, result arity/values, active-unit interaction, advancement, clearing, or lifecycle. |
| `C_CombatText.SetActiveUnit` | evidence-required | api | added | Evidence required: no modeled active-unit state or focused proof establishes valid/invalid unit handling, declassified-unit restrictions, GetActiveUnit round-trip, or CombatText event routing. |
| `C_Commentator.GetCombatEventInfo` | evidence-required | api | added | Evidence required: source metadata lacks an authoritative result contract and no modeled commentator combat-event state establishes publication, return values, event ordering, empty/repeated behavior, or lifecycle. |
| `C_CooldownViewer.CooldownViewerCooldown.category` | evidence-required | structure-field | added | Evidence required: no CooldownViewerCooldown producer establishes required Enum.CooldownViewerCategory publication, exact values, cooldown relationships, routing, invalid IDs, or lifecycle semantics. |
| `C_CooldownViewer.CooldownViewerCooldown.cooldownID` | evidence-required | structure-field | added | Evidence required: no CooldownViewerCooldown producer establishes required numeric cooldownID publication, exact values, record relationships, invalid IDs, or lifecycle semantics. |
| `C_CooldownViewer.GetValidAlertTypes` | evidence-required | api | added | Evidence required: API is absent and no cooldown-viewer state establishes non-null ordered alert-type arrays, known/unknown cooldown behavior, exact enum values, empty behavior, or Settings UI consumption. |
| `C_CreatureInfo.GetCreatureID` | evidence-required | api | added | Evidence required: GetCreatureID is absent, and no creature-GUID parser, identity table, valid/non-creature distinction, or nullable result behavior is modeled. |
| `C_CurveUtil.CreateColorCurve` | best-effort | api | added | Best-effort behavioral evidence is limited to factory return/table shape; exact retail userdata identity and color evaluation remain unproven. |
| `C_CurveUtil.CreateCurve` | best-effort | api | added | Best-effort behavioral evidence is limited to factory return/table shape; exact retail userdata identity and curve semantics remain unproven. |
| `C_CurveUtil.EvaluateColorFromBoolean` | evidence-required | api | added | Current generic proxy omits or does not faithfully establish the boolean-to-color evaluation contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `C_CurveUtil.EvaluateColorValueFromBoolean` | evidence-required | api | added | Current generic proxy omits or does not faithfully establish the boolean-to-color-value evaluation contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `C_CurveUtil.EvaluateGameCurve` | evidence-required | api | added | Current generic proxy omits or does not faithfully establish the game-curve evaluation contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `C_DamageMeter.DamageMeterAvailableCombatSession` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_DamageMeter.DamageMeterAvailableCombatSession.name` | evidence-required | structure-field | added | Evidence-required unsafe: the source register publishes a string name field, but the current seeded available-session fixture omits name and no focused test asserts it. Authoritative semantics or a correct model/test are required; no approval can close this row. |
| `C_DamageMeter.DamageMeterAvailableCombatSession.sessionID` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to the named available-session assertion that the seeded sessionID is 1; no complete retail session lifecycle or secret fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSession` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_DamageMeter.DamageMeterCombatSession.combatSources` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to seeded/empty combatSources table shape and counts from the named session tests; no complete retail aggregation or lifecycle fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSession.maxAmount` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to maxAmount == 0 for empty known meter types; the seeded session maxAmount is not asserted. No complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSessionSource` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_DamageMeter.DamageMeterCombatSessionSource.combatSpells` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to seeded/empty combatSpells table shape, counts, and spell lookup from the named tests; no complete retail aggregation or lifecycle fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSessionSource.maxAmount` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to the named seeded-source numeric maxAmount shape assertion; its value semantics are not established and no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSource` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_DamageMeter.DamageMeterCombatSource.amountPerSecond` | evidence-required | structure-field | added | Evidence-required unsafe: the current seeded fixture includes amountPerSecond, but the named tests do not assert its value or semantics. Authoritative semantics or a correct model/test are required; no approval can close this row. |
| `C_DamageMeter.DamageMeterCombatSource.classFilename` | evidence-required | structure-field | added | Evidence-required unsafe: the current seeded fixture includes classFilename, but the named tests do not assert its value or semantics. Authoritative semantics or a correct model/test are required; no approval can close this row. |
| `C_DamageMeter.DamageMeterCombatSource.isLocalPlayer` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to the named seeded source assertion that Player is local; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSource.name` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to the named seeded source assertion that the top source is Player; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSource.sourceGUID` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to seeded source identity and GUID-based lookup assertions from the named tests; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSource.specIconID` | evidence-required | structure-field | added | Evidence-required unsafe: the current seeded fixture includes specIconID, but the named tests do not assert its value or semantics. Authoritative semantics or a correct model/test are required; no approval can close this row. |
| `C_DamageMeter.DamageMeterCombatSource.totalAmount` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to seeded source totals and source-lookup total matching from the named tests; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSpell` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_DamageMeter.DamageMeterCombatSpell.amountPerSecond` | evidence-required | structure-field | added | Evidence-required unsafe: the current seeded fixture includes spell amountPerSecond, but the named tests do not assert its value or semantics. Authoritative semantics or a correct model/test are required; no approval can close this row. |
| `C_DamageMeter.DamageMeterCombatSpell.combatSpellDetails` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to the named seeded spell-detail table-shape assertion; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSpell.creatureName` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to the named seeded spell string-shape assertion; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSpell.spellID` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to the named seeded spell lookup assertion for spellID 19750; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.DamageMeterCombatSpell.totalAmount` | evidence-required | structure-field | added | Evidence-required unsafe: the current seeded fixture includes spell totalAmount, but the named tests do not assert its value or semantics. Authoritative semantics or a correct model/test are required; no approval can close this row. |
| `C_DamageMeter.DamageMeterCombatSpellUnitDetails` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_DamageMeter.DamageMeterCombatSpellUnitDetails.amount` | evidence-required | structure-field | added | Evidence-required unsafe: the current seeded fixture includes unit-detail amount, but the named tests do not assert its value or semantics. Authoritative semantics or a correct model/test are required; no approval can close this row. |
| `C_DamageMeter.DamageMeterCombatSpellUnitDetails.classification` | evidence-required | structure-field | added | Evidence-required unsafe: the current seeded fixture includes unit-detail classification, but the named tests do not assert its value or semantics. Authoritative semantics or a correct model/test are required; no approval can close this row. |
| `C_DamageMeter.DamageMeterCombatSpellUnitDetails.unitClassFilename` | evidence-required | structure-field | added | Evidence-required unsafe: the current seeded fixture includes unitClassFilename, but the named tests do not assert its value or semantics. Authoritative semantics or a correct model/test are required; no approval can close this row. |
| `C_DamageMeter.DamageMeterCombatSpellUnitDetails.unitName` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to the named seeded unit-details string-shape assertion; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.GetAvailableCombatSessions` | best-effort | api | added | Best-effort behavioral evidence is limited to returning one seeded available-session table with sessionID 1 in the named test; no complete retail session lifecycle or secret fidelity is claimed. |
| `C_DamageMeter.GetCombatSessionFromID` | best-effort | api | added | Best-effort behavioral evidence is limited to seeded ID/zero-ID lookup, missing-session nil, and empty known meter results from the named tests; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.GetCombatSessionFromType` | best-effort | api | added | Best-effort behavioral evidence is limited to seeded Overall/Current lookup and empty known meter results from the named tests; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.GetCombatSessionSourceFromID` | best-effort | api | added | Best-effort behavioral evidence is limited to seeded GUID lookup, optional creature-ID handling, missing-source nil, and empty known meter results from the named tests; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.GetCombatSessionSourceFromType` | best-effort | api | added | Best-effort behavioral evidence is limited to seeded Overall/Current source lookup, optional creature-ID handling, missing-source nil, and empty known meter results from the named tests; no complete retail aggregation/lifecycle/secret fidelity is claimed. |
| `C_DamageMeter.IsDamageMeterAvailable` | best-effort | api | added | Best-effort behavioral evidence is limited to the named seeded availability assertion returning true with no failure reason; no complete retail availability lifecycle or secret fidelity is claimed. |
| `C_DamageMeter.ResetAllCombatSessions` | evidence-required | api | added | Evidence-required unsafe: the source register adds ResetAllCombatSessions, but the current damage-meter implementation has no reset operation and the focused test set is empty. The reset lifecycle requires authoritative semantics or a correct model/test; no approval can close this row. |
| `C_DeathRecap.DeathRecapEventInfo` | evidence-required | structure | added | Pinned retail/PTR API documentation and Wowless schema expose no fields; vendor consumers reveal only partial amount/timestamp/sourceGUID usage, while current simulator state models killing blows only. Exact event-field semantics require authoritative evidence or a correct event model; no approval can close this row. |
| `C_DeathRecap.GetRecapEvents` | evidence-required | api | added | Signatures establish only an optional recap ID and table return. Event fields, ordering, default/no-argument selection, and unknown-ID behavior remain unproven; current simulator has no event-record model. |
| `C_DeathRecap.GetRecapLink` | evidence-required | api | added | Signatures and vendor display use establish only an optional recap ID and displayable string; link format and missing/unknown recap behavior remain unproven, with no current link model. |
| `C_DeathRecap.HasRecapEvents` | evidence-required | api | added | Do not infer a best-effort non-empty check from killing-blow state. Default/no-argument selection, unknown-ID handling, and the event-presence predicate remain unproven. |
| `C_DelvesUI.GetLockedTextForCompanion` | evidence-required | api | added | Evidence required: the method is absent, and no companion lock state, lock reason, progression state, text selection, nil-ID, or unknown-ID behavior is modeled. |
| `C_DelvesUI.IsTraitTreeForCompanion` | evidence-required | api | added | Evidence required: the predicate is absent, and no companion, non-companion, unknown, or invalid trait-tree classification behavior is modeled. |
| `C_DurationUtil.CreateDuration` | best-effort | api | added | Best-effort behavioral evidence covers factory return shape and LuaDurationObject method exposure; full time, secret, and curve semantics are not established. |
| `C_DurationUtil.GetCurrentTime` | evidence-required | api | added | Current behavior is constant (returns 0); authoritative time semantics or a correct modeled implementation is required, and no approval can close this row. |
| `C_EncounterTimeline.AddEditModeEvents` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.AddScriptEvent` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.CancelAllScriptEvents` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.CancelEditModeEvents` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.CancelScriptEvent` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo` | evidence-required | structure | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo.duration` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo.iconFileID` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo.icons` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo.id` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo.isApproximate` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo.maxQueueDuration` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo.severity` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo.source` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo.spellID` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineEventInfo.spellName` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineScriptEventRequest` | evidence-required | structure | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineScriptEventRequest.duration` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineScriptEventRequest.iconFileID` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineScriptEventRequest.icons` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineScriptEventRequest.maxQueueDuration` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineScriptEventRequest.overrideName` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineScriptEventRequest.paused` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineScriptEventRequest.severity` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineScriptEventRequest.spellID` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineTrackInfo` | evidence-required | structure | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineTrackInfo.id` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineTrackInfo.maximumDuration` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineTrackInfo.maximumEventCount` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineTrackInfo.minimumDuration` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineTrackInfo.minimumEventGapDuration` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineTrackInfo.minimumEventIntroDuration` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineTrackInfo.sortDirection` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.EncounterTimelineTrackInfo.type` | evidence-required | structure-field | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.FinishScriptEvent` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.GetCurrentTime` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.GetEventCountBySource` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.GetEventInfo` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.GetEventList` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.GetEventState` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.GetEventTimeElapsed` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.GetEventTimeRemaining` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.GetEventTrack` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.GetTrackInfo` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.GetTrackList` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.HasActiveEvents` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.HasAnyEvents` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.HasPausedEvents` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.HasVisibleEvents` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.IsEventBlocked` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.IsFeatureAvailable` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.IsFeatureEnabled` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.PauseScriptEvent` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.ResumeScriptEvent` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterTimeline.SetEventIconTextures` | evidence-required | api | added | The current temporary model is a partial hard-coded seeded fixture: only nine APIs are present, 46 of these 55 rows are absent, and present behavior is not authoritative retail semantics. Authoritative live evidence or a correct modeled encounter-timeline subsystem with focused tests is required; no approval can close this row. |
| `C_EncounterWarnings.EncounterWarningInfo` | evidence-required | structure | added | Current structure evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.casterGUID` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.casterName` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.duration` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.iconFileID` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.isDeadly` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.severity` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.shouldPlaySound` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.shouldShowChatMessage` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.shouldShowWarning` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.targetGUID` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.targetName` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.text` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.EncounterWarningInfo.tooltipSpellID` | evidence-required | structure-field | added | Current structure-field evidence is limited to fabricated Edit Mode preview/static payload behavior in `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.GetEditModeWarningInfo` | evidence-required | api | added | Current implementation returns a fabricated Edit Mode preview/static payload from `src/lua_api/globals/missing_surface/encounter_warnings.rs`; exact encounter state and payload field meanings require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.GetSoundKitForSeverity` | evidence-required | api | added | The examined current `src/lua_api/globals/missing_surface/encounter_warnings.rs` has no registration for this method; this is not an exhaustive lexical/runtime absence claim. Exact severity-to-sound mapping requires authoritative evidence or a correct modeled audio subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.IsFeatureAvailable` | evidence-required | api | added | The examined current `src/lua_api/globals/missing_surface/encounter_warnings.rs` has no registration for this method; this is not an exhaustive lexical/runtime absence claim. Exact feature-availability semantics require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.IsFeatureEnabled` | evidence-required | api | added | The examined current `src/lua_api/globals/missing_surface/encounter_warnings.rs` has no registration for this method; this is not an exhaustive lexical/runtime absence claim. Exact feature-enabling semantics require authoritative evidence or a correct modeled encounter subsystem/test, and no approval can close the row. |
| `C_EncounterWarnings.PlaySound` | evidence-required | api | added | Current implementation in `src/lua_api/globals/missing_surface/encounter_warnings.rs` is a no-op. Exact audio playback semantics require authoritative evidence or a correct modeled audio subsystem/test, and no approval can close the row. |
| `C_EventScheduler.CanShowEvents` | best-effort | api | added | Best-effort behavioral evidence covers tested simulator visibility derivation from explicit overrides, suppression, and event-list state. Retail event availability, refresh timing, persistence, full lifecycle, and edge semantics remain unclaimed. |
| `C_EventScheduler.EventDisplayInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_EventScheduler.EventDisplayInfo.hideDescription` | evidence-required | structure-field | added | Evidence required: empty displayInfo tables do not publish required boolean hideDescription/default false or establish description visibility, producer, or consumer semantics. |
| `C_EventScheduler.EventDisplayInfo.hideTimeLeft` | evidence-required | structure-field | added | Evidence required: empty displayInfo tables do not publish required boolean hideTimeLeft/default false or establish time-left visibility, producer, or consumer semantics. |
| `C_EventScheduler.EventDisplayInfo.overrideAtlas` | evidence-required | structure-field | added | Evidence required: absent nullable overrideAtlas does not establish populated atlas values, producer behavior, validation, or UI override/fallback semantics. |
| `C_EventScheduler.EventDisplayInfo.overrideTooltipWidgetSetID` | evidence-required | structure-field | added | Evidence required: absent nullable overrideTooltipWidgetSetID does not establish numeric values, producer behavior, validation, or tooltip override/fallback semantics. |
| `C_EventScheduler.OngoingEventInfo.displayInfo` | evidence-required | structure-field | added | Evidence required: empty-table publication does not establish required typed EventDisplayInfo shape, documented fields, producer behavior, refresh lifecycle, or consumers. |
| `C_EventScheduler.ScheduledEventInfo.displayInfo` | evidence-required | structure-field | added | Evidence required: empty-table publication does not establish required typed EventDisplayInfo shape, documented fields, producer behavior, refresh lifecycle, or consumers. |
| `C_EventScheduler.ScheduledEventInfo.eventID` | evidence-required | structure-field | added | Evidence required: temporary seeded numeric event IDs lack focused field/type/value/reset proof and do not establish authoritative scheduler identity or lifecycle semantics. |
| `C_EventUtils.IsCallbackEvent` | best-effort | api | added | Best-effort behavioral evidence covers current static-registry positive/negative classification for COMBAT_LOG_EVENT and PLAYER_LOGIN. Exact 12.0.0 registry completeness, argument validation, dynamic behavior, and lifecycle semantics remain unclaimed. |
| `C_GameRules.IsPersonalResourceDisplayEnabled` | evidence-required | api | added | Evidence required: current fallback returns nil instead of boolean, and no personal-resource-display backing state, default, mutation path, or settings integration is modeled. |
| `C_HouseExterior.GetCurrentHouseExteriorType` | best-effort | api | added | Best-effort behavioral evidence is limited to callable publication and the seeded two-return shape/types/values 1 and Sunspire Cottage. Retail housing selection, mutation, persistence, refresh, validation, and lifecycle semantics remain unclaimed. |
| `C_HouseExterior.GetFixtureDebugInfoForGUID` | evidence-required | api | added | Evidence required: the checked-in source preserves publication provenance only, while the method, exact signature, return payload, GUID lookup, and fixture-debug state are absent. |
| `C_HouseExterior.GetHouseExteriorSizeOptions` | best-effort | api | added | Best-effort behavioral evidence is limited to callable publication and seeded selectedSize/options table values. Exact option structure, enum fidelity, mutation, persistence, refresh, validation, and lifecycle semantics remain unclaimed. |
| `C_HouseExterior.GetHouseExteriorTypeOptions` | best-effort | api | added | Best-effort behavioral evidence is limited to callable publication and seeded selectedExteriorType/options table values. Exact option metadata, selection mutation, persistence, refresh, validation, and lifecycle semantics remain unclaimed. |
| `C_HouseExterior.GetHoveredFixtureDebugInfo` | evidence-required | api | added | Evidence required: the source preserves publication provenance only, while the nil fallback cannot establish an unknown signature, payload shape, hovered-fixture state, or lifecycle. |
| `C_HouseExterior.GetSelectedFixtureDebugInfo` | evidence-required | api | added | Evidence required: the source preserves publication provenance only, while the function, unknown signature/payload, selected-fixture debug lookup, and lifecycle are absent. |
| `C_HouseExterior.SetHouseExteriorSize` | evidence-required | api | added | Evidence required: the no-op setter does not model valid size transitions, enum validation, getter-visible state, persistence, refresh, reset/isolation, or lifecycle. |
| `C_HouseExterior.SetHouseExteriorType` | evidence-required | api | added | Evidence required: the no-op setter does not model valid/invalid type selection, name resolution, getter round-trip, persistence, refresh, reset/isolation, or lifecycle. |
| `C_Housing.IsHousingMarketShopEnabled` | evidence-required | api | added | Evidence required: no explicit method or dedicated market-shop availability state exists, and generic fallback behavior does not establish the required boolean result. |
| `C_Housing.OnHouseFinderClickPlot` | evidence-required | api | added | Evidence required: no explicit callback or selected-plot request/event/state model exists, so numeric argument validation, click side effects, repeated clicks, and unknown plot behavior are unproven. |
| `C_HousingBasicMode.IsFreePlaceEnabled` | evidence-required | api | added | Evidence required: hardcoded true and a no-op setter do not establish default, setter/getter state transitions, reset/isolation, persistence, or exact free-placement semantics. |
| `C_HousingBasicMode.SetFreePlaceEnabled` | evidence-required | api | added | Evidence required: the no-op setter cannot establish initial state, true/false transitions, getter round-trip, reset/isolation, persistence, or exact free-placement semantics. |
| `C_HousingBasicMode.StartPlacingPreviewDecor` | evidence-required | api | added | Evidence required: the no-op does not model decor/bundle selection, placement-mode transition, invalid IDs, repeated calls, requests/events, or lifecycle behavior. |
| `C_HousingCatalog.DeletePreviewCartDecor` | evidence-required | api | added | Evidence required: callable no-op behavior does not establish GUID validation, observable deletion, unknown/repeated deletion, cart-state updates, events, or lifecycle semantics. |
| `C_HousingCatalog.GetBundleInfo` | best-effort | api | added | Best-effort behavioral evidence covers seeded bundle 5001 lookup, two-entry shape, and wasViewed state mutation. Exact retail schema, unknown-ID behavior, clone isolation, pricing/product semantics, validation, persistence, refresh, and lifecycle remain unclaimed. |
| `C_HousingCatalog.GetCartSizeLimit` | best-effort | api | added | Best-effort behavioral evidence is limited to callable publication, one numeric return, and seeded value 20. Dynamic enforcement, configurability, consumers, persistence, refresh, and lifecycle remain unclaimed. |
| `C_HousingCatalog.GetCatalogEntryRefundTimeStampByRecordID` | evidence-required | api | added | Evidence required: unconditional nil does not establish keyed refund timestamps, refundable/non-refundable distinctions, unknown records, argument handling, refund windows, refresh, or lifecycle. |
| `C_HousingCatalog.HasFeaturedEntries` | evidence-required | api | added | Evidence required: hardcoded true does not establish empty/populated catalog behavior, state derivation, refresh transitions, persistence, or exact featured-entry semantics. |
| `C_HousingCatalog.HousingBundleInfo.canPreview` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to seeded bundle 5001 publication of boolean canPreview=true. Exact preview eligibility calculation, mutation, validation, other bundles, persistence, refresh, and lifecycle remain unclaimed. |
| `C_HousingCatalog.HousingBundleInfo.originalPrice` | evidence-required | structure-field | added | Evidence required: nil-only fixture publication does not establish populated numeric prices, no-discount nil behavior, discount/currency semantics, validation, persistence, refresh, or lifecycle. |
| `C_HousingCatalog.HousingCatalogEntryInfo.dyeIDs` | evidence-required | structure-field | added | Evidence required: the required field is absent, and no numeric dye-ID contents, ordering, empty/non-empty behavior, ownership, refresh, reset, or lifecycle is modeled. |
| `C_HousingCatalog.HousingCatalogEntryInfo.isUniqueTrophy` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to seeded entry 1001 publication of boolean isUniqueTrophy=false. Exact trophy classification, other entries, mutation, persistence, refresh, validation, and lifecycle remain unclaimed. |
| `C_HousingCatalog.HousingCatalogEntryInfo.itemID` | best-effort | structure-field | added | Best-effort behavioral evidence is limited to seeded entry 1001 numeric itemID publication. Nullable/missing-item cases, authoritative item data, other entries, validation, mutation, persistence, refresh, and lifecycle remain unclaimed. |
| `C_HousingCatalog.HousingPreviewItemData` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_HousingCatalog.HousingPreviewItemData.bundleCatalogShopProductID` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so bundle product identity and nil/populated behavior, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.HousingPreviewItemData.decorGUID` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so preview decor GUID identity and nil/populated behavior, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.HousingPreviewItemData.decorID` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so preview decor identity, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.HousingPreviewItemData.icon` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so numeric preview icon identity, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.HousingPreviewItemData.id` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so preview-item identity, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.HousingPreviewItemData.isBundleChild` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so bundle-child classification, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.HousingPreviewItemData.isBundleParent` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so bundle-parent classification, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.HousingPreviewItemData.name` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so preview-item naming, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.HousingPreviewItemData.price` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so preview-item base pricing, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.HousingPreviewItemData.productID` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so market product identity and nil/populated behavior, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.HousingPreviewItemData.salePrice` | evidence-required | structure-field | added | Evidence required: no HousingPreviewItemData producer publishes this field, so sale pricing and nil/populated behavior, payload shape, validation, event/preview-list production, refresh, persistence, and lifecycle remain unmodeled. |
| `C_HousingCatalog.IsPreviewCartItemShown` | best-effort | api | added | Best-effort behavioral evidence covers in-memory unknown=false, setter round-trip, and promotion-visible preview GUID state. GUID validation, events/refresh, persistence, repeated/unknown semantics, and lifecycle remain unclaimed. |
| `C_HousingCatalog.PromotePreviewDecor` | best-effort | api | added | Best-effort behavioral evidence covers successful preview-GUID promotion and observable shown-state mutation. decorID validation/use, failures, repeated promotion, events, persistence, refresh, and lifecycle remain unclaimed. |
| `C_HousingCatalog.RequestHousingMarketRefundInfo` | evidence-required | api | added | Evidence required: a no-op cannot establish request side effects, refund-list population, empty/populated data, repeated requests, event timing, refresh, or lifecycle. |
| `C_HousingCatalog.SetPreviewCartItemShown` | best-effort | api | added | Best-effort behavioral evidence covers in-memory boolean storage observable through the getter. GUID validation, events/refresh, persistence, reset/isolation beyond test environment, and lifecycle remain unclaimed. |
| `C_HousingCustomizeMode.IsHouseExteriorDoorHovered` | evidence-required | api | added | Evidence required: the method and exterior-door hover state are absent, so hovered/non-hovered results, hit testing, transitions, validation, refresh, and lifecycle are unmodeled. |
| `C_HousingDecor.EnterPreviewState` | best-effort | api | added | Best-effort behavioral evidence covers entering the temporary preview flag and its getter/count effects. Placement, selection, cleanup, events, persistence, refresh, and exact lifecycle remain unclaimed. |
| `C_HousingDecor.ExitPreviewState` | best-effort | api | added | Best-effort behavioral evidence covers exiting the temporary preview flag and its getter/count effects. Preview cleanup, cancellation, events, persistence, refresh, and exact lifecycle remain unclaimed. |
| `C_HousingDecor.GetNumPreviewDecor` | best-effort | api | added | Best-effort behavioral evidence covers temporary preview-state-derived numeric 0/1 results. Actual decor cardinality, multiple previews, producers, validation, persistence, refresh, and lifecycle remain unclaimed. |
| `C_HousingDecor.IsModeDisabledForPreviewState` | evidence-required | api | added | Evidence required: constant false does not establish mode-dependent availability, argument validation, preview transitions, reset/isolation, refresh, or lifecycle. |
| `C_HousingDecor.IsPreviewState` | best-effort | api | added | Best-effort behavioral evidence covers temporary preview-state boolean transitions. Retail side effects, integration, persistence, refresh, reset beyond test isolation, and lifecycle remain unclaimed. |
| `C_InstanceEncounter.IsEncounterInProgress` | best-effort | api | added | Best-effort behavioral evidence covers boolean default/state reads and parity with the shared legacy encounter flag. Encounter start/end producers, event timing, persistence, reset beyond test isolation, and full encounter lifecycle remain unclaimed. |
| `C_InstanceEncounter.IsEncounterLimitingResurrections` | evidence-required | api | added | Evidence required: no modeled resurrection-limiting encounter state, explicit API method, producer, transition behavior, or focused boolean proof exists; generic nil fallback and deprecated-wrapper publication are insufficient. |
| `C_InstanceEncounter.IsEncounterSuppressingRelease` | evidence-required | api | added | Evidence required: no modeled encounter release-suppression state, explicit API method, producer, transition behavior, or focused boolean proof exists; generic nil fallback and deprecated-wrapper publication are insufficient. |
| `C_InstanceEncounter.ShouldShowTimelineForEncounter` | evidence-required | api | added | Evidence required: no modeled encounter timeline-visibility state, explicit API method, producer, transition behavior, or focused boolean proof exists; generic nil fallback and consumer usage are insufficient. |
| `C_Item.IsItemBindToAccount` | evidence-required | api | added | Evidence required: the target method is absent, related binding probes are constant false, and the checked-in item dataset lacks account-bound bonding 7/8 fixtures. ItemInfo parsing, true/false classification, unknown-item behavior, and binding-state fidelity remain unproven. |
| `C_LFGList.AdvancedFilterOptions.generalPlaystyle1` | best-effort | structure-field | added | Best-effort behavioral evidence covers returned boolean field publication and the exact false default. Mutation, serialization, validation, interaction with search filtering, persistence, refresh, and broader retail LFG semantics remain unclaimed. |
| `C_LFGList.AdvancedFilterOptions.generalPlaystyle2` | best-effort | structure-field | added | Best-effort behavioral evidence covers returned boolean field publication and the exact false default. Mutation, serialization, validation, interaction with search filtering, persistence, refresh, and broader retail LFG semantics remain unclaimed. |
| `C_LFGList.AdvancedFilterOptions.generalPlaystyle3` | best-effort | structure-field | added | Best-effort behavioral evidence covers returned boolean field publication and the exact false default. Mutation, serialization, validation, interaction with search filtering, persistence, refresh, and broader retail LFG semantics remain unclaimed. |
| `C_LFGList.AdvancedFilterOptions.generalPlaystyle4` | best-effort | structure-field | added | Best-effort behavioral evidence covers returned boolean field publication and the exact false default. Mutation, serialization, validation, interaction with search filtering, persistence, refresh, and broader retail LFG semantics remain unclaimed. |
| `C_LFGList.LfgEntryData.generalPlaystyle` | evidence-required | structure-field | added | Evidence required: no active-listing model produces LfgEntryData, so field presence, nullable behavior, enum values, mutation, serialization, validation, and lifecycle remain unproven. The separate search-result generalPlaystyle field does not satisfy this structure contract. |
| `C_LFGList.LfgListingCreateData.generalPlaystyle` | evidence-required | structure-field | added | Evidence required: CreateListing and UpdateListing are absent, so required/default generalPlaystyle input handling, enum validation, listing creation/update state, result propagation, errors, and lifecycle remain unproven. Seeded search-result output is a different contract. |
| `C_LFGList.LfgSearchResultData.generalPlaystyle` | best-effort | structure-field | added | Best-effort behavioral evidence covers seeded search-result field presence and numeric publication. Nullable omission, exact enum-value validation, invalid values, mutation, serialization, filtering effects, persistence, refresh, and full retail search-result lifecycle remain unclaimed. |
| `C_LimitedInput.LimitedInputAllowed` | evidence-required | api | added | Evidence required: C_LimitedInput is unmodeled; per-input allowance state, authorization/taint rules, budget exhaustion, argument validation, required boolean results, transitions, and lifecycle remain unproven. Enum publication and nil fallback are insufficient. |
| `C_MajorFactions.MajorFactionData.description` | evidence-required | structure-field | added | Evidence required: MajorFactionData has no description state and GetMajorFactionData does not emit the required field. Exact strings, defaults, per-faction values, unknown behavior, mutation, refresh, and lifecycle remain unproven; name and unlockDescription are different contracts. |
| `C_MajorFactions.MajorFactionData.highlights` | evidence-required | structure-field | added | Evidence required: MajorFactionData has no highlights state and GetMajorFactionData omits the required array. Element schema, ordering, empty/non-empty behavior, per-faction values, mutation, refresh, and lifecycle remain unproven. |
| `C_MajorFactions.MajorFactionData.playerCompanionID` | evidence-required | structure-field | added | Evidence required: MajorFactionData has no player-companion association and GetMajorFactionData omits the nullable field. Present/nil behavior, authoritative IDs, per-faction values, unknown behavior, mutation, refresh, and lifecycle remain unproven. |
| `C_MajorFactions.MajorFactionRenownRewardInfo.rewardType` | evidence-required | structure-field | added | Evidence required: no MajorFactionRenownRewardInfo model or producer exists, and the major-faction reward query returns an empty table. Present/nil rewardType behavior, numeric meanings, per-level rewards, ordering, unknown behavior, refresh, and lifecycle remain unproven. |
| `C_MajorFactions.RenownHighlightInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_MajorFactions.RenownHighlightInfo.description` | evidence-required | structure-field | added | Evidence required: no RenownHighlightInfo model or highlights producer exists. Required description strings, element schema, ordering, empty/non-empty behavior, per-faction values, unknown behavior, refresh, and lifecycle remain unproven. |
| `C_MajorFactions.RenownHighlightInfo.level` | evidence-required | structure-field | added | Evidence required: no RenownHighlightInfo model or highlights producer exists. Required numeric levels, distinction from RenownLevelInfo, element schema, ordering, empty/non-empty behavior, per-faction values, unknown behavior, refresh, and lifecycle remain unproven. |
| `C_MajorFactions.RenownHighlightInfo.title` | evidence-required | structure-field | added | Evidence required: no RenownHighlightInfo model or highlights producer exists. Required title strings, element schema, ordering, empty/non-empty behavior, per-faction values, unknown behavior, refresh, and lifecycle remain unproven. |
| `C_MajorFactions.ShouldDisplayMajorFactionAsJourney` | evidence-required | api | added | Evidence required: constant false does not establish per-faction journey-display policy, positive cases, unknown-ID behavior, argument/security restrictions, state transitions, refresh, or lifecycle. Callability/default fallback proof is insufficient. |
| `C_MajorFactions.ShouldUseJourneyRewardTrack` | evidence-required | api | added | Evidence required: constant false does not establish per-faction journey reward-track policy, positive cases, unknown-ID behavior, argument/security restrictions, state transitions, refresh, or lifecycle. Callability/default fallback proof is insufficient. |
| `C_NamePlate.GetNamePlateSize` | evidence-required | api | added | Source-register signature only; current nameplate behavior has no modeled 2D size subsystem and exact 12.0.0 semantics remain unproven. |
| `C_NamePlate.SetNamePlateSize` | evidence-required | api | added | Current shim is a no-op and exact 12.0.0 size-state semantics remain unproven; no approval can close this row. |
| `C_NamePlateManager.GetNamePlateHitTestInsets` | evidence-required | api | added | No modeled nameplate-manager hit-test state exists; exact 12.0.0 semantics remain unproven. |
| `C_NamePlateManager.IsNamePlateUnitBehindCamera` | exception-requested | api | added | Permanent no-3D project scope excludes direct nameplate camera/projection behavior; already-decided scope exception, not a user approval request. |
| `C_NamePlateManager.SetNamePlateHitTestFrame` | evidence-required | api | added | No modeled nameplate-manager frame-binding state exists; exact 12.0.0 semantics remain unproven. |
| `C_NamePlateManager.SetNamePlateHitTestInsets` | evidence-required | api | added | No modeled nameplate-manager hit-test state exists; exact 12.0.0 semantics remain unproven. |
| `C_NamePlateManager.SetNamePlateSimplified` | evidence-required | api | added | No modeled nameplate-manager simplified-state subsystem exists; exact 12.0.0 semantics remain unproven. |
| `C_NeighborhoodInitiative.AddTrackedInitiativeTask` | evidence-required | api | added | Evidence required: the no-op fallback does not add a task or establish valid/unknown ID behavior, duplicate handling, ordering, removal interaction, returned task information, persistence, event dispatch, refresh, or lifecycle. Callability and nil return are insufficient. |
| `C_NeighborhoodInitiative.GetActiveNeighborhood` | evidence-required | api | added | Evidence required: the method and active-neighborhood state are absent, while generic fallback returns nil instead of the required GUID string. No-active behavior, valid GUID output, state transitions, refresh, event integration, persistence, and lifecycle remain unproven. |
| `C_NeighborhoodInitiative.GetInitiativeActivityLogInfo` | evidence-required | api | added | Evidence required: no activity-log model or explicit method exists. Generic nil fallback does not establish absent/present behavior, payload field types and contents, task-entry arrays, update timing, refresh/event behavior, persistence, or lifecycle. |
| `C_NeighborhoodInitiative.GetInitiativeTaskChatLink` | evidence-required | api | added | Evidence required: the method and initiative-task chat-link model are absent, while generic fallback returns nil instead of the required string. Valid and unknown task IDs, exact link format, absent-task behavior, updates, refresh, persistence, and lifecycle remain unproven. |
| `C_NeighborhoodInitiative.GetInitiativeTaskInfo` | evidence-required | api | added | Evidence required: constant nil covers only a placeholder absent case. Valid and unknown task IDs, exact payload fields and types, task updates, tracked-task interaction, refresh, persistence, and lifecycle remain unproven. |
| `C_NeighborhoodInitiative.GetNeighborhoodInitiativeInfo` | evidence-required | api | added | Evidence required: no neighborhood-initiative state model or explicit method exists. Generic nil fallback does not establish absent/present behavior, populated field types and values, milestone/task payloads, progress transitions, refresh, persistence, or lifecycle. |
| `C_NeighborhoodInitiative.GetRequiredLevel` | evidence-required | api | added | Evidence required: the method and required-level state are absent, while generic fallback returns nil instead of a number. No-initiative behavior, valid required levels, state transitions, refresh, persistence, and lifecycle remain unproven. |
| `C_NeighborhoodInitiative.GetTrackedInitiativeTasks` | evidence-required | api | added | Evidence required: the fixed empty payload proves only placeholder shape. Empty, add/remove, duplicate, invalid/unknown ID, ordering, refresh, persistence, event, and lifecycle behavior remain unproven without tracked-task state. |
| `C_NeighborhoodInitiative.InitiativeActivityLogEntry` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_NeighborhoodInitiative.InitiativeActivityLogEntry.amount` | evidence-required | structure-field | added | Evidence required: no InitiativeActivityLogEntry runtime model or producer exists. Numeric field presence and values, empty/present logs, updates, refresh, persistence, and lifecycle remain unproven. |
| `C_NeighborhoodInitiative.InitiativeActivityLogEntry.completionTime` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeActivityLogEntry.playerName` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeActivityLogEntry.taskID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeActivityLogEntry.taskName` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeActivityLogInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_NeighborhoodInitiative.InitiativeActivityLogInfo.isLoaded` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeActivityLogInfo.neighborhoodGUID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeActivityLogInfo.nextUpdateTime` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeActivityLogInfo.taskActivity` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeMilestoneInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_NeighborhoodInitiative.InitiativeMilestoneInfo.milestoneOrderIndex` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeMilestoneInfo.requiredContributionAmount` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeMilestoneInfo.rewards` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeMilestoneRewardInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_NeighborhoodInitiative.InitiativeMilestoneRewardInfo.decorID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeMilestoneRewardInfo.decorQuantity` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeMilestoneRewardInfo.description` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeMilestoneRewardInfo.favor` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeMilestoneRewardInfo.money` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeMilestoneRewardInfo.rewardQuestID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeMilestoneRewardInfo.title` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.ID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.completed` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.criteriaList` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.description` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.inProgress` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.progressContributionAmount` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.requirementsList` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.rewardQuestID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.sortOrder` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.supersedes` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.taskName` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.taskType` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.timesCompleted` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTaskInfo.tracked` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.InitiativeTasksTracked` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_NeighborhoodInitiative.InitiativeTasksTracked.trackedIDs` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.IsInitiativeEnabled` | untriaged | api | added | api added in 12.0.0. |
| `C_NeighborhoodInitiative.IsPlayerInNeighborhoodGroup` | untriaged | api | added | api added in 12.0.0. |
| `C_NeighborhoodInitiative.IsViewingActiveNeighborhood` | untriaged | api | added | api added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.currentCycleID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.currentProgress` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.description` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.duration` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.initiativeID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.isLoaded` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.milestones` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.neighborhoodGUID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.playerTotalContribution` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.progressRequired` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.tasks` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.NeighborhoodInitiativeInfo.title` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_NeighborhoodInitiative.PlayerHasInitiativeAccess` | untriaged | api | added | api added in 12.0.0. |
| `C_NeighborhoodInitiative.PlayerMeetsRequiredLevel` | untriaged | api | added | api added in 12.0.0. |
| `C_NeighborhoodInitiative.RemoveTrackedInitiativeTask` | untriaged | api | added | api added in 12.0.0. |
| `C_NeighborhoodInitiative.RequestInitiativeActivityLog` | untriaged | api | added | api added in 12.0.0. |
| `C_NeighborhoodInitiative.RequestNeighborhoodInitiativeInfo` | untriaged | api | added | api added in 12.0.0. |
| `C_NeighborhoodInitiative.SetActiveNeighborhood` | untriaged | api | added | api added in 12.0.0. |
| `C_NeighborhoodInitiative.SetViewingNeighborhood` | untriaged | api | added | api added in 12.0.0. |
| `C_Ping.IsPingSystemEnabled` | untriaged | api | added | api added in 12.0.0. |
| `C_PvP.AreTrainingGroundsEnabled` | untriaged | api | added | api added in 12.0.0. |
| `C_PvP.BattlegroundInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_PvP.BattlegroundInfo.battlegroundID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.canEnter` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.gameType` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.icon` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.isHoliday` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.isRandom` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.isTrainingGround` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.lfgDungeonID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.longDescription` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.mapDescription` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.mapID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.maxPlayers` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.name` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.BattlegroundInfo.shortDescription` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_PvP.CanPlayerUseTrainingGroundsUI` | untriaged | api | added | api added in 12.0.0. |
| `C_PvP.GetBattlegroundInfo` | untriaged | api | added | api added in 12.0.0. |
| `C_PvP.GetRandomTrainingGroundRewards` | untriaged | api | added | api added in 12.0.0. |
| `C_PvP.GetTrainingGrounds` | untriaged | api | added | api added in 12.0.0. |
| `C_PvP.HasMatchStarted` | untriaged | api | added | api added in 12.0.0. |
| `C_PvP.HasRandomTrainingGroundWinToday` | untriaged | api | added | api added in 12.0.0. |
| `C_PvP.JoinRandomTrainingGround` | untriaged | api | added | api added in 12.0.0. |
| `C_PvP.JoinTrainingGround` | untriaged | api | added | api added in 12.0.0. |
| `C_QuestInfoSystem.GetQuestLogRewardFavor` | untriaged | api | added | api added in 12.0.0. |
| `C_QuestLog.GetActivePreyQuest` | untriaged | api | added | api added in 12.0.0. |
| `C_Reputation.IsFactionParagonForCurrentPlayer` | untriaged | api | added | api added in 12.0.0. |
| `C_RestrictedActions.CheckAllowProtectedFunctions` | untriaged | api | added | api added in 12.0.0. |
| `C_RestrictedActions.GetAddOnRestrictionState` | untriaged | api | added | api added in 12.0.0. |
| `C_RestrictedActions.IsAddOnRestrictionActive` | untriaged | api | added | api added in 12.0.0. |
| `C_Secrets.GetPowerTypeSecrecy` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.GetSpellAuraSecrecy` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.GetSpellCastSecrecy` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.GetSpellCooldownSecrecy` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.HasSecretRestrictions` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldActionCooldownBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldAurasBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldCooldownsBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldSpellAuraBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldSpellBookItemCooldownBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldSpellCooldownBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldTotemSlotBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldTotemSpellBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldUnitAuraIndexBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldUnitAuraInstanceBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldUnitAuraSlotBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldUnitComparisonBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldUnitHealthMaxBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldUnitIdentityBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldUnitPowerBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldUnitPowerMaxBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldUnitSpellCastBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_Secrets.ShouldUnitSpellCastingBeSecret` | evidence-required | api | added | The examined current registration surface does not register/model C_Secrets or this method; this is not an exhaustive lexical/runtime absence claim. Exact secrecy levels and action/aura/cooldown/totem/unit health/identity/power/cast restriction semantics require authoritative live evidence or a correct taint/security model and tests; secret behavior must not be guessed or approved closed. |
| `C_SecureTransfer.Cancel` | untriaged | api | added | api added in 12.0.0. |
| `C_SecureTransfer.CompleteHousingPurchase` | untriaged | api | added | api added in 12.0.0. |
| `C_SecureTransfer.CompleteHousingVCPurchase` | untriaged | api | added | api added in 12.0.0. |
| `C_SecureTransfer.GetHousingPurchaseCost` | untriaged | api | added | api added in 12.0.0. |
| `C_SecureTransfer.GetHousingVCPurchaseProductID` | untriaged | api | added | api added in 12.0.0. |
| `C_SettingsUtil.NotifySettingsLoaded` | untriaged | api | added | api added in 12.0.0. |
| `C_SettingsUtil.OpenSettingsPanel` | untriaged | api | added | api added in 12.0.0. |
| `C_Sound.PlaySound` | untriaged | api | added | api added in 12.0.0. |
| `C_Spell.GetSpellChargeDuration` | evidence-required | api | added | The 12.0.0 duration contract is not currently registered/implemented with focused proof; duration-object lifecycle and semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.GetSpellCooldownDuration` | evidence-required | api | added | The 12.0.0 duration contract is not currently registered/implemented with focused proof; duration-object lifecycle and semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.GetSpellDisplayCount` | evidence-required | api | added | The 12.0.0 display-count contract is not currently registered/implemented with focused proof; spell metadata/display semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.GetSpellLossOfControlCooldownDuration` | evidence-required | api | added | The 12.0.0 duration contract is not currently registered/implemented with focused proof; duration-object lifecycle and semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.GetSpellMaxCumulativeAuraApplications` | evidence-required | api | added | The 12.0.0 spell-metadata contract is not currently registered/implemented with focused proof; spell metadata semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.GetVisibilityInfo` | evidence-required | api | added | The 12.0.0 spell-metadata contract is not currently registered/implemented with focused proof; spell metadata semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.IsConsumableSpell` | evidence-required | api | added | The 12.0.0 boolean contract is not currently registered/implemented with focused proof; boolean semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.IsExternalDefensive` | evidence-required | api | added | The 12.0.0 boolean contract is not currently registered/implemented with focused proof; boolean semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.IsPriorityAura` | evidence-required | api | added | The 12.0.0 boolean contract is not currently registered/implemented with focused proof; boolean semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.IsSelfBuff` | evidence-required | api | added | A same-name internal IsSelfBuff implementation exists, but no matching 12.0.0 publication/behavioral contract is proven; boolean semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.IsSpellCrowdControl` | evidence-required | api | added | The 12.0.0 boolean contract is not currently registered/implemented with focused proof; boolean semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_Spell.IsSpellImportant` | evidence-required | api | added | The 12.0.0 boolean contract is not currently registered/implemented with focused proof; boolean semantics require authoritative evidence or a correct model, and no approval can close the row. |
| `C_SpellBook.FindBaseSpellByID` | untriaged | api | added | api added in 12.0.0. |
| `C_SpellBook.FindFlyoutSlotBySpellID` | untriaged | api | added | api added in 12.0.0. |
| `C_SpellBook.FindSpellOverrideByID` | untriaged | api | added | api added in 12.0.0. |
| `C_SpellBook.GetSpellBookItemChargeDuration` | untriaged | api | added | api added in 12.0.0. |
| `C_SpellBook.GetSpellBookItemCooldownDuration` | untriaged | api | added | api added in 12.0.0. |
| `C_SpellBook.GetSpellBookItemLossOfControlCooldownDuration` | untriaged | api | added | api added in 12.0.0. |
| `C_SpellDiminish.GetAllSpellDiminishCategories` | evidence-required | api | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.GetSpellDiminishCategoryInfo` | evidence-required | api | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.IsSystemSupported` | evidence-required | api | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.ShouldTrackSpellDiminishCategory` | evidence-required | api | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.SpellDiminishCategoryInfo` | evidence-required | structure | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.SpellDiminishCategoryInfo.category` | evidence-required | structure-field | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.SpellDiminishCategoryInfo.icon` | evidence-required | structure-field | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.SpellDiminishCategoryInfo.name` | evidence-required | structure-field | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.SpellDiminishTrackerInfo` | evidence-required | structure | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.SpellDiminishTrackerInfo.category` | evidence-required | structure-field | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.SpellDiminishTrackerInfo.duration` | evidence-required | structure-field | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.SpellDiminishTrackerInfo.isImmune` | evidence-required | structure-field | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.SpellDiminishTrackerInfo.showCountdown` | evidence-required | structure-field | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_SpellDiminish.SpellDiminishTrackerInfo.startTime` | evidence-required | structure-field | added | Current static eight-category fixture does not prove authoritative 12.0.0 category, ruleset, or tracker semantics. |
| `C_StableInfo.IsBonusPetSlotAvailable` | untriaged | api | added | api added in 12.0.0. |
| `C_StringUtil.EscapeLuaFormatString` | evidence-required | api | added | Current C_StringUtil model does not publish the function; authoritative semantics or a correct implementation are required, and no approval can close the row. |
| `C_StringUtil.EscapeLuaPatterns` | evidence-required | api | added | Current C_StringUtil model does not publish the function; authoritative semantics or a correct implementation are required, and no approval can close the row. |
| `C_StringUtil.EscapeQuotedCodes` | best-effort | api | added | Best-effort behavioral evidence is limited to quoted-code pipe escaping for the tested plain/color-code cases; exact edge/secret/localization semantics remain unproven. |
| `C_StringUtil.FloorToNearestString` | evidence-required | api | added | Current C_StringUtil model does not publish the function; authoritative semantics or a correct implementation are required, and no approval can close the row. |
| `C_StringUtil.RemoveContiguousSpaces` | evidence-required | api | added | Current C_StringUtil model does not publish the function; authoritative semantics or a correct implementation are required, and no approval can close the row. |
| `C_StringUtil.RoundToNearestString` | evidence-required | api | added | Current C_StringUtil model does not publish the function; authoritative semantics or a correct implementation are required, and no approval can close the row. |
| `C_StringUtil.StripHyperlinks` | evidence-required | api | added | Current C_StringUtil model does not publish the function; authoritative semantics or a correct implementation are required, and no approval can close the row. |
| `C_StringUtil.TruncateWhenZero` | evidence-required | api | added | Current C_StringUtil model does not publish the function; authoritative semantics or a correct implementation are required, and no approval can close the row. |
| `C_StringUtil.WrapString` | evidence-required | api | added | Current C_StringUtil model does not publish the function; authoritative semantics or a correct implementation are required, and no approval can close the row. |
| `C_TaskQuest.GetQuestUIWidgetSetByType` | untriaged | api | added | api added in 12.0.0. |
| `C_TooltipComparison.CompareItem` | untriaged | api | added | api added in 12.0.0. |
| `C_TooltipInfo.GetOutfit` | untriaged | api | added | api added in 12.0.0. |
| `C_TooltipInfo.GetUnitAuraByAuraInstanceID` | untriaged | api | added | api added in 12.0.0. |
| `C_TradeSkillUI.GetDependentReagents` | best-effort | api | added | Best-effort behavioral evidence is limited to table return/iteration safety and nil/malformed/unknown-reagent behavior; exact retail dependency semantics remain unproven. |
| `C_TradeSkillUI.GetItemCraftedQualityInfo` | evidence-required | api | added | The examined current professions implementation/registration surface does not publish this method; exact lexical/runtime absence is not claimed. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_TradeSkillUI.GetItemReagentQualityInfo` | evidence-required | api | added | The examined current professions implementation/registration surface does not publish this method; exact lexical/runtime absence is not claimed. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_TradeSkillUI.GetRecipeItemQualityInfo` | evidence-required | api | added | The examined current professions implementation/registration surface does not publish this method; exact lexical/runtime absence is not claimed. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_TradeSkillUI.GetRecipeQualityReagentLink` | evidence-required | api | added | The examined current professions implementation/registration surface does not publish this method; exact lexical/runtime absence is not claimed. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_Transmog.TransmogSlotVisualInfo` | evidence-required | structure | added | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_Transmog.TransmogSlotVisualInfo.appliedSourceID` | evidence-required | structure-field | added | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_Transmog.TransmogSlotVisualInfo.appliedVisualID` | evidence-required | structure-field | added | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_Transmog.TransmogSlotVisualInfo.baseSourceID` | evidence-required | structure-field | added | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_Transmog.TransmogSlotVisualInfo.baseVisualID` | evidence-required | structure-field | added | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_Transmog.TransmogSlotVisualInfo.hasUndo` | evidence-required | structure-field | added | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_Transmog.TransmogSlotVisualInfo.isHideVisual` | evidence-required | structure-field | added | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_Transmog.TransmogSlotVisualInfo.itemSubclass` | evidence-required | structure-field | added | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_Transmog.TransmogSlotVisualInfo.pendingSourceID` | evidence-required | structure-field | added | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_Transmog.TransmogSlotVisualInfo.pendingVisualID` | evidence-required | structure-field | added | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_TransmogCollection.DeleteCustomSet` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.GetCustomSetHyperlinkFromItemTransmogInfoList` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.GetCustomSetInfo` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.GetCustomSetItemTransmogInfoList` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.GetCustomSets` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.GetItemTransmogInfoListFromCustomSetHyperlink` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.GetNumMaxCustomSets` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.IsValidCustomSetName` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.IsValidTransmogSource` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.ModifyCustomSet` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.NewCustomSet` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.RenameCustomSet` | evidence-required | api | added | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_TransmogCollection.TransmogAppearanceSourceInfoData` | evidence-required | structure | added | Source records the structure name only; seeded collection state does not establish the exact 12.0.0 structure contract. |
| `C_TransmogCollection.TransmogAppearanceSourceInfoData.canHaveIllusion` | evidence-required | structure-field | added | Source records the field name only; seeded collection state does not establish the exact 12.0.0 field semantics. |
| `C_TransmogCollection.TransmogAppearanceSourceInfoData.category` | evidence-required | structure-field | added | Source records the field name only; seeded collection state does not establish the exact 12.0.0 field semantics. |
| `C_TransmogCollection.TransmogAppearanceSourceInfoData.icon` | evidence-required | structure-field | added | Source records the field name only; seeded collection state does not establish the exact 12.0.0 field semantics. |
| `C_TransmogCollection.TransmogAppearanceSourceInfoData.isCollected` | evidence-required | structure-field | added | Source records the field name only; seeded collection state does not establish the exact 12.0.0 field semantics. |
| `C_TransmogCollection.TransmogAppearanceSourceInfoData.itemAppearanceID` | evidence-required | structure-field | added | Source records the field name only; seeded collection state does not establish the exact 12.0.0 field semantics. |
| `C_TransmogCollection.TransmogAppearanceSourceInfoData.itemLink` | evidence-required | structure-field | added | Source records the field name only; seeded collection state does not establish the exact 12.0.0 field semantics. |
| `C_TransmogCollection.TransmogAppearanceSourceInfoData.itemSubclass` | evidence-required | structure-field | added | Source records the field name only; seeded collection state does not establish the exact 12.0.0 field semantics. |
| `C_TransmogCollection.TransmogAppearanceSourceInfoData.sourceType` | evidence-required | structure-field | added | Source records the field name only; seeded collection state does not establish the exact 12.0.0 field semantics. |
| `C_TransmogCollection.TransmogAppearanceSourceInfoData.transmoglink` | evidence-required | structure-field | added | Source records the field name only; seeded collection state does not establish the exact 12.0.0 field semantics. |
| `C_TransmogOutfitInfo.AddNewOutfit` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ChangeDisplayedOutfit` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ChangeViewedOutfit` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ClearAllPendingSituations` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ClearAllPendingTransmogs` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ClearDisplayedOutfit` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.CommitAndApplyAllPending` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.CommitOutfitInfo` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.CommitPendingSituations` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetActiveOutfitID` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetAllSlotLocationInfo` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetCollectionInfoForSlotAndOption` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetCurrentlyViewedOutfitID` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetEquippedSlotOptionFromTransmogSlot` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetIllusionDefaultIMAIDForCollectionType` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetItemModifiedAppearanceEffectiveCategory` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetLinkedSlotInfo` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetMaxNumberOfTotalOutfitsForSource` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetMaxNumberOfUsableOutfits` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetNextOutfitCost` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetNumberOfOutfitsUnlockedForSource` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetOutfitInfo` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetOutfitSituation` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetOutfitSituationsEnabled` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetOutfitsInfo` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetPendingTransmogCost` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetSecondarySlotState` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetSetSourcesForSlot` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetSlotGroupInfo` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetSourceIDsForSlot` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetTransmogOutfitSlotForInventoryType` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetTransmogOutfitSlotFromInventorySlot` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetUISituationCategoriesAndOptions` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetUnassignedAtlasForSlot` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetUnassignedDisplayAtlasForSlot` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetViewedOutfitSlotInfo` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.GetWeaponOptionsForSlot` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.HasPendingOutfitSituations` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.HasPendingOutfitTransmogs` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.IsEquippedGearOutfitDisplayed` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.IsEquippedGearOutfitLocked` | evidence-required | api | added | Current simulator models only local outfit-lock state for this query; local tests do not establish authoritative retail 12.0.0 semantics. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.IsLockedOutfit` | evidence-required | api | added | Current simulator models only local outfit-lock state for this query; local tests do not establish authoritative retail 12.0.0 semantics. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.IsSlotWeaponSlot` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.IsUsableDiscountAvailable` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.IsValidTransmogOutfitName` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.PickupOutfit` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ResetOutfitSituations` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.RevertPendingTransmog` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.SetOutfitSituationsEnabled` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.SetOutfitToCustomSet` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.SetOutfitToSet` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.SetPendingTransmog` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.SetSecondarySlotState` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.SetViewedWeaponOptionForSlot` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.SlotHasSecondary` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitEntryInfo` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitEntryInfo.icon` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitEntryInfo.isDisabled` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitEntryInfo.isEventOutfit` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitEntryInfo.name` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitEntryInfo.outfitID` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitEntryInfo.situationCategories` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitLinkedSlotInfo` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitLinkedSlotInfo.primarySlotInfo` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitLinkedSlotInfo.secondarySlotInfo` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitSlotGroup` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitSlotGroup.appearanceSlotInfo` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitSlotGroup.illusionSlotInfo` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitSlotGroup.position` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitSlotInfo` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitSlotInfo.collectionType` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitSlotInfo.isSecondary` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitSlotInfo.slot` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitSlotInfo.slotName` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitSlotInfo.type` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitWeaponCollectionInfo` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitWeaponCollectionInfo.canHaveIllusions` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitWeaponCollectionInfo.isWeapon` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitWeaponCollectionInfo.name` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitWeaponOptionInfo` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitWeaponOptionInfo.enabled` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitWeaponOptionInfo.name` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogOutfitWeaponOptionInfo.weaponOption` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationCategory` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationCategory.description` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationCategory.groupData` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationCategory.isRadioButton` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationCategory.name` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationCategory.triggerID` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationGroup` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationGroup.groupID` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationGroup.optionData` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationGroup.secondaryID` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationOption` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationOption.equipmentSetID` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationOption.loadoutID` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationOption.situationID` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationOption.specID` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationOptionData` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationOptionData.name` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationOptionData.option` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.TransmogSituationOptionData.value` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.UpdatePendingSituation` | evidence-required | api | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo` | evidence-required | structure | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.canTransmogrify` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.displayType` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.error` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.errorText` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.hasPending` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.isPendingCollected` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.isTransmogrified` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.texture` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.transmogID` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.warning` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogOutfitInfo.ViewedTransmogOutfitSlotInfo.warningText` | evidence-required | structure-field | added | Current simulator does not model this transmog-outfit API, structure, or field contract. Source metadata alone is insufficient; authoritative evidence or a correct modeled transmog-outfit subsystem/test is required, and no approval can close this row. |
| `C_TransmogSets.GetAvailableSets` | untriaged | api | added | api added in 12.0.0. |
| `C_TransmogSets.GetSetsFilter` | untriaged | api | added | api added in 12.0.0. |
| `C_TransmogSets.IsUsingDefaultSetsFilters` | untriaged | api | added | api added in 12.0.0. |
| `C_TransmogSets.SetDefaultSetsFilters` | untriaged | api | added | api added in 12.0.0. |
| `C_TransmogSets.SetSetsFilter` | untriaged | api | added | api added in 12.0.0. |
| `C_TransmogSets.TransmogSetInfo.grantAsPrecedingVariant` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_Tutorial.GetCombatEventInfo` | untriaged | api | added | api added in 12.0.0. |
| `C_UIWidgetManager.GetPreyHuntProgressWidgetVisualizationInfo` | untriaged | api | added | api added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.frameTextureKit` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.hasTimer` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.inAnimType` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.layoutDirection` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.modelSceneLayer` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.orderIndex` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.outAnimType` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.progressState` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.scriptedAnimationEffectID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.shownState` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.textureKit` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.tooltip` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.tooltipLoc` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.widgetScale` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.widgetSizeSetting` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UIWidgetManager.PreyHuntProgressWidgetVisualizationInfo.widgetTag` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_UnitAuras.AuraIsBigDefensive` | evidence-required | api | added | Source-register signatures/defaults and adjacent seeded aura lookup/state behavior do not establish the added 12.0.0 contract; authoritative semantics or a correct model/test are required, and no approval can close this row. |
| `C_UnitAuras.DoesAuraHaveExpirationTime` | evidence-required | api | added | Source-register signatures/defaults and adjacent seeded aura lookup/state behavior do not establish the added 12.0.0 contract; authoritative semantics or a correct model/test are required, and no approval can close this row. |
| `C_UnitAuras.GetAuraApplicationDisplayCount` | evidence-required | api | added | Source-register signatures/defaults and adjacent seeded aura lookup/state behavior do not establish the added 12.0.0 contract; authoritative semantics or a correct model/test are required, and no approval can close this row. |
| `C_UnitAuras.GetAuraBaseDuration` | evidence-required | api | added | Source-register signatures/defaults and adjacent seeded aura lookup/state behavior do not establish the added 12.0.0 contract; authoritative semantics or a correct model/test are required, and no approval can close this row. |
| `C_UnitAuras.GetAuraDispelTypeColor` | evidence-required | api | added | Source-register signatures/defaults and adjacent seeded aura lookup/state behavior do not establish the added 12.0.0 contract; authoritative semantics or a correct model/test are required, and no approval can close this row. |
| `C_UnitAuras.GetAuraDuration` | evidence-required | api | added | Source-register signatures/defaults and adjacent seeded aura lookup/state behavior do not establish the added 12.0.0 contract; authoritative semantics or a correct model/test are required, and no approval can close this row. |
| `C_UnitAuras.GetRefreshExtendedDuration` | evidence-required | api | added | Source-register signatures/defaults and adjacent seeded aura lookup/state behavior do not establish the added 12.0.0 contract; authoritative semantics or a correct model/test are required, and no approval can close this row. |
| `C_UnitAuras.GetUnitAuraInstanceIDs` | evidence-required | api | added | Source-register signatures/defaults and adjacent seeded aura lookup/state behavior do not establish the added 12.0.0 contract; authoritative semantics or a correct model/test are required, and no approval can close this row. |
| `C_UnitAuras.TriggerPrivateAuraShowDispelType` | evidence-required | api | added | Source-register signatures/defaults and adjacent seeded aura lookup/state behavior do not establish the added 12.0.0 contract; authoritative semantics or a correct model/test are required, and no approval can close this row. |
| `C_UnitAurasPrivate.SetShowDispelTypeCallback` | evidence-required | api | added | source register establishes secure-only/return-shape metadata only; current temporary model is permissive/partial, and secure/private-aura semantics remain unproven. |
| `C_WeeklyRewards.GetSortedProgressForActivity` | untriaged | api | added | api added in 12.0.0. |
| `C_WeeklyRewards.WeeklyRewardActivityTierProgress` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `C_WeeklyRewards.WeeklyRewardActivityTierProgress.activityTierID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_WeeklyRewards.WeeklyRewardActivityTierProgress.difficulty` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `C_WeeklyRewards.WeeklyRewardActivityTierProgress.numPoints` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `Constants.CAAConstants.CAAEnabledDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAFrequencyDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAFrequencyMax` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAFrequencyMin` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAInterruptCastDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAInterruptCastSuccessDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAMinCastTimeDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAMinCastTimeMax` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAMinCastTimeMin` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAMinCastTimeStep` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAPartyHealthPercentDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAPlayerCastFormatDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAPlayerCastModeDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAPlayerHealthFormatDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAPlayerHealthPercentDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAPlayerResourceFormatDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAPlayerResourcePercentDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAASampleTextThrottleTime` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAASayCombatEndDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAASayCombatStartDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAASayIfTargetedDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAATargetCastFormatDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAATargetCastModeDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAATargetDeathBehaviorDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAATargetHealthFormatDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAATargetHealthPercentDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAATargetNameDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAThrottleDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAThrottleMax` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAThrottleMin` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAThrottleStep` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CAAConstants.CAAVoiceDefault` | best-effort | constant | added | Best-effort behavioral evidence is limited to startup publication, exact Lua type, and exact value for this CAAConstants entry; CVar linkage, UI behavior, localization, mutation/protection, and consumer semantics are not claimed. |
| `Constants.CatalogShopVirtualCurrencyConstants.HEARTHSTEEL_VC_CURRENCY_CODE` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CatalogShopVirtualCurrencyConstants.TRADERS_TENDER_VC_CURRENCY_CODE` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CombatLogMessageLimits.CombatLogDefaultMessageLimit` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CombatLogMessageLimits.CombatLogMaximumMessageLimit` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CombatLogObjectMasks.COMBATLOG_OBJECT_AFFILIATION_MASK` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CombatLogObjectMasks.COMBATLOG_OBJECT_CONTROL_MASK` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CombatLogObjectMasks.COMBATLOG_OBJECT_REACTION_MASK` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CombatLogObjectMasks.COMBATLOG_OBJECT_SPECIAL_MASK` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CombatLogObjectMasks.COMBATLOG_OBJECT_TYPE_MASK` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CombatLogObjectTargetMasks.COMBATLOG_OBJECT_RAID_MASK` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CombatLogObjectTargetMasks.COMBATLOG_OBJECT_RAID_TARGET_MASK` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.CurrencyConsts.CURRENCY_WALLET_TYPE_WOWMONEY` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineEventConstants.ENCOUNTER_TIMELINE_INVALID_EVENT` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineEventConstants.ENCOUNTER_TIMELINE_RESERVED_EVENT_COUNT` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineIconMasks.EncounterTimelineAllIcons` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineIconMasks.EncounterTimelineDamageAlertIcons` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineIconMasks.EncounterTimelineDeadlyIcons` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineIconMasks.EncounterTimelineDispelIcons` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineIconMasks.EncounterTimelineEnrageIcons` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineIconMasks.EncounterTimelineHealerAlertIcons` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineIconMasks.EncounterTimelineNoIcons` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineIconMasks.EncounterTimelineOtherIcons` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineIconMasks.EncounterTimelineRoleIcons` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.EncounterTimelineIconMasks.EncounterTimelineTankAlertIcons` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.ItemConsts_Mainline.HWM_SQUISH_ERA_PLAYER_DATA_ACCOUNT_ELEMENT_ID` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.ProfessionConsts.CRAFTING_ORDER_CURRENCY_WALLET_ITEM_ID` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.TTSConstants.TTSRateDefault` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.TTSConstants.TTSRateMax` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.TTSConstants.TTSRateMin` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.TTSConstants.TTSVolumeDefault` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.TTSConstants.TTSVolumeMax` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.TTSConstants.TTSVolumeMin` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.TransmogOutfitDataConsts.EQUIP_TRANSMOG_OUTFIT_MANUAL_SPELL_ID` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.DeathKnight` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.DemonHunter` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Druid` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Evoker` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Hunter` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Mage` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Monk` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Paladin` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Priest` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Rogue` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Shaman` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Warlock` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UICharacterClasses.Warrior` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UnitEventConstants.MAX_UNIT_TOKENS_IN_EVENT` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UnitPowerSpellIDs.COLLAPSING_STAR_PASSIVE_SPELL_ID` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UnitPowerSpellIDs.COLLAPSING_STAR_SPELL_ID` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UnitPowerSpellIDs.DARK_HEART_SPELL_ID` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UnitPowerSpellIDs.SILENCE_THE_WHISPERS_SPELL_ID` | untriaged | constant | added | constant added in 12.0.0. |
| `Constants.UnitPowerSpellIDs.VOID_METAMORPHOSIS_SPELL_ID` | untriaged | constant | added | constant added in 12.0.0. |
| `Cooldown.GetCountdownFontString` | best-effort | uiobject-method | added | Focused test proves countdown child creation and FontString type; retail rendering/timing/formatting/edge semantics remain unproven. |
| `Cooldown.SetCooldownFromDurationObject` | best-effort | uiobject-method | added | Focused test proves duration-object timing update/display duration and zero-duration clearing; retail timing/rendering/formatting/edge semantics remain unproven. |
| `Cooldown.SetCooldownFromExpirationTime` | best-effort | uiobject-method | added | Focused test proves expiration-to-start/duration conversion and millisecond display duration; retail timing/rendering/formatting/edge semantics remain unproven. |
| `Cooldown.SetPaused` | evidence-required | uiobject-method | added | Evidence-required/unsafe: current simulator evidence covers Pause/Resume, not SetPaused. Exact pause-state setter behavior, interaction with active cooldowns, lifecycle, events, rendering, validation, and invalid-input semantics require authoritative evidence or a correct model/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `CraftingItemSlotModification.reagent` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingOrderReagentInfo.reagentInfo` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `CraftingQualityInfo.barBackground` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.barBackgroundCap` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.barFill` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.barHighlight` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.icon` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.iconAppear` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.iconChat` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.iconDissolve` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.iconInventory` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.iconMixed` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.iconSmall` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingQualityInfo.quality` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingReagentInfo.reagent` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingReagentSlotSchematic.variableQuantities` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingResourceReturnInfo.reagent` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingVariableQuantities` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `CraftingVariableQuantities.quantity` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CraftingVariableQuantities.reagent` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CreateAbbreviateConfig` | best-effort | api | added | Best-effort behavioral evidence covers factory/table proxy behavior, method dispatch, round-trip storage, per-instance isolation, read-only keys, and tostring; exact arrayof NumberAbbrevData structure fidelity is not established. |
| `CreateUnitHealPredictionCalculator` | best-effort | api | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `CriteriaRequiredValue` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `CriteriaRequiredValue.criteriaID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CriteriaRequiredValue.requiredValue` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CriteriaRequirement` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `CriteriaRequirement.completed` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `CriteriaRequirement.requirementText` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `DAMAGE_METER_COMBAT_SESSION_UPDATED` | untriaged | event | added | event added in 12.0.0. |
| `DAMAGE_METER_CURRENT_SESSION_UPDATED` | untriaged | event | added | event added in 12.0.0. |
| `DAMAGE_METER_RESET` | untriaged | event | added | event added in 12.0.0. |
| `ENCOUNTER_STATE_CHANGED` | untriaged | event | added | event added in 12.0.0. |
| `ENCOUNTER_TIMELINE_EVENT_ADDED` | untriaged | event | added | event added in 12.0.0. |
| `ENCOUNTER_TIMELINE_EVENT_BLOCK_STATE_CHANGED` | untriaged | event | added | event added in 12.0.0. |
| `ENCOUNTER_TIMELINE_EVENT_HIGHLIGHT` | untriaged | event | added | event added in 12.0.0. |
| `ENCOUNTER_TIMELINE_EVENT_REMOVED` | untriaged | event | added | event added in 12.0.0. |
| `ENCOUNTER_TIMELINE_EVENT_STATE_CHANGED` | untriaged | event | added | event added in 12.0.0. |
| `ENCOUNTER_TIMELINE_EVENT_TRACK_CHANGED` | untriaged | event | added | event added in 12.0.0. |
| `ENCOUNTER_TIMELINE_LAYOUT_UPDATED` | untriaged | event | added | event added in 12.0.0. |
| `ENCOUNTER_TIMELINE_STATE_UPDATED` | untriaged | event | added | event added in 12.0.0. |
| `ENCOUNTER_WARNING` | untriaged | event | added | event added in 12.0.0. |
| `Enum.AccountDataUpdateStatus.Corrupt` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AccountDataUpdateStatus.Failed` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AccountDataUpdateStatus.Success` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AccountDataUpdateStatus.Toobig` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AccountStateLoadedFlags.AccountCurrenciesLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.AccountFactionsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.AccountItemsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.AccountMappingLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.AccountNotificationsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.AccountWowlabsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.AchievementsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.ArchivedPurchasesLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.AuctionableTokensLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.BanktabSettingsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.BattleNetAccountLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.BitVectorsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.BpayAddLicenseObjectsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.BpayDistributionObjectsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.BpayProductitemObjectsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.CharacterItemsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.CharactersLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.CombinedQuestLogLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.ConsumableTokensLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.CriteriaLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.CurrencyCapsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.CurrencyTransferLogLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.DataElementsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.DynamicCriteriaLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.EventRecordsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.HousingDataLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.ItemCollectionsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.LgVendorPurchaseLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.LoadedNone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.MountsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.PerksHeldItemLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.PerksPastRewardsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.PerksPendingPurchaseLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.PerksPendingRewardsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.PetjournalInitialized` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.PurchasesLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.QuestCriteriaLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.QuestLogLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.RafActivityLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.RafBalanceLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.RafRewardsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.RevokedRafRewardsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.SettingsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.TransmogOutfitsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.TrialBoostHistoryLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.VasTransactionsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.WarbandScenesLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountStateLoadedFlags.WarbandsLoaded` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.AccountTransType.HouseInitiativeFavor` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.AccountTransType.TransmogOutfitCollection` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionState.Activating` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionState.Active` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionState.Inactive` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionStateMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionStateMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionStateMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionType.ChallengeMode` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionType.Combat` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionType.Encounter` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionType.Map` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionType.PvPMatch` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AddOnRestrictionTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AuraFrameVisibleSetting.Always` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AuraFrameVisibleSetting.Hidden` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AuraFrameVisibleSetting.InCombat` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AuraFrameVisibleSettingMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AuraFrameVisibleSettingMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.AuraFrameVisibleSettingMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResult.ResultFailed` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResult.ResultInProgress` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResult.ResultInsufficientFunds` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResult.ResultOk` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResult.ResultPartialSuccess` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResult.ResultPurchaseTimeout` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResult.ResultSystemDisabled` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResult.ResultTooManyProducts` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResultMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResultMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkPurchaseResultMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkRefundResult.ResultFailed` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkRefundResult.ResultInvalidRequest` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkRefundResult.ResultOk` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkRefundResult.ResultRefundWindowExpired` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkRefundResult.ResultSystemDisabled` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkRefundResult.ResultTimeout` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkRefundResultMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkRefundResultMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.BulkRefundResultMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ChatMessagingLockdownReason.ActiveEncounter` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ChatMessagingLockdownReason.ActiveMythicKeystoneOrChallengeMode` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ChatMessagingLockdownReason.ActivePvPMatch` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ChatMessagingLockdownReasonMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ChatMessagingLockdownReasonMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ChatMessagingLockdownReasonMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertCastState.Off` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertCastState.OnCastEnd` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertCastState.OnCastStart` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertCastStateMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertCastStateMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertCastStateMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Off` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Under100Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Under10Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Under20Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Under30Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Under40Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Under50Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Under60Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Under70Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Under80Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValues.Under90Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValuesMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValuesMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPartyPercentValuesMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPercentValues.Every10Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPercentValues.Every20Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPercentValues.Every30Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPercentValues.Every40Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPercentValues.Every50Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPercentValues.Off` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPercentValuesMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPercentValuesMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPercentValuesMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerCastFormatValues.Cast` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerCastFormatValues.CastSpellname` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerCastFormatValues.Casting` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerCastFormatValues.CastingSpellname` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerCastFormatValues.Spellname` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerCastFormatValuesMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerCastFormatValuesMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerCastFormatValuesMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerHealthFormatValues.HealthFull` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerHealthFormatValues.HealthNoPercent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerHealthFormatValues.HealthNoPercentDiv10` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerHealthFormatValues.NoHealthFull` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerHealthFormatValues.NoHealthNoPercent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerHealthFormatValues.NoHealthNoPercentDiv10` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerHealthFormatValuesMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerHealthFormatValuesMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerHealthFormatValuesMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerResourceFormatValues.NoResourceFull` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerResourceFormatValues.NoResourceNoPercent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerResourceFormatValues.NoResourceNoPercentDiv10` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerResourceFormatValues.ResourceFull` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerResourceFormatValues.ResourceNoPercent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerResourceFormatValues.ResourceNoPercentDiv10` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerResourceFormatValuesMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerResourceFormatValuesMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertPlayerResourceFormatValuesMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSayIfTargetedType.Aggro` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSayIfTargetedType.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSayIfTargetedType.Targeted` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSayIfTargetedType.TargetedBy` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSayIfTargetedTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSayIfTargetedTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSayIfTargetedTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSpecSetting.Resource1Format` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSpecSetting.Resource1Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSpecSetting.Resource2Format` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSpecSetting.Resource2Percent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSpecSetting.SayIfTargeted` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSpecSettingMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSpecSettingMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertSpecSettingMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetCastFormatValues.Cast` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetCastFormatValues.CastSpellname` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetCastFormatValues.Casting` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetCastFormatValues.CastingSpellname` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetCastFormatValues.Spellname` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetCastFormatValues.TargetCastSpellname` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetCastFormatValues.TargetCastingSpellname` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetCastFormatValuesMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetCastFormatValuesMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetCastFormatValuesMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetDeathBehavior.Default` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetDeathBehavior.SayTargetDead` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetDeathBehaviorMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetDeathBehaviorMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetDeathBehaviorMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValues.HealthFull` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValues.HealthNoPercent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValues.HealthNoPercentDiv10` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValues.NoHealthFull` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValues.NoHealthNoPercent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValues.NoHealthNoPercentDiv10` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValues.TargetFull` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValues.TargetNoPercent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValues.TargetNoPercentDiv10` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValuesMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValuesMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTargetHealthFormatValuesMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertThrottle.PlayerCast` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertThrottle.PlayerHealth` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertThrottle.PlayerResource1` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertThrottle.PlayerResource2` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertThrottle.Sample` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertThrottle.TargetCast` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertThrottle.TargetHealth` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertThrottleMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertThrottleMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertThrottleMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertType.Cast` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertType.Health` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertUnit.Player` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertUnit.Target` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertUnitMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertUnitMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatAudioAlertUnitMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogMessageOrder.Newest` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogMessageOrder.Oldest` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogMessageOrderMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogMessageOrderMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogMessageOrderMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.AffiliationMine` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.AffiliationOutsider` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.AffiliationParty` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.AffiliationRaid` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.ControlNpc` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.ControlPlayer` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.Empty` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.Focus` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.Mainassist` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.Maintank` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.None` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.ReactionFriendly` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.ReactionHostile` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.ReactionNeutral` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.Target` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.TypeGuardian` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.TypeNpc` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.TypeObject` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.TypePet` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObject.TypePlayer` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectMeta.MaxValue` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectMeta.MinValue` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectMeta.NumValues` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTarget.RaidNone` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTarget.Raidtarget1` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTarget.Raidtarget2` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTarget.Raidtarget3` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTarget.Raidtarget4` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTarget.Raidtarget5` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTarget.Raidtarget6` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTarget.Raidtarget7` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTarget.Raidtarget8` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTargetMeta.MaxValue` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTargetMeta.MinValue` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CombatLogObjectTargetMeta.NumValues` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CooldownViewerAddAlertStatus.DuplicateAlert` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, numeric Lua type, and exact 12.0.0 member value. Alert creation, validation, duplicate handling, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.CooldownViewerAddAlertStatus.InvalidAlertType` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, numeric Lua type, and exact 12.0.0 member value. Alert creation, validation, duplicate handling, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.CooldownViewerAddAlertStatus.InvalidEventType` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, numeric Lua type, and exact 12.0.0 member value. Alert creation, validation, duplicate handling, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.CooldownViewerAddAlertStatus.Success` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, numeric Lua type, and exact 12.0.0 member value. Alert creation, validation, duplicate handling, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.CooldownViewerAddAlertStatusMeta.MaxValue` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. Alert creation, validation, duplicate handling, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.CooldownViewerAddAlertStatusMeta.MinValue` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. Alert creation, validation, duplicate handling, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.CooldownViewerAddAlertStatusMeta.NumValues` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. Alert creation, validation, duplicate handling, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.CooldownViewerAlertEventType.Available` | best-effort | enum | added | Best-effort behavioral evidence covers exact retail 12.0.0 startup table membership and the source-register member value: the epoch override removes the two later members and restores four-member metadata. Event dispatch, alert consumers, lifecycle, mutation/protection, and other semantics are not claimed. |
| `Enum.CooldownViewerAlertEventType.ChargeGained` | best-effort | enum | added | Best-effort behavioral evidence covers exact retail 12.0.0 startup table membership and the source-register member value: the epoch override removes the two later members and restores four-member metadata. Event dispatch, alert consumers, lifecycle, mutation/protection, and other semantics are not claimed. |
| `Enum.CooldownViewerAlertEventType.OnCooldown` | best-effort | enum | added | Best-effort behavioral evidence covers exact retail 12.0.0 startup table membership and the source-register member value: the epoch override removes the two later members and restores four-member metadata. Event dispatch, alert consumers, lifecycle, mutation/protection, and other semantics are not claimed. |
| `Enum.CooldownViewerAlertEventType.PandemicTime` | best-effort | enum | added | Best-effort behavioral evidence covers exact retail 12.0.0 startup table membership and the source-register member value: the epoch override removes the two later members and restores four-member metadata. Event dispatch, alert consumers, lifecycle, mutation/protection, and other semantics are not claimed. |
| `Enum.CooldownViewerAlertEventTypeMeta.MaxValue` | best-effort | enum | added | Best-effort behavioral evidence covers the exact 12.0.0 startup metadata restored by the epoch override. The override also removes the two later event-type members. Event dispatch, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.CooldownViewerAlertEventTypeMeta.MinValue` | best-effort | enum | added | Best-effort behavioral evidence covers the exact 12.0.0 startup metadata restored by the epoch override. The override also removes the two later event-type members. Event dispatch, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.CooldownViewerAlertEventTypeMeta.NumValues` | best-effort | enum | added | Best-effort behavioral evidence covers the exact 12.0.0 startup metadata restored by the epoch override. The override also removes the two later event-type members. Event dispatch, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.CooldownViewerAlertType.Sound` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CooldownViewerAlertType.Visual` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CooldownViewerAlertTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CooldownViewerAlertTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CooldownViewerAlertTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderItemFlags.HasEnchantmentData` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderItemFlags.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderItemFlags.NpcProvided` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderItemFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderItemFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderItemFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderItemType.Currency` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderItemType.Deprecated` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderItemType.Item` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderResult.MissingCurrency` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CraftingOrderResult.TooManyCurrencies` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.CreateAllAccountData.AccountCurrenciesDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.AccountDynamicCriteriaDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.AccountFactionsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.AccountItemsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.AccountMappingDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.AccountNotificationsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.AccountStateHousingData` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.AchievementsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.ArchivedPurchasesDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.AuctionableTokensDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.BanktabSettingsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.BattlepetsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.BitVectorsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.BpayAddLicenseObjectsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.BpayDistributionObjectsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.BpayProductitemObjectsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.CharacterItemsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.CharactersDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.CombinedQuestLogEntriesDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.ConsumableTokensDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.CriteriaDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.CurrencyTransferLogDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.CurrencycapsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.DataElementsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.EventRecordsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.ItemCollectionItemsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.LgVendorPurchaseDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.MountsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.None` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.Object` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.PerkHeldItemsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.PerkPastRewardsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.PerkPendingPurchasesDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.PerkPendingRewardsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.PurchasesDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.QuestCriteriaDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.QuestLogDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.RafActivitiesDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.RafBalanceDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.RafRewardsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.RevokedRafRewardsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.SettingsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.TransmogOutfitsLoadedDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.TrialBoostHistoryDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.VasTransactionsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.WarbandGroupsDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.WarbandScenesLoadedDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CreateAllAccountData.WowlabsDataDone` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact Lua string type, and exact value for this added account-state enum entry. Bitflag combination semantics, consumers, mutation/protection, persistence, aliases, lifecycle, and other edge semantics are not claimed. |
| `Enum.CurrencyDestroyReason.CraftingOrderReagent` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.CurrencySource.InitiativeReward` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterNumbers.Compact` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterNumbers.Complete` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterNumbers.Minimal` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterNumbersMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterNumbersMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterNumbersMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterOverrideType.AllowFriendlyFire` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterOverrideType.Ignore` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterOverrideType.IgnoreForAbsorbSpell` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterOverrideType.RedirectSourceToAuraCaster` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterOverrideType.RedirectSourceToOwner` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterOverrideTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterOverrideTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterOverrideTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSessionType.Current` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSessionType.Expired` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSessionType.Overall` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSessionTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSessionTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSessionTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSpellDetailsDisplayType.SpellAffected` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSpellDetailsDisplayType.SpellCasted` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSpellDetailsDisplayType.UnitSpecificSpellCasted` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSpellDetailsDisplayTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSpellDetailsDisplayTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterSpellDetailsDisplayTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStorageType.Absorbs` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStorageType.AvoidableDamageTaken` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStorageType.Damage` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStorageType.DamageTaken` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStorageType.Dispels` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStorageType.HealingAndAbsorbs` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStorageType.Interrupts` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStorageTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStorageTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStorageTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterStyle.Bordered` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication and exact 12.0.0 member values. Damage-meter rendering, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.DamageMeterStyle.Default` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication and exact 12.0.0 member values. Damage-meter rendering, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.DamageMeterStyle.FullBackground` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication and exact 12.0.0 member values. Damage-meter rendering, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.DamageMeterStyle.Thin` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication and exact 12.0.0 member values. Damage-meter rendering, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.DamageMeterStyleMeta.MaxValue` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. Damage-meter rendering, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.DamageMeterStyleMeta.MinValue` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. Damage-meter rendering, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.DamageMeterStyleMeta.NumValues` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. Damage-meter rendering, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.DamageMeterType.Absorbs` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterType.AvoidableDamageTaken` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterType.DamageDone` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterType.DamageTaken` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterType.Dispels` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterType.Dps` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterType.HealingDone` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterType.Hps` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterType.Interrupts` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterVisibility.Always` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterVisibility.Hidden` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterVisibility.InCombat` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterVisibilityMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterVisibilityMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DamageMeterVisibilityMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlags.AutoEnd` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlags.Cosmetic` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlags.DisableEncounterEvents` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlags.GuildNews` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlags.HideUntilCompleted` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlags.IgnoreSpawnLimit` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlags.NoAutoStart` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlags.RaidLockPlayers` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlags.StickyNews` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlags.Unused` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterTriggerType.Invalid` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterTriggerType.OnComplete` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterTriggerType.OnEnd` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterTriggerType.OnStart` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterTriggerType.PreviouslyCompleted` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterTriggerTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterTriggerTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterTriggerTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterXCreatureFlags.BossCreature` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterXCreatureFlags.DoNotDespawnOnSuccess` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterXCreatureFlags.DropLootImmediately` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterXCreatureFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterXCreatureFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DungeonEncounterXCreatureFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DurationTimeModifier.BaseTime` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DurationTimeModifier.RealTime` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DurationTimeModifierMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DurationTimeModifierMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.DurationTimeModifierMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeAccountSetting.ShowDamageMeter` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeAccountSetting.ShowEncounterEvents` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeAccountSetting.ShowExternalDefensives` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeAccountSetting.ShowPersonalResourceDisplay` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeAuraFrameSetting.Opacity` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeAuraFrameSetting.ShowDispelType` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeAuraFrameSetting.VisibleSetting` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeAuraFrameSystemIndices.ExternalDefensivesFrame` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.EditModeCooldownViewerSetting.BarWidthScale` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.BackgroundTransparency` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.BarHeight` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.FrameHeight` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.FrameWidth` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.Numbers` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.ObsoleteReuse1` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.Padding` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.ShowClassColor` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.ShowSpecIcon` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.Style` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.TextSize` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.Transparency` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSetting.Visibility` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSettingMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSettingMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeDamageMeterSettingMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSetting.BackgroundTransparency` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSetting.IconDirection` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSetting.IconSize` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSetting.Orientation` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSetting.OverallSize` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSetting.ShowSpellName` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSetting.ShowTimer` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSetting.ShowTooltips` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSetting.Transparency` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSetting.Visibility` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSettingMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSettingMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSettingMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeEncounterEventsSystemIndices.CriticalWarnings` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication and exact 12.0.0 member values. Edit Mode system selection, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.EditModeEncounterEventsSystemIndices.MediumWarnings` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication and exact 12.0.0 member values. Edit Mode system selection, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.EditModeEncounterEventsSystemIndices.NormalWarnings` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication and exact 12.0.0 member values. Edit Mode system selection, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.EditModeEncounterEventsSystemIndices.Timeline` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication and exact 12.0.0 member values. Edit Mode system selection, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.EditModeEncounterEventsSystemIndicesMeta.MaxValue` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. Edit Mode system selection, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.EditModeEncounterEventsSystemIndicesMeta.MinValue` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. Edit Mode system selection, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.EditModeEncounterEventsSystemIndicesMeta.NumValues` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. Edit Mode system selection, layout, persistence, consumer behavior, lifecycle, and other semantics are not claimed. |
| `Enum.EditModePersonalResourceDisplaySetting.HideHealthAndPower` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModePersonalResourceDisplaySetting.OnlyShowInCombat` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModePersonalResourceDisplaySettingMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModePersonalResourceDisplaySettingMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModePersonalResourceDisplaySettingMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeSystem.DamageMeter` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeSystem.EncounterEvents` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeSystem.PersonalResourceDisplay` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeUnitFrameSetting.AuraOrganizationType` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeUnitFrameSetting.IconSize` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EditModeUnitFrameSetting.Opacity` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventCastState.Casting` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventCastState.Expired` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventCastState.NotCasting` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventCastStateMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventCastStateMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventCastStateMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventFlags.Disabled` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventFlagsMeta.MaxValue` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventFlagsMeta.MinValue` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventFlagsMeta.NumValues` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmask.BleedEffect` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmask.CurseEffect` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmask.DeadlyEffect` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmask.DiseaseEffect` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmask.DpsRole` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmask.EnrageEffect` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmask.HealerRole` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmask.MagicEffect` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmask.PoisonEffect` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmask.TankRole` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmaskMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmaskMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventIconmaskMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventSeverity.High` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventSeverity.Low` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventSeverity.Medium` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventSeverityMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventSeverityMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventSeverityMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsIconDirection.Bottom` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsIconDirection.Left` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsIconDirection.Right` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsIconDirection.Top` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsIconDirectionMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsIconDirectionMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsIconDirectionMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsOrientation.Horizontal` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsOrientation.Vertical` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsOrientationMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsOrientationMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsOrientationMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsVisibility.Always` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsVisibility.DeprecatedHidden` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsVisibility.InEncounter` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsVisibilityMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsVisibilityMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterEventsVisibilityMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSortDirection.Ascending` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSortDirection.Descending` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSortDirectionMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSortDirectionMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSortDirectionMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSource.EditMode` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSource.Encounter` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSource.Script` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSourceMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSourceMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventSourceMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventState.Active` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventState.Canceled` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventState.Finished` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventState.Paused` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventStateMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventStateMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineEventStateMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineIconSet.DamageAlert` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineIconSet.Deadly` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineIconSet.Dispel` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineIconSet.Enrage` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineIconSet.HealerAlert` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineIconSet.TankAlert` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineIconSetMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineIconSetMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineIconSetMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrack.Indeterminate` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrack.Long` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrack.Medium` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrack.Queued` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrack.Short` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrackMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrackMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrackMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrackType.Hidden` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrackType.Linear` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrackType.Sorted` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrackTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrackTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.EncounterTimelineTrackTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.BattleForAzeroth` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.BurningCrusade` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.Cataclysm` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.Draenor` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.Dragonflight` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.LastTitan` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.Legion` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.Midnight` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.MistsOfPandaria` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.Northrend` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.Shadowlands` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevel.WarWithin` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevelMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevelMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ExpansionLevelMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FontStringScaleAnimationMode.FontSize` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FontStringScaleAnimationMode.Vertex` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FontStringScaleAnimationModeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FontStringScaleAnimationModeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FontStringScaleAnimationModeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FragmentID.FNeighborhoodStateData` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FragmentID.FPlayerInitiativeInfo` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FragmentID.FUnitAIGroupLink` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FrameTutorialAccount.TransmogCustomSets` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FrameTutorialAccount.TransmogOutfits` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FrameTutorialAccount.TransmogSets` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FrameTutorialAccount.TransmogSituations` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.FrameTutorialAccount.TransmogWeaponOptions` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.GameRule.EjJourneysDisabled` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.GameRule.PvPInitialRatingOverride` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.GossipNpcOption.TieredEntrance` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, exact numeric Lua type, and exact 12.0.0 value. The paired old-key absence assertion is current-publication evidence only and does not prove historical removal timing, alias semantics, or full-LoD dynamic absence. |
| `Enum.HouseExteriorWMODataFlags.AllowedInAllianceNeighborhoods` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, numeric Lua type, and exact 12.0.0 bit-flag values. Bitwise composition, neighborhood availability, housing consumers, server state, lifecycle, and other semantics are not claimed. |
| `Enum.HouseExteriorWMODataFlags.AllowedInHordeNeighborhoods` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, numeric Lua type, and exact 12.0.0 bit-flag values. Bitwise composition, neighborhood availability, housing consumers, server state, lifecycle, and other semantics are not claimed. |
| `Enum.HouseExteriorWMODataFlags.None` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, numeric Lua type, and exact 12.0.0 bit-flag values. Bitwise composition, neighborhood availability, housing consumers, server state, lifecycle, and other semantics are not claimed. |
| `Enum.HouseExteriorWMODataFlags.UnlockedByDefault` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, numeric Lua type, and exact 12.0.0 bit-flag values. Bitwise composition, neighborhood availability, housing consumers, server state, lifecycle, and other semantics are not claimed. |
| `Enum.HouseExteriorWMODataFlagsMeta.MaxValue` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. MaxValue is the highest declared flag, MinValue is the zero sentinel, and NumValues is the declared-entry count; bitwise composition, housing consumers, server state, lifecycle, and other semantics are not claimed. |
| `Enum.HouseExteriorWMODataFlagsMeta.MinValue` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. MaxValue is the highest declared flag, MinValue is the zero sentinel, and NumValues is the declared-entry count; bitwise composition, housing consumers, server state, lifecycle, and other semantics are not claimed. |
| `Enum.HouseExteriorWMODataFlagsMeta.NumValues` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup metadata publication and exact 12.0.0 value. MaxValue is the highest declared flag, MinValue is the zero sentinel, and NumValues is the declared-entry count; bitwise composition, housing consumers, server state, lifecycle, and other semantics are not claimed. |
| `Enum.HousingDecorActionFlags.PreviewDecor` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingDecorPlacementRestriction.ChildOutsideBounds` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingDecorPlacementRestriction.OutsidePlotBounds` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingDecorPlacementRestriction.OutsideRoomBounds` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSource.DecorCollection` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSource.DeferredRewards` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSource.InitiativeChest` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSource.InitiativeTask` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSource.NewHouseDecorFavor` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSource.Quest` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSource.RetroactiveDecor` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSource.Unknown` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSourceMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSourceMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateSourceMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateType.Add` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateType.InitiativeAdd` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateType.Set` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingFavorUpdateTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingItemToastType.House` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.BoundsFailureChildren` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.BoundsFailurePlot` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.BoundsFailureRoom` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.HouseExteriorAlreadyThatSize` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.HouseExteriorAlreadyThatType` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.HouseExteriorSizeNotAvailable` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.HouseExteriorTypeNeighborhoodMismatch` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.HouseExteriorTypeNotFound` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.HouseExteriorTypeSizeMismatch` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.MaxPreviewDecorReached` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.OwnerNotInGuild` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.UncollectedExteriorFixture` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.UncollectedHouseType` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.UncollectedRoom` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.UncollectedRoomMaterial` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingResult.UncollectedRoomTheme` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.HousingRoomComponentFlags.HiddenInLayoutMode` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingRoomComponentFlags.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingRoomComponentFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingRoomComponentFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.HousingRoomComponentFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.InitiativeMilestoneFlags.FinalMilestone` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.InitiativeMilestoneFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.InitiativeMilestoneFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.InitiativeMilestoneFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.InitiativeRewardFlags.PermanentWorldState` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.InitiativeRewardFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.InitiativeRewardFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.InitiativeRewardFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ItemCollectionType.ExteriorFixture` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.Heirloom` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.HouseType` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.None` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.Room` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.RoomMaterial` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.RoomTheme` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.RuneforgeLegendaryAbility` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.Toy` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.Transmog` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.TransmogIllusion` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.TransmogOutfit` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.TransmogSetFavorite` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionType.WarbandScene` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCreationContext.TimewalkerLevelUp` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ItemCreationContext.TimewalkerMaxLevel` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ItemRecraftFlags.Invalid` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, exact numeric Lua type, and exact 12.0.0 value. The paired old-key absence assertion is current-publication evidence only and does not prove historical removal timing, alias semantics, or full-LoD dynamic absence. |
| `Enum.LFGEntryGeneralPlaystyle.Expert` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LFGEntryGeneralPlaystyle.FunRelaxed` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LFGEntryGeneralPlaystyle.FunSerious` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LFGEntryGeneralPlaystyle.Learning` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LFGEntryGeneralPlaystyle.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LFGEntryGeneralPlaystyleMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LFGEntryGeneralPlaystyleMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LFGEntryGeneralPlaystyleMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LimitedInputType.MouseDown` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LimitedInputType.MouseMove` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LimitedInputType.MouseUp` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LimitedInputType.MouseWheel` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LimitedInputTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LimitedInputTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LimitedInputTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LuaCurveType.Cosine` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LuaCurveType.Cubic` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LuaCurveType.Linear` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LuaCurveType.Step` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LuaCurveTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LuaCurveTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.LuaCurveTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.MapIconUIWidgetSetType.AdventureMapDetails` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateCastBarDisplay.HighlightImportantCasts` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateCastBarDisplay.HighlightWhenCastTarget` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateCastBarDisplay.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateCastBarDisplay.SpellIcon` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateCastBarDisplay.SpellName` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateCastBarDisplay.SpellTarget` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateCastBarDisplayMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateCastBarDisplayMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateCastBarDisplayMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyNpcAuraDisplay.Buffs` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyNpcAuraDisplay.CrowdControl` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyNpcAuraDisplay.Debuffs` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyNpcAuraDisplay.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyNpcAuraDisplayMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyNpcAuraDisplayMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyNpcAuraDisplayMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyPlayerAuraDisplay.Buffs` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyPlayerAuraDisplay.Debuffs` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyPlayerAuraDisplay.LossOfControl` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyPlayerAuraDisplay.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyPlayerAuraDisplayMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyPlayerAuraDisplayMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateEnemyPlayerAuraDisplayMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateFriendlyPlayerAuraDisplay.Buffs` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateFriendlyPlayerAuraDisplay.Debuffs` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateFriendlyPlayerAuraDisplay.LossOfControl` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateFriendlyPlayerAuraDisplay.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateFriendlyPlayerAuraDisplayMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateFriendlyPlayerAuraDisplayMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateFriendlyPlayerAuraDisplayMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateInfoDisplay.CurrentHealthPercent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateInfoDisplay.CurrentHealthValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateInfoDisplay.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateInfoDisplay.RarityIcon` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateInfoDisplayMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateInfoDisplayMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateInfoDisplayMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSimplifiedType.FriendlyNpc` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSimplifiedType.FriendlyPlayer` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSimplifiedType.Minion` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSimplifiedType.MinusMob` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSimplifiedType.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSimplifiedTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSimplifiedTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSimplifiedTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSize.ExtraLarge` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSize.Huge` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSize.Large` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSize.Medium` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSize.Small` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSizeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSizeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateSizeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStackType.Enemy` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStackType.Friendly` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStackType.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStackTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStackTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStackTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStyle.Block` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStyle.CastFocus` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStyle.HealthFocus` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStyle.Legacy` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStyle.Modern` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStyle.Thin` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStyleMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStyleMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateStyleMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateThreatDisplay.Flash` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateThreatDisplay.HealthBarColor` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateThreatDisplay.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateThreatDisplay.Progressive` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateThreatDisplayMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateThreatDisplayMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateThreatDisplayMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateType.Enemy` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateType.Friendly` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NamePlateTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeChestResult.NiNoHouseFound` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeChestResult.NiNoRewards` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeChestResult.NiServiceDisabled` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeChestResult.NiSuccess` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeChestResult.NiThrottled` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeChestResult.NiUnspecifiedFailure` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeChestResultMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeChestResultMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeChestResultMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeFlags.Disabled` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeFlags.NoAbandon` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeFlags.NoRepeat` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeNeighborhoodTypes.NiNeighborhoodTypePool` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeNeighborhoodTypes.NiNeighborhoodTypeSingleton` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeNeighborhoodTypesMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeNeighborhoodTypesMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeNeighborhoodTypesMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeTaskType.RepeatableFinite` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeTaskType.RepeatableInfinite` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeTaskType.Single` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeTaskTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeTaskTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeTaskTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeUpdateStatus.Completed` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeUpdateStatus.Failed` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeUpdateStatus.MilestoneCompleted` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeUpdateStatus.Started` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeUpdateStatusMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeUpdateStatusMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativeUpdateStatusMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativesCompletionStates.NiCompletionStateNotCompleted` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativesCompletionStates.NiCompletionStatePlayerCompleted` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativesCompletionStates.NiCompletionStateSystemAbandoned` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativesCompletionStatesMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativesCompletionStatesMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NeighborhoodInitiativesCompletionStatesMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NpcCraftingOrderSetFlags.AllowDuplicate` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.NpcCraftingOrderSetFlags.AllowMultiple` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PerksVendorCategoryType.RefundUnused` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, exact numeric Lua type, and exact 12.0.0 value. The paired old-key absence assertion is current-publication evidence only and does not prove historical removal timing, alias semantics, or full-LoD dynamic absence. |
| `Enum.PlayerCompanionInfoFlags.IgnoreSeasonInScenarios` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PlayerCompanionInfoFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PlayerCompanionInfoFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PlayerCompanionInfoFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PlayerInteractionType.TieredEntrance` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, exact numeric Lua type, and exact 12.0.0 value. The paired old-key absence assertion is current-publication evidence only and does not prove historical removal timing, alias semantics, or full-LoD dynamic absence. |
| `Enum.PreyHuntProgressState.Cold` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PreyHuntProgressState.Final` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PreyHuntProgressState.Hot` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PreyHuntProgressState.Warm` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PreyHuntProgressStateMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PreyHuntProgressStateMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.PreyHuntProgressStateMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnInteractionMode.Manipulate` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnInteractionMode.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnInteractionMode.Paint` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnInteractionModeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnInteractionModeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnInteractionModeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnVolumeChunkFlags.AllSubChunksSet` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnVolumeChunkFlags.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnVolumeChunkFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnVolumeChunkFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.ProceduralSpawnVolumeChunkFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.QuestTagType.Prey` | best-effort | enum | added | Best-effort behavioral evidence is limited to current startup namespace publication, exact numeric Lua type, and exact 12.0.0 value. The paired old-key absence assertion is current-publication evidence only and does not prove historical removal timing, alias semantics, or full-LoD dynamic absence. |
| `Enum.RaidAuraOrganizationType.BuffsRightDebuffsLeft` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidAuraOrganizationType.BuffsTopDebuffsBottom` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidAuraOrganizationType.Legacy` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidAuraOrganizationTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidAuraOrganizationTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidAuraOrganizationTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidDispelDisplayType.Disabled` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidDispelDisplayType.DispellableByMe` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidDispelDisplayType.DisplayAll` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidDispelDisplayTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidDispelDisplayTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RaidDispelDisplayTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RcoCloseReason.Cancel` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RcoCloseReason.CrafterFulfill` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RcoCloseReason.Expire` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RcoCloseReason.Fulfill` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RcoCloseReason.GmCancel` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RcoCloseReason.Invalid` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RcoCloseReason.Reject` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayType.Currency` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayType.GarrFollower` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayType.Item` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayType.Mount` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayType.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayType.Spell` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayType.Title` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayType.Transmog` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayType.TransmogIllusion` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayType.TransmogSet` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardDisplayTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardsFlags.AccountUnlock` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardsFlags.Capstone` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardsFlags.Hidden` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardsFlags.Milestone` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardsFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardsFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.RenownRewardsFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SecrecyLevel.AlwaysSecret` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SecrecyLevel.ContextuallySecret` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SecrecyLevel.NeverSecret` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SecrecyLevelMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SecrecyLevelMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SecrecyLevelMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SecretAspect.Alpha` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.BarValue` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.Cooldown` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.Cursor` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.Desaturation` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.FrameLevel` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.Hierarchy` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.ID` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.MinimumWidth` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.ObjectDebug` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.ObjectName` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.ObjectSecrets` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.ObjectSecurity` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.ObjectType` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.Padding` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.Rotation` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.Scale` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.ScrollRange` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.SecureText` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.Shown` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.TexCoords` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.Text` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.Toplevel` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspect.VertexColor` | best-effort | enum | added | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.SecretAspectMeta.MaxValue` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.SecretAspectMeta.MinValue` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.SecretAspectMeta.NumValues` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.SendAddonMessageResult.AddOnMessageLockdown` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SendAddonMessageResult.TargetOffline` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SimpleOrderStatus.Creating` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SimpleOrderStatus.Failed` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SimpleOrderStatus.InProgress` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SimpleOrderStatus.Invalid` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SimpleOrderStatus.Success` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SimpleOrderStatusMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SimpleOrderStatusMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SimpleOrderStatusMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SleevesGeoRange.Default` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SleevesGeoRange.Flared` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SleevesGeoRange.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SleevesGeoRange.PandaCollar` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SleevesGeoRange.Puffy` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SleevesGeoRangeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SleevesGeoRangeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SleevesGeoRangeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellAuraVisibilityType.EnemyTarget` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellAuraVisibilityType.RaidInCombat` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellAuraVisibilityType.RaidOutOfCombat` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellAuraVisibilityTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellAuraVisibilityTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellAuraVisibilityTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategory.AoEKnockback` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategory.Disarm` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategory.Disorient` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategory.Incapacitate` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategory.Root` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategory.Silence` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategory.Stun` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategory.Taunt` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategoryMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategoryMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishCategoryMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishRuleset.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishRuleset.PvE` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishRuleset.PvP` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishRulesetMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishRulesetMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SpellDiminishRulesetMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarFillStyle.Center` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarFillStyle.Reverse` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarFillStyle.Standard` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarFillStyle.StandardNoRangeFill` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarFillStyleMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarFillStyleMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarFillStyleMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarInterpolation.ExponentialEaseOut` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarInterpolation.Immediate` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarInterpolationMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarInterpolationMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarInterpolationMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarTimerDirection.ElapsedTime` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarTimerDirection.RemainingTime` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarTimerDirectionMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarTimerDirectionMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.StatusBarTimerDirectionMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.SurveyDeliveryMoment.MythicPlusCompleted` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TableSecurityOption.DisallowSecretKeys` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TableSecurityOption.DisallowTaintedAccess` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TableSecurityOption.SecretWrapContents` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TableSecurityOptionMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TableSecurityOptionMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TableSecurityOptionMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TooltipDataLineType.SpellDescription` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TooltipDataLineType.SpellPassive` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TooltipDataType.Outfit` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TraitNodeEntryType.SpendCapstoneCircle` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TraitNodeEntryType.SpendCapstoneSquare` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TraitNodeFlag.ShowTierTrack` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDataFlags.IsCachedLocally` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDataFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDataFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDataFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDisplayType.Assigned` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDisplayType.Equipped` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDisplayType.Hidden` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDisplayType.Unassigned` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDisplayTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDisplayTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitDisplayTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntryFlags.AutomaticallyAwardedOnLogin` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntryFlags.OnlyAvailableDuringEvent` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntryFlags.SortedToTopOfList` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntryFlags.UseOverrideCostModifier` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntryFlags.UseOverrideName` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntryFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntryFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntryFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntrySource.AutomaticallyAwarded` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntrySource.PlayerPurchased` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntrySource.StampedSource` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntrySourceMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntrySourceMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEntrySourceMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEquipAction.Equip` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEquipAction.EquipAndLock` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEquipAction.Lock` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEquipAction.Remove` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEquipAction.RemoveAndLock` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEquipAction.Unlock` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEquipActionMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEquipActionMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitEquipActionMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSetType.CustomSet` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSetType.Equipped` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSetType.Outfit` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSetTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSetTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSetTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlot.Back` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.Body` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.Chest` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.Feet` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.Hand` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.Head` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.Legs` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.ShoulderLeft` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.ShoulderRight` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.Tabard` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.Waist` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.WeaponMainHand` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.WeaponOffHand` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.WeaponRanged` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlot.Wrist` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.CannotUseItem` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.IncompatibleWithMainHand` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.InvalidDestination` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.InvalidItemType` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.InvalidSlotForForm` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.InvalidSlotForRace` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.InvalidSource` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.InvalidSourceQuality` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.Legendary` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.Mismatch` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.NoIllusion` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.NoItem` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.NotSoulbound` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.Ok` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotError.SameItem` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotErrorMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotErrorMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotErrorMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotFlags.CanHaveIllusions` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotFlags.CannotBeHidden` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotFlags.IsSecondarySlot` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotOption.ArtifactSpecFour` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.ArtifactSpecOne` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.ArtifactSpecThree` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.ArtifactSpecTwo` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.DeprecatedReuseMe` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.FuryTwoHandedWeapon` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.None` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.OffHand` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.OneHandedWeapon` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.RangedWeapon` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.Shield` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOption.TwoHandedWeapon` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitSlotOptionFlags.DisablesOffhandSlot` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotOptionFlags.DynamicOptionName` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotOptionFlags.IllusionNotAllowed` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotOptionFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotOptionFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotOptionFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotOptionMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotOptionMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotOptionMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotPosition.Bottom` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotPosition.Left` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotPosition.Right` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotPositionMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotPositionMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotPositionMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotSaveFlags.AppearanceIsNotValid` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotSaveFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotSaveFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotSaveFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotWarning.InvalidEquippedDestinationItem` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotWarning.NothingEquipped` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotWarning.Ok` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotWarning.PendingWeaponChanges` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotWarning.WeaponDoesNotSupportIllusions` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotWarning.WrongWeaponCategoryEquipped` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotWarningMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotWarningMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitSlotWarningMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionFlags.AddNewOutfitMask` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitTransactionFlags.AddOutfitAndUpdateSlots` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitTransactionFlags.CreateAndUpdateOutfitInfoMask` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitTransactionFlags.CreateOutfitInfo` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitTransactionFlags.FullOutfitUpdateMask` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitTransactionFlags.UpdateMetadata` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitTransactionFlags.UpdateOutfitInfo` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitTransactionFlags.UpdateSituations` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitTransactionFlags.UpdateSituationsMask` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitTransactionFlags.UpdateSlots` | best-effort | enum | added | Best-effort behavioral evidence is limited to startup namespace publication, exact numeric Lua type, and exact value; transmog operations, validation, slot compatibility, transaction semantics, persistence, mutation/protection, lifecycle, and other edge semantics are not claimed. |
| `Enum.TransmogOutfitTransactionFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionType.CreateOutfitInfo` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionType.UpdateMetadata` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionType.UpdateOutfitInfo` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionType.UpdateSituations` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionType.UpdateSlots` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogOutfitTransactionTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.AllEquipmentSets` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.AllLocations` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.AllMovement` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.AllRacialForms` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.AllSpecs` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.EquipmentSets` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.FormNative` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.FormNonNative` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.LocationArenas` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.LocationBattlegrounds` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.LocationCharacterSelect` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.LocationDelves` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.LocationDungeons` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.LocationHouse` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.LocationRaids` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.LocationRested` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.LocationWorld` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.MovementFlyingMount` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.MovementGroundMount` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.MovementSwimming` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.MovementUnmounted` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituation.Spec` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationFlags.AllSituation` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationFlags.DefaultsToOn` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationFlags.DisabledSituation` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationFlags.DynamicallyNamed` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationFlags.IsPlayerFacing` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationFlags.NoneSituation` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationFlags.SpecUseTalentLoadout` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationGroupFlags.DynamicallyCreatesGroups` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationGroupFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationGroupFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationGroupFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationMeta.MaxValue` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationMeta.MinValue` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationMeta.NumValues` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTrigger.EquipmentSet` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTrigger.EventOutfit` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTrigger.Forms` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTrigger.Location` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTrigger.Manual` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTrigger.Movement` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTrigger.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTrigger.Specialization` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTrigger.TransmogUpdate` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerFlags.CanChangeLockedOutfit` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerFlags.CanLockOutfit` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerFlags.DisabledTrigger` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerFlags.IsPlayerFacing` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerFlags.SituationsAreExclusive` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerType.Automatic` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerType.Manual` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerType.None` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerType.TransmogUpdate` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerTypeMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerTypeMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.TransmogSituationTriggerTypeMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UICovenantDisplayInfoFlags.DisplayCovenantAsJourney` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UICovenantDisplayInfoFlags.UseJourneyRewardTrack` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UICovenantDisplayInfoFlagsMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UICovenantDisplayInfoFlagsMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UICovenantDisplayInfoFlagsMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UICursorType.Outfit` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.UIWidgetVisualizationType.PreyHuntProgress` | best-effort | enum | added | enum added in 12.0.0. |
| `Enum.UnitAuraSortDirection.Normal` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UnitAuraSortDirection.Reverse` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UnitAuraSortDirectionMeta.MaxValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UnitAuraSortDirectionMeta.MinValue` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UnitAuraSortDirectionMeta.NumValues` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UnitAuraSortRule.BigDefensive` | untriaged | enum | added | enum added in 12.0.0. |
| `Enum.UnitAuraSortRule.Default` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 1; aura sorting and consumer semantics are not claimed. |
| `Enum.UnitAuraSortRule.Expiration` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 3; aura sorting and consumer semantics are not claimed. |
| `Enum.UnitAuraSortRule.ExpirationOnly` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 4; aura sorting and consumer semantics are not claimed. |
| `Enum.UnitAuraSortRule.Name` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 5; aura sorting and consumer semantics are not claimed. |
| `Enum.UnitAuraSortRule.NameOnly` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 6; aura sorting and consumer semantics are not claimed. |
| `Enum.UnitAuraSortRule.Unsorted` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 0; aura sorting and consumer semantics are not claimed. |
| `Enum.UnitAuraSortRuleMeta.MaxValue` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 6; aura sorting and consumer semantics are not claimed. |
| `Enum.UnitAuraSortRuleMeta.MinValue` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 0; aura sorting and consumer semantics are not claimed. |
| `Enum.UnitAuraSortRuleMeta.NumValues` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 7; aura sorting and consumer semantics are not claimed. |
| `Enum.UnitDamageAbsorbClampMode.MaximumHealth` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 2; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitDamageAbsorbClampMode.MissingHealth` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 0; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitDamageAbsorbClampMode.MissingHealthWithoutIncomingHeals` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 1; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitDamageAbsorbClampModeMeta.MaxValue` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 2; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitDamageAbsorbClampModeMeta.MinValue` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 0; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitDamageAbsorbClampModeMeta.NumValues` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 3; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitHealAbsorbClampMode.CurrentHealth` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 0; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitHealAbsorbClampMode.MaximumHealth` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 1; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitHealAbsorbClampModeMeta.MaxValue` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 1; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitHealAbsorbClampModeMeta.MinValue` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 0; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitHealAbsorbClampModeMeta.NumValues` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 2; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitHealAbsorbMode.ReducedByIncomingHeals` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 0; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitHealAbsorbMode.Total` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 1; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitHealAbsorbModeMeta.MaxValue` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 1; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitHealAbsorbModeMeta.MinValue` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 0; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitHealAbsorbModeMeta.NumValues` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 2; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitIncomingHealClampMode.MaximumHealth` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 1; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitIncomingHealClampMode.MissingHealth` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 0; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitIncomingHealClampModeMeta.MaxValue` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 1; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitIncomingHealClampModeMeta.MinValue` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 0; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.UnitIncomingHealClampModeMeta.NumValues` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace/metadata publication, numeric Lua type, and exact value 2; heal prediction and absorb/clamp semantics are not claimed. |
| `Enum.VasTransactionPurchaseResult.DbHouseOwnerRestriction` | best-effort | enum | added | Behavioral proof is limited to current 12.0.0 startup namespace publication, numeric Lua type, and exact value 20096; VAS transaction and consumer semantics are not claimed. |
| `ExpansionDisplayInfo.glueAmbianceSoundKit` | evidence-required | structure-field | added | Current implementation is absent, nil-only, or generic/placeholder-backed and does not establish the exact field contract, state, security, or consumer semantics. |
| `ExpansionDisplayInfo.glueCreditsSoundKit` | evidence-required | structure-field | added | Current implementation is absent, nil-only, or generic/placeholder-backed and does not establish the exact field contract, state, security, or consumer semantics. |
| `ExpansionDisplayInfo.glueMusicSoundKit` | evidence-required | structure-field | added | Current implementation is absent, nil-only, or generic/placeholder-backed and does not establish the exact field contract, state, security, or consumer semantics. |
| `FACTION_STANDING_CHANGED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `factionID: number, updatedStanding: number`, timing, lifecycle, ordering, or duplicate behavior. |
| `FontString.GetScaleAnimationMode` | evidence-required | uiobject-method | added | Evidence-required/unsafe: no matching simulator implementation or focused test was found. Exact return-value/default behavior, FontStringScaleAnimationMode mapping, validation, persistence, animation, layout, rendering, and edge semantics require authoritative evidence or a correct model/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `FontString.SetScaleAnimationMode` | evidence-required | uiobject-method | added | Evidence-required/unsafe: no matching simulator implementation or focused test was found. Accepted modes, validation/errors, state persistence, animation behavior, layout/render effects, and edge semantics require authoritative evidence or a correct model/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `Frame.IsIgnoringChildrenForBounds` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to the focused false/true/false stored-state round-trip for IsIgnoringChildrenForBounds. Actual bounds exclusion, layout effects, rendering, invalid arguments, lifecycle, and other edge semantics remain unproven. Implementation ancestor is 7a7a440402; test-file ancestor is e7fb98b910. |
| `Frame.RegisterEventCallback` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to registering MINIMAP_PING and receiving it through the focused FireEvent/OnEvent dispatch path. Complete callback-event allowlists, return values, restricted-event behavior, lifecycle, validation, taint/security, and other edge semantics remain unproven. |
| `Frame.RegisterUnitEventCallback` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to a frame-owned UNIT_HEALTH callback filtered to player: registration succeeds, owner identity is preserved, player dispatch fires, and target dispatch is suppressed. Complete unit validation, lifecycle, return values, restricted events, taint/security, and other edge semantics remain unproven. |
| `Frame.SetIgnoringChildrenForBounds` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to the focused false/true/false stored-state mutation for SetIgnoringChildrenForBounds. Actual bounds exclusion, layout effects, rendering, invalid arguments, lifecycle, and other edge semantics remain unproven. Implementation ancestor is 7a7a440402; test-file ancestor is e7fb98b910. |
| `GameTooltip.GetLeftLine` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to indexed left-line FontString lookup, first/second line text, and nil for the focused out-of-range/zero-index cases. Tooltip layout/rendering, line lifecycle, invalid arguments, localization, and other edge semantics remain unproven. |
| `GameTooltip.GetRightLine` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to indexed right-line text for a double line and the focused left-only-line nil/empty-text case. Tooltip layout/rendering, line lifecycle, invalid arguments, localization, and other edge semantics remain unproven. |
| `GetCollapsingStarCost` | untriaged | api | added | api added in 12.0.0. |
| `HOUSE_EXTERIOR_TYPE_UNLOCKED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `fixtureID: number`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_DECOR_ADD_TO_PREVIEW_LIST` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `previewItemData: structure C_HousingCatalog.HousingPreviewItemData`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_DECOR_FREE_PLACE_STATUS_CHANGED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `isFreePlaceEnabled: boolean`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_DECOR_PREVIEW_LIST_REMOVE_FROM_WORLD` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `decorGUID: string`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_DECOR_PREVIEW_LIST_UPDATED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `no arguments`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_DECOR_PREVIEW_STATE_CHANGED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `isPreviewState: boolean`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_EXPERT_MODE_PLACEMENT_FLAGS_UPDATED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `targetType: enum HousingExpertModeTargetType, activeFlags: enum HousingDecorPlacementRestriction`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_FIXTURE_UNLOCKED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `fixtureID: number`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_REFUND_LIST_UPDATED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `no arguments`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_SET_EXTERIOR_HOUSE_SIZE_RESPONSE` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `result: enum HousingResult`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_SET_EXTERIOR_HOUSE_TYPE_RESPONSE` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `result: enum HousingResult`, timing, lifecycle, ordering, or duplicate behavior. |
| `HOUSING_SET_FIXTURE_RESPONSE` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `result: enum HousingResult`, timing, lifecycle, ordering, or duplicate behavior. |
| `HouseExteriorSizeOption` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `HouseExteriorSizeOption.isLocked` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseExteriorSizeOption.name` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseExteriorSizeOption.size` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseExteriorSizeOptionsInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `HouseExteriorSizeOptionsInfo.options` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseExteriorSizeOptionsInfo.selectedSize` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseExteriorTypeOption` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `HouseExteriorTypeOption.houseExteriorTypeID` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `HouseExteriorTypeOption.isLocked` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseExteriorTypeOption.lockReasonString` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseExteriorTypeOption.name` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseExteriorTypeOptionsInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `HouseExteriorTypeOptionsInfo.options` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseExteriorTypeOptionsInfo.selectedExteriorType` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseLevelInfo` | best-effort | structure | added | Provenance-only: no runtime behavior claimed. |
| `HouseLevelInfo.exteriorDecorPlacementBudget` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseLevelInfo.exteriorFixtureBudget` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseLevelInfo.interiorDecorPlacementBudget` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseLevelInfo.level` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HouseLevelInfo.roomPlacementBudget` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `HousingDecorInstanceInfo.isRefundable` | evidence-required | structure-field | added | Temporary housing data does not establish the exact field contract, authoritative values, state transitions, ordering/localization, or consumer behavior; no focused proof exists. |
| `INITIATIVE_ACTIVITY_LOG_UPDATED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `no arguments`, timing, lifecycle, ordering, or duplicate behavior. |
| `INITIATIVE_COMPLETED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `initiativeTitle: string`, timing, lifecycle, ordering, or duplicate behavior. |
| `INITIATIVE_TASKS_TRACKED_LIST_CHANGED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `initiativeTaskID: number, added: boolean`, timing, lifecycle, ordering, or duplicate behavior. |
| `INITIATIVE_TASKS_TRACKED_UPDATED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `no arguments`, timing, lifecycle, ordering, or duplicate behavior. |
| `INITIATIVE_TASK_COMPLETED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `taskName: string`, timing, lifecycle, ordering, or duplicate behavior. |
| `IsRaidMarkerSystemEnabled` | untriaged | api | added | api added in 12.0.0. |
| `LEGACY_LOOT_RULES_CHANGED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `isLegacyLootModeEnabled: boolean`, timing, lifecycle, ordering, or duplicate behavior. |
| `LE_FRAME_TUTORIAL_JOURNEYS_TAB` | best-effort | global | added | Behavioral proof is limited to current 12.0.0 startup numeric publication and exact value 163; tutorial/pet-journal and consumer semantics are not claimed. |
| `LE_FRAME_TUTORIAL_LINK_TRANSMOG_CUSTOM_SET` | best-effort | global | added | Behavioral proof is limited to current 12.0.0 startup numeric publication and exact value 110; tutorial/pet-journal and consumer semantics are not claimed. |
| `LE_FRAME_TUTORIAL_TRANSMOG_CUSTOM_SET_DROPDOWN` | best-effort | global | added | Behavioral proof is limited to current 12.0.0 startup numeric publication and exact value 38; tutorial/pet-journal and consumer semantics are not claimed. |
| `LE_GAME_ERR_CHARTER_NEIGHBORHOOD_OWNERSHIP_TRANSFER_SUCCESS` | evidence-required | global | added | Source register value `1231` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_CHARTER_NEIGHBORHOOD_RENAME_NOTIFICATION_S` | evidence-required | global | added | Source register value `1232` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_CHARTER_SIGNATURE_REMOVED` | evidence-required | global | added | Source register value `1225` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_ENDEAVOR_REWARD_AVAILABLE` | evidence-required | global | added | Source register value `1226` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_HOUSING_EXTERIOR_FAILSAFE_RESET` | evidence-required | global | added | Source register value `1217` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_HOUSING_RESULT_COSMETIC_OWNER_NOT_IN_GUILD` | evidence-required | global | added | Source register value `1227` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_HOUSING_RESULT_MISSING_PRIVATE_NEIGHBORHOOD_INVITE` | evidence-required | global | added | Source register value `1230` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_HOUSING_RESULT_PLOT_NOT_VACANT` | evidence-required | global | added | Source register value `1228` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_HOUSING_RESULT_PLOT_RESERVED` | evidence-required | global | added | Source register value `1229` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_LFG_JOINED_TRAINING_GROUNDS_QUEUE` | evidence-required | global | added | Source register value `1236` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_PVP_TRAINING_GROUNDS_DISABLED` | evidence-required | global | added | Source register value `1234` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_GAME_ERR_SOLO_JOIN_TRAINING_GROUND` | evidence-required | global | added | Source register value `1235` conflicts with current fallback publication; reconcile authoritative epoch/value evidence before implementation. |
| `LE_PET_JOURNAL_FILTER_TYPE_BATTLE_PETS` | best-effort | global | added | Behavioral proof is limited to current 12.0.0 startup numeric publication and exact value 3; tutorial/pet-journal and consumer semantics are not claimed. |
| `LE_PET_JOURNAL_FILTER_TYPE_NON_COMBAT_PETS` | best-effort | global | added | Behavioral proof is limited to current 12.0.0 startup numeric publication and exact value 4; tutorial/pet-journal and consumer semantics are not claimed. |
| `LayeredRegion.SetVertexColorFromBoolean` | evidence-required | uiobject-method | added | Evidence-required/unsafe: no matching simulator implementation or focused test was found. Exact boolean branch, color/default arguments, vertex-color state interaction, propagation, rendering, validation, and edge semantics require authoritative evidence or a correct model/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `LuaColorCurveObject` | best-effort | luaobject | added | Best-effort evidence covers color-object table/method shape only; exact retail userdata identity, curve types, ordering/duplicates, color evaluation, secret propagation, and defaults remain unproven. |
| `LuaColorCurveObject.AddPoint` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the color-point insertion contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaColorCurveObject.ClearPoints` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the color-point clearing contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaColorCurveObject.Copy` | best-effort | luaobject-method | added | Best-effort evidence covers color-object copy/table shape only; exact retail userdata identity, curve types, ordering/duplicates, color evaluation, secret propagation, and defaults remain unproven. |
| `LuaColorCurveObject.Evaluate` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the color evaluation contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaColorCurveObject.EvaluateUnpacked` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the unpacked color evaluation contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaColorCurveObject.GetPoint` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the color-point retrieval contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaColorCurveObject.GetPointCount` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the color point-count contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaColorCurveObject.GetPoints` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the color point collection contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaColorCurveObject.RemovePoint` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the color-point removal contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaColorCurveObject.SetPoints` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the color-point replacement contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaColorCurveObject.SetToDefaults` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the color default-state contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaColorCurvePoint` | evidence-required | structure | added | Related curve proxy tests do not prove exact typed structure, field, or payload semantics; authoritative evidence or a correct model/test is required. |
| `LuaColorCurvePoint.x` | evidence-required | structure-field | added | Current implementation is absent, nil-only, or generic/placeholder-backed and does not establish the exact field contract, state, security, or consumer semantics. |
| `LuaColorCurvePoint.y` | evidence-required | structure-field | added | Current implementation is absent, nil-only, or generic/placeholder-backed and does not establish the exact field contract, state, security, or consumer semantics. |
| `LuaCurveObject` | best-effort | luaobject | added | Best-effort evidence covers table/method shape, per-instance fields, and tostring only; exact retail userdata identity, curve types, ordering/duplicates, secret propagation, and defaults remain unproven. |
| `LuaCurveObject.AddPoint` | best-effort | luaobject-method | added | Best-effort behavioral evidence covers scalar point insertion only; exact retail userdata identity, curve types, ordering/duplicates, color evaluation, secret propagation, and defaults remain unproven. |
| `LuaCurveObject.ClearPoints` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the retail ClearPoints contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaCurveObject.Copy` | best-effort | luaobject-method | added | Best-effort evidence covers scalar copy/table shape and copied point count only; exact retail userdata identity, curve types, ordering/duplicates, color evaluation, secret propagation, and defaults remain unproven. |
| `LuaCurveObject.Evaluate` | best-effort | luaobject-method | added | Best-effort behavioral evidence covers scalar two-point linear interpolation only; exact retail userdata identity, curve types, ordering/duplicates, color evaluation, secret propagation, and defaults remain unproven. |
| `LuaCurveObject.GetPoint` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the point retrieval contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaCurveObject.GetPointCount` | best-effort | luaobject-method | added | Best-effort evidence covers copied scalar point count only; exact retail userdata identity, curve types, ordering/duplicates, color evaluation, secret propagation, and defaults remain unproven. |
| `LuaCurveObject.GetPoints` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the point collection contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaCurveObject.RemovePoint` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the point removal contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaCurveObject.SetPoints` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the point replacement contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaCurveObject.SetToDefaults` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the default-state contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaCurveObjectBase` | evidence-required | luaobject | added | Current generic proxy omits or does not faithfully establish the LuaCurveObjectBase contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaCurveObjectBase.GetType` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the curve type contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaCurveObjectBase.HasSecretValues` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the secret-value propagation contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaCurveObjectBase.SetType` | evidence-required | luaobject-method | added | Current generic proxy omits or does not faithfully establish the curve type mutation contract; authoritative semantics or a correct modeled implementation are required, and no approval can close this row. |
| `LuaDurationObject` | best-effort | luaobject | added | Best-effort behavioral evidence covers table-backed object shape, default-zero behavior, per-instance fields, and tostring; full time, secret, and curve semantics are not established. |
| `LuaDurationObject.Assign` | evidence-required | luaobject-method | added | Current behavior is a no-op and incomplete; authoritative mutation semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.Copy` | evidence-required | luaobject-method | added | Current behavior is a fresh default object and is incomplete; authoritative copy semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.EvaluateElapsedDuration` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.EvaluateElapsedPercent` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.EvaluateRemainingDuration` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.EvaluateRemainingPercent` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.GetElapsedDuration` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.GetElapsedPercent` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.GetEndTime` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.GetModRate` | evidence-required | luaobject-method | added | Current behavior is constant (returns 1); authoritative rate semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.GetRemainingDuration` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.GetRemainingPercent` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.GetStartTime` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.GetTotalDuration` | evidence-required | luaobject-method | added | Current behavior is constant (returns 0); authoritative duration semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.HasSecretValues` | evidence-required | luaobject-method | added | Current behavior is constant (returns false); authoritative secret-value semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.IsZero` | best-effort | luaobject-method | added | Best-effort behavioral evidence covers default-zero IsZero behavior only; non-default duration, time, secret, and curve semantics are not established. |
| `LuaDurationObject.Reset` | evidence-required | luaobject-method | added | Current behavior is a no-op and incomplete; authoritative mutation semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.SetTimeFromEnd` | evidence-required | luaobject-method | added | Current behavior is a no-op and incomplete; authoritative mutation semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.SetTimeFromStart` | evidence-required | luaobject-method | added | Current behavior is a no-op and incomplete; authoritative mutation semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.SetTimeSpan` | evidence-required | luaobject-method | added | Current behavior is a no-op and incomplete; authoritative mutation semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaDurationObject.SetToDefaults` | evidence-required | luaobject-method | added | Current behavior is a no-op and incomplete; authoritative mutation semantics or a correct modeled implementation is required, and no approval can close this row. |
| `LuaFunctionContainer` | best-effort | luaobject | added | Tested method exposure, cancellation/invoke suppression, per-instance fields, read-only keys, and tostring are covered; exact retail callback validation, metatable/equality identity beyond tests, timer integration, lifecycle/GC, and API metadata fidelity remain unproven. |
| `LuaFunctionContainer.Cancel` | best-effort | luaobject-method | added | Tested method exposure, cancellation/invoke suppression, per-instance fields, read-only keys, and tostring are covered; exact retail callback validation, metatable/equality identity beyond tests, timer integration, lifecycle/GC, and API metadata fidelity remain unproven. |
| `LuaFunctionContainer.Invoke` | best-effort | luaobject-method | added | Tested method exposure, cancellation/invoke suppression, per-instance fields, read-only keys, and tostring are covered; exact retail callback validation, metatable/equality identity beyond tests, timer integration, lifecycle/GC, and API metadata fidelity remain unproven. |
| `LuaFunctionContainer.IsCancelled` | best-effort | luaobject-method | added | Tested method exposure, cancellation/invoke suppression, per-instance fields, read-only keys, and tostring are covered; exact retail callback validation, metatable/equality identity beyond tests, timer integration, lifecycle/GC, and API metadata fidelity remain unproven. |
| `Model.SetUseGBuffer` | exception-requested | uiobject-method | added | Permanent no-3D project scope excludes direct Model G-buffer behavior; already-decided scope exception, not a user approval request. |
| `NAME_PLATE_UNIT_BEHIND_CAMERA_CHANGED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `unitTarget: unit, isBehindCamera: boolean`, timing, lifecycle, ordering, or duplicate behavior. |
| `NEIGHBORHOOD_INITIATIVE_UPDATED` | evidence-required | event | added | Event registration exists, but no modeled producer or focused proof establishes payload `no arguments`, timing, lifecycle, ordering, or duplicate behavior. |
| `NewCraftingOrderInfo.reagentInfos` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `NumberAbbrevData` | evidence-required | structure | added | Related AbbreviateConfig proxy tests do not prove exact typed structure, field, array, or payload semantics; authoritative evidence or a correct model/test is required. |
| `NumberAbbrevData.abbreviation` | evidence-required | structure-field | added | Generic AbbreviateConfig proxy behavior does not establish the typed field contract, defaults/nullability, validation, ordering, or formatting semantics. |
| `NumberAbbrevData.abbreviationIsGlobal` | evidence-required | structure-field | added | Generic AbbreviateConfig proxy behavior does not establish the typed field contract, defaults/nullability, validation, ordering, or formatting semantics. |
| `NumberAbbrevData.breakpoint` | evidence-required | structure-field | added | Generic AbbreviateConfig proxy behavior does not establish the typed field contract, defaults/nullability, validation, ordering, or formatting semantics. |
| `NumberAbbrevData.fractionDivisor` | evidence-required | structure-field | added | Generic AbbreviateConfig proxy behavior does not establish the typed field contract, defaults/nullability, validation, ordering, or formatting semantics. |
| `NumberAbbrevData.significandDivisor` | evidence-required | structure-field | added | Generic AbbreviateConfig proxy behavior does not establish the typed field contract, defaults/nullability, validation, ordering, or formatting semantics. |
| `NumberAbbrevOptions` | evidence-required | structure | added | Related AbbreviateConfig proxy tests do not prove exact typed structure, nested field, or payload semantics; authoritative evidence or a correct model/test is required. |
| `NumberAbbrevOptions.breakpointData` | evidence-required | structure-field | added | Generic AbbreviateConfig proxy behavior does not establish the typed field contract, defaults/nullability, validation, ordering, or formatting semantics. |
| `NumberAbbrevOptions.config` | evidence-required | structure-field | added | Generic AbbreviateConfig proxy behavior does not establish the typed field contract, defaults/nullability, validation, ordering, or formatting semantics. |
| `NumberAbbrevOptions.locale` | evidence-required | structure-field | added | Generic AbbreviateConfig proxy behavior does not establish the typed field contract, defaults/nullability, validation, ordering, or formatting semantics. |
| `PARTY_KILL` | untriaged | event | added | event added in 12.0.0. |
| `PLAYER_TARGET_DIED` | untriaged | event | added | event added in 12.0.0. |
| `PrivateAuraIconInfo.borderScale` | evidence-required | structure-field | added | Current implementation is absent, nil-only, or generic/placeholder-backed and does not establish the exact field contract, state, security, or consumer semantics. |
| `REMOVE_NEIGHBORHOOD_CHARTER_SIGNATURE` | untriaged | event | added | event added in 12.0.0. |
| `Region.IsAnchoringSecret` | evidence-required | uiobject-method | added | Local flag behavior does not establish retail anchoring-secret relationship, secret propagation, authorization, lifecycle, or edge semantics; authoritative evidence or a correct security model/test is required. |
| `Region.SetAlphaFromBoolean` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to tested true/false branch selection and same-value no-op dirty behavior. Full Region defaults, clamping edges, effective-alpha propagation beyond the fixture, rendering, invalid arguments, lifecycle, and other edge semantics remain unproven. Implementation ancestor is b02c8fb4d6; test ancestors are b02c8fb4d6 and 379de99713. |
| `Region.SetVertexColorFromBoolean` | untriaged | docs-method-metadata | added | docs-method-metadata added in 12.0.0. |
| `RegisterEventCallback` | untriaged | api | added | api added in 12.0.0. |
| `RegisterUnitEventCallback` | untriaged | api | added | api added in 12.0.0. |
| `RegularReagentInfo.reagent` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `SECURE_TRANSFER_CONFIRM_HOUSING_PURCHASE` | untriaged | event | added | event added in 12.0.0. |
| `SECURE_TRANSFER_HOUSING_CURRENCY_PURCHASE_CONFIRMATION` | untriaged | event | added | event added in 12.0.0. |
| `SETTINGS_PANEL_OPEN` | untriaged | event | added | event added in 12.0.0. |
| `SET_SEEN_PRODUCTS` | untriaged | event | added | event added in 12.0.0. |
| `SHOW_JOURNEYS_UI` | untriaged | event | added | event added in 12.0.0. |
| `SHOW_NEW_PRODUCT_NOTIFICATION` | untriaged | event | added | event added in 12.0.0. |
| `SetCursorPosition` | untriaged | api | added | api added in 12.0.0. |
| `SetTableSecurityOption` | untriaged | api | added | api added in 12.0.0. |
| `ShowCloak` | untriaged | api | added | api added in 12.0.0. |
| `ShowHelm` | untriaged | api | added | api added in 12.0.0. |
| `ShowingCloak` | untriaged | api | added | api added in 12.0.0. |
| `ShowingHelm` | untriaged | api | added | api added in 12.0.0. |
| `SimpleScriptRegionAPI.IsAnchoringSecret` | untriaged | docs-method-metadata | added | docs-method-metadata added in 12.0.0. |
| `SimulateMouseClick` | untriaged | api | added | api added in 12.0.0. |
| `SimulateMouseDown` | untriaged | api | added | api added in 12.0.0. |
| `SimulateMouseUp` | untriaged | api | added | api added in 12.0.0. |
| `SimulateMouseWheel` | untriaged | api | added | api added in 12.0.0. |
| `Sound_EnableEncounterWarningsSounds` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `1`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `Sound_EncounterWarningsVolume` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `1.000000`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `SpellCooldownInfo.isOnGCD` | evidence-required | structure-field | added | Current implementation is absent, nil-only, or generic/placeholder-backed and does not establish the exact field contract, state, security, or consumer semantics. |
| `SpellCooldownInfo.timeUntilEndOfStartRecovery` | evidence-required | structure-field | added | Current implementation is absent, nil-only, or generic/placeholder-backed and does not establish the exact field contract, state, security, or consumer semantics. |
| `StatusBar.GetInterpolatedValue` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to the tested interpolation state machine: the displayed value stays old during Smooth target assignment, GetValue is target-facing, GetInterpolatedValue reports the displayed value, and SetToTargetValue snaps and clears interpolation; timing/animation progression, repeated-target behavior, invalid modes, render, and event fidelity are not established. |
| `StatusBar.GetTimerDuration` | best-effort | uiobject-method | added | Best-effort behavioral evidence covers exact duration-object identity round-trip through StatusBar timer storage; full timer semantics are not established. |
| `StatusBar.IsInterpolating` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to the tested interpolation state machine: the displayed value stays old during Smooth target assignment, GetValue is target-facing, GetInterpolatedValue reports the displayed value, and SetToTargetValue snaps and clears interpolation; timing/animation progression, repeated-target behavior, invalid modes, render, and event fidelity are not established. |
| `StatusBar.SetTimerDuration` | best-effort | uiobject-method | added | Best-effort behavioral evidence covers exact duration-object identity round-trip through StatusBar timer storage; full timer semantics are not established. |
| `StatusBar.SetToTargetValue` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to the tested interpolation state machine: the displayed value stays old during Smooth target assignment, GetValue is target-facing, GetInterpolatedValue reports the displayed value, and SetToTargetValue snaps and clears interpolation; timing/animation progression, repeated-target behavior, invalid modes, render, and event fidelity are not established. |
| `TOOLTIP_SHOW_ITEM_COMPARISON` | untriaged | event | added | event added in 12.0.0. |
| `TRAINING_GROUNDS_ENABLED_STATUS_UPDATED` | untriaged | event | added | event added in 12.0.0. |
| `TRANSMOG_CUSTOM_SETS_CHANGED` | untriaged | event | added | event added in 12.0.0. |
| `TRANSMOG_DISPLAYED_OUTFIT_CHANGED` | untriaged | event | added | event added in 12.0.0. |
| `TUTORIAL_COMBAT_EVENT` | untriaged | event | added | event added in 12.0.0. |
| `TextureBase.ResetTexCoord` | best-effort | uiobject-method | added | Best-effort behavioral evidence is limited to the focused SetTexCoord-then-ResetTexCoord reset to default eight-corner coordinates; commit 090af1ec8 corrected the stale four-value GetTexCoord assertion. Atlas-specific reset, rendering, invalid arguments, and other edge semantics remain unproven. |
| `TextureBase.SetSpriteSheetCell` | evidence-required | uiobject-method | added | Current SetSpriteSheetCell is a no-op; exact cell indexing, row/column coordinate mapping, optional dimensions, validation, and rendering require authoritative evidence or a correct model/test, and no approval can close the row. |
| `UIObject.HasAnySecretAspect` | evidence-required | uiobject-method | added | Exact secret-aspect aggregation, propagation, security/taint interaction, lifecycle, and edge semantics remain unproven; current frame-state tests show simulator consistency only. |
| `UIObject.HasSecretAspect` | evidence-required | uiobject-method | added | Exact Enum.SecretAspect mapping, combinations, propagation, authorization, lifecycle, and edge semantics remain unproven; current frame-state tests show simulator consistency only. |
| `UIObject.HasSecretValues` | evidence-required | uiobject-method | added | Exact secret-value propagation, source/consumer relationships, authorization, lifecycle, and edge semantics remain unproven; current frame-state tests show simulator consistency only. |
| `UIObject.IsPreventingSecretValues` | evidence-required | uiobject-method | added | Exact prevention authorization, propagation, lifecycle, interaction with secret values/aspects, and edge semantics remain unproven; current frame-state tests show simulator consistency only. |
| `UIObject.SetPreventSecretValues` | evidence-required | uiobject-method | added | Exact secure authorization, propagation, lifecycle, interaction with secret values/aspects, and edge semantics remain unproven; current frame-state tests show simulator consistency only. |
| `UNIT_DIED` | untriaged | event | added | event added in 12.0.0. |
| `UNIT_LOOT` | untriaged | event | added | event added in 12.0.0. |
| `UNIT_SPELL_DIMINISH_CATEGORY_STATE_UPDATED` | untriaged | event | added | event added in 12.0.0. |
| `UPDATE_BULLETIN_BOARD_MEMBER_TYPE` | untriaged | event | added | event added in 12.0.0. |
| `UnitCastingDuration` | untriaged | api | added | api added in 12.0.0. |
| `UnitChannelDuration` | untriaged | api | added | api added in 12.0.0. |
| `UnitClassFromGUID` | untriaged | api | added | api added in 12.0.0. |
| `UnitCreatureID` | best-effort | api | added | Bounded focused proof covers only current modeled/vendor behavior; full retail semantics, invalid inputs, lifecycle, and untested states remain unclaimed. |
| `UnitEmpoweredChannelDuration` | untriaged | api | added | api added in 12.0.0. |
| `UnitEmpoweredStageDurations` | untriaged | api | added | api added in 12.0.0. |
| `UnitEmpoweredStagePercentages` | untriaged | api | added | api added in 12.0.0. |
| `UnitGetDetailedHealPrediction` | best-effort | api | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealPredictionCalculator` | best-effort | luaobject | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealPredictionCalculator.GetDamageAbsorbClampMode` | best-effort | luaobject-method | added | Best-effort behavioral evidence is limited to the focused setter/getter round-trip: the calculator is created, the local damage-absorb clamp mode is set to 3, and the getter returns 3 after the initial read. Exact retail mode mapping, defaults, validation, lifecycle, secret behavior, and edge semantics remain unproven. |
| `UnitHealPredictionCalculator.GetDamageAbsorbs` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy returns the locally stored totalDamageAbsorbs field plus a false flag; this generic field/default behavior is not authoritative. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.GetHealAbsorbClampMode` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy returns the locally stored heal-absorb clamp mode or a generic default; exact retail mode mapping/default semantics are not established. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.GetHealAbsorbMode` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy returns the locally stored heal-absorb mode or a generic default; exact retail mode mapping/default semantics are not established. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.GetHealAbsorbs` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy returns the locally stored totalHealAbsorbs field plus a false flag; this generic field/default behavior is not authoritative. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.GetIncomingHealClampMode` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy returns the locally stored incoming-heal clamp mode or a generic default; exact retail mode mapping/default semantics are not established. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.GetIncomingHealOverflowPercent` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy returns the locally stored overflow percentage or a generic default; exact retail default/range/secret semantics are not established. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.GetIncomingHeals` | best-effort | luaobject-method | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealPredictionCalculator.GetPredictedValues` | best-effort | luaobject-method | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealPredictionCalculator.HasSecretValues` | best-effort | luaobject-method | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealPredictionCalculator.Reset` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy resets local modes, overflow, predicted fields, maximum-health mode, and a secret flag; reset scope/order/defaults are not authoritative. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.SetDamageAbsorbClampMode` | best-effort | luaobject-method | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealPredictionCalculator.SetHealAbsorbClampMode` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy stores the supplied local heal-absorb clamp mode without establishing accepted modes, validation, or retail semantics. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.SetHealAbsorbMode` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy stores the supplied local heal-absorb mode without establishing accepted modes, validation, or retail semantics. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.SetIncomingHealClampMode` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy stores the supplied local incoming-heal clamp mode without establishing accepted modes, validation, or retail semantics. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.SetIncomingHealOverflowPercent` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy stores the supplied local overflow percentage without establishing accepted range, validation, or secret semantics. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.SetPredictedValues` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy copies a selected local predicted-value subset; exact payload fields, missing-field behavior, secret values, and prediction integration are not established. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionCalculator.SetToDefaults` | evidence-required | luaobject-method | added | Evidence-required/unsafe: Current temporary proxy delegates to its local Reset implementation; exact default values, reset scope, and lifecycle semantics are not established. Exact retail contract, mode/default behavior, validation, secret/lifecycle semantics, and edge behavior require authoritative evidence or a correct modeled implementation/test; tests and assertions remain empty, with null commit, approval, and scope exception. |
| `UnitHealPredictionValues` | best-effort | structure | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealPredictionValues.health` | best-effort | structure-field | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealPredictionValues.healthMax` | best-effort | structure-field | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealPredictionValues.totalDamageAbsorbs` | best-effort | structure-field | added | Bounded focused proof covers only current modeled/vendor behavior; full retail semantics, invalid inputs, lifecycle, and untested states remain unclaimed. |
| `UnitHealPredictionValues.totalHealAbsorbs` | untriaged | structure-field | added | structure-field added in 12.0.0. |
| `UnitHealPredictionValues.totalIncomingHeals` | best-effort | structure-field | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealPredictionValues.totalIncomingHealsFromHealer` | best-effort | structure-field | added | Best-effort behavioral evidence covers only proxy/default/health/incoming-heal behavior; exact clamp/absorb/overflow/secret/full typed semantics are not established. |
| `UnitHealthMissing` | untriaged | api | added | api added in 12.0.0. |
| `UnitHealthPercent` | untriaged | api | added | api added in 12.0.0. |
| `UnitIsHumanPlayer` | best-effort | api | added | Bounded focused proof covers only current modeled/vendor behavior; full retail semantics, invalid inputs, lifecycle, and untested states remain unclaimed. |
| `UnitIsLieutenant` | untriaged | api | added | api added in 12.0.0. |
| `UnitIsMinion` | untriaged | api | added | api added in 12.0.0. |
| `UnitIsNPCAsPlayer` | untriaged | api | added | api added in 12.0.0. |
| `UnitIsSpellTarget` | best-effort | api | added | Bounded focused proof covers only current modeled/vendor behavior; full retail semantics, invalid inputs, lifecycle, and untested states remain unclaimed. |
| `UnitNameFromGUID` | untriaged | api | added | api added in 12.0.0. |
| `UnitPowerMissing` | untriaged | api | added | api added in 12.0.0. |
| `UnitPowerPercent` | untriaged | api | added | api added in 12.0.0. |
| `UnitSexBase` | untriaged | api | added | api added in 12.0.0. |
| `UnitShouldDisplaySpellTargetName` | untriaged | api | added | api added in 12.0.0. |
| `UnitSpellTargetClass` | untriaged | api | added | api added in 12.0.0. |
| `UnitSpellTargetName` | untriaged | api | added | api added in 12.0.0. |
| `UnitThreatLeadSituation` | untriaged | api | added | api added in 12.0.0. |
| `UnregisterEventCallback` | untriaged | api | added | api added in 12.0.0. |
| `UnregisterUnitEventCallback` | untriaged | api | added | api added in 12.0.0. |
| `VIEWED_TRANSMOG_OUTFIT_CHANGED` | untriaged | event | added | event added in 12.0.0. |
| `VIEWED_TRANSMOG_OUTFIT_SECONDARY_SLOTS_CHANGED` | untriaged | event | added | event added in 12.0.0. |
| `VIEWED_TRANSMOG_OUTFIT_SITUATIONS_CHANGED` | untriaged | event | added | event added in 12.0.0. |
| `VIEWED_TRANSMOG_OUTFIT_SLOT_REFRESH` | untriaged | event | added | event added in 12.0.0. |
| `VIEWED_TRANSMOG_OUTFIT_SLOT_SAVE_SUCCESS` | untriaged | event | added | event added in 12.0.0. |
| `VIEWED_TRANSMOG_OUTFIT_SLOT_WEAPON_OPTION_CHANGED` | untriaged | event | added | event added in 12.0.0. |
| `VOICE_CHAT_TTS_PLAYBACK_BOOKMARK` | untriaged | event | added | event added in 12.0.0. |
| `WorldTextCritScreenY_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0.0275`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextGravity_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0.500000`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextMinAlpha_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0.500000`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextNonRandomZ_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `2.5`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextRampDuration_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `1.000000`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextRampPowCrit_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `8.000000`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextRampPow_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `1.900000`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextRandomXY_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0.0`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextRandomZMax_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `1.5`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextRandomZMin_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0.8`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextScale_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `1.000000`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextScreenY_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0.015`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `WorldTextStartPosRandomness_v2` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `1.0`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `addonChatRestrictionsForced` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `alwaysShowRuneIcons` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `auctionSortByBuyoutPrice` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `auctionSortByUnitPrice` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `canaccessallvalues` | untriaged | api | added | api added in 12.0.0. |
| `canaccesssecrets` | untriaged | api | added | api added in 12.0.0. |
| `canaccesstable` | untriaged | api | added | api added in 12.0.0. |
| `canaccessvalue` | untriaged | api | added | api added in 12.0.0. |
| `chatBubblesRaid` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `combatWarningsEnabled` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `1`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `damageMeterEnabled` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `disableSuggestedLevelActivityFilter` | best-effort | cvar | added | Behavioral proof is limited to current 12.0.0 startup/default publication and exact string value `0`; rendering/UI/policy/persistence and consumer semantics are not claimed. |
| `docs.extra_apis.C_CombatLogInternal.GetCurrentEventInfo` | untriaged | docs-extra-api | added | docs-extra-api added in 12.0.0. |
| `docs.extra_enums.CharCreateAnimTurnType` | untriaged | docs-extra-enum | added | docs-extra-enum added in 12.0.0. |
| `docs.extra_enums.CharSectionCondition` | untriaged | docs-extra-enum | added | docs-extra-enum added in 12.0.0. |
| `docs.extra_events.COMBAT_LOG_APPLY_FILTER_SETTINGS` | untriaged | docs-extra-event | added | Transient docs-extra-event existed in an intermediate 12.0.0 snapshot but was absent at both patch endpoints. |
| `docs.extra_events.COMBAT_LOG_EVENT_INTERNAL_UNFILTERED` | untriaged | docs-extra-event | added | docs-extra-event added in 12.0.0. |
| `docs.extra_events.COMBAT_LOG_REFILTER_ENTRIES` | untriaged | docs-extra-event | added | Transient docs-extra-event existed in an intermediate 12.0.0 snapshot but was absent at both patch endpoints. |
| `docs.extra_script_objects.FrameAPITooltip` | untriaged | docs-extra-script-object | added | Transient docs-extra-script-object existed in an intermediate 12.0.0 snapshot but was absent at both patch endpoints. |
| `dropsecretaccess` | untriaged | api | added | api added in 12.0.0. |
| `enablePetBattleFloatingCombatText_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `encounterTimelineEnabled` | untriaged | cvar | added | cvar added in 12.0.0. |
| `encounterTimelineHideForOtherRoles` | untriaged | cvar | added | cvar added in 12.0.0. |
| `encounterTimelineHideLongCountdowns` | untriaged | cvar | added | cvar added in 12.0.0. |
| `encounterTimelineHideQueuedCountdowns` | untriaged | cvar | added | cvar added in 12.0.0. |
| `encounterTimelineIconographyEnabled` | untriaged | cvar | added | cvar added in 12.0.0. |
| `encounterTimelineIconographyHiddenMask` | untriaged | cvar | added | cvar added in 12.0.0. |
| `encounterWarningsDefaultMessageDuration` | untriaged | cvar | added | cvar added in 12.0.0. |
| `encounterWarningsEnabled` | untriaged | cvar | added | cvar added in 12.0.0. |
| `encounterWarningsHideIfNotTargetingPlayer` | untriaged | cvar | added | cvar added in 12.0.0. |
| `encounterWarningsLevel` | untriaged | cvar | added | cvar added in 12.0.0. |
| `endeavorInitiativesLastPoints` | untriaged | cvar | added | cvar added in 12.0.0. |
| `equipmentManager` | untriaged | cvar | added | cvar added in 12.0.0. |
| `externalDefensivesEnabled` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextAuraFade` | untriaged | cvar | added | Transient cvar existed in an intermediate 12.0.0 snapshot but was absent at both patch endpoints. |
| `floatingCombatTextAuraFade_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextAuras_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextCombatDamageAllAutos_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextCombatDamageDirectionalOffset_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextCombatDamageDirectionalScale_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextCombatDamage_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextCombatHealingAbsorbSelf_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextCombatHealingAbsorbTarget_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextCombatHealing_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextCombatLogPeriodicSpells_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextCombatState_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextComboPoints_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextDamageReduction_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextDodgeParryMiss_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextEnergyGains_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextFloatMode_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextFriendlyHealers_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextHonorGains_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextLowManaHealth_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextPeriodicEnergyGains_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextPetMeleeDamage_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextPetSpellDamage_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextReactives_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `floatingCombatTextRepChanges_v2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `hasanysecretvalues` | untriaged | api | added | api added in 12.0.0. |
| `issecrettable` | untriaged | api | added | api added in 12.0.0. |
| `issecretvalue` | untriaged | api | added | api added in 12.0.0. |
| `lastTransmogCustomSetIDNoSpec` | untriaged | cvar | added | cvar added in 12.0.0. |
| `lastTransmogCustomSetIDSpec1` | untriaged | cvar | added | cvar added in 12.0.0. |
| `lastTransmogCustomSetIDSpec2` | untriaged | cvar | added | cvar added in 12.0.0. |
| `lastTransmogCustomSetIDSpec3` | untriaged | cvar | added | cvar added in 12.0.0. |
| `lastTransmogCustomSetIDSpec4` | untriaged | cvar | added | cvar added in 12.0.0. |
| `lastTransmogOutfitIDNoSpec` | untriaged | cvar | added | cvar added in 12.0.0. |
| `lfgListAdvancedFiltersVersion` | untriaged | cvar | added | cvar added in 12.0.0. |
| `majorFactionRenownMap` | untriaged | cvar | added | cvar added in 12.0.0. |
| `mapvalues` | untriaged | api | added | api added in 12.0.0. |
| `nameplateAuraScale` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateCastBarDisplay` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateDebuffPadding` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateEnemyNpcAuraDisplay` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateEnemyPlayerAuraDisplay` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateFriendlyPlayerAuraDisplay` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateInfoDisplay` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowCastBars` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowClassColor` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowFriendlyClassColor` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowFriendlyNpcs` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowFriendlyPlayerGuardians` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowFriendlyPlayerMinions` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowFriendlyPlayerPets` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowFriendlyPlayerTotems` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowFriendlyPlayers` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowOffscreen` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateShowOnlyNameForFriendlyPlayerUnits` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateSimplifiedTypes` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateSize` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateStackingTypes` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateStyle` | untriaged | cvar | added | cvar added in 12.0.0. |
| `nameplateThreatDisplay` | untriaged | cvar | added | cvar added in 12.0.0. |
| `petJournalFilterVersion` | untriaged | cvar | added | cvar added in 12.0.0. |
| `raidFramesCenterBigDefensive` | untriaged | cvar | added | cvar added in 12.0.0. |
| `raidFramesDispelIndicatorOverlay` | untriaged | cvar | added | cvar added in 12.0.0. |
| `raidFramesDispelIndicatorType` | untriaged | cvar | added | cvar added in 12.0.0. |
| `raidFramesDisplayLargerRoleSpecificDebuffs` | untriaged | cvar | added | cvar added in 12.0.0. |
| `raidFramesHealthBarColor` | untriaged | cvar | added | cvar added in 12.0.0. |
| `scriptWarnings` | untriaged | cvar | added | cvar added in 12.0.0. |
| `script_object.AbbreviateConfigAPI` | best-effort | script-object | added | Best-effort behavioral evidence is limited to the tested proxy table/method shape, data round-trip, per-instance fields, read-only keys, and tostring prefix; retail abbreviation algorithms, formatting, lifecycle, validation, and edge semantics remain unproven. |
| `script_object.FrameAPITooltip` | evidence-required | script-object | added | No examined simulator construction, registration, method surface, lifecycle, or focused behavioral test; generated documentation presence is provenance only, not runtime behavior. |
| `script_object.LuaColorCurveObjectAPI` | best-effort | script-object | added | Best-effort behavioral evidence is limited to tested proxy table/Evaluate shape and copy return shape; color interpolation, point semantics, curve type, lifecycle, validation, secret behavior, and edge semantics remain unproven. |
| `script_object.LuaCurveObjectAPI` | best-effort | script-object | added | Best-effort behavioral evidence is limited to tested proxy table/method shape, two-point scalar evaluation, copy shape/count, per-instance fields, and tostring; complete curve-type, ordering, extrapolation, lifecycle, validation, and edge semantics remain unproven. |
| `script_object.LuaCurveObjectBaseAPI` | evidence-required | script-object | added | Shared concrete-curve methods and indirect tests do not establish a complete base method inventory, construction/identity contract, inheritance behavior, lifecycle, validation, or edge semantics. |
| `script_object.LuaDurationObjectAPI` | best-effort | script-object | added | Best-effort behavioral evidence is limited to tested proxy table/method shape, default-zero state, per-instance fields/tostring, and StatusBar duration-object identity round-trip; exact timing/clock, duration, secret, lifecycle, validation, and edge semantics remain unproven. |
| `script_object.UnitHealPredictionCalculatorAPI` | best-effort | script-object | added | Best-effort behavioral evidence is limited to tested proxy table/method shape, clamp-mode round-trip, predicted/current/maximum-health fixture behavior, and tostring; complete retail heal/absorb prediction, secret values, unit integration, lifecycle, validation, and edge semantics remain unproven. |
| `scrubsecretvalues` | untriaged | api | added | api added in 12.0.0. |
| `secretChallengeModeRestrictionsForced` | untriaged | cvar | added | cvar added in 12.0.0. |
| `secretCombatRestrictionsForced` | untriaged | cvar | added | cvar added in 12.0.0. |
| `secretEncounterRestrictionsForced` | untriaged | cvar | added | cvar added in 12.0.0. |
| `secretMapRestrictionsForced` | untriaged | cvar | added | cvar added in 12.0.0. |
| `secretPvPMatchRestrictionsForced` | untriaged | cvar | added | cvar added in 12.0.0. |
| `secretunwrap` | untriaged | api | added | api added in 12.0.0. |
| `secretwrap` | untriaged | api | added | api added in 12.0.0. |
| `securecallmethod` | untriaged | api | added | api added in 12.0.0. |
| `showAllItemsInTransmog` | untriaged | cvar | added | cvar added in 12.0.0. |
| `showCustomSetDetails` | untriaged | cvar | added | cvar added in 12.0.0. |
| `spellDiminishPVPEnemiesEnabled` | untriaged | cvar | added | cvar added in 12.0.0. |
| `spellDiminishPVPOnlyTriggerableByMe` | untriaged | cvar | added | cvar added in 12.0.0. |
| `string.concat` | untriaged | api | added | api added in 12.0.0. |
| `trackedInitiativeTasks` | untriaged | cvar | added | cvar added in 12.0.0. |
| `transmogHideIgnoredSlots` | untriaged | cvar | added | cvar added in 12.0.0. |
| `transmogrifySetsFilters` | untriaged | cvar | added | cvar added in 12.0.0. |
| `typedef.AbbreviateConfig` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.CooldownFrame` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.DurationSeconds` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.EncounterTimelineEventID` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.EventCallbackType` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.FrameEventCallbackType` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.FrameScriptObject` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.FrameTime` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.LuaColorCurveObject` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.LuaCurveEvaluatedResult` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.LuaCurveObject` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.LuaCurveObjectBase` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.LuaDurationObject` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.SimpleCheckbox` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.SoundHandle` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.Tooltip` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.UISoundSubType` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.UnitHealPredictionCalculator` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `typedef.UnitTokenVariant` | best-effort | typedef | added | Provenance-only: no runtime behavior claimed. |
| `COMBAT_LOG_EVENT` | untriaged | event | changed | event changed in 12.0.0. |
| `COMBAT_LOG_EVENT_UNFILTERED` | untriaged | event | changed | event changed in 12.0.0. |
| `C_FunctionContainers.CreateCallback` | best-effort | api | changed | Tested method exposure, cancellation/invoke suppression, per-instance fields, read-only keys, and tostring are covered; exact retail callback validation, metatable/equality identity beyond tests, timer integration, lifecycle/GC, and API metadata fidelity remain unproven. |
| `C_Housing.RequestHouseFinderNeighborhoodData` | untriaged | api | changed | api changed in 12.0.0. |
| `C_Item.CanItemTransmogAppearance` | untriaged | api | changed | api changed in 12.0.0. |
| `C_Item.GetItemInfo` | untriaged | api | changed | api changed in 12.0.0. |
| `C_ItemInteraction.ItemInteractionFrameInfo.flags` | untriaged | structure-field | changed | structure-field changed in 12.0.0. |
| `C_LFGList.DoesEntryTitleMatchPrebuiltTitle` | untriaged | api | changed | api changed in 12.0.0. |
| `C_LFGList.GetPlaystyleString` | untriaged | api | changed | api changed in 12.0.0. |
| `C_LFGList.SetEntryTitle` | untriaged | api | changed | api changed in 12.0.0. |
| `C_PerksActivities.PerksActivityInfo.criteriaList` | untriaged | structure-field | changed | structure-field changed in 12.0.0. |
| `C_PerksActivities.PerksActivityInfo.requirementsList` | untriaged | structure-field | changed | structure-field changed in 12.0.0. |
| `C_PingSecure.ClearPendingPingInfo` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.CreateFrame` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.DisplayError` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.GetTargetPingReceiver` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.GetTargetWorldPing` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.GetTargetWorldPingAndSend` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.SendPing` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.SetPendingPingOffScreenCallback` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.SetPingCooldownStartedCallback` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.SetPingPinFrameAddedCallback` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.SetPingPinFrameRemovedCallback` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.SetPingPinFrameScreenClampStateUpdatedCallback` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.SetPingRadialWheelCreatedCallback` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.SetSendMacroPingCallback` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_PingSecure.SetTogglePingListenerCallback` | evidence-required | api | changed | Source contract secure-only; current surface is no-op/inert callback storage/partial or absent. Exact secure enforcement, targeting, frame/error/audio/UI dispatch, callback invocation, and PingResult semantics require authoritative live evidence or a correct ping/security model and direct tests; no approval can close. |
| `C_Reputation.GetFactionParagonInfo` | untriaged | api | changed | api changed in 12.0.0. |
| `C_Reputation.IsFactionParagon` | untriaged | api | changed | api changed in 12.0.0. |
| `C_SpecializationInfo.GetSpecializationInfo` | untriaged | api | changed | api changed in 12.0.0. |
| `C_Timer.After` | evidence-required | api | changed | No focused executable proof exists. Callback/lifecycle semantics require a correct modeled implementation and executable behavioral proof; exact scheduling, lifecycle, GC, and edge semantics remain unproven, and no approval can close this row. |
| `C_Timer.NewTicker` | best-effort | api | changed | Best-effort behavioral evidence is limited to function/container acceptance, returned container identity, cancellation, and independent per-registration ticker counts covered by the four named tests. Exact scheduling, lifecycle, GC, and edge semantics remain unproven. |
| `C_Timer.NewTimer` | best-effort | api | changed | Best-effort behavioral evidence is limited to the shared function/container wrapper path and the named NewTimer proxy/handle test: the fired proxy receives one argument, compares equal to the returned handle, remains a distinct raw key, and shares its fields. Exact scheduling, lifecycle, GC, and edge semantics remain unproven. |
| `C_TooltipInfo.GetRecipeResultItem` | untriaged | api | changed | api changed in 12.0.0. |
| `C_TooltipInfo.GetRecipeResultItemForOrder` | untriaged | api | changed | api changed in 12.0.0. |
| `C_TradeSkillUI.GetEnchantItems` | evidence-required | api | changed | The examined current professions implementation/registration surface does not publish this method; exact lexical/runtime absence is not claimed. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_TradeSkillUI.GetRecraftRemovalWarnings` | evidence-required | api | changed | Current registration binds this changed method to a placeholder empty-table return; exact replaced-reagent warning semantics are unproven. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_TradeSkillUI.IsRecraftReagentValid` | evidence-required | api | changed | The examined current professions implementation/registration surface does not publish this method; exact lexical/runtime absence is not claimed. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_TradeSkillUI.RecraftLimitCategoryValid` | evidence-required | api | changed | Current registration binds this changed method to a placeholder true return; exact recraft-limit semantics are unproven. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_Transmog.GetSlotVisualInfo` | evidence-required | api | changed | Source-register names/signature transition only; current C_Transmog has no modeled slot-visual/pending/apply state or direct behavioral proof. |
| `C_TransmogCollection.GetAppearanceSourceInfo` | evidence-required | api | changed | Current collection surface is seeded/partial; custom-set lifecycle, hyperlinks, validation, and changed appearance-source semantics remain unproven. |
| `C_UnitAuras.GetUnitAuras` | evidence-required | api | changed | Source-register signatures/defaults and adjacent seeded aura lookup/state behavior do not establish the added 12.0.0 contract; authoritative semantics or a correct model/test are required, and no approval can close this row. |
| `C_UnitAurasPrivate.AddPrivateAuraUpdateCallback` | evidence-required | api | changed | source register establishes secure-only/return-shape metadata only; current temporary model is permissive/partial, and secure/private-aura semantics remain unproven. |
| `C_UnitAurasPrivate.AnchorPrivateAura` | evidence-required | api | changed | source register establishes secure-only/return-shape metadata only; current temporary model is permissive/partial, and secure/private-aura semantics remain unproven. |
| `C_UnitAurasPrivate.GetAllPrivateAuras` | evidence-required | api | changed | source register establishes secure-only/return-shape metadata only; current temporary model is permissive/partial, and secure/private-aura semantics remain unproven. |
| `C_UnitAurasPrivate.GetAuraDataByAuraInstanceIDPrivate` | evidence-required | api | changed | source register establishes secure-only/return-shape metadata only; current temporary model is permissive/partial, and secure/private-aura semantics remain unproven. |
| `C_UnitAurasPrivate.GetPrivateAuraAnchors` | evidence-required | api | changed | source register establishes secure-only/return-shape metadata only; current temporary model is permissive/partial, and secure/private-aura semantics remain unproven. |
| `C_UnitAurasPrivate.SetPrivateAuraAnchorAddedCallback` | evidence-required | api | changed | source register establishes secure-only/return-shape metadata only; current temporary model is permissive/partial, and secure/private-aura semantics remain unproven. |
| `C_UnitAurasPrivate.SetPrivateAuraAnchorRemovedCallback` | evidence-required | api | changed | source register establishes secure-only/return-shape metadata only; current temporary model is permissive/partial, and secure/private-aura semantics remain unproven. |
| `C_UnitAurasPrivate.SetPrivateRaidBossMessageCallback` | evidence-required | api | changed | source register establishes secure-only/return-shape metadata only; current temporary model is permissive/partial, and secure/private-aura semantics remain unproven. |
| `C_UnitAurasPrivate.SetPrivateWarningTextFrame` | evidence-required | api | changed | source register establishes secure-only/return-shape metadata only; current temporary model is permissive/partial, and secure/private-aura semantics remain unproven. |
| `C_VoiceChat.SpeakText` | untriaged | api | changed | api changed in 12.0.0. |
| `CanBeRaidTarget` | untriaged | api | changed | api changed in 12.0.0. |
| `ClearRaidMarker` | untriaged | api | changed | api changed in 12.0.0. |
| `EmitterCombatRange` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `Enum.AccountStateLoadedFlagsMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.AccountTransTypeMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.AccountTransTypeMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.CharCustomizationTypeMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CharCustomizationTypeMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderItemTypeMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderItemTypeMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.MissingItem` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.MissingNpc` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.MissingOrder` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.MissingRecraftItem` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.NoAccountItems` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.NotClaimed` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.NotCrafted` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.NotInGuild` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.NotYetImplemented` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.OutOfPublicOrderCapacity` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.ServerIsNotAvailable` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.TargetCannotCraft` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.TargetLocked` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.ThrottleViolation` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.Timeout` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.TooManyItems` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResult.WrongVersion` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResultMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingOrderResultMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingReagentItemFlag.TooltipShowsAsStatModifications` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingReagentItemFlagMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.CraftingReagentItemFlagMeta.MinValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.CreateAllAccountDataMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.CurrencyDestroyReasonMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.CurrencyDestroyReasonMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.CurrencySourceMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.CurrencySourceMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeAccountSettingMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeAccountSettingMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeAuraFrameSettingMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeAuraFrameSettingMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeAuraFrameSystemIndicesMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeAuraFrameSystemIndicesMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeCooldownViewerSettingMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeCooldownViewerSettingMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeSystemMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeSystemMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeUnitFrameSettingMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.EditModeUnitFrameSettingMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.FragmentIDMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.FrameTutorialAccountMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.FrameTutorialAccountMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.GameRuleMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.GameRuleMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingCatalogEntrySubtype.OwnedModifiedStack` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingCatalogEntrySubtype.OwnedUnmodifiedStack` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingCatalogEntrySubtypeMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingCatalogEntrySubtypeMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingDecorActionFlagsMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingDecorActionFlagsMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingDecorPlacementRestriction.InvalidCollision` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingDecorPlacementRestriction.InvalidTarget` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingDecorPlacementRestrictionMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingDecorPlacementRestrictionMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingItemToastTypeMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingItemToastTypeMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.CannotAfford` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.CharterComplete` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.CollisionInvalid` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.DbError` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.DecorCannotBeRedeemed` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.DecorItemNotDestroyable` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.DecorNotFound` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.DecorNotFoundInStorage` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.DuplicateCharterSignature` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.FilterRejected` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.FixtureCantDeleteDoor` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.FixtureHookEmpty` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.FixtureHookOccupied` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.FixtureHouseTypeMismatch` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.FixtureNotFound` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.FixtureSizeMismatch` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.FixtureTypeMismatch` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.GenericFailure` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.GuildMoreAccountsNeeded` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.GuildMoreActivePlayersNeeded` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.GuildNotLoaded` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.HookNotChildOfFixture` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.HouseEditLockFailed` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.HouseExteriorRootNotFound` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.HouseNotFound` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.IncorrectFaction` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.InvalidDecorItem` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.InvalidDistance` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.InvalidGuild` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.InvalidHouse` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.InvalidInstance` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.InvalidInteraction` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.InvalidMap` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.InvalidNeighborhoodName` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.InvalidRoomLayout` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.LockOperationFailed` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.LockedByOtherPlayer` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.MaxDecorReached` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.MissingCoreFixture` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.MissingDye` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.MissingExpansionAccess` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.MissingFactionMap` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.MissingPrivateNeighborhoodInvite` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.MoreHouseSlotsNeeded` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.MoreSignaturesNeeded` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.NeighborhoodNotFound` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.NoNeighborhoodOwnershipRequests` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.NotInDecorEditMode` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.NotInFixtureEditMode` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.NotInLayoutEditMode` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.NotInsideHouse` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.NotOnOwnedPlot` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.OperationAborted` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.PermissionDenied` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.PlacementTargetInvalid` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.PlayerNotFound` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.PlayerNotInInstance` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.PlotNotFound` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.PlotNotVacant` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.PlotReservationCooldown` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.PlotReserved` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.RoomNotFound` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.RoomUpdateFailed` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.RpcFailure` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.ServiceNotAvailable` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.StaticDataNotFound` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.TimeoutLimit` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.TimerunningNotAllowed` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.TokenRequired` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.TooManyRequests` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.TransactionFailure` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResult.UnlockOperationFailed` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResultMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.HousingResultMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.ItemCollectionTypeMeta.MaxValue` | best-effort | enum | changed | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.ItemCollectionTypeMeta.NumValues` | best-effort | enum | changed | Best-effort behavioral: startup namespace publication, numeric type, and exact source-register value only; no collection or secret semantics claimed. |
| `Enum.MapIconUIWidgetSetTypeMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.MapIconUIWidgetSetTypeMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.SendAddonMessageResultMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.SendAddonMessageResultMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.SurveyDeliveryMomentMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.SurveyDeliveryMomentMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.TooltipDataLineTypeMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.TooltipDataLineTypeMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.TooltipDataTypeMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.TooltipDataTypeMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.TraitNodeEntryTypeMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.TraitNodeEntryTypeMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.TraitNodeFlagMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.TraitNodeFlagMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.UICursorTypeMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.UICursorTypeMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.UIWidgetVisualizationTypeMeta.MaxValue` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.UIWidgetVisualizationTypeMeta.NumValues` | best-effort | enum | changed | enum changed in 12.0.0. |
| `Enum.VasTransactionPurchaseResult.EndDbErrors` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.VasTransactionPurchaseResultMeta.MaxValue` | untriaged | enum | changed | enum changed in 12.0.0. |
| `Enum.VasTransactionPurchaseResultMeta.NumValues` | untriaged | enum | changed | enum changed in 12.0.0. |
| `GameTooltip.GetMinimumWidth` | best-effort | uiobject-method | changed | Setter/getter state round-trip is behaviorally tested with SetMinimumWidth(150) and GetMinimumWidth() == 150; rendering/layout effects, clamping, invalid arguments, and edge semantics remain unproven. |
| `GameTooltip.GetPadding` | best-effort | uiobject-method | changed | Setter/getter state round-trip is behaviorally tested with SetPadding(8) and GetPadding() == 8; rendering/layout effects, clamping, invalid arguments, and edge semantics remain unproven. |
| `GameTooltip.SetMinimumWidth` | best-effort | uiobject-method | changed | Setter/getter state round-trip is behaviorally tested with SetMinimumWidth(150) and GetMinimumWidth() == 150; rendering/layout effects, clamping, invalid arguments, and edge semantics remain unproven. |
| `GameTooltip.SetPadding` | best-effort | uiobject-method | changed | Setter/getter state round-trip is behaviorally tested with SetPadding(8) and GetPadding() == 8; rendering/layout effects, clamping, invalid arguments, and edge semantics remain unproven. |
| `GameTooltip.SetText` | best-effort | uiobject-method | changed | Best-effort behavioral evidence is limited to clearing existing tooltip lines, inserting supplied text as the first line, and the focused `NumLines() == 1` assertion; formatting, wrapping, colors, localization, rendering, invalid arguments, and edge semantics remain unproven. |
| `GetRaidTargetIndex` | untriaged | api | changed | api changed in 12.0.0. |
| `HOUSE_LEVEL_CHANGED` | untriaged | event | changed | event changed in 12.0.0. |
| `HOUSING_BASIC_MODE_PLACEMENT_FLAGS_UPDATED` | untriaged | event | changed | event changed in 12.0.0. |
| `HOUSING_BASIC_MODE_SELECTED_TARGET_CHANGED` | untriaged | event | changed | event changed in 12.0.0. |
| `HOUSING_DECOR_PLACE_SUCCESS` | untriaged | event | changed | event changed in 12.0.0. |
| `IsRaidMarkerActive` | untriaged | api | changed | api changed in 12.0.0. |
| `LE_EXPANSION_LEVEL_CURRENT` | untriaged | global | changed | global changed in 12.0.0. |
| `LE_EXPANSION_LEVEL_PREVIOUS` | untriaged | global | changed | global changed in 12.0.0. |
| `LE_GAME_ERR_CHARTER_NEIGHBORHOOD_RENAME` | untriaged | global | changed | global changed in 12.0.0. |
| `LE_GAME_ERR_GUILD_NEIGHBORHOOD_BUILT_HOUSE_S` | untriaged | global | changed | global changed in 12.0.0. |
| `LE_GAME_ERR_GUILD_NEIGHBORHOOD_NEW_SUBDIVISION` | untriaged | global | changed | global changed in 12.0.0. |
| `LE_GAME_ERR_GUILD_NEIGHBORHOOD_RENAME_S` | untriaged | global | changed | global changed in 12.0.0. |
| `LE_GAME_ERR_GUILD_NEIGHBORHOOD_SOLD_HOUSE_S` | untriaged | global | changed | global changed in 12.0.0. |
| `LE_GAME_ERR_HOUSING_RESULT_MISSING_EXPANSION_ACCESS` | untriaged | global | changed | global changed in 12.0.0. |
| `LE_GAME_ERR_HOUSING_RESULT_PERMISSION_DENIED` | untriaged | global | changed | global changed in 12.0.0. |
| `LE_GAME_ERR_RECENT_ALLY_PIN_SERVER_ERROR` | untriaged | global | changed | global changed in 12.0.0. |
| `NUM_LE_EXPANSION_LEVELS` | untriaged | global | changed | global changed in 12.0.0. |
| `NUM_LE_FRAME_TUTORIALS` | untriaged | global | changed | global changed in 12.0.0. |
| `NUM_LE_PET_JOURNAL_FILTERS` | untriaged | global | changed | global changed in 12.0.0. |
| `NonEmitterCombatRange` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `PlaceRaidMarker` | untriaged | api | changed | api changed in 12.0.0. |
| `RemoveRaidTargets` | untriaged | api | changed | api changed in 12.0.0. |
| `SetRaidTarget` | untriaged | api | changed | api changed in 12.0.0. |
| `StatusBar` | untriaged | uiobject | changed | uiobject changed in 12.0.0. |
| `StatusBar.GetFillStyle` | evidence-required | uiobject-method | changed | Current GetFillStyle returns constant STANDARD instead of the stored style; exact accepted styles, validation, state round-trip, rendering, and edge semantics require authoritative evidence or a correct implementation/test, and no approval can close the row. |
| `StatusBar.SetFillStyle` | evidence-required | uiobject-method | changed | Current SetFillStyle stores a value but its public round-trip is broken by GetFillStyle returning constant STANDARD; exact accepted styles, validation, state, rendering, and edge semantics require authoritative evidence or a correct implementation/test, and no approval can close the row. |
| `StatusBar.SetMinMaxValues` | best-effort | uiobject-method | changed | Best-effort behavioral evidence is limited to the focused range round-trip and clamping of an already-stored current value: (10, 50) clamps 80 to 50, and (30, 40) clamps 20 to 30. Interpolation-target behavior, rendering, events, invalid/reversed ranges, and other edge semantics remain unproven. |
| `StatusBar.SetValue` | best-effort | uiobject-method | changed | Best-effort behavioral evidence is limited to the focused StatusBar immediate/Smooth SetValue state machine: 0.25 displayed value, 0.75 target-facing value, preserved displayed value, and snap completion. Range clamping beyond the fixture, timing/animation progression, invalid modes, render, events, and edge semantics remain unproven. |
| `TRANSMOG_OUTFITS_CHANGED` | untriaged | event | changed | event changed in 12.0.0. |
| `TextureBase.GetTexCoord` | best-effort | uiobject-method | changed | Best-effort behavioral evidence is limited to the tested atlas remapping, including partial UV mapping; complete atlas fidelity, CASC/texture loading beyond these assertions, filtering/wrap edge cases, invalid arguments, and rendering correctness remain unproven. |
| `TextureBase.SetAtlas` | best-effort | uiobject-method | changed | Best-effort behavioral evidence is limited to the tested known/unknown lookup, direct atlas slice, tiling-flag state, and render-preferred 2x path selection; complete atlas fidelity, CASC/texture loading beyond these assertions, filtering/wrap edge cases, invalid arguments, and rendering correctness remain unproven. |
| `UNIT_SPELLCAST_CHANNEL_START` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_CHANNEL_STOP` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_CHANNEL_UPDATE` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_DELAYED` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_EMPOWER_START` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_EMPOWER_STOP` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_EMPOWER_UPDATE` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_FAILED` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_FAILED_QUIET` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_INTERRUPTED` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_SENT` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_START` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_STOP` | untriaged | event | changed | event changed in 12.0.0. |
| `UNIT_SPELLCAST_SUCCEEDED` | untriaged | event | changed | event changed in 12.0.0. |
| `UnitCastingInfo` | untriaged | api | changed | api changed in 12.0.0. |
| `UnitChannelInfo` | untriaged | api | changed | api changed in 12.0.0. |
| `UnitFullName` | untriaged | api | changed | api changed in 12.0.0. |
| `UnitIsUnit` | untriaged | api | changed | api changed in 12.0.0. |
| `UnitName` | untriaged | api | changed | api changed in 12.0.0. |
| `UnitNameUnmodified` | untriaged | api | changed | api changed in 12.0.0. |
| `VOICE_CHAT_TTS_PLAYBACK_FAILED` | untriaged | event | changed | event changed in 12.0.0. |
| `VOICE_CHAT_TTS_PLAYBACK_FINISHED` | untriaged | event | changed | event changed in 12.0.0. |
| `VOICE_CHAT_TTS_PLAYBACK_STARTED` | untriaged | event | changed | event changed in 12.0.0. |
| `advFlyKeyboardMaxPitchFactor` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `advFlyKeyboardMaxTurnFactor` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `advFlyKeyboardMinPitchFactor` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `advFlyKeyboardMinTurnFactor` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `advFlyPitchControlCameraChase` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `mountJournalGeneralFilters` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `mountJournalSourcesFilter` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `mountJournalTypeFilter` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateGameObjectMaxDistance` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateLargerScale` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateMaxAlpha` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateMaxAlphaDistance` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateMaxDistance` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateMaxScale` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateMaxScaleDistance` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateMinAlpha` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateMinAlphaDistance` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateMinScale` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateMinScaleDistance` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateOccludedAlphaMult` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateOverlapH` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateOverlapV` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplatePlayerLargerScale` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplatePlayerMaxDistance` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateSelectedAlpha` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateSelectedScale` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateSelfAlpha` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateShowSelf` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `nameplateTargetBehindMaxDistance` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `partyBackgroundOpacity` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `petJournalFilters` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `petJournalSourceFilters` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `petJournalTypeFilters` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `scrub` | untriaged | api | changed | api changed in 12.0.0. |
| `spellActivationOverlayOpacity` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `string.trim` | untriaged | api | changed | api changed in 12.0.0. |
| `superTrackerDist` | untriaged | cvar | changed | cvar changed in 12.0.0. |
| `typedef.TickerCallback` | best-effort | typedef | changed | Provenance-only: no runtime behavior claimed. |
| `typedef.TimerCallback` | best-effort | typedef | changed | Provenance-only: no runtime behavior claimed. |
| `ActionHasRange` | best-effort | api | removed | api removed in 12.0.0. |
| `BNSendGameData` | best-effort | api | removed | api removed in 12.0.0. |
| `BNSendWhisper` | best-effort | api | removed | api removed in 12.0.0. |
| `BNSetCustomMessage` | best-effort | api | removed | api removed in 12.0.0. |
| `C_CatalogShop.OpenCatalogShopInteraction` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_EventUtils.NotifySettingsLoaded` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_HouseExterior.GetCurrentHouseExteriorTypeName` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_HousingBasicMode.InvalidPlacementInfo` | evidence-required | structure | removed | Source metadata and current housing method/model coverage do not prove runtime removal or replacement identity; source-text or method absence alone cannot close this row. |
| `C_HousingBasicMode.InvalidPlacementInfo.anyRestrictions` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_HousingBasicMode.InvalidPlacementInfo.invalidCollision` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_HousingBasicMode.InvalidPlacementInfo.invalidTarget` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_HousingBasicMode.InvalidPlacementInfo.notInRoom` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_HousingBasicMode.InvalidPlacementInfo.tooFar` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_HousingBasicMode.IsNudgeEnabled` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_HousingBasicMode.SetNudgeEnabled` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_HousingCatalog.HousingCatalogEntryInfo.numStored` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_HousingDecor.GetMaxDecorPlaced` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_HousingDecor.HousingLevelInfo` | evidence-required | structure | removed | Source metadata and current housing method/model coverage do not prove runtime removal or replacement by HouseLevelInfo; source-text or method absence alone cannot close this row. |
| `C_HousingDecor.HousingLevelInfo.exteriorDecorPlacementBudget` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_HousingDecor.HousingLevelInfo.exteriorFixtureBudget` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_HousingDecor.HousingLevelInfo.interiorDecorPlacementBudget` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_HousingDecor.HousingLevelInfo.level` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_HousingDecor.HousingLevelInfo.roomPlacementBudget` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_NamePlate.GetNamePlateEnemyClickThrough` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.GetNamePlateEnemyPreferredClickInsets` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.GetNamePlateEnemySize` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.GetNamePlateFriendlyClickThrough` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.GetNamePlateFriendlyPreferredClickInsets` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.GetNamePlateFriendlySize` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.GetNamePlateSelfClickThrough` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.GetNamePlateSelfPreferredClickInsets` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.GetNamePlateSelfSize` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.GetNumNamePlateMotionTypes` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.SetNamePlateEnemyClickThrough` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.SetNamePlateEnemyPreferredClickInsets` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.SetNamePlateEnemySize` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.SetNamePlateFriendlyClickThrough` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.SetNamePlateFriendlyPreferredClickInsets` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.SetNamePlateFriendlySize` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.SetNamePlateSelfClickThrough` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.SetNamePlateSelfPreferredClickInsets` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_NamePlate.SetNamePlateSelfSize` | best-effort | api | removed | 12.0.0 source register records removal; full-LoD rawget probe proves current runtime absence for all 19 removed C_NamePlate methods while retained APIs remain callable. Source scanning is auxiliary; no historical load-order or broad scanner claim. |
| `C_PerksActivities.PerksActivityCriteria` | evidence-required | structure | removed | Source metadata and current perks fixture/default coverage do not prove runtime removal or replacement identity; source-text or method absence alone cannot close this row. |
| `C_PerksActivities.PerksActivityCriteria.criteriaID` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_PerksActivities.PerksActivityCriteria.requiredValue` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_PerksActivities.PerksActivityRequirement` | evidence-required | structure | removed | Source metadata and current perks fixture/default coverage do not prove runtime removal or replacement identity; source-text or method absence alone cannot close this row. |
| `C_PerksActivities.PerksActivityRequirement.completed` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_PerksActivities.PerksActivityRequirement.requirementText` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `C_PingSecure.GetCooldownInfo` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_PingSecure.GetDefaultPingOptions` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_PingSecure.GetTextureKitForType` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_PlayerInfo.CanPlayerUseEventScheduler` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_PlayerInfo.IsExpansionLandingPageUnlockedForPlayer` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_PvP.CanDisplayDamage` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_PvP.CanDisplayHealing` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_PvP.CanDisplayKillingBlows` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_StorePublic.IsDisabledByParentalControls` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_TaskQuest.GetQuestIconUIWidgetSet` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_TaskQuest.GetQuestTooltipUIWidgetSet` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_Texture.GetCraftingReagentQualityChatIcon` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_TooltipInfo.GetTransmogrifyItem` | best-effort | api | removed | 12.0.0 source register records removal; one full-LoD namespace-safe rawget probe proves current runtime absence for all 19 remaining removed runtime APIs. Source scanning is auxiliary; no replacement behavior is inferred. |
| `C_TradeSkillUI.GetReagentRequirementItemIDs` | evidence-required | api | removed | Current registration does not bind this removed method; 12.0.0 removal behavior is unproven and exact lexical/runtime absence is not claimed. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_TradeSkillUI.GetRecipeFixedReagentItemLink` | evidence-required | api | removed | Current registration does not bind this removed method; 12.0.0 removal behavior is unproven and exact lexical/runtime absence is not claimed. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_TradeSkillUI.GetRecipeQualityReagentItemLink` | evidence-required | api | removed | Current registration does not bind this removed method; 12.0.0 removal behavior is unproven and exact lexical/runtime absence is not claimed. Authoritative profession semantics or a correct model/test are required, and no approval can close the row. |
| `C_Transmog.ApplyAllPending` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.CanTransmogItem` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.CanTransmogItemWithItem` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.ClearAllPending` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.ClearPending` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.Close` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.GetApplyCost` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.GetApplyWarnings` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.GetBaseCategory` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.GetCreatureDisplayIDForSource` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.GetPending` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.GetSlotEffectiveCategory` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.GetSlotInfo` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.GetSlotUseError` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.IsSlotBeingCollapsed` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.IsTransmogEnabled` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.LoadOutfit` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.SetPending` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence for the removed C_Transmog API; five retained C_Transmog methods remain callable. The removed TransmogApplyWarningInfo declaration is evidence-required/unsafe because runtime rawget or source metadata does not prove structure removal, replacement identity, or field semantics; its two structure-field rows remain untriaged. |
| `C_Transmog.TransmogApplyWarningInfo` | evidence-required | structure | removed | Auxiliary source-token checks and method absence do not prove runtime structure removal, replacement identity, or field semantics; source-text alone cannot close this row. |
| `C_Transmog.TransmogApplyWarningInfo.itemLink` | untriaged | structure-field | removed | Metadata-only structure-field removal remains untriaged; runtime rawget does not prove structure publication absence. |
| `C_Transmog.TransmogApplyWarningInfo.text` | untriaged | structure-field | removed | Metadata-only structure-field removal remains untriaged; runtime rawget does not prove structure publication absence. |
| `C_TransmogCollection.DeleteOutfit` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence; retained appearance methods remain callable. |
| `C_TransmogCollection.GetItemTransmogInfoListFromOutfitHyperlink` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence; retained appearance methods remain callable. |
| `C_TransmogCollection.GetNumMaxOutfits` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence; retained appearance methods remain callable. |
| `C_TransmogCollection.GetOutfitHyperlinkFromItemTransmogInfoList` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence; retained appearance methods remain callable. |
| `C_TransmogCollection.GetOutfitInfo` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence; retained appearance methods remain callable. |
| `C_TransmogCollection.GetOutfitItemTransmogInfoList` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence; retained appearance methods remain callable. |
| `C_TransmogCollection.GetOutfits` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence; retained appearance methods remain callable. |
| `C_TransmogCollection.ModifyOutfit` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence; retained appearance methods remain callable. |
| `C_TransmogCollection.NewOutfit` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence; retained appearance methods remain callable. |
| `C_TransmogCollection.RenameOutfit` | best-effort | api | removed | Full-LoD rawget proof establishes current publication absence; retained appearance methods remain callable. |
| `CancelEmote` | best-effort | api | removed | api removed in 12.0.0. |
| `ChangeActionBarPage` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatLogAddFilter` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatLogAdvanceEntry` | evidence-required | api | removed | api removed in 12.0.0. |
| `CombatLogClearEntries` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatLogGetCurrentEntry` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatLogGetCurrentEventInfo` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatLogGetNumEntries` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatLogGetRetentionTime` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatLogResetFilter` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatLogSetCurrentEntry` | evidence-required | api | removed | api removed in 12.0.0. |
| `CombatLogSetRetentionTime` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatLogShowCurrentEntry` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatLog_Object_IsA` | best-effort | api | removed | api removed in 12.0.0. |
| `CombatTextSetActiveUnit` | best-effort | api | removed | api removed in 12.0.0. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_CATEGORIES_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_ENTRY_TAGS_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_FILTER_TAGS_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_NONE_TAG_ID` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_NUM_FEATURED_BUNDLES_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_NUM_FEATURED_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_NUM_FILTER_TAG_GROUPS` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_OPTIONS_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_OPTIONS_PER_CATEGORY_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_OPTIONS_PER_SUBCATEGORY_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_ROOMS_CATEGORY_ID` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_ROOMS_SUBCATEGORY_ID` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_SIZE_DATAGROUP_ID` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_SUBCATEGORIES_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_SUBCATEGORIES_PER_CATEGORY_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `Constants.HousingCatalogConsts.HOUSING_CATALOG_TAG_GROUP_TAGS_EXPECTED` | evidence-required | constant | removed | Evidence-required/unsafe: source-register and current constants_values.lua evidence show the simulator bootstrap omits this key while retaining Constants.HousingCatalogConsts, but source/bootstrap absence is insufficient to prove full runtime or dynamic publication, historical load-order timing, replacement semantics, or exact 12.0.0 removal. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, load_addon, and provenance_only. |
| `CraftingItemSlotModification.itemID` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `CraftingOrderReagentInfo.reagent` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `CraftingReagentInfo.itemID` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `CraftingResourceReturnInfo.itemID` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `DeathRecap_GetEvents` | best-effort | api | removed | api removed in 12.0.0. |
| `DeathRecap_HasEvents` | best-effort | api | removed | api removed in 12.0.0. |
| `DoEmote` | best-effort | api | removed | api removed in 12.0.0. |
| `Enum.AccountDataUpdateStatus.AccountDataUpdateCorrupt` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.AccountDataUpdateStatus.AccountDataUpdateFailed` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.AccountDataUpdateStatus.AccountDataUpdateSuccess` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.AccountDataUpdateStatus.AccountDataUpdateToobig` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.AccountStateLoadedFlags.AccountStateAccountCurrenciesLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateAccountFactionsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateAccountItemsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateAccountMappingLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateAccountNotificationsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateAccountWowlabsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateAchievementsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateArchivedPurchasesLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateAuctionableTokensLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateBanktabSettingsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateBattleNetAccountLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateBitVectorsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateBpayAddLicenseObjectsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateBpayDistributionObjectsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateBpayProductitemObjectsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateCharacterItemsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateCharactersLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateCombinedQuestLogLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateConsumableTokensLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateCriteriaLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateCurrencyCapsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateCurrencyTransferLogLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateDataElementsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateDynamicCriteriaLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateEventRecordsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateHousingDataLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateItemCollectionsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateLgVendorPurchaseLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateMountsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStatePerksHeldItemLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStatePerksPastRewardsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStatePerksPendingPurchaseLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStatePerksPendingRewardsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStatePetjournalInitialized` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStatePurchasesLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateQuestCriteriaLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateQuestLogLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateRafActivityLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateRafBalanceLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateRafRewardsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateRevokedRafRewardsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateSettingsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateTrialBoostHistoryLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateVasTransactionsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateWarbandScenesLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.AccountStateWarbandsLoaded` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.AccountStateLoadedFlags.None` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CharCustomizationType.Facepaint` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.CharCustomizationType.FacepaintColor` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.CharCustomizationType.Outfit` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.CraftingOrderItemType.NpcProvided` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.CraftingOrderItemType.Reagent` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.CreateAllAccountData.CreateAllAccountCurrenciesDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllAccountDynamicCriteriaDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllAccountFactionsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllAccountItemsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllAccountMappingDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllAccountNotificationsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllAccountStateHousingData` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllAchievementsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllArchivedPurchasesDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllAuctionableTokensDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllBanktabSettingsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllBattlepetsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllBitVectorsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllBpayAddLicenseObjectsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllBpayDistributionObjectsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllBpayProductitemObjectsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllCharacterItemsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllCharactersDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllCombinedQuestLogEntriesDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllConsumableTokensDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllCriteriaDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllCurrencyTransferLogDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllCurrencycapsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllDataElementsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllEventRecordsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllItemCollectionItemsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllLgVendorPurchaseDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllMountsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllNone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllPerkHeldItemsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllPerkPastRewardsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllPerkPendingPurchasesDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllPerkPendingRewardsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllPurchasesDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllQuestCriteriaDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllQuestLogDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllRafActivitiesDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllRafBalanceDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllRafRewardsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllRevokedRafRewardsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllSettingsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllTrialBoostHistoryDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllVasTransactionsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllWarbandGroupsDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllWarbandScenesLoadedDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateAllWowlabsDataDone` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.CreateAllAccountData.CreateObject` | evidence-required | enum | removed | Evidence-required/unsafe: current bootstrap source omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias replacement semantics, or absence after all LoD addons. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.ExpansionLandingPageType.Dragonflight` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ExpansionLandingPageType.None` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ExpansionLandingPageType.WarWithin` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ExpansionLandingPageTypeMeta.MaxValue` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ExpansionLandingPageTypeMeta.MinValue` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ExpansionLandingPageTypeMeta.NumValues` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.GossipNpcOption.Placeholder_6` | evidence-required | enum | removed | Evidence-required/unsafe: the source register pairs this removed name with a same-value added name, but current bootstrap omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias compatibility, or semantic replacement identity. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.HousingCatalogEntrySubtype.MarketItem` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.HousingDecorPlacementRestriction.NotInsideRoom` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.HousingResult.FixtureNotOwned` | evidence-required | enum | removed | enum removed in 12.0.0. |
| `Enum.HousingResult.MissingTheme` | evidence-required | enum | removed | enum removed in 12.0.0. |
| `Enum.ItemCollectionType.ItemCollectionExteriorFixture` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionHeirloom` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionNone` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionRoom` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionRoomMaterial` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionRoomTheme` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionRuneforgeLegendaryAbility` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionToy` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionTransmog` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionTransmogIllusion` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionTransmogSetFavorite` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.ItemCollectionWarbandScene` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCollectionType.NumItemCollectionTypes` | evidence-required | enum | removed | Evidence-required/unsafe: bootstrap omission does not prove full runtime/dynamic removal, historical timing, replacement semantics, or all-LoD absence. |
| `Enum.ItemCreationContext.Placeholder_12_0_0` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ItemCreationContext.Timewalker` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ItemRecraftFlags.ItemRecraftFlagInvalid` | evidence-required | enum | removed | Evidence-required/unsafe: the source register pairs this removed name with a same-value added name, but current bootstrap omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias compatibility, or semantic replacement identity. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.NeighbordhoodInitiativeCategory.Current` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.NeighbordhoodInitiativeCategory.Legacy` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.NeighbordhoodInitiativeCategoryMeta.MaxValue` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.NeighbordhoodInitiativeCategoryMeta.MinValue` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.NeighbordhoodInitiativeCategoryMeta.NumValues` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.NpcCraftingOrderSetFlags.CraftingOrderFlagAllowDuplicate` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.NpcCraftingOrderSetFlags.CraftingOrderFlagAllowMultiple` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.PerksVendorCategoryType.UnusedPerksVendorCategoryRefundUnused` | evidence-required | enum | removed | Evidence-required/unsafe: the source register pairs this removed name with a same-value added name, but current bootstrap omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias compatibility, or semantic replacement identity. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.PlayerInteractionType.PlaceholderType79` | evidence-required | enum | removed | Evidence-required/unsafe: the source register pairs this removed name with a same-value added name, but current bootstrap omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias compatibility, or semantic replacement identity. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.QuestTagType.Placeholder_1` | evidence-required | enum | removed | Evidence-required/unsafe: the source register pairs this removed name with a same-value added name, but current bootstrap omission is insufficient to prove full runtime or dynamic publication absence, historical removal timing, alias compatibility, or semantic replacement identity. Authoritative evidence or a correct model/test is required; tests and assertions remain empty, with null commit, approval, scope exception, and provenance_only. |
| `Enum.RcoCloseReason.RcoCloseCancel` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.RcoCloseReason.RcoCloseCrafterFulfill` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.RcoCloseReason.RcoCloseExpire` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.RcoCloseReason.RcoCloseFulfill` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.RcoCloseReason.RcoCloseGmCancel` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.RcoCloseReason.RcoCloseInvalid` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.RcoCloseReason.RcoCloseReject` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ReportStorageProvider.Alibaba` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ReportStorageProvider.Aws` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ReportStorageProvider.Gcp` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ReportStorageProviderMeta.MaxValue` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ReportStorageProviderMeta.MinValue` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.ReportStorageProviderMeta.NumValues` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.VoiceTtsDestination.LocalPlayback` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.VoiceTtsDestination.QueuedLocalPlayback` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.VoiceTtsDestination.QueuedRemoteTransmission` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.VoiceTtsDestination.QueuedRemoteTransmissionWithLocalPlayback` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.VoiceTtsDestination.RemoteTransmission` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.VoiceTtsDestination.RemoteTransmissionWithLocalPlayback` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.VoiceTtsDestination.ScreenReader` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.VoiceTtsDestinationMeta.MaxValue` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.VoiceTtsDestinationMeta.MinValue` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.VoiceTtsDestinationMeta.NumValues` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.WMOExteriorID.DefaultAlliance` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.WMOExteriorID.DefaultHorde` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.WMOExteriorID.Invalid` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.WMOExteriorIDMeta.MaxValue` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.WMOExteriorIDMeta.MinValue` | untriaged | enum | removed | enum removed in 12.0.0. |
| `Enum.WMOExteriorIDMeta.NumValues` | untriaged | enum | removed | enum removed in 12.0.0. |
| `FindBaseSpellByID` | best-effort | api | removed | Compatibility claim is limited to dynamic wrapper forwarding to C_SpellBook.FindBaseSpellByID; exact retail return semantics are not claimed. |
| `FindFlyoutSlotBySpellID` | best-effort | api | removed | Compatibility claim is limited to dynamic wrapper forwarding to C_SpellBook.FindFlyoutSlotBySpellID; exact retail return semantics are not claimed. |
| `FindSpellOverrideByID` | best-effort | api | removed | Compatibility claim is limited to dynamic wrapper forwarding to C_SpellBook.FindSpellOverrideByID; exact retail return semantics are not claimed. |
| `ForceAllowAero` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `GetActionAutocast` | best-effort | api | removed | api removed in 12.0.0. |
| `GetActionBarPage` | best-effort | api | removed | api removed in 12.0.0. |
| `GetActionCharges` | best-effort | api | removed | api removed in 12.0.0. |
| `GetActionCooldown` | best-effort | api | removed | api removed in 12.0.0. |
| `GetActionCount` | best-effort | api | removed | api removed in 12.0.0. |
| `GetActionLossOfControlCooldown` | best-effort | api | removed | api removed in 12.0.0. |
| `GetActionText` | best-effort | api | removed | api removed in 12.0.0. |
| `GetActionTexture` | best-effort | api | removed | api removed in 12.0.0. |
| `GetBattlegroundInfo` | best-effort | api | removed | Compatibility claim is limited to the tested seeded Wintergrasp row and unknown-ID nil behavior; complete retail dataset/lifecycle fidelity is not claimed. |
| `GetBonusBarIndex` | best-effort | api | removed | api removed in 12.0.0. |
| `GetBonusBarOffset` | best-effort | api | removed | api removed in 12.0.0. |
| `GetCurrentCombatTextEventInfo` | best-effort | api | removed | api removed in 12.0.0. |
| `GetDeathRecapLink` | best-effort | api | removed | api removed in 12.0.0. |
| `GetExtraBarIndex` | best-effort | api | removed | api removed in 12.0.0. |
| `GetMultiCastBarIndex` | best-effort | api | removed | api removed in 12.0.0. |
| `GetOverrideBarIndex` | best-effort | api | removed | api removed in 12.0.0. |
| `GetOverrideBarSkin` | best-effort | api | removed | api removed in 12.0.0. |
| `GetTempShapeshiftBarIndex` | best-effort | api | removed | api removed in 12.0.0. |
| `GetVehicleBarIndex` | best-effort | api | removed | api removed in 12.0.0. |
| `HOUSING_CATALOG_SEARCHER_RELEASED` | untriaged | event | removed | event removed in 12.0.0. |
| `HOUSING_DECOR_NUDGE_STATUS_CHANGED` | untriaged | event | removed | event removed in 12.0.0. |
| `HasAction` | best-effort | api | removed | api removed in 12.0.0. |
| `HasBonusActionBar` | best-effort | api | removed | api removed in 12.0.0. |
| `HasExtraActionBar` | best-effort | api | removed | api removed in 12.0.0. |
| `HasOverrideActionBar` | best-effort | api | removed | api removed in 12.0.0. |
| `HasTempShapeshiftActionBar` | best-effort | api | removed | api removed in 12.0.0. |
| `HasVehicleActionBar` | best-effort | api | removed | api removed in 12.0.0. |
| `IsActionInRange` | best-effort | api | removed | api removed in 12.0.0. |
| `IsAttackAction` | best-effort | api | removed | api removed in 12.0.0. |
| `IsAutoRepeatAction` | best-effort | api | removed | api removed in 12.0.0. |
| `IsConsumableAction` | best-effort | api | removed | api removed in 12.0.0. |
| `IsConsumableSpell` | best-effort | api | removed | Current full-LoD rawget absence only; no source-scanner, replacement-behavior, or historical timing claim. |
| `IsCurrentAction` | best-effort | api | removed | api removed in 12.0.0. |
| `IsEncounterInProgress` | best-effort | api | removed | api removed in 12.0.0. |
| `IsEncounterLimitingResurrections` | best-effort | api | removed | api removed in 12.0.0. |
| `IsEncounterSuppressingRelease` | best-effort | api | removed | api removed in 12.0.0. |
| `IsEquippedAction` | best-effort | api | removed | api removed in 12.0.0. |
| `IsItemAction` | best-effort | api | removed | api removed in 12.0.0. |
| `IsPossessBarVisible` | best-effort | api | removed | api removed in 12.0.0. |
| `IsStackableAction` | best-effort | api | removed | api removed in 12.0.0. |
| `IsUsableAction` | best-effort | api | removed | api removed in 12.0.0. |
| `LEARNED_SPELL_IN_TAB` | untriaged | event | removed | event removed in 12.0.0. |
| `LE_FRAME_TUTORIAL_LINK_TRANSMOG_OUTFIT` | untriaged | global | removed | global removed in 12.0.0. |
| `LE_FRAME_TUTORIAL_TRANSMOG_OUTFIT_DROPDOWN` | untriaged | global | removed | global removed in 12.0.0. |
| `LE_WORLD_ELAPSED_TIMER_TYPE_CHALLENGE_MODE` | untriaged | global | removed | global removed in 12.0.0. |
| `LE_WORLD_ELAPSED_TIMER_TYPE_NONE` | untriaged | global | removed | global removed in 12.0.0. |
| `LE_WORLD_ELAPSED_TIMER_TYPE_PROVING_GROUND` | untriaged | global | removed | global removed in 12.0.0. |
| `NamePlateClassificationScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `NamePlateHorizontalScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `NamePlateMaximumClassificationScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `NamePlateVerticalScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `NameplatePersonalClickThrough` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `NameplatePersonalHideDelayAlpha` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `NameplatePersonalHideDelaySeconds` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `NameplatePersonalShowAlways` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `NameplatePersonalShowInCombat` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `NameplatePersonalShowWithTarget` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `NewCraftingOrderInfo.reagentItems` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `PlaySound` | best-effort | api | removed | Compatibility claim is limited to accepting a numeric sound ID without error; audio output fidelity is not claimed. |
| `RegularReagentInfo.itemID` | untriaged | structure-field | removed | structure-field removed in 12.0.0. |
| `SHOW_DELVES_DISPLAY_UI` | untriaged | event | removed | event removed in 12.0.0. |
| `SetActionUIButton` | best-effort | api | removed | api removed in 12.0.0. |
| `SetPortraitToTexture` | best-effort | api | removed | Compatibility claim is limited to the tested portrait circular mask and no duplicate mask on repeat; broader portrait semantics are not claimed. |
| `SetRaidTargetProtected` | best-effort | api | removed | Current full-LoD rawget absence only; no source-scanner, replacement-behavior, or historical timing claim. |
| `ShowClassColorInFriendlyNameplate` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `ShowClassColorInNameplate` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `ShowNamePlateLoseAggroFlash` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `SpellGetVisibilityInfo` | best-effort | api | removed | Vendor-present wrapper publication and focused visibility-key forwarding proof; full C_Spell visibility semantics remain unproven. |
| `SpellIsAlwaysShown` | best-effort | api | removed | Current full-LoD rawget absence only; no source-scanner, replacement-behavior, or historical timing claim. |
| `SpellIsPriorityAura` | best-effort | api | removed | api removed in 12.0.0. |
| `SpellIsSelfBuff` | best-effort | api | removed | api removed in 12.0.0. |
| `StripHyperlinks` | best-effort | api | removed | Current full-LoD rawget absence only; no source-scanner, replacement-behavior, or historical timing claim. |
| `TerrainBlendBakeEnable` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `TerrainUnlitShaderEnable` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `TransmogPendingInfo` | evidence-required | structure | removed | Source metadata and current transmog method absence do not prove runtime removal, TransmogPendingInfoMixin identity, or replacement semantics; source-text alone cannot close this row. |
| `WorldTextCritScreenY` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextGravity` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextMinAlpha` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextNonRandomZ` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextRampDuration` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextRampPow` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextRampPowCrit` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextRandomXY` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextRandomZMax` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextRandomZMin` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextScreenY` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `WorldTextStartPosRandomness` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `activeCUFProfile` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `advancedWatchFrame` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `currencyTokensBackpack1` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `currencyTokensBackpack2` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `currencyTokensUnused1` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `currencyTokensUnused2` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `displayedRAFFriendInfo` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `docs.extra_apis.ShowCloak` | untriaged | docs-extra-api | removed | docs-extra-api removed in 12.0.0. |
| `docs.extra_apis.ShowHelm` | untriaged | docs-extra-api | removed | docs-extra-api removed in 12.0.0. |
| `docs.extra_apis.ShowingCloak` | untriaged | docs-extra-api | removed | docs-extra-api removed in 12.0.0. |
| `docs.extra_apis.ShowingHelm` | untriaged | docs-extra-api | removed | docs-extra-api removed in 12.0.0. |
| `docs.extra_events.COMBAT_LOG_APPLY_FILTER_SETTINGS` | untriaged | docs-extra-event | removed | Transient docs-extra-event existed in an intermediate 12.0.0 snapshot but was absent at both patch endpoints. |
| `docs.extra_events.COMBAT_LOG_REFILTER_ENTRIES` | untriaged | docs-extra-event | removed | Transient docs-extra-event existed in an intermediate 12.0.0 snapshot but was absent at both patch endpoints. |
| `docs.extra_script_objects.FrameAPITooltip` | untriaged | docs-extra-script-object | removed | Transient docs-extra-script-object existed in an intermediate 12.0.0 snapshot but was absent at both patch endpoints. |
| `enablePetBattleFloatingCombatText` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextAllSpellMechanics` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextAuraFade` | untriaged | cvar | removed | Transient cvar existed in an intermediate 12.0.0 snapshot but was absent at both patch endpoints. |
| `floatingCombatTextAuras` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextCombatDamage` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextCombatDamageAllAutos` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextCombatDamageDirectionalOffset` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextCombatDamageDirectionalScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextCombatDamageStyle` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextCombatHealing` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextCombatHealingAbsorbSelf` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextCombatHealingAbsorbTarget` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextCombatLogPeriodicSpells` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextCombatState` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextComboPoints` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextDamageReduction` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextDodgeParryMiss` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextEnergyGains` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextFloatMode` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextFriendlyHealers` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextHonorGains` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextLowManaHealth` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextPeriodicEnergyGains` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextPetMeleeDamage` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextPetSpellDamage` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextReactives` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextRepChanges` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextSpellMechanics` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `floatingCombatTextSpellMechanicsOther` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `friendsSmallView` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `friendsViewButtons` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `guildRosterView` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_BaseOrbScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_BaseRingScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_DistScaleMax` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_DistScaleMin` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_HighlightDefault` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_HighlightDragging` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_HighlightHovered` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_HighlightKeybind` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_HighlightSelected` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_OrbPosOffset` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_ScaleDistanceMax` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_ScaleDistanceMin` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_SnapDegrees` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_TextMode` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_XRayCheckerSize` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_XRayDarkAlpha` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Rotation_XRayLightAlpha` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_BaseArrowHeadScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_BaseArrowStemScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_BaseCubeScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_DistScaleMax` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_DistScaleMin` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_HighlightDefault` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_HighlightDragging` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_HighlightHovered` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_HighlightKeybind` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_HighlightSelected` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_MaxDistanceFromCamera` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_Padding` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_ScaleDistanceMax` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_ScaleDistanceMin` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_XRayCheckerSize` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_XRayDarkAlpha` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `housingExpertGizmos_Translation_XRayLightAlpha` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2503` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2507` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2510` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2511` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2564` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2570` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2574` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2590` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2593` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2594` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2600` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2653` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2658` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2685` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2688` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastRenownForMajorFaction2736` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastTransmogOutfitIDSpec1` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastTransmogOutfitIDSpec2` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastTransmogOutfitIDSpec3` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastTransmogOutfitIDSpec4` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lastVoidStorageTutorial` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lfGuildComment` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lfGuildSettings` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lfgAutoFill` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `lfgAutoJoin` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `mapAnimDuration` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `mapAnimMinAlpha` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `mapAnimStartDelay` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `math.huge` | untriaged | api | removed | api removed in 12.0.0. |
| `math.pi` | untriaged | api | removed | api removed in 12.0.0. |
| `minimapAltitudeHintMode` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `minimapShowArchBlobs` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `minimapShowQuestBlobs` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateClassResourceTopInset` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateGlobalScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateHideHealthAndPower` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateLargeBottomInset` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateLargeTopInset` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateMotion` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateMotionSpeed` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateOtherBottomInset` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateOtherTopInset` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateResourceOnTarget` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateSelfBottomInset` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateSelfScale` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateSelfTopInset` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateShowFriendlyBuffs` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateShowFriendlyGuardians` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateShowFriendlyMinions` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateShowFriendlyNPCs` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateShowFriendlyPets` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateShowFriendlyTotems` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateShowFriends` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateShowOnlyNames` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `nameplateShowPersonalCooldowns` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `playerStatLeftDropdown` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `playerStatRightDropdown` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `removeChatDelay` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `showQuestObjectivesOnMap` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `showTokenFrameHonor` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `splashScreenBoost` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `splashScreenNormal` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `splashScreenSeason` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `strtrim` | best-effort | api | removed | Compatibility claim is limited to the tested default and custom-character trimming behavior; broader string semantics are not claimed. |
| `trackQuestSorting` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `unlockedExpansionLandingPages` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `watchFrameBaseAlpha` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `watchFrameIgnoreCursor` | untriaged | cvar | removed | cvar removed in 12.0.0. |
| `watchFrameState` | untriaged | cvar | removed | cvar removed in 12.0.0. |
## Category counts

| Category | Occurrences |
|---|---:|
| `api` | 661 |
| `constant` | 100 |
| `cvar` | 350 |
| `docs-extra-api` | 5 |
| `docs-extra-enum` | 2 |
| `docs-extra-event` | 5 |
| `docs-extra-script-object` | 2 |
| `docs-method-metadata` | 2 |
| `enum` | 1575 |
| `event` | 99 |
| `global` | 35 |
| `luaobject` | 7 |
| `luaobject-method` | 68 |
| `script-object` | 7 |
| `structure` | 63 |
| `structure-field` | 368 |
| `typedef` | 21 |
| `uiobject` | 1 |
| `uiobject-method` | 39 |

## Additional classified slices

The bounded remaining-removal slice classifies exactly 19 removed runtime API rows as best-effort/behavioral using one full-LoD namespace-safe rawget batch at `patch-tests/patch_12_1/strict_removals.rs::removed_remaining_runtime_apis_are_absent_after_full_lod_load`; three obsolete simulator publications were removed, 16 were already absent, source scanning is auxiliary, and no replacement behavior is inferred.

- The bounded 12.0.0 simulator-compatibility slice classifies exactly seven removed globals `FindBaseSpellByID`, `FindFlyoutSlotBySpellID`, `FindSpellOverrideByID`, `GetBattlegroundInfo`, `PlaySound`, `SetPortraitToTexture`, and `strtrim` as best-effort/compat using the focused `patch-tests/patch_12_1/legacy_compat.rs::simulator_legacy_compat_globals_preserve_tested_behavior` proof at `abba2bd2a`; the claim is limited to tested wrapper forwarding, seeded battleground/unknown-ID behavior, numeric sound acceptance without audio fidelity, portrait masking, and default/custom trimming. SpellGetVisibilityInfo is separately classified vendor-present; CombatLogAdvanceEntry and CombatLogSetCurrentEntry are classified evidence-required/unsafe because their retained fixture-only compatibility semantics remain unproven.

## Sources

- `tools/gen_patch_12_0_0_register.py` — reproducible wowless-history source generator.
- `data/patch-api/sources/12.0.0-register.json` — normalized source/provenance register.

## See Also

- [[patch-api-audit-manifest]] — register schema and completion contract.

The bounded seven-row `C_ChatInfo`/`C_CombatText`/`C_Commentator` API-gap slice classifies all seven rows as evidence-required/unsafe. Current fallbacks are absent, no-op, constant-false, or adjacent-state only and do not model lockdown, emote, active-unit, combat-text, or commentator event state, restrictions, result contracts, transitions, events, ordering, or lifecycle.
