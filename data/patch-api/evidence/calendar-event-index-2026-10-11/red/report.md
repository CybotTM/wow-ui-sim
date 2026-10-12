# Calendar bounded RED proof

**Genuine public-behavior RED:** exit 101, 0 passed / 9 failed, 1.35s; 10,736 filtered out. Nine logical tests, not eighteen: failure recap repeats their names. No execution rerun.

Stage `a4159ce536bceee5351e90634d546d83abff5b62`; staged test SHA256 `899ae8800ae3f47670bb6b079e2815a932689fd63c45f0fd1b219adb91e8334a` matches revision receipt. `9f1a21a7e17470f835310ef759332da6262ec344` changes docs only. Current uncommitted producer/test changes are NOT attributed to this RED artifact.

Compile exit 0/build-finished success; new integration Cargo record (`fresh=false`) matches artifact metadata. One sealed-artifact authentication: 467458640 bytes, SHA256 `8cde230cde65ca3e7381794ebbeeae99fa302540fe60eb0733b96f232ec5067a`, exact metadata match. No old binary/library reads. Source-unchanged is the compile receipt's historical assertion, not a reconstructed historical working-tree audit.

All nine environments initialize and public namespace/getter checks pass. Eight failures are `configured fixture must return a DTO` (Lua source line14); one is `invalid input must raise an error` (line34), because zero-argument pcall succeeds. Rust panic wrapper: tests/calendar_event_index_info.rs:42:10. These are actual public query assertions, not compile/setup failures.

## Reached / unreached

- **default_session_returns_three_exact_dtos** (call line 49): First get(910001) returns one non-table; DTO type assertion fails. Remaining two fixture queries and DTO fields not reached.
- **each_mismatching_filter_returns_one_nil** (call line 89): 18 mismatching-filter calls return exactly one nil; positive control fails DTO type. Positive DTO fields not reached; nil results alone cannot distinguish absent producer from filtering.
- **invalid_numbers_cannot_alias_fixture_positions** (call line 171): 32 fractional/nonfinite/extreme-number calls return exactly one nil; first positive control fails DTO type. Second positive control and both DTO field checks not reached; anti-aliasing with a functioning producer unproved.
- **omitted_nil_and_matching_filters_return_dtos** (call line 64): First loop fixture get(910001) fails DTO type after arity check. Nil/omitted/matching optional filter combinations and remaining fixtures not reached.
- **required_and_optional_numbers_reject_wrong_types** (call line 135): pcall(get) with zero arguments succeeds; rejected() fails invalid input must raise an error. Required nil, wrong-type matrix, optional validation and final positive control not reached.
- **result_tables_are_independent_snapshots** (call line 115): Two calls made; first snapshot fails DTO type after arity check. Second snapshot validation, identity comparisons, mutations and state preservation not reached.
- **tainted_secret_inputs_are_denied_before_lookup** (call line 210): Taint mode and secret creation succeed; addon enters CalendarIndexProbe taint; plain-input positive control fails DTO type. Secret denial/authentication precedence, wrapper preservation, declassification prevention and caller restoration assertions not reached.
- **unknown_identity_returns_one_nil** (call line 105): 15 unknown-identity calls return exactly one nil; configured-ID positive control fails DTO type. Positive DTO fields not reached; miss behavior not evidence of a modeled lookup.
- **untainted_secret_inputs_preserve_wrappers** (call line 185): Untainted entry verified and secret wrappers created; first wrapped-ID query fails DTO type after arity check. Other secret argument positions, wrapper preservation, unwrap and final taint checks not reached.

Modeled getter producer absent at staged source; no staged src GetEventIndexInfo reference. Existing callable surface does not establish implementation. Earlier outcome.json is compile-only/stale; runtime-outcome.json and full runtime logs establish execution. Six manifest deprecation warnings remain in compile stderr; no setup failure.

RED accepted only for this bounded query regression stage. No GREEN, native parity, Calendar lifecycle, owned-state update or full-handoff completion. No builds/tests/checks/ops, polling, delegation or repo mutation performed. Detailed per-case boundaries: proof.json.
