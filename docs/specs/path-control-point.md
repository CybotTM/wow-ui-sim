# Supplied Path points and NONE playback

Path/ControlPoint configuration, XML loading and actual computed layout must agree. Four pre-producer cases executed RED; point/XML/NONE playback producers are implemented, current GREEN remains pending. This is current API behavior, not a historical 3.3.0 publication row. Architecture: [layout](../layout-system.md), [loading](../addon-loading-pipeline.md).

## What it must do

- [ ] Preserve supplied `CreateControlPoint(nil, nil, order)`, `SetOffset(x,y)` and subsequent `SetOrder`; public order and two-value offset getters report current input.
- [ ] **Simulator policy:** `GetControlPoints` returns multiple point values in ascending distinct positive order, not creation order; maximum query returns greatest order, not count.
- [ ] Normal addon TOC/XML loading preserves Path `curve="NONE"`, point names and offsets. **Simulator policy:** declaration position assigns orders 1/2; no XML order attribute invented.
- [ ] **Simulator policy:** curve token NONE is a round-tripping string. With smoothing/looping NONE and duration2, playback immediately starts at first point, interpolates each adjacent ordered pair over equal time regardless of distance/order gaps, and has no implicit origin point.
- [ ] **Simulator policy:** displacement adds to saved pre-play presentation offset at unit scale using existing screen-layout axes. Child-key target alone moves; owner/sibling positions and target dimensions remain unchanged. XML case targets group owner.
- [ ] **Simulator policy:** Stop/natural finish restore pre-play offsets; replay snapshots a changed baseline. Duration-expiry post-tick state is restored, not retained terminal point. No callback ordering specified.

Cached `SimpleAnimPathAPIDocumentation.lua` declares optional order and stride return; `SimpleControlPointAPIDocumentation.lua` declares order/offset methods. Cached `Blizzard_SharedXML/UI.xsd` explicitly permits optional ControlPoint `name` and offsetX/Y, and Path curve NONE/SMOOTH. Native interpolation, ordering and restoration semantics are not captured here.

## How it works

- [Layout system](../layout-system.md)
- [Addon loading](../addon-loading-pipeline.md)
- [Events and update ticks](../event-system.md)

## Implementation inventory

| Path | Current role / pending gap |
|---|---|
| `src/lua_api/animation.rs` | Path curve/ordered point state and saved per-target offsets. |
| `src/lua_api/frame/methods/button_anchor_hierarchy/animations/{creation,path,path_runtime}.rs` | Path-only/control-point methods, named point creation, NONE interpolation and restoration. |
| `src/xml/types_animation.rs` | Parses curve, point names and offsets. |
| `src/loader/helpers_anim.rs` | Emits Path curve and declaration-ordered point configuration through the same API. |
| `src/layout.rs` | Existing animation offsets project into computed rectangles. |
| `tests/path_control_point.rs` | Four real API/XML/tick/layout tests, `retail-12-0-0` gate. |

## Tests asserting this spec

Generated `--test integration` filter `path_control_point::` (expected four): order9/2→1/2 and signed offset mutations; actual named two-point XML loading and midpoint/restoration; three-point sorted NONE progression with child/owner/sibling isolation; Stop and finish restoration across changed baselines. At `ff72f3c3b`, all four fail on missing SetCurveType/GetCurveType or supplied-order behavior, with no observed fixture failures; named XML loading succeeds before its missing-query assertion. Movement, XML point projection and restoration assertions remain unreached where earlier API assertions fail. Current producer GREEN is pending. Fixture failure is not behavior proof.

## Known gaps (current cycle)

- [x] Compile and execute genuine behavioral failures before point/XML/tick producers.
- [x] Model point/curve state and apply XML configuration.
- [x] Implement NONE movement/restoration after genuine API-boundary RED.
- [ ] Execute real API/XML/layout-output GREEN plus applicable startup/profile checks.
- [ ] Existing hidden-subtree stop restoration hook is source-only; no separate behavioral acceptance claimed.

## Out of scope

SMOOTH, native/historical parity, zero/one point, duplicate/default/invalid orders, SetParent, templates, broader receiver repair, delays/loops/reverse/pause, callbacks, large-delta semantics, non-unit scale, competing animations and GPU/pixels. No vendor patches. Only NONE curve playback is modeled; unsupported curves are not silently substituted.
