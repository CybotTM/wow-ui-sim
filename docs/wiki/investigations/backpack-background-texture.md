# Backpack Background Texture

User reported the backpack body in the simulator displays as flat gray, while
their retail screenshot of the open Backpack window shows a textured tan/brown
body very similar to the BankFrame. Investigation found that the public
`Gethe/wow-ui-source` XML — both the pinned `12.0.5` vendor and the current
`live` branch — defines `ContainerFrame1` and `ContainerFrameCombinedBags` as
solid `PANEL_BACKGROUND_COLOR` panels with no body texture, so our simulator
matches the published source. The mismatch with retail therefore points at
something outside the public source: an addon overlay, a recent live-client
patch not yet mirrored, or a runtime path we haven't located. Closed without a
sim-side fix; reopen if the externally-applied texture path is identified.

## Symptom

- User screenshot: open `Backpack` window (combined-bags view, all bags grouped
  under #1 Backpack, #2-#5 Linen Bag, #6 Simply Stitched Reagent Bag) renders
  with a dark tan/brown tiled body, similar visual style to the bank panel.
- Simulator: same combined-bags / per-bag windows render with solid
  `PANEL_BACKGROUND_COLOR` (≈0.15, 0.15, 0.15) body and atlas-rounded bottom
  corners only.

## Investigation

### Render trace confirms the sim is rendering exactly what the XML asks

Adding a debug eprintln to `build_texture_quads` (worktree-only,
not committed) showed the `Bg.TopSection` and `Bg.BottomEdge` textures of
`ContainerFrame1` reaching render time with `color_texture =
Some(Color { r: 0.15, g: 0.15, b: 0.15, a: 1.0 })` and emitting solid quads
covering the panel body. So:

1. XML codegen emits `tex:SetColorTexture(c:GetRGBA())` for those textures.
2. The Lua call lands in `set_color_texture` with the right RGBA.
3. `color_texture` survives to render and a solid quad is pushed.
4. The visible gray *is* the rendered solid color, not a missing texture.

### What the XML actually defines

`ContainerFrameTemplate` and `ContainerFrameCombinedBags` both inherit
`PortraitFrameFlatTemplate` →
`PortraitFrameFlatBaseTemplate` →
`FlatPanelBackgroundTemplate`. That last one
(`Blizzard_SharedXML/Mainline/SharedUIPanelTemplates.xml:404`) is structured as:

- `BottomLeft` / `BottomRight`: atlas
  `uiframebackground-nineslice-cornerbottomleft`/`-cornerbottomright`, vertex
  color `PANEL_BACKGROUND_COLOR` — rendered as small rounded-corner sprites.
- `BottomEdge` / `TopSection`: no atlas/file, `<Color
  color="PANEL_BACKGROUND_COLOR"/>` — rendered as solid color quads via
  `SetColorTexture`.

There are no `edgetop`/`edgeleft`/`edgeright`/`cornertopleft`/`cornertopright`
or `center` atlas pieces under the `uiframebackground-nineslice-` prefix in the
listfile, so the `nineslice` naming is misleading — only the two bottom corners
exist as atlases. The rest of the panel is intentionally a flat color fill.

### Lua side does not add a body texture either

- `ContainerFrameMixin:GetBackgroundColor()` returns
  `PANEL_BACKGROUND_COLOR`.
- `ContainerFrameMixin:UpdateBackground()` calls `SetBackgroundColor` with
  that value, which is implemented by
  `PortraitFrameFlatBaseMixin:SetBackgroundColor` (in
  `Blizzard_SharedXML/PortraitFrame.lua:125`) — it tints the corner atlases
  via `SetVertexColor` and refreshes the solid-color edges via
  `SetColorTexture`. No atlas/file is applied.
- `ContainerFrameCombinedBagsMixin:UpdateBackground()` is a no-op (`-- nop, the
  background never changes`).
- `ItemSlotBackgroundCombinedBagsTemplate` *is* a textured template (file
  `Interface\ContainerFrame\UI-Bag-Components`, with explicit `<TexCoords>`),
  but it is instantiated only **per item button** in
  `ContainerFrameItemButtonMixin:Initialize` and `SetAllPoints(self)` is called
  on the *button*, not the panel. It contributes the per-slot tile look, not a
  panel-wide tiled body.

### Live-branch verification

WebFetch against
`raw.githubusercontent.com/Gethe/wow-ui-source/live/.../ContainerFrame.xml` and
`...ContainerFrame.lua` confirmed the same structure on `live` HEAD: no
`Background`/`Bg` texture on `ContainerFrameCombinedBags`, no atlas
applied via Lua to the panel body. Codex (gpt-5.5, high reasoning) reached the
same conclusion citing the live branch line ranges:
`SharedUIPanelTemplates.xml:404-435`,
`ContainerFrame.xml:218-283`,
`BankFrame.xml:673-684`.

### Bank vs Backpack are genuinely different

`BankFrame` defines its own body fill with
`<Texture parentKey="Background" atlas="bank-frame-background" horizTile="true"
vertTile="true">` (`BankFrame.xml:677`). The atlas resolves in the simulator to
`Interface\bankframe\bankframebackground` 256×256. So the bank gets a tiled
tan/marble body; the bag does not — by published-XML design.

## Conclusion

Per the public Blizzard source, the simulator's render matches what is
authored: solid `PANEL_BACKGROUND_COLOR` body for both `ContainerFrame1` and
`ContainerFrameCombinedBags`, atlas only for the two bottom corners.

The user's retail screenshot nonetheless shows a textured body. Three
candidate explanations remain unverified:

1. An addon (e.g. BetterBags, Bagnon, ElvUI) reskinning the bag UI.
2. A retail-client patch that adds a body texture but has not yet propagated
   to `Gethe/wow-ui-source`.
3. A code path in `Blizzard_Settings` / `Blizzard_EditMode` / a textureKit
   system that swaps in a tiled background under some preset, which we have
   not located.

Closing the investigation without a simulator change. Reopen if/when the
externally-applied texture path is identified — at that point the fix is to
add the corresponding `<Texture>` (or runtime atlas application) to the
simulator's container frame.

## Sources

- `vendor/wow-ui-source/Interface/AddOns/Blizzard_UIPanels_Game/Mainline/ContainerFrame.xml`
- `vendor/wow-ui-source/Interface/AddOns/Blizzard_UIPanels_Game/Mainline/ContainerFrame.lua`
- `vendor/wow-ui-source/Interface/AddOns/Blizzard_SharedXML/Mainline/SharedUIPanelTemplates.xml:404,618,643`
- `vendor/wow-ui-source/Interface/AddOns/Blizzard_SharedXML/PortraitFrame.lua:123-135`
- `vendor/wow-ui-source/Interface/AddOns/Blizzard_UIPanels_Game/Mainline/BankFrame.xml:673-684`
- `Gethe/wow-ui-source` `live` branch (verified via WebFetch) and Codex citations

## See Also

- [[hero-spec-dialog-anchors]] — another `PortraitFrameFlatTemplate`-derived
  panel; useful for understanding how the Bg/NineSlice layers are wired.

## Mists Note

The Mists/Classic container path is different from the retail flat-panel path
above. `ContainerFrame_GenerateFrame` selects
`Interface\ContainerFrame\UI-BackpackBackground` for bag ID 0, and
`ItemButtonTemplate` still gives every bag item button a `UI-Quickslot2`
`NormalTexture`. Do not clear those normal textures in a post-load shim: doing
so leaves only the baked backpack background wells and makes the main backpack
slot chrome diverge from Blizzard's authored item-button template.
