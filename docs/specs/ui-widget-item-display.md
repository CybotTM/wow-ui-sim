# Supplied ItemDisplay widgets

`C_UIWidgetManager.GetItemDisplayVisualizationInfo` exposes explicit supplied ItemDisplay records through the existing manager subsystem. Complete records, empty environment-owned storage, typed feeds and a fresh nested getter projection are present in `src/c_api/ui_widget_manager/`. The passive library's populated fixtures reached behavioral RED before getter implementation; [bounded Retail acceptance](#bounded-retail-acceptance--2026-10-16) now covers the getter and completed synchronous cached consumer. PTR remains pending. See [manager state and feeds](../wiki/systems/ui-widget-manager-state.md) for the existing model.

## What it must do

- [ ] Support the public getter in Retail and PTR, grounded in each profile's actual cached `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua`: required numeric `widgetID`, nullable `ItemDisplayVisualizationInfo`, and `SecretArguments = "AllowedWhenUntainted"`. Preserve the existing manager secret policy; secret-wrapper behavior requires separate proof.
- [ ] Reject missing, explicit nil, string (including numeric string), boolean, table and function selectors at the shared required-number boundary. Diagnostics contain `must be a number` and identify `GetItemDisplayVisualizationInfo`; the existing `GetWidgetSetInfo` rejection is the control.
- [ ] Return exactly one nil for absent numeric `widgetID` `934207.5`. Do not infer catalog membership or introduce integer, positivity or range restrictions.

### Populated supplied-state scope — bounded Retail accepted; PTR pending

- [ ] Project exactly the 15 outer and 16 nested declaration rows below; represent the four optional fields as Some/value or None/nil. Preserve Some empty strings and numeric zero; never synthesize item metadata into the manager record.
- [ ] Return fresh public outer/nested snapshots; isolate caller mutations, retained snapshots, all-field replacements, fractional IDs, environments, removal and restoration.
- [ ] Publish complete typed type27 state and membership before synchronous manager events; reject mismatched type/missing set without changing state or emitting events.
- [ ] Apply the existing manager policy to actual permitted/denied secret wrappers, preserving wrappers, taint and state. No new policy.
- [ ] Reach unchanged cached type27 ItemDisplay and XML BaseItem through normal feed with existing synchronous item metadata/load: completed update, supplied stack-count display and nil-count hiding through the cached default-one branch, nil optional text, hide/reacquire, membership/full-set removal, live unregister, child color-callback cleanup and later-feed inactivity. Check text/texture identity and completed parent/child sizing, not assets/pixels or pending/cancellation races. A populated getter assertion precedes every cached setup dependency.

Cached Retail and PTR `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua` rows were inspected on 2026-10-16 and match field-for-field. This is a supplied-state contract, not a native/catalog/defaults claim. See [manager state](../wiki/systems/ui-widget-manager-state.md).

### ItemDisplayVisualizationInfo — complete projection

| Requirement | Field | Type | Nullable |
| --- | --- | --- | --- |
| [ ] | `shownState` | `WidgetShownState` | false |
| [ ] | `tooltipLoc` | `UIWidgetTooltipLocation` | false |
| [ ] | `itemInfo` | `UIWidgetItemInfo` | false |
| [ ] | `widgetSizeSetting` | `number` | false |
| [ ] | `textureKit` | `textureKit` | false |
| [ ] | `frameTextureKit` | `textureKit` | false |
| [ ] | `hasTimer` | `bool` | false |
| [ ] | `orderIndex` | `number` | false |
| [ ] | `widgetTag` | `string` | false |
| [ ] | `inAnimType` | `WidgetAnimationType` | false |
| [ ] | `outAnimType` | `WidgetAnimationType` | false |
| [ ] | `widgetScale` | `UIWidgetScale` | false |
| [ ] | `layoutDirection` | `UIWidgetLayoutDirection` | false |
| [ ] | `modelSceneLayer` | `UIWidgetModelSceneLayer` | false |
| [ ] | `scriptedAnimationEffectID` | `number` | false |

### UIWidgetItemInfo — complete projection

| Requirement | Field | Type | Nullable |
| --- | --- | --- | --- |
| [ ] | `itemID` | `number` | false |
| [ ] | `stackCount` | `number` | true |
| [ ] | `overrideItemName` | `string` | true |
| [ ] | `infoText` | `string` | true |
| [ ] | `overrideTooltip` | `string` | true |
| [ ] | `textDisplayStyle` | `ItemDisplayTextDisplayStyle` | false |
| [ ] | `tooltipEnabled` | `bool` | false |
| [ ] | `iconSizeType` | `WidgetIconSizeType` | false |
| [ ] | `infoTextEnabledState` | `WidgetEnabledState` | false |
| [ ] | `showAsEarned` | `bool` | false |
| [ ] | `itemNameTextFontType` | `UIWidgetFontType` | false |
| [ ] | `itemNameTextSizeType` | `UIWidgetTextSizeType` | false |
| [ ] | `infoTextFontType` | `UIWidgetFontType` | false |
| [ ] | `infoTextSizeType` | `UIWidgetTextSizeType` | false |
| [ ] | `itemNameCustomColor` | `WidgetEnabledState` | false |
| [ ] | `itemNameCustomColorOverrideState` | `UIWidgetOverrideState` | false |

## How it works

- [Manager state and feeds](../wiki/systems/ui-widget-manager-state.md)

## Implementation inventory

Getter production source `64a5576b1` has [accepted bounded Retail execution](#bounded-retail-acceptance--2026-10-16) at saved normal source `bbeac6213`; PTR remains pending. Namespace registration is unconditional, matching neighboring getters; selector handling reuses `read_numeric_id` without a new policy.

- `src/c_api/ui_widget_manager/queries.rs` — shared required-number/secret selector and modeled ItemDisplay getter registration.
- `src/c_api/ui_widget_manager/item_display.rs` — fresh rooted outer/nested snapshots, complete15+16 projection, omitted None fields and one-nil absence; no defaults or new selector policy. Bounded Retail getter execution is accepted; PTR remains pending.
- `src/c_api/ui_widget_manager/inputs.rs` — empty per-environment `item_display` map; no seeded records.
- `src/c_api/ui_widget_manager/feed.rs` — `WidgetVisualization::ItemDisplay`, type27 and supplied storage arm; existing feed validation and notification ordering unchanged.

- `src/c_api/ui_widget_manager/item_display_record.rs` — complete15-field `ItemDisplayRecord` and16-field `UiWidgetItemRecord`; four nullable fields use `Option`, no defaults.
- `src/c_api/ui_widget_manager.rs` — getter/record modules and public exports. Passive normal library source `3eab91eeb` compiled before populated RED; getter source is a later change with separately accepted bounded Retail execution.

## Tests asserting this spec

- `tests/ui_widget_item_display.rs` plus `tests/widget_item_display/{queries,feed,security,consumer}.rs` — eight logical cases reached the populated nil/non-table RED boundary against passive normal Retail source `3eab91eeb`. The unchanged eight populated cases and two original selector cases later PASS under the bounded Retail receipt below; historical RED boundaries remain unchanged.

- `tests/ui_widget_item_display_selector.rs` — two public-selector regressions at test source `f67005a12a07661592c6a4f5615015102db5b9f3`; fresh `WowLuaEnv` instances, no mock registration, supplied records or cached UI load.

### Historical Retail selector RED — 2026-10-16

Saved runtime epoch: `/home/osso/.local/state/wow-ui-sim/verification/wrath-source-current/item-display-selector-retail-red-20261016T135057967011Z`. Component compile exits0 with no Rust diagnostics; tests link separately sealed normal Retail library `7273875b2d35ae305428661b2650116e802733e5`, not a whole-f670 build.

Exactly two tests run: **0 PASS / 2 FAIL / exit101**. Missing-input and explicit-nil cases each pass the shared `GetWidgetSetInfo` required-number control, public getter function check and absent numeric `934207.5` exactly-one-nil assertions. Each fails `ItemDisplay invalid widgetID must reject`: the invalid target call succeeds. Target diagnostics and later string, numeric-string, boolean, table and function iterations are **UNREACHED**.

Independent full-stream/source/feature/seal/CVar/readability RED reconciliation: `/home/osso/.local/state/wow-ui-sim/verification/item-widget-next/selector-red-report.md`. Child-only current-CVar copy and paired original stat/hash are unchanged at their observed boundaries. No reset/restore or historical preservation claim. This confirms bounded RED, not selector acceptance; PTR, secret wrappers, populated projection, cached consumers, native parity and parent acceptance remain unproved.

### Populated Retail RED before getter — 2026-10-16

Normal Retail `--lib` build source `3eab91eebdbe588f579f45d7bfe64ad950459526` (passive production `417f1badd`) finished successfully with730 Cargo JSON records and zero compiler-message records; pressure guard exited before sealing. Library epoch: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/item-widget-passive-retail-library-retry1-20261016T151704000183Z`.

Component epoch `/home/osso/.local/state/wow-ui-sim/verification/wrath-source-current/item-display-populated-retail-red-20261016T152638018148Z` snapshots the committed test/common inputs separately. Matching emitted Retail cfg and normal-profile externs compile successfully with38 inherited helper warnings. Runtime reaches **0 PASS / 8 unique FAIL / exit101** at `populated ItemDisplay getter returned nil/non-table`; parent/timeout-child prints are nonadditive. Cached consumer fails its populated precondition before vendor loading. Complete field values, nullable transitions, feed rejection/state preservation, later secret-policy controls and cached lifecycle assertions remain **UNREACHED**, not accepted.

Fresh child-only current-CVar copy and paired original stat/hash remain unchanged. No reset/restore, native/catalog/assets/pixels, PTR or whole-integration claim. Independent saved-evidence reconciliation: `/home/osso/.local/state/wow-ui-sim/verification/item-widget-next/populated-red-report.md` accepts eight unique behavioral failures at the one-return non-table boundary, source snapshots, seals, emitted cfg/externs and paired CVar guards. The diagnostic does not distinguish nil from another non-table scalar. New getter execution remains separate.

## Bounded Retail acceptance — 2026-10-16

**ACCEPTED: bounded Retail only; PTR pending.** Independent saved-evidence receipt: `/home/osso/.local/state/wow-ui-sim/verification/item-widget-next/retail-green-report.md`. Normal Retail library/binary source `bbeac62133cfbf678faf3eca07eab27f6b9bcec8` includes production getter `64a5576b1fa7b8f21bcbade605a0058ba8366d88`; production inputs between those revisions are unchanged. Build exit0, 7m32s, 731 Cargo JSON records, zero compiler-message diagnostics; terminal pressure guard exited and library/binary seals reconciled. Separate component compile exit0 retains 38 inherited helper warnings. All 33 immutable fixture/common snapshots match committed `bbeac6213` test bytes, unchanged from `3eab91eeb`.

Ten unique tests PASS/exit0: eight unchanged populated cases plus two original selector regressions; timeout-child replays are nonadditive. Reached complete 15+16 projection, fresh/deep retained snapshots, four nullable fields including Some(0)/empty strings, replacement/fractional/environment/removal isolation, synchronous publication and rejection atomicity, and actual permitted/denied secret wrappers with taint/state preservation. Actual cached type27/XML BaseItem completes synchronous item loading: supplied counts17/23, nil optional text/count hiding, live identity/update, hide/reacquire, membership/full-set removal, live unregister, color-callback cleanup and later-feed inactivity. Text/texture identity and completed sizing are accepted, not assets or pixels. Historical selector RED and eight-populated RED above remain retained, not reclassified.

Separate sealed normal Retail startup PASS/exit0: stdout `[]\n`, final `Status: CLEAN`, zero Lua errors. Both runs use distinct fresh child-only `XDG_DATA_HOME` byte-exact copies of the then-current persisted CVar file; paired original stat/hash unchanged, no reset/restore. Startup child later differs only in serialization, not parsed values. Both runs disable CASC/sound; startup disables third-party addons/SavedVariables. This is not full integration or all-assets/all-addons proof.

Source/schema/readability audit separately PASS; it does not establish current type-check or whole-project formatter acceptance. PTR is only queued via `/home/osso/.local/state/wow-ui-sim/handoff/item-widget-getter-ptr-green-admission-current.txt`; no PTR acceptance follows. No native/catalog, assets/pixels, alternate text-style, asynchronous pending-load/cancellation, projected animation/effect/timer activation, whole-integration or parent-goal acceptance. All generic dual-profile acceptance checkboxes remain open pending PTR.

## Known gaps (current cycle)

- [ ] Establish selector acceptance under Retail/PTR: bounded Retail accepted above; PTR pending. Historical Retail RED remains retained.
- [x] Implement the modeled ItemDisplay getter/projection after populated behavioral RED: source `64a5576b1`; bounded Retail execution accepted separately above, PTR pending.
- [x] Compile and execute the populated fixtures separately against the passive normal Retail library: eight unique behavioral failures before getter implementation.
- [ ] Complete dual-profile getter and later-assertion acceptance: bounded Retail 15+16 projection, nullable, feed, actual secret wrappers and completed synchronous cached-consumer assertions accepted above; PTR pending. Generic dual-profile checkboxes remain unchecked.

## Out of scope

Native parity/catalog/assets/pixels, other profiles, new selector restrictions/policy, new item providers, asynchronous pending loads/cancellation races, WorldQuestReward embedded-tooltip behavior, animation/timer/effect activation and broader parent acceptance remain outside this slice. Only the existing synchronous completed item-load model is included; no vendor behavior may be replaced.
