# Wrath native chat layout overwritten during preparation

F02's original Wrath FrameXML fixture exposed simulator preparation replacing a successfully loaded native chat layout. The [contract](../../specs/wrath-chat-layout.md) owns requirements and bounded RED→GREEN, headless and Retail preservation proof.

## Root cause and boundary

`chat_init::activate_chat_frame` executed modern normalization for every profile. A bound original-consumer regression at `60fcdf225` captured `430x120`, BOTTOMLEFT `(32,130)` and editbox `440x32`, then preparation changed the chat to `430x170`, `(35,50)`. These are observed fixture values, not universal native constants. Native editbox visibility was already true; the XML template's hidden default alone is not its post-load state.

Capturing after `PLAYER_ENTERING_WORLD` hid the defect: `env_events.rs` automatically runs post-event workarounds at that boundary. The regression instead follows actual pre-addon ordering: unchanged native FrameXML load, snapshot, preparation, then post-load/post-event checks.

## Implementation

Wrath leaves activation geometry to original FrameXML. Existing non-Wrath normalization remains intact. Shared editbox-width normalization retains the existing post-event parent-frame guard and headless guard, while declining to replace native Wrath geometry. No vendor Lua, profile-specific replacement coordinates or editbox visibility guesses are installed.

## Sources

- [Chat initialization](../../../src/lua_api/chat_init.rs).
- [Post-event layout](../../../src/lua_api/workarounds/temporary/post_event_frame_layout.rs).
- [Startup](../../../src/startup.rs).
- [Native regression](../../../tests/chat_frame/native_layout.rs).
- [Committed Wrath inventory](../../../data/blizzard-ui-files/wrath.txt), loading original `FrameXML/FloatingChatFrame.xml` and `FrameXML/ChatFrame.xml`.

## See Also

- [Chat preservation contract](../../specs/wrath-chat-layout.md).
- [Widget system](../systems/widget-system.md).
