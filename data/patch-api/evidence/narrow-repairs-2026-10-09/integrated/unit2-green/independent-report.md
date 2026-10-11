# Independent functional finalgate — approved unit2 repair

**PASS — bounded seven-profile functional gate.** Repair `cc4f4c782f66c955e635d857bfc85a79a021ca24`. Epoch `20261010T235452Z`; unit `wow-unit2-green-20261010t235452z`. **49/49 test commands exit 0; 119/119 actual cases pass; 0 failed, 0 ignored, 0 measured.** Seven successful compilations, default/PTR `check --tests` both exit 0, formatter exit 0. Separate Retail/PTR/Forever normal-artifact startup captures each report **0 unique Lua errors, 0 occurrences**, exit 0.

Read `/home/osso/AgentConfig/skills/verify/SKILL.md`, implementation/test diffs, spec, retained literal-pair source, authentic RED report, saved worker/request/submission, complete compile/test/check/startup output files, JSON receipts and source snapshots. No build, test, replay, operational command, delegation, Bash, rg or fd. Only read-only Git inspection, receipt parsing/hashing, bounded saved-file polling and private report/evidence writes. Polling ended when outcome appeared at October 11, 2026, 00:06:17 UTC, within the requested 20-minute bound. Final source identity check at 00:06:46 UTC. CPU-cost analysis excluded.

## Exact matrix

| Profile | Commands | Unit2 cases | Visibility cases | Admin filter/query cases | Modeled producer cases | Passed | Failed | Compile exit |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Retail/default | 7 | 2 | 10 | 3 | 2 | 17 | 0 | 0 |
| PTR | 7 | 2 | 10 | 3 | 2 | 17 | 0 | 0 |
| Wrath | 7 | 2 | 10 | 3 | 2 | 17 | 0 | 0 |
| Mists | 7 | 2 | 10 | 3 | 2 | 17 | 0 | 0 |
| Era | 7 | 2 | 10 | 3 | 2 | 17 | 0 | 0 |
| Anniversary | 7 | 2 | 10 | 3 | 2 | 17 | 0 | 0 |
| Forever | 7 | 2 | 10 | 3 | 2 | 17 | 0 | 0 |
| **Total** | **49** | **14** | **70** | **21** | **14** | **119** | **0** | **7 exits 0** |

All 49 stdout files contain exactly one successful result with expected counts `(2,10,1,1,1,1,1)` per profile and corresponding actual named `... ok` records. All seven profiles execute the identical ordered set of 17 test names. Exact selector inventory and parsed result tuples, including filtered-out counts, are preserved in `independent-evidence.json`. No zero-selection pass admitted. All 49 commands use sealed profile-specific integration binaries, `timeout 90`, `--nocapture`, `--test-threads=1`; five single-case selectors additionally use `--exact`. Zero nonzero exits and zero timeouts.

Selected scopes:

- `unit_event_second_unit::`: modeled player/target health delivery and re-registration/unregister/unregister-all lifecycle.
- `frame_creation::visibility_scripts::`: 10 actual cases covering boolean SetShown, child visibility, intrinsic ordering/error continuation, reentrant state, recursion and depth controls.
- `admin_event_api::test_register_unit_event_reports_registered_unit`, `test_fire_event_intrinsic_order_and_unit_filter`, `test_fire_event_frame_callbacks_keep_their_own_unit_filter`: first-unit/invalid-first query, payload/binding order, independent callback filter.
- `admin_spell_effects_api::test_damage_fires_unit_health_event`, `test_heal_fires_unit_health_event`: existing modeled producers.

**Intentional error output:** each profile's visibility stderr contains exactly one `Lua error: (string):6: visibility precall failure` (7 total), deliberately injected by `test_visibility_intrinsic_error_does_not_skip_later_bindings` and asserted in its source. These are not failed tests or startup capture errors. Do not describe all test stderr as error-free.

## Additional saved gates

| Gate | Observed result |
|---|---|
| `cargo fmt --all --check` | Exit 0; stdout/stderr both empty |
| Default `cargo check --offline --locked --tests` | Exit 0; 0 error-prefixed lines; 16 warning-prefixed lines |
| PTR equivalent with `--no-default-features --features sound,gui,casc,client-ptr` | Exit 0; 0 error-prefixed lines; 7 warning-prefixed lines |
| Seven compilation stdout streams | Each 738 JSON records: 663 compiler-artifact, 74 build-script-executed, 1 build-finished success true; 0 compiler-message records |

Each compile stderr contains 7 warning-prefixed lines (49 total): six dependency manifest lint-key deprecations plus their summary. Default check also reports unused test-helper warnings. Passing gates are **not warning-free proof**. No warnings were suppressed or changed by this verifier.

## Separate normal-artifact startup captures

Each capture runs `timeout 90 <profile>/wow-sim-normal --no-addons --no-saved-vars lua-errors` with `WOW_SIM_CASC=0`. Compiler records identify non-test `wow-sim` targets; normal/test profile metadata and feature lists match their respective receipts. Normal binaries are independently hashed once at final identity inspection and differ from integration binaries and from each other.

| Profile | Normal SHA256 | Bytes | Exit | JSON entries | Unique Lua errors | Occurrences |
|---|---|---:|---:|---:|---:|---:|
| Retail | `66402b0b9bd93cbad7dc082706962ff28a41ca79601e61b804e9542bd10c4451` | 407315976 | 0 | 0 | 0 | 0 |
| PTR | `b0500bd7ccadd5aa9ae2e63e6bead15a4d88073009b0aa0b834f66269c967ce5` | 414284560 | 0 | 0 | 0 | 0 |
| Forever | `e16c9f75482983efeb487eea427ea098c9b5c283192c4b554352c0fc9fda3ea2` | 402340488 | 0 | 0 | 0 | 0 |

All three stdout files parse as `[]`; all three complete stderr files end with `Status: CLEAN`, `Lua errors: 0 unique, 0 occurrence(s)`, `Attributed owners: 0`, `Unattributed Lua errors: 0 occurrence(s)`. Their ALSA/no-audio-device diagnostics are not Lua errors. This proves only this CASC-disabled/no-third-party-addon/no-SavedVariables CLI startup scope, not GUI, normal asset loading, full addon pool, audio availability or native parity. Other four profile startups were not requested or executed in this epoch.

## Identity and endpoint composition

All **3846** tracked proof-scope paths in baseline and every profile's before/after-compile/after-execution snapshots are identical: **0 missing snapshot entries, 0 changed entries**. Independently recomputed endpoint SHA256 matches all 3846 entries; Git blob identities match the cc4f4c782 tree for all paths: **0 mismatches**. Scope: `src`, `tests`, `Cargo.toml`, `Cargo.lock`, `build.rs`, `.cargo/config.toml`.

Retail/PTR submission receipts name cc4f4c782. Remaining five name docs-only successor `24c727075a69a8a4bbf813156248e0547f3150f9`. Read-only Git diff shows exactly four documentation paths changed (`docs/event-system.md`, wiki index, patch-5-0-4 investigation, wiki log), **0 proof-scope source/build/test changes**. Initial revision-string mismatch therefore is not a functional drift failure; all proof-scope blobs remain cc4f4c782. This distinction is recorded in `independent-evidence.json`.

Each integration compiler artifact has source `tests/integration.rs`, matching saved feature/build-profile composition and exactly one matching `client-*` feature. All seven sealed integration files independently recomputed once match receipt SHA256 and byte size; **0 identity mismatches**. Exact seals/sizes remain in `independent-evidence.json` and profile artifact receipts. No repeated large-binary hashing during polling.

Submission exit 0; saved cgroup membership names the requested unit for all seven phases; all seven admission receipts admit on attempt 1. Controller contains one completed evaluation with epoch console path; controller stderr empty. Final outcome includes seven completed profiles. No pending, resource-blocked or missing profile receipts.

## Requirement/source and RED composition

Retained `data/patch-api/sources/5.0.4-api-changes.txt:63` explicitly documents `Frame:RegisterUnitEvent("event", "unit1"[, "unit2"])`. This supports the bounded literal player/target pair, not unrestricted units, unlimited argument counts, or native query-return parity.

Implementation is substantive and wired: `text_attribute_event/mod.rs:572–577` registers `events::register_unit_event`; `events.rs:60–77` retains optional valid stack-slot-4 token; Frame stores `(String, Option<String>)`; registry overwrites the complete pair on re-registration and removes it on either unregister path; `event_dispatch.rs:31–49` OR-matches the original first payload and `dispatch_named_event_handlers` applies that predicate before ordered handlers without rewriting payload. Frame-size accounting includes optional string storage. Newly added production diff contains **0 TODO, 0 FIXME, 0 HACK, 0 XXX** markers.

First-unit query still returns exactly two values (`events.rs:160–177`, `Ok(2)`), projecting only the first token. Existing invalid-first policy, event validation, forbidden EventRegistrations checks, string extraction and registration/dispatch boundaries remain unchanged outside the pair repair. The selected query control directly passes valid-first and invalid-first registration behavior; exact query arity and unchanged security preflight are source evidence, not newly executed native/security-contract proof. No secret-value relaxation or new secret conversion is added; no new native secret/forbidden suite was run.

`tests/unit_event_second_unit.rs` asserts concrete player heal 50000→70000, target damage 50000→35000, original player/target payloads, single-unit suppression, unmatched-token suppression, one-unit replacement, unregister and unregister-all. Both cases now pass across all seven profiles. The repair diff changes no vendor files, catalog entries or producer health behavior; tests are unchanged from the tests-only RED revision.

Authentic RED report `../unit2-red-current/independent-report.md` records c4f90ce2e: 2 unit2 cases executed, 1 passed/1 failed, exit 101; exact failure `second unit must receive target damage`, after successful health mutations and single-unit controls. Lifecycle and boolean SetShown controls passed; query following the failed assertion was not reached. Current GREEN reaches the previously failing assertion and succeeding query. Admin health setters are fixture seeding, not event producers; existing CastSpellByID modeled effects provide event production.

## Limits and conclusion

**0 in-scope failed tests, 0 failed gate exits, 0 unresolved receipt/identity/composition blockers.** Proof is saved-receipt verification, not replay or hermetic build provenance. External/vendor dependency closure and actual cargo/rustc version receipts are absent from this epoch; no version command was run. Spec's pending proof checkboxes were not edited by this verifier. Broader security/native parity, full suites, deployment and handoff completion are not claimed.

**PASS for approved literal player/target second-unit repair and the requested bounded functional matrix.**
