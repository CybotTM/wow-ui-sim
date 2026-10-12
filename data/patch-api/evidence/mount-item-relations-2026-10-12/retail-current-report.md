# Current Retail mount-item bounded verification

**RETAIL_BOUNDED_PASS — not final query, consumer, or parent completion.**

Source `ab8dd8739`, producer `afc6f34bc`; epoch `retail-green-20261012T152010421981Z`. Existing main-owned unit terminal: `active/exited`, `Result=success`, `ExecMainStatus=0`. Waited for actual terminal within 15-minute bound; no compile/runtime/check replay or unit mutation.

## Exact captured execution

| Scope | Actual result | Credit |
|---|---|---|
| Mount-item query | 8 passed, 0 failed; exit 0; 1.14s | Seven prior RED cases now pass; mapped-secret case first execution GREEN, no prior RED |
| Existing mount-spell identifiers | 16 passed, 0 failed; exit 0; 1.63s | Existing simulator policy preserved; native AllowedWhenTainted remains unmodeled |
| Whole PlayerChoice baseline | 1 passed, 0 failed; exit 0; 2.09s | Existing baseline boolean assertion only; no mount branch or modeled dressability credit |
| Normal CLI | exit 0; one literal `MOUNT_ITEM_NORMAL_PROBE_PASS` stdout line followed by `[]` | Raw probe actually executed; parsed empty error JSON, not inferred empty stdout |
| Formatting | exit 0; stdout/stderr both empty | Captured `cargo fmt --check`, not verifier rerun |

### Eight actual query cases

| Case | Proof level |
|---|---|
| `mount_item_authenticates_secret_input_before_type_or_lookup` | PASS, actual execution |
| `mount_item_empty_relation_returns_exactly_one_nil` | PASS, actual execution |
| `mount_item_requires_one_numeric_identity` | PASS, actual execution |
| `supplied_mount_item_mapping_does_not_leak_into_independent_fresh_environment` | PASS, actual execution |
| `supplied_mount_item_mapping_needs_no_catalog_target_or_ownership` | PASS, actual execution |
| `supplied_mount_item_mapping_replacement_and_removal_are_live` | PASS, actual execution |
| `supplied_mount_item_numeric_identities_are_distinct_without_truncation_aliases` | PASS, actual execution |
| `supplied_mount_item_secret_identity_hits_and_recovers_across_callers` | PASS, actual execution |

## Source and wiring inspection

All 11 retained input paths exist. Saved before/after-compile/after-runtime and independent current hashes match; applicability diff against `ab8dd8739` is empty. Exact hashes and paths in `retail-current-proof.json`. No source edits by verifier.

- `src/c_api/mod.rs:130-131` and `registration.rs:30-31` gate the module and registration to `client-retail`/`client-ptr`. `c_mount_item.rs:10-12` registers the real function in `C_MountJournal` without replacing the namespace.
- `c_mount_item.rs:16` authenticates original secret input before numeric validation or lookup; `:17-20` reuses `helpers.rs:7-16` finite/integral/signed32-bit supplied-relation policy. This is simulator policy, not a native range contract.
- `c_mount_item.rs:21-26` reads current environment map and returns exactly one numeric ID or nil. `state/sim_state.rs:580` owns the explicit map; `state.rs:496` initializes it empty. Actual tests cover live replacements/removal, isolation, no catalog/ownership dependency, non-aliasing boundaries, mapped secure secrets, tainted denial and ordinary recovery.
- Existing `c_mount_spell_lookup.rs` has no diff from pre-producer revision `3cc6d36af`; separate public SpellIdentifier reader and ordinary journal-record lookup remain. Actual 16-case preservation and normal Brown Horse assertion pass. Earlier profile exclusion is source-inspected, not runtime-tested; PTR pending.
- Manual Rust readability review found no violations in scoped changed Rust lines/tests. Query and consumer test files plus new query have zero TODO/FIXME/HACK/XXX/allow markers. No metrics/check execution. Spec remains explicitly pending; retained stale “dressability missing” prose is not substituted for runtime evidence.

## Actual consumer boundary and diagnostics

**Observed result is PASS, not a guessed missing-dressability failure.** Whole cached addon closure loaded; source test requires original reward template/intrinsic ItemButton, vendor method identities/provenance, real Hearthstone metadata/link, original `SetItem(6948)`, item identity, and boolean from original `IsDressupReward`. Existing captured baseline completed with no invocation error or recorded Lua errors. No earliest failed prerequisite exists in this capture.

Consumer stdout contains only test harness output; provenance/stage/API-observation/result print lines were not emitted into retained stdout/stderr. Thus boolean value, actual dressability provider identity, and branch taken are not observable here. Do not infer a mount hit, real item classification, native catalog, or model dressability from 1/1. Cached vendor method at `Blizzard_PlayerChoiceOptionBase.lua:662-675` checks dressability before mount then pet; no controlled positive mapping consumer was executed.

Retained full consumer stderr: 59 load warning records, all empty; 20 nonempty nil-observation records; 9 nonempty missing-requirement records. All 60 Lua-error/error-record/error-count snapshots empty. Missing-requirement diagnostics are observations, not failed boundary assertions. Full diagnostics and every raw phase stream are embedded in proof and remain in original epoch files; no truncation of stored evidence.

## Artifact and compiler evidence

Integration and normal new seals authenticated independently exactly once each, immediate exclusive receipts `independent-auth-integration.json` and `independent-auth-normal.json` in this epoch. Existing receipt checked first; neither new seal re-read/rehashed nor old seal accessed. Both byte counts/SHA256 match metadata and each metadata Cargo artifact record occurs exactly once in retained compiler JSON. Both records carry default Retail, `retail-12-1-0`, GUI/CASC/sound features. Compiler build-finished success=true, compile exit 0; no compiler-message records.

Integration: 468235008 bytes, SHA256 `03cb92b5d9070ff137a686c32b664454c3306dd6d7c3767dae7b25b0d47458c9`, receipt 2026-10-12T15:22:15.999351+00:00.
Normal: 407542384 bytes, SHA256 `04245709c87680787a22173a1690b0c56fe5f72414728db37084c794204f3fbb`, receipt 2026-10-12T15:22:16.202613+00:00.

Cargo/rustc 1.99.0 Arch, rustc commit `b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`, host x86_64-unknown-linux-gnu, LLVM 23.1.1. Compile stderr retains six `iced-wgpu-patched/Cargo.toml` deprecated hyphenated Clippy keys (`large-enum-variant`, `map-entry`, `match-wildcard-for-single-variants`, `redundant-closure-for-method-calls`, `trivially-copy-pass-by-ref`, `type-complexity`) and six-warning summary. **Not warning-free.** Full raw streams retained.

Inherited runtime allowlist captured as unset; query/consumer no overrides; normal uses `WOW_SIM_NO_SOUND=1` plus `--no-addons --no-saved-vars`. This does not attest historical/child/cache-content equivalence.

## Immutable RED and remaining gates

Initial 0/3 and expanded 0/7 failures remain untouched at missing `C_MountJournal.GetMountFromItem`; prior downstream assertions receive no retrospective RED credit. Retained report/proof hashes recorded without reading old executable seals. New eighth case has only its actual current execution credit.

**Old default check INVALID after library/state changes. Fresh default check PENDING. PTR PENDING. Final query/consumer/parent acceptance OPEN.** No normal catalog seeding or native/model dressability claim.

Observer-only diagnostic: one Pyrun observation attempt raised `NameError: authenticate_available is not defined` before commands; self-contained observer then authenticated and inspected terminal outputs. No operation replay/delegation/source or unit mutation. Report/proof created after terminal using exclusive creation, preserving existing evidence.
