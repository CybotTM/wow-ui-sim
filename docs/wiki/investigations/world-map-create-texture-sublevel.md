# World Map CreateTexture Sublevel Churn

After the steady-state `SetFrameLevel()` rebuild loop was fixed, the world map still rebuilt strata buckets during open because many map textures were created at the wrong draw sublevel and then immediately corrected with `SetDrawLayer()`. The simulator implemented `CreateTexture(name, layer, inherits, subLevel)` but ignored the `subLevel` argument, so pooled world-map textures started at sublevel `0` instead of their requested ordering.

## Symptoms

- World-map open still showed a short burst of bucket rebuilds even after the no-op `SetDrawLayer()` guard was added.
- `WOW_SIM_TRACE_STRATA_INVALIDATIONS=1` showed paired invalidations from:
  - `src/lua_api/frame/methods/methods_create.rs`
  - `src/lua_api/frame/methods/methods_texture_visual.rs`
- In the post-open window of the reduced repro (`ToggleWorldMap()`, `--no-addons`, `WOW_SIM_NO_SAVED_VARS=1`), the trace still contained `150` `SetDrawLayer()` invalidations before the `CreateTexture()` fix.

## Root Cause

Blizzard's pool helpers create textures with both a layer and a sublevel:

- `CreateSecureTexturePoolInstance()` in `Pools.lua` calls `parent:CreateTexture(name, layer, template, subLayer)`.

The simulator only applied:

1. the texture name,
2. the draw layer,
3. the inherited template size.

It dropped the fourth `subLevel` argument completely. That mattered on the world map because pooled map textures are often assigned their final ordering at creation time or are expected to start close to it. When they instead started at sublevel `0`, Blizzard code immediately repaired them with `SetDrawLayer()`, which forced extra `strata_buckets` invalidations during map population.

## Fix

- `CreateTexture()` now parses and applies the fourth `subLevel` argument to `draw_sub_layer`.
- The local `CreateTexturePool()` stub now forwards both `template` and `subLayer` to `CreateTexture()` instead of dropping them.
- The earlier `SetDrawLayer()` no-op guard remains in place, so same-value repair calls no longer invalidate buckets.

## Result

The post-open traced repro stopped showing `SetDrawLayer()` invalidations entirely in the `>=63s` world-map-open window:

- before `CreateTexture(..., subLevel)` fix: `150` `methods_texture_visual.rs` invalidations
- after fix: `0` `methods_texture_visual.rs` invalidations

What remained in that window was mostly genuine `CreateTexture()` invalidation plus show-path repair. So the "create wrong, then fix immediately" ordering churn is gone, but the map still does substantial real creation work during open.

## Verification

- Added `test_create_texture_applies_sublevel_argument` in `tests/c_map_api.rs`.
- Added `same_draw_layer_preserves_cached_strata_buckets` in `tests/render_order.rs`.
- `late_set_draw_layer_invalidates_cached_strata_buckets` still passed, confirming real draw-layer changes still rebuild ordering.
- Runtime repro with `WOW_SIM_TRACE_STRATA_INVALIDATIONS=1` confirmed post-open `SetDrawLayer()` invalidations dropped from `150` to `0`.

## Sources

- [src/lua_api/frame/methods/methods_create.rs](../../../src/lua_api/frame/methods/methods_create.rs) — `CreateTexture()` now applies `subLevel`
- [src/lua_api/frame/methods/methods_texture_visual.rs](../../../src/lua_api/frame/methods/methods_texture_visual.rs) — no-op `SetDrawLayer()` guard
- [src/lua_api/globals/utility_stubs.rs](../../../src/lua_api/globals/utility_stubs.rs) — local `CreateTexturePool()` stub now forwards `template` and `subLayer`
- [tests/c_map_api.rs](../../../tests/c_map_api.rs) — regression for `CreateTexture(..., subLevel)`
- [tests/render_order.rs](../../../tests/render_order.rs) — cached-bucket no-op `SetDrawLayer()` regression
- [Interface/BlizzardUI/Blizzard_SharedXMLBase/Pools.lua](../../../Interface/BlizzardUI/Blizzard_SharedXMLBase/Pools.lua) — texture pool creation passes `subLayer`

## See Also

- [[world-map-frame-level-rebuilds]] — earlier world-map rebuild investigation
- [[world-map-texture-loading-budget]] — follow-up world-map open cost after rebuild churn was reduced
- [[action-bar-spell-icons]] — earlier draw-layer/sublevel correctness bugs in the renderer
