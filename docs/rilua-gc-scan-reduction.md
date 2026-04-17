# Rilua GC Scan Reduction

Plan to cut GC mark cost in rilua. Two tracks, sequenced:

1. **Freeze-after-bootstrap** — cheap, bigger single win. Eliminates `_G` and `__secureenv` (~94k entries) from every scan.
2. **Frame-table treatment** — investigate first, then pick an approach (generational, off-heap, or segmented freeze).

## Measured motivation

From `/tmp/claude/perf.data` (release `lua-errors`, --no-addons --no-saved-vars, 2026-04-17):

- `Gc::traverse_table` self-time: 2.55% (50M cycles)
- `Gc::mark_value`: 1.19%
- Total GC category: ~6% of startup cycles

Top traversed tables (measured via `dump-tree --exec-lua`):

| Table | Entries | Addressable by |
|---|---|---|
| `_G` | 66,331 | Freeze (track 1) |
| `REGISTRY.__rilua_frame_refs` | 47,333 | Frame track (needs investigation) |
| `REGISTRY.__rilua_frame_fields` | 47,333 | Frame track (needs investigation) |
| `_G.__secureenv` | 28,327 | Freeze (track 1) |
| `REGISTRY.__scripts` | 20,713 | Frame track (needs investigation) |

Track 1 alone removes **94,658 entries** from every GC mark cycle.

## Track 1: Freeze-after-bootstrap

### Insight

`_G` and `__secureenv` grow during bootstrap (Blizzard UI load, initial addon registration) and are essentially stable afterwards. Most entries are built-ins, `Enum.*` tables, mixin definitions, global constants. Addon runtime code mostly reads these; when it writes, it writes to its own tables, not to `_G`.

If we snapshot `_G` at the bootstrap boundary and refuse mutations to those keys thereafter, the GC can skip traversal entirely.

### Why naive pinning doesn't work

Pinning `_G` alone is unsound: `_G.UIParent` is a separate GC object. If `_G` isn't traversed, `UIParent` is seen as white and swept.

Correct approach: transitively pin every object reachable from `_G` in a single walk at freeze time, then forbid mutation. Without a barrier, pinned objects can't write references to non-pinned objects (the non-pinned target would be swept). Forbidding all mutation on pinned objects removes the need for a barrier and the need for any future scan.

### Design

New rilua API:

```rust
impl Gc {
    /// Freezes a table and everything transitively reachable from it.
    /// After this call:
    ///   - All reached objects are permanently Black (never swept).
    ///   - Writes to any frozen table raise a runtime error (or are silently
    ///     routed to a shadow table — see policy below).
    ///   - GC mark phase skips traversal of frozen objects.
    pub fn freeze_table(&mut self, root: GcRef<Table>);
}
```

Implementation sketch (`~/Repos/rilua/src/vm/gc/collector.rs`):

1. Walk object graph from `root`, setting a new `Flag::Frozen` bit on each arena entry. Use the same BFS shape as mark phase.
2. In `traverse_table`, `mark_value`, and sweep: check `Flag::Frozen` first and skip.
3. In `raw_set` / `raw_set_impl`: if table is frozen, either raise or forward to a live shadow (decided by wow-sim-side wiring).

### Shadow-table policy (wow-sim side)

Addons do occasionally overwrite built-ins (`_G.print = myprint`). To keep that working, wow-sim wraps `_G` with a metatable proxy at freeze time:

```lua
-- _G_live is a regular Lua table, starts empty
-- _G_frozen is the pre-freeze snapshot
-- _G proxy: reads check live first, then frozen; writes always go to live
```

`_G_live` participates in GC normally. It'll grow with addon-declared globals — probably a few hundred entries after full startup, not 66k.

Same treatment for `_G.__secureenv`.

### Non-goals for Track 1

- Runtime un-freezing (one-shot, no undo).
- Partial freezing of subtrees (can extend later; not needed for `_G`/`__secureenv`).
- Handling mutation to frozen *values* (they're whole-object frozen; `_G_frozen.Enum.ItemClass` itself can't be mutated either). Addon code that tries to add new `Enum` entries post-bootstrap will hit the shadow path.

### Phases

- [ ] Phase 1a: Rilua: add `Flag::Frozen` bit to arena entries (`~/Repos/rilua/src/vm/gc/arena.rs`). Single bit; no size regression.
- [ ] Phase 1b: Rilua: implement `Gc::freeze_table(root)` that walks transitively and sets Frozen on every reached object. Use BFS over `Val::Table`, `Val::Function`, `Val::Closure`, `Val::UserData`.
- [ ] Phase 1c: Rilua: `traverse_table`, `mark_value`, and sweep routines check `is_frozen` and early-return / skip.
- [ ] Phase 1d: Rilua: writes to frozen tables — initial policy is `raise Lua error`. Add a config toggle to switch to "silent deny" later if addon code hits this.
- [ ] Phase 1e: Wow-sim: add `_G_live` + metatable proxy. Call `gc.freeze_table(_G)` at the end of `register_globals` / after runtime_surface_bootstrap, before any third-party addon loads.
- [ ] Phase 1f: Wow-sim: repeat for `_G.__secureenv`.
- [ ] Phase 1g: Re-flamegraph. Target: `traverse_table` samples drop ~50% (94k entries removed, roughly 45% of the 209k-entry total).

### Risks

1. **Bootstrap is not a clean boundary.** Some "addons" in `Interface/BlizzardUI` load late and still register globals. Need to pick the exact freeze moment: after Blizzard UI, before user addons (`WOW_SIM_NO_ADDONS` path has no user addons — freeze works trivially; full load needs a deliberate hook).
2. **Tests that mutate `_G` mid-run.** Integration tests may set globals after bootstrap. Proxy handles it (writes go to `_G_live`), but behavior of `rawset(_G, key, value)` changes — `rawset` bypasses the metatable, and if the proxy replaces `_G` itself, callers of `rawset(_G, ...)` break unless we intercept. Audit `rawset` usages in `src/` and `tests/`.
3. **Reentrancy during freeze.** If freeze walk triggers a metamethod (e.g. iterating a table with `__pairs`), state gets weird. The walk must use raw access only (`raw_get`, no metamethods).
4. **Weak tables inside `_G`.** If any reached object is a weak table, freezing it breaks weak semantics (its values would never be collected). Need to skip weak tables and not descend into them; they stay on the normal GC path.
5. **Finalizers.** Frozen objects can't be finalized. Confirm nothing in the bootstrap graph has a `__gc` equivalent (rilua doesn't expose `__gc` on userdata today — low risk).

## Track 2: Frame-table investigation

`__rilua_frame_refs` (47k), `__rilua_frame_fields` (47k), `__scripts` (20k) don't fit the freeze model — they grow throughout runtime as frames and script handlers are created. Before picking an approach, we need data.

### Investigation questions

- [ ] What fraction of `__rilua_frame_refs` entries added during bootstrap survive to shutdown vs. get overwritten? (Hypothesis: 95%+ survive. If true, early entries are effectively "old" and a generational model would skip them.)
- [ ] How often is a frame destroyed (entry removed from the table)? Per-startup count.
- [ ] How many `__scripts` entries are added during bootstrap vs. during runtime? (Hypothesis: most are bootstrap.)
- [ ] Is there a natural segmentation? E.g., all Blizzard UI frames get IDs < N, all addon frames get IDs ≥ N. If yes, a segmented freeze ("freeze everything with id < watermark") could work.
- [ ] How expensive is it to move these tables off the Lua heap entirely? Previously rejected (they hold Lua userdata that needs GC tracing), but might be feasible if we register them as a single rooted collection with a traversal callback that reports contents without per-entry Lua-table overhead.

### Candidate approaches (ranked, pending investigation)

1. **Segmented freeze**: watermark ID set at end of bootstrap; entries with id ≤ watermark go into a frozen sub-table. Requires Track 1 to land first.
2. **Generational GC in rilua**: Lua 5.4 port (scope previously drafted in this doc). 500–800 LOC, bounded. Wins on the "95% of entries are old and stable" pattern.
3. **Off-heap storage with GC root callback**: rilua exposes a hook "here's a `Vec<GcRef<UserData>>` treated as roots, mark these and move on." Bypasses per-entry Lua table overhead. Needs rilua API work.

Decision deferred until we have data from the investigation phase.

## Alternatives considered

- **Pure generational (original plan)**: more general, but Track 1's freeze is ~10× simpler and captures 45%+ of the entry-count savings on its own. Better to ship freeze first, then measure what's left.
- **Immortal-root flag without transitive pinning**: unsound — cascade sweeps descendants.
- **Mark-only-if-modified per entry**: requires full barrier tracking, which is essentially the generational machinery.
- **Bigger GC pause multiplier**: trades heap size for less frequent collection. Doesn't touch per-collection work. Orthogonal; can stack with freeze.

## Sequencing

1. Land Track 1 (freeze). Measure.
2. Run Track 2 investigation. Gather data on frame-table growth and lifetime.
3. Pick the winning Track 2 approach based on data. Generational remains the default fallback if segmentation doesn't work out.
