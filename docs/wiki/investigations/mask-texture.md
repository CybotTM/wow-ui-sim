# Mask Texture

WoW uses MaskTextures to clip child textures to shapes (rounded squares, circles, etc.) using the mask's alpha channel.

## How It Works

1. `MaskTexture` is created with `CreateMaskTexture()` or XML `<MaskTexture>`
2. `<MaskedTextures>` block calls `icon:AddMaskTexture(mask)` on each referenced child
3. During rendering, the masked texture's quads carry `mask_tex_index` and `mask_tex_coords` vertex attributes
4. Fragment shader multiplies output color by `mask_color.a` — where mask alpha=0, the pixel is fully transparent

## UV Computation

The mask UV maps the icon's screen position into the mask's screen area. Critical: the mask should be **larger** than the icon it clips, so the icon samples only the opaque center of the mask texture.

For a 64×64 mask centered on a 45×45 icon:
- UV range: `(9.5/64, 54.5/64)` = `(0.148, 0.852)` on both axes
- Icon samples only the center 70% of the mask texture

If mask size is 0×0 (broken), the full mask (0–1 UV) maps to the icon, clipping visible area at transparent borders.

## Key Behavior: `useAtlasSize` Default

MaskTextures default to `useAtlasSize=true` when not specified in XML. Without this, the mask frame is 0×0 and the full mask texture (including transparent borders) maps to the icon, shrinking the visible area.

## `SmallActionButtonMixin` Override

For 30×30 small buttons, `SmallActionButtonMixin_OnLoad` explicitly sets IconMask to 45×45. This overrides the atlas-derived 64×64 size, giving similar proportional UV coverage (0.167–0.833).

## Action Bar Icon Chain

1. Icon (45×45, fills button) → masked by IconMask (64×64 rounded square atlas)
2. SlotBackground (dark fill visible at rounded corners)
3. SlotArt (decorative golden border)
4. NormalTexture (frame border, OVERLAY layer)

## Wrap Modes

`CLAMPTOBLACKADDITIVE` on a MaskTexture means areas outside the atlas are fully transparent — important for the sheen animation's mask.

## Key Files

- `src/iced_app/masking.rs` — mask UV computation
- `src/render/shader/quad.wgsl:148-151` — fragment shader mask sampling
- `src/loader/xml_texture.rs` — XML MaskTexture creation

## Sources

- [mask-texture-system.md](../../mask-texture-system.md) — full system description

## See Also

- [[action-bar-spell-icons]] — concrete use of IconMask on action buttons
- [[talent-sheen]] — sheen animation uses MaskTexture for button shape clipping
