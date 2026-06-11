# Mouse Input Dead at "50 FPS" (Probe Blockers + Idle Tick Stall)

In a full-addon GUI session (`WOW_SIM_EDIT_MODE_LAYOUT=Ultrawide`), the mouse stopped affecting the UI — no hover, no cursor reaction — while the title bar still showed ~50 FPS. Three stacked causes: CoreBehaviorProbe left two full-screen mouse-enabled DIALOG blockers over UIParent; those stayed up because pending `C_Timer.After` callbacks don't wake the tick loop once the app idles; and the probe addon shouldn't have loaded at all — its folder was renamed `CoreBehaviorProbe.disabled`, but the loader's TOC scan accepted any `.toc` in the folder.

## Content

### Symptoms and misleading signals

- Hover/click targeting dead everywhere; "mouse cursor not updating".
- Title bar showed ~50 FPS — **stale**: `update_fps_counter` only runs inside `finish_timer_tick`, so when timer ticks stop, the FPS display freezes at its last value.
- Only manifested with `WOW_SIM_EDIT_MODE_LAYOUT=Ultrawide` — indirect: under the default layout something keeps strata perpetually dirty (`dirty=0x218` every tick), so ticks never stop, the probe's cleanup chain completes, and the blockers get hidden. Ultrawide reaches true idle.

### Diagnosis path (reusable)

1. `wow-cli mouse-move --x X --y Y` prints the hovered frame — same frame at every position ⇒ a blocker is eating input.
2. `wow-cli lua -e '...GetMouseFocus()...'` → anonymous Frame, DIALOG:10, full-screen, `EnableMouse(true)`, parent UIParent.
3. `wow-cli dump-tree` attributes anonymous frames to their owning addon (`@CoreBehaviorProbe.disabled`), and adjacent creation-order entries identify the creation site.
4. Hiding the blockers live restored hover instantly — confirming cause before any code change.

### Cause 1: CoreBehaviorProbe leftover blockers

`makeRaiseHitFrame` (Raise/Lower hit probe) creates two full-screen `SetAllPoints(UIParent)` + `DIALOG` + `EnableMouse(true)` frames (levels 1 and 10) and hides them only at the end of a 3-deep `C_Timer.After(0)` chain. `CoreBehaviorProbeDB.raiseHit.status` stuck at `"pending"` ⇒ chain never completed ⇒ blockers persist for the whole session.

### Cause 2 (fixed): subscription churn starved timer ticks under continuous input

`compute_tick_interval` returned the raw *remaining time* to the next pending C_Timer. iced keys the `time::every` subscription identity by the interval value and rebuilds the subscription set after **every** update — so each incoming message (every mouse move, every IPC command) produced a different interval, tore down the tick stream, and reset its first fire. Under continuous input the stream was recreated before it ever fired: ProcessTimers starved, pending `C_Timer` chains stalled, OnUpdate stopped, and the FPS display froze. The user moving the mouse to "wake" the UI was exactly what kept it dead.

The default layout masked it because something keeps strata perpetually dirty there (constant fast-tick interval → stable stream); `Ultrawide` reached a clean state and exposed the shrinking-interval path.

Fixed in commit `397742569`: `stable_timer_interval` quantizes the timer wait into fixed buckets (16/50/250/1000 ms), so the subscription identity survives successive updates (at most one stream recreation per bucket boundary; timers still fire within one tick of their deadline). Regression tests: `stable_timer_interval_uses_fixed_buckets`, `compute_tick_interval_is_stable_while_pending_timer_shrinks` (fails against the raw-delay code).

Note: the FPS-display-freezes-with-ticks coupling (`update_fps_counter` only runs in `finish_timer_tick`) is still true and worth decoupling someday — a dead tick loop should read 0 FPS, not the last value.

### Cause 3 (fixed): renamed `.disabled` addon folders still loaded

`find_toc_file`'s last-resort scan accepted any `.toc` in the folder, so `CoreBehaviorProbe.disabled/CoreBehaviorProbe.toc` loaded. Real WoW only loads a TOC whose stem is the folder name plus an optional `_`/`-` flavor suffix — that rule is exactly what makes rename-to-disable work. Fixed in commit `b580ea005` (`toc_stem_matches_folder`, tests in `src/loader/tests/toc_discovery.rs`); case-mismatch and flavor-suffix fallbacks still work.

### Live remediation (no restart)

```lua
for _, child in ipairs({ UIParent:GetChildren() }) do
  if child:GetName() == nil and child:GetFrameStrata() == "DIALOG" and child:IsShown()
     and child:GetWidth() > 1000 and child:IsMouseEnabled() then
    child:Hide()
  end
end
```

## Sources

- `/syncthing/World of Warcraft/_retail_/Interface/AddOns/CoreBehaviorProbe.disabled/CoreBehaviorProbe.lua` — `makeRaiseHitFrame`, `startRaiseHitProbe`
- `src/iced_app/app.rs` — `compute_tick_interval`; `src/iced_app/update_runtime.rs` — `handle_process_timers`
- `src/iced_app/render_textures.rs` — `build_overlay` (software cursor); `src/iced_app/update.rs` — `update_fps_counter`
- `src/loader/mod.rs` — `find_toc_file` / `scan_for_compatible_flavor_toc`

## See Also

- [[retail-core-behavior-probes]] — what CoreBehaviorProbe measures and why it exists
- `docs/hit-testing.md` — hit grid and hover resolution
