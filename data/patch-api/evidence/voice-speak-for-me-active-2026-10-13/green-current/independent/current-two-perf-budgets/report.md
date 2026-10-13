# Current two performance budgets — saved-artifact verification

**PASS: 2 logical parent cases, 0 failures.** Each saved stdout has a child and parent recap for the same selector: **1 PASS each case, not 2**. Actual waiting launcher exit **0 each**, confirmed by saved result and launcher process `status=0/SUCCESS`. No tests rerun.

| Exact selector | Observation | Budget | Result |
|---|---:|---:|---|
| `startup_perf::full_game_startup_stays_under_budget` | **7.083142179 s** | <20 s | PASS |
| `addon_loading_perf::loading_phase_breakdown_stays_within_budgets` — XML parse | **0.20259 s** | <2.5 s | PASS |
| same case — Lua compile | **0.56432 s** | <2 s | PASS |
| same case — lifecycle | **0.83159 s** | <7 s | PASS |

Startup stderr parsed completely: **220 addon rows + 1 startup_summary**, contiguous ordinals 1–220; summary readiness true. Addon loading 5.744633025 s, post-load workarounds 0.345974281 s, startup events 0.669637698 s. These subphases do not exhaust the total (environment initialization also timed). Phase decimals are converted from authoritative two-decimal-ms log output, not invented nanosecond precision. Layers 0.05134 s and 2482 frames are observations, not extra assertions.

## Diagnostics are separate from PASS

Startup rows record **88 missing requirements across 26 rows**, **185 nil-symbol observations across 38 rows**, **0 warnings**. Counts are observations, not unique API gaps or assertion failures. No names printed. Neither CLEAN nor global API coverage proven. Phase stderr has no diagnostic records; its test does not assert loader diagnostic cleanliness. Startup name mismatches: 73 rows, not reclassified as diagnostics. Cache totals: 1497 hits; zero misses, lookup misses, replay failures, store successes and store failures. This does not establish cold-cache repeatability.

## Unchanged tests and clocks

Exact bodies and helpers compared unchanged against receipt source `a8a148e98d8dc32cf81e789fc8926830c9ed0161` (saved git comparison exit 0). Source receipt is not executable/library identity.

Startup clocks start before game-environment creation and end after Blizzard addon loading, workarounds and startup events. Readiness eval/JSON emission excluded. Readiness only asserts UIParent, PlayerFrame and IsLoggedIn. Fixture uses 1024×768 and Blizzard addon base; not personal-addon GUI startup.

Phase test accumulates loader timings with SavedVariablesManager; it does not execute the full startup workaround/event sequence. XML parse wraps parsing; Lua compile includes cached-bytecode load and inline compilation; lifecycle wraps XML OnLoad/OnShow dispatch. Nested counters are not exclusive wall-clock partitions. Phase test asserts only those three budgets, not all phase/API behavior. Timeout child is re-executed and replayed; two recaps are not two independent runs.

## Envelope and evidence boundaries

Saved argv: waiting systemd launcher, **CPUQuota=200%**, **MemoryMax=4 GiB**, MemoryHigh=3 GiB, one test thread, outer timeout 86 s; timeout helper 120 s. CPU quota is a two-CPU-time envelope, not affinity isolation. Startup service runtime 7.510 s, CPU time 6.204 s, peak 1.9G; phase runtime 3.554 s, CPU time 3.476 s, peak 1.4G; reported swap 0B each. Service durations are not assertion clocks.

CASC/sound not forcibly disabled in saved argv; HOME only explicit environment override. Recorded selected ambient CASC, sound, saved-vars, addon, timeout-fixture, bytecode-cache and test-thread controls all null (unset), not proof of effective subsystem activity. Full admission pressure/memory stats and all numerical addon rows retained in proof.json.

Admission timestamps **October 13, 2026, 15:04:01.193887 UTC** and **15:04:08.731362 UTC**. File-stat times in proof.json explicitly UTC; durations timezone-independent. Private unit/path/addon/symbol names omitted. File stats establish receipt metadata only, not build/runtime identity.

Protected GUI and unrelated workers not stopped (recorded context). Private `/tmp` can isolate gate inode: no global exclusivity, no-overlap or historical CCD comparability claim. Cache warmth, historical quota/environment equivalence and exact overlap unknown. No causal optimization, deterministic repeatability, full-suite or native claim. Frozen **CCD FAIL remains historical and unchanged**.

Only this untracked report/proof packet created. No executable/library hashing, authentication, build, runtime/operational actions, delegation or tracked edits. Original streams/argv/launcher/result/admission/context read in full and reused without rewriting.
