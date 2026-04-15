# World Map Fog Of War Overlay Model

The first follow-up after the fog-pin sizing fix went in the wrong direction.
The simulator stopped hard-coding a half-map fog shape, but it still invented
fog coverage by deriving the complement of exploration overlays. That was still
not Blizzard's model, and it produced visible overlap between explored tiles and
the fake fog overlay on the current world map.

## Symptom

The current world map showed real irregular exploration tiles, but an additional
fog layer still darkened parts of the same map. Hiding the `FogOfWarPin`
changed rendered pixels even though Blizzard has no fog-of-war DB entry for the
current `UiMapID`.

## Correct Root Cause

Two simulator assumptions were wrong:

- `C_FogOfWar.GetFogOfWarForMap()` returned a fog ID for any map that had art
- `src/iced_app/quad_builders.rs` rendered fog by synthesizing geometry from
  exploration overlay gaps

That meant the simulator rendered a fog layer for maps that should not have any
fog at all. The current world map (`UiMapID 2248`) has no `UiMapFogOfWar`
record, so the correct behavior is for the fog pin to stay hidden and render
nothing.

## Fix

- Added `data/db2/UiMapFogOfWar.csv` and
  `data/db2/UiMapFogOfWarVisualization.csv`
- Changed `C_FogOfWar` to use DB-backed map-to-fog lookup instead of inventing a
  fog ID from map art presence
- Stopped rendering synthetic fog geometry from exploration overlays
- Kept the exploration APIs on the real irregular `WorldMapOverlay` data

The current map then exposed another simulator simplification: all default-visible
exploration overlays were treated as already explored, so Isle of Dorn showed as
fully discovered. Until per-character exploration state exists, the simulator now
leaves one real overlay chunk (`WorldMapOverlay.ID = 4885`, The Three Shields /
Skolzgal Mill) unexplored. That keeps the state data and the rendered overlay
tiles aligned without hardcoding a fake fog shape.

The simulator still does not render Blizzard's real fog background+mask model.
That is intentional here: removing the fake overlay is safer than preserving a
known-wrong hardcoded shape.

## Verification

- `cargo test --test c_map_api test_c_fog_of_war_returns_nil_for_current_map_without_fog_data -- --nocapture`
- `cargo test --test c_map_api test_get_explored_area_ids_leave_one_current_map_sub_zone_unexplored -- --nocapture`
- `cargo test --test test_keybindings_panels_detail world_map_current_map_keeps_fog_of_war_pin_hidden_without_fog_data -- --nocapture`
- `cargo test --test render_order isolated_world_map_current_map_does_not_render_fog_of_war_without_db_entry -- --nocapture`
- `cargo test --test test_keybindings_panels_detail world_map_exploration_pin_has_visible_overlay_textures_after_opening -- --nocapture`

## Sources

- [data/db2/UiMapFogOfWar.csv](../../../data/db2/UiMapFogOfWar.csv) — local fog
  presence data used by `C_FogOfWar`
- [src/lua_api/globals/c_map_api.rs](../../../src/lua_api/globals/c_map_api.rs)
  — DB-backed fog lookup and info table assembly
- [src/iced_app/quad_builders.rs](../../../src/iced_app/quad_builders.rs) —
  removal of synthetic fog geometry
- [src/map_exploration.rs](../../../src/map_exploration.rs) — exploration
  overlays still come from real irregular chunk data
- [tests/c_map_api.rs](../../../tests/c_map_api.rs) — fog API regression
- [tests/render_order.rs](../../../tests/render_order.rs) — render regression
- [tests/test_keybindings_panels_detail.rs](../../../tests/test_keybindings_panels_detail.rs)
  — live world-map pin visibility regression

## See Also

- [[world-map-fog-of-war-first-open-size]] — the separate first-open fog-pin
  sizing bug
- [[world-map-texture-loading-budget]] — earlier first-frame world-map artifact
  investigation
