# Button active font state

Button text selects a configured FontObject and projects its supplied fields into the existing FontString. This clean-room requirement comes from the user's functional description; historical source comparison is not executed native or universal modern-profile proof. See [widget system](../widget-system.md).

## What it must do

- [ ] Select disabled font while disabled; otherwise select highlight font while hovered or highlight-locked; otherwise select normal font. Pushed state does not suppress highlight-font selection.
- [ ] Refresh selection on enabled/state, hover/lock and active-slot changes. Updating an inactive slot must not overwrite the active style.
- [ ] Preserve the existing FontString identity and update its selected FontObject reference plus supplied font path, size, flags and color fields. Its emitted glyphs must reflect that style.
- [ ] Apply pushed text offset independently of selected font, including pushed plus active highlight.
- [ ] Preserve text, parent, anchors, draw layer and explicit texture ownership. Text selection must not reuse a texture-visibility predicate that suppresses pressed highlight.

## How it works

- [Widget system](../widget-system.md)
- [Existing locked highlight texture contract](button-locked-highlight-rendering.md)
- [FontString spacing and snapshot boundary](fontstring-spacing.md)

## Implementation inventory

- `src/lua_api/frame/methods/button_anchor_hierarchy/buttons.rs`: current font slots and normal-only snapshot; C7 producer pending.
- `src/lua_api/frame/methods/button_anchor_hierarchy/font_strings.rs`: existing FontString reference and snapshot helpers.
- `src/lua_api/frame/methods/text_attribute_event/text/formatting.rs`: public FontString FontObject reference/snapshot behavior.
- `src/iced_app/mouse.rs`: actual GUI hover transition; `A_Admin.SetMouseOverFrame` supplies separate simulated hover.

## Tests asserting this spec

Three new cases in `tests/button_state_textures.rs` are **UNCOMPILED / UNEXECUTED**: highlight/lock/pushed-offset styling, disabled transitions, and active-slot replacement/reference/child identity. Two style cases omit initial reference assertions so a reference-storage failure cannot mask the style boundary. Actual glyph quads are compared with a separately rendered FontString; public `GetFont` observes the selected path. No GPU/pixel/native proof or loaded-face identity claim.

## Known gaps (current cycle)

- [ ] Obtain behavioral RED, implement bounded projection, and verify affected rendering/state controls.
- [ ] Cover actual GUI pointer/hit-test/dispatch; authored tests use public simulated hover, not manual OnEnter callbacks.

## Out of scope

Later shared FontObject mutation propagation is separate T6 work; snapshot projection is not complete native FontObject inheritance. Missing-slot policy and unspecified/local-field restoration are not inferred here. No texture ownership redesign, vendor edits or reactive FontObject graph.
