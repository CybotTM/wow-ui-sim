# Supplied ItemDisplay widgets: selector contract

`C_UIWidgetManager.GetItemDisplayVisualizationInfo` is planned to expose supplied ItemDisplay state through the existing manager subsystem in `src/c_api/ui_widget_manager/`. No modeled ItemDisplay producer exists in the inspected source. This thin spec covers only the public selector boundary; see [manager state and feeds](../wiki/systems/ui-widget-manager-state.md) for the existing model.

## What it must do

- [ ] Support the public getter in Retail and PTR, grounded in each profile's actual cached `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua`: required numeric `widgetID`, nullable `ItemDisplayVisualizationInfo`, and `SecretArguments = "AllowedWhenUntainted"`. Preserve the existing manager secret policy; secret-wrapper behavior requires separate proof.
- [ ] Reject missing, explicit nil, string (including numeric string), boolean, table and function selectors at the shared required-number boundary. Diagnostics contain `must be a number` and identify `GetItemDisplayVisualizationInfo`; the existing `GetWidgetSetInfo` rejection is the control.
- [ ] Return exactly one nil for absent numeric `widgetID` `934207.5`. Do not infer catalog membership or introduce integer, positivity or range restrictions.

## How it works

- [Manager state and feeds](../wiki/systems/ui-widget-manager-state.md)

## Implementation inventory

Source inspection on 2026-10-16, not runtime evidence:

- `src/c_api/ui_widget_manager/queries.rs` — shared required-number/secret selector and modeled getter registration; no modeled ItemDisplay getter registration.
- `src/c_api/ui_widget_manager/inputs.rs` — supplied visualization records/maps; no ItemDisplay record or map.
- `src/c_api/ui_widget_manager/feed.rs` — typed supplied visualization feed; no ItemDisplay variant/storage arm.

## Tests asserting this spec

- `tests/ui_widget_item_display_selector.rs` — two uncompiled, unexecuted RED candidates mirroring the existing currencies selector pattern. Fresh `WowLuaEnv` instances call the actual public getter, first checking shared rejection, getter presence and exactly-one-nil absence, then target rejection/diagnostics. One test covers missing input; one iterates explicit nil, nonnumeric string, numeric string, boolean, table and function inputs. Earlier failure prevents credit for later assertions/iterations. No mock registration, supplied-record scaffold or cached UI load.

These are planned assertions, not observed runtime RED or PASS. Compilation and execution belong to the parent workload; no acceptance is claimed.

## Known gaps (current cycle)

- [ ] Compile and execute both selector candidates under Retail/PTR and record reached assertions and actual failure boundaries separately.
- [ ] Supply the modeled ItemDisplay producer; it remains missing from the inspected source inventory above.
- [ ] Add a separately scoped complete projection and populated fixture contract. Cached Retail/PTR declarations have 15 outer fields and 16 nested `UIWidgetItemInfo` fields, including four nullable fields; this selector test does not assert that projection.

## Out of scope

Complete projection, populated records, cached consumer/lifecycle acceptance and native parity are deferred beyond this selector slice. No nested item fields are asserted or invented. Item loading, native catalog/assets/rendering, other profiles, new selector restrictions and new secret policy are outside this slice.
