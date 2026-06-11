# XML Scale Attribute Ignored

Hero talent node buttons overflowed the bottom edge of the 284×362
`ExpandedContainer` backplate in the class talents panel. Root cause: the
XML `scale` attribute on `<Frame>` elements was silently dropped, so
`HeroTalentsTreeNodesContainerTemplate`'s `scale="0.85"` never applied.

Fixed in commit `91835d898` (2026-06-11).

## Symptoms

- Hero talents box (atlas `talents-heroclass-backplate-full-expanded`,
  fixed 284×362, no runtime resize) did not encompass the bottom node rows.
- `NodesContainer:GetScale()` returned 1.0 instead of 0.85.

## Root cause

`FrameXml` in `src/xml/types.rs` had no `@scale` field; serde ignores
unknown attributes, so all 127 `scale="..."` occurrences in Blizzard XML
were no-ops. No `apply_xml_scale` existed in either property-application
path.

Blizzard sizes the hero tree purely statically:

- `Blizzard_PlayerSpells/ClassTalents/Blizzard_HeroTalentsContainer.xml`
  line 255: `ExpandedContainer` is hardcoded 284×362 with a
  `setAllPoints` background.
- Line 11: `HeroTalentsTreeNodesContainerTemplate` declares
  `scale="0.85"`, anchors `TOP y=-90` + inline `LEFT x=60`,
  `BOTTOMRIGHT x=-60 y=60`.
- Buttons span 272 local units vertically
  (`UpdateHeroTalentButtonPosition` →
  `TalentFrameUtil.GetNormalizedSubTreeNodePosition`).

Anchor offsets apply in the scaled child's coordinate space, so the
container's local height is `362/0.85 − 90 − 60 ≈ 275.9` — just enough
for the 272-unit tree. At scale 1.0 the available space shrinks to 212
local units and the bottom rows spill past the backplate.

## Fix

- `src/xml/types.rs`: `scale: Option<f32>` with `#[serde(rename = "@scale")]`.
- `src/lua_api/globals/template/direct.rs`: `set_scale` (mirrors
  `set_alpha`; calls `propagate_effective_scale` + `mark_rect_dirty`,
  rejects non-positive values) and `apply_xml_scale` (instance value
  first, then first match in the template chain).
- Wired into both property paths:
  `src/loader/xml_frame/setup.rs:apply_xml_properties_direct` (XML loader)
  and `src/lua_api/globals/create_frame/template_chain/runtime.rs:
  apply_runtime_child_direct_properties_with_inherits` (runtime
  `CreateFrame` with template — also covers the top-level frame via
  `apply_runtime_template_direct_properties`).

Regression test: `template_scale_attribute_is_inherited_and_affects_layout`
in `tests/template_child_anchor_override.rs` (mirrors the Blizzard
templates, asserts scale inheritance and the 275.9 local height).

## Related

- [hero-spec-dialog-anchors](hero-spec-dialog-anchors.md) — earlier anchor
  preservation fix for the same NodesContainer template chain.
