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

## Verification

- Red phase:
  - `cargo test test_is_mouse_over_clean_layout_does_not_require_mutable_state_borrow --lib`
  - failed before the fix with `RefCell already borrowed`
- Green phase:
  - `cargo test test_is_mouse_over_clean_layout_does_not_require_mutable_state_borrow --lib`
  - `cargo test test_is_mouse_over_uses_mouse_position_and_optional_offsets --lib`
  - `cargo test test_is_mouse_over_requires_mouse_enabled --lib`
- Runtime repro:
  - `LD_LIBRARY_PATH=target/debug:target/debug/deps WOW_SIM_VERBOSE=1 timeout 160 ./target/debug/wow-sim --no-addons --no-saved-vars --exec-lua "ToggleWorldMap()"`
  - The captured log had `onupdate_lines=0` for `\[fire_on_update\]` / `\[OnUpdate\]` matches.
  - The last log line was `[119.910s] [Startup] Firing UPDATE_CHAT_WINDOWS`; the process then stayed below verbose OnUpdate logging thresholds until `timeout` killed it at 160 seconds. This is an inference from the empty tail after startup.

## Sources

- [methods_core_region.rs](../../../src/lua_api/frame/methods/methods_core_region.rs) — `IsMouseOver()` implementation and the clean-path fix
- [mod.rs](../../../src/loader/tests/mod.rs) — regression test for immutable-borrow-safe clean hover queries
- [FloatingChatFrame.lua](../../../Interface/BlizzardUI/Blizzard_ChatFrameBase/Mainline/FloatingChatFrame.lua) — `FCF_OnUpdate` hover polling
- [UIParent.lua](../../../Interface/BlizzardUI/Blizzard_UIParent/Mists/UIParent.lua) — `UIParent_OnUpdate` fan-out
- [on-update-dirty-handlers.md](../../on-update-dirty-handlers.md) — earlier OnUpdate investigation context

## See Also

- [[on-update-dirty]] — separate investigation into dirty suppression after OnUpdate handlers
- [[world-map-frame-level-rebuilds]] — earlier fix for steady-state world-map bucket rebuilds
- [[world-map-texture-loading-budget]] — follow-up fix for world-map texture stalls after rebuilds were removed
