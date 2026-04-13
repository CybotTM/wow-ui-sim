# Transparent Wrapper Render Order

Transparent `Frame` / `ScrollFrame` wrappers were creating fake frame-level boundaries in `build_strata_buckets()`. That buried descendant regions under decorative sibling frames in quest-log/world-map layouts, even when the wrappers themselves rendered nothing. The final fix hoists transparent-wrapper regions through those wrappers, but keeps track of wrapper depth so a wrapper's own background regions still render before deeper descendant content like world-map tiles.

## Symptoms

- World map / quest log content could appear underneath decorative border art even though the visible content should sit above it.
- The existing render-order regression reproduced this with a content icon inside one wrapper frame and a high-level decorative border inside another wrapper frame.
- The loaded Blizzard world map still needed to keep the actual map visible while real world quest pins stayed above the map tiles.

## Root Cause

`src/lua_api/state_render.rs` built strata buckets with a depth-first walk:

1. emit the parent frame
2. emit the parent's direct regions
3. recurse into child frames
4. emit deferred font strings

That behavior is fine for real render boundaries, but it treated every child `Frame` as a meaningful z-order boundary. Generic wrappers such as:

- quest log content holders
- border helper frames
- POI display subframes
- scroll child wrappers

often have no backdrop, no nine-slice, and no quest blob quads of their own. They only exist to group anchors or hold regions. When those wrappers stayed in the DFS tree as hard boundaries, their descendant textures were emitted too late relative to decorative siblings.

The first hoist fix was too aggressive: it flattened every descendant region into one shared region list. That fixed the border/pin ordering bug, but it let `WorldMapFrame.ScrollContainer.Child.TiledBackground` sort after deeper world-map tile textures, which made the map go black.

## Fix

`build_strata_buckets()` now tracks quest-blob owners and `dfs_emit()` treats renderless `Frame` / `ScrollFrame` children as transparent containers:

- keep the wrapper frame ID in the bucket
- hoist descendant regions into the current frame's region ordering
- record how many transparent-wrapper levels a hoisted region crossed
- recurse only into non-transparent child frames

The region sort now uses wrapper depth before draw-layer ordering. That preserves the useful flattening for peer transparent wrappers, but keeps a wrapper's own regions ahead of deeper descendants. In practice:

- `TestBorderTex` and `TestIcon` are both depth-2 hoisted regions, so they still sort together and the icon stays above the decorative border.
- `TiledBackground` is shallower than the real world-map tile textures, so the map art stays visible.

This preserves hit-testing/bookkeeping while removing the false render boundary without flattening away the world map's background/content relationship.

## Verification

- `high_level_border_does_not_cover_lower_level_content` now passes.
- Added `world_map_tiles_render_after_tiled_background` to prove the world map remains visible.
- Added `world_quest_pin_icon_renders_after_world_map_tiles` to prove the fix does not push real world quest pins behind map art.
- Button-state rendering still passes after the bucket-builder change.

## Sources

- [src/lua_api/state_render.rs](../../../src/lua_api/state_render.rs) — strata bucket construction and transparent-wrapper fix
- [tests/render_order.rs](../../../tests/render_order.rs) — generic border/content regression and world-map pin regression
- [QuestMapFrame.xml](../../../Interface/BlizzardUI/Blizzard_UIPanels_Game/Mainline/QuestMapFrame.xml) — `QuestLogBorderFrameTemplate` uses a high-level decorative border wrapper
- [WorldQuestDataProvider.xml](../../../Interface/BlizzardUI/Blizzard_SharedMapDataProviders/WorldQuestDataProvider.xml) — world quest pins use wrapper subframes like `Display` / `TimeLowFrame`

## See Also

- [[rendering-pipeline]] — strata buckets feed the quad emission path
- [[action-bar-spell-icons]] — earlier render-order work on draw-layer/sublevel sorting
