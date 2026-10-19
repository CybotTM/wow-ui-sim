# Wrath tooltip, password and message-frame contracts

Wrath 12340 contracts from laptop findings batches 4, 6, 7 and 8 (2026-10-18). Original LFR and UIErrorsFrame consumers supply behavioral integration boundaries; native execution is not claimed. Other profiles retain their existing behavior.

## What it must do

- [ ] U2: ClearLines clears row text/identity, used child visibility, left widths/wrap and non-forced minimum width; preserves tooltip size/visibility. OnTooltipCleared is synchronous only when previous rows existed.
- [ ] U1: AddTexture attaches to the last existing row, never inserts a row. Fewer than two rows, missing template slot or ten used slots is a no-op. Preserve template dimensions/UVs. Empty path reserves spacing without an image handle.
- [ ] U1: A most-recent attachment on the immediately previous row aligns captions; otherwise offset by current slot width plus converted 0.005 gap. Icon right edge is that gap left of text; pre-anchor the next text slot back by current width plus gap.
- [ ] U3: Nonempty right text disables wrap; no half-width partition. Measure left+right with converted fixed gap; height depends only on left strings, row gaps and borders. Defer sizing to Show.
- [ ] U3: Wrapped-width candidate is capped at converted 230.39999, not final shared width. Refine only when shared width is smaller; measure up to thirty wrapped byte segments, last through full length. Grow width without revisiting previous rows; bypass preserves existing left width.
- [ ] C4: Password presentation has one star per UTF-8 byte and masked caret geometry, without modifying stored text. XML password="1" applies before OnLoad, including virtual inheritance.
- [ ] C6: MessageFrame defaults are fading=true, visible=10, fade=3; original UIErrorsFrame XML sets visible=5. Queue insertion processes one row per update and resets only that row. Fade is linear.
- [ ] C6: SetTimeVisible resets only nonzero active visible timers; SetFadeDuration resets only nonzero fade timers. Expired rows must not revive.
- [ ] C5: History capacity zero disables insertion/navigation. Preserve duplicate physical ring slots; insertion overwrites then advances. Shrink keeps physical prefix and clamps cursor to new capacity minus one. Single-line arrow navigation skips empty slots and wraps, without bypassing arrow admission. Wrath has no OnArrowPressed script; engine-side motion/history is direct. Other profiles retain their callback behavior.
- [ ] C5: Capacity three with A,B,C,D, then shrink to two, recalls D,B,D on successive Up presses.

## How it works

- [Widget system](../widget-system.md)
- [Rendering pipeline](../rendering-pipeline.md)

## Implementation inventory

- `src/lua_api/frame/methods/widgets/tooltip/` — tooltip line/attachment producers.
- `src/lua_api/tooltip.rs` — tooltip state.
- `src/iced_app/tooltip.rs` — measured tooltip layout.
- `src/widget/editbox.rs` — password presentation.
- `src/xml/types.rs` and `src/lua_api/globals/template/direct.rs` — XML attributes.
- `src/lua_api/message_frame.rs` — message lifecycle.
- `src/lua_api/frame/methods/widgets/message_frame/` — message producers.
- `src/iced_app/message_frame_render.rs` — row alpha projection.

## Tests asserting this spec

`tests/widgets_b6_u2.rs`, `tests/widgets_b6_u1.rs`, `tests/widgets_b6_u3.rs`, `tests/widgets_b6_c4.rs`, `tests/widgets_b6_c6.rs`, `tests/widgets_b6_c5.rs`. Geometry assertions are relational, never absolute font-metric native oracles. Preserve existing `tests/key_dispatch.rs` and `tests/editbox_arrow_pressed.rs`.

## Known gaps (current cycle)

- [ ] Forced-width input and item/context cleanup have no authenticated populated fixtures.
- [ ] Interior-byte cursor positions and XML password values other than 1/0 are unknown.
- [ ] Exact native scheduling, image pixels and complete startup compatibility are not established by bounded consumer tests.
- [ ] C5 first-Down step convention, ClearHistory cursor reset and GetHistoryLines capacity-versus-occupancy meaning are not authenticated.

## Out of scope

SetOwner and SetAllowShowWithNoLines are not redefined. Cataclysm's distinct capped-width algorithm and native modern-profile claims are excluded.
