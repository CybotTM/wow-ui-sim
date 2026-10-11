# Independent observational diagnostic proof

**OBSERVATIONAL_COHERENCE_PASS** — epoch `compile-20261011T233940578091Z`. No acceptance oracle; no project tests rerun.

## Artifact and compile

- Authenticated only the new probe once by streamed size/SHA-256: 349244544 bytes; `5a18de543b8b3420b79dc0bfd7dd720f19ba5f284134033f91a2d7870e41090b`. Matches `artifact.json`. No old executable/library byte reads.
- Compile receipt exit 0; compile stdout/stderr both empty. This proves no diagnostics for this private wrapper compilation only, not current project/library warning cleanliness. Saved rustc: Arch 1.99.0, b940084d7, LLVM 23.1.1. Submission: edition 2024, opt-level 1, clang/mold, native CPU, explicit selected rlib/dependency directory.
- Initial 233855 controller failed at existing epoch directory creation before any Rust invocation. Retry used `compile-` plus microseconds; supplied receipts contain one rustc compilation.
- Frozen `probe.rs` SHA `452ba91fdb80e6c9509ad49ea04f424a73f6551c1a5d6cd6764a9a48541e8cd5` matches submission and root probe source.

## Library/source linkage

Selected `target/debug/deps/libwow_ui_sim-65495b7501c08485.rlib`: current stat 430419488 bytes, mtime_ns 1791742016858538422. Saved Cargo compiler-artifact record independently matches the selected-library projection: one wow_ui_sim library, retail/default profile, opt-level 1, line-tables-only, debug assertions/overflow checks; no timed-signal-maps feature. Cargo source path is this repository `src/lib.rs`. Cargo epoch revision `6a4801a406c80a53bd95091f60f31e6590b90273`; wrapper submission HEAD `8c224f02eb9b368aee18bedec8dd33264a529ada`. Read-only git comparison to current `src`, Cargo.toml/Cargo.lock and the referenced test returns exit 0 and no differences. This is source-source linkage and path/stat/projection evidence, NOT hermetic inputs, authenticated library bytes, or frozen ba identity.

## Source contract/readability

Frozen `probe.rs:39-70` preserves the latency test startup: environment creation, 1024x768 game screen, profile cache directory under explicit repository root, addon discovery/loading, post-load workarounds, headless settle, identical talent toggle and visibility predicate. `:95-136,167-184` preserves 20 baseline ticks, open panel, 10 settle ticks, 24 steady ticks, each process_timers → ensure_layout_rects → public fire_on_update(0.016). Added before/after snapshots and clocks observe rather than introduce budget assertions or acceptance selectors. Setup visibility assertion remains fixture sanity, not a timing budget.

Optional stored cache is mapped to Option length (`:72-79`) and serialized as null (`:138-143`), never defaulted to zero. No direct call/exposure of pub(crate) fire_on_update_timed: env_runtime.rs:271-282 shows public fire_on_update internally invokes it. Thus whole public OnUpdate timing includes internal dispatch/layout/animation/GC etc.; this probe cannot separate those stages. Manual Rust assessment: short named helpers, explicit contextual error handling, brief borrow lifetimes and straightforward sequential clocks. No blocking readability/source contract defect; dense JSONL line is the external row contract. No automated readability/lint/check run.

## Runtime and independent raw-row arithmetic

Parent terminal receipts show success, ExecMainStatus=0, active/exited due to RemainAfterExit, RuntimeMaxUSec=1min30s, KillMode=control-group, empty ControlGroup, no remaining probe processes. Runtime admission load1 8.69 / available RAM 40.46298599243164 GiB. Only receipts inspected; no service operation or polling.

Raw stdout independently parsed: 54 rows, exact phase-local indices baseline0..19, talent_settle0..9, talent_steady0..23; 0 other stdout lines. All integer durations/counts nonnegative; all totals cover timers+layout+whole OnUpdate; residuals nonnegative. Parent summary equals independently computed aggregates exactly (including parent float representation). Authoritative exact rational means/sums/min/max and signed deltas are in `proof.json`.

| Phase | Rows | Total mean ns (exact) | Total max ns | Residual mean ns (exact) | Residual min..max ns | Timer invocations |
|---|---:|---:|---:|---:|---:|---:|
| baseline | 20 | 711085/2 | 396021 | 1973/20 | 70..181 | 40 |
| talent_settle | 10 | 28583082/5 | 52976676 | 1151/10 | 80..310 | 22 |
| talent_steady | 24 | 4860707/12 | 410048 | 2167/24 | 70..120 | 48 |

Steady minus baseline exact mean delta ns: timers_ns=6769/30, layout_ns=-149/40, onupdate_ns=739543/15, total_ns=594197/12, residual_ns=-1003/120.

Tracked baseline before/after 647; settle before {659,653}, after 653; steady before/after 653 (stored delta +6). All 108 optional visible snapshots null: absent cache, NOT zero visible frames. Tracked/cache cardinalities do not measure handlers fired. Timer totals 40/22/48 count invocations returned by process_timers, not successful completions (env_runtime.rs:231-240,504-536; callback errors can be routed separately). Timers schedule against real clock, not synthetic tick elapsed. Residual is unassigned outer measurement/interstage overhead, not an internal stage or causal attribution.

## Boundaries

Preserved supplied parent broadba **FAIL 15038/3/19/15041** as metadata only, not reauthenticated or reinterpreted. No loaded-failure causality, native parity, acceptance-test pass, or full hand-off acceptance claim. No builds, tests, checks, reruns, profiling, delegation, network, source edits, or runtime operations. Only private root `independent-report.md` and `proof.json` written.
