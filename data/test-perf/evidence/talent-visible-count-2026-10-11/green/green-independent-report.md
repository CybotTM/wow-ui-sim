# COLLECTOR_REPAIR_PASS — bounded final behavior/check gate

Revision: `5d2e4ece1c08ec967cc8a7fd8f8bbe60d5a02ba4`. GREEN epoch: `green-compile-20261012T002653774260Z`; default-check epoch: `check-20261012T004234946429Z`.

**Collector repair PASS. Broad BA FAIL; full handoff OPEN.** Saved text/metadata inspection only. No runtime, test, check, formatter, rebuild, operation, polling, delegation, session attach/resume, executable/library byte read or hash performed.

## Reused evidence first

Read `green-compile-source-report.md` and `green-compile-source-proof.json` before new receipts. Reuse verifier2470's first NEW artifact authentication: 467445040 bytes, SHA-256 `aa801f09926eb845115af4fe88a6ab341b9e6b85b3739c3923bee8daf368555d`, exact selected Cargo integration-record match. No reauthentication. Compile exit0/source_unchanged=true, current-source/5d linkage, unchanged regression/original controls, manual readability with no violations, and final formatter exit0 are reused, not recomputed or rerun. Recorded provenance is not hermetic provenance.

Read verifier2469's saved `red-compile-20261012T000621243005Z/red-independent-report.md`; reuse accepted RED: 0 passes/1 failure, collector0 versus expected1 after callback1. Later hide/restoration assertions were unreached in RED; original controls executed0. No RED rerun or rehash.

## Coverage / exact counts

Full GREEN `runtime.stdout` (853 bytes) and `runtime.stderr` (18311 bytes, all 289 lines) read, together with submission, admission, terminal/result and process receipts. Raw parent recap:

```text
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 10733 filtered out; finished in 6.20s
```

Two nested child recaps each say `1 passed; 0 failed; 0 ignored; 0 measured; 10735 filtered out` (3.25s latency; 2.74s ceiling). These re-exec the original controls; **not two extra logical cases**. Independent raw-log accounting: exactly three logical bodies, one new regression and two original controls; 3 PASS/0 FAIL. Parent rows for controls span the nested child output and finish `ok`; the regression row ends `ok`. No failed recap or panic/failure diagnostic in the full raw logs. Filtered counts are discovery metadata, not broad-suite acceptance.

| Case | Observable proof | Verdict / limit |
|---|---|---|
| `visible_onupdate_collector_tracks_child_parent_visibility` | Final raw named `ok`; sequential source assertions reached through body completion: callback counts 1/1/2; counts baseline+1/baseline/baseline+1; actual child name included/excluded/restored; child stays locally shown under hidden parent | PASS, isolated collector regression. Values are witnessed by reached assertions, not standalone numeric logging |
| `talent_panel_open_visible_onupdate_handlers_stay_within_ceiling` | Child named `ok`, parent completes `ok`; unchanged added-count predicate <=192 after settled startup and talent open | PASS original control; exact before/after/added counts are not logged |
| `talent_panel_open_onupdate_tick_latency_stays_under_budget` | Child named `ok`, parent completes `ok`; stderr baseline435.58µs/talent450.17µs/max489.31µs; unchanged <=35ms average, <=75ms maximum, <=baseline+20ms average predicates | PASS original control; rounded logged delta14.59µs, not exact unrounded delta or causal proof |
| Default `cargo check` | Saved result exit0 at5d, empty stdout, full stderr ends `Finished dev profile ... in 0.63s` | PASS with six inherited manifest deprecations, not warning-clean |
| Compile/source/readability/final format | Reused verifier2470 evidence above | PASS reused scoped evidence; no new CLI audit |

Source witness: `tests/talent_panel_onupdate_perf.rs:53-73,109-206,209-280`; constants at11-17 retain192,35/75/20ms and20/24/10 ticks. No source edit in this gate. Source unchanged/current5d is the reused2470 witness plus supplied same-source continuity, not a newly recomputed Git/hash result.

Collector reads registered `on_update_frames`, applies existing ancestor visibility, sorts numeric IDs and retains name mapping. Reused source evidence establishes registrations track **OnUpdate OR OnPostUpdate**, not mode-eligible membership or actual fired-callback counts. Regression separately exercises a public OnUpdate callback. Do not conflate these domains.

## Terminal and check receipts

Runtime submission selects `talent_panel_onupdate_perf::`, `--nocapture --test-threads=1`, same epoch's already authenticated `integration-sealed`. Admitted2026-10-12T00:40:40.496589Z. Unit `wow-talent-counter-green-cases-20261012t004040496600z.service` terminal observation2026-10-12T00:41:54.059433Z: `ActiveState=active`, `SubState=exited`, `Result=success`, `ExecMainStatus=0`, `ControlGroup=` empty; RemainAfterExit explains active/exited, not running work. Saved process observation2026-10-12T00:41:54.077493Z: remaining sealed processes[]. Runtime result `OBSERVED_TERMINAL`, success/0 agrees with raw logs.

GREEN `outcome.json` still says `COMPILED_NOT_EXECUTED`/executed_cases0. This is stale compile-stage metadata, contradicted/superseded for runtime by later admission/submission, complete raw logs and terminal receipts; not silently treated as current. Earlier RESOURCE_BLOCKED evidence in2470 remains historical, not the final runtime verdict.

Check submission: `/usr/bin/cargo check --offline --locked -j 12`, repo cwd, revision5d; admitted2026-10-12T00:42:34.953762Z. `result.json` exit0 at5d; `outcome.json` CHECK_EXECUTED/exit0. Six inherited iced-wgpu-patched manifest warnings: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity; final summary is six warnings, not a seventh distinct warning. No warning-clean claim.

## Acceptance boundary

This closes the missing final collector behavior/default-check report only. Isolated regression plus original controls support collector repair, not a loaded-latency root cause, production OnUpdate repair, old broad counts/artifacts, native or third-party acceptance. Broad BA remains FAIL; full handoff OPEN. Only private `green-independent-report.md` and `green-proof.json` written. No new acceptance requirement, test case, algorithm or repo change.
