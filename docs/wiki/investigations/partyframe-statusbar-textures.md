# PartyFrame status-bar textures resolve to nothing

The party frame health and mana bars are not missing atlas data. The simulator builds the bar children through the XML loader, then `SetStatusBarTexture(bar)` drops the source because the setter only reads strings and numbers. The bar texture child is a userdata frame, so `val_to_string()` returns `None`, `apply_statusbar_texture_source()` clears `texture` and `atlas`, and the bar renders as missing. The adjacent mask textures still work because they are set through `SetAtlas()` directly.

## Root Cause

The XML loader creates the bar texture child and then calls:

`src/loader/xml_frame_extras.rs`

- `parent:SetStatusBarTexture(bar)`

That flows into:

`src/lua_api/frame/methods/widgets/statusbar.rs`

- `set_status_bar_texture()` calls `val_to_string(state, texture_val)`
- `val_to_string()` only handles `Val::Str`
- userdata frames fall through to `None`
- `apply_bar_texture()` passes `path = None`
- `apply_status_bar_texture_source()` clears `frame.texture`, `frame.atlas`, and UVs

The relevant Blizzard UI usage is:

`Interface/BlizzardUI/Blizzard_UnitFrame/Mainline/PartyFrameTemplates.xml`

- `BarTexture atlas="UI-HUD-UnitFrame-Party-PortraitOn-Bar-Health"`
- `BarTexture atlas="UI-HUD-UnitFrame-Party-PortraitOn-Bar-Mana"`
- `MaskTexture atlas="UI-HUD-UnitFrame-Party-PortraitOn-Bar-Health-Mask"`
- `MaskTexture atlas="UI-HUD-UnitFrame-Party-PortraitOn-Bar-Mana-Mask"`

The masks still render because `MaskTexture` uses `SetAtlas()` and the atlas lookup is valid for:

- `UI-HUD-UnitFrame-Party-PortraitOn-Bar-Health-Mask`
- `UI-HUD-UnitFrame-Party-PortraitOn-Bar-Mana-Mask`

The bar atlases also exist in `data/atlas.rs` and point at `Interface\\hud\\uipartyframe`, so this is not a missing asset problem.

## Evidence

- `data/atlas.rs:8290-8308` maps the party bar atlases to `Interface\\hud\\uipartyframe` and the masks to separate `UIPartyFramePortraitOn*Mask.BLP` files.
- `src/loader/xml_frame_extras.rs:51-80` creates the bar texture child and calls `parent:SetStatusBarTexture(bar)`.
- `src/lua_api/frame/methods/widgets/statusbar.rs:182-193` accepts only string/number texture values.
- `src/lua_api/methods.rs:445-450` shows `val_to_string()` only succeeds for Lua strings.
- `Interface/BlizzardUI/Blizzard_UnitFrame/Mainline/PartyMemberFrame.lua:31,50,58` sets the bar and mask atlases directly in Blizzard UI code.

## Fix

Fixed in the status-bar setter, not in the atlas database and not in Blizzard UI.

`src/lua_api/frame/methods/widgets/statusbar.rs` now distinguishes the
`SetStatusBarTexture(existingTextureChild)` path from the string / fileDataID
path:

- if the argument is an existing texture userdata, the setter now adopts that
  child as the bar texture and preserves its existing `atlas` / `texture`
  source
- if the argument is a string or fileDataID, the setter still rewrites the bar
  source as before

That keeps the XML loader contract intact: it can keep creating the child,
setting the atlas on that child, then calling `parent:SetStatusBarTexture(bar)`.

## Verification

- `tests/widget_methods_colorselect.rs` now includes
  `test_statusbar_texture_userdata_preserves_existing_atlas_source`
- direct `wow-sim dump-tree --filter-key PartyFrame` probe now shows the party
  mana / rage / focus bars resolving to `Interface\\hud\\uipartyframe` with the
  expected atlases instead of `MISSING`

## See Also

- [[partyframe-tree]] — parent layout investigation for the same Blizzard UI path
- [[xml-template-system]] — loader-side XML element creation order
- [[mask-texture]] — why the masks render correctly

## Sources

- [PartyFrameTemplates.xml](../../../Interface/BlizzardUI/Blizzard_UnitFrame/Mainline/PartyFrameTemplates.xml)
- [PartyMemberFrame.lua](../../../Interface/BlizzardUI/Blizzard_UnitFrame/Mainline/PartyMemberFrame.lua)
- [xml_frame_extras.rs](../../../src/loader/xml_frame_extras.rs)
- [statusbar.rs](../../../src/lua_api/frame/methods/widgets/statusbar.rs)
- [methods.rs](../../../src/lua_api/methods.rs)
- [atlas.rs](../../../data/atlas.rs)
