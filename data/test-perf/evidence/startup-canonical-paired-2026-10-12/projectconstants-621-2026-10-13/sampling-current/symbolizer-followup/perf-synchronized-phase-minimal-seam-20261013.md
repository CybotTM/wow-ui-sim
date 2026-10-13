# Synchronized ProjectConstants phase sampling: minimal seam

**Read-only design assessment, 2026-10-13.** No source, build, runtime, profiling, installation, system, or delegation actions. Source paths below are canonical `<REPO>`.

## Prior evidence boundary

The two requested handoffs establish that capture `6216715` has 92.45% unknown symbol attribution and does not align perf samples to the per-file `ProjectConstants.lua` interval. The earlier `compile=696.0ms` is an aggregate loader timer, not proof of compiler-only duration; first cache initialization can be nested in it. Nothing here attributes the unknown samples or establishes a performance cause or precision.

## Smallest useful simulator-side seam

Add one opt-in, structured phase trace at the existing Rust loader/cache boundaries, without touching Blizzard/vendor Lua:

- `src/loader/lua_file.rs::load_lua_file`: emit per-file begin/end markers around the existing stages, carrying normalized `chunk_name` (therefore ProjectConstants identity), phase, and boundary timestamps. The existing `compile_lua_file` wall interval encloses hash, cache lookup/replay, and possible source compilation; `execute_lua_file` encloses the VM call plus its surrounding Rust setup/teardown. Preserve those coarse intervals and expose their distinct names rather than labeling compile time “compiler.”
- `src/loader/bytecode_cache.rs::with_cached_bytecode` and `ensure_loaded` / `load_pack_from_path`: mark the cache lookup interval, and only the first `ensure_loaded` initialization interval; distinguish it from later hits. This places actual pack open/read/validation/index construction within the first-init interval. `content_hash` occurs before `with_cached_bytecode` in `lua_file.rs`, so it remains attributable to the encompassing per-file compile interval unless separately bounded; do not imply the cache marker contains it.
- In `load_cached_or_compile` in `lua_file.rs`, identify cache replay (`compile_with_rilua` on cached bytes) versus source compilation (`compile_from_source`), since both currently run inside the same compile timer. These boundaries are necessary to separate cache lookup, replay/undump, and VM source compilation causally.

Each marker needs monotonic start **and end** timestamps from a single clock domain, stable phase/file labels, and a shared run identifier. An event pair is sufficient; durations alone cannot align samples. Emit only when explicitly enabled. The existing `Instant` elapsed timers are monotonic but do not expose a portable raw timestamp; retain their duration semantics while adding a timestamp source whose relationship to perf is proven, not assumed.

## Required perf alignment premise

The marker timestamps must be directly comparable to `perf` sample timestamps for the same process and recording: same clock domain and epoch, or a measured, documented conversion with bounded error. A Rust `Instant` value itself is opaque and must not be presumed equivalent to perf `cpu-clock` timestamps. Alternatively, marker events recorded into the same perf event stream supply the required common timeline. Include PID/run identity and verify the selected perf clock/event timestamp basis and conversion before classifying samples. Bracketing sample timestamps by markers then bounds phase samples; it does not turn sampled CPU time into wall time or fix unresolved symbols.

## Unproven assumptions / limits

- Whether the intended perf event timestamp clock can be read from Rust with compatible semantics, or whether same-stream marker events are available in the authorized capture workflow, is unverified.
- Exact instrumentation emission overhead and timestamp-conversion error are unmeasured; do not claim millisecond precision until bounded empirically.
- Per-file scope must account for any errors/early returns so end markers remain well-formed; implementation design should use a scoped guard or equivalent, not hand-assume success paths.
- Cache initialization is process-global and may occur during the first relevant lookup; which file triggers it depends on runtime order and must be read from emitted markers, not inferred from source order or the old log.
- These markers establish temporal phase membership only. Dominance requires usable resolved inclusive stacks (or another suitable attribution); the old 92.45% unknown result remains unresolved.
