# SimpleHTML structured presentation

SimpleHTML retains its plain-text API projection separately from document layout. The [owning contract](../../specs/simple-html-structured-content.md) defines bounded paragraphs, headings and anchor gestures, not general HTML/CSS or native metrics.

## Content

`Frame.text` retains source text; `text_stripped` supplies the existing flattened projection. Optional `Frame.simple_html` stores `SimpleHtmlData` styles and hyperlink options; the former separate `SimState.simple_htmls` map is removed.

`simple_html/document.rs` produces ordered p/h1/h2/h3 blocks and anchor byte ranges. `simple_html/layout.rs` shapes them with the configured font/width/spacing, returning block positions, content height and per-glyph link regions. Content-height queries, `simple_html_render.rs` and `simple_html_input.rs` share that layout rather than reconstructing independent wrapping.

Direct XML FontString/header declarations configure roles, not child FontString widgets. Loader role application uses the actual resolved frame global name; an invented `__frame_ID` alias is not available at that phase.

Left-button release dispatches `OnHyperlinkClick(self,target,label,"LeftButton")` only when down/up share a current link region and hyperlinks are enabled. Normal mouse hit eligibility still applies: every ancestor needs layout anchors. Direct quad emission alone does not establish pointer eligibility. Disabled/replaced content cannot supply stale link regions.

## Sources

- [State/styles](../../../src/lua_api/simple_html.rs), [shared layout](../../../src/lua_api/simple_html/layout.rs), [XML roles](../../../src/loader/simple_html.rs)
- [Original-consumer tests](../../../src/iced_app/simple_html_tests.rs)
- [Bounded proof and remaining gates](../../baselines/approved-batch-2026-10-16.md)

## See Also

- [[widget-system]] — frame-owned state and hit eligibility
- [[rendering-pipeline]] — glyph emission and presentation
- [[adventure-guide-simplehtml-markup]] — preserved plain-text projection history
