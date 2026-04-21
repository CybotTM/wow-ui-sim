# Tooltip Layout Timing

Tooltip text and background were out of sync for one frame because tooltip sizing ran after layout had already been resolved.

## Root Cause

`update_tooltip_sizes()` computes tooltip width/height from `SimState.tooltips` and then mutates the tooltip frame. Its own doc comment says it "must be called before layout computation so anchors resolve with correct dimensions" (`src/iced_app/tooltip.rs:58-61`).

The live render path does the opposite:

- `resolve_layout_and_buckets()` calls `ensure_layout_rects()` first
- then calls `update_tooltip_sizes()`
- then builds strata buckets and renders

That means the current frame uses the old `layout_rect` when the tooltip content changed. `build_render_list()` then prefers the stale cached `layout_rect` instead of recomputing it, so the tooltip background and text stay on the previous size for one frame.

## What Is Not The Main Issue

- `collect_tooltip_data()` is fresh. It only copies line text/colors/alpha from `state.tooltips`.
- The mismatch is not primarily a stale tooltip-data copy problem.
- Separate `FontString` children exist for `GetLeftLine()` / `GetRightLine()`, but they are API-facing handles, not the source of this timing bug.

## Evidence

- `src/iced_app/render.rs:460-474` calls layout before tooltip sizing.
- `src/iced_app/tooltip.rs:58-87` mutates frame size and marks rect dirty.
- `src/iced_app/strata_emit.rs:145-165` renders from cached `layout_rect` when present.
- `src/iced_app/render/rebuild.rs:31-63` can also reuse cached strata quads, so stale geometry can persist until a dirty rebuild happens.

## Likely Fix Shape

Move tooltip sizing ahead of layout resolution, or run a second layout pass after sizing. That keeps the tooltip's measured box, anchors, and render bounds in the same frame.

## Sources

- [tooltip.rs](../../../src/iced_app/tooltip.rs)
- [render.rs](../../../src/iced_app/render.rs)
- [strata_emit.rs](../../../src/iced_app/strata_emit.rs)
- [render/rebuild.rs](../../../src/iced_app/render/rebuild.rs)
- [widgets/tooltip.rs](../../../src/lua_api/frame/methods/widgets/tooltip.rs)

## See Also

- [[tooltip-alignment]] — text inset math inside the tooltip shell
- [[tooltip-double-shell]] — duplicate chrome issue in the tooltip render path
