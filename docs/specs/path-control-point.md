# Supplied Path points and modeled curve playback

Path/ControlPoint configuration, XML loading and actual computed layout must agree. Four pre-producer cases executed RED and current point/XML/NONE playback cases executed GREEN. At `fcb0b7823`, six cases executed: four NONE/control PASS and two SMOOTH behavioral FAIL at setter/XML configuration; motion assertions remained unreached. Configuration and modeled SMOOTH interpolation are implemented; saved per-test GREEN is recorded below, while fresh startup acceptance remains pending. The retained pre-fix startup emits 12 SMOOTH Path setup errors despite exit0 and an empty Lua-error list. This is current API behavior, not a historical 3.3.0 publication row. Architecture: [layout](../layout-system.md), [loading](../addon-loading-pipeline.md).

## What it must do

- [ ] Preserve supplied `CreateControlPoint(nil, nil, order)`, `SetOffset(x,y)` and subsequent `SetOrder`; public order and two-value offset getters report current input.
- [ ] **Simulator policy:** `GetControlPoints` returns multiple point values in ascending distinct positive order, not creation order; maximum query returns greatest order, not count.
- [ ] Normal addon TOC/XML loading preserves Path `curve="NONE"`, point names and offsets. **Simulator policy:** declaration position assigns orders 1/2; no XML order attribute invented.
- [ ] **Simulator policy:** curve token NONE is a round-tripping string. With smoothing/looping NONE and duration2, playback immediately starts at first point, interpolates each adjacent ordered pair over equal time regardless of distance/order gaps, and has no implicit origin point.
- [ ] Accept and round-trip SMOOTH without aborting XML point configuration. Reject unknown curve tokens explicitly.
- [ ] **Simulator policy, not native parity:** SMOOTH uses uniform Catmull–Rom interpolation through ordered supplied points, duplicating endpoints for missing neighbors. Adjacent segments receive equal time; no implicit origin. Existing smoothing/looping NONE requirements remain.
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
| `src/lua_api/frame/methods/button_anchor_hierarchy/animations/{creation,path,path_runtime}.rs` | Path-only/control-point methods, named point creation, NONE/SMOOTH interpolation and restoration. |
| `src/xml/types_animation.rs` | Parses curve, point names and offsets. |
| `src/loader/helpers_anim.rs` | Emits Path curve and declaration-ordered point configuration through the same API. |
| `src/layout.rs` | Existing animation offsets project into computed rectangles. |
| `tests/path_control_point.rs` | Six real API/XML/tick/layout tests, `retail-12-0-0` gate; SMOOTH geometry is modeled policy. |

## Tests asserting this spec

Generated `--test integration` filter `path_control_point::` (expected six): order9/2→1/2 and signed offset mutations; actual named two-point XML loading and midpoint/restoration; three-point sorted NONE progression with child/owner/sibling isolation; Stop and finish restoration across changed baselines. At `ff72f3c3b`, all four fail on missing SetCurveType/GetCurveType or supplied-order behavior, with no observed fixture failures; named XML loading succeeds before its missing-query assertion. Movement, XML point projection and restoration assertions remain unreached where earlier API assertions fail. The four NONE cases pass at `6b522c595`; two added SMOOTH cases fail at `fcb0b7823`. SMOOTH tests cover three-point modeled geometry and unchanged cached bag-slot Path XML configuration, child motion and restoration. Saved Retail broad run `4fc115b9357e39794423a7da2145960f27273e4e` individually reports PASS for `path_smooth_uses_declared_catmull_rom_geometry_through_supplied_points` (log line 7973) and `path_actual_cached_smooth_xml_configures_points_and_restores_playback` (line 7982). The relevant Path animation/XML/tests and Cargo inputs are unchanged through `eba5a1617`; newer changes concern widget APIs/tests. This reuses historical per-test proof, not a new run, fresh startup/profile acceptance or whole-parent completion. The [broad-run owner](../wiki/investigations/fullsuite-983f02-2026-10-14.md#terminal-broad-run-4fc115b93--2026-10-14) records aggregate scope and limitations. Fixture failure is not behavior proof.

## Known gaps (current cycle)

- [x] Compile and execute genuine behavioral failures before point/XML/tick producers.
- [x] Model point/curve state and apply XML configuration.
- [x] Implement NONE movement/restoration after genuine API-boundary RED.
- [x] Recover individual SMOOTH API/XML/layout-output PASS from the saved `4fc115b93` run; no duplicate execution.
- [ ] Complete applicable fresh startup/profile checks; historical per-test proof is not full-parent acceptance.
- [ ] Existing hidden-subtree stop restoration hook is source-only; no separate behavioral acceptance claimed.

## Out of scope

Native/historical parity, zero/one point, duplicate/default/invalid orders, SetParent, templates, broader receiver repair, delays/loops/reverse/pause, callbacks, large-delta semantics, non-unit scale, competing animations and GPU/pixels. No vendor patches. NONE/SMOOTH playback is implemented with historical per-test proof above; fresh diagnostics-clean startup/profile acceptance remains pending. Unsupported curves must not be silently substituted.
