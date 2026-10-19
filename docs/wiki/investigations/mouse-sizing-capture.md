# Original mouse sizing capture

Original Retail DamageMeter resize callbacks exposed simulator motion-capture and release-routing defects. The bounded contract lives in [original mouse gestures](../../specs/original-mouse-gestures.md); native outside-release ordering and scaled-resize policy remain unproved.

## Root cause

`StartSizing` set native sizing flags without selecting the active motion frame. Subsequent physical movement therefore had no frame to resize. Separately, crossing the drag threshold marked a script drag even when no registered OnDragStart existed, redirecting release away from the original OnMouseUp cleanup.

`StartSizing` now captures the resized frame. `StopMovingOrSizing` releases matching capture. Threshold dispatch marks a script drag only when a registered handler starts; an ordinary sizing gesture retains ordinary mouse-up routing. Blizzard callbacks are unchanged.

## Proof boundary

Tests were committed before the producer repair. Five original DamageMeter cases reproduced geometry/moved-release failures; stationary release passed. All six cases pass after repair, alongside five existing native sizing-state controls. Omitting physical mouse-up fails the stationary stop assertion. Geometry cases stop before release so bounds do not accidentally assert outside-handle ordering. Wrath original chat proof is tracked separately by the contract.

## Sources

- [Gesture contract](../../specs/original-mouse-gestures.md) — acceptance and exclusions.
- [Mouse dispatch](../../../src/iced_app/mouse.rs) — physical threshold and release routing.
- [Native motion methods](../../../src/lua_api/frame/methods/misc/drag_input.rs) — sizing capture and stop state.
- [Original-consumer tests](../../../tests/f2_original_mouse_consumers.rs) — shared Retail/Wrath module.

## See Also

- [[widget-system]] — native widget state.
- [[event-system]] — original script dispatch.
