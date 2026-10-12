# Expanded Bounty positive RED — artifact review

**Behavioral RED confirmed: compile exit 0; runtime exit 101; 0 passed / 11 failed. Authentication evidence gap: one sealed-payload SHA-256 read completed, but its printed digest/comparison was lost from truncated tool output and ephemeral Pyrun context. No second payload read performed. Do not treat this report as an independently authenticated full handoff.**

## Attribution and evidence

Repository `/home/osso/Projects/wow/wow-ui-sim`; source exclusively committed `9602fa8fbb8fdba3176ac44d7cfa4a370ba720df`, not current working tree or future producer. Committed test SHA-256 `1eec1a5c32e2b610c6e485972b55af6f2e906af41f903104d2a1e457c4c27f56` matches revision record.

This commit adds `SimState.bounty_set_maps: HashMap<i32, Vec<i32>>` at `src/lua_api/state/sim_state.rs:579`, initializes it with `HashMap::new()` at `src/lua_api/state.rs:496`, and adds five tests. No getter producer is implemented by this commit; spec explicitly records that gap. Real owned input is not a replacement/mock query. `relation_env` at test L179-189 injects four relations: 90001 → [71,19,71], 90002 → [42,19], 90003 → [], i32::MAX → [42]. Default input remains empty; these are supplied fixtures, not a native catalog.

Recorded new sealed artifact: 467777768 bytes, SHA-256 `37bc2c81b6f9923c9d5bdcd702f5a26ad16e1300b33092aa3a6a5e06d04e35e1` in `artifact.json`. Its integration compiler-artifact record matches `compile.stdout`; `fresh: false`, `build-finished success: true`, `compile-result.json` exit 0/source unchanged. Artifact bytes were read once; recorded hash is not mislabeled as a retained independent computed match. No initial6/Calendar binaries or libraries read.

Recorded toolchain: Cargo/rustc 1.99.0, x86_64-unknown-linux-gnu. Compile stderr records six iced manifest deprecation warnings, not warning-free proof. Runtime command from `runtime-submission.json`: `/usr/bin/timeout 90 <epoch>/integration-sealed c_map_bounty_set_maps:: --nocapture --test-threads=1`; started October 12, 2026 at 02:40:40.626346 UTC (October 11 at -0500). Entire runtime stdout/stderr inspected. Result: 0 passed, 11 failed, 0 ignored, 0 measured, 10747 filtered out, 0.79 seconds; exit 101, not timeout. Retail artifact includes retail-12-1-0 and actually executes both gated secret tests. No PTR runtime proof. Compile-stage `outcome.json` says COMPILED_NOT_EXECUTED; later runtime outcome/logs supply execution evidence.

## Eleven-case reached/unreached matrix

References below are committed test source. Initial six case interpretation reused from initial6 report/proof; their repeated failures in this epoch were inspected independently. All fail in public Lua assertions, not construction/setup.

| Test suffix | Reached and observed failure | Unreached; NOT proof |
|---|---|---|
| `actual_map_util_consumes_relations_and_preserves_cache` | FIRST stderr failure. Owned fixture installed; entire unmodified cached `MapUtil.lua` read and executed successfully (L306-311). First actual `MapUtil.GetBountySetMaps(90001)` at L315 returns into `sequence`; non-table assertion L156 fails. Rust panic is `relation_probe` L196:6, not source-read or source-execution expect. | Arity, supplied sequence/order/duplicates, other/empty relations, memoization, state update L324-327, cache retention and caller mutation L331-338. |
| `default_associations_return_one_empty_sequence` | First get(90001), L41; non-table assertion L12 fails; panic L34:10. | Arity/empty shape/ipairs, second ID 90002. |
| `empty_results_are_fresh_and_mutation_isolated` | First get(90001), L51; L12 fails; panic L34:10. | Other queries, identity/freshness, mutations and snapshot isolation. |
| `fractional_and_extreme_inputs_do_not_alias_supplied_ids` | Owned fixture installed. First positive get(90001), L285; L156 fails; panic L196:6. | Second positive and i32::MAX controls, ENTIRE fractional/overflow/nonfinite loop L288-295, subsequent positive controls. No numeric non-alias proof. |
| `numeric_edges_return_empty_without_associations` | First loop value 0 at L93 reaches L12 and fails; panic L34:10. | Empty shape for 0 and every later numeric input. |
| `owned_updates_and_removal_preserve_snapshots_and_other_sets` | Owned fixture installed. First positive get(90001), L219; L156 fails; panic L196:6. | Unrelated snapshot, insert update, removal, every snapshot-preservation and isolation assertion; Rust mutations themselves are not reached. |
| `positive_results_are_fresh_and_mutation_isolated` | Owned fixture installed. First positive get(90001), L258; L156 fails; panic L196:6. | Later queries, all rawequal/mutation/isolation checks, Rust final state assertions L274-276. |
| `required_number_rejects_missing_nil_and_wrong_types` | First rejected() at L74 calls public getter without argument; L26 fails `invalid bounty set input must raise an error`; panic L34:10. Missing argument did not raise. | Contextual-error assertion, explicit nil, all wrong-type inputs, valid recovery. |
| `supplied_relations_preserve_dense_order_and_duplicates` | Owned fixture installed. First positive get(90001), L205; L156 fails; panic L196:6. | Positive dense shape/order/duplicates/ipairs, 90002, explicit empty 90003, absent 90004. |
| `tainted_secret_inputs_are_denied_before_lookup` | Wrapper setup and tainted-entry assertion pass. First wrapped 90001 public query, L129; combined denial/string assertion L130 fails `secret query denied`; panic L34:10. | Error text/authentication ordering, taint/wrapper preservation, declassification checks, second secret, plain queries, outer restoration. Combined assertion does not disambiguate successful pcall from non-string error. |
| `untainted_secret_inputs_preserve_wrappers` | Untainted entry assertion passes; first wrapped 90001 public query at L107 returns into empty; L12 fails; panic L34:10. | Second secret query, shape/arity, secrecy/value/context preservation. |

All nine table failures have exact message `bounty set query must return a non-nil table`. This proves a non-table result at those observed calls, not specifically nil, zero returns, or return arity.

## Meaning and limits

The five added tests provide meaningful positive-input RED: real owned membership exists before public/consumer calls, yet the first supplied-positive query violates table shape. Actual source consumer successfully loads and executes before failing at its first returned-result assertion. Neither boundary is a setup error. No later sequence/cache/update/numeric/security behavior is promoted into proof.

Initial6 artifacts and Calendar scopes remain unchanged; initial6 report/proof reused without reopening its sealed payload. No native catalog, native ordering, native unknown-ID, producer success, full suite, parent acceptance, or full handoff claims. No builds/tests/checks/operations/polling/delegation/repository changes performed. Only this epoch's report.md and proof.json written.
