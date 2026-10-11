# Final independent dev-harness audit

**OVERALL: BOUNDED GREEN for `256e54ef1`; parent handoff OPEN; cache trial PENDING.**

Audited 2026-10-11. Compiler revision: `256e54ef1d2b9a53b58da62924b8971bd5347a33`. Receipt epoch: `20261011T051913Z/`. Followed `/home/osso/AgentConfig/skills/verify/SKILL.md` as independent artifact verifier. No commands/builds/tests rerun, operations, Bash/rg/fd, delegation, git-history queries, or binary rehash. Only requested final report files written.

## Receipt coverage

All saved stdout/stderr consumed in full; entire compiler JSONL and entire old/new listings independently parsed, not inferred from worker success flags. All exact-case outputs, exit metadata, artifact/profile metadata, final check streams and nine-input seal examined. JSON report includes exact case/check stdout/stderr and independently derived listing counts.

| Gate | Result | Concrete proof |
|---|---|---|
| Retail compile | PASS | `retail/compile-result.json`: exit 0; complete `compile.stdout`: 742 records (666 compiler-artifact, 75 build-script-executed, one successful build-finished), zero compiler-message records. |
| Mists compile | PASS | Same record counts and successful exit; submission explicitly uses `--no-default-features --features sound,gui,casc,client-mists`. |
| Retail integration listing | PASS | Old/new 10,718 / 10,718; zero duplicates, additions, removals; both exits 0. |
| Retail prefork listing | PASS | Old/new 2,325 / 2,325; zero duplicates, additions, removals; both exits 0. |
| Mists integration listing | PASS, explicit prior delta | Old/new 8,199 / 8,200; zero duplicates/removals; only addition is prior reward test below. |
| Mists prefork listing | PASS | Old/new 3 / 3; exact original three fixture names; no additions/removals/duplicates. |
| Exact selected executions | PASS | Retail 8/8 and Mists 5/5; each names its selected case, reports exactly one passed / zero failed, and exits 0. |
| Format | PASS | `check-0.json`: `/usr/bin/cargo fmt --all -- --check`, exit 0; empty stdout/stderr. |
| Check | PASS with warnings | `check-1.json`: `/usr/bin/cargo check --offline --locked --tests`, exit 0; full stderr ends `Finished dev profile ... in 1m 01s`. This is default-profile check, not a separate Mists check. |
| Final nine-input seal | PASS, bounded | Both profile before maps equal `outcome.json.source_hashes_after`; `inputs_unchanged=true`. Independently hashed current nine source/config inputs also match. |

Both profiles emit exactly one compiler artifact for each requested target. All four have `fresh=false` (newly compiled, not Cargo-fresh/reused); executable/features/build-profile metadata match worker artifact receipts, and sealed-copy paths exist. Binary seals are recorded worker evidence, not independently rehashed.

**Mists delta is not harness drift:** the sole addition is `achievements_api::test_reward_item_id_tracks_supplied_achievement_state`, explicitly identified as the prior `069f1cfbd` addition against an older Mists baseline. No other delta was accepted or normalized.

Mists prefork names remain:

- `chat_frame::test_chat_editbox_click_type_and_submit`
- `chat_frame::test_chat_editbox_text_color_after_activation`
- `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix`

Those names were listed, not their fixture bodies executed by this worker. Historical cast-bar RED remains unclosed; no fix or reclassification follows.

## Exact case proof

| Case | Retail | Mists |
|---|---|---|
| `achievements_api::test_reward_item_id_tracks_supplied_achievement_state` | PASS | PASS |
| `conformance::filtering_and_listing` | PASS | PASS |
| `conformance::exact_fixture_groups_list_each_case_once` | PASS | PASS |
| `conformance::read_only_bytecode_cache_child` | PASS | PASS |
| `method_diff_coverage::diff_methods_extra_snapshot_matches_current_metatable_surface` | PASS | Not selected |
| `method_diff_coverage::diff_methods_missing_snapshot_matches_current_metatable_surface` | PASS | Not selected |
| `conformance::generated_registry_lists_nested_marker_case` | PASS | Retail-only |
| `conformance::generated_registry_lists_path_declared_marker_case` | PASS | Retail-only |
| `conformance::mists_registry_lists_only_original_fixtures` | Not applicable | PASS |

Ordinary integration outputs say `1 passed; 0 failed; 0 ignored; 0 measured` (10,717 Retail / 8,199 Mists filtered out). Prefork conformance outputs say `1 passed; 0 failed; 1 total`.

Source-inspected scope: filtering checks actual child listing/filter/skip/zero-match behavior (`tests/prefork_full_ui.rs:806`); exact-fixture and Mists registry checks inspect child listing (`:861`, `:889`); Retail nested/path marker checks inspect precompiled registry listings (`:1082`, `:1101`). Read-only cache conformance (`:1504`) uses an isolated simulator bytecode-cache child. Its PASS is not changed-input/native invalidation or the pending cache trial.

## Source contract matrix

| Contract | Result | Evidence |
|---|---|---|
| Dev-only dependency | SOURCE PASS | `Cargo.toml:97-100`: helper under dev-dependencies; `:11-12`: workspace member. Helper manifest: `publish=false`, syn/quote build dependencies. Root build dependencies `:328-333` contain no syn/quote. |
| Generation/watch ownership | SOURCE PASS | Helper `build.rs:10-17` owns both generators and directory watches; `:198-206` per-source watches. Root `build.rs:1-43` retains native-duration call, locked-Rilua identity publication, Windows resource branch, and has no test generation. |
| Real implementation | SOURCE PASS | Helper build script 483 lines; library 8 lines; manifest 10 lines. Filesystem/recursive path discovery, sorting, attribute copying, duplicate/marker validation and registry rendering are substantive, not placeholders. |
| Wiring | SOURCE/COMPILE PASS | Library `:7-8` includes generated declarations. External macro callers: `tests/integration.rs:1`, `tests/prefork_full_ui.rs:17`, Retail-gated expression `:222-224`. `Case` and `crate::...::run` resolve in invoking test scope. Mists compile does not prove execution of Retail-gated expression. |
| Anti-pattern scan | PASS, new helper scope | Zero TODO/FIXME/HACK/XXX markers; no placeholder or empty implementation in helper source. Generated allow attributes remain; no warning-clean claim. |
| Input-edit invalidation/discovery | PENDING | Source watches establish intent. No saved changed-input/rebuild trial here. Existing generated-marker listings do not establish discovery after an edit. |

Earlier `independent-report.md` records historical extraction/fixture comparison. This final audit rechecks current source and wiring; it does not replay a historical git diff or independently promote that prior report into new provenance proof.

## Warnings and provenance boundaries

Each profile compilation emits six deprecated iced-wgpu manifest lint-key warnings plus an aggregate line. Final check adds seven dead-code diagnostics in `tests/common/publication_sweep.rs:184,213,218,227,244,269,275` (one diagnostic in `patch_2_4_2_factory`, six in `patch_3_0_3_factory`). Exit 0 is not warning-clean. No cargo/rustc version receipt is present in this epoch; toolchain-version acceptance is not established.

Old prefork binaries were previously unsealed local artifacts, first copied/sealed by this worker. Saved Cargo fingerprint features match respective new profile features. That identifies copied bytes/configuration, **not old whole-source/native provenance**. New artifact records likewise do not seal all source/dependency/native inputs. No binary rehash performed.

Nine sealed inputs: `Cargo.toml`, `Cargo.lock`, `build.rs`, `crates/test-harness/{Cargo.toml,build.rs,src/lib.rs}`, `tests/{integration.rs,prefork_full_ui.rs,achievements_api.rs}`. Before/after/current equality is nine-input attribution, not whole-source provenance.

User identifies follow-up `f0e5f64ac` as docs-only with no documented compile-input changes. Current nine-input equality corroborates no drift in those inputs; no follow-up git diff or unsealed source attestation was performed. Bounded saved proof remains attributed to compiler revision `256e54ef1`, not a fresh compile of the documentation revision.

**Scopes stay separate:** source structure PASS; selected simulator compile/list/conformance/reward/metatable cases PASS; native/client acceptance NOT ESTABLISHED. No full-suite/all-profile success, whole-parent closure, native invalidation, optimization acceptance, speedup, or cache-trial completion follows. Parent handoff remains **OPEN**. Cache trial remains **PENDING**.
