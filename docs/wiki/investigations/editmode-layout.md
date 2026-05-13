# EditMode Layout

Four frame position regressions after commit 73e6032 (which made `__index` check Lua fields before Rust methods, matching real WoW behavior).

## Root Cause

`EditModeSystemMixin:OnSystemLoad` replaces three core methods on all 43 registered system frames by writing into each frame's per-frame fenv table:

```lua
self.SetPoint  = self.SetPointOverride    -- expects 5 explicit args
self.SetScale  = self.SetScaleOverride    -- adjusts anchor offsets on scale change
self.ClearAllPoints = self.ClearAllPointsOverride
```

After 73e6032, `__index` checks the fenv table before Rust methods, so all `:SetPoint()` / `:SetScale()` calls now hit these Lua overrides.

## Regressions

| Frame | Root Cause |
|---|---|
| FocusFrame | `SetScaleOverride` adjusts existing anchor offsets when scale changes; offset shifted from 520 to 693.3 |
| ObjectiveTrackerFrame | `SetPointOverride` expects 5 explicit args; 3-arg form `SetPoint("TOPRIGHT", -offset, -topOffset)` mapped numbers as relativeTo/relativePoint |
| UIParentRightManagedFrameContainer | `GetActionBarToggles` returning `(1,1,1,1)` showed MultiBarLeft/Right, making `GetRightActionBarWidth()` return ~100px |
| PetFrame | Hidden frame (no pet); removed from test |

## Fix (`workarounds_editmode.rs`)

**`clear_edit_mode_overrides`**: After `UpdateSystems` runs, clear all three overrides from every system frame's fenv via `rawset(debug.getfenv(frame)[1], "SetPoint", nil)` etc.

**`GetActionBarToggles` fix**: Changed stub to return `(false, false, false, false)` — default for a character with no optional bars enabled. This hides MultiBarLeft/Right, making `GetRightActionBarWidth` return 0 naturally.

**`reposition_managed_frames`**: Re-run `UpdateManagedFrames()` after all fixes to trigger correct `VerticalLayoutMixin:Layout()`.

**`patch_update_systems` / `patch_init_anchors`**: Skip `isManagedFrame` frames — their position comes from the container layout, not preset anchors.

## Key Constraint

`rawset(frame, key, nil)` fails — FrameRef is userdata, not a table. Per-frame fields must be accessed through `debug.getfenv(frame)[1]`.

## Follow-up: Active Profile State

The runtime bootstrap fallback for `C_EditMode.GetLayouts()` used to return
`{ layouts = {}, activeLayout = 1 }` on every call, while
`C_EditMode.SetActiveLayout()` was a no-op. Blizzard's
`EditModeManagerFrameMixin:UpdateLayoutInfo()` prepends preset layouts before
saved account/character layouts, so a hard-coded `activeLayout = 1` selects the
first preset profile and presents the default UI.

The fallback now keeps an in-memory `C_EditMode` state model:
`SaveLayouts()` stores saved layout data, `SetActiveLayout()` updates the
selected layout index, and `GetLayouts()` returns a defensive copy.

Real EditMode layout data is not a Lua SavedVariables file. WoW writes it to
`WTF/Account/<account>/edit-mode-cache-account.txt`, while the active
per-specialization selection lives in
`WTF/Account/<account>/<realm>/<character>/edit-mode-cache-character.txt`.
Startup now imports those cache files before Blizzard addons load, decodes the
compact system/anchor/settings rows, and seeds `C_EditMode`. Blizzard addons
are also loaded through the saved-variable-aware loader so Blizzard-owned WTF
SavedVariables participate in full UI startup.

Follow-up: `EditModeManagerFrame.layoutInfo` prepends Blizzard preset layouts
before appending saved layouts. The active value from the character cache is an
index into the saved layouts, so startup must remap it after the prepend; without
that remap, `C_EditMode` selected `Widescreen` while `EditModeManagerFrame`
activated the `Classic` preset. Preset-only compatibility tweaks must also avoid
rewriting saved layout settings.

Follow-up: selecting the saved layout still was not enough for the visible UI.
The startup fast path for action bars seeded Widescreen `systemInfo` and replayed
settings, but skipped `ApplySystemAnchor`, leaving bars at the temporary
`TOPLEFT, 0, 0` anchor from `InitSystemAnchors()`. The fast path now applies the
saved anchor before refreshing bar layout/art.

Follow-up: the main action bar side art is an EditMode setting
(`Enum.EditModeActionBarSetting.HideBarArt`), not independent renderer state.
Sparse saved-layout rows can omit default-valued settings, so startup now merges
missing defaults from Blizzard's modern preset map before seeding systems. The
action-bar fast path applies those fields directly instead of calling Blizzard
handlers that iterate `self.actionButtons` before the simulator has a real Lua
button array there.

Follow-up: startup no longer forces the active profile's cast bar to lock under
the player frame. Saved profiles can intentionally set
`Enum.EditModeCastBarSetting.LockToPlayerFrame = 0` and
`Enum.EditModeUnitFrameSetting.CastBarUnderneath = 0`; those values must remain
profile data, while missing defaults are still filled from the preset map.

Follow-up: some EditMode systems are intentionally seeded instead of sent through
Blizzard's full `UpdateSystem()` during startup because those paths do expensive
roster work or can apply scale before the frame is ready. Seeding still needs to
replay saved settings, so the startup path now calls `UpdateSystemSetting()` for
those systems after loading their saved `systemInfo`. Unit-frame
`BuffsOnTop` is deferred when a seeded frame has no `UpdateAuras()` method,
which keeps ScriptErrors clean without dropping the saved setting from the
profile data.

Regression coverage lives in `tests/edit_mode_api.rs`.

## Files Modified

- `src/lua_api/workarounds_editmode.rs`
- `src/lua_api/globals/c_stubs_api.rs`
- `tests/frame_positions.rs`, `tests/action_bar.rs`
- `src/lua_api/env_init/runtime_surface_bootstrap.lua`
- `src/saved_variables.rs`
- `src/bin/wow_sim/main.rs`
- `src/bin/wow_sim/addon_loading.rs`
- `tests/edit_mode_api.rs`

## Sources

- [editmode-layout-fixes.md](../../editmode-layout-fixes.md) — full investigation
- [runtime_surface_bootstrap.lua](../../../src/lua_api/env_init/runtime_surface_bootstrap.lua) — `C_EditMode` fallback state
- [saved_variables.rs](../../../src/saved_variables.rs) — WTF cache import bridge

## See Also

- [[method-dispatch-refactor]] — the `__index` ordering change that exposed these bugs
