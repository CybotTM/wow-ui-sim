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

## Attempted migration

Tried migrating (on this branch, later reverted):

- `rilua_methods::frame_ref_cache` → direct `intern_string_static(FRAME_REFS_KEY.as_bytes())`
- `rilua_methods::attach_frame_metatable` → direct `intern_string_static(b"__rilua_frame_mt")`
- `rilua_methods::{registry_get, registry_set, registry_table_or_create}` → `key: &'static str`, internally use `intern_string_static`
- `rilua_script_helpers::{registry_table, registry_table_or_create}` → same
- Fan-out callers (`precompiled::store_precompiled`, etc.) → `&'static str` params

Build passed. Runtime intern-stats count dropped **1,250,287 → 318,410 (−74.5%)**.

**But release startup produced 22 new Lua errors** ("attempt to call method 'SetForbidden'/'IsVisible'/'RegisterEvent' on a nil value" — frames without their shared metatable). Bisected to: *using `intern_string_static` inside `registry_set` alone is sufficient to break frame metatable lookup*. Using `intern_string_static` only in `registry_get` is fine. So the written key and the read key must resolve to different `GcRef<LuaString>` values for the same content, at least sometimes.

This is surprising because `intern_string_static`'s miss path falls back to `intern_string`, which is content-deduped, so it should return the same `GcRef` as a parallel plain `intern_string` call. Not reproduced in rilua's 685-test suite. Blocking the migration until the rilua behaviour is understood — filed as a follow-up.

## Follow-ups

- **rilua**: reproduce the mismatch in a minimal test case, then either fix `intern_string_static` or document the hazard (e.g. forbid `intern_string_static` for keys later looked up via plain `intern_string`).
- **wow-ui-sim**: once the rilua side is safe, land the `registry_*` migration — the counter says it will remove ~930K intern calls from a cold startup (74% of `intern_string` traffic).
- **wow-ui-sim**: `__scripts` keys are synthesised as `format!("{widget_id}_{handler_name}")` — unavoidable. The per-handler suffix (`_OnAttributeChanged`) could be a shared `&'static str` if we switch to a two-level key table (`scripts[widget][handler]`).

## Sources

- `src/vm/intern_stats.rs` (rilua, `intern-stats` feature) — counter module
- `src/lua_errors.rs` `print_intern_stats` — dump path
- `src/lua_api/rilua_methods.rs` — `frame_ref_cache`, `attach_frame_metatable`, registry helpers
- `src/lua_api/rilua_script_helpers.rs` — `registry_table`, script storage

## See Also

- [[table-rehashing]] — same instrumentation pattern, different hot path
- [[layout-profile]] — companion profiling task
