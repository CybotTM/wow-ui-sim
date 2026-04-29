# Dialog Background DXT3 Stripes

Escape-menu and dialog backgrounds showed vertical stripe artifacts when rendered from `Interface\DialogFrame\UI-DialogBox-Background`. The texture is visually flat, but its BLP content is DXT3; the simulator incorrectly treated DXT3 as BC3 on the raw GPU-compressed upload path, so the alpha block layout was decoded as the wrong compression format.

## Content

The debugging split was:

- Hiding `GameMenuFrame.Border.Bg` removed the stripes.
- Hiding buttons, text, and metal border pieces left the stripes visible, proving the artifact was in `Bg`.
- `SetColorTexture(0,0,0,0.6)` on the same frame removed the stripes, proving the issue was textured rendering rather than alpha blending or background bleed-through.
- Forcing the texture through RGBA decode removed the stripes, while the normal texture path produced them.

Root cause: `BlpContent::Dxt3` was grouped with `Dxt5` and mapped to `BcTextureFormat::Bc3`. DXT3 is BC2, not BC3. Until a BC2 atlas exists, DXT3 must fall back through the RGBA decode path.

Fix: `bc_texture_data()` now rejects DXT3 for raw BC upload and preserves DXT5 as BC3. The regression test `bc_texture_result_rejects_dxt3_without_bc2_support` prevents DXT3 from being reintroduced as BC3.

## Sources

- [src/texture.rs](../../../src/texture.rs) — BLP compressed-format selection and regression tests.
- [DialogTemplates.xml](../../../Interface/BlizzardUI/Blizzard_SharedXML/Shared/Dialog/DialogTemplates.xml) — `DialogBorderTemplate` uses `UI-DialogBox-Background` for `Border.Bg`.

## See Also

- [[texture-atlas]] — texture loading, BLP decoding, and GPU atlas paths.
- [[three-slice-button-tiling]] — earlier escape-menu stripe investigation that ruled out button highlights as the remaining background artifact.
