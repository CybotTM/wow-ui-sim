# Wrath tooltip, password and message-frame contracts

Wrath 12340 contracts from laptop findings batches 4, 6, 7 and 8 (2026-10-18). Original LFR and UIErrorsFrame consumers supply behavioral integration boundaries; native execution is not claimed. Other profiles retain their existing behavior.

Clean-room unit evidence: `wow-tooltip-units-12340-facts.md`, reconstruction revision 0e4e9fe0, independently source-verified by laptop, not native-run. Raw DDC icon gap .0049999999 converts to local UI gap `.005*768*sqrt(aspect²+1)` (6.4 at 4:3); effective layout scale follows afterwards. U3's 38.400002 two-column contribution and 230.39999 left-wrap candidate maximum are already local UI values, not border padding or a universal final-width cap.

XML anchoring evidence: `wow-xml-texture-default-anchors-12340-facts.md`, revision 0e4e9fe0, source-verified by laptop, not native-run. `CSimpleRender.cpp:2237–2251` defaults an XML texture with a parent and no active point to parent TOPLEFT/BOTTOMRIGHT, regardless of Size. This preserves stored dimensions while queried geometry spans the parent (`CLayoutFrame.cpp:921–959`). AddTexture replaces those points with TOPRIGHT-to-text-TOPLEFT (`Tooltip.cpp:3795–3798`), so stored dimensions determine the effective size. This is not evidence about Lua CreateTexture or FontStrings.

## What it must do

- [ ] U2: ClearLines clears row text/identity, used child visibility, left widths/wrap and non-forced minimum width; preserves tooltip size/visibility. OnTooltipCleared is synchronous only when previous rows existed.
- [ ] U1: AddTexture attaches to the last existing row, never inserts a row. Fewer than two rows, missing template slot or ten used slots is a no-op. Preserve stored template dimensions/UVs, distinct from parent-fill queried geometry. Empty path reserves spacing without an image handle.
- [ ] U1: A most-recent attachment on the immediately previous row aligns captions; otherwise offset by current slot width plus converted 0.005 gap. Icon right edge is that gap left of text; pre-anchor the next text slot back by current width plus gap.
- [ ] U3: Nonempty right text disables wrap; no half-width partition. Measure left+right with a local 38.400002 contribution only when both columns are shown (nonempty AddDoubleLine inputs); an empty left column adds no gap. Height depends only on left strings, row gaps and borders. Defer sizing to Show.
- [ ] U3: Wrapped-width candidate is capped at converted 230.39999, not final shared width. Refine only when shared width is smaller; measure up to thirty wrapped byte segments, last through full length. Grow width without revisiting previous rows; bypass preserves existing left width.
- [ ] C4: Password presentation has one star per UTF-8 byte and masked caret geometry, without modifying stored text. XML password="1" applies before OnLoad, including virtual inheritance.
- [ ] C6: MessageFrame defaults are fading=true, visible=10, fade=3; original UIErrorsFrame XML sets visible=5. MessageFrame insertion is queued and processes one row per update, resetting only that row. Fade is linear. ScrollingMessageFrame chat history retains its existing insertion and timestamp-based behavior.
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

- [ ] Retail and other profiles' default XML anchoring is unverified; retained simulator defaults are not native-parity evidence. This slice covers sized stock Wrath slots, not empty/inherited-anchor combinations.

- [ ] Zero-sized texture-slot initialization has no fixture in this slice; retained nonzero template dimensions are covered.

- [ ] Forced-width input and item/context cleanup have no authenticated populated fixtures.
- [ ] Interior-byte cursor positions and XML password values other than 1/0 are unknown. Masked-caret parity is required at UTF-8 boundaries (0, 2, 5 for é猫).
- [ ] Exact native scheduling, image pixels and complete startup compatibility are not established by bounded consumer tests.
- [ ] C5 first-Down step convention, ClearHistory cursor reset and GetHistoryLines capacity-versus-occupancy meaning are not authenticated.

## Out of scope

SetOwner and SetAllowShowWithNoLines are not redefined. Cataclysm's distinct capped-width algorithm and native modern-profile claims are excluded.
