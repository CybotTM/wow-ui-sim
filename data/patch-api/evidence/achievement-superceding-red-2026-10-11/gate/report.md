# Initial RED gate: PASS (expected test failure)

Artifact directory: `/home/osso/.local/state/wow-ui-sim/verification/achievement-superceding-red-current/20261011T141132Z`

## Findings

- Recorded revision `37d17371b17f02de64363a0a7f26dfd2bbe5150e`: compile exit **0**, `build-finished.success=true`; six manifest lint-name deprecation warnings.
- **65 executed: 64 controls PASS, 1 new regression FAIL**, 0 ignored, 0 measured, 10658 filtered out; execution exit **101**.
- Failed test: `c_achievement_info_probes::superceding::get_superceding_achievements_returns_one_empty_table_for_absent_relation`.
- Recorded panic: `/home/osso/Projects/wow/wow-ui-sim/tests/c_achievement_info_probes/superceding.rs:16:10`: `expected a relation table`.
- **One nil result before namespace producer** is caller-provided historical interpretation. Recorded runtime failure corroborates non-table output; initial test source is hash-sealed, not snapshotted here.

## Authentication

Complete execution stderr (697 lines) and compile stdout (741 JSON records) read. Compile artifact path/profile/features match metadata. All ten recorded before/after input seals match. Sealed binary SHA-256 checked **once** and matches `4cee9c970875034af4e47cdb7cf7094929b6b15d6eb2c905fd631580ea369468`. No second artifact hash performed.

Evidence files:

- `/home/osso/.local/state/wow-ui-sim/verification/achievement-superceding-red-current/20261011T141132Z/compile-submission.json`
- `/home/osso/.local/state/wow-ui-sim/verification/achievement-superceding-red-current/20261011T141132Z/compile-result.json`
- `/home/osso/.local/state/wow-ui-sim/verification/achievement-superceding-red-current/20261011T141132Z/compile.stdout`
- `/home/osso/.local/state/wow-ui-sim/verification/achievement-superceding-red-current/20261011T141132Z/compile.stderr`
- `/home/osso/.local/state/wow-ui-sim/verification/achievement-superceding-red-current/20261011T141132Z/artifact.json`
- `/home/osso/.local/state/wow-ui-sim/verification/achievement-superceding-red-current/20261011T141132Z/outcome.json`
- `/home/osso/.local/state/wow-ui-sim/verification/achievement-superceding-red-current/20261011T141132Z/achievement.stdout`
- `/home/osso/.local/state/wow-ui-sim/verification/achievement-superceding-red-current/20261011T141132Z/achievement.stderr`

## Scope limits

Current observed source hashes differ:

- `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/state.rs`: `0a9603dcb351ab356f41bb6f1fd8d6e6a7e3a114d790cea1236fe0157253e351`
- `/home/osso/Projects/wow/wow-ui-sim/src/lua_api/state/sim_state.rs`: `1454e312e6996cd9b7bb64ed4a37bdd85ff881e3a67bd5ef10d19306e11e9cd4`
- `/home/osso/Projects/wow/wow-ui-sim/tests/c_achievement_info_probes/superceding.rs`: `1bafdc75527f8941309904caf8912abe895bc71c6bc8d56f78f5f776fbe03cf0`

Other seven selected source-input seals still match. Historical panic location is not a current-source location claim. This authenticates **initial RED only**, not current parity, GREEN, production state, or native/profile parity. Empty-table behavior is approved simulator policy.

No rerun, build, delegation, operational command, or cwd change. Only requested reports written. An initial report-generation evaluation failed before writing because its prior evaluation variables were unavailable; report generation retried self-contained, without rehashing the artifact.
