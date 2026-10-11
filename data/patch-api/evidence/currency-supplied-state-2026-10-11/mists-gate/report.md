# Mists currency final bounded gate

**PASS — 20/20 test executions; 0 failed, 0 ignored; four processes exit 0.**

Revision/current HEAD: `1dcc59c0788d0e09ee3d02516123df965ba8dc63`.
Saved evidence: `/home/osso/.local/state/wow-ui-sim/verification/currency-projection-mists-current/20261011T074441Z`.

## Compile and provenance

- Full compile.stdout parsed: 572,387 bytes, 742 JSON records (666 compiler-artifact, 75 build-script-executed, one build-finished success=true). Zero compiler-message records; compile-result exit 0. Full compile.stderr read.
- Submission: offline/locked Cargo test --no-run; --no-default-features; requested sound,gui,casc,client-mists. Exact effective features: casc, client-mists, gui, prefork-full-ui, rodio, sound. Only client-* feature is client-mists. Both actual test targets and executable paths match artifacts.json; both test artifacts fresh=false and profile.test=true.
- All eight saved before/after input hashes equal current source/config hashes. Current HEAD matches compiled revision. No tracked working-tree differences; unrelated untracked .code-index.db and verification/ remain.
- Both sealed binaries read and SHA-256 checked once each: integration 438,376,384 bytes/ec1390c0cb20f6681f6b7708ecd7683d61e1ea94e2e7d005ce2843cfe9b7353b; mists_currency_list 341,252,976 bytes/5a16c1556493918118e1b4bb07f5fa7c647ca299eb70d16ef39e31834e50ea49. Both match private seals.
- Private receipts inspected: admission.json, compile-submission.json, compile-result.json, artifacts.json, execution-results.json, outcome.json. Receipt claims cross-checked against actual stdout/stderr and source, not accepted as counts by themselves.

## Actual executions

| Set | Target/selector | Actual PASS | Exit |
|---|---|---:|---:|
| 0 | integration / `currency_info_probes::` | 16 | 0 |
| 1 | mists_currency_list / `currency_list_info_preserves_` | 2 | 0 |
| 2 | mists_currency_list / `legacy_currency_list_size_returns_one_numeric_backing_count` | 1 | 0 |
| 3 | mists_currency_list / `legacy_currency_display_updates_preserve_static_rows_and_backpack_membership` | 1 | 0 |

20 distinct emitted test names; all resolve to actual #[test] functions. Full four stdout/stderr sets inspected. Stderr: 176/24/11/11 lines, respectively; 16/2/1/1 WowLuaEnv constructions complete; only startup lines and two bounded static diagnostics, no unexpected stderr lines.

### Verbatim stdout and actual source locations

#### 0.stdout

```text

running 16 tests
test currency_info_probes::get_backpack_currency_info_reflects_successive_existing_currency_updates ... ok
test currency_info_probes::get_backpack_currency_info_returns_nothing_when_existing_currency_row_is_removed ... ok
test currency_info_probes::get_basic_currency_info_returns_display_info_for_saved_instances_hover ... ok
test currency_info_probes::get_currency_container_info_returns_display_info_with_passed_quantity ... ok
test currency_info_probes::get_currency_container_info_returns_nothing_for_unknown_id ... ok
test currency_info_probes::get_currency_info_covers_saved_instances_currency_ids ... ok
test currency_info_probes::get_currency_info_exposes_all_retail_fields ... ok
test currency_info_probes::get_currency_info_from_link_parses_currency_hyperlink ... ok
test currency_info_probes::get_currency_info_from_link_returns_nothing_for_non_currency_link ... ok
test currency_info_probes::get_currency_info_from_link_returns_nothing_for_unknown_currency_id ... ok
test currency_info_probes::get_currency_info_matches_new_saved_instances_currency_metadata ... ok
test currency_info_probes::get_currency_info_reflects_sim_state_mutation ... ok
test currency_info_probes::get_currency_info_returns_nothing_for_unknown_id ... ok
test currency_info_probes::get_currency_info_returns_seeded_valorstones_row ... ok
test currency_info_probes::get_currency_list_info_reflects_successive_existing_currency_updates ... ok
test currency_info_probes::get_currency_list_info_returns_nil_when_existing_currency_row_is_removed ... ok

test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 8189 filtered out; finished in 1.72s
```

- `currency_info_probes::get_backpack_currency_info_reflects_successive_existing_currency_updates` — `tests/currency_info_probes.rs:329`
- `currency_info_probes::get_backpack_currency_info_returns_nothing_when_existing_currency_row_is_removed` — `tests/currency_info_probes.rs:401`
- `currency_info_probes::get_basic_currency_info_returns_display_info_for_saved_instances_hover` — `tests/currency_info_probes.rs:175`
- `currency_info_probes::get_currency_container_info_returns_display_info_with_passed_quantity` — `tests/currency_info_probes.rs:464`
- `currency_info_probes::get_currency_container_info_returns_nothing_for_unknown_id` — `tests/currency_info_probes.rs:486`
- `currency_info_probes::get_currency_info_covers_saved_instances_currency_ids` — `tests/currency_info_probes.rs:46`
- `currency_info_probes::get_currency_info_exposes_all_retail_fields` — `tests/currency_info_probes.rs:141`
- `currency_info_probes::get_currency_info_from_link_parses_currency_hyperlink` — `tests/currency_info_probes.rs:422`
- `currency_info_probes::get_currency_info_from_link_returns_nothing_for_non_currency_link` — `tests/currency_info_probes.rs:438`
- `currency_info_probes::get_currency_info_from_link_returns_nothing_for_unknown_currency_id` — `tests/currency_info_probes.rs:451`
- `currency_info_probes::get_currency_info_matches_new_saved_instances_currency_metadata` — `tests/currency_info_probes.rs:104`
- `currency_info_probes::get_currency_info_reflects_sim_state_mutation` — `tests/currency_info_probes.rs:209`
- `currency_info_probes::get_currency_info_returns_nothing_for_unknown_id` — `tests/currency_info_probes.rs:200`
- `currency_info_probes::get_currency_info_returns_seeded_valorstones_row` — `tests/currency_info_probes.rs:24`
- `currency_info_probes::get_currency_list_info_reflects_successive_existing_currency_updates` — `tests/currency_info_probes.rs:276`
- `currency_info_probes::get_currency_list_info_returns_nil_when_existing_currency_row_is_removed` — `tests/currency_info_probes.rs:380`

#### 1.stdout

```text

running 2 tests
test currency_list_info_preserves_max_quantity_in_namespace_and_legacy_tuple ... ok
test currency_list_info_preserves_watched_flags_in_namespace_and_legacy_tuple ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.28s
```

- `currency_list_info_preserves_max_quantity_in_namespace_and_legacy_tuple` — `tests/mists_currency_list.rs:195`
- `currency_list_info_preserves_watched_flags_in_namespace_and_legacy_tuple` — `tests/mists_currency_list.rs:130`

#### 2.stdout

```text

running 1 test
test legacy_currency_list_size_returns_one_numeric_backing_count ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.19s
```

- `legacy_currency_list_size_returns_one_numeric_backing_count` — `tests/mists_currency_list.rs:103`

#### 3.stdout

```text

running 1 test
test legacy_currency_display_updates_preserve_static_rows_and_backpack_membership ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.15s
```

- `legacy_currency_display_updates_preserve_static_rows_and_backpack_membership` — `tests/mists_currency_list.rs:227`

## Behavioral coverage matrix

| Contract | Concrete source/assertions | Proof |
|---|---|---|
| Static namespace/legacy fields | tests/mists_currency_list.rs:130-224; Valorstones true/true, crest false/false at slot5; cap strict number 90/90 at slot8 | 2 emitted PASS |
| Singleton size | tests/mists_currency_list.rs:103-127; type number, select arity1, namespace/backing equality; current backing has 19 rows | 1 emitted PASS |
| Mutable legacy mapping and conflicting map flag | tests/mists_currency_list.rs:227-309; existing2245 updated positive then zero; supplied backpack flag=false, static watched=true; list arity12, backpack arity4 | 1 emitted PASS, both iterations asserted |
| Static header/unwatched/neighbor/order | New Mists test compares all12 header/unwatched tuple slots and all4 neighbor tuple slots against baseline, asserts backpack neighbor3089 remains index2 and updated2245 index1; size unchanged | Same emitted PASS |
| Shared display projection | tests/currency_info_probes.rs:276-377; successive73/250 and0/0 values, icons134400/134401 and qualities4/0; nine list fields/four backpack fields | 2 emitted PASS |
| Missing map/no compaction | tests/currency_info_probes.rs:380-419; removed2245 list1-nil/backpack0-values/detail0-values; common helper:238-273 preserves size19, header1, list6/3089, backpack2/3089 and3/1792, invalid indices | 2 emitted PASS |
| Existing shared controls | Actual names/locations above, unchanged baseline assertions/constructors | 12 emitted PASS, not comprehensive catalog acceptance |

## Preservation and readability

Compared 085f7d799^..HEAD: shared tests +183/-0, Mists tests +86/-0, compat bootstrap and currency_data no diff. Existing assertions and constructors unchanged. HEAD^..HEAD adds only one Mists test and four spec lines; no existing test replacement.

Readability: no findings in changed tests/mists_currency_list.rs. Manual audit of all 86 added lines; new test body82 lines (<200 test threshold), explicit local borrow/mutation, no complex Rust conditions, no high nesting, no suppressions/TODO/FIXME/HACK/XXX. Embedded Lua assertions identify concrete API tuple slots and share one mutation/baseline scenario. No metrics tool was invoked; no numeric complexity score claimed.

## Warnings

Six manifest deprecation warnings remain in iced-wgpu-patched/Cargo.toml; not fixed or suppressed by this read-only gate. Keys: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity. Each should use its underscore spelling. Cargo summary says six warnings. No compiler diagnostic error records or unexpected execution stderr.

### Verbatim compile.stderr

```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
    Finished `test` profile [optimized + debuginfo] target(s) in 1m 21s
```

## Limitations — remain open

- Bounded saved-artifact Mists gate only; no new build, test, check, full suite, operations, cache acquisition, delegation, Bash, rg or fd was run.
- No native Mists parity, comprehensive catalog acceptance, other/shared-profile execution, or loaded-vendor startup acceptance. The shared source module was executed under client-mists only.
- TokenUI wrapper startup and injected missing-size tests are present and unchanged but were filtered out of these four runs; no claim that they executed here.
- Source producer audit and default RED/check evidence are separate: not rerun, rehashed or independently recertified by this report.
- Eight explicit inputs were matched, not the complete dependency/vendor tree or compiler environment. Saved submission does not include actual cargo/rustc version receipts; no toolchain-version attestation.
- New mutable-row probe asserts arity 12 and slots 1-8, 11-12 as applicable, but does not explicitly assert nil in mutated-row unsupported slots 9 and 10. Unchanged header/unwatched baseline comparisons span all 12 slots; wrapper source is unchanged.
- Tracked spec still describes the new wrapper probe and targeted GREEN as pending. This report supplies private evidence only; no tracked documentation was edited.

## Proof ledger

Saved-artifact audit covers `1dcc59c0788d0e09ee3d02516123df965ba8dc63` and exactly eight scoped hashes. Compile exit0; execution16+2+1+1=20 PASS. No source changed in this gate. Report does not promote historical/default/native/vendor evidence into new acceptance. Source/receipt/seal changes invalidate corresponding proof.
