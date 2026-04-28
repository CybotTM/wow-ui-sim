# Three-Slice Button Tiling

Escape menu red buttons showed vertical bands because the center atlas art is
not horizontally seamless. Stretching the full center strip made broad bands;
repeating the full strip made seams; an attempted aspect-ratio correction made
dense 18px stripes.

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
`82x36`, and Center to about `85x36`. The Center source art is not seamless
across its 64px width, so full-strip repeats expose a vertical seam. Repeating
it every `18px` made the same source variation much denser.

Fix: red-button Center atlases use a seam-safe center strip as the repeated
source region. This avoids using the non-seamless 64px source span.

## Sources

- [ThreeSliceButtonTemplate.lua](../../Interface/BlizzardUI/Blizzard_SharedXML/Shared/Button/ThreeSliceButtonTemplate.lua) — Blizzard mixin scale and atlas setup
- [ThreeSliceButtonTemplate.xml](../../Interface/BlizzardUI/Blizzard_SharedXML/Shared/Button/ThreeSliceButtonTemplate.xml) — Center texture `horizTile=true`
- [data/atlas.rs](../../data/atlas.rs) — RedButton atlas dimensions and tiling metadata
- [tiling.rs](../../src/iced_app/tiling.rs) — simulator tile-size computation and regression test

## See Also

- [[texture-atlas]] — atlas metadata and UV remapping
- [[rendering-pipeline]] — quad emission and texture request flow
