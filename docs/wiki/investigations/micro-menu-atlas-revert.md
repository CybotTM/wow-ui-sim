# Micro Menu Atlas Revert

Micro menu icons could disappear after hover because button atlas setters restored the correct Lua texture state without preserving atlas sub-region metadata on the generated child texture.

## Content

Symptoms:

- Micro menu icons appeared on hover, then disappeared when the mouse left and Blizzard code restored the normal texture alpha.
- Live Lua state showed `GetNormalTexture():GetAlpha() == 1` and the normal texture child was shown, so the bug was below Lua hover state.
- Connected render dumps showed the normal child still had atlas UVs, but the child texture lacked `atlas_tex_coords`, so the texture builder could not convert the atlas sub-region into an isolated `@crop:` request.

Root cause:

`Button:SetNormalAtlas` / `SetPushedAtlas` / `SetDisabledAtlas` / `SetHighlightAtlas` use the button texture helper path, not the generic `Texture:SetAtlas` path. The button helper wrote `atlas`, `texture`, and `tex_coords` onto the generated child texture, but did not write `atlas_tex_coords`. Generic texture atlas rendering depends on `atlas_tex_coords` to detect that the texture is an atlas sub-region and remap it to a cropped request.

Fix:

- The rilua button atlas setter now writes `atlas_tex_coords = Some(tex_coords)` on generated child textures.
- The legacy mlua setter path was updated the same way.
- The early same-atlas short-circuit was removed so existing children missing the metadata are repaired on repeated atlas setter calls.
- Regression coverage in `cached_button_state_texture_restores_normal_after_hover` asserts both the child metadata and the restored normal texture `@crop:` request.

## Sources

- [textures.rs](../../src/lua_api/frame/methods/button_anchor_hierarchy/textures.rs) — active button atlas setter path
- [methods_button_texture.rs](../../src/lua_api/frame/methods/methods_button_texture.rs) — legacy button atlas setter path
- [render.rs](../../src/iced_app/render.rs) — regression test for hover leave and restored normal texture crop requests

## See Also

- [[texture-atlas]] — atlas sub-region and crop request behavior
- [[rendering-pipeline]] — quad batches and deferred texture requests
