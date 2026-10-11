# Pending spec-pass reconciliation — c75

Read-only reconciliation against `/home/osso/Projects/wow/full-suite-results/c75ccfe02c52f76a0456ae393581a3b9ca601ce7.log`, source at `c75ccfe02c52f76a0456ae393581a3b9ca601ce7`, and current tests/specs. No execution, build, ops, delegation, or repo edits performed.

## c75 proof status

- Nextest integration target: 10,699/10,699 passed, 19 skipped (log line 10732; skipped count line 14). Separate unit target: 2,001/2,001 passed, 0 skipped (line 15830). These counts do not imply all tests passed.
- `on_update_modes::on_update_modes_process_actual_managed_aura_dirty_phases`: **PASS**, integration result line 7176, position 7150/10699.
- `scenario_unit_criteria::scenario_unit_criteria_secret_outputs_do_not_contaminate_public_values`: **PASS**, line 8247, position 8221/10699. This is the test containing the tainted plain-token restricted-output assertions; there is no separately named `...tainted_plain_token...` test.
- All four `scenario_unit_criteria` tests are named PASS at lines 8244–8247; no matching ignored/absent case among these source test names. The whole target still has 19 unrelated skipped tests; do not generalize that every integration test passed.

## Exact assertions credited

- `docs/specs/patch-12-1-0-onupdate.md` L58/L59: covered by the named managed-aura test body in `tests/on_update_modes.rs` (current/c75 identical): construct `ManagedAuraContainer` with `CustomAuraContainerTemplate`; obtain its private object table; assert `ProcessDirtyFlags` exists; clean initial dirty flags; hide; call `UpdateAllAuras()`; assert mode `2` and dirty. Fire update while hidden; assert mode remains `2` and dirty remains. Show and fire update; assert clean and mode `0`. Fire another update; assert still clean and disabled. Empty Lua error list also asserted. This directly resolves the spec's pending dirty-lifecycle proof for those asserted transitions.
- `docs/specs/scenario-unit-criteria.md` untainted row outputs and tainted plain-token restricted row: `scenario_unit_criteria_secret_outputs_do_not_contaminate_public_values` body asserts three returned restricted outputs are secret/inaccessible and unwrap to supplied `3`, `1.5`, `"1.5%"`; same-valued public row remains public/accessibile, including literals; secret token returns three secrets. In an addon-tainted closure, plain `"target"` returns three secret/inaccessible values, preserves `debug.getstacktaint() == 'ScenarioCriteriaProbe'` and `not issecure()`, rejects secret unwrap and tainted secret-token query; public party row remains readable. After row classification changes to unrestricted, values become public. This is simulator behavior proof, not native-client parity.
- Important scope: `identity_restricted` is a classification supplied directly on each fixture row by `set_credit`; this test does **not** prove a global identity type/resolver, native identity policy, or native secrecy behavior.

## Source/pin reuse check

Read-only Git comparison found current `tests/on_update_modes.rs` and `tests/scenario_unit_criteria.rs` byte-identical to their c75 versions. `Cargo.toml` and `Cargo.lock` have no diff from c75. Thus these PASS results can be reused for those test bodies and dependency pin at current checkout, subject to other runtime/source dependencies not asserted by this comparison. No tests were rerun.

## Still missing / do not over-credit

- OnUpdate spec: native-client parity/live-service evidence is explicitly out of scope; no claim beyond the tested simulator lifecycle.
- Scenario spec: native probes listed under Known gaps remain missing (unknown/despawned/native no-data cases, identity transitions/policy, native tainted/plain and secret-token behavior, exact diagnostics, localization/percentage behavior, etc.). Existing tests cover simulator fixture contracts only. The four named tests do not validate native source contract against a runtime client.
- The `identity_restricted` per-row input is not a global identity restriction model; no model repair or approval is implied.
