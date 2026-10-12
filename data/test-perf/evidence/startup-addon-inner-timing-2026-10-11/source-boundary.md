# Startup addon inner-timing boundary — 2026-10-12

**Scope:** requested source inspection only. Existing runtime evidence is `verification/startup-cost-diagnostic-current/runtime-independent-report.md`; no runtime files were re-read or used to infer counters beyond that report.

## Finding

`load_addon` already publicly returns `Result<LoadResult, LoadError>` (`src/loader/mod.rs:417-419`); `load_addon_from_toc` and saved-variable variants return the same `LoadResult` (`mod.rs:479-500`). `LoadResult` contains `name`, Lua/XML file counts, `timing: LoadTiming`, and diagnostics (`mod.rs:277-310`). `LoadTiming` already exposes IO, XML parse/process/frame sub-buckets, Lua compile/call/combined execution, SavedVariables, frame/count fields, and bytecode cache counters (`mod.rs:314-370`). Thus a caller receiving the result has useful per-invocation/per-addon inner timing without adding a production API.

The buckets are not all mutually exclusive: `lua_exec_time` includes compile + call; frame timing fields are documented subsets of XML timing; `LoadTiming::total()` sums only IO + XML parse + XML process + Lua execution + SavedVariables (`mod.rs:371-380`). XML processing explicitly subtracts independently counted child work (`src/loader/xml_file.rs:49-75`); do not infer an exclusive XML cost by subtracting nested file records from enclosing addon loads.

The result is scoped to the invocation, not a promise of exclusive ownership. `load_addon_internal` initializes one result, loads its files and optional secure replay into that timing, then returns it (`src/loader/addon.rs:193-230`). XML `<Script>`/`<Include>` paths pass the enclosing `&mut LoadTiming` into nested loading (`src/loader/xml_file.rs:1-48`, `src/loader/addon.rs:586-650`); timings therefore include work performed through those paths. Bootstrap loading also returns a `LoadResult` (`addon.rs:125-152`). A call skipped because the addon was already loaded returns the empty result (`addon.rs:193-210`). These are source-level semantics; no runtime call-graph attribution was established.

## Existing startup output and gaps

Normal Blizzard startup already receives each `LoadResult` and accumulates its timing (`src/bin/wow_sim/addon_loading.rs:100-145`). With `WOW_SIM_VERBOSE`, it prints per-addon total, XML process, Lua execution/compile/call, and frame count (`addon_loading.rs:117-145`). Otherwise the captured stdout has an aggregate Blizzard summary, not per-addon timings. The only `Slowest addons:` printer is for a supplied `AddonTiming` list (`src/bin/wow_sim/addon_loading/slowest_addons.rs:3-19`); the startup path inspected does not establish that such a per-addon list is populated or printed. Current diagnostic evidence explicitly reports the slowest enclosing addon and file rows, but those rows are separate instrumentation scopes and not equivalent to `LoadResult` buckets.

The exact runtime capture was already parsed/inspected in the supplied report: stdout 382,510 bytes/225 JSON lines, stderr 8,776 bytes/138 lines; it reports no `Slowest addons:` output and describes phase/file records, not `LoadTiming` per-addon fields. Main rejects the disabled-Blizzard claim: the wrapper explicitly loaded220 discovered Blizzard addons. `WOW_SIM_NO_ADDONS=1` does not negate those calls. Missing inner fields arise because the wrapper did not retain `LoadResult.timing`, not because Blizzard loading was disabled. Do not claim those current streams contain per-addon inner timing.

## Boundary conclusion

No new loader measurement boundary is required to obtain per-addon inner buckets: existing return values expose them. The missing boundary for a **runtime diagnostic** that wants those existing fields is capture/retention of each startup `LoadResult.timing` (including addon identity and load kind) in the diagnostic's addon-load observation. This is a missing diagnostic observation, not missing production instrumentation. No claim that this is absent from all other diagnostic paths; only the described current diagnostic and inspected stdout/stderr were considered. If the requested boundary means an additional timing primitive beyond `LoadTiming`, result available in verbose startup, and existing 220 addon-load / 1,497 file scopes, the answer is **NONE** from this evidence.

**Proof level:** source inspection for public return fields, bucket definitions, and known startup logging; existing independently authenticated runtime report for current stream sizes/rows/slowest records/configuration. No new execution, instrumentation, build, test, or native/root-cause claim.
