# Independent saved-artifact runtime gate — PARTIAL PASS / consumer FAIL

Date: 2026-10-11
Artifact epoch: `20261011T174814Z`
Artifact root: `/home/osso/.local/state/wow-ui-sim/verification/addon-compatible-green-current/20261011T174814Z`
Proven revision: `b5802042013b84a8311de53b374a502d450518ab`

Followed `/home/osso/AgentConfig/skills/verify/SKILL.md` as the independent verifier, using saved-artifact evidence only. No tests, builds, compiler checks, formatting runs, source edits, operations, delegation, waiting or polling performed. Read-only Git queries authenticated recorded inputs. This report is the only written file.

## Authentication and proof boundaries

`compile-submission.json` records the exact revision and default-feature command: `/usr/bin/cargo test --offline --locked --jobs 12 --test integration --test prefork_full_ui --no-run --message-format=json`. `compile-result.json` records exit 0; `compile.stdout` contains `build-finished` success=true. All 18 recorded input hashes match the corresponding files retrieved directly from the recorded Git revision. Before/after input maps are identical; `outcome.json` records `inputs_unchanged=true`.

The checkout HEAD observed during this audit is `31c2e3e09924ed95eed82caba6af1d3fbb8b4762`, not b58020420. This gate proves the sealed b58020420 scope, not arbitrary later checkout work or the whole current handoff.

No prior independent binary authentication record was established by this audit. Per the permitted first independent seal, each saved binary was hashed once; all three SHA-256 values and byte counts match `artifacts.json`. No duplicate binary hashing performed. Each has exactly one matching Cargo compiler-artifact record, with identical executable path, feature list and build profile.

| Saved artifact | SHA-256 | Bytes | Cargo profile |
|---|---|---:|---|
| `integration-test` | `f4bbc3a574d0176a711bb72af2d0dcad5e1ea40542fa665c822e8452933c7b68` | 467406472 | test=true |
| `prefork_full_ui-test` | `f36ca996d7ec369d2a0d4b798beefb63699aaddc52f2e9e876a52379163504ff` | 509410736 | test=true |
| `wow-sim-normal` | `fe505658b0fcbca2043a959d9f41b981aa6320346ff9e0e135ffeb19f94620f4` | 407216392 | test=false |

All three profiles record debug_assertions=true, overflow_checks=true, opt_level=1, debuginfo=line-tables-only: these are optimized debug artifacts, not release builds. All share exactly these features:

`aura-containers, aura-instance-enumeration, aura-xml-widgets, base-spell-relationships, casc, client-retail, default, forbidden-aspects, gui, native-duration-formatting, numeric-rule-formatters, on-update-modes, player-cast-durations, prefork-full-ui, profile-retail, retail-12-0-0, retail-12-0-5, retail-12-0-7, retail-12-1-0, rodio, sound`.

This authenticates Retail/default with the retail-12-1-0 secret-argument path enabled; it does not prove PTR or other client profiles. `prefork-full-ui` is a Cargo feature shared by the normal binary, not proof that it is a test executable: the normal wow-sim Cargo record explicitly has test=false.

Endpoint identity is grounded by the producers and tests: `src/c_api/c_achievement_info_relations.rs:8-15` registers `C_AchievementInfo.GetSupercedingAchievements`; `src/c_api/c_garrison_damage_classes.rs:20-26` registers `C_Garrison.GetAutoCombatDamageClassValues`. Saved achievement and Garrison selectors exercise these literal endpoints. Existing source/wiring/readability PASS is retained from `source-report.md` (the supplied source2290 PASS); no duplicate readability audit performed.

## Unique runtime coverage

Every recorded runtime command uses `/usr/bin/timeout 90`, the corresponding sealed executable, `--nocapture --test-threads=1` for tests, and exact matching for prefork cases. The worker records `WOW_SIM_CASC=0` and `WOW_SIM_NO_SOUND=1` for runtime invocations. Counts below come from full saved stdout and execution-result exit codes, not repetition of child output.

| Scope / selector | Unique result | Exit | Evidence |
|---|---:|---:|---|
| `c_achievement_info_probes::` | 72 PASS, 0 FAIL | 0 | `achievement.stdout`; includes all 8 superceding probes |
| `achievements_api::` | 44 PASS, 0 FAIL | 0 | `reward-preservation.stdout`; includes `test_reward_item_id_tracks_supplied_achievement_state` |
| `garrison_damage_classes::` | 2 PASS, 1 FAIL | 101 | `garrison.stdout`, `garrison.stderr` |
| Original `blizzard_garrison_ui_loads::blizzard_garrison_ui_loads_explicitly_via_load_addon_without_errors` | 1 PASS | 0 | `garrison-original.stdout` |
| `patch_8_1_0_publication_sweep::patch_8_1_0_publication_sweep` | 1 PASS | 0 | `publication-810.stdout` |
| `patch_9_0_1_publication_sweep::patch_9_0_1_publication_sweep` | 1 PASS | 0 | `publication-901.stdout` |

Total: **122 unique tests: 121 PASS, 1 FAIL**. Garrison consumer appears in both parent and timeout-child output: one unique failed test, not two. Prefork zero-test trailing summaries are not additional cases.

All eight superceding probes PASS: absent relation/single empty table; supplied distinct sequences including duplicates; updates/removal; isolated results; nonrepresentable-number non-aliasing; numeric-input requirement; untainted-secret acceptance; tainted-secret rejection without changing taint. The broader 72-test achievement count includes these eight, not 72+8. Reward preservation is covered by the separate 44-test suite, including the supplied-state reward case.

The two Garrison producer tests PASS: one table with readable enUS pure school masks; fresh outer tables and rows with mutation isolation. This establishes the simulator contract, not native catalog/order/locale parity.

## Consumer failure — not all Garrison GREEN

Unique failed case:

`garrison_damage_classes::garrison_adventures_combat_log_consumes_fire_school_mask_not_damage_ordinal`

Saved `garrison.stderr:305-306`:

> panicked at /home/osso/Projects/wow/wow-ui-sim/tests/garrison_damage_classes.rs:180:18
> Blizzard_AdventuresCombatLog.lua:129: attempt to index local 'textColor' (a nil value)

The parent reports child exit 101 at `tests/common/timeout_reexec.rs:184:9` (`garrison.stderr:341-342`). This is a Lua consumer failure, not a timeout.

The inspected Retail profile cache consumer at `/home/osso/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/Blizzard_GarrisonUI/Mainline/Blizzard_AdventuresCombatLog.lua:116-129` reads `self.damageTypeKeys[spellInfo.schoolMask]`, derives `textColor` through `GetCombatLogTextColor`, and dereferences `textColor:GetRGB()` when adding a message. Its color helper selects ADVENTURES_COMBAT_LOG_GREY/BLUE/ORANGE. The saved failure establishes nil `textColor` at that dereference; it does not independently establish why the selected color value was absent. No vendor/source fix attempted.

The test fixture calls authentic `OnLoad`, checks mask 4='Fire' and mask 2='Holy', then calls authentic `AddCombatEvent`. Those preceding assertions were reached before the saved later failure, but final Fire/Holy rendered-message assertions are **not proven**. Therefore authentic consumer completion remains FAIL despite producer PASS.

The original Garrison prefork load case independently PASSes. Its implementation (`tests/blizzard_garrison_ui_loads.rs:259-289`) filters Garrison-related errors and exempts intentional 3D model gaps. Its saved stderr contains model-method diagnostics. Do not reinterpret this PASS as zero raw stderr errors or as proof that the separate combat-log consumer succeeds.

Prior RED: caller-accepted pre-producer epoch173738 at `a0833b2dd`, 8 achievement and 3 unique Garrison failures, retained in `source-report.md`. This audit does not independently authenticate or replay those historical logs. Current saved evidence changes the eight achievement probes to PASS and two Garrison producer cases to PASS; the consumer now has the recorded nil-textColor boundary. No claim that all prior Garrison failures are GREEN.

## Normal startup and main-owned checks

`startup-result.json` invokes sealed **normal, non-test** `wow-sim-normal --no-addons --no-saved-vars lua-errors`, with CASC/audio disabled as above, warm Retail cache. Exit **0**. `lua-errors.stdout` is exactly `[]\n`; saved stderr final summary records **0 unique, 0 occurrences**, including zero unattributed errors. This is normal startup proof, not a prefork-test surrogate, full third-party-addon proof, cold-cache proof or Garrison LoD interaction proof.

At this audit's single saved-artifact read, `checks.json` already records main-owned fmt exit 0 and default check exit 0. `default-check.stderr` records `Finished dev profile ... in 23.86s`, plus six iced-wgpu-patched manifest deprecation warnings. The recorded command is `/usr/bin/cargo check --offline --locked --jobs 12`; fmt is `/usr/bin/cargo fmt --all --check`. No check rerun, worker polling or waiting performed. These are saved worker results, not verifier-run checks; no claim of warning-free compilation. A separate pending check outside this artifact epoch is not certified here.

## Verdict

**PARTIAL PASS; authentic Garrison combat-log consumer FAIL.** Sealed revision/features/profile identities independently authenticated; achievement probes, reward preservation, two Garrison producer cases, original Garrison prefork load, both publication cases, and normal startup have the bounded PASS evidence above. Readability remains the prior source PASS without duplicate audit. Whole-handoff completion and all-Garrison-GREEN claims are rejected.
