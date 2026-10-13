# Offline perf analysis — source 621

**Input:** `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-removal-green-20261013T193523870575Z/runtime-perf-cache-compile-sampling/perf.data` (56,986,476 bytes), recorded for retail normal `wow-sim --no-addons --no-saved-vars lua-errors`. Analysis only; no runtime, sampling, build, authentication, or delegation.

## Evidence

- perf 7.2.10-1; recorded kernel 7.2.9-arch1-1 (host note supplied kernel 7.2.10); event `cpu-clock:u`, 999 Hz, DWARF 8192. Perf report: 6K samples, approximate event count 6,721,721,715, lost samples 0.
- `stderr` was read in full (173,668 bytes; 2665 lines). Runtime says: `[  5.200s] Blizzard addons loaded in 4.65s (io=150.10ms xml=333.91ms xmlproc=2.66s frames⊂xmlproc=2.65s lua=1.08s [compile=914.10ms call=164.51ms] gc=deferred, bytecode cache: 1569/1569 hits (100%, lookup_miss=0 replay_fail=0 stored=0 store_fail=0))`
- Verbose phase result: compile 914.10 ms, call 164.51 ms, XML processing 2.66 s, frame setup 1.25 s, finalize 3.12 s. Cache result: 1569/1569 hits (100%), no lookup misses/replay failures/stores. The `compile` timer may include cache replay/undump work; it does not mean all 914 ms was source compilation.
- Top relevant self sample rows: `rilua::vm::execute::execute_inner::<false>` 5.90%; `try_plain_table_get_ref` 3.17%; `StringTable::intern_hashed_owned` 1.85%; `intern_hashed` 1.74%; `collect_table_hash_mark_values` 1.21%; `cfree` 3.11%; `malloc` 1.18%; `realloc` 0.09%; `Lexer::scan` 0.61%; bytecode `load_pack_bytes` 0.52%. Full report: `perf-report.stdout`; complete script: `perf-script.stdout`.
- Symbolization diagnostics preserved in `perf-report.stderr` and `perf-script.stderr`: repeated `addr2line ... could not read first record`. Perf report contains unknown rows (combined displayed self percentages 38.48%, not a sample proportion). DWARF chains have incomplete symbol attribution.

## Assessment

Cache packing has a measurable but small visible leaf (`load_pack_bytes`, 0.52% self); no named `ensure_loaded`/cache-phase timer was recovered. Cache-backed bytecode was used, yet the loader's aggregate `compile` timer reports 914.10 ms; perf sampling separately shows lexer/compiler plus undump/replay work. Avoid treating that timer as source compile alone. VM execution/table lookup have larger named self rows. Thus evidence favors substantial Lua VM/runtime execution and table access over cache-pack loading as the sampled CPU focus; compilation/replay and allocation/GC are present but do not establish exclusive dominance or addon-specific causality.

These are userspace CPU samples/overlap, not elapsed time, kernel/off-CPU cost, or exclusive addon attribution. One run cannot establish frozen CCD repair causality. Native/render/full-parent behavior remains unproved. No fix proposed without a more specific observed cost.
