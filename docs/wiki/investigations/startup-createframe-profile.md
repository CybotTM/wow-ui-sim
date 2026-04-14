# Startup CreateFrame Profile

Runtime `CreateFrame` profiling shows startup pain is concentrated in script-created action-bar buttons, not disk I/O. The biggest bucket is explicit XML template application inside `CreateFrame`, with action-bar button families alone costing about 4.1s in a no-addons/no-saved-vars startup run.

## Findings

### Instrumentation

`src/lua_api/globals/create_frame.rs` now supports runtime profiling for top-level Lua `CreateFrame` calls via:

- `WOW_SIM_PROFILE_CREATE_FRAME=1` — log slow top-level runtime `CreateFrame` calls
- `WOW_SIM_PROFILE_CREATE_FRAME=all` — log every top-level runtime `CreateFrame` call
- `WOW_SIM_PROFILE_CREATE_FRAME_MIN_MS=<n>` — minimum total duration to log, default `5`

The logger records:

- total `CreateFrame` time
- `finalize` time
- intrinsic template time
- explicit template time
- deferred child `OnLoad` time and child count
- frame self `OnLoad` time

Nested/suppressed `CreateFrame` calls are excluded so the log stays focused on top-level runtime callers such as action-bar construction in `ActionBarMixin:ActionBar_OnLoad()`.

Focused follow-up template timing broke template apply into mixins, key values, direct Rust props, layers, button textures, child creation, animation groups, and script wiring. That run showed the next remaining action-bar bottlenecks were concentrated in child creation for `ActionButtonSpellFXTemplate` and `MinimalScrollBar`.

### Action-bar buttons dominate runtime frame creation

`ActionBarMixin:ActionBar_OnLoad()` creates one button container and one button per slot. The button creation call is:

- `CreateFrame("CheckButton", buttonName, buttonContainer, self.buttonTemplate, i)` in `Interface/BlizzardUI/Blizzard_ActionBar/Shared/ActionBar.lua`

In a profiled `wow-sim --no-addons --no-saved-vars` run, the four action-bar button families contributed:

- `34` top-level button frames
- `4093.84ms` total
- `3807.45ms` in explicit template application
- `247.97ms` in self `OnLoad`
- `15.29ms` in deferred child `OnLoad`
- `442` deferred child `OnLoad` fires

This means the bottleneck is overwhelmingly template expansion/cloning, not the Lua `OnLoad` bodies.

### Worst offenders

Top template totals from the same run:

1. `MainBarActionBarButtonTemplate` — `1897.87ms` across `12` frames
2. `PetActionButtonTemplate` — `1745.11ms` across `10` frames
3. `MinimalScrollBar` — `513.49ms` across `35` frames
4. `StanceButtonTemplate` — `389.79ms` across `10` frames
5. `CompactArenaFrameTemplate` — `347.48ms` for one frame
6. `CompactPartyFrameTemplate` — `242.76ms` for one frame

Representative action-bar button samples:

- `ActionButton4` (`MainBarActionBarButtonTemplate`) — `186.58ms`, with `172.54ms` in explicit templates
- `PetActionButton8` (`PetActionButtonTemplate`) — `190.43ms`, with `183.30ms` in explicit templates
- `StanceButton2` (`StanceButtonTemplate`) — `60.55ms`, with `51.07ms` in explicit templates

### Why explicit templates are so expensive

The action-bar button templates are deep inheritance stacks:

- `MainBarActionBarButtonTemplate` → `ActionBarButtonTemplate`
- `ActionBarButtonTemplate` → `ActionButtonTemplate`, `ActionBarButtonCodeTemplate`
- `ActionButtonTemplate` → `ActionButtonSpellFXTemplate`, `FlyoutButtonTemplate`
- `PetActionButtonTemplate` / `StanceButtonTemplate` also inherit `SmallActionButtonTemplate`, `QuickKeybindButtonTemplate`, and `SecureFrameTemplate`

`ActionButtonTemplate.xml` also creates many child regions/frames per button: icon layers, masks, cooldowns, overlays, text containers, autocast overlay, animation groups, and button state textures. Each runtime button therefore pays for substantial XML-driven child creation and template merging before its Lua `OnLoad` code even matters.

### Follow-up profiling changed the hot path

Section-level template timing showed the next real bottleneck was not key values or mixin copying. On the hot action-bar families, the dominant buckets were:

- `ActionBarButtonCodeTemplate` — almost entirely `scripts`
- `PetActionButtonTemplate` — almost entirely `scripts`
- `ActionButtonSpellFXTemplate` — almost entirely `children`
- `ActionButtonTemplate` — mostly `children`, `layers`, and `button_textures`
- `MinimalScrollBar` — mostly `children`

That led to two targeted fast paths:

1. In `template/elements.rs`, method-only XML script handlers now bypass generated per-frame Lua chunks and install handlers directly.
2. In `template/children.rs`, direct Rust child creation was widened from the initial action-button family to `ActionButtonSpellFXTemplate` and `MinimalScrollBar`.

### Current results on the shared worktree (2026-04-13)

On an idle-core `wow-sim --no-addons --no-saved-vars` run in the current shared worktree, startup improved:

- Blizzard addons loaded: `36.79s -> 28.89s`
- `xmlproc`: `33.22s -> 24.87s`
- setup `exec_lua`: `12.23s -> 11.46s`
- lifecycle: `18.38s -> 10.34s`
- `PLAYER_ENTERING_WORLD`: `44.02s -> 37.44s`

Representative template-total drops on the same shared-tree comparison:

- `MainBarActionBarButtonTemplate`: `2219.04ms -> 307.30ms`
- `PetActionButtonTemplate`: `1590.63ms -> 204.03ms`
- `MultiBar7ButtonTemplate`: `1438.66ms -> 343.10ms`
- `MultiBar1ButtonTemplate`: `1101.71ms -> 304.82ms`
- `MinimalScrollBar`: `560.88ms -> 310.38ms`

Residual hotspots after those changes:

- `MinimalScrollBar` still spends most of its time in child creation
- `ActionButtonSpellFXTemplate` still spends most of its time in child creation
- `ActionButtonTemplate` still spends meaningful time in child creation, layers, and button textures
- non-action-bar one-offs like `CompactArenaFrameTemplate` and `CompactPartyFrameTemplate` are now relatively more visible

### Nested SpellFX child fast path follow-up (2026-04-14)

The first `ActionButtonSpellFXTemplate` fast path still left a lot of work in nested inherited children:

- `ActionButtonInterruptTemplate`
- `ActionButtonCastingAnimFrameTemplate`

Those templates create their own child frames, so the outer `SpellFX` direct-create path still fell back to Lua string child creation inside the inherited descendants.

`template/children.rs` now includes those nested templates in the same direct runtime child-creation selector. On a new profiled `wow-sim --no-addons --no-saved-vars` run with `WOW_SIM_PROFILE_CREATE_FRAME=1 WOW_SIM_PROFILE_CREATE_FRAME_MIN_MS=10`, the hot action-button families improved again:

- summed explicit template time across main-bar, pet-bar, and `MultiBar1-7` buttons: `1688.92ms -> 1205.84ms` (`-28.6%`)
- summed total `CreateFrame` time across the same button families: `2641.24ms -> 1945.03ms` (`-26.4%`)
- `MainBarActionBarButtonTemplate` average explicit time: `17.12ms -> 10.38ms`
- `MultiBar7ButtonTemplate` average explicit time: `18.17ms -> 12.44ms`
- `PetActionButtonTemplate` average explicit time: `15.31ms -> 12.81ms`

Shared-worktree startup also moved again on the same no-addons/no-saved-vars path:

- Blizzard addons loaded: `28.89s -> 17.67s`
- `xmlproc`: `24.87s -> 15.08s`
- setup `exec_lua`: `11.46s -> 6.59s`
- lifecycle: `10.34s -> 6.77s`
- `PLAYER_ENTERING_WORLD`: `37.44s -> 23.66s`

### MinimalScrollBar recursive child fast path follow-up (2026-04-14)

`MinimalScrollBar` was already on the top-level direct child-create hot list, but the fast path stopped one level too early. The template creates:

- `Track`
- `Track.Thumb`
- `Back`
- `Forward`

Only the top-level children were using the Rust-side direct path. The nested `Track -> Thumb` creation still went through generated Lua `CreateFrame(...)` code.

`template/children.rs` now propagates the direct child-create flag into inline descendants and scroll-child descendants instead of dropping back to the Lua path inside `apply_inline_frame_content()`.

The focused regression `test_runtime_minimal_scrollbar_avoids_lua_createframe_for_nested_thumb` proves the change directly by wrapping Lua `CreateFrame`:

- before the fix: runtime `CreateFrame("EventFrame", ..., "MinimalScrollBar")` hit Lua `CreateFrame` twice (root + nested thumb fallback)
- after the fix: the same path hits Lua `CreateFrame` once (root only)

On a current `wow-sim --no-addons --no-saved-vars` rerun with `WOW_SIM_PROFILE_CREATE_FRAME=1 WOW_SIM_PROFILE_CREATE_FRAME_MIN_MS=10`, startup moved slightly again:

- Blizzard addons loaded: `19.03s -> 18.51s`
- `PLAYER_ENTERING_WORLD`: `24.20s -> 23.74s`

## Implications

Small Lua micro-optimizations in `ActionBarActionButtonMixin:OnLoad()` will not move startup enough. The dominant win needs to come from reducing explicit template application cost for runtime-created buttons.

Most promising direction:

1. Replace Lua-string/XML-style child creation for hot runtime template paths with direct Rust child creation.
2. Cache/resuse resolved template expansion data for repeated runtime button templates.
3. Treat action-bar button families as first-class hot paths before chasing smaller costs like scrollbar templates.

## Sources

- [create_frame.rs](../../src/lua_api/globals/create_frame.rs) — runtime `CreateFrame` profiling hooks and timing buckets
- [template/elements.rs](../../src/lua_api/globals/template/elements.rs) — method-only XML script fast path
- [template/children.rs](../../src/lua_api/globals/template/children.rs) — direct Rust child creation hot-path selector
- [ActionBar.lua](../../Interface/BlizzardUI/Blizzard_ActionBar/Shared/ActionBar.lua) — `ActionBar_OnLoad()` runtime button creation loop
- [ActionButton.lua](../../Interface/BlizzardUI/Blizzard_ActionBar/Shared/ActionButton.lua) — action-button `OnLoad` handlers for comparison against template cost
- [ActionButtonTemplate.xml](../../Interface/BlizzardUI/Blizzard_ActionBar/Mainline/ActionButtonTemplate.xml) — action-button template inheritance and child regions
- [MainActionBar.xml](../../Interface/BlizzardUI/Blizzard_ActionBar/Mainline/MainActionBar.xml) — `MainBarActionBarButtonTemplate` definition

## See Also

- [[global-frame-index]] — planned pure-Rust template child creation work
- [[talent-performance]] — earlier startup-oriented performance investigation
