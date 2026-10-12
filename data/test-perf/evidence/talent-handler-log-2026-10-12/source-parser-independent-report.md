# Independent source/parser-development proof

Date: 2026-10-12. Scope: `1d2b0ea30afa2c76128bc1356b2125615c6e1748` against `f41dc05c2`, `/home/osso/Projects/wow/wow-ui-sim`.

**PASS — bounded source and saved parser-development evidence only. No accepted in-scope findings.** Read verify and Rust-readability skills; performed verification directly without delegation. No Cargo, build, simulator runtime, unit-test rerun, repository edit, or binary hash.

## Coverage matrix

| Claim | Result / proof level |
|---|---|
| Actual Rust edits limited to test instrumentation | PASS / commit diff and current-source equality |
| Markers outside original tick clock | PASS / source inspection |
| Canonical setup, 20/10/24 order, 35/75/+20 ms assertions preserved | PASS / source and exact Rust diff |
| Production/vendor/config unchanged in commit range | PASS / exhaustive changed-file inventory |
| Existing OnUpdate/OnPostUpdate timing paths | PASS / source wiring and interval boundaries; conditional, not runtime proof |
| Parser implementation and 12 behavioral tests | PASS / source inspection and retained GREEN evidence bound to current source hashes |
| Genuine RED46, initial syntax error excluded | PASS / retained outputs and metadata |
| Changed Rust readability; Python simplicity | PASS / manual changed-line audit; no accepted violations |
| Real runtime 54 windows, causal/performance/native acceptance | NOT VERIFIED / expressly excluded |

## Rust source and preservation

Only four tracked paths changed: new `docs/specs/talent-handler-log-observation.md`, modified `tests/talent_panel_onupdate_perf.rs`, new `tools/talent_handler_log.py`, new `tools/test_talent_handler_log.py`. No production, vendor, configuration, dependency, asset, or script change in this commit range. Current contents of these four paths and the three inspected logger files equal commit `1d2b0ea30`.

`tests/talent_panel_onupdate_perf.rs:75-93`: sampling now maps each index to `run_logged_headless_tick`. Its begin `eprintln!` precedes `run_headless_tick`; end follows its returned Duration. Original clock starts inside `run_headless_tick`, before timers/layout/OnUpdate, and ends at `started.elapsed()`. Neither marker print belongs to that clock. Handler logging inside dispatch still belongs to the clock and may perturb latency; markers outside the clock do not establish a non-perturbing run.

Constants at lines 11-17 are unchanged: visible-handler added ceiling 192, average 35 ms, max 75 ms, delta 20 ms; baseline 20, settle 10, steady 24. Setup at lines 25-39 and opening behavior are untouched. Latency test at lines 239-284 retains settled load → baseline sampling/mean → open → settle → steady sampling/mean/max → original three inclusive assertions. Visible-handler test remains unchanged and unmarked.

Changed Rust readability audit: short effect-revealing logging helper, explicit returned duration, simple two-combinator iterator, three clearly named parameters, no new conditional/nesting burden, suppressions, magic thresholds, hidden side effects, duplication, or unnecessary state. Existing longer tests are below the skill's 200-line test limit. No readability edit needed.

## Existing logger semantics and limits

`src/lua_api/script_helpers/event_dispatch.rs:224-267` filters OnUpdate modes/visibility, absent handlers and non-functions before calling `dispatch_on_update_handler` at line 312. That helper starts its clock after owner/function lookup, before executing-addon replacement and budgeted call. Error formatting/error-handler work and executing-addon restoration precede elapsed capture. It then reaches one log emission through `log_dispatched_handler` and `handler_timing::log_with_source`; repeated threshold checks do not emit additional records.

`src/lua_api/on_update.rs:144-157` resolves OnPostUpdate after the OnUpdate pass and calls `env.fire_script_handler`. `env_events.rs:452-480` obtains bindings and calls `call_widget_handler` once per dispatched binding. At `env_events.rs:333-375`, its clock includes owner mutation, budgeted call, error handling and owner clearing, then reaches one timing log through lines 95-126. Both paths log attempted calls including ordinary returned errors; record count is not unique callbacks or proof of completed bodies.

`handler_timing.rs:17-44,62-97`: `OnceLock` caches the environment threshold; `WOW_SIM_LOG_HANDLER_TIMINGS=0` before its first initialization includes zero durations. Output rounds milliseconds to three decimals. An unset threshold disables records; positive threshold drops short intervals. No actual process configuration is authenticated here. Visibility/mode/cache/absent-binding filters remove candidates before dispatch; quota exhaustion can prevent Lua-body execution despite an attempted-call interval. Nested helper/callback work may be included in an outer interval, and separately logged nested calls can double-count. Neither interval nor aggregate is pure/exclusive body time. Abrupt termination or dropped/truncated capture can lose records; syntactically complete windows cannot prove capture completeness or all registered callbacks ran. Not every dispatch entry point is established to use these two loggers.

## Parser and behavioral evidence

Parser is 195 lines, standard-library-only. CLI reads a raw file, calls pure `parse_log`, emits JSON or a contextual nonzero error. Ordered full-match marker validation enforces begin/end sequence and canonical default counts; malformed handler-prefixed lines fail inside or outside windows. Integer thousandths preserve rounded sums/maxima. Repeated attempts, three handler categories, phase/tick/outside statistics and identity grouping remain distinct. Sources retain trailing source text. Group ordering is deterministic. Functions are bounded and direct; no speculative abstractions or fallback paths. Anti-pattern scan found zero TODO/FIXME/HACK/XXX in the three code files.

211-line test module has 10 parser tests and 2 CLI tests. Concrete inputs exercise duplicates, aggregates/identity/category separation, outside/unparsed records, zeros/optional sources/deterministic ordering, exposed measurement limits, malformed records/durations/markers, wrong/overlapping/duplicate/missing/unclosed windows, and subprocess file-to-JSON/nonzero-error behavior. Assertions inspect outputs and externally exposed metadata, not implementation source shape. The CLI's 54-window fixture is synthetic, not simulator runtime evidence.

Retained command for both genuine RED and GREEN: `python3 -B -m unittest -v tools.test_talent_handler_log`, cwd this repo. No rerun performed.

- Initial `red-16926da3f96e4aa9a2ed65399e8d603f`: exit 1, SyntaxError at test line 211; **not RED credit**.
- Genuine `red-assertions-8043c1b159de4903874794837abb8a61`: exit 1; `Ran 12 tests in 0.058s`, `FAILED (failures=46)`. Full saved stderr inspection found 46 FAIL blocks, 46 AssertionError lines, zero ERROR blocks. Messages: 39 missing ValueErrors, two missing ticks outputs, two unexpected successful CLI exits, one each missing measurement/outside/grouping outputs. Test-source hash equals GREEN/current; RED parser hash differs as expected for preimplementation code.
- `green-bb8f027b3e1c4baf91e210f84f670452`: exit 0; all 12 named tests `ok`, `Ran 12 tests in 0.099s`, `OK`; stdout empty. Both saved scope SHA256 values equal current source and current source equals target commit. Parser `65650b6ce4f49e42c7ab16d65ac7d0719756cec2b32c5a25d2db0b8d1797c333`; tests `4384ae2f251a4d803f1ac00ed6f23c55a62af0434078c60c2d3496a0faa72bb6`.

Saved metadata/logs establish recorded development outcomes and current GREEN scope, not independently witnessed past execution or cryptographic provenance of the historical RED implementation (not retained here).

## Finding falsification and boundaries

Marker-overhead-in-clock concern rejected against the actual helper ordering. Runtime redesign/extra configuration rejected against unchanged production paths and literal test-scoped requirements. Measurement-metadata string assertions are output-contract behavior, not source-shape tests. Synthetic 54-window success cannot substitute for real capture. No causal, repaired-performance, native parity, build, formatting-command, runtime acceptance, or broader historical/full-handoff completion claim.

Read-only evidence commands: scoped `git diff`, `git show`, `git status`, `git rev-parse`; direct source/log reads; Python file hashing and retained-log summaries. Initial command failed because Pyrun cwd was not the repo; subsequent Git commands explicitly set cwd. File finder was unavailable; explicit-path reads/grep supplied evidence. Preexisting untracked `.code-index.db` and `verification/` were observed and untouched.
