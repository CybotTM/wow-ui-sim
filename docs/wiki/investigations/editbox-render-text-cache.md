# EditBox Input Text Rendering

Typing into the SimCommands search box can fail visibly for two independent reasons: stale EditBox render text (`Frame.text_stripped`) and render ordering where opaque child background textures cover the EditBox's internal text/caret emitter.

## Content

### Stale stripped text cache

`SimCommandsSearchBox` clears itself with `SetText("")` when the palette opens. That updates both `text` and `text_stripped` to an empty string.

Keyboard input then flows through `WowLuaEnv::send_key_press()` and `splice_text_at_cursor()`. Before the fix, that path changed only `Frame.text` and the cursor position. Rendering still passed the stale cached stripped text to `emit_text_quads()`, and `emit_text_quads()` returned early because the cached string was empty.

The fix keeps the render cache synchronized for typed inserts, Backspace, and Delete by refreshing `text_stripped` and clearing formatted `text_segments` after the keyboard path mutates `Frame.text`. Regression coverage lives in `tests/editbox_input_render_text.rs`.

### Child background covering input text

`CreateSearchBox()` also creates an opaque child `BACKGROUND` texture with `SetAllPoints()`. The simulator's strata DFS normally emitted a frame before its texture regions, and EditBox text/caret are emitted by the EditBox frame itself rather than by a separate FontString region. That meant the parent EditBox emitted typed glyphs first, then the child background texture rendered on top and hid them.

The fix special-cases EditBox render ordering in `state_render_buckets.rs`: same-strata child regions render before the EditBox frame emitter, so the background remains underneath internal input text and caret. The focused regression is `editbox_regions_render_before_internal_text_emitter`.

## Sources

- [SimCommands.lua](../../../Interface/AddOns/SimCommands/SimCommands.lua) — palette search box setup and `SetText("")` on show
- [key_dispatch.rs](../../../src/lua_api/key_dispatch.rs) — focused EditBox keyboard input path
- [state_render_buckets.rs](../../../src/lua_api/state_render_buckets.rs) — DFS render ordering for frames and regions
- [quad_builders.rs](../../../src/iced_app/quad_builders.rs) — widget text rendering passes cached stripped text
- [quad_builders_button.rs](../../../src/iced_app/quad_builders_button.rs) — EditBox frame emitter draws internal text and caret
- [glyph.rs](../../../src/render/glyph.rs) — glyph renderer prefers pre-stripped text when supplied
- [editbox_input_render_text.rs](../../../tests/editbox_input_render_text.rs) — regression tests

## See Also

- [[frame-data-flow]] — Lua and Rust frame state synchronization
- [[rendering-pipeline]] — text quad emission and glyph atlas rendering
- [[widget-system]] — Frame text fields and widget render state
