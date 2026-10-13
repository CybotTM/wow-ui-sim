# PTR Calendar day RED classification

**PASS — genuine seeded-behavior RED. Feature result: FAIL, 0/9 passed; actual exit 101.**

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/calendar-day-ptr-red-20261013T213459928274Z`
Source: `48902323e64bef2229376a1c1c8c8b4e4cf9e09a` (PTR pre-producer snapshot, not current canonical source).

## Runtime evidence
Full `runtime-calendar-day-red/stdout`: 0 passed, 9 failed, 0 ignored, 0 measured, 10171 filtered out; 1.42s. Full stderr: nine `WowLuaEnv::new complete` records, then eight `MissingSuppliedRecord: expected owned DTO` failures and one `exact day count`. All panics terminate at `tests/calendar_day_events/fixtures.rs:122:10`, the public-behavior assertion boundary.

`launcher.txt` independently records `code=exited, status=101/n/a`, service runtime 1.482s, memory peak 70.6M, swap 0B. `result.json` agrees. `admission.json` records `allowed: true`; this was neither denied admission nor a timeout/setup/compiler failure. The unit name contains “green”; actual output proves RED.

`argv.json` launches the sealed integration executable with `calendar_day_events:: --nocapture --test-threads=1`, through systemd/timeout/bwrap. Source snapshot is read-only bound over canonical project path; target separately read-only bound. CASC disabled and sound disabled. No command was relaunched here.

## Source-backed failure boundary
Snapshot `tests/calendar_day_events/fixtures.rs`: `supply` inserts owned records into the environment day map. `supplied_env` completes initialization, seeds `(0,15)` with one concrete row, then requires a positive GetDayEvent table before returning. Eight cases fail this guard before later test logic. Namespace and function guards precede it and do not fail.

Snapshot `tests/calendar_day_events/queries.rs`: count case completes initialization, asserts initially empty day storage, seeds two rows, then requires `GetNumDayEvents(0,15)` to return 2. It fails `exact day count` before subsequent row/count assertions.

Snapshot `src/c_api/c_calendar.rs` registers only GetEventIndexInfo. `day_event_record.rs` supplies passive owned record types, not Lua projection/query behavior. `src/lua_api/state/sim_state.rs:621` owns day storage; `state.rs:522` initializes it empty. `calendar_defaults.rs` provides constant-zero GetNumDayEvents compatibility behavior. Seeded host storage is therefore not yet connected to the required positive day-query results. The record failure proves a non-table result, not exact nil/arity semantics.

## Coverage matrix
| Scope | Evidence | Classification |
|---|---|---|
| Compile | 741 records, build-finished success true; terminal exit 0 | PASS compile-only |
| Admission / environment initialization | allowed true; nine completed initializations | Reached; not failure cause |
| Positive seeded day record | Eight positive guards fail | Genuine behavior RED |
| Positive seeded day count | Two supplied rows; exact count assertion fails | Genuine behavior RED |
| DTO snapshots / secrets / selectors / isolation / cached consumers | Test bodies blocked by positive guard/count | Unreached, neither PASS nor downstream FAIL |

Both security tests fail before taint/secret setup. Both consumer tests fail before cache discovery/loading; the broadcast test also fails before its separate numeric-index fixture is inserted. Replacement/erase, full DTO comparison, nested freshness, type/representation and isolation assertions are unproven. No native parity or current canonical producer claim.

## Compile and authentication
Full expected `compile.stderr` read: six iced-wgpu manifest deprecations plus aggregate warning line; finished test profile in 3m 38s. No compiler failure; compile-result source compiler messages empty. `compile-terminal-status.txt`: Result=success, ExecMainStatus=0.

Existing copy receipt PASS reused. Executable **stat only**, matching seal, receipt and runtime reused stat: `{"st_dev": 66306, "st_ino": 146703560, "st_size": 470760120, "st_mtime_ns": 1791927518134228938}`. No hashing or executable opens performed. Copy integrity is not independent native/runtime/source provenance; external libraries, cache/settings and source archive remain outside authentication scope.

Original `auth-report.md` preserved. Separate `auth-filename-addendum.md` corrects the prompt's missing-whitespace filename typo, not the original report. No genuine compile-log gap inferred.

No builds, runtime runs/reruns, delegation, or tracked edits. Detailed per-test errors and scope in `proof.json`.
