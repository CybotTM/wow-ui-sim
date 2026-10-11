# Saved-artifact final verification: PASS (bounded)

Test revision: `069f1cfbd9ae3b184c81b4fa898cca4cd098510f`.
Retention/document revision: `9cc194968`.
Saved epoch: `20261011T044347Z`.

Prior accepted source/runtime/compile/format proof in `independent-report.md` and `.json` reused; no commands rerun, builds, operations, binary reads/rehashes, or delegation. This report supersedes only their missing-final-receipts and pending-retention conclusions, not their scope limits. Four previously missing final artifacts are now present and read completely: check-1.json, check-1.stdout (0 bytes), check-1.stderr, outcome.json.

## Saved command evidence

- Exact runtime selector `achievements_api::test_reward_item_id_tracks_supplied_achievement_state`: 1 executed, 1 passed, 0 failed, 0 ignored, 0 measured, 10717 filtered out; exit 0; 0.12s. Saved runtime stderr contains 11 startup lines only.
- Prior accepted compile: exit 0; 738 JSON records = 663 artifacts + 74 build-script records + 1 successful build-finished; 1 integration artifact, 0 compiler-message records; 6 dependency manifest deprecation diagnostics. Artifact hash remains producer-recorded receipt evidence, not an independent binary rehash.
- Prior accepted format: exit 0, stdout/stderr both 0 bytes.
- Newly admitted check: `cargo check --offline --locked --tests`, exit 0; finished dev profile in 19.37s. Complete stderr contains 6 iced-wgpu-patched deprecated lint-name diagnostics and 7 Rust dead-code diagnostics (1 patch_2_4_2_factory, 6 patch_3_0_3_factory), 13 individual warning diagnostics total; generated-warning summary lines are not additional diagnostics. Not warning-free. Dead-code locations: tests/common/publication_sweep.rs:275,184,213,218,227,244,269. No error diagnostics observed.
- Outcome: executed_cases=1, case_exit=0, one_named_case_passed=true, inputs_unchanged=true. All 6 before/after source/config hash entries match exactly. This covers only the six listed inputs, not whole dependency provenance.

## Retention/document audit

All 10 manifest files exist, contain actual receipts/output, and match untouched private copies and committed `9cc194968` bytes exactly: 10/10 byte equalities to private, 10/10 to commit, 10/10 manifest size matches, 10/10 manifest SHA-256 matches across all three representations; 4912 total bytes. Files: admission.json (178), compile-submission.json (1157), compile-result.json (63), artifact.json (1002), execution.json (452), reward.stdout (200), reward.stderr (641), check-0.json (103), check-1.json (115), outcome.json (1001). Private files were read only.

README.md, retention-manifest.json, and docs/lua-api.md also match their committed bytes: 3/3. All 3 scoped markdown links at docs/lua-api.md:358 resolve: existing probe tests, new update test, retained packet. README's selector resolves to the actual executed case; all 10 manifest source/retained references resolve. README and docs/lua-api.md correctly restrict their claims and acknowledge prior basic reward coverage.

Existing tests/c_achievement_info_probes.rs:34 tests seeded reward/nil (including an unknown-ID observation); :116 tests a supplied reward. These are existing tests, not additional executions or newly accepted unknown-ID evidence. New tests/achievements_api.rs:13 adds 1 case/54 lines only; producer unchanged. It inserts 2 synthetic existing records (700001/700002), observes Some(180001)/None, updates both in the same live environment to None/Some(180002), and asserts 4 exact-one-return arities alongside value/nil observations. Prior source review records TODO/FIXME/HACK/XXX=0 each. Saved named execution establishes reachability.

Default Retail only: artifact includes default/client-retail/profile-retail/retail-12-1-0 plus GUI/sound/CASC features; runtime CASC disabled. No GUI/catalog acceptance follows. No RED bug repair, native mapping/parity, unknown-ID contract, secrecy/taint, native catalog, other-profile, full-suite or whole-handoff acceptance.

## Handoff

Bounded saved-artifact and retention/document gate PASS. README and docs/lua-api.md pending-status wording should be updated after main acceptance; it is not a premature documentation failure. Original independent reports/private receipts remain unchanged. No outstanding missing receipt for this assigned gate.
