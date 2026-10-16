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

- `src/lua_api/frame/methods/button_anchor_hierarchy/buttons.rs`: font slots and button font projection; C7 producer committed in `3b4dc7c99`, with no GREEN evidence here. The saved baseline predates that producer.
- `src/lua_api/frame/methods/button_anchor_hierarchy/font_strings.rs`: existing FontString reference and snapshot helpers.
- `src/lua_api/frame/methods/text_attribute_event/text/formatting.rs`: public FontString FontObject reference/snapshot behavior.
- `src/iced_app/mouse.rs`: actual GUI hover transition; `A_Admin.SetMouseOverFrame` supplies separate simulated hover.

## Tests asserting this spec

Saved Retail-epoch baseline `d16876fcc9ec0e49851436a6af7a1e308d0a344c` executed all three C7 cases in `tests/button_state_textures.rs`: **0 passed, 3 failed, exit 101**. The private saved epoch is `c8-producer-c7-c9-retail-baseline-20261016T003140348455Z`, selection `button-font-first-red`. Highlight/lock/pushed-offset styling and disabled transitions reached Lua assertion failures; active-slot replacement reached the explicit `selected FontObject reference` assertion failure. These are executed failures, not compilation failures; later assertions do not independently establish RED or acceptance.

Two style cases omit initial reference assertions so a reference-storage failure cannot mask the style boundary. The authored tests compare emitted glyph quads with a separately rendered FontString and observe the selected path through public `GetFont`; the failing baseline does not prove those assertions pass. C7 producer `3b4dc7c99` has no GREEN evidence here. Modern native, all-profile, GPU/pixel behavior and loaded-face identity remain unproven.

## Known gaps (current cycle)

- [ ] Verify the committed bounded projection against the saved three-case RED and affected rendering/state controls; no producer GREEN or independent verification is established here.
- [ ] Cover actual GUI pointer/hit-test/dispatch; authored tests use public simulated hover, not manual OnEnter callbacks.

## Out of scope

Later shared FontObject mutation propagation is separate T6 work; snapshot projection is not complete native FontObject inheritance. Missing-slot policy and unspecified/local-field restoration are not inferred here. No texture ownership redesign, vendor edits or reactive FontObject graph.
