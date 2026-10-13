# Supplied TextWithState visualization — unexecuted input stage

Host-supplied TextWithState input belongs to `src/c_api/ui_widget_manager/`; the [widget manager architecture](../wiki/systems/ui-widget-manager-state.md) owns existing supplied-record/event behavior. This staged input adds only a complete DTO, environment-local map and typed host-feed insertion. Getter projection and Lua query registration are intentionally absent. No executed RED, GREEN, compiler or cached-consumer acceptance exists for this stage.

## What it must do

- [ ] `C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo(number)` returns exactly one nil for an unknown input, otherwise a fresh complete21-field snapshot preserving supplied strings/numbers/enums/bool without inferred defaults or consumer clamping. Caller mutation, host replacement/deletion and another environment cannot alter retained snapshots.
- [ ] Numeric keys and strict non-number rejection follow existing `WidgetId` and `queries::read_numeric_id`, not a new ID-range policy. Real host-secret NUMBER arguments are allowed when untainted, denied before supplied/absent lookup when tainted; public IDs and public snapshot fields recover normally. Both cached schemas explicitly declare `SecretArguments = "AllowedWhenUntainted"`; no new secrecy policy is invented.
- [ ] The existing typed host feed accepts TextWithState type8, validates supplied-set existence and entry/visualization type before mutation, stores complete input and membership before synchronous `UPDATE_UI_WIDGET`, and retains independent visualization after membership removal through `UPDATE_ALL_UI_WIDGETS`.
- [ ] The unchanged active Retail/PTR cached type8 manager/template activates through the real `UIWidgetContainerTemplate` constructor, consumes text/tooltip/width/font/enabled-state/alignment/bottom-padding/order/scale/layout fields, processes incremental updates, suppresses Hidden, restores visible membership, and releases active frames on membership removal. Flush actual OnUpdate before pool/count assertions. Query snapshots remain independent of consumer clamping.

Literal fields from both cached `TextWithStateWidgetVisualizationInfo` declarations (all non-nil): `shownState`, `enabledState`, `text`, `tooltip`, `textSizeType`, `fontType`, `bottomPadding`, `tooltipLoc`, `hAlign`, `widgetSizeSetting`, `textureKit`, `frameTextureKit`, `hasTimer`, `orderIndex`, `widgetTag`, `inAnimType`, `outAnimType`, `widgetScale`, `layoutDirection`, `modelSceneLayer`, `scriptedAnimationEffectID`.

Explicit fixtures are host-input test policy, not native catalog records. Consumer-safe enum choices are read from both cached API-doc enum declarations and compared against loaded enums after the supplied getter boundary: Shown1/Hidden0, White3/Green4, Normal0/Shadow1, Medium16Pt1/Standard14Pt4, Top4, Left0/Right2, animation None0, scale OneHundred0, layout Vertical1, model layer None0, set Vertical0, visualization TextWithState8. Query-only texture-kit strings, timer true, fractional scripted effect ID and negative order values never enter the consumer fixture. Negative consumer padding exercises the unchanged template's clamp to zero; query snapshots preserve the supplied negative value.

## How it works

- [Widget manager state](../wiki/systems/ui-widget-manager-state.md)
- [Existing manager/feed contract](ui-widget-manager-state.md)
- [Event system](../event-system.md)

## Implementation inventory

- `src/c_api/ui_widget_manager/text_state_record.rs`: explicit DTO21 only; no Default record.
- `src/c_api/ui_widget_manager/inputs.rs`: empty-by-default `text_with_state` map keyed by existing `WidgetId`.
- `src/c_api/ui_widget_manager/feed.rs`: public `WidgetVisualization::TextWithState` type8 and insertion in existing feed before notification; no new transport/event implementation.
- `src/c_api/ui_widget_manager.rs`: public DTO export/module wiring.

No getter producer or query registration is added to this stage. Main owns those after actual RED.

## Tests asserting this spec

`tests/ui_widget_manager_text_state.rs` is discovered by existing `integration_tests!()` build harness; it explicitly includes `tests/widget_text_state/{fixtures,queries,security,feed,consumer}.rs`. Reuses existing `tests/widget_discrete/fixtures.rs::assert_lua` error capture and `common::blizzard_addon_harness` isolated Game environment/closure loader; no copied broad harness, vendor behavior replacement or fake global consumer.

Six unconditional cases under Retail/PTR: complete snapshot1, strict numeric1, environment isolation1, feed2, cached consumer1. A seventh real-secret case follows existing Discrete `retail-12-1-0` gate. No case has been compiled or executed. Intended first supplied-query failures precede cache loading, constructor and security wrapper prerequisites; the first feed callback checks a supplied table immediately, before later transitions. Rejected-feed test can already pass with the passive scaffold; it is not expected to prove missing-query RED. If the getter itself is absent rather than an existing nil compatibility closure, the publication assertion fails first; preserve that result separately and do not call it supplied-result RED.

## Known gaps (current cycle)

- [ ] New source-coverage gap baseline: no supplied TextWithState producer/registration or previous cached type8 lifecycle coverage was found in the inspected source. This is not a test result or native-availability conclusion.
- [ ] Main must integrate/reconcile this candidate and physically execute bounded RED before implementing a getter/registration; preserve exact reached failure boundary and unreached assertions.
- [ ] Main owns subsequent fresh complete21 projection with existing numeric/secret validator, GREEN and actual Retail/PTR cached-consumer acceptance. This input stage performs no implementation or proof at that layer.

## Out of scope

Native catalogs, native events, transport, rendered/pixel output, 3D, reference counting and allocation/lifetime semantics remain unknown/excluded. Native enum admissibility beyond reading declarations, real server fixtures, native quotas, unit-context behavior, unregister coverage, other profiles and full widget-system acceptance are not asserted. No vendor/cache/Wowless/WTF/settings changes. Runtime acceptance remains unverified.
