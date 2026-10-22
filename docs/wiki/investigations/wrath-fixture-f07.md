# Wrath F07 fixture boundaries

F07 separates 11 repairable scroll/anchor/update fixture rows from 21 literal modern-only rows. The [contract](../../specs/wrath-fixture-f07.md) owns requirements and bounded proof. This is cached-runtime compatibility evidence, not authenticated native 12340 parity.

## Content

The original fixtures mixed modern SharedXML assumptions with Wrath inputs: parentKey child fields, MinimalScrollBar/EventFrame structure, AnimatedShineMixin and a SharedXMLBase corpus directory. Some update fixtures also omitted Wrath's top-level FrameXML load. These are fixture boundaries; modern capability absence must not be disguised by substituting a different API under the same test name.

Faux panel fixtures load the profile's original cached `UIPanelTemplates.lua/xml`; named `$parent` globals expose scrollbar buttons and thumb. The original inline XML consumers retain exact offset/wheel synchronization and empty-handler clearing assertions. F06 already changed five of these functions; their historical RED receipts bind saved originals, not the modified F07 base functions.

For generic anchor inheritance, Wrath uses explicitly authored nested Frame templates with the same outer/inner anchor assertions. This is simulator XML coverage, not a claim that an invented control ships in Wrath. The separate dollar-parent button fixture is retained.

Active update fixtures load profile FrameXML before settling startup. Wrath's shine owner has a named `Shine` child using `AnimatedShineTemplate`; its named textures are exercised by original `AnimatedShine_Start`, update and stop functions. Retail retains the existing mixin fixture. No vendor or update-handler rewrite is part of F07.

The corpus case selects the directory beside Wrath's FrameXML TOC instead of modern SharedXMLBase; 140 cached XML files parse at the resumed base. File-specific modern XML cases remain unchanged and failing report-only rows.

## Evidence limits

The laptop's 2026-10-21 F07 response has no version-proven shipped 12340 UIParent.lua or UIPanelTemplates.xml. Pulse/shine functions, Shine1..4 naming and Faux empty-range/named-child declarations are all UNCONFIRMED. Expectations derived from the committed cache remain INFERRED. Lack of cache hits for modern identifiers is not authenticated native absence. No decompiled source was read.

Before at `62ed26a45` and after at docs revision `c28dfe7f7`, all 11 repairable cases pass; only the 21 unchanged report-only rows fail. Retail is 32/32 on both sides. Wrath stock returns byte-exact `[]\n` on both sides; both profile checks and fmt pass without warnings. The [contract](../../specs/wrath-fixture-f07.md#tests-asserting-this-spec) records artifact reuse and proof limits. No new producer defect is reproduced, so no producer change is warranted. Historical 32-case RED receipts remain separate from these fresh executions.

## Sources

- [F07 contract](../../specs/wrath-fixture-f07.md) — scope, tests and proof status.
- [Scroll fixtures](../../../tests/scroll_widgets.rs) — original cached Faux consumers and modern-only rows.
- [Anchor fixtures](../../../tests/scroll_widgets_minimal.rs) — authored generic and dollar-parent controls.
- [Update fixtures](../../../tests/uiparent_onupdate_worklists.rs) — profile FrameXML and active worklists.
- [XML corpus fixture](../../../tests/xml_parsing.rs) — profile-specific corpus selection.

## See Also

- [[chatframe-scrollbar-anchor-reapply]] — actual-parent substitution.
- [[xml-template-system]] — template inheritance and handler installation.
