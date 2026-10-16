# Registered XML intrinsic tags

## Contract

A parsed frame tag whose same-name registered template declares `intrinsic="true"`
implicitly inherits that template before explicit instance `inherits`, for both
top-level and nested frames. Original mixins, properties and script ordering must
apply through the existing template pipeline, as for `CreateFrame` inheritance.
Ordinary native tags and virtual templates do not gain implicit inheritance merely
because a same-name non-intrinsic template exists. No arbitrary-tag parsing or
vendor behavior replacement is added.

## Grounding and implementation

Cached Retail `Blizzard_AccessibilityTemplates/AccessibilityIntrinsics.xml:3-9`
defines `UIThemeContainerFrame` with `UIThemeContainerMixin` and a precall OnLoad.
`Mainline/AccessibilityTemplates.lua:3-7` initializes `cvar`, `fontStrings`, `frames`.
Original `Blizzard_UIPanels_Game/Mainline/GossipFrame.xml:34` uses that tag and its
shared Update calls `self:UpdateTheme()`.

`src/loader/xml_frame.rs::registered_intrinsic_tag` reads the registered declaration;
top-level `xml_file.rs` and nested `xml_frame/finalize.rs` append it to their existing
resolved tag mapping. Existing inheritance, mixin and lifecycle code performs the work.

## Tests and evidence

`tests/xml_intrinsic_tags.rs` loads the unchanged active-profile AccessibilityTemplates
addon closure, then uses that original intrinsic definition in a minimal test addon.
It asserts method identity, actual preload-created fields, precall-before-instance
ordering, explicit-template size preservation, nested parent binding and a runtime
CreateFrame control. Test commit `eeb2b3bfa` produced behavioral RED against the saved
Retail normal library: missing original cached mixin. Saved source `f963b6b18`
then passed the focused case, including every preload/nested/CreateFrame assertion.
All `tests/xml_templates/*` cases ran: 170 PASS, one pre-existing ignored tooltip
fast-path TODO, zero failures (38.95s); this includes intrinsic script ordering and
runtime CreateFrame inheritance cases. The separately compiled same-source headless
`wow-sim --no-addons --no-saved-vars lua-errors` returned `[]`, exit0. Original CVar
stat/hash remained unchanged for every runtime. Private receipts live in
`~/.local/state/wow-ui-sim/handoff/quest-lifecycle-build/` under
`intrinsic-green-retail-*` and `intrinsic-startup-retail-*`; no full-suite/native claim.

Retail declaration inventory also exposes previously omitted `EventFrame` and
`EventEditBox` intrinsic behavior, and `DropDownToggleButton`/`EventButton` top-level
intrinsic behavior. Other cached declared intrinsic tags already have explicit
mapping: AuraButton, AuraContainer, DropdownButton, ScrollingMessageFrame,
ContainedAlertFrame, ItemButton, EventScrollFrame. Runtime loading/consumer reachability
is not implied by source inventory.

## Exclusions

Native parity, arbitrary unknown XML tags, new widget domains and vendor patches.
See [XML system](../wiki/systems/xml-template-system.md).
