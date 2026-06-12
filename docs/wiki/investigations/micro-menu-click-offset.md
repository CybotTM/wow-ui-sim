# Micro Menu Clicks Missed (Stale Quadrant Anchor)

Clicking micro-menu buttons (e.g. the Group Finder / LFD button) in the GUI did
nothing. The button under the cursor at mouse-down was no longer there at
mouse-up: the first press triggered a deferred `MicroMenu` re-layout that moved
every button by one `QueueStatusButton` slot (~46.5px), so the
`mouse_down_frame == released_on` guard in `dispatch_left_mouse_release`
silently skipped `OnClick`.

## Symptoms

- GUI: clicking LFD (and neighbors) in the micro menu did nothing; the bar
  could visibly shift on the first press.
- Probe trace (`WOW_SIM_DEBUG_CLICK_DISPATCH=1`): mouse-down hit
  `LFDMicroButton`, release hit `GuildMicroButton` at the same cursor point —
  the menu's frames moved between the two events.
- `LFDMicroButton:Click()` worked fine (opens PVEFrame), all buttons enabled,
  handlers registered. Pure GUI-dispatch failure, like
  [mount-journal-click-selection](mount-journal-click-selection.md).

## Root Cause

`MicroMenuMixin:Layout()` (vendor `Blizzard_MicroMenu/Shared/MicroMenuContainer.lua`)
anchors the menu *inside* `MicroMenuContainer` based on
`MicroMenuContainer:GetPosition()` — which screen **quadrant** the container's
center is in (BottomRight → menu bottom-aligned, leaving the
QueueStatusButton slot above, etc.).

Blizzard guarantees this layout runs with final anchors: `EditModeSystemMixin:
UpdateSystem` applies the system anchor *inside* itself (line 394), then the
micro-menu override runs `self:Layout()`; the manager's layout-apply pass also
ends with `InvokeOnAnyEditModeSystemAnchorChanged(force)` so quadrant-dependent
systems re-run after everything settles.

The simulator broke that contract twice:

1. `apply_system_anchors.lua` calls `systemFrame:UpdateSystem(systemInfo)` and
   applies the saved anchor *afterwards* (`apply_system_anchor_if_safe`), so
   the micro menu's `Layout()` saw the container's pre-anchor position.
2. `init_edit_mode_layout` (post-load and post-event) runs while the env still
   has the default layout dimensions; the GUI applies its real window size
   later via `set_screen_size`, which can flip the container's quadrant —
   and nothing re-ran the quadrant-dependent layout.

Result: menu anchored for the wrong quadrant, off by the container-minus-menu
size (46.5px = QueueStatusButton slot). The first interactive press marked the
grid layout dirty, recomputed with the *current* quadrant, and the whole menu
snapped — between mouse-down and mouse-up.

## Fix

`invoke_anchor_changed_hooks` (`src/lua_api/workarounds_editmode.rs`) replays
Blizzard's `EditModeManagerFrame:InvokeOnAnyEditModeSystemAnchorChanged(force)`:

- as a step in `init_edit_mode_layout` right after
  `finalize_action_bar_positions` (mirrors the end of Blizzard's
  `UpdateLayoutInfo`), and
- from `WowLuaEnv::set_screen_size` (`env_runtime.rs`) after the
  `DISPLAY_SIZE_CHANGED`/`UI_SCALE_CHANGED` pair, because our EditMode apply
  ran before the real window size was known.

The broadcast is Blizzard's own mechanism for "anchors moved after the fact";
the base handler is a no-op, micro menu re-runs `Layout()`, objective tracker
and action bars run their own small overrides.

## Verification

`headless-click-probe micromenu` clicks `LFDMicroButton` through the real GUI
mouse pipeline and asserts PVEFrame opens. Before the fix the release landed
on `GuildMicroButton`; after, hover/down/up all stay on LFD and the panel
opens. Full lib suite (1580), mounts/achievements probes, and `lua-errors`
(minimal and full load) stay clean.

## Open Edges

- With `--no-saved-vars`, the loaded EditMode cache sets micro menu
  orientation to Vertical (`MicroMenu.isHorizontal == false`) and anchors the
  container `TOP UIParent TOP 657 -208.5`. Whether that matches the cache's
  intent (vs. a settings-replay gap) was not verified here — the quadrant fix
  is orientation-agnostic.
- The probe boots at 1024x768; env init uses different default dimensions.
  Any other system whose layout reads its own screen position during startup
  has the same hazard — the broadcast now covers all of them, but only for
  systems that implement `OnAnyEditModeSystemAnchorChanged`.

## Related

- [mount-journal-click-selection](mount-journal-click-selection.md) — same
  symptom class (GUI click dead, `:Click()` fine), different layer.
- `docs/wiki/investigations/display-size-ui-scale-events.md` — the resize
  event pair this fix piggybacks on.
- `docs/editmode-layout-fixes.md` — earlier EditMode startup workarounds.
