# Independent initial Friend RED verification

Scope: `retail-red-20261012T120059906734Z` only; recorded source `740e51339fbdb00e4da6a4012bf9cc7532b3374d` (test/spec preparation; production getter unchanged in saved compile evidence).

## Authentication

NEW `integration-sealed` authenticated once and receipt immediately persisted in `independent-auth-integration.json`. Expected and actual: **467812576 bytes**, SHA256 **ad1f4892b5fdad3e40512e09eda542325379ac6a60c9ec9a1f93be1b9d444c46**. Both match. Receipt reused for report; no second hash, no old Quest artifact/root-proof/auth reads.

## Actual compile and runtime

Full compile stdout consumed: 741 JSON records / 574432 bytes; 665 compiler-artifacts, 75 build-script-executed, one build-finished with success=true; zero compiler-message records. Full compile stderr: six deprecated manifest lint-key warnings in iced-wgpu-patched/Cargo.toml, one aggregate warning line, two compiling lines, final finish in 36.32s. Compile exit 0. No warning-free claim.

Full runtime stdout/stderr read. Actual summary:

> test result: FAILED. 0 passed; 4 failed; 0 ignored; 0 measured; 10780 filtered out; finished in 0.83s

Runtime exit 101 corroborates failure, not the source of the four-failure count.

| Fixture (`friend_info_exact::`) | Observed first failure | Proof boundary |
|---|---|---|
| exact_friend_lookup_authenticates_secret_name_by_caller_taint | tests/friend_info_exact.rs:43:6 — missing exact C_FriendList.GetFriendInfo | Direct missing-query RED; secret/caller-taint controls unreached |
| exact_friend_lookup_reads_live_notes_without_environment_leaks | tests/friend_info_exact.rs:43:6 — missing exact C_FriendList.GetFriendInfo | Direct missing-query RED; first positive result, second environment, mutation/isolation controls unreached |
| exact_friend_lookup_rejects_nonstring_types_and_returns_nil_for_miss | tests/friend_info_exact.rs:43:6 — missing exact C_FriendList.GetFriendInfo | Direct missing-query RED; type/miss controls unreached |
| real_mainline_friend_note_popup_prefills_and_reopens_with_live_note | tests/friend_info_exact.rs:151:5 — popup API readiness Blizzard_SharedXML; missing C_Reveal attributed to DebugBarManager.lua:106 (Public environment) | Closure prerequisite blocked; not missing-query consumer evidence |

The three direct cases stop at the raw exact-member function-presence guard (test source lines 31–32), before table/arity/name/note assertions. Popup stops in load_popup_addon missing-requirements assertion; no completed closure, Mainline OnShow/provenance, public popup, edit-box note, hide/reopen, or changed-note proof. Test names are intended coverage, not executed downstream coverage.

## Source and controller limits

revision.json and source-applicability.json record the source740 HEAD, empty applicability diff, exit 0. compile-outcome.json records identical before/after values for Cargo.toml, Cargo.lock, friend_list.rs, and friend_info_exact.rs, with sources_unchanged=true. This supports recorded compile-stage source stability only. No post-runtime source attestation or independently authenticated source740 spec snapshot is present; no source hashes recomputed. Read local test/spec/getter context without treating current working files as sealed historical snapshots. Getter context registers GetFriendInfoByName, not exact GetFriendInfo.

All epoch JSON parsed: compile-result exit 0, runtime-result exit 101, outcome status EXECUTED. Admission histories record admitted=true. **No controller completed/error fields exist in the supplied epoch JSON**, so those states cannot be independently claimed. Cargo build-finished success=true proves compile completion, not test success. Operation/admission success does not replace the actual libtest summary. No service poll.

## Test-source readability / exclusions

Manual test-only read: concrete supplied SocialFriend state, explicit raw exact-query guard, named closure/provenance/popup helpers, contextual failure messages; no readability violations identified. No metrics/check commands executed; no production readability claim. integration.rs uses integration_tests!(); actual runtime enumeration proves these four fixtures were wired into this sealed executable.

**Result: authenticated executed RED, 3 direct missing-query failures + 1 blocked consumer prerequisite.** Not four direct missing-query failures. No producer permission, GREEN, native parity, or whole-handoff completion. No builds/tests/runtime commands/checks/operations/delegation/questions/repo edits. Raw epoch files preserved; only independent authentication/report/proof files added.
