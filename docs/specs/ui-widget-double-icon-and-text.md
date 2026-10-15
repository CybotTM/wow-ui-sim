# Supplied DoubleIconAndText widgets

`C_UIWidgetManager.GetDoubleIconAndTextWidgetVisualizationInfo(widgetID)` must expose explicit supplied records and drive the unchanged cached Retail consumer. The [manager-state contract](ui-widget-manager-state.md) owns supplied state and lifecycle; the [manager architecture](../wiki/systems/ui-widget-manager-state.md) describes existing feeds. The getter is not modeled yet. The first public-selector regression is authored, **UNEXECUTED**; no behavioral RED is claimed.

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

No new production implementation. Existing `src/c_api/ui_widget_manager/queries.rs` owns shared query policy; DoubleIconAndText has no modeled projection or registration. A generic callable/nil namespace response is not implementation evidence.

## Tests asserting this spec

- `tests/ui_widget_double_icon_selector.rs`: first public missing-selector counterexample, with existing required-number behavior as its control. Authored UNEXECUTED; no passive scaffold dependency.

## Known gaps (current cycle)

- [ ] Execute the public selector regression and distinguish a reached contract failure from compilation/setup failure.
- [ ] Review supplied record/map/feed scaffolding and populated query/secret/isolation/cached-lifecycle fixtures before integration. Private authored candidates are not compiled or executed evidence.
- [ ] Implement the modeled getter against behavioral counterexamples, then verify focused behavior, profile preservation and normal startup. Full parent acceptance remains separate.

## Out of scope

Native catalogs/defaults, demo seeds, DB2 inference, vendor patches, mocked providers/Setup/pools, rendered/native parity and cross-profile acceptance. Synthetic fixture inputs must be explicit supplied values, not claimed native metadata. No asset-application or pixel proof follows from text/layout assertions.
