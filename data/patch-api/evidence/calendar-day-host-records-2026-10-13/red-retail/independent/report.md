# Calendar Retail genuine RED: confirmed, narrowly scoped

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/calendar-day-retail-red-retry1-20261013T212632455347Z`
Source revision (existing metadata, not byte authentication): `122e272bb87a473b908083fb8105079ccf4185ae`.

## Actual recorded outcome

`test result: FAILED. 0 passed; 9 failed; 0 ignored; 0 measured; 10994 filtered out; finished in 1.15s`

Launcher: `Main processes terminated with: code=exited, status=101/n/a`; runtime 1.208s, CPU 1.085s, peak memory 66.4M. Nine completed `WowLuaEnv::new` initializations. Eight errors are exactly `MissingSuppliedRecord: expected owned DTO`; one is exactly `exact day count`. All Rust panics report `tests/calendar_day_events/fixtures.rs:122:10` (the exec-result expect, not the semantic cause).

Classification: **genuine public API behavioral RED after host seeding**, not compiler, admission, missing-function, consumer setup, or secret-authentication RED. This proves the initial populated producer boundary fails. It does not prove nine distinct downstream contract defects.

## Seeded calls actually reached

`fixtures.rs:54-58` inserts owned records directly into `env.state().calendar_day_events`; key is `(0,15)` (`fixtures.rs:4`). `supplied_env` (`fixtures.rs:127-134`) creates an environment, inserts one full row, and immediately calls `C_Calendar.GetDayEvent(0,15,1)` before returning. Lua namespace and both function guards (`fixtures.rs:66-68`) pass. `CalendarDayOne` fails its first `type((...)) == 'table'` assertion (`fixtures.rs:103-104`). Thus eight actual seeded, ordinary numeric calls return a non-table first value (including possible no value); exact value/arity are not recorded. Field inventory, outer/nested identity, and even the next one-return assertion remain unreached.

Count case (`queries.rs:57-70`) separately creates an environment, successfully asserts the day map is empty, inserts two rows, and calls `C_Calendar.GetNumDayEvents(0,15)`. `CalendarDayCount` passes the one-return assertion (`fixtures.rs:115`), then fails numeric value/type equals 2 (`fixtures.rs:116`). The log does not print the actual return value or type. The source compatibility implementation returns constant 0 (`calendar_defaults.rs:60-64`), consistent with this failure, but conditional registration alone does not independently identify the actual bound function.

## Why absent producer, not fixture/setup RED

Public `CalendarDayEventRecord` contains 21 owned outer fields, with optional icon, opaque String event/club IDs, two six-field owned time records, numeric fractions (`difficulty=7.5`, `num_sequence_days=2.5`), and no Lua projection (`src/c_api/c_calendar/day_event_record.rs:1-38`). The fixture supplies these values; expected Lua keys are camelCase, with 21 outer fields when icon is present and 20 when absent. This is a Rust storage DTO, not proof of a produced public Lua table.

Storage is public `HashMap<(i32,u32), Vec<CalendarDayEventRecord>>` (`sim_state.rs:619-624`) initialized empty (`state.rs:522`). In this snapshot, modeled Calendar registration (`c_calendar.rs:25-32`) registers only `GetEventIndexInfo`; `src` search finds no day-map read/query/projection, only storage/default initialization. `GetNumDayEvents` has an inert compatibility default wired through `workarounds/mod.rs:316`. Existing numeric index-map handling is separate and unchanged. Day records are seeded but no modeled day-event/count producer consumes that storage. Actual runtime failures occur at this exact positive result boundary, not an absence-only test or namespace availability guard.

## Coverage: reached vs downstream unreached

| Case | Reached | Unreached |
|---|---|---|
| Complete snapshots (`queries.rs:5`) | One-row seed, ordinary GetDayEvent, non-table failure | DTO field checks, fresh outer/time tables, caller mutation, replacement/erase snapshots, optional icon |
| Exact keys/dense positions (`queries.rs:57`) | Empty default map assertion; two-row seed; count arity; positive count failure | Both row reads, dense positions, misses, adjacent keys/negative month, empty bucket |
| Environment isolation (`queries.rs:97`) | First environment seed/GetDayEvent failure | Second environment creation, cross-environment replacement/erase/isolation |
| Required numeric selectors (`queries.rs:121`) | Common positive seeded call failure | Missing arguments, nil/non-numeric type rejection |
| Nonrepresented numbers (`queries.rs:145`) | Common positive seeded call failure | NaN/infinity/fraction/out-of-width selectors, alias/miss policy |
| Untainted secrets (`security.rs:6`) | Ordinary numeric common call failure | Taint setup, secretwrap, secret calls, retained wrappers/context |
| Tainted secrets (`security.rs:34`) | Ordinary numeric common call failure | Addon taint setup, authentication-before-type/lookup, denial and no declassification |
| Cached ItemRef (`consumer.rs:46`) | Common call failure before `load_consumers` | Cache resolution/loading, whole ItemRef file execution, links, title replacement/deletion |
| Cached ongoing broadcast (`consumer.rs:76`) | Common call failure | Numeric index fixture insertion, cache loading, GetEventIndexInfo call, CalendarUtil broadcast/replacement |

Security cases did not fail a secret authentication check: `supplied_env()` panics before any secret setup. Consumer cases did not fail cached Lua loading: `load_consumers` is after the failed prerequisite. Their test names describe intended coverage, not reached coverage.

## Admission, compilation, launcher and integrity scope

Prior epoch `calendar-day-retail-red-20261013T211443862053Z`: admission `allowed=false`, 12GiB worker request, directory contains only `admission.json`. Retained as **prior denial, not RED**; no launch artifact observed; untouched.

Retry compile admission: allowed true. Existing compile metadata: build-finished success true, source compiler messages empty, six Clippy-manifest naming deprecations. Full compile stderr ends in successful test-profile completion; recorded terminal `SubState=exited`, `Result=success`, `ExecMainStatus=0`. Compiler JSON records were not independently recounted.

Runtime admission at 2026-10-13T21:35:49.457114+00:00: allowed true; 4GiB max/3GiB high. Full argv invokes the sealed integration executable directly, filter `calendar_day_events::`, `--nocapture`, `--test-threads=1`, under systemd limits, timeout 86s, bwrap read-only epoch-source overlay, network unshared, CASC disabled and sound disabled. No Cargo/compiler runtime step. The reused unit name contains `speak-retail-green`; that label does not alter recorded Calendar filter or FAILED result. Recorded termination is exit101, not timeout/OOM/admission denial.

Existing receipt/seal/auth report read as historical metadata only. Executable **stat only** now matches both recorded stat sets: device 66306, inode 146669627, size 471069528, mtime_ns 1791927108726490700. No executable bytes opened or hashes computed. Source archive, libraries and executable were not authenticated. Revision/source provenance remains metadata-scoped; native parity and cache provenance unproved.

## Operations

Read all six runtime files in full, existing receipt/seal metadata, compile status/stderr and scoped snapshot sources. No tests/builds/runtimes/authentication/delegation, executable/source/archive/library hashing, tracked edits, or prior-denial mutations. Only this report and proof JSON are newly saved outside the repository, mode0600, with exact readback.
