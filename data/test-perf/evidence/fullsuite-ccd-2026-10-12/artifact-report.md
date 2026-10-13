# Frozen full-suite artifact audit — FAIL

Revision: `ccd526c6c9f783cf7d3fe15eda05c42c6cabab59`. ARTIFACT-only; excludes new staged43/current source.

## Exact counts

| Suite | Pass | Fail | Skip | Timeout | Run | Exit |
|---|---:|---:|---:|---:|---:|---:|
| integration | 10877 | 5 | 19 | 0 | 10882 | 100 |
| prefork | 2326 | 2 | 0 | 0 | 2328 | 1 |
| lib | 2001 | 0 | 0 | 0 | 2001 | 0 |
| Total | 15204 | 7 | 19 | 0 | 15211 | FAIL |

Integration: 10,882 unique primary nextest outcomes, ordinals 1..10,882 exactly once. Summary at log:11527 agrees: 10,877 passed (46 slow), 5 failed, 19 skipped. Starting at log:14 agrees. Repeated failure recap and nested child summaries are not additional tests.

Prefork: main group at log:13879 is 2,322 pass / 2 fail / 2,324 total. Subsequent groups at log:13884,13888,13892 add 2+1+1 passes. Entire prefork section therefore has 2,326 pass / 2 fail / 2,328 total, all names unique; no ignored or timeout outcomes.

Lib: 2,001 unique primary nextest outcomes, ordinals 1..2,001 exactly once; terminal Summary at log:16405 agrees. Zero failed/skipped/timeouts.

Zero timeout outcomes does not erase the two `timeout child ... failed (exit code 101)` messages at log:2913,10424. These accompany budget assertion panics, not runner TIMEOUT results.

## Seven retained failures

### Integration

- `addon_loading_perf::loading_phase_breakdown_stays_within_budgets` — log:2869,2908–2913; tests/addon_loading_perf.rs:261:5: Lua compile 3.13s exceeds 2.00s. Parent tests/common/timeout_reexec.rs:184:9 reports child exit101.
- `animation_query_lifecycle::bounce_loop_callback_observes_direction_change` — log:772,798–799; tests/animation_query_lifecycle.rs:200:10: unwrap of Lua assertion error.
- `animation_query_lifecycle::delaying_uses_order_start_and_longest_parallel_duration` — log:805,831–834; tests/animation_query_lifecycle.rs:308:9: reverse timeline at 1.5; left (3.0, false), right (1.5, false).
- `patch_10_2_5_probe_factory::patch_10_2_5_vertex_color_probe_reaches_missing_endpoint_methods` — log:7684,7710–7713; tests/patch_10_2_5_probe_factory.rs:27:5: left "VertexColor; lookup=function", right "VertexColor; lookup=nil".
- `startup_perf::full_game_startup_stays_under_budget` — log:10032,10419–10424; tests/startup_perf.rs:128:13: startup 27.63s exceeds 20.00s. Parent tests/common/timeout_reexec.rs:184:9 reports child exit101.

### Prefork

- `patch_3_3_0_publication_sweep::patch_3_3_0_publication_sweep` — log:13725,13730–13733; tests/common/publication_sweep.rs:315:5: new gaps []; resolved/stale gaps ["wt-widgets-Texture:GetFileHeight-28", "wt-widgets-Texture:GetFileWidth-27"].
- `patch_4_0_1_publication_sweep::patch_4_0_1_publication_sweep` — log:13741,13745–13746; tests/common/publication_sweep.rs:315:5: new gaps []; resolved/stale gaps ["wt-global-api-UnitSetRole-348"].

Source file references above are log-reported paths relative to `[PRIVATE_PATH]`, not source inspected in this audit.

## JSON completeness and exit interpretation

PASS: supplied result JSON is complete for the recorded integration/prefork/lib run: matching revision, three steps, commands/exits consistent with log headers, positive durations, recorded environment, finished timestamp, and all seven failed names (none omitted/added). Every suite has terminal accounting. JSON has no numerical counts; countproof supplies independently parsed log counts. `new_failures` duplicates current failures, but historical novelty is not verified.

Submission JSON records exit 0 and systemd unit acceptance; user reports systemd wrapper exit 0. Neither overrides integration exit 100 / prefork exit 1. Final wrapper status is not independently present in supplied artifacts. OVERALL FAIL.

## Warnings and proof limitations

- `warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition` (also repeated in prefork/lib).
- `warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition` (also repeated in prefork/lib).
- `warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition` (also repeated in prefork/lib).
- `warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition` (also repeated in prefork/lib).
- `warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition` (also repeated in prefork/lib).
- `warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition` (also repeated in prefork/lib).

- Frozen ccd526c6c9f783cf7d3fe15eda05c42c6cabab59 artifacts only; excludes new staged43/current source.
- No rerun, build, test, executable/library byte or hash inspection, operations, delegation, or repository edits.
- Integration 19 skip names not present in log; counts from Starting and Summary only.
- Zero timeout means zero runner timeout outcomes; timeout-child exit101 messages are assertion failures, not timeout outcomes.
- Six distinct iced manifest deprecations recur in each suite: 18 deprecation lines plus three generated-6-warnings notices.
- Performance failures observed under full-suite concurrency; root cause and isolated timings unproven. Lib logs two package-cache lock waits.
- Null WOW_SIM_TIMEOUT_FIXTURE_CAPACITY does not establish the effective default capacity.
- No platform/profile/feature matrix completeness or current source correctness claim.


## Retention transformation
Prior independent report sanitized; private Inputs list omitted. Counts/accounting reused from established report/countproof without re-audit. Prior audit PASS statements are historical evidence, not new validation. Result JSON byte-preserved. Selected log is bounded, not full-map or raw-log parity; provenance records text-stream hashes, inclusive ranges and redacted original line identities. Full raw log/countproof remain private and are not copied.

This retained audit describes its frozen-artifact checkpoint, before later selected corrections were executed. The packet README and integrated SSOT own subsequent revision-specific status; no current-source verdict follows from this report. Public-copy validation is separate from the frozen suite outcome. No executable/library bytes inspected or hashed.
