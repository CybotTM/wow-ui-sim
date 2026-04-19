# Wiki Log

Chronological record of wiki operations.

## [2026-04-19] ingest | partyframe status-bar texture drop

Added `investigations/partyframe-statusbar-textures.md` to capture the root cause for the party health/mana bar `MISSING` render. The XML loader creates the bar child correctly, but `SetStatusBarTexture(bar)` passes a userdata frame into a setter that only accepts strings/numbers, so the status-bar source is cleared. Updated `index.md` with the new page.

## [2026-04-18] update | Track 2 metatable handle threading

Updated `investigations/track-2-intern-audit.md` with the current
handle-threading progress: a shared `hot_metatable_key(...)` accessor
now reuses the prewarmed registry handle for `__index` / `__newindex`
lookups in `methods.rs`, `globals/create_frame/helpers.rs`,
`globals/security.rs`, and `env_init/freeze_globals.rs`, with a static
fallback for bootstrap-skipping tests.

## [2026-04-17] ingest | rilua vs mlua gap audit

Added `investigations/rilua-mlua-gap-audit.md` after comparing the current
rilua registration path against `master`'s mlua-era Lua API surface. Recorded
the highest-signal missing handling buckets: no-op sandbox cleanup in
`env_init`, dropped `MessageFrame` method registration, unfinished
attribute/event/text widget parity, and the unwired `patch_namespace_stubs()`
runtime hook. Updated `index.md` with the new investigation page.

## [2026-04-17] update | startup XML loader fast-path follow-up

Updated `investigations/startup-createframe-profile.md` with the current XML
loader fast-path state after the runtime `CreateFrame` work. Recorded the
safe widening steps in `xml_frame/setup.rs` and the split
`template_chain/` machinery, the current clean `xml fast path` counters
(`hits=1868`, `slow=350`, `scripts=234`), the shared-worktree debug startup
range (~`4.8s`-`5.8s` on `--no-addons --no-saved-vars`), and the failed
generic global-method literal-arg experiment that still regresses real
startup. Refreshed the `index.md` summary for the page.

## [2026-04-16] update | intern_string perf re-profile correction

Corrected the earlier PLAN/wiki summary for the post-migration interning
profile. Fresh release `perf` on `wow-sim --no-saved-vars lua-errors` shows
`Gc::intern_string` at 179.5M cycles (2.98%), `StringTable::intern_hashed`
at 169.2M (2.81%), and inline `lua_hash` at only ~4.8M (0.08%). The original
"lua_hash essentially flat" note was wrong; the hash primitive is no longer
the bottleneck, and the remaining cost sits in bucket traversal / dedup work.

## [2026-04-16] update | intern_string_static mid-cycle fix + migration landed

Found the root cause of the earlier migration breakage. `intern_string_static`
inserts into `static_intern_cache`, but `mark_gc_roots` only scans that cache
at cycle start — a mid-Propagate insert is still pre-flip current-white, gets
swept at cycle end, and the cache ends up pointing at a freed slot. Fixed in
rilua by colouring the new ref Black during Propagate/Sweep/Finalize; added
`intern_string_static_mid_cycle_survives_sweep` regression test.

Applied the migration for `registry_get/set/table_or_create`, `registry_table`,
`attach_frame_metatable`, and fan-out callers — intern counter 1,250,287 →
1,096,266 (−12%), release startup 1.18s → 1.15s median (n=5). `frame_ref_cache`
(the biggest single call site, 286K/startup) remains deferred: even with the
GC fix, migrating that path cascades into "OnLoad (a nil value)" failures
for ~300 addons and the root cause isn't yet understood.

## [2026-04-16] ingest | intern_string call-site ranking

Added `investigations/intern-string-ranking.md` and the rilua
`intern-stats` feature. Startup runs 1.25M `intern_string` calls with
top 5 literals accounting for 40%. Attempted migrating the
registry/frame helpers to `intern_string_static` (counter dropped
−74.5%) but release triggered 22 new Lua errors — frames lose their
metatable methods when `registry_set` uses `intern_string_static`.
Reverted the migration; filed as a rilua follow-up.

## [2026-04-16] ingest | layout computation profile

Added `investigations/layout-profile.md`. Release `perf` on `lua-errors`
showed layout-attributed samples at 7.5% of total (470M of 6.3B), with
the biggest single contributor being `LayoutCache::get` via siphash —
the cache was a default `HashMap<u64, CachedFrameLayout>`. Swapped to
`FxHashMap`. Layout 7.5% → 5.0%, total siphash 295M → 76M, release
startup median 1.21s → 1.18s (n=10). Remaining layout cost is real
anchor arithmetic and `resolve_parent_rect` recursion — no further
easy wins.

## [2026-04-16] update | table rehashing fix #2 declined

Tried fix #2 (short-circuit `raw_set_impl` for sequential integer keys
when hash is empty). Two variants both net-negative:
- `array.push` (grow by 1): rehashes 69K → 86K (+25%), wall time tied.
- `array.resize(next_power_of_two)`: rehashes 69K → 57K (−18%) but
  wall time +30% from eager nil-fill on each boundary.

Conclusion documented in `investigations/table-rehashing.md`: the
existing rehash path's `compute_sizes` already does good amortization;
naive replacements either skip the over-allocation (more rehashes
later) or pay the over-allocation eagerly (worse wall time). Status
quo retained.

## [2026-04-16] update | table rehashing fix #1 applied

Applied fix #1 from `investigations/table-rehashing.md`: rilua
`OpCode::NewTable` now pre-allocates 4 hash slots when both size hints
are zero. Measured: 97,340 → 69,105 rehashes (−29%); release startup
median 1.31s → 1.21s (−8%, n=5 each). All 685 rilua tests pass (includes
new `op_newtable_empty_hint_preallocates_hash_to_avoid_first_rehash`).

## [2026-04-16] ingest | table rehashing investigation

Added `investigations/table-rehashing.md`. Profiled startup rehash counts via a
new `rehash-stats` feature in rilua. Found 97K rehashes — 98% from non-frame
tables (`OP_NEWTABLE(0,0)` from addon `local t = {}` pattern), 81% landing at
hash size ≤ 16. Frame-table pre-sizing (64 slots) is working: only 1.6K frame
rehashes. Documented three candidate fixes; none applied in this card.

## [2026-04-14] ingest | world-map exploration seed follow-up

Updated `investigations/world-map-fog-of-war-overlay-model.md` with the current
map exploration follow-up. After removing synthetic fog, Isle of Dorn still
showed fully explored because every default-visible overlay was treated as
discovered. Documented the temporary seed that leaves one real overlay chunk
(`WorldMapOverlay.ID = 4885`, The Three Shields / Skolzgal Mill) unexplored
until per-character exploration state exists, and refreshed the `index.md`
summary.

## [2026-04-14] ingest | world-map fog-of-war overlay model correction

Updated `investigations/world-map-fog-of-war-overlay-model.md` to correct the
previous diagnosis. The current world map does not have a `UiMapFogOfWar` DB
row, so the bug was not a wrong irregular fog shape; it was the simulator
inventing fog for any map art and rendering synthetic geometry from exploration
overlay gaps. Documented the DB-backed fog lookup, the removal of the synthetic
renderer, and the new API/render regressions. Updated `index.md` with the
corrected summary.

## [2026-04-14] ingest | world-map fog-of-war overlay model

Created `investigations/world-map-fog-of-war-overlay-model.md` for the third
world-map fog bug: exploration APIs were already using real irregular overlay
chunks, but the fog renderer still assumed a synthetic half-map model.
Documented the root cause, the `FogOfWarFrame` `uiMapID` plumbing, the new
overlay-complement fog geometry, and the focused API/render regressions.
Updated `index.md` with the new investigation page.

## [2026-04-14] ingest | world-map texture loading budget follow-up

Updated `investigations/world-map-texture-loading-budget.md` with a first-frame
world-map follow-up: BC-preloaded tiles were landing in `bc_cache`, but
`TextureManager::is_cached()` only consulted the RGBA cache. That caused
budgeted draw to pause early after the first BC upload and could make the
world map open with an apparent quarter-map fog/exploration artifact. Added
the BC-cache root cause, fix, and regression coverage to the investigation
page.

## [2026-04-14] ingest | world-map fog-of-war first-open size

Created `investigations/world-map-fog-of-war-first-open-size.md` for the fog
overlay bug where first-open world-map fog could keep a stale size even though
the map tiles were already correct. Documented the missing
`FogOfWarPinMixin:OnCanvasSizeChanged()` handling, the simulator-side
workaround that patches both the mixin and existing fog pins, and the focused
regression tests. Updated `index.md` with the new investigation page.

## [2026-04-14] investigations | world map 90s OnUpdate recapture

Updated `investigations/world-map-onupdate-hover-polling.md` with the fresh
90s world-map profile after the recent OnUpdate fixes. Recorded the new
`/tmp/worldmap-onupdate-20260414.log` numbers (`485` total `fire_on_update`
spikes, `31` steady-state handlers, `64.73ms` post-90s average), added the
new `world_map_onupdate_inventory` handler-ceiling regression test, and
refreshed the `index.md` summary for the page.

## [2026-04-14] investigations | startup XML lifecycle frame-id threading

Updated `investigations/startup-createframe-profile.md` with the loader
follow-up that removes repeated `name -> id -> frame_ref` lifecycle resolution
during XML finalize. Recorded the new `xml_frame.rs` / `xml_lifecycle.rs`
threaded-frame-id path, the focused regression test that fires lifecycle
handlers with a wrong display name but the correct frame id, and refreshed the
`index.md` summary for the page.

## [2026-04-14] investigations | world map UIParent empty worklist follow-up

Updated `investigations/world-map-onupdate-hover-polling.md` with the
`UIParent_OnUpdate` fan-out follow-up: `FCF_OnUpdate`, `ButtonPulse_OnUpdate`,
and `AnimatedShine_OnUpdate` were still doing empty-list Lua dispatch every
tick. Recorded the new post-load wrappers in `workarounds.rs`, the focused
`uiparent_onupdate_worklists` regression tests, and refreshed the `index.md`
summary for the page.

## [2026-04-14] investigations | on-update dirty GameTimeFrame calendar atlas follow-up

Updated `investigations/on-update-dirty.md` with the `GameTimeFrame_SetDate()`
follow-up: same-day calendar atlas updates were still dirtying render because
the plain button texture setter took visual mutable borrows before checking for
real changes. Recorded the new no-op fast path in
`apply_set_button_texture_path()`, the focused atlas-backed button regression
test, the full-UI `GameTimeFrame_SetDate()` regression test, and refreshed the
`index.md` summary for the page.

## [2026-04-14] investigations | on-update dirty handler audit follow-up

Updated `investigations/on-update-dirty.md` with focused handler-audit results:
`LeaveInstanceGroupButton` now shows pure query/dispatch cost once its mutators
settle, while the remaining BuffFrame button cost comes from
`AuraButtonMixin:OnUpdate` doing duration formatting and font-threshold work on
every tick before the no-op setters bail out. Refreshed the `index.md` summary
for the page.

## [2026-04-14] investigations | on-update dirty solo compact raid manager follow-up

Updated `investigations/on-update-dirty.md` with the compact-raid follow-up:
`A_Admin.SetPartySize(0)` now fires `GROUP_ROSTER_UPDATE`, so solo transitions
hide `CompactRaidFrameManager` and remove `LeaveInstanceGroupButton` from the
visible `OnUpdate` handler set. Refreshed the `index.md` summary for the page.

## [2026-04-14] investigations | startup CreateFrame profiling ActionButtonTemplate regions

Updated `investigations/startup-createframe-profile.md` with the direct
`ActionButtonTemplate` layer/fontstring/button-texture fast path: the new
Rust-side region creation in `template/elements*.rs`, the focused regression
test that proves the hot path avoids Lua region fallback, and isolated
`WOW_SIM_PROFILE_CREATE_FRAME` numbers showing another `-27.36%` drop in
explicit template time across the profiled action-bar button families.

## [2026-04-14] investigations | startup CreateFrame profiling nested SpellFX follow-up

Updated `investigations/startup-createframe-profile.md` with the nested `ActionButtonSpellFXTemplate` follow-up: the remaining `ActionButtonInterruptTemplate` / `ActionButtonCastingAnimFrameTemplate` child creation fallback, the widened direct-child selector in `template/children.rs`, and new `WOW_SIM_PROFILE_CREATE_FRAME` numbers showing another `-28.6%` drop in explicit template time across action-bar button families.

## [2026-04-14] investigations | startup CreateFrame profiling MinimalScrollBar recursive fast path

Updated `investigations/startup-createframe-profile.md` with the `MinimalScrollBar` follow-up: the missed `Track -> Thumb` Lua `CreateFrame` fallback inside `apply_inline_frame_content()`, the recursive direct-child propagation change, the new focused regression test, and the smaller but measurable no-addons startup improvement after the fix.

## [2026-04-13] ingest | startup CreateFrame profiling

Created `investigations/startup-createframe-profile.md` to record runtime `CreateFrame` profiling results for Blizzard startup. Documented the new `WOW_SIM_PROFILE_CREATE_FRAME` instrumentation, the measured dominance of action-bar button template expansion (~4.1s across 34 runtime-created buttons), and the link to the planned pure-Rust template child creation work. Updated `index.md` with the new investigation page.

## [2026-04-13] ingest | world map preload API follow-up

Updated `investigations/world-map-texture-loading-budget.md` with the remaining explored-overlay delay root cause: Blizzard's `MapTexturePreloader.lua` was calling `C_Map.RequestPreloadMap()`, but the simulator stubbed that API as a no-op. Recorded the new queued preload path for map art + exploration overlays, the focused `request_preload_map_warms_map_art_and_overlay_textures` regression test, and refreshed the `index.md` summary for that page.

## [2026-04-13] ingest | chat frame scrollbar anchor reapply

Created `investigations/chatframe-scrollbar-anchor-reapply.md` to document the `ChatFrame1` scrollbar/edit-box layout bug. Recorded the real root cause in `reapply_inline_anchors()`: inherited child-frame anchors were resolving `$parent...` against the child name instead of the actual parent frame name, which broke `relativeTo="$parentBackground"` lookups and pushed the resize/scrollbar chain to screen-relative layout. Updated `index.md` with the new investigation page.

## [2026-04-13] ingest | world map texture loading budget follow-up

Updated `investigations/world-map-texture-loading-budget.md` with the second root cause behind the remaining world-map stalls: preload cleared `textures_pending` after CPU cache warmup even while the GPU atlas still lacked most tiles. Recorded the new `gpu_uploaded_textures`-based pending check, the focused `budgeted_preload` regression tests, and refreshed the `index.md` summary for that page.

## [2026-04-13] ingest | world map CreateTexture sublevel investigation

Created `investigations/world-map-create-texture-sublevel.md` to document the follow-up world-map open ordering churn: `CreateTexture(..., subLevel)` ignored its fourth argument, pooled textures started at sublevel 0, and Blizzard immediately repaired them with `SetDrawLayer()`. Recorded the new regressions for `CreateTexture(..., subLevel)` and no-op `SetDrawLayer()`, plus the traced repro where post-open `SetDrawLayer()` invalidations dropped from 150 to 0. Updated `index.md` with the new investigation page.

## [2026-04-13] ingest | world map voice chat alert investigation

Created `investigations/world-map-voice-chat-alerts.md` to document the reduced-stack world-map overlay where voice prompt frames appeared above the panel. Recorded the two harness prerequisites behind it: `Blizzard_Channels` needs `Blizzard_SocialToast` for `SocialToastTemplate hidden="true"`, and alert positioning needs the real chat-alert addons instead of the `ChatAlertFrame` stub. Updated `index.md` with the new investigation page.

## [2026-04-13] ingest | world map OnUpdate hover polling investigation

Created `investigations/world-map-onupdate-hover-polling.md` to document the post-texture-fix `UIParent_OnUpdate` cost: `FCF_OnUpdate` hover polling, the unnecessary mutable borrow in `IsMouseOver()`, the new immutable-borrow regression test, and the runtime repro where verbose OnUpdate logs stayed quiet after startup. Updated `index.md` with the new investigation page.

## [2026-04-13] ingest | world map texture loading budget investigation

Created `investigations/world-map-texture-loading-budget.md` to document the post-rebuild-fix world-map stalls: hidden BC tile uploads, preload/draw source-cache mismatch, the new BC cache in `TextureManager`, and the smaller draw/tick texture budgets. Updated `index.md` with the new investigation page.

## [2026-04-13] ingest | world map frame-level rebuild investigation

## [2026-04-13] investigations | startup CreateFrame profiling follow-up

Updated `investigations/startup-createframe-profile.md` with section-level template profiling, the method-only XML script fast path, widened direct-child creation for `ActionButtonSpellFXTemplate` / `MinimalScrollBar`, and current shared-worktree startup numbers showing `36.79s -> 28.89s` on `--no-addons --no-saved-vars`.

Created `investigations/world-map-frame-level-rebuilds.md` to document the world-map performance bug where map pins repeatedly called `SetFrameLevel()` with the same value, forcing unnecessary `strata_buckets` invalidation and bucket rebuilds. Updated `index.md` with the new investigation page.

## [2026-04-12] ingest | transparent wrapper render-order investigation

Created `investigations/transparent-wrapper-render-order.md` for the world map / quest log render-order fix. Updated it after a follow-up regression to document the depth-aware transparent-wrapper hoist in `state_render.rs`, including both world-map visibility coverage (`world_map_tiles_render_after_tiled_background`) and world-quest pin ordering coverage.

## [2026-04-09] ingest | systems/ pages created (10 pages)

Created all 10 systems/ pages from source docs in docs/:

- systems/layout-system.md — from layout-system.md + anchor-resolution.md
- systems/rendering-pipeline.md — from rendering-pipeline.md
- systems/widget-system.md — from widget-system.md + button-text-rendering.md
- systems/lua-api.md — from lua-api.md
- systems/event-system.md — from event-system.md
- systems/xml-template-system.md — from xml-template-system.md
- systems/addon-loading.md — from addon-loading-pipeline.md
- systems/texture-atlas.md — from texture-atlas-system.md
- systems/frame-data-flow.md — from frame-data-flow.md
- systems/taint-system.md — from protected-frame-enforcement.md + src/lua_api/globals/security_api.rs + src/lua_api/secure_env.rs + src/lua_api/frame/methods/combat_lockdown.rs

Updated index.md systems/ table.

## [2026-04-10] ingest | Initial bulk ingest from 30+ existing docs

Bootstrapped wiki from root-level documentation files. Created pages across systems/, design/, investigations/, and reference/ categories.

## [2026-04-09] ingest | design/ and reference/ pages created

Created 7 pages from DESIGN.md, SCALING.md, docs/debug-tools.md, FUTURE.md, docs/c-api-signature-audit.md, docs/c-api-stub-audit.md, AGENTS.md, and PLAN.md.

Pages created:
- design/architecture-overview.md
- design/scaling-coordinates.md
- design/debug-tools.md
- reference/api-coverage.md
- reference/cli-commands.md
- reference/addon-compatibility.md
- reference/development-phases.md
