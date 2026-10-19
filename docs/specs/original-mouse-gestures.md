# Original consumer mouse move and resize gestures

Draft input contract for original DamageMeter sizing and Wrath floating-chat moving consumers. Input/motion code lives in `src/iced_app/`; native method state lives in `src/lua_api/frame/methods/misc/drag_input.rs`. See [layout system](docs/layout-system.md). Staged assertions have not been compiled or executed; no checkbox below denotes an accepted pass.

## What it must do

### DamageMeter sizing

- [ ] Original DamageMeter InitializeResizeButton mouse-down starts sizing; actual subsequent mouse motion changes geometry without adding synthetic drag registration or replacement Lua callbacks.
- [ ] Bottom-right sizing preserves top-left; bottom-left sizing preserves top-right. Positive outward movement increases dimensions; bounds constrain logical width and height.
- [ ] Stationary release and release after motion clear native sizing/moving flags, active drag state, original isResizing and original userPlaced as set by the original mouse-up closure.

### Wrath floating-chat moving

- [ ] Original chat tab OnDragStart moves the owning chat frame by the mouse delta, without resizing it or replacing the XML handler.
- [ ] After release, original FCFTab_OnUpdate reaches FCFTab_OnDragStop/FCF_StopDragging and clears motion state, MOVING_CHATFRAME and tab dragButton.

## How it works

- [Layout system](docs/layout-system.md).
- [Event system](docs/event-system.md).

## Implementation inventory

- `src/iced_app/mouse.rs` — real GUI down/move/up dispatch, drag threshold/capture and release routing.
- `src/iced_app/mouse_drag.rs` — movement, sizing/bounds, fixed-corner reanchoring.
- `src/lua_api/frame/methods/misc/drag_input.rs` — StartMoving/StartSizing/StopMovingOrSizing state.
- Proposed `src/iced_app/mouse_sequence_probe.rs` — Rust-only integration adapter to existing App mouse dispatch. No producer fix and no Lua behavior substitution.

## Tests asserting this spec

- Proposed `tests/f2_original_mouse_consumers.rs` — separate right/left sizing, min/max, scale, stationary release, moved release, outside release and original Wrath XML chat gesture.
- Existing `src/iced_app/mouse_drag_scaled_tests.rs` — synthetic moving-scale preservation control; not original resize acceptance.

## Known gaps (current cycle)

- [ ] Expose the staged Rust adapter before compiling integration tests; missing exports are setup work, not behavioral RED.
- [ ] First observe RED at original DamageMeter geometry after mouse-down, not missing cache, script-loading or compiler failures.
- [ ] After the first demonstrated motion defect is addressed, evaluate release and scale tests independently; current source suggests further capture/release and scale problems.
- [ ] Confirm outside-release ordering and scaled native resize behavior with laptop observations.
- [ ] Wrath full original FrameXML loading may expose unrelated startup failures; classify them instead of adding shims. Other-profile DamageMeter applicability is unproved.

## Out of scope

- Keyboard dispatch, quest paths, EditBox paths, CompactUnitFrame vehicle behavior and vendor edits.
- Automatic docking policy redesign, persistence policy redesign, full GUI startup and all-profile acceptance.
- Rewriting Blizzard callbacks or registering synthetic OnDragStart to make the resize tests pass.
