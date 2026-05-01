# EditBox Render Text Cache

Typing into the SimCommands search box updated the EditBox logical text but rendered nothing because the keyboard input path did not refresh `Frame.text_stripped`. The glyph renderer prefers `text_stripped` when present, so after `SetText("")` left a cached empty string, later typed text in `Frame.text` was ignored during shaping.

## Content

`SimCommandsSearchBox` clears itself with `SetText("")` when the palette opens. That updates both `text` and `text_stripped` to an empty string.

Keyboard input then flows through `WowLuaEnv::send_key_press()` and `splice_text_at_cursor()`. Before the fix, that path changed only `Frame.text` and the cursor position. Rendering still passed the stale cached stripped text to `emit_text_quads()`, and `emit_text_quads()` returned early because the cached string was empty.

The fix keeps the render cache synchronized for typed inserts, Backspace, and Delete by refreshing `text_stripped` and clearing formatted `text_segments` after the keyboard path mutates `Frame.text`. Regression coverage lives in `tests/editbox_input_render_text.rs`.

## Sources

- [SimCommands.lua](../../../Interface/AddOns/SimCommands/SimCommands.lua) — palette search box setup and `SetText("")` on show
- [key_dispatch.rs](../../../src/lua_api/key_dispatch.rs) — focused EditBox keyboard input path
- [quad_builders.rs](../../../src/iced_app/quad_builders.rs) — widget text rendering passes cached stripped text
- [glyph.rs](../../../src/render/glyph.rs) — glyph renderer prefers pre-stripped text when supplied
- [editbox_input_render_text.rs](../../../tests/editbox_input_render_text.rs) — regression tests

## See Also

- [[frame-data-flow]] — Lua and Rust frame state synchronization
- [[rendering-pipeline]] — text quad emission and glyph atlas rendering
- [[widget-system]] — Frame text fields and widget render state
