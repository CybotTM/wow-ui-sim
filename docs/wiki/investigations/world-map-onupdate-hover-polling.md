# World Map OnUpdate Hover Polling

With the world map open, the next dominant performance issue after the rebuild and texture fixes was `UIParent_OnUpdate`. The expensive work was not the Lua dispatcher itself, but repeated chat-frame hover checks calling into a heavyweight `IsMouseOver()` implementation every tick.

## Symptom

Representative user logs during a world-map repro showed idle `OnUpdate` work well above frame budget:

```text
[fire_on_update] 35 handlers: OnUpdate=22.5ms total=22.7ms
[OnUpdate]    16.5ms  UIParent_OnUpdate
[OnUpdate]     5.5ms  [Button]_OnUpdate
```

Reading the Blizzard side narrowed `UIParent_OnUpdate` to a small fan-out:

- `FCF_OnUpdate(elapsed)`
- `ButtonPulse_OnUpdate(elapsed)`
- `AnimatedShine_OnUpdate(elapsed)`
- `HelpOpenWebTicketButton_OnUpdate(...)`

`FCF_OnUpdate` was the important part. It polls `IsMouseOver()` on chat frames, tabs, scrollbars, and quick-join widgets to drive hover fade behavior.

## Root Cause

Our `Frame:IsMouseOver()` fast path was not actually fast:

- it took `state_rc.borrow_mut()` unconditionally,
- called `resolve_rect_if_dirty(id)` even when the frame was already clean,
- then read mouse position and layout data.

That mattered because `FCF_OnUpdate` calls `IsMouseOver()` repeatedly while idle. The clean-layout case should have been a read-only query, but every call still paid for mutable `SimState` access.

The regression test that exposed this precisely was holding an immutable `SimState` borrow while calling `IsMouseOver()` on a clean frame. Before the fix it failed with:

```text
RefCell already borrowed
```

That panic came from the unconditional mutable borrow inside `IsMouseOver()`.

## Fix

`IsMouseOver()` now splits into two paths:

1. Read-only check to see whether the frame is rect-dirty.
2. Only if dirty, take a mutable borrow and resolve the rect.
3. Finish the actual bounds check through a read-only helper.

This preserves layout resolution for dirty frames while removing mutable state access from the common clean-layout hover query path.

## 2026-04-14 UIParent idle fan-out follow-up

The next follow-up stayed on the same `UIParent` fan-out, but on a simpler idle
cost: the mainline `UIParent` `OnUpdate` script always called
`FCF_OnUpdate(elapsed)`, `ButtonPulse_OnUpdate(elapsed)`, and
`AnimatedShine_OnUpdate(elapsed)` every frame.

Tracing the Blizzard side showed that:

- `ButtonPulse_OnUpdate` immediately iterates `PULSEBUTTONS`
- `AnimatedShine_OnUpdate` immediately iterates `SHINES_TO_ANIMATE`
- `FCF_OnUpdate` immediately iterates `CHAT_FRAMES`

That means the empty-worklist case still paid for Lua dispatch plus a `pairs()`
walk setup every tick, even though there was nothing to update.

The simulator fix was a post-load Lua patch in `workarounds.rs` that wraps
those three globals and returns early only when their backing table exists and
`next(worklist) == nil`.

The important limit: this only removes the empty-list case. In real settled game
UI, `CHAT_FRAMES` is still non-empty, so the remaining `FCF_OnUpdate` hover
polling cost still needs a later pass if it stays hot after re-profiling.

## 2026-04-14 90s world-map recapture

After the compact-raid, `GameTimeFrame_SetDate()`, and empty-worklist fixes, a
fresh manual repro still showed that the world-map `OnUpdate` problem is not
solved yet.

Command:

```bash
timeout 105 env LD_LIBRARY_PATH=target/debug:target/debug/deps \
  WOW_SIM_VERBOSE=1 \
  ./target/debug/wow-sim --no-addons --no-saved-vars --exec-lua "ToggleWorldMap()"
```

Captured log: `/tmp/worldmap-onupdate-20260414.log`

Key numbers from that run:

- `485` total `[fire_on_update]` spikes
- first spike at `48.006s`: `30` handlers, `264.0ms` total
- all later spikes used `31` visible handlers
- post-90s window (`>= 90s` absolute runtime): `169` spikes
- post-90s average total: `64.73ms`
- post-90s max total: `125.7ms` at `92.732s`
- no per-handler `[OnUpdate]` lines crossed the per-handler `5ms` log threshold

That last point matters: the remaining cost is now spread across many sub-5ms
handlers rather than one obvious single-frame spike.

Updated expectations from this recapture:

- **Immediate-open inventory ceiling:** keep the visible world-map `OnUpdate`
  set at or below `32` handlers. This now has a focused regression test.
- **Steady-state goal:** future fixes should drive the world-map idle run back
  below the `20ms` `[fire_on_update]` logging threshold, ideally reaching zero
  post-startup `fire_on_update` lines again.

## Verification

- Red phase:
  - `cargo test test_is_mouse_over_clean_layout_does_not_require_mutable_state_borrow --lib`
  - failed before the fix with `RefCell already borrowed`
- Green phase:
  - `cargo test test_is_mouse_over_clean_layout_does_not_require_mutable_state_borrow --lib`
  - `cargo test test_is_mouse_over_uses_mouse_position_and_optional_offsets --lib`
  - `cargo test test_is_mouse_over_requires_mouse_enabled --lib`
  - `cargo test --test uiparent_onupdate_worklists`
  - `cargo test --test world_map_onupdate_inventory`
- Runtime repro:
  - `LD_LIBRARY_PATH=target/debug:target/debug/deps WOW_SIM_VERBOSE=1 timeout 160 ./target/debug/wow-sim --no-addons --no-saved-vars --exec-lua "ToggleWorldMap()"`
  - The captured log had `onupdate_lines=0` for `\[fire_on_update\]` / `\[OnUpdate\]` matches.
  - The last log line was `[119.910s] [Startup] Firing UPDATE_CHAT_WINDOWS`; the process then stayed below verbose OnUpdate logging thresholds until `timeout` killed it at 160 seconds. This is an inference from the empty tail after startup.
  - 2026-04-14 recapture: `timeout 105 env LD_LIBRARY_PATH=target/debug:target/debug/deps WOW_SIM_VERBOSE=1 ./target/debug/wow-sim --no-addons --no-saved-vars --exec-lua "ToggleWorldMap()"`
  - That run still produced `485` `[fire_on_update]` spikes; the post-90s window averaged `64.73ms` across a stable `31` visible-handler set.

## Sources

- [methods_core_region.rs](../../../src/lua_api/frame/methods/methods_core_region.rs) — `IsMouseOver()` implementation and the clean-path fix
- [mod.rs](../../../src/loader/tests/mod.rs) — regression test for immutable-borrow-safe clean hover queries
- [FloatingChatFrame.lua](../../../Interface/BlizzardUI/Blizzard_ChatFrameBase/Mainline/FloatingChatFrame.lua) — `FCF_OnUpdate` hover polling
- [UIParent.xml](../../../Interface/BlizzardUI/Blizzard_UIParent/Mainline/UIParent.xml) — mainline `UIParent` `OnUpdate` fan-out
- [workarounds.rs](../../../src/lua_api/workarounds.rs) — empty-worklist wrappers for `FCF_OnUpdate`, `ButtonPulse_OnUpdate`, and `AnimatedShine_OnUpdate`
- [uiparent_onupdate_worklists.rs](../../../tests/uiparent_onupdate_worklists.rs) — focused regression coverage for empty and active worklists
- [world_map_onupdate_inventory.rs](../../../tests/world_map_onupdate_inventory.rs) — initial-open visible-handler ceiling for the world-map `OnUpdate` set
- [on-update-dirty-handlers.md](../../on-update-dirty-handlers.md) — earlier OnUpdate investigation context

## See Also

- [[on-update-dirty]] — separate investigation into dirty suppression after OnUpdate handlers
- [[world-map-frame-level-rebuilds]] — earlier fix for steady-state world-map bucket rebuilds
- [[world-map-texture-loading-budget]] — follow-up fix for world-map texture stalls after rebuilds were removed
