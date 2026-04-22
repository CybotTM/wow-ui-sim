# World Map Frame Level Rebuilds

Opening the Blizzard world map used to fall into a steady `[rebuild] buckets=~16-20ms` loop even when the UI had settled. The root cause was not the pulse animation itself and not the final renderer cache: map canvas pins were reapplying their frame level, and the simulator invalidated `strata_buckets` on every `SetFrameLevel()` call even when the numeric level was unchanged. The fix was to treat no-op `SetFrameLevel()` calls as no-ops for render ordering too.

## 2026-04-22 Follow-up Regression

A second steady-state rebuild loop reappeared later with world map open (`[rebuild]` every ~500ms, buckets ~16-31ms). `WOW_SIM_TRACE_STRATA_INVALIDATIONS=1` showed the new invalidation source was still `SetFrameLevel`, but now from a child `widgetContainer` under a map pin:

- `MapCanvasMixin:ApplyPinPosition()` called `pin:SetParent(canvas)` on every position refresh.
- The simulator treated same-parent `SetParent` as a partial no-op, but still ran descendant `propagate_strata_level`.
- That propagation reset child frame levels (including `widgetContainer`) back to inherited parent+offset every refresh.
- `MapCanvasPinMixin:ApplyFrameLevel()` then reapplied the intended explicit level (`frameLevel - 10`), creating a repeating level tug-of-war and forcing bucket invalidation.

Fix: make same-parent reparenting a true no-op for strata/level propagation in `reparent_widget()`.

Regression test: `test_same_parent_set_parent_preserves_child_custom_frame_level` in `tests/frame_level.rs`.

## Symptoms

- Repro: open the world map with `WOW_SIM_VERBOSE=1`.
- Steady-state logs showed a single visual dirty ID on most ticks, but periodic spikes rebuilt strata buckets anyway:

```text
[tick] ... dirty=0x8 ids=Some(1)
[rebuild] layout=... buckets=16-20ms
[tick] ... dirty=0x8 ids=Some(21)
```

- The expensive part was bucket rebuilding, not layout resolution.

## Investigation

The first hypothesis was that the permanent world-map highlight pulse was toggling child texture visibility and forcing full bucket invalidation. That produced a small improvement path in `state_render.rs`, but it did not remove the periodic rebuilds.

Adding an env-gated invalidation trace (`WOW_SIM_TRACE_STRATA_INVALIDATIONS=1`) showed the steady-state spikes were coming from repeated `SetFrameLevel()` invalidations in `methods_core_state.rs`, not from `Show()`:

- `src/lua_api/frame/methods/methods_core_state.rs` — `SetFrameLevel()` invalidated `strata_buckets` unconditionally.
- `Interface/BlizzardUI/Blizzard_MapCanvas/MapCanvas_DataProviderBase.lua` — `MapCanvasPinMixin:ApplyFrameLevel()` called `self:SetFrameLevel(frameLevel)`.
- The traced world-map repro showed repeated hits at `methods_core_state.rs` during the spike window.

That narrowed the problem to map pins reapplying an already-correct frame level.

## Root Cause

`SetFrameLevel()` was implemented as:

1. Mutate `frame.frame_level`
2. Propagate strata/level to descendants
3. Invalidate `strata_buckets`

There was no guard for the common no-op case where the requested frame level already matched the current frame level.

On the world map, `MapCanvasPinMixin:ApplyFrameLevel()` is called as part of pin placement and refresh work. In steady state, that often reapplies the same numeric level. The simulator still cleared `strata_buckets`, so the next render path paid the full bucket rebuild cost for an ordering state that had not changed.

## Fix

`SetFrameLevel()` now returns early when the requested level equals the current one. That skips:

- the visual dirty write for an unchanged field
- descendant strata/level propagation
- `strata_buckets` invalidation

Related work in the same patch:

- `state_render.rs` gained a narrow show-path subtree repair for visible same-strata descendants. That helps visibility toggles, but it was not the root cause of the world-map steady-state rebuild loop.
- `invalidate_strata_buckets()` is now `#[track_caller]` and can log caller locations behind `WOW_SIM_TRACE_STRATA_INVALIDATIONS=1`, which makes future ordering regressions easier to trace.

## Verification

- Added `set_frame_level_same_value_preserves_strata_buckets` in `methods_core_state.rs`.
- Added `state_render.rs` tests covering same-strata subtree repair for `Show()`.
- `cargo test incremental_world_map_open_keeps_circle_after_overlapping_map_tiles --test render_order` still passed.
- Runtime repro with the world map open for ~18 seconds after load showed only 4 `[rebuild]` lines total, all during world-map open. The previous steady-state periodic rebuilds disappeared.

## Sources

- [src/lua_api/frame/methods/methods_core_state.rs](../../../src/lua_api/frame/methods/methods_core_state.rs) — `SetFrameLevel()` no-op guard and test
- [src/lua_api/state_render.rs](../../../src/lua_api/state_render.rs) — `strata_buckets` invalidation tracing and show-path subtree repair
- [Interface/BlizzardUI/Blizzard_MapCanvas/MapCanvas_DataProviderBase.lua](../../../Interface/BlizzardUI/Blizzard_MapCanvas/MapCanvas_DataProviderBase.lua) — `MapCanvasPinMixin:ApplyFrameLevel()`

## See Also

- [[rendering-pipeline]] — strata buckets feed incremental quad emission
- [[transparent-wrapper-render-order]] — another world-map issue rooted in bucket construction
- [[world-map-create-texture-sublevel]] — follow-up world-map open churn from textures born at the wrong draw sublevel
