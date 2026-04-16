# Layout Profile

Release `perf` profile of `wow-sim lua-errors` (with full Blizzard UI, `--no-saved-vars`) to break down the ~5–8% of wall time spent in frame layout computation. Found one quick win (`LayoutCache` siphash) and confirmed the rest of the cost is real arithmetic.

## Method

```
cargo build --release --bin wow-sim
LD_LIBRARY_PATH=target/release \
  perf record -g --freq 999 -m 512K -o /tmp/perf.data \
  -- ./target/release/wow-sim lua-errors
perf script -i /tmp/perf.data | inferno-collapse-perf > /tmp/collapsed.txt
```

Categorise by grep on the collapsed stacks.

## Pre-fix breakdown

Total samples: 6.30B. Layout-attributed: 470M = **7.5% of wall time**.

| Sub-path | Samples | Notes |
|---|---:|---|
| `compute_frame_rect_cached` (self) | 69M | Top frame of layout |
| `resolve_uncached_frame_layout` → `resolve_frame_layout_rect` | 62M | Real arithmetic for single-anchor frames |
| `LayoutCache::get` via `hashbrown::find_inner` (std `RandomState` / siphash) | ~64M | **Cache hit path, siphash dominated** |
| `WidgetRegistry::get` (already FxHash) | 27M | Fast-path frame lookup |
| `resolve_single_anchor` + `anchor_position` | 45M | Coordinate math |
| `resolve_parent_rect` recursion | 18M | Walks up the parent chain |
| `recompute_layout_subtree` | 17M | Invalidation-triggered recompute |

`RandomState` siphash across all stacks (not just layout) summed to **295M samples**.

## Fix

`pub type LayoutCache = HashMap<u64, CachedFrameLayout>` → `FxHashMap<u64, CachedFrameLayout>`. `u64` frame IDs don't need a DOS-resistant hash.

One-line change (`src/iced_app/layout.rs`); commit `5dc4d1c6` ("Switch LayoutCache to FxHashMap").

## Post-fix breakdown

Total samples: 5.95B. Layout-attributed: 300M = **5.0% of wall time**.

- Layout samples: 470M → 300M (−170M, **−36%**)
- Total siphash samples across all stacks: 295M → 76M (−219M, **−74%**)
- Release startup (`lua-errors`, n=10): median 1.21s → 1.18s wall time

The siphash saving is larger than the layout saving because `LayoutCache` was hit from many places (anchor resolution, parent resolution, dependent recomputation) — each call that was previously using siphash now uses fxhash.

## Remaining layout cost

- `resolve_uncached_frame_layout` and `resolve_single_anchor` / `anchor_position` are real arithmetic on `f32`. No obvious win without restructuring the resolution algorithm.
- `resolve_parent_rect` recursion is bounded by parent-chain depth (typically ≤ 10). Already memoised via `LayoutCache`.
- `recompute_layout_subtree` fires on invalidation (anchors/size/scale/parent changed). Batching invalidations or skipping unchanged subtrees could reduce this further.

## Sources

- `src/iced_app/layout.rs` — `LayoutCache`, `compute_frame_rect_cached`, `resolve_*`
- `src/lua_api/state_render.rs` — `invalidate_layout`, `recompute_layout_subtree` callers
- Commit `5dc4d1c6` — the FxHashMap fix

## See Also

- [[table-rehashing]] — rilua table rehash profiling, same methodology
- [[on-update-dirty]] — OnUpdate dirty propagation; related to when layout recompute fires
