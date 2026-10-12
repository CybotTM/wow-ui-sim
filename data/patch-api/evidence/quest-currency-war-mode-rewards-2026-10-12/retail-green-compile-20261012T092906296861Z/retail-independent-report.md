# Retail independent precursor verification

**PRECURSOR BEHAVIORAL PASS — not final source/readability/full acceptance. Parent remains OPEN.**

Scope: this epoch only, pinned `4e19a8bad30ffe039a7604f3dc0cff45334e46ee`. Followed verify skill as assigned verifier; no delegation/questions, builds/tests/replay, or old binary rehash. No repository edits.

## Independent artifact authentication

Each NEW sealed artifact received exactly one streaming SHA256 pass; bytes counted during that pass. Each matching digest/size receipt was saved immediately before further work.

| Artifact | Bytes | SHA256 | Result/receipt |
|---|---:|---|---|
| integration-sealed | 467866024 | 8e4c4fd4de2b766942bde5b5e659113917adaee4340d1b5dd82448af585eca4d | MATCH, retail-independent-auth-integration.json |
| wow-sim-sealed | 407384608 | fd4c88f00760f3fcaf532a513f0942b37a77ba76ec9c3734e74b0b80eabb82b9 | MATCH, retail-independent-auth-normal.json |

## Compile and output inspection

Full compile stdout parsed: 574438 bytes, 741 JSON records (665 compiler-artifact, 75 build-script-executed, one build-finished success=true). No compiler-message records. Both manifest Cargo records match exact compiler-artifact records with fresh=false, client-retail and retail-12-1-0 enabled. Compile exit0; stderr finishes test profile in 2m16s. Recorded local toolchain cargo/rustc1.99.0, x86_64-unknown-linux-gnu, LLVM23.1.1. Full compile stderr, runtime stdout/stderr and normal stdout/stderr inspected, including tails.

Six actual inherited iced-wgpu-patched/Cargo.toml manifest deprecations: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity; each recommends underscore spelling. Additional summary: `iced_wgpu` (manifest) generated 6 warnings. No emitted Rust compiler diagnostics in JSON. This was compilation, not a new clippy run or warning-free claim.

`outcome.json` is earlier COMPILED_NOT_EXECUTED metadata; later runtime-outcome.json/normal-outcome.json both exit0 and green-outcome.json EXECUTED agree with actual terminal outputs.

## Behavioral coverage matrix

Actual terminal: `13 passed; 0 failed; 0 ignored; 0 measured; 10767 filtered out; finished in 4.74s`, exit0. Source tests/quest_currency_war_mode.rs inspected completely (528 lines); source SHA256 equals revision.json ee2b5cedce9beaebc211353a01084cc6be127a7f23d1ea97450d209a851148a5. Each PASS below means recorded execution plus inspected behavioral assertions, not replay.

| Exact test suffix (quest_currency_war_mode_) | Observable assertions | Proof |
|---|---|---|
| desired_has_one_boolean_result | one boolean false; percentage/default10 | PASS |
| eligibility_requires_numeric_identity | string/bool/table/function/missing/nil rejected; absent false | PASS |
| faction_query_requires_numeric_identity | same invalid types; one nullable nil result | PASS |
| relation_queries_authenticate_tainted_secrets_first | valid/invalid wrapped inputs denied before type/lookup; wrappers and caller taint preserved/restored | PASS |
| actual_vendor_ordinary_reward_baseline | exact icon/count/name row; no Lua errors | PASS |
| supplied_desired_is_live_independent_and_environment_owned | live changes, generic PvP independent, environments isolated | PASS |
| supplied_percentage_is_live_without_changing_default_query | one result25.5, live0, default10 unchanged | PASS |
| supplied_eligibility_is_not_world_quest_classification | independent eligibility/classification; i32 endpoints; fractional/wrapped-range/nonfinite no aliases; removal/update | PASS |
| supplied_faction_projection_updates_removes_and_preserves_identity | supplied47/89; endpoints; nonrepresented no aliases; update99/removal nil; previous value unchanged | PASS |
| supplied_untainted_secrets_preserve_identity_and_caller | wrapped numeric IDs resolve; wrapped invalid type rejected; wrapper identities and untainted caller preserved | PASS |
| actual_vendor_bonus_and_each_independent_gate | exact25% positive; independently disabled desired, eligibility, world-quest, nullable/false applies, faction exclusion; restoration; live40%; no errors | PASS |
| actual_vendor_duplicate_first_instances_are_per_currency | three ordinary rows7/5/11 retained; bonus counts1,1,2,3,2,1 across repeated tooltip/once changes/independent currency/faction exclusion; no errors | PASS |
| actual_vendor_inputs_are_environment_local | positive vs negative in separate real vendor environments, then reversed | PASS |

Vendor evidence: runtime stderr identifies loaded public closure `@Interface/AddOns/Blizzard_FrameXMLUtil/Mainline/QuestUtils.lua`. Test fixture loads Blizzard_FrameXMLUtil closure, supplies Rust state, and calls actual public handler without replacing getters/vendor logic. Current cached caller lines683-693 show desired/applies/first-instance/world-quest/eligibility/no-faction gates; lines898-915 track first instances per currency; lines969-983 emit ordinary reward and optional percentage then three return values. Tests use ordinary path; currency-container path remains outside scope. Duplicate test compares exact row multiset and exact bonus count, not arbitrary sorted tie order.

## Normal loaded runtime

Saved normal-submission.json runs NEW wow-sim-sealed with no third-party addons/SavedVariables, exec-lua assertions, lua-errors. All four getters raw-published functions, one result each, unseeded values false/10/false/nil; GetWarModeRewardBonusDefault stays10. Both relation getters reject numeric strings with number diagnostic. Broader strict-type/secret cases above execute in integration. Lua equality to these expected primitive values establishes returned types; no separate explicit type(value) checks in normal probe. Full normal.stderr loads291 Blizzard addons and ends `Status: CLEAN`, `Lua errors: 0 unique, 0 occurrence(s)`; normal.stdout exactly `[]
`, normal exit0. Casc disabled/font fallback is logged runtime configuration, not a test fallback introduced here.

## Source applicability and open finding

Pinned source-applicability.json recorded base=head4e19a8bad and zero diff; compile-result source_unchanged=true. Independent current check HEAD `a5eaf4fc63e44027b1e9757b146b398c62e3a4b3`: committed changes since pinned revision only docs and retained evidence/data source-contract snapshots. No tracked source/test/Cargo/build configuration drift; no tracked worktree edits. Untracked .code-index.db and verification/ do not assert source changes. Current test source digest matches pin. This supports historical behavioral applicability, not new final-code acceptance.

Wiring inspected: src/c_api/c_pvp.rs:35 registers modeled WarModeState getters; src/c_api/mod.rs:349 registers quest getter; c_currency.rs:45-63 publishes relation getters; legacy canned quest and reward getters excluded for current profiles. Quest/faction authenticate secrets before shared numeric validation/state lookup. Explicit initial policy false/10 and empty relations are not native catalog evidence.

Pending source readability duplicate is real: c_currency.rs:74-83 validate_war_mode_currency_id repeats helpers.rs:7-17 validate_supplied_relation_id numeric type/range/integrality logic. **Not changed here. Consolidation ONLY after active PTR build terminal per instruction; that later code change invalidates current-code final proof.** PTR terminal status not inspected or claimed. Readability/final source/full acceptance not PASS. No cargo fmt/check/full suite/native parity/earlier-profile proof added. No parent closure.

Machine evidence: retail-independent-proof.json; receipts above. Only these four verification files written inside assigned epoch.

## Post-inspection source change — proof invalidation

Incoming main message reports consolidation committed as `a5eaf4fc6` after PTR compilation finished; independent git inspection confirms commit and current HEAD `a5eaf4fc63e44027b1e9757b146b398c62e3a4b3`. Pinned-to-current currency source diff replaces duplicate validation with shared helper. Earlier no-source-drift observation above describes the inspection snapshot, not latest current source. **Historical Retail precursor behavioral PASS only; current-source proof invalidated.** No current-code/readability/final acceptance claim. Final gate must cover Quest13 and currency-query9 cases per profile; no new commands executing those were run because assigned scope forbids them. Main message’s pending runtime admission concerns separate work; this Retail epoch already records runtime13 and normal exit0. No PTR runtime or parent closure claimed.
