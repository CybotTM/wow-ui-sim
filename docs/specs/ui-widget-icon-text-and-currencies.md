# Supplied IconTextAndCurrencies widgets

`C_UIWidgetManager.GetIconTextAndCurrenciesWidgetVisualizationInfo` exposes explicit environment-owned supplied records to the unchanged cached type-7 consumer. Bounded Retail GREEN is independently accepted below; PTR/check gates remain pending. See [manager state and feeds](../wiki/systems/ui-widget-manager-state.md).

## What it must do

### Eligibility and public query

- [ ] Support Retail OR PTR builds, grounded in each profile's actual cached `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua` declaration and `Blizzard_UIWidgets/Blizzard_UIWidgetTemplateIconTextAndCurrencies.lua` registration/consumer. Both cached shared enum declarations give `Enum.UIWidgetVisualizationType.IconTextAndCurrencies = 7`. No other-profile acceptance is claimed.
- [ ] Require numeric `widgetID` using the existing manager required-number and secret policy, including contextual rejection of missing, explicit nil, string (including numeric string), boolean, table and function arguments. Preserve declared `SecretArguments = "AllowedWhenUntainted"` through that shared policy, with actual permitted/denied secret-wrapper coverage; do not introduce a new policy or integer/positive/range restrictions.
- [ ] An absent numeric ID returns exactly one nil. Numeric IDs never imply records or catalog membership.
- [ ] Project every supplied field below, all non-nil, preserving values and declared Lua types: exactly 20 outer fields and all 10 fields of every ordered `UIWidgetCurrencyInfo` row. Enum/fileID values are numeric scalars, texture kits are strings; do not clamp, synthesize defaults, or omit fields unused by Setup.

### Outer IconTextAndCurrenciesWidgetVisualizationInfo (20 fields)

| Exact field | Cached documented type |
| --- | --- |
| `shownState` | `WidgetShownState` |
| `enabledState` | `WidgetEnabledState` |
| `descriptionShownState` | `WidgetShownState` |
| `descriptionEnabledState` | `WidgetEnabledState` |
| `text` | `string` |
| `description` | `string` |
| `currencies` | `table` of `UIWidgetCurrencyInfo` |
| `tooltipLoc` | `UIWidgetTooltipLocation` |
| `widgetSizeSetting` | `number` |
| `textureKit` | `textureKit` |
| `frameTextureKit` | `textureKit` |
| `hasTimer` | `bool` |
| `orderIndex` | `number` |
| `widgetTag` | `string` |
| `inAnimType` | `WidgetAnimationType` |
| `outAnimType` | `WidgetAnimationType` |
| `widgetScale` | `UIWidgetScale` |
| `layoutDirection` | `UIWidgetLayoutDirection` |
| `modelSceneLayer` | `UIWidgetModelSceneLayer` |
| `scriptedAnimationEffectID` | `number` |

### Nested UIWidgetCurrencyInfo (10 fields per row)

| Exact field | Cached documented type |
| --- | --- |
| `iconFileID` | `fileID` |
| `leadingText` | `string` |
| `text` | `string` |
| `tooltip` | `string` |
| `isCurrencyMaxed` | `bool` |
| `textFontType` | `UIWidgetFontType` |
| `textSizeType` | `UIWidgetTextSizeType` |
| `textEnabledState` | `WidgetEnabledState` |
| `iconSizeType` | `WidgetIconSizeType` |
| `updateAnimType` | `UIWidgetUpdateAnimType` |

### Snapshots and isolation

- [ ] Each query returns a fresh outer table, fresh `currencies` array and fresh row tables, preserving supplied order and empty arrays. Concrete complete multi-row records demonstrate all 20+10 fields, not merely fields read by Setup. Caller edits, row/array insertion/removal and retained snapshots cannot mutate supplied state or subsequent results.
- [ ] Replacements affect later queries only. Distinct numeric IDs, record absence/removal/restoration, and separate fresh `WowLuaEnv` instances remain isolated, using existing manager numeric identity rules without new selector restrictions.

### Normal feed and unchanged cached type-7 lifecycle

- [ ] Use existing normal `WowLuaEnv::replace_ui_widget_set`, `update_ui_widget` and `remove_ui_widget` feeds with explicit complete records and type-7 membership. State is observable before synchronous `UPDATE_UI_WIDGET`; wrong-type/absent-set updates reject atomically without mutation or notification. Membership removal/full-set replacement reconciles through `UPDATE_ALL_UI_WIDGETS` while independent visualization records remain queryable.
- [ ] Execute actual cached Retail/PTR manager and template activation through the real getter and supplied membership, without replacing provider, getter, Setup, pools or vendor code. Assert actual text, enabled/desaturated state (including cached Yellow-to-White text handling), conditional description visibility/content/enabled state, currency acquisition count/order and concrete observable supplied row content. Assert requested `Icon` texture-kit region name via cached `%s` policy only, not asset availability or pixels.
- [ ] Exercise ordinary supplied updates, two-widget supplied ordering/layout, full-set replacement, hidden release/shown reacquisition, membership removal and live unregister cleanup (pools, subscriptions and stopped updates). Allow normal cached deferred layout/removal before assertions; no vendor timing shortcuts. Ordinary updates retain acquisition `widgetTag`; reacquisition uses current supplied tag, while queries expose every replacement field. This is supplied-state lifecycle, not native animation/timing parity.

## How it works

- [Manager state and feeds](../wiki/systems/ui-widget-manager-state.md)
- [Event dispatch](../event-system.md)

## Implementation inventory

Records/map/feed/getter and fresh projection compile in normal Retail source `7273875b2`; bounded Retail GREEN below does not close combined Retail/PTR acceptance:

- `src/c_api/ui_widget_manager/icon_text_and_currencies_record.rs` — explicit20-field `IconTextAndCurrenciesRecord` with ordered10-field `UiWidgetCurrencyRecord` rows, no defaults/seeds; fileID and enum fields follow existing `u32` host-input conventions, plain numbers use `f64`.

- `src/c_api/ui_widget_manager/icon_text_and_currencies.rs` — shared required-number/secret selection, exact-one-nil absence and complete fresh rooted outer/array/row projection.
- `src/c_api/ui_widget_manager/queries.rs` — namespace registration using the existing shared selector policy.
- `src/c_api/ui_widget_manager/feed.rs` — typed `IconTextAndCurrencies` variant/type7 and supply arm; existing validation/state-before-event behavior unchanged.
- `src/c_api/ui_widget_manager/inputs.rs` — empty environment-owned `icon_text_and_currencies` map using existing numeric identity.
- `src/c_api/ui_widget_manager.rs` — public record exports; no default/demo catalog or new identity policy.

## Tests asserting this spec

- `tests/ui_widget_icon_text_and_currencies_selector.rs` — historical RED: newly compiled test source `825d7bf84` against sealed normal Retail library `84da38a3b`, not a whole825 library. Compile exit0/no Rust diagnostics; actual **0 PASS / 2 FAIL / exit101**. Existing `GetWidgetSetInfo` missing/nil required-number controls, public getter presence and absent numeric ID934207.5 exactly-one-nil controls pass before each target failure. Target missing/nil calls succeed instead of rejecting; target diagnostic checks and later numeric-string/boolean/table/function iterations are unreached. No supplied data, mocks, source-shape assertions or cached load. Receipt `/home/osso/.local/state/wow-ui-sim/verification/wrath-source-current/iconcurrencies-selector-retail-red-20261016T113344496157Z`; independent source/spec/readability/seal/full-stream RED attribution PASS: `/home/osso/.local/state/wow-ui-sim/verification/iconcurrencies-next/selector-red-report.md`. This is failing behavior, not feature acceptance or actual PTR execution.

- `tests/ui_widget_icon_text_and_currencies.rs` and `tests/widget_icon_currencies/` — historical populated RED, superseded by bounded Retail GREEN below: nine populated cases (eight without the security epoch): complete20+10-field deep snapshots/all-field replacement, environment/fractional-ID/removal isolation, fresh empty arrays/restoration, normal-feed publication/atomicity, actual secret wrappers and one combined unchanged cached type7 lifecycle. Concrete multirow data and recursive public-result assertions; existing common/snapshot helpers reused, not replaced. Consumer covers row anchor order/content, Yellow-to-White/disabled presentation, description branches, empty rows/restoration, two-widget sorting/full-set replacement, hidden/reacquire/tag behavior and live unregister/inactive feed. At test/library source `a2d2d2792`, standalone compilation with real common helpers and emitted normal Retail features/dependencies exits0 (38 inherited unused-helper warnings); runtime is **0 PASS / 9 unique FAIL / exit101**. All cases reach a populated query and fail on nil (`wrong type: widget`); consumer fails before cached setup. Nested child/parent summaries are nonadditive; later assertions remain uncredited. Receipt: `/home/osso/.local/state/wow-ui-sim/verification/wrath-source-current/iconcurrencies-populated-retail-red-20261016T121727570751Z`; independent full-stream/source/seal reconciliation PASS: `/home/osso/.local/state/wow-ui-sim/verification/iconcurrencies-next/populated-red-report.md`. Requested icon/atlas identity is not asset/pixel proof.

Independent passive source/spec/readability PASS at source `7e3586e97`: `/home/osso/.local/state/wow-ui-sim/verification/iconcurrencies-next/passive-source-report.md`. That review establishes schemas, empty map, exports and typed feed storage only. Subsequent normal Retail `--lib` build at `a2d2d2792` (production basis `7e3586e97`) exits0: 730 JSON records, zero compiler diagnostics, 7m05s, guard exited and emitted library sealed. Independent saved-build acceptance: `/home/osso/.local/state/wow-ui-sim/verification/iconcurrencies-next/passive-library-report.md`. No getter or populated GREEN follows from that historical passive build; current Retail evidence follows below.

## Bounded Retail execution — 2026-10-16

Normal Retail source `7273875b2`: `--lib --bin` build9m45s/terminal0, guard exited, artifacts sealed. Unchanged nine populated plus two original selectors: **11 unique PASS / 0 FAIL / exit0**, component compile0/38 inherited unused-helper warnings; nested summaries are nonadditive. Actual unchanged cached type-7 lifecycle assertions reached. Separate startup `[]`/full stderr CLEAN; current CVar copy and original unchanged. Independent full-stream/source/seal/runtime acceptance: `/home/osso/.local/state/wow-ui-sim/verification/iconcurrencies-next/retail-green-report.md`. PTR normal build running and default check pending. Combined Retail/PTR requirements remain unchecked; no native/catalog/assets/pixels/effect-activation/whole-integration/full-parent acceptance.

Component receipt: `/home/osso/.local/state/wow-ui-sim/verification/wrath-source-current/iconcurrencies-retail-green-component-20261016T123446079197Z`; test source `0eba46a74`, sealed normal Retail library basis `7273875b2`, not whole-integration compilation. Separate startup receipt: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/iconcurrencies-retail-green-20261016T122137194568Z`.

## Known gaps (current cycle)

- [x] Establish and independently reconcile the selector baseline, preserving missing/nil failures and later inputs as unreached. This closes RED provenance only.
- [x] Compile passive20+10-field records/map/feed and reproduce populated missing-query RED before projection. Independent RED reconciliation passed; later bounded Retail snapshot/isolation/shared secret-policy GREEN is recorded above; PTR remains pending.
- [x] Execute unchanged normal-feed and cached type-7 lifecycle fixtures in bounded Retail: included in11 unique PASS and independent Retail acceptance above.
- [ ] Complete separate PTR execution and default check; combined Retail/PTR requirements remain open.

## Out of scope

- Native execution/parity, reconstructed C++/wowdecomp/reconstruction reports, inferred native/default/catalog data: cached declarations and supplied state do not establish these.
- Asset existence, rendered pixels, animation execution/timing or native catalog defaults: names, numeric fileIDs and data projection are not proof.
- Other profiles, new secret/numeric identity policy and vendor overrides: outside this bounded Retail/PTR supplied-record contract.
