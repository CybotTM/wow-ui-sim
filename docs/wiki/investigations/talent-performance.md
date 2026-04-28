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

## Benchmark Binary

`src/bin/bench_talents.rs` — loads all addons, fires startup events, then opens/closes the talent panel 10 times. Use `RUSTFLAGS="-C force-frame-pointers=yes"` for flamegraph profiling.

## Sources

- [bench-talents.md](../../bench-talents.md) — benchmark setup and results
- [talent-onupdate-loop.md](../../talent-onupdate-loop.md) — OnUpdate loop root cause and fix

## See Also

- [[on-update-dirty]] — OnUpdate handler dirty tracking
- [[global-frame-index]] — the `__index` on `_G` lazy lookup design
