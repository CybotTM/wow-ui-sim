# Initial mount-item RED audit

**GENUINE INITIAL RED: 3 executed, 0 passed, 3 failed; runtime exit 101.** Source revision `0808b3cd0`, epoch `red-20261012T142226038789Z`. Controller terminal `active/exited`, Result=success, ExecMainStatus=0 means capture completed, not tests passed.

## Saved evidence

- Compile exit 0; JSON build-finished success=true. Exactly one integration executable; Retail/default features include `retail-12-1-0`. Cargo/rustc 1.99.0 (Arch). Six `iced-wgpu-patched/Cargo.toml` deprecated hyphenated Clippy lint-key warnings plus summary; no compiler-message records. Compile is not warning-free.
- Runtime: `FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 10786 filtered out; finished in 0.44s`. No timeout. All failures report `missing C_MountJournal.GetMountFromItem` after successful environment construction.
- Formatting exit 0, both streams empty. Saved source-before/after-compile/after-runtime hashes match for tests/spec/Cargo.toml/Cargo.lock. Saved initial applicability diff against revision is empty. This is retained four-file source proof, not independent full-repo post-runtime hashing.
- New 468138872-byte seal independently authenticated exactly once at 2026-10-12T14:23:33.440379+00:00. Matching SHA256 `9fb9172a6e7bc45f35cc151c5a02189b5b63abda71b9cc4bf7840fb9695a685e`. Immediate receipt: `red-20261012T142226038789Z/independent-auth-integration.json`. No old seals read, no rehash, no execution by verifier.

## Exact capability coverage

| Test | Earliest observed boundary | Proof credited |
|---|---|---|
| empty_relation_returns_exactly_one_nil | tests/mount_from_item.rs:15 API function guard; Rust expect :24 | Missing API only; neither query nor nil/arity assertions reached |
| requires_one_numeric_identity | tests/mount_from_item.rs:34 API function guard; Rust expect :47 | Missing API only; no missing/nil/nonnumeric validation reached |
| authenticates_secret_input_before_type_or_lookup | tests/mount_from_item.rs:58 API function guard; Rust expect :105 | Initial taint setup reached; wrappers, permission/type ordering, denied closure, taint/wrapper restoration not reached |

Tests use fresh `WowLuaEnv`, real global lookup, `secretwrap`, `issecretvalue`, and `debug.setobjecttaint(addon, 'MountItemTest')`; no API replacement or simulated secret flags. Source `src/lua_api/globals/security/secret_values.rs:30-58` registers native wrap/unwrap under `retail-12-1-0`, included in the saved artifact features. These are authored authentic controls, **not executed secret-behavior proof**. Manual Rust readability audit after reading the skill found no violations in the three-test source; no metrics/check run.

## Contract and exclusions

Cached Retail `Blizzard_APIDocumentationGenerated/MountJournalDocumentation.lua:234-246` declares required `itemID` number, nullable numeric `mountID`, and `SecretArguments = "AllowedWhenUntainted"`. It provides no typed optional filter or native catalog requirement. Exactly one nil on empty miss is explicit simulator policy, not evidence about native catalog contents or Hearthstone classification.

Spec initial tests match this pre-producer boundary. Backing model, positive mapping/fields, updates/isolation, other-profile preservation, startup parity and real PlayerChoice consumer remain unproved. Private speculative optional-consumer content was excluded, not source and not credited.

## Preservation / observer boundary

No repository edits, delegation, implementation, production queries, or build/test/runtime/check replay. Existing epoch/outcome/proofs untouched except mandated new authentication receipt. Separate report/proof created with exclusive creation. An observation-only Pyrun attempt failed with `NameError: time not defined` before waiting or hashing; self-contained observation followed, without replaying main commands. Runtime admission initially deferred for capacity; main later captured the actual three failures. No downstream assertions inferred from compiled source.
