# intern_string Call-Site Ranking

Ranked `LuaState::intern_string` calls during release startup (`lua-errors`, full Blizzard UI, `--no-saved-vars`) to decide which literals are worth migrating to `intern_string_static`. Identifies the top offenders and documents a rilua bug that blocks migrating the registry-set path today.

## Method

Added `intern-stats` feature to rilua (see `src/vm/intern_stats.rs`). When enabled, `Gc::intern_string` bumps a `Mutex<HashMap<Vec<u8>, u64>>` counter keyed by the interned bytes. `wow-ui-sim` exposes the same feature flag; `lua_errors::print_intern_stats` dumps the top 40 strings and totals on exit.

Build: `cargo build --bin wow-sim --features intern-stats`
Run: `WOW_SIM_NO_SAVED_VARS=1 LD_LIBRARY_PATH=$PWD/target/debug ./target/debug/wow-sim lua-errors 2>stats.log`

## Ranking (startup, --no-saved-vars)

- **Total intern_string calls**: 1,250,287
- **Unique strings**: 118,751

Top 20 by call count (all are compile-time literals unless noted):

| Rank | Calls | String | Notes |
|-----:|------:|--------|-------|
| 1 | 286,070 | `__rilua_frame_refs` | `FRAME_REFS_KEY` — `frame_ref_cache` is on every Rust→Lua handle lookup |
| 2 | 73,230 | `string` | Lua stdlib `type()` return value |
| 3 | 49,100 | `__scripts` | `SCRIPTS_KEY` — `get_script`/`set_script`/`remove_script` every handler I/O |
| 4 | 47,072 | `__rilua_frame_fields` | `get_or_create_frame_fields` — once per frame lookup |
| 5 | 47,042 | `__rilua_frame_mt` | `attach_frame_metatable` — once per frame creation |
| 6 | 14,493 | `next` | Lua `next()` global lookup |
| 7 | 13,455 | `table` | `type()` return value |
| 8 | 10,479 | `function` | `type()` return value |
| 9 | 6,674 | `__index` | metatable key |
| 10 | 5,941 | `number` | `type()` return value |
| 11 | 5,714 | `OnLoad_Intrinsic` | XML intrinsic init dispatch |
| 12 | 5,517 | `height` | XML attribute parser |
| 13 | 5,516 | `width` | XML attribute parser |
| 14–22 | 5,463 each | `elementName`, `filename`, `rightTexCoord`, `tilesHorizontally`, `leftTexCoord`, `rawSize`, `tilesVertically`, `bottomTexCoord`, `topTexCoord` | XML nine-slice / texture attributes |
| 23 | 5,317 | `e` | (single-char, unknown caller — likely `tostring` for `1e-6`-style numbers) |
| 24 | 4,988 | `nil` | `type()` / `tostring(nil)` |
| 25 | 4,891 | `__event_individual` | Event dispatch registry key |
| ... | ... | single/double-char letters (`r`, `a`, `t`, `n`, `o`, `m`, `U`, `I`) | Likely `tostring` of numbers or iteration variables |

Two dynamic strings appear in the top 40: `36821_OnAttributeChanged` (2,899 calls) and `36377_OnAttributeChanged` (2,194). These are `<widget_id>_<handler_name>` keys in `__scripts`; the widget ID is runtime data, so these cannot be migrated to `intern_string_static`.

## Migratable candidates

Call sites that reach `intern_string` via a `&'static str`:

| Call site | Dominant string | Count |
|-----------|-----------------|------:|
| `rilua_methods::frame_ref_cache` | `__rilua_frame_refs` | 286K |
| `rilua_methods::attach_frame_metatable` | `__rilua_frame_mt` | 47K |
| `rilua_methods::get_or_create_frame_fields` → `registry_table_or_create(..., "__rilua_frame_fields")` | `__rilua_frame_fields` | 47K |
| `rilua_script_helpers::{get,set,remove}_script` → `registry_table(..., SCRIPTS_KEY)` | `__scripts` | 49K |
| `text_attribute_event::*` → `registry_get(..., "__event_individual" / "__event_all")` | `__event_*` | 7K+ |
| `rilua_methods::{registry_get, registry_set, registry_table_or_create}` | all `&'static str` keys |  |

The `registry_*` helpers route every call site listed above through a single `intern_string(key.as_bytes())`. Migrating those three functions to `intern_string_static` would eliminate most of the top 5.

## Root cause of the original breakage

`intern_string_static` inserts its newly-interned string into `static_intern_cache`, but `mark_gc_roots` only iterates that cache at cycle start (the Pause→Propagate transition via `mark_roots`). If the insert happens mid-Propagate, the new string is still the pre-flip current-white; the atomic flip turns that colour into "other white" = dead; sweep frees the slot and `sweep_dead` removes the ref from the string bucket. The cache is left pointing at a freed object.

A later plain `intern_string` for the same content does a bucket lookup, doesn't find the (now-removed) ref, and allocates a **new** `GcRef`. Writes that used the cached static ref and reads that used the plain ref therefore land on different keys in the same table, and every Blizzard call site that registered a script handler or metatable before the simulator caught up ended up firing on a frame without methods.

Traced end-to-end with a diagnostic in `raw_set`: stored ref was `GcRef(9106)`, later readers saw `GcRef(57980)` for the same bytes. The string buckets confirmed 9106 was pulled out between the write and the read.

### Fix (rilua)

`intern_string_static` now colours the newly-interned ref `Black` when the GC is in Propagate / SweepString / Sweep / Finalize. Black survives the current cycle's sweep; the next cycle's `mark_gc_roots` takes over as usual.

Regression test: `intern_string_static_mid_cycle_survives_sweep` (vm/state.rs). Fails without the fix, passes with it. All 686 rilua tests pass.

## Applied migration

`wow-ui-sim` now routes these through `intern_string_static`:

- `rilua_methods::{registry_get, registry_set, registry_table_or_create}` (`key: &'static str`)
- `rilua_methods::attach_frame_metatable` (`b"__rilua_frame_mt"` direct)
- `rilua_script_helpers::{registry_table, registry_table_or_create}` (`key: &'static str`)
- Fan-out callers (`precompiled::store_precompiled`, `rilua_utility_system_spell::set_registry_bool`, etc.) tightened to `&'static str` params.

Intern counter: **1,250,287 → 1,096,266 (−12%)** in the cold-startup path. Release wall time: **1.18s → 1.15s** median (n=5). Smaller than the raw call-count delta suggests because `intern_string_static` hits are themselves fast but not free, and we only migrated sites whose keys are known `&'static`.

## Post-migration perf re-profile

After landing the static-intern migration, re-profiled release startup on the
same workload (`wow-sim --no-saved-vars lua-errors`) with:

```bash
perf record -o /tmp/intern-reprofile.perf.data -F 997 -g --call-graph dwarf -- \
  env LD_LIBRARY_PATH="$PWD/target/release" ./target/release/wow-sim --no-saved-vars lua-errors
perf report -i /tmp/intern-reprofile.perf.data --stdio --no-children \
  -F overhead,period,symbol
```

`cargo flamegraph` was attempted first, but the `sudo` path could not see
`libiced_dynamic.so`; the saved `perf` data still gives the same flat profile
numbers the flamegraph would be built from.

Current startup profile:

- Total sampled event count: **6.03B cycles** (`1966` samples)
- `rilua::vm::state::Gc::intern_string`: **179.5M cycles (2.98%)**
- `rilua::vm::string::StringTable::intern_hashed`: **169.2M cycles (2.81%)**
- inline `lua_hash`: **~4.8M cycles (0.08%)**

Interpretation:

- The old startup note in `PLAN.md` had string interning / `lua_hash` as an
  early headline hotspot at roughly **25% / 136M** samples.
- Post-migration, the **hash primitive itself is basically gone** as a
  bottleneck. The remaining interning cost is now the surrounding
  bucket-walk / dedup work inside `intern_string` and `intern_hashed`.
- `frame_ref_cache` (`__rilua_frame_refs`, 286K calls/startup) has since been
  moved to the hot metatable-key registry. A 2026-05-19 retry in an isolated
  worktree no longer reproduced the old `OnLoad` cascade, and full addon
  `lua-errors` returned `[]` with the migration applied.

### Resolved: `rilua_methods::frame_ref_cache`

`frame_ref_cache` was the single biggest intern call site (286K calls of
`__rilua_frame_refs`). Earlier attempts to use the pointer-keyed hot-literal
path caused broad "attempt to call method 'OnLoad' (a nil value)" failures, so
the call site stayed on content-keyed interning. After the rilua GC/string fixes
landed, the simulator now uses `hot_metatable_key(..., RILUA_FRAME_REFS)` for
this registry lookup.

The same retry also made static registry-key interning safe for the script
registry tables. `__scripts`, `__scripts_pre`, `__scripts_post`,
`__on_update_scripts`, and `__on_post_update_scripts` now use
`intern_string_static` through `registry_key_ref`, removing roughly another
222K startup intern calls without changing handler-key semantics.

Follow-up caller tracing showed the largest remaining content-keyed caller was
the simulator's WoW `type(v)` override (`utility_system_spell::type_fn`), not
rilua's base `type`. Switching those static type names to `intern_string_static`
removed roughly 688K more startup intern calls: total `intern_string` traffic
fell from about 2.33M to about 1.66M, and `"string"`, `"table"`, `"function"`,
`"number"`, and `"nil"` disappeared from the top 40.

The event hlist membership key `_s` was another static simulator-owned key.
Switching `rilua_hlist_set` to `intern_string_static(b"_s")` removed roughly
28K more startup intern calls and dropped `_s` out of the top 40.

## Follow-ups

- **rilua**: reproduce the mismatch in a minimal test case, then either fix `intern_string_static` or document the hazard (e.g. forbid `intern_string_static` for keys later looked up via plain `intern_string`).
- **wow-ui-sim**: rerun `intern-stats` after the `__rilua_frame_refs` migration
  and rank the remaining hot literal candidates.
- **wow-ui-sim**: script handler keys are still synthesised as
  `format!("{widget_id}_{handler_name}")`. The per-handler suffix
  (`_OnAttributeChanged`) could be a shared `&'static str` if we switch to a
  two-level key table (`scripts[widget][handler]`).

## Sources

- `src/vm/intern_stats.rs` (rilua, `intern-stats` feature) — counter module
- `src/lua_errors.rs` `print_intern_stats` — dump path
- `src/lua_api/rilua_methods.rs` — `frame_ref_cache`, `attach_frame_metatable`, registry helpers
- `src/lua_api/rilua_script_helpers.rs` — `registry_table`, script storage
- `/tmp/intern-reprofile.perf.data` + `perf report --stdio` — post-migration release profile

## See Also

- [[table-rehashing]] — same instrumentation pattern, different hot path
- [[layout-profile]] — companion profiling task
