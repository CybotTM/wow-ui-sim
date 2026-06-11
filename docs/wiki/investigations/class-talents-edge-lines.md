# Class Talents Edge Lines

Class-talent connector lines disappeared because two simulator contracts disagreed with Blizzard's edge positioning flow: `IsRectValid()` did not resolve dirty anchored layouts, and the render list rejected endpoint-positioned `Line` widgets under anchorless edge frames.

## Root Cause

`TalentEdgeArrowMixin:UpdatePosition()` guards line placement with `startButton:IsRectValid()` and `endButton:IsRectValid()` before calling `Line:SetStartPoint()` / `Line:SetEndPoint()`. The simulator's `IsRectValid()` only checked the rect-dirty flag, so dirty-but-resolvable buttons stayed invalid and Blizzard never assigned line endpoints.

After that was fixed, the line widgets still needed render-list support. Blizzard edge frames are anchorless containers, and the `Line` children have no normal anchors; they are positioned by `line_start` / `line_end`. The render list now treats explicit line endpoints as renderable geometry and allows children anchored entirely to non-parent targets to render through anchorless geometry-carrier parents.

## Fix

- `IsRectValid()` now calls `resolve_rect_if_dirty()` before checking the dirty flag.
- `strata_emit` recognizes `Line` widgets with explicit start/end target IDs as renderable.
- `strata_emit` skips the ancestor rect requirement when a frame's geometry is independent of its parent, preserving the existing guard for ordinary children anchored to anchorless parents.

## Regression Coverage

- `test_set_size_marks_dirty` now asserts `IsRectValid()` resolves a dirty anchored frame.
- `render_list_keeps_line_with_endpoint_geometry_under_unanchored_edge_frame` asserts an endpoint-positioned line survives render-list filtering and emits a quad.
- `render_list_keeps_child_anchored_to_non_parent_under_unanchored_edge_frame` covers the talent arrowhead pattern.

## Sources

- [region.rs](../../../src/lua_api/frame/methods/core_state/region.rs) — `IsRectValid()` dirty-layout resolution
- [strata_emit.rs](../../../src/iced_app/strata_emit.rs) — render-list geometry eligibility
- [strata_emit_endpoint_tests.rs](../../../src/iced_app/strata_emit_endpoint_tests.rs) — endpoint line and arrowhead coverage
- [layout_size.rs](../../../src/loader/tests/layout_size.rs) — `IsRectValid()` regression coverage

## See Also

- [[class-talents-edge-frame-levels]] — adjacent edge rendering issue around frame-level ordering
- [[class-talents-trait-loadout-state]] — class talent state and hero subtree visibility model
- [[unanchored-frame-render-leak]] — why ordinary children of unanchored parents stay filtered
