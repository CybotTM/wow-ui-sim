# Character Select Performance

Two startup hitches on the character-select screen: texture stalls from lazy atlas crop generation, and a layout spike on first real window resize.

## Problem 1: Texture Stalls (FIXED)

### Symptom

```
[draw] quads=2.1ms textures=939.0ms (1 new)
```

### Root Cause

Visible atlas textures were converted to unique `@crop:` requests at render time. These were materialized lazily inside the draw path. The preload path only warmed base texture paths, not the cropped render-time requests.

### Fix (commit `8dfe2ebf`)

- Preload actual initial render texture requests before first draw.
- Route `@crop:` requests through `TextureManager` sub-region caching.

### Result

```
[preload] warmed 24 render requests (8 new base textures, 24 total requests)
[draw] quads=53.7ms textures=1.2ms
```

## Problem 2: First Real Resize Rebuild

### Symptom (post-texture-fix)

```
Window size: 853x872 (was 1600x1200)
[rebuild] layout=46.6ms buckets=12.2ms
```

`set_screen_size()` invalidated all cached layout rects, triggering a full-tree relayout.

### Root Causes Found

**Full-tree alpha/scale propagation in bucket building (commit `f3f40ddd`)**: `build_strata_buckets()` called `propagate_all_effective_alpha/scale()` on every rebuild. Moved to startup initialization. Bucket time: ~12ms → ~8ms.

**Duplicate subtree layout**: `ensure_layout_rects()` could recompute the same subtree twice — once from `pending_layout_ids`, again from `rect_dirty_ids`. Fix: clear `rect_dirty_ids` for subtrees already processed in the pending phase.

**Resize queued every frame as pending**: `clear_all_layout_rects()` inserted every frame into `pending_layout_ids`. Fix: after resize, only insert top-level roots; `ensure_layout_rects()` only processes topmost pending roots.

### Current Local Result (not yet committed at writing)

```
[rebuild] layout=33.1ms buckets=7.3ms
[draw] quads=42.7ms textures=635.8µs
```

The remaining ~33ms appears to be real resize work. Likely next: profile `recompute_layout_subtree()` to find the heaviest roots, and avoid the initial `1600x1200` placeholder size.

## Experiments That Did Not Help

- Resolving layout only for visible render buckets
- Moving resize work to `window::resize_events()` in Iced
- Deferring hidden pending roots

## Sources

- [character-select-performance.md](../../character-select-performance.md) — full investigation

## See Also

- [[talent-performance]] — related layout dirty tracking fixes
