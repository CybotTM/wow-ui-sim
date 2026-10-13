# Supplied Path points and NONE playback

Path/ControlPoint configuration, XML loading and actual computed layout must agree. Four tests are staged; producers unchanged, tests uncompiled/unexecuted. This is current API behavior, not a historical 3.3.0 publication row. Architecture: [layout](../layout-system.md), [loading](../addon-loading-pipeline.md).

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
| `src/lua_api/animation.rs` | Existing Path type/timing; point/curve and offset restoration state absent. |
| `src/lua_api/frame/methods/button_anchor_hierarchy/animations/creation.rs` | Generic ControlPoint factory, no modeled point state. |
| `src/xml/types_animation.rs` | Parses curve/offsets; schema-supported point name is not represented. |
| `src/loader/helpers_anim.rs` | Path fields/points not emitted into runtime. |
| `src/layout.rs` | Existing animation offsets project into computed rectangles. |
| `tests/path_control_point.rs` | Four prospective real API/XML/tick/layout tests, `retail-12-0-0` gate. |

## Tests asserting this spec

Generated `--test integration` filter `path_control_point::` (expected four): order9/2→1/2 and signed offset mutations; actual named two-point XML loading and midpoint/restoration; three-point sorted NONE progression with child/owner/sibling isolation; Stop and finish restoration across changed baselines. All UNCOMPILED/UNRUN. Missing requested methods are prospective behavioral boundaries, not executed RED. Fixture failure is not behavior proof.

## Known gaps (current cycle)

- [ ] Compile and execute genuine behavioral failures before point/XML/tick producers.
- [ ] Model point/curve state and apply complete XML configuration.
- [ ] Implement NONE movement/restoration and execute real layout-output GREEN.

## Out of scope

SMOOTH, native/historical parity, zero/one point, duplicate/default/invalid orders, SetParent, templates, broader receiver repair, delays/loops/reverse/pause, callbacks, large-delta semantics, non-unit scale, competing animations and GPU/pixels. No vendor patches or production changes in this stage.
