# World Map Fog Of War First-Open Size

The world-map fog bug on first open was not a missing tile upload. The map art
tiles were present, but the fog pin could keep a stale size until a later scale
change callback.

## Symptom

On the first visible world-map frame, the fog-of-war overlay could cover only a
fraction of the map instead of matching the full canvas. The map tiles
underneath were still visible, which made the issue look like a bad fog mask or
quarter-map fog rectangle rather than a missing tile.

## Root Cause

`FogOfWarPinMixin` in Blizzard UI only resizes itself in
`OnCanvasScaleChanged()`:

- `MapExplorationPinMixin`, `QuestBlobPinMixin`, `ScenarioBlobPinMixin`, and
  other full-canvas map pins implement `OnCanvasSizeChanged()` and resize there
- `FogOfWarPinMixin` did not
- in the simulator, the first visible world-map frame can hit
  `OnCanvasSizeChanged()` before any `OnCanvasScaleChanged()` callback
- when that happened, the fog pin kept its previous/stale size instead of
  resizing to `DenormalizeHorizontalSize(1.0)` /
  `DenormalizeVerticalSize(1.0)`

That left the fog overlay geometry out of sync with the actual map canvas on
first open.

## Fix

Added a targeted Lua workaround in `src/lua_api/workarounds.rs`:

- patch `FogOfWarPinMixin` to resize in `OnCanvasSizeChanged()`
- keep `OnCanvasScaleChanged()` resizing behavior intact
- patch already-created `FogOfWarPinTemplate` instances after startup so the
  fix applies even when the pin was created before the workaround runs

This keeps the fix local to simulator behavior without modifying Blizzard UI
source files.

## Verification

- `cargo test --test test_keybindings_panels_detail world_map_fog_of_war_pin_resizes_on_canvas_size_changed -- --nocapture`
- `cargo test --test test_keybindings_panels_detail world_map_fog_of_war_pin_matches_canvas_size_on_first_open -- --nocapture`

## Sources

- [src/lua_api/workarounds.rs](../../../src/lua_api/workarounds.rs) — runtime
  fog-pin size patch
- [tests/test_keybindings_panels_detail.rs](../../../tests/test_keybindings_panels_detail.rs) —
  first-open fog-pin sizing regression coverage
- [Interface/BlizzardUI/Blizzard_SharedMapDataProviders/FogOfWarDataProvider.lua](../../../Interface/BlizzardUI/Blizzard_SharedMapDataProviders/FogOfWarDataProvider.lua) —
  upstream fog pin only resized on canvas scale changes
- [Interface/BlizzardUI/Blizzard_SharedMapDataProviders/MapExplorationDataProvider.lua](../../../Interface/BlizzardUI/Blizzard_SharedMapDataProviders/MapExplorationDataProvider.lua) —
  reference pattern for full-canvas pins resizing on canvas size changes

## See Also

- [[world-map-fog-of-war-overlay-model]] — later fix for the separate explored
  chunk versus fog-geometry mismatch
- [[world-map-texture-loading-budget]] — separate world-map preload/upload work
  that affected first-open responsiveness
