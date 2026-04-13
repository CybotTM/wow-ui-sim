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

## Implications

Small Lua micro-optimizations in `ActionBarActionButtonMixin:OnLoad()` will not move startup enough. The dominant win needs to come from reducing explicit template application cost for runtime-created buttons.

Most promising direction:

1. Replace Lua-string/XML-style child creation for hot runtime template paths with direct Rust child creation.
2. Cache/resuse resolved template expansion data for repeated runtime button templates.
3. Treat action-bar button families as first-class hot paths before chasing smaller costs like scrollbar templates.

## Sources

- [create_frame.rs](../../src/lua_api/globals/create_frame.rs) — runtime `CreateFrame` profiling hooks and timing buckets
- [ActionBar.lua](../../Interface/BlizzardUI/Blizzard_ActionBar/Shared/ActionBar.lua) — `ActionBar_OnLoad()` runtime button creation loop
- [ActionButton.lua](../../Interface/BlizzardUI/Blizzard_ActionBar/Shared/ActionButton.lua) — action-button `OnLoad` handlers for comparison against template cost
- [ActionButtonTemplate.xml](../../Interface/BlizzardUI/Blizzard_ActionBar/Mainline/ActionButtonTemplate.xml) — action-button template inheritance and child regions
- [MainActionBar.xml](../../Interface/BlizzardUI/Blizzard_ActionBar/Mainline/MainActionBar.xml) — `MainBarActionBarButtonTemplate` definition

## See Also

- [[global-frame-index]] — planned pure-Rust template child creation work
- [[talent-performance]] — earlier startup-oriented performance investigation
