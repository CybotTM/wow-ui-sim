# Currency fullsuite saved-artifact gate

**Overall suite: FAIL. Saved-artifact accounting/consistency gate: PASS.**

Tested `1dcc59c0788d0e09ee3d02516123df965ba8dc63`; prior accepted artifact `c75ccfe02c52f76a0456ae393581a3b9ca601ce7`.

## Counts (executions, not deduplicated selectors)

| Phase / batch | PASS | FAIL | skips | actual executions |
|---|---:|---:|---:|---:|
| integration / 1 | 10703 | 0 | 19 | 10703 |
| prefork / 1 | 2320 | 1 | 0 | 2321 |
| prefork / 2 | 2 | 0 | 0 | 2 |
| prefork / 3 | 1 | 0 | 0 | 1 |
| prefork / 4 | 1 | 0 | 0 | 1 |
| lib / 1 | 2001 | 0 | 0 | 2001 |
| **Overall** | **15028** | **1** | **19** | **15029** |

Execution-plus-skip slots: 15048. Prior: 15024 PASS, 1 FAIL, 19 skips; 15025 actual executions (15044 slots). Delta: +4 PASS executions, no removed PASS execution selectors.

Garrison FAIL preserved: `blizzard_garrison_ui_loads::blizzard_garrison_ui_loads_explicitly_via_load_addon_without_errors`; `tests/blizzard_garrison_ui_loads.rs:286:5` (current log lines 11405/11622; prior 11379/11596). Private failure payload not reproduced.

## Four new currency tests
- PASS `currency_info_probes::get_backpack_currency_info_reflects_successive_existing_currency_updates` — current raw log line 5462.
- PASS `currency_info_probes::get_backpack_currency_info_returns_nothing_when_existing_currency_row_is_removed` — current raw log line 5463.
- PASS `currency_info_probes::get_currency_list_info_reflects_successive_existing_currency_updates` — current raw log line 5478.
- PASS `currency_info_probes::get_currency_list_info_returns_nil_when_existing_currency_row_is_removed` — current raw log line 5481.

## Method pair and library
- PASS `method_diff_coverage::diff_methods_extra_snapshot_matches_current_metatable_surface` — log line 6858.
- PASS `method_diff_coverage::diff_methods_missing_snapshot_matches_current_metatable_surface` — log line 6859.
- Library: exactly 2001 PASS, 0 FAIL, 0 skips; 2001 outcome rows match summary line 15856.

## Commands, exits, durations

- `cargo nextest run --test integration --no-fail-fast --test-threads 16 --offline --locked` — exit 0; metadata elapsed 1417.1 s; summaries agree.
- `cargo test --test prefork_full_ui --offline --locked` — exit 1; metadata elapsed 123.9 s; summaries agree.
- `cargo nextest run --lib --no-fail-fast --test-threads 16 --offline --locked` — exit 0; metadata elapsed 133.8 s; summaries agree.

Nextest runner-only duration: integration 1324.852 s; library 67.962 s. Prior runner-only durations: integration 823.153 s; library 54.601 s. These differ from metadata step duration including build/overhead.

Current wall: 1675 s (27m55s), 2026-10-11 02:47:33–03:15:28 -0500; metadata step sum 1674.8 s. Prior wall: 1218 s (20m18s), 00:40:47–01:01:05 -0500; metadata step sum 1218.5 s. Prior step durations: integration 955.7 s, prefork 105.7 s, lib 157.1 s.

## Ref/code authentication

`1dcc59c07` JSON ref and full SHA match named artifact and existing Git commit. Later HEAD `2b1ab710dc00c55bd26d7ce8bbffaf1258441e37` descends from tested SHA. Read-only Git diff is empty for `src`, `tests`, `Cargo.toml`, `Cargo.lock`, `build.rs`; corresponding Git objects plus `.cargo` match exactly. Tracked code worktree clean. Object IDs and commands/results recorded in JSON. This authenticates repository equality, not unrecorded executable provenance.

## Limits / open gates

- Recorded exits agree with JSON and result summaries; actual process exit capture is not independently provable beyond saved records.
- Command argv from phase headers: integration/lib --test-threads 16; prefork no explicit thread argument. Actual concurrency, environment overrides and auxiliary launch argv are not logged.
- Auxiliary prefork batches execute 2 + 1 + 1 tests; durations and independent exit codes unrecorded. Parent prefork exit remains 1.
- 19 integration skips count separately, not passes or executions. Names/reasons unrecorded; no complete skip-identity comparison possible.
- Prior artifact corroborates same named Garrison failure at same source location; no new recorded failing selector. This is not blanket no-regression proof, causal attribution, or evidence about skipped/unexecuted native/other-profile cases.
- Mutable baseline JSON new_failures is not accepted as regression proof.
- No native, CI, push, deployment or full handoff closure. No builds, tests, checks, reruns, polling, operations, delegation, source/cache mutations or binary rehash performed.
- Initial read-only git attempts lacked cwd and failed; subsequent explicitly scoped git commands supplied identity evidence. No test command was run.

- Caller reports FULL_SUITE_JOBS=12 did not propagate and compiler jobs remained 12. Saved phase argv independently establishes nextest --test-threads 16, not compiler-job count or environment propagation; those caller claims are not authenticated by this gate.

Both complete raw logs read privately; all batches scanned. Exact outcome rows, ordinals, summaries, source locations and raw-byte SHA-256 hashes recorded in `countproof.json`. No private payload copied into reports.
