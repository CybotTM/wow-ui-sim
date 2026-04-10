# Hit Testing

How the simulator determines which frame is under the mouse cursor.

## Algorithm

Hit testing runs on every mouse event (move, click, scroll) in two phases:

**Phase 1 — Grid lookup**: A `HitGrid` (64px cell spatial index) is built from all hittable frames sorted by `(strata, level, id)`. `hit_test(pos)` computes the cell containing the cursor, iterates frames in reverse (highest strata/level first), returns the first whose rect contains the point. O(1) lookup + O(k) scan per cell.

**Phase 2 — Drill down**: Starting from the Phase 1 hit, walks down `frame.children` in reverse order. If a hittable child's rect contains the point, it becomes the new current frame. Repeats to the deepest mouse-enabled descendant — matching WoW's behavior where children receive clicks over parents regardless of frame level.

## Hittable Frame Conditions

A frame is eligible for hit testing when all four are true:
1. `frame.visible == true`
2. `frame.effective_alpha > 0`
3. `frame.mouse_enabled == true`
4. Not in `HIT_TEST_EXCLUDED`: `UIParent`, `WorldFrame`, `Minimap`, `ChatFrame1`, `EventToastManagerFrame`, `EditModeManagerFrame`

## Hit Rect Insets

`SetHitRectInsets(left, right, top, bottom)` shrinks the clickable area inward. Applied during grid construction, not during hit testing. Stored as `frame.hit_rect_insets: (f32, f32, f32, f32)`.

## Mouse Event Flow

| Event | Action |
|---|---|
| Move | `hit_test` → update `hovered_frame` → fire `OnLeave`/`OnEnter` |
| Down | `hit_test` → fire `OnMouseDown`, track for click/drag |
| Up | `hit_test` → if same frame: `OnClick` + `PostClick`; otherwise `OnMouseUp` |
| Scroll | `hit_test` → walk parent chain for `OnMouseWheel` handler |
| Middle click | `hit_test` → open inspector panel (simulator-only) |

Drag: 5px threshold from mouse-down position before firing `OnDragStart`.

## Key Files

- `src/iced_app/hit_grid.rs` — `HitGrid` spatial index
- `src/iced_app/view.rs` — `hit_test()` Phase 1 + Phase 2
- `src/iced_app/frame_collect.rs` — hittable list collection and `HIT_TEST_EXCLUDED`
- `src/iced_app/render.rs` — `build_hittable_rects()` (applies insets, builds grid)
- `src/iced_app/mouse.rs` — mouse event handlers

## Sources

- [hit-testing.md](../../hit-testing.md) — full system description

## See Also

- [[on-update-dirty]] — rendering pipeline context
