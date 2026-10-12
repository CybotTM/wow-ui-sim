# Initial Bounty RED6 artifact verification

**PASS: authentic bounded initial RED6. Behavioral result: FAIL, 0 passed / 6 failed; runtime exit 101.** Not Bounty implementation, native parity, full-suite or parent acceptance.

## Provenance and constraints

- Repository: `/home/osso/Projects/wow/wow-ui-sim`. Examined committed source with `git show e09b0c1d1fdea73035cb31d319670183a593aa4b`, not the potentially changing working tree. Commit adds only `tests/c_map_bounty_set_maps.rs` (149 lines) and `docs/specs/bounty-set-map-relations.md` (36 lines); no producer change in this commit.
- Test-source SHA-256 from committed source: `879994e639e466a242773c782dbd8043fe0997d76a09c1f23f8a32fc4b845479`, matches `revision.json`.
- Authenticated FIRST NEW `integration-sealed` payload once before runtime/source inspection: 467725480 bytes; SHA-256 `1404170fc0485795d9d342b399b36806977280978c92fe35b18c197e63c6e071`, matching `artifact.json`. No other binary, library or cache payload read. An earlier Pyrun context lookup failed before opening any binary; the successful authentication was the only payload read.
- `compile.stdout` integration compiler-artifact record equals `artifact.json.cargo_record`, reports `fresh: false`, and ends with `build-finished success: true`. `compile-result.json`: exit 0, source unchanged. Recorded build: `/usr/bin/cargo test --test integration --no-run --message-format=json --offline --locked -j 12`. Recorded toolchain: Cargo 1.99.0, rustc 1.99.0, x86_64-unknown-linux-gnu. Compile stderr has six iced manifest deprecation warnings; not warning-free evidence.
- Recorded execution: `/usr/bin/timeout 90 <epoch>/integration-sealed c_map_bounty_set_maps:: --nocapture --test-threads=1`; cwd above. Runtime start `2026-10-12T02:25:21.698048+00:00` (October 11 in repository commit's -0500 timezone). Runtime stdout/stderr read in full. Result: 0 passed, 6 failed, 0 ignored, 0 measured, 10747 filtered out, 0.58 seconds; exit 101, not timeout.
- Artifact features include client-retail and retail-12-1-0; both gated secret tests actually executed. No PTR runtime proof.
- `outcome.json` is compile-stage `COMPILED_NOT_EXECUTED`; subsequent `runtime-outcome.json` and full logs establish execution. Neither record was rewritten.

## Exact failure and reachability matrix

All six Rust panics are at `tests/c_map_bounty_set_maps.rs:34:10`, the `.expect` around the public Lua probe. Below, line references are committed test source at e09, and assertion messages are verbatim runtime evidence.

| Test suffix | Executed boundary and failure | Later assertions NOT reached |
| --- | --- | --- |
| `default_associations_return_one_empty_sequence` | First public `get(90001)` at L41 returns normally into `empty`; L12 fails: `bounty set query must return a non-nil table`. | L13-20 single return/empty table/sequence/ipairs checks; second ID 90002 at L42. |
| `empty_results_are_fresh_and_mutation_isolated` | First `get(90001)` at L51 reaches and fails L12 with same message. | Second/other queries, every rawequal assertion, caller mutations, subsequent fresh snapshots and mutation isolation at L52-65. |
| `numeric_edges_return_empty_without_associations` | First loop value **0** at L90/93 reaches and fails L12 with same message. | Empty shape checks for 0; all later inputs -1, 90001.5, 90002.5, NaN, positive/negative infinity, positive/negative 1e308. No fractional/nonfinite/overflow identity or non-aliasing proof. |
| `required_number_rejects_missing_nil_and_wrong_types` | First `rejected()` at L74 calls `pcall(get)` with no argument; L26 fails: `invalid bounty set input must raise an error`. Thus missing input returned without raising. | L27 contextual-error check; explicit nil at L75; string, empty string, table, false, true, function cases at L76-78; valid-number recovery at L79. No observed wrong-type/nil outcome. |
| `tainted_secret_inputs_are_denied_before_lookup` | Setup creates wrappers and stamps addon; tainted-entry assertion at L127 passes. First wrapped 90001 query reaches L129; L130 fails: `secret query denied`. | Authentication message at L131-132, preserved taint/wrapper and failed declassification L133-137, second wrapper, ordinary queries L139-140, restored outer caller L144, subsequent untainted wrapper queries L145-146. Combined assertion failure proves required error/string denial contract failed; log does **not** disambiguate successful pcall from non-string error. Authentication-before-lookup is NOT proved. |
| `untainted_secret_inputs_preserve_wrappers` | Taint mode setup and nil entry-stack assertion at L105 pass; first wrapped 90001 query at L107 returns into `empty` and fails L12 with same message. | Second wrapped query L108; wrapper secrecy L110, wrapped values L112-113, preserved caller context L114; all shape assertions after L12. Wrapper preservation is NOT proved. |

Public query failure is real, not a mock: helper binds `local get = C_Map.GetBountySetMaps` at L8 and each test uses `WowLuaEnv::new()` at L32 with `env.exec` at L33. Source has no replacement getter or future owned-state symbols. The table assertion proves a **non-table result** at the observed calls; it does not distinguish nil/no values from another non-table value or establish return arity. Runtime did not fail with a missing-function invocation error or environment construction failure.

## Meaning and remaining scope

Absence-to-empty-sequence is explicitly **simulator policy**, not native unknown-ID/order/catalog evidence (test L1-2; spec L8-9 and L36). The cached native-contract assertions in the spec were not independently rechecked: no vendor/cache payload reads were authorized.

This artifact contains only initial absence/shape, snapshot-intent, numeric-edge-intent, required-input and secret-boundary tests. It contains **no real owned empty-input field or supplied-record tests**. Later main-thread changes must not be attributed to this artifact. Remaining planned sequence: main adds real owned empty input plus supplied-record tests, obtains a separate second positive RED, then implements getter/producer and later GREEN gates. Those steps were not executed here.

Historical Calendar proof (52 copies) preserved without reads, rechecks or rewrites. No builds, tests, checks, operations, polling, delegation or repository changes performed. Only this epoch's `report.md` and `proof.json` written.
