# Retail tooltip path separator (U1)

Bounded simulator contract for the original cached Retail `SocialUIUtil.AddSeparatorToTooltip` consumer. Source: `src/lua_api/frame/methods/widgets/tooltip/line_frames.rs` and `src/iced_app/tooltip.rs`. Staged proposal only; numeric behavior is preserved. Geometry below is an explicit simulator policy, not established native semantics.

## What it must do

- [ ] Execute the unchanged cached `retail/AddOns/Blizzard_SocialUIShared/SocialUIUtil.lua:138-152` and attach its path/options divider to the wrapped blank row it just added, without increasing NumLines again.
- [ ] Emit one deferred texture request for that path between Header and Footer, spanning the content width with zero horizontal margins.
- [ ] Proposed geometry policy: supplied height=10 is image height; row height is height + top + bottom margins (6 here). Top=-2 places the image 2 units above the row origin; bottom=-2 extends it 2 units below the row. Existing inter-row spacing remains unchanged. Apply effective scale and tooltip alpha.
- [ ] ClearLines removes the attachment; rebuilding text-only content emits no stale divider.
- [ ] Numeric and numeric-string AddTexture inputs still append their own rows, including when an options table is supplied; keep FileDataId storage unchanged.

## How it works

- [Rendering pipeline](../rendering-pipeline.md)
- [Widget system](../widget-system.md)

## Implementation inventory

- `src/lua_api/tooltip.rs`: attached path and height/margin data.
- `src/lua_api/frame/methods/widgets/tooltip/line_frames.rs`: bounded path/options decoding and existing-row attachment.
- `src/lua_api/frame/methods/widgets/tooltip/sizing.rs`: immediate Lua-visible row sizing.
- `src/iced_app/tooltip.rs`: measured sizing, render collection and deferred path quad emission.

## Tests asserting this spec

- `tests/tooltip_separator_u1.rs`: original complete cached Lua file, helper invocation, row/text order, emitted quad bounds at scales 1 and 0.5, alpha, clear/rebuild and numeric preservation. No image decode or CASC access required for deferred quad evidence.
- `tests/tooltip_text.rs`: existing numeric and numeric-string behavior controls.

## Known gaps (current cycle)

- [ ] Obtain explicit acceptance of proposed geometry policy before landing producer.
- [ ] Execute test-only patch and establish behavioral RED before producer lands. Predicted RED: no divider texture request, not a compile failure.
- [ ] Execute combined patches and preserve exact revision/output; no build or runtime evidence accompanies staging.

## Out of scope

Other texture anchor modes, path-only/legacy positional calls, arbitrary nonblank-row attachments, numeric rendering changes, atlas changes, native parity, texture decode/assets/GPU acceptance, other profiles and vendor Lua modifications.
