# FontString Default Anchors

Unanchored XML `FontString` layer children do not always default to the parent center. Retail WoW gives them one implicit anchor selected from `justifyH`: `LEFT`, `CENTER`, or `RIGHT`; `justifyV` does not affect that implicit point.

## Content

The simulator previously generated `fs:SetPoint("CENTER", parent, "CENTER", 0, 0)` for every XML `FontString` without explicit `<Anchors>` and without `setAllPoints="true"`. A real-client addon test using nine `justifyH` x `justifyV` permutations showed the actual behavior:

- `justifyH="LEFT"` -> one point: `LEFT` relative to parent `LEFT`, offset `0, 0`
- `justifyH="CENTER"` -> one point: `CENTER` relative to parent `CENTER`, offset `0, 0`
- `justifyH="RIGHT"` -> one point: `RIGHT` relative to parent `RIGHT`, offset `0, 0`
- `justifyV="TOP"`, `"MIDDLE"`, and `"BOTTOM"` leave the implicit anchor unchanged

This is not theoretical. The retail Blizzard UI manifest contains many unanchored XML `FontString`s with direct `justifyH="LEFT"` or `justifyH="RIGHT"` attributes, including Catalog Shop product text, Encounter Timeline labels, UI widget text templates, and shared tooltip/text templates. Anchoring all of those at center shifts real Blizzard text.

The XML code generator now chooses the default `SetPoint` point from the parsed `justify_h` field and falls back to `CENTER` when `justifyH` is absent or unrecognized.

## Sources

- [xml_fontstring.rs](../../../src/loader/xml_fontstring.rs) - default XML `FontString` anchor code generation
- [xml_basics_extra.rs](../../../src/loader/tests/xml_basics_extra.rs) - regression test covering all nine `justifyH` x `justifyV` permutations
- [data/blizzard-ui-files.txt](../../../data/blizzard-ui-files.txt) - retail Blizzard UI manifest used to confirm real usage

## See Also

- [[xml-template-system]] - XML parsing and code generation overview
- [[layout-system]] - anchor resolution behavior
