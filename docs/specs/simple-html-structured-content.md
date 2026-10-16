# SimpleHTML structured content (H1)

Bounded simulator contract for paragraph/header geometry and HTML anchor callbacks. Sources live in `src/lua_api/simple_html/`, text API methods and GUI emission/input. Approval: H1, 2026-10-16. This is not native-parity evidence.

## What it must do

- [ ] Lay out p/h1/h2/h3 as separate ordered blocks, honoring their configured font sizes/colors and line spacing; retain plain-text GetText behavior separately from document presentation.
- [ ] Render and measure the same wrapping/block geometry; GetContentHeight includes all blocks and changes with resolved width.
- [ ] Retain HTML anchor targets and labels; glyph hit regions include wrapped continuation lines, not the whole paragraph rectangle.
- [ ] A left-button press/release within a link dispatches OnHyperlinkClick(self, target, label, "LeftButton"); blank/header clicks, disabled links and replaced content do not dispatch stale targets.
- [ ] Execute original cached Wrath ItemTextFrame ITEM_TEXT_READY content producer and Retail GlueDialog SimpleHTML callback in isolated dependency fixtures. URL sink is capture-only, never an OS launch.

## How it works

- [Widget architecture](../widget-system.md)
- [Rendering](../rendering-pipeline.md)

## Implementation inventory

- `src/lua_api/simple_html.rs` and `simple_html/`: structured content and layout.
- `src/lua_api/frame/methods/text_attribute_event/text/`: styles and measurement.
- `src/iced_app/quad_builders.rs`, `simple_html_render.rs`, `mouse.rs`: presentation/input.

## Tests asserting this spec

- `src/iced_app/simple_html_tests.rs`: original cached consumer regions/functions; emitted geometry and pointer-to-callback assertions.
- `tests/simple_html.rs`: existing public text/style/width contracts remain preservation tests.

## Known gaps (current cycle)

- [ ] All new tests are staged and unexecuted. Behavioral RED is predicted, not observed. Producers must not land before an observed in-scope RED.
- [ ] Build/type-check, runtime, profile/startup and native verification remain open.

## Out of scope

General HTML/CSS, tables/images, external resources, browser behavior, URL launching, full glue boot, other profiles and native equivalence. Unknown tags supply no layout semantics; malformed HTML behavior is not a compatibility promise.
