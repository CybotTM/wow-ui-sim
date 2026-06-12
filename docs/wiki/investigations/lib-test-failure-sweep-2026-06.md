# Lib test failure sweep (2026-06-12)

Nine `cargo test --lib` failures had accumulated unnoticed across several weeks
of parallel work. Each had a distinct root cause; none were caught when
introduced because the full lib suite wasn't run per-commit. Fixed across
commits `085eda2f5..fae7085ef`.

## Root causes

### 1. Runtime foundation order missed Blizzard_ScriptErrors (2 tests)

`Blizzard_SharedXMLBase.toc` declares `## Dependencies: Blizzard_ScriptErrors`.
ScriptErrors wasn't in `GAME_RUNTIME_FOUNDATIONS` (`src/c_api/c_addons.rs`), so
a runtime `LoadAddOn` walked: SharedXMLBase → dep ScriptErrors → ScriptErrors'
*own* foundation chain (everything before it in the list) → Blizzard_SharedXML
— all while SharedXMLBase sat in the cycle-guard `loading` set with **none of
its files executed**. SharedXML then died on nil `FlagsUtil`/`MathUtil`, and
everything downstream (`EventRegistry:RegisterFrameEvent`,
`AuraUtil.GetDebuffDisplayInfoTable`) cascaded.

Fix: ScriptErrors is now first in `GAME_RUNTIME_FOUNDATIONS`.
Debugging tool: `WOW_SIM_TRACE_LOAD_ADDON=1` prints the recursive load walk.

### 2. hooksecurefunc(C_AddOns, "LoadAddOn") is silently refused (2 tests)

Since `44ed0b078` (2026-05-28), the shared-bootstrap `hooksecurefunc` fallback
**no-ops** for the `C_AddOns.LoadAddOn` target. Any workaround using that hook
to run "after addon X loads" logic never installs — silently.

- `dispatcher_surface` and `achievement_search_preview` were rewired through
  `apply_blizzard_post_load_patches` (`src/loader/addon.rs`), which fires for
  both startup and runtime loads and is the canonical per-addon post-load hook.
- The third dead hook — `runtime_surface_bootstrap.lua` for
  `Blizzard_CharacterSelectNavBar`, UIParent worklists, and MapCanvas/FogOfWar
  pins — was rewired in `fa3c18300`: the `__wow_patch_*` functions are exposed
  as globals and fired from `patch_runtime_surface_for_addon_load`. That commit
  also fixed the CreateFrame wrapper in `frame_helper_defaults.rs`, which
  called `__wow_patch_map_canvas_scroll_container_methods` by global name
  while it was still chunk-local (nil). A guard test asserts the four globals
  stay exposed.

Rule of thumb: never use `hooksecurefunc(C_AddOns, "LoadAddOn", ...)` in
simulator-owned Lua; add an arm to `apply_blizzard_post_load_patches` instead.

### 3. Code lost in the classic-profile rebase (2 tests)

Second and third instances of work dropped by the 2026-05-19 master rebase
(`save/classic-profile-rollout-before-master-rebase-20260519` holds the
originals):

- The Lua-side custom-OnUpdate guard in `__wow_mark_layout_frame_dirty`
  (originally `b08b68971`) — restored in `546f307d5`.
- `process_element_with_exclusive_timing` (originally `3d14dff8e`), without
  which `xml_process_time` double-counts `<Script>` Lua time — restored in
  `7c691bcca`.

When a test that asserts a *subtraction/guard* behavior fails after a rebase,
diff the pre-rebase save branch before assuming the test is stale.

### 4. Duplicated `== nil`-guarded Lua installers drifting (2 tests)

`AddDataProvider` had three competing installers (per-frame in
`frame_helper_defaults.rs`, frameIndex in the same file, frameIndex in
`shared_bootstrap.lua`); only the per-frame one ran `provider:OnAdded` and
seeded `provider.pin`, and a bare frameIndex variant won the guard. Same
pattern with `__wow_modified_clicks`: the runtime bootstrap seeds retail
defaults (`SELFCAST = "ALT"`), the workaround installer seeded an empty table.
Both sets are now aligned and cross-referenced in comments.

### 5. Tests stale against deliberate semantic changes (3 tests)

- `8f4f885c3` stopped copying Lua table fields onto recreated named frames
  (matches real-client probe); two `global_frame_access` tests asserted the
  old copying behavior.
- `157d744ff` made `IsRectValid` resolve dirty anchored rects on demand and
  updated the equivalent assertion in `layout_size.rs` but missed the
  identical one in `layout_anchoring.rs`.

## Related

- [frame-surrogate-identity-slot](frame-surrogate-identity-slot.md) — fresh
  identity on duplicate-name CreateFrame.
- [class-talents-edge-lines](class-talents-edge-lines.md) — the IsRectValid
  change shipped with this work.
