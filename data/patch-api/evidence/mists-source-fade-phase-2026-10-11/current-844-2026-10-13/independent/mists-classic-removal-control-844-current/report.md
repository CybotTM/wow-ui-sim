# Mists classic-removal control — saved runtime verification

## Verdict
**Targeted Lua assertions: PASS. Full normal startup / CLI: FAIL (exit 1).** Neither verdict supersedes the other. Saved evidence only; no rerun, build, executable/library/archive read, source authentication, delegation, tracked edit, or fixes.

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/mists-original-cast-prefork-current-20261013T195457944782Z`.
Runtime: `runtime-classic-removal-control-retry1`.
Recorded revision: `844a63adb95c7111e1286fa668e632f7cd237806`; profile: `mists`.

## Evidence read completely
- `classic-removal-control.lua`: exact 28-line assertion script; completion print follows every assertion.
- Runtime `stdout`: 898 bytes; completion marker `MISTS_CLASSIC_EVENT_REMOVAL_CONTROL_OK` followed by two JSON error entries, each count 1.
- Runtime `stderr`: 15,527 bytes, 231 lines, read through final summary: `Status: FAILED`, `Lua errors: 2 unique, 2 occurrence(s)`, both attributed to `Blizzard_UIPanels_Game`, zero unattributed occurrences.
- Runtime `result.json`, `argv.json`, `admission.json`, `launcher.txt`; saved `receipt-wow-sim-normal.json`; prior `runtime-classic-removal-control/blocked-receipt-schema.json`.

## Exact behavioral coverage
| Script lines | Statement / behavior | Saved proof |
|---|---|---|
| 2, 10–13 | Permissive unknown ordinary event `SIM_CLASSIC_EVENT_REMOVAL_CONTROL` registration, registered state, actual delivery | PASS: registered-state assertion and deliveries = 1 after dispatch; callback asserts frame identity and event name |
| 8–9 | Initially absent removal returns exactly one value and false on repeated absent removal | PASS: `select("#", ...) == 1`, then false |
| 14–17 | Registered ordinary removal returns true, stops delivery, repeated removal false | PASS: count stays 1 after removal and dispatch |
| 18–22 | Unknown unit registration with two filters, `player` and `target`, followed by `focus` dispatch | PASS: aggregate count advances from 1 to 3; exact sequence accepts two matching deliveries and excludes focus under this control; no separate per-unit counters |
| 23–27 | Unit-event removal returns true, blocks both subsequent matching dispatches, repeated removal false | PASS: count remains 3, repeated removal false |

Completion marker proves all top-level assertions and exercised callback assertions reached completion. Return arity is explicitly checked only for initially absent removal, not independently for registered ordinary/unit removal. No claim of broader arity coverage.

## Separate startup failure
`stdout:4–11` contains original frame-creation error and its ScriptErrors wrapper, count 1 each. `stderr:40–47` records the same failure during Blizzard addon loading: `__Blizzard_UIPanels_Game_20279` cannot anchor to itself (`Action[SetPoint]`). Wrapper references `Blizzard_ScriptErrors.lua:59` and `:72`. These are two unique recorded messages for the original/wrapped self-anchor failure, not two failed control assertions. Full normal startup remains FAIL; launcher reports exited status 1, runtime 12.866s. No fix attempted.

## Receipt and launch classification
Existing normal `wow-sim-normal` once-auth receipt records PASS, one hash read, and `pathstat_after`. Runtime `receipt_stat_reused` exactly matches that saved field (device 66306, inode 147198538, size 398088896, mtime_ns 1791921555412436844). This is saved-receipt correlation, not new authentication or current binary validation. Receipt explicitly records metadata-only build association and no source-byte authentication.

Prior runtime directory contains only `blocked-receipt-schema.json`: `SETUP_BLOCKED_ZERO_LAUNCH`, caused by reading `stat_after` instead of `pathstat_after`; no admission/argv/runtime performed. Classify as setup blocked, not behavioral FAIL. Retry is the single saved normal runtime launch reviewed here.

## Acceptance boundary
Mists-only targeted unknown ordinary/unit registration, delivery and removal control PASS. Full normal startup FAIL. Other Classic profiles, native WoW parity, all-profile acceptance, and other feature acceptance are not established.
