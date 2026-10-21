# Wrath native chat layout preservation

Wrath startup keeps the geometry and editbox state produced by original `FloatingChatFrame`/`ChatFrame` consumers. Simulator preparation lives in `src/lua_api/chat_init.rs`; [diagnosis](../wiki/investigations/wrath-chat-layout.md) records the boundary and source evidence.

## What it must do

- [ ] Preparation before third-party addons preserves native chat dimensions, anchor, editbox dimensions and visibility.
- [ ] Post-load and post-event preparation do not replace that native surface with simulator-authored modern geometry.
- [ ] Headless normalization does not impose the modern editbox width on Wrath.
- [ ] Retail chat preparation and layout remain unchanged.

## How it works

- [Native chat diagnosis](../wiki/investigations/wrath-chat-layout.md).

## Implementation inventory

- `src/lua_api/chat_init.rs` — native-layout policy and simulator-owned normalization.
- `src/lua_api/workarounds/temporary/post_event_frame_layout.rs` — post-event normalization caller.
- `src/startup.rs` — headless normalization caller.

## Tests asserting this spec

- `tests/chat_frame/native_layout.rs` — original native load followed by pre-addon/post-load/post-event preparation, comparing observable surface snapshots.
- `tests/chat_frame/layout_lock.rs`, `tests/chat_frame/scrollbar.rs` — original native chat/scroll-control geometry.
- `tests/chat_frame.rs` — chat input and Retail preservation cases.

## Known gaps (current cycle)

- [ ] Current-source GREEN and Retail preservation remain pending.

## Out of scope

Native client execution, saved-chat-layout persistence, other profiles' native geometry and unrelated chat protocol changes are not established by this bounded original-consumer regression.
