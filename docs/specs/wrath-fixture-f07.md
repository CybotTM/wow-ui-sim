# Wrath F07 scroll, anchor and update fixtures

F07 covers 32 existing integration cases: 11 repairable fixture rows and 21 unchanged modern-only report rows. Its profile-aware fixtures live in `tests/`; they exercise cached Wrath FrameXML or explicitly authored generic XML, not substitute modern APIs. See the [fixture investigation](../wiki/investigations/wrath-fixture-f07.md).

## What it must do

- [x] Inspect Faux scrollbar buttons and thumb through the cached `$parent` global names, including button textures; do not require modern parentKey fields.
- [x] Exercise the original cached Faux inline vertical-scroll/wheel handlers and empty range-handler override. Retain sentinel-global isolation and exact scrollbar/frame offsets. Direct handler invocation is not physical wheel-input proof.
- [x] Preserve outer child anchors and inherited inner anchors using an explicitly authored generic Frame control on Wrath; resolve `$parentBackground` against the actual parent during child-anchor reapplication.
- [x] Load profile FrameXML before testing active button-pulse and animated-shine worklists. Wrath shine uses original free functions and named template textures; Retail retains its existing mixin fixture.
- [x] Parse the cached Wrath FrameXML directory for the corpus case; retain the existing SharedXMLBase corpus on Retail.
- [ ] Preserve all 32 Retail outcomes across the batch and retain all 21 modern-only Wrath failures unchanged and ungated.

## How it works

- [Fixture investigation](../wiki/investigations/wrath-fixture-f07.md)
- [Inline XML arguments](xml-scroll-arguments.md)
- [Empty script clearing](xml-empty-script-clearing.md)
- [Anchor reapplication](../wiki/investigations/chatframe-scrollbar-anchor-reapply.md)

## Implementation inventory

- `tests/common/mod.rs`: profile-aware cached panel-template loading and named child lookup.
- `tests/scroll_widgets.rs`: Faux named scrollbar children and button textures; unchanged modern report rows.
- `tests/scroll_widgets/xml_arguments.rs`: cached Faux XML argument consumer.
- `tests/xml_templates/inline_advanced/empty_script_overrides.rs`: cached Faux empty-function consumer.
- `tests/scroll_widgets_minimal.rs`: authored generic anchor control on Wrath and unchanged modern report rows.
- `tests/uiparent_onupdate_worklists.rs`: profile FrameXML loading and cached Wrath animated-shine fixture.
- `tests/xml_parsing.rs`: profile-specific corpus directory; unchanged modern file-specific cases.

## Tests asserting this spec

The files above are registered under the existing `integration` target. Each F07 invocation selects one fully qualified case with `--exact` and must execute more than zero tests. The worklist cases have nested timeout-process summaries; these are not additional distinct cases.

At resumed base `62ed26a4547f405dc19c52bdaff3de1b4b1d95be`, Wrath is **11 PASS / 21 FAIL**. All 11 repairable rows pass before any new producer change; the corpus parses 140 XML files. Retail at the same base is **32 PASS / 0 FAIL**. These baselines do not justify inventing a producer fix. Wrath stock `--no-addons --no-saved-vars lua-errors` returns byte-exact `[]\n` with exit 0 at that base. After counts and final profile checks remain pending.

## Known gaps (current cycle)

- [ ] Native 12340 provenance: the laptop's 2026-10-21 response marks the requested UIParent pulse/shine functions, texture names and Faux declarations UNCONFIRMED. No version-proven shipped UIParent.lua/UIPanelTemplates.xml was available. Cached expectations remain INFERRED, not authenticated native acceptance or absence evidence.
- [ ] Preserve the 21 literal modern-only cases as report-only candidates, not passing tests. No new cfg exclusions, ignores or replacement capabilities are authorized.

## Out of scope

Modern ScrollUtil/ScrollBox/MinimalScrollBar publication on Wrath; vendor or rilua edits; physical wheel input; rendering/performance changes; unrelated producer defects; full native-client parity. Undefined native behavior remains a gap, not an implementation requirement.
