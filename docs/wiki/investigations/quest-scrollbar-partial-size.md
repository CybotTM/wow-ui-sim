# Quest Scrollbar Partial Size

`QuestScrollFrame.ScrollBar.Track` and its thumb rendered four pixels too far right because their XML template width was never stored. The Blizzard template uses partial sizes such as `<Size x="8"/>` with `TOP`/`BOTTOM` anchors; the simulator previously applied XML sizes only when both `x` and `y` were present, leaving these frames at width `0`.

## Content

Symptoms:

- The vertical scrollbar track/thumb appeared offset from the stepper buttons.
- `dump-tree --filter-key QuestScrollFrame` showed `.Track [Frame] (0x391)` and `.Thumb [Button] (0x289)` while their 8px child textures started at the center anchor line.
- The surrounding `.Back` and `.Forward` buttons were 17px wide by Blizzard design, centered on the same x-axis.

Root cause:

- `MinimalScrollBar.xml` declares the track with `<Size x="8"/>` and `TOP`/`BOTTOM` anchors.
- The layout resolver already preserves explicit width for vertical multi-anchor frames.
- `apply_xml_size()` in the direct XML path merged partial size values but only wrote them when both final width and final height were `Some`.

Fix:

- Apply XML width and height independently in `apply_xml_size()`.
- Preserve `width_is_text_auto = false` when XML sets width, matching explicit `SetWidth()` behavior.
- Add `test_xml_partial_size_preserves_single_dimension` to cover both `<Size x="..."/>` and `<Size y="..."/>`.

Verification:

- `cargo test --lib test_xml_partial_size_preserves_single_dimension`
- `cargo build --bin wow-sim`
- Quest scrollbar dump after the fix shows `.Track [Frame] (8x391) [stored=8x0]` and `.Thumb [Button] (8x289)` at x=1034, matching the 8px track textures.

## Sources

- [MinimalScrollBar.xml](../../../vendor/wow-ui-source/Interface/AddOns/Blizzard_SharedXML/Shared/Scroll/MinimalScrollBar.xml) — Blizzard scrollbar template with partial track size
- [direct.rs](../../../src/lua_api/globals/template/direct.rs) — XML size merge and application path
- [xml_basics.rs](../../../src/loader/tests/xml_basics.rs) — partial XML size regression test

## See Also

- [[layout-system]] — multi-anchor layout preserves explicit size on the orthogonal axis
- [[xml-template-system]] — XML inheritance and property application
- [[chatframe-scrollbar-anchor-reapply]] — another scrollbar layout regression rooted in XML/template application
