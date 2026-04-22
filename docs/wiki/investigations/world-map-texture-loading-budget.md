# World Map Texture Loading Budget

Opening the Blizzard world map still had large stalls after the `SetFrameLevel()` invalidation fix. The steady-state bucket rebuild loop was gone, but the map open path still spent tens of milliseconds per frame in synchronous texture work. Two later follow-ups mattered here: preload could clear `textures_pending` once the sources were CPU-cached even though the GPU atlas still had a large backlog, and Blizzard's own `MapTexturePreloader.lua` path was inert because `C_Map.RequestPreloadMap()` was stubbed as a no-op.

## Symptom

Representative repro with `WOW_SIM_VERBOSE=1`, `--no-addons`, and `ToggleWorldMap()`:

```text
[draw] quads=7.1ms textures=50.8ms (new=30 rgba=1 bc=29)
[tick] 91.9ms (layout=166.6µs dirty=0x8 ids=Some(21) pending=true)
```

The misleading part was the old `(0 new)` / low-count logging: the expensive work was mostly BC-compressed world-map tiles, not RGBA uploads.

## Root Cause

The original stall was three issues stacked:

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

The remaining explored-overlay gap had a separate root cause:

4. `Interface/BlizzardUI/Blizzard_WorldMap/MapTexturePreloader.lua` called `C_Map.RequestPreloadMap(mapID)`, but the simulator's `C_Map.RequestPreloadMap()` implementation returned immediately without queueing any world-map assets.
   - Blizzard's `MapExplorationDataProvider` hides explored overlays during a full refresh until the detail layers and its texture load group are ready.
   - Without the API-side preload hook, map detail tiles got first access to the preload/upload budget and explored overlays commonly showed one phase later than the base map.

## Fix

- `TextureManager` now caches BC-compressed sources (`load_bc()` no longer reparses the same BLP every time).
- Budgeted preload uses the BC path when BC compression is supported, so preload and draw share the same cached source.
- Draw-thread texture loading budget was reduced from ~50ms to ~10ms.
- Tick-time preload budget was reduced from ~75ms to ~25ms.
- Budgeted preload now keeps `textures_pending=true` until every requested path is either present in `gpu_uploaded_textures` or known-failed, so the fast 16ms tick keeps driving draw uploads until the atlas catches up.
- Perf logging now reports both RGBA and BC upload counts.
- `C_Map.RequestPreloadMap()` now resolves the target map's art tiles plus exploration overlay textures into concrete WoW texture paths and queues them into the existing texture preload pass.
- Queued API-side preloads share the same budgeted preload loop as normal render requests, and unfinished paths are re-queued when the budget is exhausted instead of being dropped.

## 2026-04-14 Follow-up: First-Frame Quarter-Map Artifact

After the preload/draw budget work landed, the first visible world-map frame could still open in a partially streamed state where only one map tile quadrant had uploaded. The screenshot looked like the fog of war was covering only one quarter of the map, but the geometry was a red herring: the fog and exploration pins still matched the full canvas size.

The actual bug was that budgeted draw treated BC-preloaded world-map tiles as "not cached":

- `TextureManager::is_cached()` only checked the RGBA `cache`
- world-map preload now warms many map tiles into `bc_cache`
- draw-path budget checks in `render_textures.rs` call `is_cached()` to decide whether they can stop once the deadline is hit
- after the first BC upload, draw could hit its deadline, see the next tile as "uncached", and pause early even though preload had already warmed the source

That mismatch made the first interactive frame vulnerable to partial tile upload progress, which visually showed up as an incorrect quarter-map explored/fog composition.

The fix was small but critical: `TextureManager::is_cached()` now returns true when the normalized path exists in either `cache` or `bc_cache`. That keeps the upload loop streaming BC-preloaded map tiles across the deadline boundary instead of stalling after the first quadrant.

## 2026-04-22 Follow-up: Retained Live-GUI Display Path

The earlier preload and GPU-pending fixes were still not the full story for the live GUI. A one-shot screenshot render of `ToggleWorldMap()` could show the textured map on first render, while the retained interactive path still showed seemingly random missing map tiles or explored regions until later interaction. That ruled out the failed-texture cache and pointed back at retained render state.

The later investigation found four stacked issues in the live path:

1. Tick warmup was loading the wrong request set first:
   - it started from raw widget texture fields,
   - but the retained renderer actually draws cached `QuadBatch` request paths, including derived `@crop:` requests,
   - so warmup could finish “successfully” while still skipping the exact request the current frame needed.
2. Full redraw invalidation could be downgraded later in the same tick:
   - warmup called `mark_all_strata_dirty()`, which uses `pending_dirty_ids = None` as the full-rebuild sentinel,
   - `collect_tick_dirty()` later overwrote that sentinel with unrelated per-frame dirty IDs,
   - the next rebuild could then miss clean world-map strata that only needed texture re-resolution.
3. Request priority was inverted:
   - the sort key treated `is_world_map=true` as a larger value,
   - so ascending sort pushed world-map requests behind unrelated UI textures,
   - budgeted warmup then spent time on the wrong assets first.
4. `textures_pending` still had split ownership:
   - tick warmup and draw use it to mean “the current render request is not display-ready yet,”
   - queued preload in `render.rs` still treated it as “the explicit preload queue has work left,”
   - when the queue drained, that path could clear `textures_pending` before draw had uploaded and re-resolved cached world-map batches.
5. The retained path also used the wrong readiness set:
   - `render_textures.rs` inserted paths into `gpu_uploaded_textures` inside `load_pending_texture()`,
   - that happens before `prepare()` uploads the bytes into `GpuTextureAtlas`,
   - so `gpu_uploaded_textures` was really a draw-staging set, not a “visible in the atlas” set,
   - warmup and queued-preload checks that treated it as display-ready could clear `textures_pending` a frame too early.

That fourth issue matches the “click fixes it” symptom directly. `render()` only calls `recover_pending_textures()` when `had_textures_pending` is true before draw starts. If queued preload clears the flag too early, clean cached strata are never re-injected into `prepare()`, so already-decoded map textures remain invisible until some unrelated interaction dirties the UI again.

The fix on 2026-04-22 was to split those meanings:

- keep `gpu_uploaded_textures` as the draw-staging dedupe set,
- add a separate atlas-ready tracker populated from `WowUiPrimitive::prepare()`,
- switch warmup and cached-request pending checks to the atlas-ready set instead of the staging set.

## Result

After the cache + budget changes, the world map no longer took repeated ~50ms draw stalls while tiles streamed in. The same repro shifted to progressive smaller chunks:

```text
[draw] quads=3.9ms textures=11.1ms (new=16 rgba=1 bc=15)
[draw] quads=4.3ms textures=11.6ms (new=16 rgba=0 bc=16)
[tick] 31.5ms ... pending=true
```

The remaining work is still synchronous and visible, but it is spread across more frames instead of landing as a few 50-90ms spikes. The pending-bit follow-up removes the idle gaps between those chunks: once preload has warmed the sources, draw keeps receiving 16ms ticks until the GPU atlas backlog is empty instead of waiting ~500ms for unrelated UI activity.

After wiring `RequestPreloadMap()`, the base map and explored overlays no longer depend entirely on the first visible draw pass. World-map preloader requests now warm both classes of textures ahead of time, which removes the artificial "base map first, explored overlay later" delay caused by the no-op API stub.

## Verification

- `cargo test texture::tests::test_is_cached_reports_bc_preloaded_textures --lib`
- `cargo test texture::tests::test_load_bc_caches_dxt_blp_data --lib`
- `cargo test budgeted_preload --lib`
- `cargo test --lib request_preload_map_warms_map_art_and_overlay_textures`
- `cargo test --test render_order isolated_world_map_stack_opens_and_populates_world_quest_pins`
- `cargo test --test test_keybindings_panels_detail world_map_fog_of_war_pin_matches_canvas_size_on_first_open`
- Runtime repro with `ToggleWorldMap()` confirmed the shift from ~50ms draw spikes to ~11ms draw chunks.

## Sources

- [src/iced_app/render.rs](../../../src/iced_app/render.rs) — preload budget loop, `textures_pending` bookkeeping, and focused regression tests
- [src/iced_app/update.rs](../../../src/iced_app/update.rs) — tick-start warmup, cached render request selection, and dirty sentinel handling
- [src/iced_app/render_textures.rs](../../../src/iced_app/render_textures.rs) — unresolved-request ordering and GPU-upload bookkeeping
- [src/texture.rs](../../../src/texture.rs) — `bc_cache`, `TextureManager::is_cached()`, and BC-cache regression coverage
- [src/lua_api/globals/c_map_api.rs](../../../src/lua_api/globals/c_map_api.rs) — `C_Map.RequestPreloadMap()` queueing
- [src/lua_api/state.rs](../../../src/lua_api/state.rs) — API-side queued texture preload storage
- [src/texture/preload.rs](../../../src/texture/preload.rs) — map-art / exploration-overlay path collection
- [Interface/BlizzardUI/Blizzard_WorldMap/MapTexturePreloader.lua](../../../Interface/BlizzardUI/Blizzard_WorldMap/MapTexturePreloader.lua) — Blizzard-side preload request entrypoint
- [Interface/BlizzardUI/Blizzard_SharedMapDataProviders/MapExplorationDataProvider.lua](../../../Interface/BlizzardUI/Blizzard_SharedMapDataProviders/MapExplorationDataProvider.lua) — explored-overlay visibility waits on detail/background load
- [tests/render_order.rs](../../../tests/render_order.rs) — isolated world-map integration coverage used as regression proof

## See Also

- [[world-map-frame-level-rebuilds]] — earlier fix for the periodic bucket rebuild loop
- [[world-map-create-texture-sublevel]] — follow-up fix for world-map textures that were immediately repaired with `SetDrawLayer()`
- [[rendering-pipeline]] — retained strata batches, prepare-time texture upload, and why clean strata need re-injection for re-resolution
- [[world-map-fog-of-war-overlay-model]] — later fix for the separate
  exploration-data versus fog-geometry mismatch
- [[character-select-performance]] — earlier preload/draw-path texture stall investigation
