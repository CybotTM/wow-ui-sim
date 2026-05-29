# Mists World Map Startup

Mists addon startup can fail on third-party WorldMap users when the Mists-specific Blizzard WorldMap cache files are absent or when Cata/Mists map providers inherit incomplete simulator surfaces. The fix is to load the real Mists `Blizzard_WorldMap` / `Blizzard_SharedMapDataProviders` files, keep the unsafe Mists `Blizzard_UIParentPanelManager` exclusion in place, and provide narrow Mists compatibility surfaces for the map calls that otherwise depended on that panel manager.

## Content

The visible failures were:

- `WorldMapFrame` / `WorldMapTrackQuest` nil errors from addons such as AllTheThings and Leatrix_Maps.
- `UpdateUIPanelPositions` nil from `Blizzard_WorldMap/Cata/Blizzard_WorldMap.lua`.
- `FogOfWarFrameMixin` nil while Leatrix loaded `Blizzard_BattlefieldMap`.
- `MapCanvasMixin:AddDataProvider()` calling `dataProvider:OnAdded()` on Mists providers that lacked copied base callbacks.

Root causes:

- The runtime cache had only the retail/Mainline WorldMap TOC, while Mists discovery skips `_Mainline` TOCs. The Mists profile needs `Blizzard_WorldMap_Mists.toc`, `Cata/Blizzard_WorldMap.*`, `Wrath/QuestLogOwnerMixin.lua`, `Blizzard_WorldMapTooltip.xml`, and the Mists `Blizzard_SharedMapDataProviders` TOC/files.
- Loading the real Mists `Blizzard_UIParentPanelManager` still reintroduces the `bottomEdgeExtent` layout failure, so WorldMap cannot depend on fully enabling that addon yet.
- Cata WorldMap calls panel-position globals (`UpdateUIPanelPositions`, `MaximizeUIPanel`, `RestoreUIPanelArea`) and expects `WorldMapLevelDropDown.header`.
- Mists shared map providers reference `FogOfWarFrameMixin`, but the Mists source tree does not ship a matching `FogOfWarFrameTemplates` implementation.
- `Blizzard_MapCanvas.lua` is copied into XML templates before post-load wrappers can affect it, so provider default callback backfills must happen in a source patch on `MapCanvasMixin:AddDataProvider()`.

Implementation notes:

- Cache membership is guarded in `src/blizzard_ui_sync/profile_cache.rs` so stale retail files do not satisfy the Mists profile.
- `src/mists/post_load.lua` owns Mists-only panel-position, FogOfWar, and WorldMap dropdown header compatibility.
- `src/lua_api/workarounds/temporary/source_patches.rs` owns the Mists `Blizzard_MapCanvas.lua` source patch that backfills provider callbacks before `OnAdded`.
- The final startup baseline after this fix kept only the unrelated Syndicator `tradeskill` and Baganator `TokenFramePopup` errors.

## Sources

- [data/blizzard-ui-files.txt](../../../data/blizzard-ui-files.txt) — Mists WorldMap and SharedMapDataProviders cache manifest entries.
- [profile_cache.rs](../../../src/blizzard_ui_sync/profile_cache.rs) — Mists cache-entry allowlist and usability checks.
- [post_load.lua](../../../src/mists/post_load.lua) — Mists-only compatibility surfaces.
- [source_patches.rs](../../../src/lua_api/workarounds/temporary/source_patches.rs) — MapCanvas provider callback source patch.

## See Also

- [[client-profiles]] — profile-specific TOC and Blizzard UI cache behavior.
- [[addon-loading]] — Blizzard addon discovery and runtime loading.
- [[world-map-fog-of-war-overlay-model]] — related WorldMap/FogOfWar data-provider behavior.
