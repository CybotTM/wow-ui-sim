# Frozen ba full-suite performance scheduling — source investigation

Date: 2026-10-12

## Scope and inputs

Read-only source investigation of `tools/full_suite.py`, `.config/nextest.toml`, and the timeout/workload-gate definitions plus the two registered test bodies. Prior execution facts are taken from `/home/osso/.local/state/wow-ui-sim/verification/ba-fullsuite-current/report.md`; no broad log/proof re-audit. No tests, builds, checks, profiling, source edits, or runtime actions performed. Project wiki index read first.

Frozen run context: report records ba413e8e177f04dc6fc7491834606ce36c750954, with `integration` 15,038 PASS / 3 FAIL / 19 skipped and `lib` accounting included in that total. Exact relevant failures: startup 96.24s against 20s; talent OnUpdate delta 24.47ms against 20ms. The report’s isolated controls passed, but isolation alone does not attribute either frozen failure to contention.

## Source facts

- Main read only the retained log's first line, which records exact resolved command `cargo nextest run --test integration --no-fail-fast --test-threads 16 --offline --locked (exit 100)`. Thus actual recorded cap16 is established without reconstructing `FULL_SUITE_JOBS`. Runner source uses that environment variable/default16; recorded CLI overrides Nextest profile defaults.
- No nextest test-group/exclusive scheduling configuration is present in `.config/nextest.toml`; it only sets profile thread counts and JUnit settings.
- `perf_test_timeout!` calls `with_performance_timeout(120, ...)`. On Linux this enters `timeout_reexec::run(..., Mode::Exclusive, ...)`; ordinary `test_timeout!` enters `Mode::Shared`. The workload gate uses local `RwLock` plus process-shared `flock`; an exclusive holder excludes shared and exclusive holders. On non-Linux, performance timeout delegates to ordinary timeout, without that gate.
- The two requested tests are each registered `#[test]` cases and use `perf_test_timeout!`: `tests/startup_perf.rs::full_game_startup_stays_under_budget` and `tests/talent_panel_onupdate_perf.rs::talent_panel_open_onupdate_tick_latency_stays_under_budget`. Their timed workloads include large game-UI constructors/load/tick sampling; the timeout wrapper is a 120-second deadline, not a 20ms/20s measurement budget.
- This source establishes conditional Linux exclusion when fixture capacity is not externally reserved: each requests the exclusive gate. With `reserved`, parent gate acquisition is bypassed; original-run exclusion remains unproved. It does **not** establish that the other integration tests avoid overlapping: ordinary `test_timeout!` work is shared and may overlap each other, but is mutually excluded while either exclusive performance test holds the gate. Tests that do not use timeout/gate helpers are not governed by this mechanism.
- Main qualification: `timeout_reexec.rs:35–57` bypasses workload-gate acquisition when `WOW_SIM_TIMEOUT_FIXTURE_CAPACITY=reserved`; the reexec child also runs its closure under an inherited parent reservation. Exclusive-mode source claims are conditional on that environment state. The original environment was not retained. Gate wraps parent supervision otherwise; no actual CPU saturation/overlap proof.
- Main `git diff ba413... HEAD -- tests/common/timeout_reexec.rs tests/common/timeout.rs tools/full_suite.py .config/nextest.toml` returned empty; named scheduling sources unchanged. This is source equivalence only, not historical environment/executable identity.

## Answer / boundary

`--test-threads 16` permits Nextest to schedule up to 16 integration test slots; it does not itself force the two named measured closures to run simultaneously. Without the reservation bypass, the Linux gate serializes these wrappers and excludes shared-gated timeout workloads. Therefore16 slots alone do not prove a race; the original reservation state prevents unconditional exclusion claims too. There is no evidence here that all potentially costly full-UI constructors are gated: any constructor path without `perf_test_timeout!`, `test_timeout!`, `lock_env`, or an explicit workload-gate call can escape this policy, and ordinary timeout-backed constructors share capacity with each other. The requested boundary was the two exact tests; tracing every integration constructor would exceed scope.

Actual recorded concurrency cap16 is established by the preserved command header; reservation environment and competing workload scope remain unknown. Also uncertain: whether all relevant competing tests enter shared gates and whether resource contention occurred during this run. No causal claim follows from isolated passes or the source inspection.

## Smallest next bounded boundary

Recorded command reconciliation is complete; do not replay it. Missing evidence is `WOW_SIM_TIMEOUT_FIXTURE_CAPACITY` state for the original unit plus constructor/gate scopes of actual competitors, not the thread count. Inspect only exact constructor/helper nesting if it contributes a concrete protected-interval fact. If the environment is unavailable, report that as an evidence gap. No rerun needed to answer the source-policy question.

If a broader contract requires every full-UI constructor to be isolated, the current source has a policy-coverage gap candidate, not proof that this caused these failures. Whether to extend that contract is a user-owned scope/tradeoff; no change proposed.
