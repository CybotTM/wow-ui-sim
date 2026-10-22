# Wrath F07 scroll, anchor and update fixtures

F07 covers 32 existing integration cases. The REJECT repair restores the original shine timer bound and the original MinimalScrollBar/EventFrame consumer. Fresh counts are pending; historical 11 PASS / 21 FAIL results used weakened/substituted fixtures and are not acceptance evidence. See the [investigation](../wiki/investigations/wrath-fixture-f07.md).

## What it must do

- [x] Inspect cached Faux scrollbar buttons, thumb and textures through their `$parent` names; retain original inline offset/wheel and empty-handler assertions.
- [x] Restore the unchanged EventFrame/MinimalScrollBar anchor fixture on Wrath, ungated. Missing cached declarations are INFERRED absence; this row is a report-only gate candidate, not permission to substitute generic XML.
- [x] Load profile FrameXML for active pulse/shine worklists. Wrath uses cached free functions and named textures; Retail keeps its mixin fixture. Both require `(timer - 0.016).abs() < f64::EPSILON`.
- [x] Retain the separate actual-parent `$parentBackground` fixture and cached Wrath FrameXML corpus parser; Retail keeps its SharedXMLBase corpus.
- [ ] Produce fresh Wrath/Retail counts for all 32 exact rows, Wrath stock lua-errors, both profile checks and fmt. Every selected case must execute more than zero tests.

## Boundaries

No vendor/rilua changes, failure-hiding cfg gates, fallbacks, invented consumer replacements or tolerance relaxation. A failing original shine assertion requires a grounded producer diagnosis; otherwise retain the failure as a producer-bug receipt/report-only row. No native 12340 declaration is authenticated: provenance remains UNCONFIRMED, cached expectations INFERRED. No decompiled source may be read.

## Tests asserting this spec

Existing `integration` rows span `scroll_widgets`, `scroll_widgets_minimal`, `uiparent_onupdate_worklists`, `xml_parsing`, and the cached Faux empty-script override. Exact row statuses and current revision proof will replace this pending statement after execution. No optional generic anchor test is added.

## Related docs

- [Fixture investigation](../wiki/investigations/wrath-fixture-f07.md)
- [Inline XML arguments](xml-scroll-arguments.md)
- [Empty script clearing](xml-empty-script-clearing.md)
- [Anchor reapplication](../wiki/investigations/chatframe-scrollbar-anchor-reapply.md)

## Out of scope

Modern ScrollUtil/ScrollBox/MinimalScrollBar publication on Wrath; physical wheel input; rendering/performance changes; unrelated producer defects; full native-client parity.
