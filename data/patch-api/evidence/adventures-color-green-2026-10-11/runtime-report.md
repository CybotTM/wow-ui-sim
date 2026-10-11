# Independent saved-artifact runtime verification

**PASS — bounded Retail GREEN epoch at `33cd076872da25692cea7f3bd7bc2e466f9442b3`.**

Evidence directory: `/home/osso/.local/state/wow-ui-sim/verification/adventures-color-green-current/20261011T180245Z`.
Verified 2026-10-11. Followed `/home/osso/AgentConfig/skills/verify/SKILL.md` as verifier; authenticated saved evidence rather than executing proof commands. No builds, tests, checks, operations, repository edits, delegation, or cwd changes performed. Only report written. Source/data/readability PASS was supplied separately and was not repeated.

## Artifact authentication and provenance

First independent SHA-256 authentication of this epoch: all three sealed files match `artifacts.json` exactly, including byte counts:

| Sealed artifact | Bytes | Authenticated SHA-256 |
|---|---:|---|
| `integration-test` | 467421960 | `790ac48104a2b45b52cf4d56aa6ded41d925f2c455e573efa607f0d9cbea233d` |
| `prefork_full_ui-test` | 509411120 | `c079872522a5c65c3cd10afd27762b7e0c1133ed4b366f3afc266118bbd15d6b` |
| `wow-sim-normal` | 407216528 | `62b4415a9b15adee48c263e40aa9e026652e836cc80a761e561f198745f047e4` |

All artifact paths are under the evidence directory above. `compile.stdout` contains 742 parsed Cargo JSON records, one matching compiler-artifact record per manifest executable, and `{"reason":"build-finished","success":true}`. Each matching record agrees with the manifest's complete features and build profile. Source endpoints:

- `/home/osso/Projects/wow/wow-ui-sim/tests/integration.rs` → `/home/osso/Projects/wow/wow-ui-sim/target/debug/deps/integration-9f07e4a55f2bbbf7`.
- `/home/osso/Projects/wow/wow-ui-sim/tests/prefork_full_ui.rs` → `/home/osso/Projects/wow/wow-ui-sim/target/debug/deps/prefork_full_ui-17a690b4e27ba81b`.
- `/home/osso/Projects/wow/wow-ui-sim/src/bin/wow_sim/main.rs` → `/home/osso/Projects/wow/wow-ui-sim/target/debug/wow-sim`.

`compile-submission.json` records revision `33cd076872da25692cea7f3bd7bc2e466f9442b3`; `compile-result.json` records exit 0. All 19 before/after input entries in `compile-submission.json` and `outcome.json` are identical, not merely an asserted boolean. This is recorded input stability, not a fresh all-repository/cache audit.

Exact common features: `aura-containers`, `aura-instance-enumeration`, `aura-xml-widgets`, `base-spell-relationships`, `casc`, `client-retail`, `default`, `forbidden-aspects`, `gui`, `native-duration-formatting`, `numeric-rule-formatters`, `on-update-modes`, `player-cast-durations`, `prefork-full-ui`, `profile-retail`, `retail-12-0-0`, `retail-12-0-5`, `retail-12-0-7`, `retail-12-1-0`, `rodio`, `sound`.
Build profiles: debug assertions and overflow checks enabled, line-tables-only debuginfo, opt-level 1; test=true for both test artifacts, test=false for normal wow-sim.

## Exact proof counts and boundaries

| Saved evidence | Authenticated result | Scope |
|---|---|---|
| `garrison.stdout`, `execution-results.json` | 4 passed, 0 failed, 0 ignored, 0 measured, 10730 filtered out; exit 0 | Four unique Garrison/color cases |
| `garrison-original.stdout`, `execution-results.json` | 1 passed, 0 failed, 1 total; exit 0 | Original exact-name prefork Garrison load boundary |
| `lua-errors.stdout`, `startup-result.json`, `lua-errors.stderr` | Exact JSON `[]`; exit 0; CLEAN, 0 unique / 0 occurrences | Normal non-test Retail startup |
| `checks.json`, `fmt.stdout`, `fmt.stderr` | fmt exit 0; both streams empty | Saved cargo fmt --all --check |
| `checks.json`, `default-check.stdout`, `default-check.stderr` | check exit 0 | Saved offline/locked default Cargo check |

Four unique integration names:

1. `garrison_damage_classes::adventures_combat_log_colors_bootstrap_with_mists_row_rgb_palette`
2. `garrison_damage_classes::garrison_adventures_combat_log_consumes_fire_school_mask_not_damage_ordinal`
3. `garrison_damage_classes::garrison_damage_classes_returns_fresh_tables_and_rows_without_mutation_leaks`
4. `garrison_damage_classes::garrison_damage_classes_returns_one_table_with_en_us_pure_school_masks`

The consumer's child-process output reports 1 passed / 10733 filtered out; it is the same named consumer case, not a fifth integration test. Original prefork output's subsequent three zero-test summaries contribute no additional tests. Total unique executed cases: **5**, not 6.

Consumer source `/home/osso/Projects/wow/wow-ui-sim/tests/garrison_damage_classes.rs` asserts authentic cached `AdventuresCombatLogMixin`, runs `OnLoad`, and checks `damageTypeKeys[4] == 'Fire'` and `[2] == 'Holy'`. It then invokes `AddCombatEvent` twice with fixture spell school masks 4 and 2. Final assertions require exactly two messages, exact localized-format output with `Caster`, `Fixture Spell`, `Target`, 37, and respectively `Fire` / `Holy`, and unequal messages. UI wiring and spell input are collaborators; the damage-class query and consumer methods are not replaced. Passing the named consumer case therefore covers final rendered-string assertions, not merely catalog lookup.

Original Garrison case `/home/osso/Projects/wow/wow-ui-sim/tests/blizzard_garrison_ui_loads.rs:259-290` clears recorded errors before explicit load, then asserts no Garrison/OrderHall/BFA/Covenant/Adventures errors after excluding the intentional 3D-model gap. It does **not** assert globally empty stderr. Its source bytes match the epoch's recorded input hash.

Normal startup uses the sealed normal binary with `--no-addons --no-saved-vars lua-errors`, `WOW_SIM_CASC=0`, `WOW_SIM_NO_SOUND=1`, and warm profile cache. `lua-errors.stderr` reports 291 Blizzard addons loaded and ends with CLEAN / 0 unique / 0 occurrences / 0 attributed owners / 0 unattributed occurrences. This is not an all-addons, sound, CASC, GUI-rendering, or native-client proof.

## Diagnostics and revision limits

`garrison.stderr` contains 29 `Lua error:`-prefixed records; `garrison-original.stderr` contains 13. These are diagnostic records, not unique error counts. They include nil surfaces and intentionally unsupported 3D-method errors. They do not invalidate the named assertions, but prohibit claiming both test loads emitted no Lua diagnostics. Normal startup stderr contains zero such records. Compilation and default check retain six deprecated iced-wgpu manifest lint-key warnings; exit 0 is not warning-free proof.

Read-only git comparison used explicit `-C /home/osso/Projects/wow/wow-ui-sim`. From the proven revision to `ba413e8e1`, the code/build/tests path comparison changes only `/home/osso/Projects/wow/wow-ui-sim/tests/garrison_damage_classes.rs`: move Retail gating from the entire file to the `common` import and consumer test. Reversing precisely that cfg-only patch in memory reproduces the epoch's test-source SHA-256. The four Retail test bodies are unchanged; runtime source, crate code, Cargo inputs, build.rs, and original Garrison test have no endpoint changes in that comparison. Additional endpoint changes are documentation/evidence, not runtime code.

Thus the saved Retail behavioral proof is scope-equivalent for those unchanged bodies at `ba413e8e1`; **no binary at that later SHA was built or executed here**, and saved fmt/check proof remains tied to the original revision. Newly ungated pure-profile matrix remains **pending**. No all-profile or native catalog/order/palette parity claim. No reauthentication of the separately completed 2298-row source/data/readability assessment.

**OVERALL: PASS for authenticated saved Retail runtime scope only.**
