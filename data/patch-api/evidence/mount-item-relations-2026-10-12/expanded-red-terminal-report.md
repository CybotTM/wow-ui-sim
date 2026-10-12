# Expanded mount-item terminal verification — ACTUAL RED

Unit: `wow-mount-item-expanded-red-20261012t143207719476z.service`
Revision: `3cc6d36af560f28968d1a2a4ec38958583d56dbb`
Epoch: `expanded-red-20261012T143207789737Z`

## Terminal gate

- Controller terminal: `active/exited`, `Result=success`, `ExecMainStatus=0`. This means capture completed, NOT tests passed. Unit already terminal at first observation; no wait or replay required.
- Compile PASS, exit 0: full 574,439-byte JSON stdout parsed (741 records: 665 compiler-artifacts, 75 build-script records, one successful build-finished); unique integration record matches artifact.json. Full stderr inspected: six deprecated hyphenated Clippy manifest-key warnings in iced-wgpu-patched/Cargo.toml; zero compiler-message records. Not a warning-free build. Duration 2m25s.
- New integration-sealed authenticated once: 467,973,920 bytes, SHA256 `3e33540e2d03a6d8a8d1406e424ff09ab75e16ee0acc723d6fa3d2f2b755fcae`. Receipt absence checked before the sole read/hash; independent-auth-integration.json saved immediately. No old seal reads/hashes or new seal rehash.
- Format PASS, exit 0; stdout/stderr empty.
- Actual runtime FAIL/RED, exit 101: **0 passed, 7 failed**, 0 ignored/measured, 10,786 filtered out, 0.49s. All seven fail with `missing C_MountJournal.GetMountFromItem`. No timeout, capacity block, missing artifact or failed compile.

## Actual coverage boundary

| Case | Failure location | Reached / unreached |
|---|---|---|
| `mount_item_authenticates_secret_input_before_type_or_lookup` | `tests/mount_from_item.rs:105` | Taint mode enabled and untainted setup assertion reached; missing-API guard fails before wrapper creation, permission/type ordering, taint preservation and restoration checks. |
| `mount_item_empty_relation_returns_exactly_one_nil` | `tests/mount_from_item.rs:24` | Fresh environment and missing-API guard reached; exact-one-nil checks and both queries unreached. |
| `mount_item_requires_one_numeric_identity` | `tests/mount_from_item.rs:47` | Fresh environment and missing-API guard reached; missing/nil/nonnumeric argument checks unreached. |
| `supplied_mount_item_mapping_does_not_leak_into_independent_fresh_environment` | `tests/mount_from_item.rs:128` | First fresh relation empty assertion passed; first mapping inserted. First query guard fails; second environment never constructed, isolation and first-environment retention unreached. |
| `supplied_mount_item_mapping_needs_no_catalog_target_or_ownership` | `tests/mount_from_item.rs:128` | Fresh relation empty assertion passed; catalog cleared, 901001->401 inserted and pre-query catalog-empty assertion passed. Guard fails; mapped result/arity and post-query catalog assertion unreached. |
| `supplied_mount_item_mapping_replacement_and_removal_are_live` | `tests/mount_from_item.rs:128` | Fresh relation empty assertion passed; 901001->401 inserted. First query guard fails; result, replacement->402 and removal->nil unreached. |
| `supplied_mount_item_numeric_identities_are_distinct_without_truncation_aliases` | `tests/mount_from_item.rs:128` | Fresh relation empty assertion passed; six explicit mappings inserted. First 901000 query guard fails; all six result/arity checks and fourteen miss/nonalias controls unreached. |

The four positive cases are authored/compiled and reach their first API-existence guard after explicit fixture setup. They do NOT prove any successful query, mapped result, exact arity, replacement/removal, fresh-environment isolation or numeric nonalias behavior. Original secret case fails before authentic secret wrappers are created; permission ordering is not runtime-proven.

## Source applicability

All six saved paths match before compile, after compile, after runtime, and independent current-source hashes: Cargo.lock, Cargo.toml, docs/specs/mount-item-relations.md, src/lua_api/state.rs, src/lua_api/state/sim_state.rs, tests/mount_from_item.rs. Saved revision applicability is exit 0 with empty relevant diff. Current HEAD independently observed at the exact revision above.

Independent original-to-expanded commit diff contains only three files / 119 insertions: empty HashMap field/default plus 116 test lines. Original entire 4,628-byte three-case file remains an exact prefix of the 8,158-byte expanded file. src search finds only field at sim_state.rs:580 and default at state.rs:496; no GetMountFromItem producer or ordinary relation population. No native catalog claims.

Toolchain: cargo 1.99.0 (5f94df478, 2026-08-27), rustc 1.99.0 (b940084d7, 2026-09-28), x86_64-unknown-linux-gnu, LLVM 23.1.1.

Old default cargo-check evidence is NOT current library-scope evidence after state changed. No check run now. No build/test/check/runtime replay, unit mutation, delegation, repository edits, dynamic seeding or mocked calls by verifier.

Pending expanded-red-report.md / expanded-red-proof.json and initial-red-report.md / initial-red-proof.json preserved. Terminal proof is expanded-red-terminal-proof.json; authentication receipt resides in the new epoch. Actual pre-producer RED confirmed; feature remains unimplemented and this report grants no producer greenlight.
