# Supplied DoubleIconAndText widgets

`C_UIWidgetManager.GetDoubleIconAndTextWidgetVisualizationInfo(widgetID)` must expose explicit supplied records and drive the unchanged cached Retail consumer. The [manager-state contract](ui-widget-manager-state.md) owns supplied state and lifecycle; the [manager architecture](../wiki/systems/ui-widget-manager-state.md) describes existing feeds. The modeled getter is authored after the populated baseline below; producer execution remains pending. At source `a56e11267`, the first public-selector regression reaches the required-number rejection failure after its existing-manager control passes: one unique case FAIL, exit101. Nested parent/child summaries are not two independent cases. Passive supplied state is now authored below; it is not getter or populated-result acceptance.

## What it must do

- [ ] Require a numeric `widgetID` through the existing manager required-number/secret policy. Missing/non-number selectors reject rather than returning nil; unknown numeric selectors return exactly one nil.
- [ ] Return complete fresh snapshots of supplied `DoubleIconAndTextWidgetVisualizationInfo` fields and Lua types, without invented defaults. Caller mutation, replacement, removal, distinct IDs and environments remain isolated.
- [ ] Existing normal feeds publish supplied state before notifications and reject wrong-type/absent-set updates atomically.
- [ ] Actual cached type5 consumers activate/update label, left/right text and tooltips, and layout from supplied state; hidden transitions, membership removal and live unregister clean up. Preserve acquisition tags during ordinary updates and use the current tag on reacquisition.

The active Retail generated `UIWidgetManagerDocumentation.lua` declares the required number, nullable record and `AllowedWhenUntainted` secret policy. Its record fields are `shownState`, `label`, `leftText`, `leftTooltip`, `rightText`, `rightTooltip`, `tooltipLoc`, `widgetSizeSetting`, `textureKit`, `frameTextureKit`, `hasTimer`, `orderIndex`, `widgetTag`, `inAnimType`, `outAnimType`, `widgetScale`, `layoutDirection`, `modelSceneLayer`, and `scriptedAnimationEffectID`. The active cached `Blizzard_UIWidgetTemplateDoubleIconAndText.lua` is the consumer target; these declarations do not establish execution or native parity.

## How it works

- [Manager state and feed](../wiki/systems/ui-widget-manager-state.md)
- [Event dispatch](../event-system.md)

## Implementation inventory

- `src/c_api/ui_widget_manager/double_icon_and_text_record.rs`: complete explicit supplied record, without defaults or demo seeds.
- `src/c_api/ui_widget_manager/{inputs,feed}.rs`: environment-owned initially empty map and typed type5 host feed through existing membership/event ordering.
- `src/c_api/ui_widget_manager.rs`: public record export.

- `src/c_api/ui_widget_manager/double_icon_and_text.rs`: uses shared required-number/secret handling, clones the supplied record before Lua allocation, roots and fills a fresh result table, and returns exactly one nil for absent IDs.
- `src/c_api/ui_widget_manager/queries.rs`: registers the modeled getter.

The passive scaffold and fixtures compiled at source1571253e9; getter source `c3096cd6b` is **UNCOMPILED / UNEXECUTED**. Source inspection shows all 19 explicit fields (10 numbers, eight strings, one boolean), shared `read_numeric_id`, record cloning before allocation, and a rooted fresh Lua result; this is not behavioral acceptance. No default records, demo seeds or vendor modifications are introduced.

## Tests asserting this spec

- `tests/ui_widget_double_icon_selector.rs`: first public missing-selector counterexample, with existing required-number behavior as its control. Executed at sourcea56: one unique FAIL/exit101 at “missing widgetID must reject, not return nil”; the failed assertion establishes successful target call, not independently its returned value. Environment/control setup passed; later target error-message assertions were unreached. Independent saved audit: `/home/osso/.local/state/wow-ui-sim/verification/doubleicon-selector-next/runtime-red-report.md`. No passive scaffold dependency or populated/lifecycle proof.

- `tests/ui_widget_double_icon_and_text.rs` and `tests/widget_double_icon/`: nine supplied-result cases executed at source1571253e9: **1 unknown-ID control PASS / 8 FAIL, exit101**. Seven failures reach supplied-table assertions (including the cached case before consumer setup); one reaches invalid-selector acceptance. Nested timeout parent/child summaries are the same nine identities, not eighteen. Later projection, tainted denial/recovery and cached lifecycle phases remain unreached. Producer GREEN remains pending. Authored assertions target complete fields/freshness/replacement (including distinct in/out animation values), numeric selectors and actual secret wrappers, distinct IDs/removal/restoration, environments, normal-feed state-before-event/rejection atomicity, and unchanged cached activation/update/Hidden/reacquisition/membership removal/live unregister. Feed callback failures are retained and asserted outside handlers. Saved private baseline `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/doubleicon-populated-retail-red-20261015T214331907251Z/doubleicon-populated-first-red` owns source1571253e9 execution; getter source `c3096cd6b` follows these reached failures. Independent saved-runtime reconciliation at `/home/osso/.local/state/wow-ui-sim/verification/doubleicon-populated-next/runtime-red-report.md` confirms all nine unique identities, full streams and build/seal receipts; the cached case failed before consumer loading, so cached lifecycle was not reached.

## Known gaps (current cycle)

- [x] Execute the public selector regression and independently distinguish its reached rejection-contract failure from compilation/setup failure; retained as RED, not acceptance.
- [x] Integrate explicit record/map/feed scaffold and source-audited populated query/secret/isolation/cached-lifecycle fixtures, correcting swallowed callback failures and ambiguous animation-field values. Source review is not compilation or behavior proof.
- [x] Compile and execute the nine supplied-result cases before the getter: source1571253e9 baseline reaches 1 control PASS /8 contract FAIL, exit101; not acceptance. Independent reconciliation is separate.
- [ ] Compile/execute the authored modeled getter against the behavioral counterexamples, then verify focused behavior, profile preservation and normal startup. Full parent acceptance remains separate.

## Out of scope

Native catalogs/defaults, demo seeds, DB2 inference, vendor patches, mocked providers/Setup/pools, rendered/native parity and cross-profile acceptance. Synthetic fixture inputs must be explicit supplied values, not claimed native metadata. No asset-application or pixel proof follows from text/layout assertions.
