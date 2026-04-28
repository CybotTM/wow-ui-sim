# Three-Slice Button Tiling

Escape menu red buttons still showed vertical bands after atlas center textures
were tiled instead of stretched. The remaining mismatch was tile scale: the
simulator repeated `_128-RedButton-Center` at its raw 64px atlas width even
though the 128px-tall source strip was drawn into a 36px-tall button, so the
center tile was still horizontally stretched relative to its vertical scale.

## Content

`ThreeSliceButtonMixin` sets Left and Right with `useAtlasSize=true`, scales them
from `buttonHeight / leftAtlasInfo.height`, and sets Center with
`SetAtlas(self:GetCenterAtlasName())`. The Center texture is anchored between
the scaled Left and Right textures and has `horizTile=true`.

For `BigRedThreeSliceButtonTemplate`, the pieces are:

- `128-RedButton-Left`: `114x128`, not tiled
- `_128-RedButton-Center`: `64x128`, `tiles_horizontally=true`
- `128-RedButton-Right`: `292x128`, not tiled

In a 200x36 GameMenu button, Left resolves to about `32x36`, Right to about
`82x36`, and Center to about `85x36`. A non-stretched horizontal repeat must
therefore use the same scale as the drawn vertical axis: `64 * 36 / 128 = 18`.
Repeating at 64px produced too few, too-wide center tiles.

Fix: atlas-backed single-axis tiling now preserves source aspect ratio from the
drawn orthogonal axis. Horizontal atlas tiling computes tile width from
`source_width * bounds.height / source_height`; vertical atlas tiling mirrors
that using destination width.

## Sources

- [ThreeSliceButtonTemplate.lua](../../Interface/BlizzardUI/Blizzard_SharedXML/Shared/Button/ThreeSliceButtonTemplate.lua) — Blizzard mixin scale and atlas setup
- [ThreeSliceButtonTemplate.xml](../../Interface/BlizzardUI/Blizzard_SharedXML/Shared/Button/ThreeSliceButtonTemplate.xml) — Center texture `horizTile=true`
- [data/atlas.rs](../../data/atlas.rs) — RedButton atlas dimensions and tiling metadata
- [tiling.rs](../../src/iced_app/tiling.rs) — simulator tile-size computation and regression test

## See Also

- [[texture-atlas]] — atlas metadata and UV remapping
- [[rendering-pipeline]] — quad emission and texture request flow
