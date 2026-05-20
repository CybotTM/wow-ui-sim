# Character Select Performance Notes

This note captures the investigation and fixes for the character-select startup hitch.

## Summary

There were two separate problems on the character-select screen:

1. Texture stalls from lazy `@crop:` atlas sub-region generation on the draw path
2. A one-time layout/rebuild spike after the real window size arrives from the window manager

The texture issue is fixed and committed. The layout issue is improved locally but not yet committed in this note's current branch state.

## Reproduction

Baseline repro command:

```bash
LD_LIBRARY_PATH=target/debug:target/debug/deps \
timeout 20 target/debug/wow-sim --screen character-select --no-saved-vars --no-addons
```

Useful signals in logs:

- `[draw] quads=... textures=...`
- `[rebuild] layout=... buckets=...`
- `Window size: ...`

## Problem 1: Texture Stalls

Original symptom:

```text
[draw] quads=2.1ms textures=939.0ms (1 new)
[draw] quads=1.7ms textures=105.7ms (7 new)
```

Root cause:

- visible atlas textures were converted into unique `@crop:` requests at render time
- those requests were materialized lazily inside the draw path
- the preload path only warmed base texture paths, not the cropped render-time requests

Relevant files:

- `src/iced_app/quad_builders.rs`
- `src/iced_app/render.rs`
- `src/texture.rs`

Fix:

- preload actual initial render texture requests
- route `@crop:` requests through `TextureManager` sub-region caching

Committed in:

- `8dfe2ebf` `Preload character select render textures`

Result:

```text
[preload] warmed 24 render requests (8 new base textures, 24 total requests)
[draw] quads=53.7ms textures=1.2ms
```

## Problem 2: First Real Resize Rebuild

After the texture fix, the remaining bad frame was tied to the first real window-size settle.
The original capture below was from the older path where the GUI inherited the
`SimState` default size (`1600x1200`) before Iced reported the actual canvas
size:

```text
Window size: 853x872 (was 1600x1200)
[rebuild] layout=46.6ms buckets=12.2ms
[draw] quads=62.4ms textures=1.2ms
```

### What was happening

`set_screen_size()` invalidated all cached layout rects. On the next draw,
`ensure_layout_rects()` rebuilt layout for the whole pending tree after the
window changed from the startup size to the actual canvas size.

Current GUI startup waits until the first Iced draw reports the real canvas
bounds before dispatching WoW startup events, initializing render state, and
resolving first-frame layout. `init_and_load()` still seeds the environment with
the logical window fallback (`1024x768`) before addon file loading because Iced
does not expose canvas bounds until the application is running; GUI startup
events no longer treat that fallback as authoritative.

Relevant files:

- `src/lua_api/env.rs`
- `src/lua_api/state.rs`
- `src/widget/registry.rs`
- `src/iced_app/render.rs`

### Root causes found

#### 1. Full-tree alpha/scale propagation in bucket building

`build_strata_buckets()` was calling:

- `propagate_all_effective_alpha()`
- `propagate_all_effective_scale()`

on every rebuild, even though those are startup-style full-tree passes.

Fix:

- move that propagation to startup initialization

Committed in:

- `f3f40ddd` `Initialize render state at startup`

Effect:

- bucket time dropped modestly, roughly from `10-12ms` to `7-9ms`

#### 2. Duplicate subtree layout work in `ensure_layout_rects()`

`ensure_layout_rects()` has two phases:

1. resolve frames in `pending_layout_ids`
2. resolve frames in `rect_dirty_ids`

The same subtree could be recomputed twice in one call:

- once because it was pending
- again because descendants were still dirty

Instrumentation showed calls like:

```text
[layout] total=261.5ms pending=2247 pending_roots=1002 dirty=4760
```

Fix that worked:

- when the pending phase recomputes a subtree, clear `rect_dirty_ids` for that subtree

#### 3. Resize invalidation queued every frame as pending

`clear_all_layout_rects()` was setting `layout_rect = None` for every frame and inserting every frame into `pending_layout_ids`.

That created many overlapping pending roots after resize. Instrumentation showed:

```text
[layout] total=68.1ms pending=6281 pending_roots=1225 dirty=0
```

Fix that worked:

- after resize, only top-level roots are added to `pending_layout_ids`
- in `ensure_layout_rects()`, only topmost pending roots are recomputed

## Current Local Result

With the local layout dedupe changes applied, the isolated repro is:

```text
Window size: 853x872 (was 1600x1200)
[rebuild] layout=33.1ms buckets=7.3ms
[draw] quads=42.7ms textures=635.8µs (24 new)
```

Compared to the earlier post-texture-fix baseline:

```text
[rebuild] layout=46.6ms buckets=12.2ms
[draw] quads=62.4ms textures=1.2ms
```

So the remaining resize frame improved materially, but it is still a one-time expensive relayout.

## Experiments That Did Not Help

These were tested and backed out:

- resolving layout only for currently visible render buckets
- moving resize/layout work to `window::resize_events()` in Iced
- deferring hidden pending roots

These either regressed timings, caused extra size churn, or just moved work between `pending` and `dirty` without reducing the total.

## Remaining Work

After the dedupe fixes, the remaining `~33ms` layout cost appears to be mostly real resize work, not duplicate work.

Likely next directions:

- add finer timing inside `recompute_layout_subtree()` / `compute_frame_rect_cached()` to find the heaviest roots
- identify whether a specific glue subtree dominates the first-resize relayout
- avoid the initial `1600x1200` placeholder size so the first real resize does not trigger a full-screen relayout

## Current Code State

Committed:

- `8dfe2ebf` `Preload character select render textures`
- `f3f40ddd` `Initialize render state at startup`

Local, not yet committed at the time of writing:

- pending-root dedupe changes in `src/widget/registry.rs`
- pending/dirty overlap fix in `src/lua_api/state.rs`
