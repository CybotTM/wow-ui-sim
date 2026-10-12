# Retail profile-gate independent verification — PENDING

Epoch: `retail-profile-gate-compile-20261012T110200680260Z`. Source `91324b10380f9be3e8d196491e9a2d20bc597fa1`; build/docs HEAD `298caf47a53eac18960c1cef6716489e54a351cc`.

## Coverage matrix

| Scope | Observed evidence | Result |
|---|---|---|
| New integration binary | First independent read persisted immediately at 2026-10-12T11:04:53.751534+00:00; expected/actual size 467770528 bytes and SHA256 `9bce78a7255ac0576c42cf56d71b53bf55fbd6982c075f11d2a96bc2728d683b` both match; stable during read. Receipt reused for report, never rehashed. | Authenticated, once |
| New Retail compile | Complete 574438-byte stdout / 1275-byte stderr inspected. 741 JSON records: 665 compiler-artifact, 75 build-script-executed, one build-finished(success=true). Compile exit0; 1m30s. Cargo test --no-run; fresh=false means newly compiled artifact (Cargo record), not a test run. | Compile succeeded only |
| Quest caller | 13 source test cases inspected; actual new runtime executions **0**, PASS0/FAIL0. Outcome says COMPILED_NOT_EXECUTED. Three recorded runtime admissions rejected. No runtime streams or runtime result present. | PENDING |
| Changed currency flags | **0 new flag cases**. Prior unchanged nine-case evidence is reused only for its historical scope per instruction, not inspected again or counted as new execution. | No new credit |
| Normal publication/startup | Expected four raw getters: IsWarModeDesired, GetWarModeRewardBonus, QuestCanHaveWarModeBonus, GetFactionGrantedByCurrency. Actual queries0; normal-artifact.json and wow-sim-sealed absent. No normal streams/outcome, no startup execution or zero-error claim. **No normal auth receipt fabricated.** | UNAVAILABLE/PENDING |

Six manifest deprecation warnings remain (large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity); complete stderr retained verbatim in proof. No compiler-message JSON diagnostics. Toolchain recorded Cargo/rustc1.99.0, Arch local.

## Independent source applicability

Direct Git commit/tree inspection, without commands: src, tests, Cargo.toml, Cargo.lock, build.rs, Interface, crates and .cargo have identical identities between source913 and docs298. Full commit differences are docs/evidence only; recorded source-applicability.json also has empty relevant diff/exit0. Current master points to docs298.

Eight relevant working-tree files match exact committed Git blobs: c_pvp.rs, war_mode.rs, quest_currency_war_mode.rs, c_quest_log.rs, c_currency.rs, c_api/mod.rs, Cargo.toml and Cargo.lock. Test source SHA256 `ee2b5cedce9beaebc211353a01084cc6be127a7f23d1ea97450d209a851148a5` matches revision.json. Manual inspection confirms one current-profile publication gate at c_pvp.rs:34-36; shared getters retain live state reads and one Bool/Num result. c_api/mod.rs:387 wires PvP; :348-349 wires Quest; c_currency.rs:50-62 wires the faction query. All 528 test-source lines inspected; real cached QuestUtils closure provenance is asserted rather than replacing vendor behavior. This is source/compile evidence, not runtime acceptance.

Limit: exhaustive tracked worktree traversal hit a packed Git tree object; only explicit relevant working-tree files were authenticated. No whole-repository cleanliness claim.

## Lifecycle and exclusions

Main owns admission/execution/normal builds and operations. No service polling or controller terminal inspection; controller success would not establish runtime success. No commands, builds, tests, replay, delegation, questions, repository edits, long-build waiting, worker cancellation or takeover. No older a5e binary, root proof or receipt opened/hashed. This epoch remains **PENDING**, not current PASS.

## Unique epoch-local proof paths

- `profile-gate-independent-auth-integration.json` — expected/actual sizeSHA256+match, one binary hash pass.
- `profile-gate-independent-auth-normal.json` — unavailable; not created.
- `profile-gate-independent-source-applicability.json` — direct commit/tree/source identity checks.
- `profile-gate-retail-independent-proof.json` — structured coverage and complete compile stderr.
- `profile-gate-retail-independent-report.md` — this report.
