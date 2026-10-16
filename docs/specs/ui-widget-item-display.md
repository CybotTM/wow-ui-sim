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

- `tests/ui_widget_item_display_selector.rs` — two public-selector regressions at test source `f67005a12a07661592c6a4f5615015102db5b9f3`; fresh `WowLuaEnv` instances, no mock registration, supplied records or cached UI load.

### Historical Retail selector RED — 2026-10-16

Saved runtime epoch: `/home/osso/.local/state/wow-ui-sim/verification/wrath-source-current/item-display-selector-retail-red-20261016T135057967011Z`. Component compile exits0 with no Rust diagnostics; tests link separately sealed normal Retail library `7273875b2d35ae305428661b2650116e802733e5`, not a whole-f670 build.

Exactly two tests run: **0 PASS / 2 FAIL / exit101**. Missing-input and explicit-nil cases each pass the shared `GetWidgetSetInfo` required-number control, public getter function check and absent numeric `934207.5` exactly-one-nil assertions. Each fails `ItemDisplay invalid widgetID must reject`: the invalid target call succeeds. Target diagnostics and later string, numeric-string, boolean, table and function iterations are **UNREACHED**.

Independent full-stream/source/feature/seal/CVar/readability RED reconciliation: `/home/osso/.local/state/wow-ui-sim/verification/item-widget-next/selector-red-report.md`. Child-only current-CVar copy and paired original stat/hash are unchanged at their observed boundaries. No reset/restore or historical preservation claim. This confirms bounded RED, not selector acceptance; PTR, secret wrappers, populated projection, cached consumers, native parity and parent acceptance remain unproved.

## Known gaps (current cycle)

- [ ] Establish selector acceptance under Retail/PTR, retaining reached/unreached boundaries; the historical Retail run remains RED and does not establish PTR behavior.
- [ ] Supply the modeled ItemDisplay producer; it remains missing from the inspected source inventory above.
- [ ] Add a separately scoped complete projection and populated fixture contract. Cached Retail/PTR declarations have 15 outer fields and 16 nested `UIWidgetItemInfo` fields, including four nullable fields; this selector test does not assert that projection.

## Out of scope

Complete projection, populated records, cached consumer/lifecycle acceptance and native parity are deferred beyond this selector slice. No nested item fields are asserted or invented. Item loading, native catalog/assets/rendering, other profiles, new selector restrictions and new secret policy are outside this slice.
