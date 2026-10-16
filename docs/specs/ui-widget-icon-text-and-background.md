# Supplied IconTextAndBackground widgets

`C_UIWidgetManager.GetIconTextAndBackgroundWidgetVisualizationInfo(widgetID)` must project explicit environment-local supplied records for the unchanged cached Retail/PTR consumer. The source boundary is `src/c_api/ui_widget_manager/`; the [manager-state contract](ui-widget-manager-state.md) owns shared supplied-state behavior and the [manager architecture](../wiki/systems/ui-widget-manager-state.md) describes the existing lifecycle. Selector tests have an independently reconciled two-case PTR RED against normal-library source `161dad02a`; this is failing behavior, not acceptance. Passive record/map/feed plumbing follows that baseline without a modeled getter. This is not populated-query, feed, consumer or native acceptance.

## What it must do

### Source eligibility and query

- [ ] Support this record on Retail/PTR, where both cached `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua` declarations and the registered `Blizzard_UIWidgets/Blizzard_UIWidgetTemplateIconTextAndBackground.lua` consumer exist. Eligibility rests on those actual sources, not a callable namespace entry or an inferred native catalog. No other-profile acceptance is claimed.
- [ ] Require numeric `widgetID` using the existing manager required-number/secret policy. Missing, nil and non-number arguments (including numeric strings) reject at that boundary. Preserve documented `SecretArguments = "AllowedWhenUntainted"` through the shared policy; do not introduce a new secret policy. An absent numeric ID returns exactly one nil.
- [ ] Project all 14 non-nil fields below with their supplied values and documented types; do not invent defaults or truncate the record to fields read by Setup. Enum values are supplied numeric scalars; texture kits are supplied strings.

| Exact field | Cached documented type |
| --- | --- |
| `shownState` | `WidgetShownState` |
| `text` | `string` |
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

- [ ] Each query returns a fresh snapshot. Caller mutation, retained snapshots, supplied replacements, distinct IDs and separate `WowLuaEnv` instances stay isolated. Record absence/removal and restoration affect subsequent queries only. Use the existing manager numeric identity policy; do not add integer, positivity or inferred-range restrictions.

### Normal feed and cached lifecycle

- [ ] Existing normal host feeds accept the supplied visualization with the documented `Enum.UIWidgetVisualizationType.IconTextAndBackground` membership type. Store state before synchronous `UPDATE_UI_WIDGET`; wrong-type and absent-set updates reject without state mutation or notification. Membership removal/full set replacement uses existing `UPDATE_ALL_UI_WIDGETS` reconciliation and preserves independently queryable visualization records.
- [ ] Actual cached Retail/PTR manager/template execution acquires this consumer from supplied membership, projects through the real getter, sets `Text` from `text`, and applies `textureKit` to `Icon`/`Glow` using the cached `%s-icon`/`%s-glow` region policy. Exercise supplied updates and base ordering/layout through normal feeds; do not replace providers, getter, Setup, pools or vendor behavior. Texture-region application is not proof of asset availability or rendered pixels.
- [ ] Actual cached consumers release hidden widgets and reacquire shown widgets, remove consumers after membership removal, and stop updates/clean pools and subscriptions on live unregister. Let cached deferred layout/removal finish before assertions. Ordinary updates retain the acquisition `widgetTag`; reacquisition uses the current supplied tag. Snapshot replacement still exposes every updated field. These are supplied-state lifecycle requirements, not native timing/animation parity.

## How it works

- [Manager state and feeds](../wiki/systems/ui-widget-manager-state.md)
- [Event dispatch](../event-system.md)

## Implementation inventory

Passive supplied-input plumbing is present but uncompiled/unexecuted; the modeled getter is still absent. No populated-query or lifecycle acceptance follows:

- `src/c_api/ui_widget_manager/icon_text_and_background_record.rs` — complete explicit 14-field input record, without defaults or seeds.

- `src/c_api/ui_widget_manager.rs` — public supplied records and shared numeric widget identity.
- `src/c_api/ui_widget_manager/inputs.rs` — environment-owned supplied inputs.
- `src/c_api/ui_widget_manager/queries.rs` — existing modeled-query registration and shared required-number/secret boundary; no modeled `IconTextAndBackground` getter registration yet.
- `src/c_api/ui_widget_manager/feed.rs` — typed visualization feeds and synchronous state-before-event lifecycle.

## Tests asserting this spec

- `tests/ui_widget_icon_text_and_background_selector.rs` — auto-discovered by the internal integration test harness on Retail/PTR; two public-selector cases (missing; explicit nil/numeric string/boolean/table/function), fresh `WowLuaEnv` and existing `GetWidgetSetInfo` required-number control for each invocation. No record scaffold, mocks or provider replacement. Main-observed PTR baseline: compile exit0; two unique FAIL, exit101, after shared controls. Missing-selector and explicit-nil target calls succeed instead of rejecting; later non-number loop inputs are unreached. Receipt: `/home/osso/.local/state/wow-ui-sim/verification/wrath-source-current/iconbackground-selector-ptr-first-red-20261016T095402891303Z`. The newly compiled test source is `e1a2ce806`; actual normal-library basis is `161dad02a`, not a whole-e1 library. Independent report: `/home/osso/.local/state/wow-ui-sim/verification/iconbackground-next/selector-red-report.md`. Its inferred missing-source-wiring finding is rejected: the internal harness generates module references for top-level test files; whole-integration execution remains unproved. A successful target call fails rejection; an unrelated missing-function error fails the required-number diagnostic assertion. The test does not assert what a successful call returned.
- `tests/ui_widget_icon_text_and_background.rs` and `tests/widget_icon_background/` — eight authored cases: complete14-field fresh/replacement snapshots, distinct fractional IDs/removal/restoration, required selectors, independent environments, normal-feed ordering/atomicity, actual secret-wrapper access and unchanged cached type4 lifecycle. The consumer includes two-widget ordering and full-set replacement. Existing `widget_two_records/helpers.rs` is reused, not replaced. Compilation/populated RED is pending; later assertions are not reached evidence.

## Known gaps (current cycle)

- [x] Independently reconcile the newly compiled two-case selector RED against normal PTR library `161dad02a`; retain later inputs as unreached, not tested. This closes baseline attribution only.
- [ ] Compile passive plumbing and execute the eight populated missing-getter cases before adding query projection.
- [ ] Implement this complete supplied-record query/feed slice with behavioral coverage of snapshots, selectors/security and cached lifecycle on Retail/PTR; retain exact source/build/runtime attribution separately.

## Out of scope

Native catalog/default records, native population/transport, demo seeds, DB2 inference and native/rendered parity: neither the declarations nor the supplied-state model establish them. No vendor edits, other record types, additional profiles, new secret/selector policy, animation/timer/model-effect execution or speculative behavior. Main owns source integration, builds and runtime evidence; this draft changes none of them.
