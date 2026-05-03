# LFD Role Icon Slowness

LFD role selection uses button atlas crops from `Interface\lfgframe\uilfgprompts`; the visible pause was not `GetIconForRole` lookup, but first-time crop materialization decoding the full 2048x2048 BLP before extracting small role-icon regions.

## Content

Blizzard's LFD role buttons call `SetNormalAtlas(GetIconForRole(...))` and `SetDisabledAtlas(GetIconForRole(...))` through `LFGRoleButtonTemplate_OnLoad`. The atlas entries for tank, healer, DPS, disabled variants, backgrounds, ready marks, and related LFG prompts all share `Interface\lfgframe\uilfgprompts`.

The simulator already rewrites atlas sub-regions into `@crop:` texture requests so the GPU atlas uploads isolated cropped regions instead of sampling neighboring atlas content. Before this fix, `TextureManager::load_sub_region` still had to load and decode the full source texture on every fresh process before it could extract a crop. A focused `cache-texture Interface\lfgframe\uilfgprompts` run on the cached source BLP still took about 5-6 seconds, which matches a large CPU decode cost rather than Lua role-state work.

The fix adds a persistent crop cache under the user cache directory. Exact sub-region keys are stored as small PNGs after first extraction; later identical crop requests can load the cached crop directly into `sub_cache` without resolving or decoding the base BLP. This preserves the existing `@crop:` render path and avoids changing Blizzard Lua or role state.

## Sources

- [LFDFrame.xml](../../../Interface/BlizzardUI/Blizzard_GroupFinder/Mainline/LFDFrame.xml) - LFD role button templates and role button declarations
- [LFGFrame.lua](../../../Interface/BlizzardUI/Blizzard_GroupFinder/Shared/LFGFrame.lua) - `LFGRoleButtonTemplate_OnLoad` atlas setters
- [TextureUtil.lua](../../../Interface/BlizzardUI/Blizzard_SharedXMLBase/TextureUtil.lua) - `GetIconForRole` and `GetBackgroundForRole` atlas names
- [resolve.rs](../../../src/texture/resolve.rs) - crop extraction and persistent crop cache
- [quad_builders_button.rs](../../../src/iced_app/quad_builders_button.rs) - button atlas crop request emission

## See Also

- [[lfd-dungeon-list-empty]] - related LFD panel behavior investigation
- [[micro-menu-atlas-revert]] - button atlas crop path for restored normal textures
- [[texture-atlas]] - texture manager and atlas rendering system
