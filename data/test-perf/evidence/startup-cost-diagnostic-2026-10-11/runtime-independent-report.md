# Runtime independent observation — OBSERVATIONAL_COHERENCE_PASS

Epoch `compile-20261012T040530851181Z`; actual request `runtime-request-20261012T043629253584Z.jsonl`. Bounded diagnostic only. No repair, full acceptance or handoff completion.

## Coverage

- Full raw stdout: **382,510 bytes, 225 JSON lines**; full stderr: **8,776 bytes, 138 lines**. Every JSON line parsed; complete stderr inspected. **224 phase observations + 1 final summary**, not 225 acceptance cases; acceptance cases executed by this gate: **0**.
- Exact phase order: initialization → discovery → 220 addon_load rows → postworkarounds → startup_events → final summary. Addon ordinals exactly **1–220**, names nonempty/unique in that recorded order. Full ordinal/name list and every file record retained in proof; no independently reacquired discovery/source list.
- All phase/file durations and counters nonnegative. Every adjacent before/after snapshot continuous, monotone, correct record delta and file-counter delta. Initial snapshot zero; final records/cache_hits **1,497**; cache_lookup_misses/cache_misses/cache_replay_failures **0**. Appended record slot intervals partition **[0,1497)**, each listed once. **233 repeated chunk names** represent distinct recorded slots, not a violated chunk-uniqueness requirement.
- File fields exactly chunk, compile_ns, call_ns and four cache counters. Totals: compile **442,518,444 ns**, call **165,928,710 ns**, hits **1,497**, other counters **0**; final snapshot equals record totals. Dynamic nested costs/counters are **not exclusive**; no cold-cache inference from compile_ns.
- Enclosing **3,948,015,850 ns** ≥ phase sum **3,906,798,217 ns**; residual **41,217,633 ns**. Residual includes instrumentation/snapshot/retention work, not a measured exclusive cause. Readiness/output occur outside enclosing clock under reused preflight boundary.

## Actual request and terminal boundaries

Full request read as data, AST inspected, never executed. Sole top-level expression invokes submit_runtime for this epoch; compile_probe is defined but uncalled. Submission exit **0**, exact epoch probe, profiling enabled and CASC/sound/addons/SavedVariables disabled. Admission **2026-10-12T04:36:29.265354+00:00**, load **22.37**, RAM **25.1142578125 GiB**, relevant Cargo **[]**; timestamped and latest receipts equal.

Saved terminal at **2026-10-12T04:36:56.326897+00:00**: Result=success, ExecMainStatus=0, ActiveState=active/SubState=exited, empty ControlGroup. RemainAfterExit=yes explains retained active state; not evidence of running probe. Process snapshot at **04:36:56.351472 UTC**: **[]**. CPU quota 12s/s, MemoryMax 17,179,869,184 bytes, RuntimeMax 90s, KillMode control-group are saved configured properties, not stress-tested enforcement. Main result exactly matches independently parsed **225 rows**, one summary, **sequence_completed=true, ready=true, error=null** and terminal properties/process snapshot.

## Diagnostics and measured slowest records

**0 returned phase/summary errors; 0 explicit simulator error/failure/panic/warning/budget diagnostics observed in full captured streams.** Only error-word candidate is stderr line 88: `[  3.398s] [Startup] apply_post_load_workarounds: seterrorhandler restored` — restoration log, not an error. Internal logged-error collector/callback state is not in these snapshots; no universal error-free startup claim. Ready=true is the reused probe readiness predicate, not full UI acceptance.

Within **this warm-cache, CASC=0, profiled diagnostic only**:

| Measurement | Slowest observed | Time |
|---|---|---:|
| Individual phase row | startup_events | 545,482,533 ns |
| Addon load | #58 Blizzard_UIPanels_Game | 281,181,794 ns |
| File record, compile+call | @Interface/AddOns/Blizzard_ProjectConstants/ProjectConstants.lua | 277,820,187 ns |

Slowest file decomposition: compile **277,768,489 ns**, call **51,698 ns**, cache_hits **1**, other counters **0**. These rankings do not establish root cause, loaded-ba identity, native parity, cold-cache performance or benchmark/control equivalence. Startup events is an aggregate phase; file scopes may nest. No native or acceptance conclusions.

## Reused evidence and limits

First authentication reused from compile-independent-auth.json: **349,260,352 bytes**, SHA256 **1ad974424c3f89b2e82f4a3e3e2e3380e775c0e0b6058638fd0bdefe0318dae1**. No probe/library payload read or hash. Reused compile-independent-report.md/proof.json, preflight-report.md/proof.json, readability.json and format.json; no repeated audit/check/format/build. Compile outcome COMPILED_NOT_EXECUTED describes historical compile boundary, not absence of this actual runtime. Prior failed guards/history unchanged.

Saved observations are point-in-time; no live process poll, guard refresh, source/cache/payload acquisition, network, operations, commands or delegation. Earlier library producer/source/provenance limits retained. Only runtime-independent-report.md and runtime-independent-proof.json written. All bounded comparisons pass; **broader acceptance/fix/handoff remains unproven**.
