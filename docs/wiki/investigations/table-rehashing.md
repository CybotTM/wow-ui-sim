# Table Rehashing

Profile investigation: why rilua's `Table::rehash` was still sampling ~11M times on startup despite frame-table pre-sizing. Root cause is not frame tables — it's addon-created Lua tables (`OP_NEWTABLE` with zero size hints) gaining their first few entries.

## Symptoms

After the main startup optimizations (pre-sized frame tables to 64 hash slots, frame-ref cache at 4096), `cargo flamegraph` still showed `Table::rehash` as ~11M samples during `wow-sim lua-errors --no-saved-vars`. Pre-sizing did not eliminate the hot path.

## Instrumentation

Added `rehash-stats` feature to rilua. `src/vm/rehash_stats.rs` holds static `AtomicU64` counters; `Table::rehash` bumps them at the top. `wow-ui-sim` enables the feature via its own `rehash-stats` feature and `lua_errors::print_rehash_stats` dumps the snapshot on `lua-errors` exit.

Build: `cargo build --bin wow-sim --features rehash-stats`
Run: `WOW_SIM_NO_SAVED_VARS=1 LD_LIBRARY_PATH=$PWD/target/debug ./target/debug/wow-sim lua-errors 2>stats.log`

## Findings

Startup profile, `--no-saved-vars`:

```
total=97339 from_empty=37672 grow=56952 frame_backed=1600 nonframe=95739

by new hash size (2^i):
  size      0: 39586   ← array-only resizes
  size      2: 16203
  size      4: 13373
  size      8: 12047
  size     16: 10371
  size     32:  2670
  size     64:  1753
  size    128:   923
  size    256:   346
  size    512+:   71

resizes to hash=0, grouped by old hash size:
  from      0: 17650   ← pure-array growth (t[N+1] past array.len)
  from    1-128: 23
```

### Interpretation

- **frame_backed / nonframe = 1600 / 95739 → frame pre-sizing is doing its job.** Only 1.7% of rehashes come from `create_frame_table`-backed tables; the remaining 98% are plain Lua tables.
- **81% of grow-to-nonzero rehashes land at hash size ≤ 16.** The tables that rehash are tiny.
- **37K rehashes `from_empty`.** First string key hits `Table::new()` / `NewTable(0,0)` → `new_key` errors "hash empty" → rehash allocates.
- **17.6K "array-only" rehashes `from 0 to 0`.** Classic `local t = {}; for i=1,N do t[i]=i end` — every time the loop index passes the current array length, `raw_set_impl` falls through to `new_key`, fails (hash empty), and `rehash` decides `nh_size == 0` and extends the array. Cheap per call (no hash rebuild) but still runs the full `num_use_*` pass and `array.resize`.
- **About 29K grow-rehashes land at size 2 or 4.** These would be eliminated if empty-hint `NewTable` started with 4 hash slots instead of 0.

## Root cause

rilua's `OP_NEWTABLE` honours the Lua compiler's size hint literally. Addon code shaped like `local t = {}; t.x = v; t.y = w` compiles to `NewTable(0, 0)`, producing a table with zero array slots and zero hash slots, which forces a rehash on the very first `raw_set`.

Rehash is cheap individually but frequent: 97K calls × hundreds of cycles each matches the observed ~11M profile samples.

## Candidate fixes

1. **Minimum hash on empty `NewTable` hint.** **Applied in rilua**: `vm::execute.rs OpCode::NewTable` — when `narray == 0 && nhash == 0`, allocate 4 hash slots. Measured: 97,340 → 69,105 rehashes (−29%); release startup 1.31s → 1.21s (−8%, n=5 each). Cost: ~32 B per table that stays empty.
2. **Short-circuit array growth.** In `raw_set_impl`, when key is an integer `== array.len() + 1` and hash is empty, extend the array inline instead of going through `new_key` → `rehash`. Eliminates the 17.6K `from=0, to=0` rehashes. Requires rilua change.
3. **Sized `Table::new()` in hot wow-ui-sim sites.** Lower leverage — frame-backed rehashes are already only 1.6K — but the sites in `rilua_text_attribute_event.rs`, `rilua_timer_layout.rs`, `rilua_script_helpers.rs` create tables per event/timer tick; sizing them to 4–8 would remove the first rehash on each.

## Post-fix profile (after #1)

```
total=69105 from_empty=23182 grow=42957 frame_backed=1602 nonframe=67503
by new hash size (2^i):
  size      0: 25344
  size      2:  8523   (was 16203)
  size      4:  7055   (was 13374)
  size      8: 12049   (unchanged — tables that grow past 4 still rehash)
  ...
```

Buckets 2 and 4 shrank as predicted; bucket 8 is unchanged because tables that eventually hold 5–8 entries still rehash from 4 → 8. To eliminate those, either start at 8 (doubles memory cost) or adopt fix #2.

## Sources

- `src/vm/table.rs` `Table::rehash` / `raw_set_impl` — rehash trigger path
- `src/vm/rehash_stats.rs` — instrumentation module (rilua, `rehash-stats` feature)
- `src/lua_errors.rs` `print_rehash_stats` — stats dump on `lua-errors` exit
- Baseline: `PLAN.md` "Current Profile (1.7s baseline) — Table operations: ~15% (raw_set 53M, main_position 49M)"

## See Also

- [[startup-createframe-profile]] — earlier CreateFrame profiling
- [[talent-performance]] — another rehash-adjacent hot path
