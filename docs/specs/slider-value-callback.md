# Slider value callbacks

`Slider:SetValue(value, treatAsMouseEvent)` synchronously delivers `OnValueChanged` after changing the stored value. Source: `src/lua_api/frame/methods/widgets/slider.rs`. See [event dispatch](../event-system.md) for script infrastructure.

## What it must do

- [ ] Deliver `(self, effectiveValue, treatAsMouseEvent)` before returning, with the updated value readable inside the callback; omitted mouse flag is `false`, explicit `true` is forwarded.
- [ ] Clamp and quantize Slider assignments as described below; suppress calls whose effective value is unchanged, including same-value reentry from a callback.
- [x] Dispatch pre/normal/post bindings in order, including intrinsic bindings when no normal script exists.
- [x] Report each handler error through the existing error handler and continue later bindings.
- [x] Permit callbacks to update other widgets and CVars without a held simulator-state borrow; hidden sliders still dispatch.

The cached Forever `SimpleSliderAPIDocumentation.lua` documents `SetValue(value, treatAsMouseEvent=false)`. Dispatch/order/error behavior follows the simulator's existing script model and unchanged addon usage, not a new native-client probe claim.

## Clean-room step assignment follow-up

C1 defines a shared clean-room simulator policy, not a recovered native algorithm or verified all-profile completion. Current positive default step remains unchanged. For finite ordered bounds and positive finite step, choose the nearest minimum-relative lattice value or the maximum endpoint; ties choose the greater numeric value. Step zero is continuous. Thus `[0,10]`, step2, value3 becomes4; step4, value9 becomes10 rather than8.

- [ ] Quantize only Slider programmatic value assignment; preserve StatusBar clamping and callback shape.
- [ ] Reapply the current stored Slider value through the same assignment when `SetValueStep` is called. Dispatch synchronously only if the effective value changes, with a false mouse flag after releasing the simulator-state borrow.
- [ ] Preserve offset/negative minima, exact endpoints, same-effective-value suppression and existing dispatch/reentry/error behavior. Keep existing clamp semantics for unsupported nonfinite values/bounds, invalid steps and unrepresentable spans without attempting lattice arithmetic. This does not establish their native argument policy.

Producer authored in `src/lua_api/frame/methods/widgets/slider.rs`: pure quantization used by `apply_slider_value`, plus step-triggered reapplication using the existing dispatcher. No default initialization, C8 range-callback/lifecycle, mouse-drag producer or StatusBar changes.

Actual Retail baseline: source `3fa90ab7b6f8cf836e09c19ed34115eba9f245c1`, epoch `cleanroom-controls-retail-baseline-20261015T223734529083Z/slider-step-first-red`, executed the five existing `tests/slider_step_assignment.rs` cases: **0 passed, 5 unique failures, exit 101**. Full stdout/stderr and `result.json` remain in the private verification archive under that epoch; no published receipt is claimed. Four failures report incorrect stored effective values; the duplicate-suppression case reports unwanted dispatch. These receipts establish reached RED against the clamp-only/stores-only producer, not GREEN for this edit.

**GREEN pending main integration and verification.** No tests, builds or checks run for this authored producer. Existing five cases cover nearest lattice/max and upward ties, offset/negative minima, endpoints/duplicate suppression, step reapplication, and continuous Slider versus unsnapped StatusBar. No duplicate tests added. The fractional scale/CVar consumer explicitly chooses step0.01 to express its existing precision.

## C8 changed-range callback/lifecycle candidate

Explicit shared simulator candidate only, not a native-client claim. This tests/spec-only integration adds no C8 production behavior and establishes no RED/GREEN proof. Existing validation remains owned by [widget range validation](widget-range-validation.md); C8 does not change it or StatusBar behavior.

- [ ] Make Slider `OnMinMaxChanged` publicly discoverable through `HasScript`, bindable/readable through `SetScript`/`GetScript`, and clearable without removing type capability or dispatching on binding alone.
- [ ] After successful validation of changed bounds, commit both bounds, synchronously notify `OnMinMaxChanged(self,min,max)` while the old stored value remains readable, then reapply that value through ordinary Slider assignment. Hidden sliders follow the same lifecycle; equal bounds are included.
- [ ] Reapply the value even without a range binding. Deliver `OnValueChanged(self,effectiveValue,false)` synchronously only when the effective value changes, with committed bounds/value readable in that callback.
- [ ] Notify range once when changed bounds retain the effective value, without a value notification; identical bounds are silent.
- [ ] Reuse the pending C1 minimum-relative step/max-endpoint policy for range-triggered assignment, not a second quantizer. The step-interaction test depends on C1 and cannot independently establish C8.

Exactly five existing candidate bodies are integrated in `tests/slider_range_lifecycle.rs`: **5 UNCOMPILED / UNEXECUTED**. Public Lua traces assert new-bounds/old-value range observation followed by effective-value observation; these are candidate policy assertions, not independent native evidence. No admission, behavioral RED/GREEN, or production completion is claimed. Existing callback and C1 proof statements above remain unchanged.

Defaults, range initialization, pre-initialization and first-value policy remain untouched and **UNRESOLVED**, outside C8. StatusBar, duplicate validation/quantizer tests, and speculative callback mutations of range/value are excluded. Existing dispatch/error/reentry guarantees are not weakened; this bounded set adds no separate proof for them.

## How it works

- [Event system](../event-system.md)
- [Widget system](../widget-system.md)

## Implementation inventory

- `src/lua_api/frame/methods/widgets/slider.rs`: changed-value application and synchronous slider script delivery.

## Tests asserting this spec

`tests/widget_slider.rs` in the grouped integration target:

- `set_value_dispatches_synchronously_with_mouse_payload`
- `set_value_clamps_suppresses_unchanged_and_allows_reentry`
- `set_value_dispatches_intrinsic_bindings_after_reported_errors`
- `set_value_updates_castbar_style_scale_and_cvar_consumer`

Frozen `d1e2487f` executes identical test bodies and XML intrinsic templates: four expected missing-dispatch failures. Committed producer `3d6017fe3` passes all nine grouped slider tests, including the four new callbacks and five existing slider regressions. Independent verification reuses that 9/9 proof, passes `cargo fmt --check` and default `cargo check --offline`, and runs a direct StatusBar regression: `/tmp/forever-addon-audit/verify-slider-value-ledger.json`.

Frozen `3d6017fe` runs unchanged ClassicCastBar `8909724` through scale `1 → 1.35 → 1`, icon toggles, reset and `DONE` with no Lua errors; the no-addons control returns `[]`: `/tmp/forever-addon-runtime/classic-slider-workflow-q675457h/ledger.json` and `/tmp/forever-addon-runtime/slider-control-0uwemvbq/ledger.json`.

## Known gaps (current cycle)

- [ ] Mouse dragging, settings navigation, casting behavior, rendered pixels and restart persistence remain unverified.

## Out of scope

StatusBar behavior, other slider methods outside the step follow-up and bounded C8 candidate, mouse-drag producers, new secret handling, argument-validation fidelity and native timing conformance remain unchanged or unverified. No addon/vendor changes or manual callback invocation.
