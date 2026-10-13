# Cache first-use timer-domain audit — 2026-10-13

**Scope:** read-only source and saved-profile audit. No runtime, new sampling, build, hash operation, delegation, or tracked-file changes. Canonical source: `<REPO>`, revision `621649e56d8418d21c4539b26d787ad40c73431f`. Saved evidence: `<PRIVATE_VERIFICATION_ROOT>/perf-cache-compile-sampling-621-current/`.

## Evidence reviewed

- Saved profile is Retail normal, `cpu-clock:u`, 999 Hz, DWARF 8192; ~6.7K samples, lost=0. `perf-report.stderr` / `perf-script.stderr` record repeated `addr2line ... could not read first record`; 38.48 summed displayed unknown-row self percentages is not a sample proportion.
- Runtime summary reports 1569/1569 cache hits, zero lookup misses/replay failures/stores; `compile=914.10ms`, `call=164.51ms`. The timer boundary is source-visible: `compile_lua_file` times all of `load_cached_or_compile_for_chunk`, which includes key hashing, cache initialization/read/replay, and possible source compilation. Thus that aggregate is not 914 ms of source compiler work.
- In `bytecode_cache.rs`, first cache use passes through `with_cached_bytecode` → mutex/`pack_path` → `ensure_loaded` → `load_pack_from_path` → `read_pack_within_limit` (open, metadata, `read_to_end`) → `load_pack_bytes` (validation and full entry-index construction). This call is synchronous inside the compile timer on the first cache access in a process. Even when all 1569 lookups hit, first-use work can therefore be charged to that first file's compile timer; subsequent hits reuse initialized in-memory values/index.
- `load_pack_bytes` is the only specifically reported cache leaf in the saved top-symbol rows: 0.52% self. That does **not** bound the full first-use path: read/open, hash/index allocation, lock and surrounding Rust/std calls can be elsewhere or unsymbolized. Conversely it does not prove those costs are large.
- Profile also reports VM `execute_inner` 5.90% self, table-get 3.17%, allocator free 3.11%, owned/interned strings 1.85%/1.74%, GC mark 1.21%, and undump/compiler leaves. Self percentages are sampled on-CPU attribution, not elapsed phases; parent/child inclusive values overlap.

## Strongest bounded interpretation

Yes: process-global first-cache initialization can occur inside the per-file `compile` measurement, and its cost can appear as inlined/hash/allocation/I/O/Rust-loader or unresolved frames rather than a symbol literally named `load_pack_bytes`. Saved evidence neither isolates nor rules out that contribution. The 0.52% `load_pack_bytes` self row is only that named leaf, not an inclusive cache-load measurement. The 914.10 ms aggregate must be called cache-hit-path-plus-any-compilation work, not source compilation alone. The profile does not establish that first cache load is a significant share, nor that it explains overall startup cost or any frozen CCD result.

## What an inclusive stack must prove

A useful inclusive attribution needs working symbols and an intact call chain for samples whose stack has `ensure_loaded` / `load_pack_from_path` / `read_pack_within_limit` beneath the cache lookup and loader compile boundary. The inclusive cache-first-use total should include descendant work (file open/metadata/read, byte-vector allocation and fill, pack validation/index construction, and any invoked hash/allocation helpers), while separating work that happens only once from later cache hits. It must be explicit that inclusive descendants overlap their own self rows and other aggregates; do not sum them. Hashing each chunk occurs on every lookup and is distinct from one-time pack initialization. A resolved stack should also retain the caller into `compile_lua_file` so this domain can be tied to the existing timer boundary.

A repaired `addr2line` alone is a prerequisite, not proof: native/inlined frames must actually resolve in the recorded executable and the stack must contain the relevant callers/descendants. If it does not, the cache first-use domain remains unknown.

## Conditional, nonduplicate next measurement

Only if symbolizer diagnostics are fixed and a saved profile demonstrates usable resolved inclusive caller chains: derive an offline inclusive stack breakdown for the already captured `perf.data`, grouping samples under `ensure_loaded`/`load_pack_from_path` and comparing that first-use subtree with later cache-hit/replay and VM execution. No new runtime or sample is needed for this conditional re-analysis. If symbolization remains broken or those symbols/stacks are absent, stop: current strongest result is the unresolved timer-domain limitation above; do not repeat sampling merely to get the same opaque profile.
