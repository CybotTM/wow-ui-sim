# Mists Panel Stack Overflow Layout Cycle

## Summary

Achievements and Talents could still abort the process with:

```text
thread 'main' (...) has overflowed its stack
fatal runtime error: stack overflow, aborting
```

The earlier check was insufficient because it opened panel Lua paths without exercising the real GUI hit-test and click update path. The better check is `headless-click-probe`, which boots the GUI app headlessly, rebuilds the hit grid, and sends real mouse move/down/up events to named frame centers.

## Root Cause

The crash was not another hit-test recursion bug. A coredump captured from `headless-click-probe achievements` showed repeated layout frames:

```text
compute_frame_rect_cached
resolve_parent_rect
resolve_uncached_frame_layout
resolve_single_anchor
```

Full addon and SavedVariables state can create parent/anchor layout cycles. `LayoutCache` only memoized completed frame layouts, so an in-progress frame could re-enter itself through parent or relative-frame resolution and recurse until Rust aborted the process.

## Fix

`LayoutCache` now tracks both resolved frames and frames currently being resolved. Re-entering an active frame returns the invalid/missing layout sentinel instead of recursing forever, and the active marker is cleared after resolution completes.

The fix is intentionally in the simulator layout model, not in Blizzard Lua or addon shims. Cyclic layout state should not be able to crash the host process.

## Regression Coverage

- `parent_cycle_does_not_recurse_until_stack_overflow`
- `anchor_cycle_does_not_recurse_until_stack_overflow`
- `headless-click-probe achievements`
- `headless-click-probe talents`

The hidden probe is the important panel-level check because it catches native aborts that Lua-only open-panel probes miss.

## Commands

```bash
cargo test --lib cycle_does_not_recurse_until_stack_overflow
cargo test --lib get_server_time_returns_unix_seconds_for_addon_date_calls
cargo build --no-default-features --features "sound,gui,casc,client-mists" --bin wow-sim
timeout 180 target/debug/wow-sim headless-click-probe achievements
timeout 180 target/debug/wow-sim headless-click-probe talents
timeout 150 target/debug/wow-sim --no-saved-vars headless-click-probe achievements
timeout 150 target/debug/wow-sim --no-saved-vars headless-click-probe talents
```
