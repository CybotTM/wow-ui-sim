# World Map Texture Loading Budget

Opening the Blizzard world map still had large stalls after the `SetFrameLevel()` invalidation fix. The steady-state bucket rebuild loop was gone, but the map open path still spent tens of milliseconds per frame in synchronous texture work. A follow-up bug then stretched the remaining upload work across ~500ms gaps: preload could clear `textures_pending` once the sources were CPU-cached, even though the GPU atlas still had a large backlog.

## Symptom

Representative repro with `WOW_SIM_VERBOSE=1`, `--no-addons`, and `ToggleWorldMap()`:

```text
[draw] quads=7.1ms textures=50.8ms (new=30 rgba=1 bc=29)
[tick] 91.9ms (layout=166.6µs dirty=0x8 ids=Some(21) pending=true)
```

The misleading part was the old `(0 new)` / low-count logging: the expensive work was mostly BC-compressed world-map tiles, not RGBA uploads.

## Root Cause

Three issues stacked:

1. The render-thread texture log only counted RGBA uploads, so BC tile uploads looked like “no-op” draw stalls.
2. Preloading and draw were not sharing the same source cache:
   - budgeted preload warmed decoded RGBA data,
   - draw still reparsed BLPs through the BC path,
   - so the world map paid large synchronous chunks in draw even after preload had already touched the same assets.
3. `textures_pending` tracked preload iteration, not GPU upload state:
   - draw hit its ~10ms upload budget, pushed a handful of BC tiles into the atlas, and left `textures_pending=true`,
   - the next timer tick ran preload against the now-hot CPU cache, finished quickly, and set `textures_pending=false`,
   - once `strata_dirty` was also clear, the app dropped out of the 16ms tick cadence even though most world-map tiles still were not in `gpu_uploaded_textures`,
   - the remaining tiles only advanced when some unrelated event dirtied the UI again.

## Fix

- `TextureManager` now caches BC-compressed sources (`load_bc()` no longer reparses the same BLP every time).
- Budgeted preload uses the BC path when BC compression is supported, so preload and draw share the same cached source.
- Draw-thread texture loading budget was reduced from ~50ms to ~10ms.
- Tick-time preload budget was reduced from ~75ms to ~25ms.
- Budgeted preload now keeps `textures_pending=true` until every requested path is either present in `gpu_uploaded_textures` or known-failed, so the fast 16ms tick keeps driving draw uploads until the atlas catches up.
- Perf logging now reports both RGBA and BC upload counts.

## Result

After the cache + budget changes, the world map no longer took repeated ~50ms draw stalls while tiles streamed in. The same repro shifted to progressive smaller chunks:

```text
[draw] quads=3.9ms textures=11.1ms (new=16 rgba=1 bc=15)
[draw] quads=4.3ms textures=11.6ms (new=16 rgba=0 bc=16)
[tick] 31.5ms ... pending=true
```

The remaining work is still synchronous and visible, but it is spread across more frames instead of landing as a few 50-90ms spikes. The pending-bit follow-up removes the idle gaps between those chunks: once preload has warmed the sources, draw keeps receiving 16ms ticks until the GPU atlas backlog is empty instead of waiting ~500ms for unrelated UI activity.

## Verification

- `cargo test texture::tests::test_load_bc_caches_dxt_blp_data --lib`
- `cargo test budgeted_preload --lib`
- `cargo test --test render_order isolated_world_map_stack_opens_and_populates_world_quest_pins`
- Runtime repro with `ToggleWorldMap()` confirmed the shift from ~50ms draw spikes to ~11ms draw chunks.

## Sources

- [src/iced_app/render.rs](../../../src/iced_app/render.rs) — preload budget loop, `textures_pending` bookkeeping, and focused regression tests
- [src/iced_app/render_textures.rs](../../../src/iced_app/render_textures.rs) — draw-path GPU upload budget and `gpu_uploaded_textures` semantics
- [tests/render_order.rs](../../../tests/render_order.rs) — isolated world-map integration coverage used as regression proof

## See Also

- [[world-map-frame-level-rebuilds]] — earlier fix for the periodic bucket rebuild loop
- [[world-map-create-texture-sublevel]] — follow-up fix for world-map textures that were immediately repaired with `SetDrawLayer()`
- [[character-select-performance]] — earlier preload/draw-path texture stall investigation
