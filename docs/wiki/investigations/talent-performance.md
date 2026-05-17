# Talent Performance

Three separate performance problems affecting the talent panel: slow initial load, an OnUpdate loop that dropped the sim to ~5 FPS, and a multi-second Spec→Talents tab switch caused by a recursive `issecretvalue`.

## Problem 1: Slow Initial Load

Opening the talent panel demand-loads `Blizzard_PlayerSpells`, which creates hundreds of frames. Cost per frame includes mlua userdata allocation, metatable setup, and `_G` insertion.

### Fixes Applied

**`__index` on `_G` (lazy frame lookup)**: Set a metatable `__index` on `lua.globals()` that creates a FrameHandle only when Lua code actually accesses a frame by name. Frames exist in Rust only; Lua userdata is materialized on first access and cached via `rawset`. Eliminated GC overhead during talent panel creation (9.5% → ~0% GC in debug).

**LightUserData migration**: Light userdata has no `__gc` finalizer, so the GC skips it in `luaC_separateudata`. GC overhead dropped from 9.5% → 5.0% in the profile (release build).

### Benchmark Results (release, 10 opens)

| Version | First open | Subsequent |
|---|---|---|
| Baseline | 431ms | 94ms |
| `__index` on `_G` | 263ms | 92ms |
| + LightUserData | 262ms | 76ms |

Remaining cost: Rust-side template instantiation (`get_template`, `compute_frame_rect`, HashMap hashing) and Lua error traceback building (`luaH_next` in `compat53_findfield`).

## Problem 1b: Discarded Strata Repair Work on Show

### Symptom

After the shallow `issecretvalue` fix, the repeated open path was still slower than expected. Release `bench_talents` runs showed first opens in the expected broad range, but subsequent opens around 200ms+ instead of the documented ~76-90ms range.

### Root Cause

`SimState::set_frame_visible` always called `try_repair_strata_buckets_after_show` after `Show()`. That helper built a same-strata subtree repair plan before checking whether `strata_buckets` existed. In headless benchmark/loading paths the buckets are often `None`, so every talent button/region show paid `collect_same_strata_subtree_ids` / `frame_render_alpha` work and then discarded the result.

### Fix

`try_repair_strata_buckets_after_show` now returns immediately when `strata_buckets` is `None`. The existing repair path is preserved for live rendered buckets; unloaded buckets can still stay invalidated and build lazily on first render.

### Benchmark Results

Release `bench_talents` on this worktree:

| Version | First open | Subsequent |
|---|---:|---:|
| Before guard | 259-472ms | 211-282ms |
| After guard | 364-398ms | 112-150ms |

## Problem 2: OnUpdate Loop (5 FPS)

### Symptom

Opening the talent panel caused OnUpdate to fire every tick at 90–190ms.

### Root Cause

All 134 talent buttons had `IsRectValid() = false`. `TalentEdgeArrowMixin:UpdatePosition()` calls `startButton:IsRectValid()` — on false, it calls `MarkEdgesDirty` → `RegisterOnUpdate()` on every edge every tick, creating an infinite loop.

Buttons appeared rect-invalid due to two bugs in `is_rect_dirty()`:

1. **Stale `Some(true)` caches**: `is_rect_dirty()` cached `Some(true)` on the walked ancestor path. After `drain_rect_dirty()` cleared the ancestor, descendants retained stale dirty caches.
2. **`resolve_rect_if_dirty` didn't clear ancestors**: Only cleared the target frame's dirty flag; the ancestor (ButtonsParent) remained in `rect_dirty_ids`.

### Fix

1. `is_rect_dirty()` now only caches `Some(false)` (clean) results — dirty results are never cached.
2. `resolve_rect_if_dirty(id)` walks up the ancestor chain, finds all dirty roots, computes their layout rects, and clears their flags before resolving the target.
3. Removed `workarounds_talents.rs` (Lua-side workaround targeting the wrong cause).

## Problem 3: Spec→Talents Tab Switch (~3.5s)

### Symptom

Switching from the Specializations tab back to Talents took multiple seconds (user-reported "about 20s" with full addon load). `Blizzard_ClassTalentsFrame.xml` sets `refreshOnShow=true`, so `LoadTalentTreeInternal` rebuilds the tree on every Show: `talentButtonCollection:ReleaseAll()` releases all 135 talent buttons, then re-instantiates them. The release alone took ~2.1s — ~16ms per button.

### Root Cause

`SecureObjectPoolMixin:CheckAllowReleaseObject` calls `issecretvalue(object)` on every released frame, and `SecureMap`/`SecureStack`/`SecureNumber` assertions add 2–3 more `issecretvalue` calls per release. The Rust fallback `value_is_secret` recursed into nested table contents via `table_is_secret`, walking the entire WoW frame's deep table tree (~7.4ms per call on a frame).

### Fix

`src/lua_api/globals/security/secret_values.rs` adds `value_is_secret_shallow`, used by `issecretvalue`, `canaccessvalue`, and `canaccessallvalues`. For tables it only inspects direct slot taints set via `__sim_mark_slot_taint` — no entry walk, no recursion. `canaccesstable` keeps the deep `table_is_secret` walk so its accessibility semantics still detect nested secret strings (verified by `test_table_containing_party_identity_is_not_accessible`).

### Benchmark Results

| Operation | Before | After |
|---|---|---|
| `talentButtonCollection:ReleaseAll()` (135 buttons) | 2159ms | 2.6ms |
| Spec→Talents tab switch | ~3500ms | ~90ms |
| `issecretvalue(frame)` × 1000 | 7411ms | <10ms |

## Problem 4: Eager `animation_frame_ids_for_group` (1.3 FPS panel-open)

### Symptom

GUI title bar reported `1.3 FPS | tick:304ms | draw:401ms` while the talent panel sat open. Flamegraph attached to the live process: `animation_frame_ids_for_group` was **63.9% of all CPU time**.

### Root Cause

`advance_animation_group` (in `src/lua_api/frame/methods/button_anchor_hierarchy/animations.rs`) called `animation_frame_ids_for_group(sim, group_id)` unconditionally on every group, every tick. That helper does a full linear scan of `sim.anim_frame_to_anim` (a `HashMap<frame_id, (group_id, animation_index)>` covering every animation in the sim).

Cost was **O(groups × total_anim_frames) per tick**. With the talent panel open, the sheen animation alone produces ~135 active groups, plus everything else in the UI; `anim_frame_to_anim` carries thousands of entries. Result: the inner loop dominated steady-state CPU.

The collected `Vec<u64>` had a single consumer: `finished_animation_scripts.extend(...)` inside `if group_finished { ... }`. For a sheen on a 22-second sync cycle, `group_finished` is false on virtually every tick, so the work was thrown away.

### Fix

Move the call inside the `if group_finished` branch. Restructure the inner block to return `group_finished`, then call `animation_frame_ids_for_group` only when needed.

### Result

Re-profiled with the same setup (talent panel open, 12s perf record):

| Metric | Before | After |
|---|---:|---:|
| `animation_frame_ids_for_group` | 63.9% | 0.00% |
| `advance_animation_groups` total | dominant | 3.5% |

Tests: `animation_anim`, `animation_group`, `animation_group_state`, `anim_target_visibility` — 64 tests, all green.

## Problem 5: Full-addon Mists first talent open pays deferred AceAddon enable queue

### Symptom

With third-party addons loaded, the Mists talent panel did open, but the first `ToggleTalentFrame()` looked like a hang. `headless-click-probe talents` showed the process survived, but setup Lua took 30-40s before any tab clicks ran.

### Root Cause

The simulator skipped BlizzMove's `PLAYER_LOGIN` handler under Mists. BlizzMove embeds AceAddon-3.0, and that skipped login handler left AceAddon's `enablequeue` populated after startup:

```text
initializequeue=0 enablequeue=27 IsLoggedIn=true
```

The next non-early `ADDON_LOADED` event was `Blizzard_TalentUI`. AceAddon treated that late addon load as a chance to flush the pending enable queue, so `LoadAddOn("Blizzard_TalentUI")` paid unrelated addon enable work. Profiling showed `EnableAddon ElvUI` was the expensive queued item, while Blizzard_TalentUI's own files loaded in well under a second.

### Fix

Let BlizzMove receive `PLAYER_LOGIN` again. That flushes the AceAddon queue during startup, where it belongs, instead of charging the first talent-panel open.

`headless-click-probe` now logs setup and click durations so this failure mode is visible. After the fix, full-addon/no-saved-vars talent setup dropped to under 1s and tab clicks stayed under ~1s.

### Follow-up

This moves the remaining ElvUI login cost back to `PLAYER_LOGIN` and exposes an existing ElvUI/oUF aura error (`button:SetSize` nil). Track that separately from the talent-open regression.

## Benchmark Binary

`src/bin/bench_talents.rs` — loads all addons, fires startup events, then opens/closes the talent panel 10 times. Use `RUSTFLAGS="-C force-frame-pointers=yes"` for flamegraph profiling.

## Sources

- [bench-talents.md](../../bench-talents.md) — benchmark setup and results
- [talent-onupdate-loop.md](../../talent-onupdate-loop.md) — OnUpdate loop root cause and fix

## See Also

- [[on-update-dirty]] — OnUpdate handler dirty tracking
- [[global-frame-index]] — the `__index` on `_G` lazy lookup design
