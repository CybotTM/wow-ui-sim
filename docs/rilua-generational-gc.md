# Rilua Generational GC Port

Plan to port Lua 5.4's generational GC to rilua. Addresses the "stable-root re-scan" cost: 209k+ entries across `_G`, `__rilua_frame_refs`, `__rilua_frame_fields`, `__secureenv`, `__scripts` get remarked on every GC cycle even though 95%+ never change after bootstrap.

## Measured motivation

From `/tmp/claude/perf.data` (release `lua-errors`, --no-addons --no-saved-vars, 2026-04-17):

- `Gc::traverse_table` self-time: 2.55% (50M cycles)
- `Gc::mark_value`: 1.19%
- `Gc::mark_gc_roots` (triggered once per cycle, walks registry): measurable tail
- Total GC category: ~6% of startup cycles

Top traversed tables (measured via `dump-tree --exec-lua` walk of `_G` + registry):

| Table | Entries |
|---|---|
| `_G` | 66,331 |
| `REGISTRY.__rilua_frame_refs` | 47,333 |
| `REGISTRY.__rilua_frame_fields` | 47,333 |
| `_G.__secureenv` | 28,327 |
| `REGISTRY.__scripts` | 20,713 |

The bootstrap subset of `_G` (built-ins, `Enum.*`, mixin definitions) is ~50k+ entries, all stable after addon load finishes.

## Why not Go-style, not "immortal roots"

- **Go**: concurrent tri-color on separate threads. Lua VM is single-threaded; doesn't map. Go is also not generational — won't solve "walks stable roots every cycle."
- **Immortal roots**: require a dual invariant (pin the table, pin every value it transitively reaches). Every addon that writes to `_G` breaks the invariant. Abandoned.

Lua 5.4 generational is the right template: per-object age, "gray again" list for old→young writes, minor collections skip old objects entirely.

## Current rilua state

Source: `~/Repos/rilua/src/vm/gc/` — 2,980 LOC total.

- Tri-color incremental, Lua-5.1-shape:
  - `Color::{White0, White1, Gray, Black}` (`arena.rs:35-48`)
  - `GcPhase::{Pause, Propagate, SweepString, Sweep, Finalize}` (`collector.rs:102`)
  - `barrier_back` for black→white writes
- Per-object metadata today is just `Color`. No age field.
- `mark_gc_roots` seeds the gray queue from the registry each cycle (`collector.rs:1271` transitions `Pause → Propagate`).
- `traverse_table` is at `collector.rs:410` — the per-element `self.tables.get(r)` re-borrow is a separate optimization, worth doing independently (see "Quick win" below).

80% of the machinery is there. The port adds age state + a second collection mode.

## Design (Lua 5.4 mapping)

### Age states

Lua 5.4 uses 7 age states packed alongside color (`lgc.h`):

- `G_NEW` — just allocated, survived 0 collections
- `G_SURVIVAL` — survived 1 minor collection
- `G_OLD0` — survived enough to promote, awaiting next major
- `G_OLD1` — promoted, one minor cycle passed
- `G_OLD` — fully old
- `G_TOUCHED1`, `G_TOUCHED2` — old object was the target of a barrier, must be revisited

Rilua mapping: add `age: u8` (3 bits) next to `color` on each arena entry. Packs into the existing 4-byte header. No size regression.

### Collection modes

Add `GcMode::Incremental` (current behavior) and `GcMode::Generational`. Switchable at runtime via an API call (mirrors Lua 5.4's `collectgarbage("generational")`).

Generational mode dispatches two collection shapes:

- **Minor**: traces only objects currently `G_NEW` or `G_SURVIVAL`, plus the "gray again" list (`TOUCHED1`/`TOUCHED2` olds). Sweeps only young. Fast — bounded by young-object churn, not heap size.
- **Major**: equivalent to today's full incremental cycle. Promotes `OLD0` → `OLD1` → `OLD` as objects survive. Resets `TOUCHED*` back to `OLD`.

Heuristic (from Lua 5.4 `genStep`): after a major, track `GCmajorminor` = ratio of live-size-after-major to young-allocation-quota. Run minors until quota spent, then a major.

### Barrier changes

Existing `barrier_back` already handles black-writes-white. For generational, when a `G_OLD*` object has a pointer installed to a non-old object, move the old object to the "gray again" list:

```rust
fn barrier_gen(&mut self, src: GcRef<_>, target: Val) {
    if is_old(src) && is_young(target) {
        set_age(src, G_TOUCHED1);
        gc_state.gray_again.push(src);
    }
}
```

Piggybacks on the existing barrier call sites — just one extra branch.

### Mark phase changes

`mark_gc_roots` today seeds gray from registry + main thread. Under minor:

- Walk registry as before, but for each reached object: if age is `G_OLD` and it's not on `gray_again`, skip descendant traversal (its subtree is trusted-reachable until a barrier says otherwise).
- Still mark it black so the sweep doesn't collect it, but don't traverse its fields.

The skip is the win: 50k stable `_G` entries stop being re-walked on every minor.

### Sweep phase changes

Under minor, sweep iterates only the "young" sublist of each arena. Simplest way: maintain two linked lists per arena (young chain + old chain) as Lua 5.4 does. Objects move between lists during promotion.

Alternative (simpler for rilua's flat `Vec<Entry>` arena): scan the whole arena but `continue` on old entries. O(N) with early-out, vs O(young_count). Probably fine given rilua arenas are already compact.

## Phases

### Phase 0: Quick win (do first, independent of gen-GC)

- [ ] Hoist `self.tables.get(r)` out of the two `traverse_table` loops (`collector.rs:438-460`). Collect values into `SmallVec<[Val; 32]>` under one borrow, then release and call `mark_value` in a second loop. Expected: ~50M fewer arena lookups (2.5% of startup cycles).

No semantic change. Independent of the generational work below.

### Phase 1: Age field plumbing

- [ ] Add `age: u8` to arena `Entry` (`arena.rs`). 3 bits used, 5 reserved.
- [ ] Add `Age` enum mirroring Lua 5.4 (`G_NEW`/`G_SURVIVAL`/`G_OLD0`/`G_OLD1`/`G_OLD`/`G_TOUCHED1`/`G_TOUCHED2`).
- [ ] `alloc` sets age to `G_NEW`.
- [ ] Helpers: `is_old(age)`, `is_young(age)`, `advance_age(age)`.

No behavior change yet — field is written but not read.

### Phase 2: GcMode switch

- [ ] Add `GcMode::{Incremental, Generational}` to `GcState`.
- [ ] `singlestep` dispatches on mode; generational path is a stub that delegates to incremental for now.
- [ ] Test: `collectgarbage("generational")` flips mode, collection still works.

### Phase 3: Promotion & minor collection

- [ ] On each major cycle, advance ages: `G_NEW → G_SURVIVAL → G_OLD0 → G_OLD1 → G_OLD`.
- [ ] Add minor-collection path: gray queue seeded from `gray_again` + objects touched by allocator since last minor. Skip descendant traversal on `G_OLD*` except `TOUCHED1/2`.
- [ ] Sweep young only during minor.
- [ ] Heuristic: alternate minor/major based on young-allocation pressure (Lua 5.4 `luaC_genStep`).

### Phase 4: Gen-barrier

- [ ] Extend `barrier_back` call sites: if src is `G_OLD`, set age to `G_TOUCHED1` and push to `gray_again`.
- [ ] Reset `TOUCHED*` → `G_OLD` at end of major.
- [ ] Test: old table mutated mid-cycle → new value is reachable after next collection.

### Phase 5: Measurement + cutover

- [ ] Re-profile with generational enabled by default during wow-sim startup.
- [ ] Target: `traverse_table` samples drop 80%+ (skip `_G`, `__rilua_frame_refs`, etc. after first major).
- [ ] If wins confirmed, make `Generational` the rilua default for embedders that opt in.

## Risks / open questions

1. **Finalizers in minor collections**: Lua 5.4 runs `__gc` finalizers on young objects during minor, skips old. Rilua has no `__gc` on userdata yet — but the tables arena has finalizers for weak tables. Need to confirm weak-table handling during minors.
2. **String interning table**: rilua's `StringTable` is indexed by hash-bucket, not treated as a normal GcRef arena. It's swept separately (`SweepString` phase). The generational work leaves string handling alone — strings either survive forever (common for method-name literals) or get swept during full sweeps. Not on the generational path.
3. **Gray-again list sizing**: if addon code mutates `_G` heavily post-bootstrap (e.g., `_G["MyGlobal"] = someFrame`), every such write moves `_G` to `gray_again`. Worst case this reverts to incremental-cost per cycle. Needs measurement — the prediction is that addon code mostly writes to its own tables, not `_G` directly, so `_G` should stay stably old.
4. **Generation assignment for `_G`**: `_G` is created before any addon loads. A single major collection before addon-load would promote it to `G_OLD` immediately. Need to ensure wow-sim bootstrap triggers at least one major before `handle_addon_load_start` fires.
5. **Rilua fork divergence**: wow-sim is already on a rilua fork (`wow-ui-sim-fork`, per PLAN.md String Interning). The gen-GC port would be additional fork-local commits until upstreamed.
6. **Test coverage**: rilua has 684+ tests for the current GC. Generational mode needs its own test matrix — minor correctness, barrier completeness, promote-during-cycle, finalizer ordering. Probably 30-50 new tests.

## Non-goals

- Concurrent collection (would require multi-threaded rilua; out of scope).
- Compacting / moving objects (rilua arena is non-moving by design).
- Full Lua 5.4 GC feature parity (emergency collection, `__close`, etc.) unless needed.

## Alternatives considered

- **Bigger GC step / pause multiplier**: reduces collection frequency, doesn't reduce per-collection work. Helps wall time but leaves the 6% ceiling.
- **Cache a "clean" flag per table**: mark-only-if-written-since-last-cycle. Violates tri-color invariants; needs full barrier tracking anyway — at which point you've built generational.
- **Move internal bookkeeping off the Lua heap**: explored and rejected — `__rilua_frame_refs` values are Lua values (userdata), must participate in GC for correct lifetime.
