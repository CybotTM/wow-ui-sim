# Saved full-suite gate — FAIL; parent OPEN

Verified 2026-10-11 as assigned independent verifier; read/followed `/home/osso/AgentConfig/skills/verify/SKILL.md` first. No delegation, runtime/build/check execution, live status/poll, network, operations, cwd changes, or binary/source hashing. Only report/countproof written.

## Exact run and evidence

Read entire saved log in memory and parsed phase boundaries and named terminal rows:

- `/home/osso/Projects/wow/full-suite-results/ba413e8e177f04dc6fc7491834606ce36c750954.json`
- `/home/osso/Projects/wow/full-suite-results/ba413e8e177f04dc6fc7491834606ce36c750954.log` — 2,104,914 bytes; 16,079 lines.

JSON ref/sha: `ba413e8e177f04dc6fc7491834606ce36c750954`. Started **2026-10-11 15:07:31-0500**, finished **15:48:40-0500**. Actual unit **full-suite-1791749250.service**, known ended main inactive/dead0, supplied by caller; not independently live-polled. Unit main0 does not override failed test-phase exits.

Reused already-verified submission epoch **20261011T200730Z / gate2367** via `/home/osso/.local/state/wow-ui-sim/verification/query-final-suite-submission-retention-current/report.md`; no rehash or reauthentication of old receipts/matrix. Retained submission specifies this exact SHA and actual unit, invocation `2b8e8907e6dd49468ce93711dea413e7`, submission exit0. Submission success is not suite success.

## Counts — authenticated from named rows

| Phase | PASS | FAIL | Skipped | Executed | Exit | Evidence |
|---|---:|---:|---:|---:|---:|---|
| Integration | 10,713 | 2 | 19 | 10,715 | 100 | header line1; summary11182 |
| Prefork main | 2,321 | 0 | none recorded | 2,321 | phase0 | lines11188–13511 |
| Prefork auxiliary | 4 | 0 | none recorded | 4 | included in phase0 | blocks2+1+1, lines13512–13524 |
| Lib | 2,000 | 1 | 0 | 2,001 | 100 | header14016; summary16076 |

Total **15,038 PASS / 3 FAIL / 15,041 executed**, plus19 integration skips. These are phase executions, not globally distinct tests; prefork can repeat integration capabilities.

Integration terminal ordinals cover exactly1–10715, lib1–2001, one unique `(binary,test)` per ordinal. Raw nextest terminal-row counts10717 and2002 include respectively2 and1 repeated failure-summary rows; excluded. Repeated statuses agree. Child-process `running1`, `FAILED` rows and inner summaries are diagnostic output, not additional phase cases. Prefork has2325 distinct named `test ... ok` rows, matching four run/result blocks2321+2+1+1. Failure names match saved JSON exactly. Skipped19 is authenticated by nextest start/summary, not19 invented named rows.

Prefork auxiliary names, each one PASS:

- `chat_frame::test_chat_editbox_click_type_and_submit` (13513)
- `chat_frame::test_chat_editbox_text_color_after_activation` (13514)
- `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix` (13518)
- `blizzard_player_spells_loads::mainline_spellbook_keybind_opens_and_closes_without_runtime_errors` (13522)

## Requested query/bootstrap coverage — PASS, already inside totals

Original Garrison case `blizzard_garrison_ui_loads::blizzard_garrison_ui_loads_explicitly_via_load_addon_without_errors`: **one prefork PASS**, line11832. Not added again to integration counts.

Eight achievement cases under `c_achievement_info_probes::superceding::`, all integration PASS:

| `get_superceding_achievements_` suffix | Log line |
|---|---:|
| accepts_untainted_secret_argument | 3631 |
| does_not_alias_unrepresented_numbers | 3634 |
| observes_updates_and_removal | 3635 |
| returns_isolated_tables | 3636 |
| projects_distinct_supplied_sequences | 3637 |
| rejects_tainted_secret_without_changing_taint | 3638 |
| requires_a_number | 3639 |
| returns_one_empty_table_for_absent_relation | 3640 |

Four `garrison_damage_classes::` cases, all integration PASS:

| Case | Log line |
|---|---:|
| adventures_combat_log_colors_bootstrap_with_mists_row_rgb_palette | 5926 |
| garrison_damage_classes_returns_fresh_tables_and_rows_without_mutation_leaks | 5927 |
| garrison_damage_classes_returns_one_table_with_en_us_pure_school_masks | 5928 |
| garrison_adventures_combat_log_consumes_fire_school_mask_not_damage_ordinal | 6031 |

These13 requested executions are subsets of the phase totals. In particular, bootstrap colors are one of the four Garrison cases, not a fifth case. Saved green consumer result does not rewrite historical red receipts or prove native palette parity.

## Failures — exact observed boundaries

1. `startup_perf::full_game_startup_stays_under_budget`: log9905–9912, `tests/startup_perf.rs:93:13`. Raw: `full game startup took 96.24s, exceeding budget 20.00s`. Timeout wrapper at `tests/common/timeout_reexec.rs:184:9` reports child exit101; one integration failure, not two.
2. `talent_panel_onupdate_perf::talent_panel_open_onupdate_tick_latency_stays_under_budget`: log10153–10160, `tests/talent_panel_onupdate_perf.rs:166:13`. Baseline1.18ms, talent25.65ms, max45.69ms; delta **24.47ms**, derived from displayed rounded averages, above20.00ms. Average<35ms and max<75ms are not the failed boundary. Raw: `talent-panel open average OnUpdate tick regressed too far over baseline (baseline 1.18ms, talent 25.65ms, delta budget 20.00ms)`. Wrapper child exit101 is not another case.
3. `asset_resolver_config::tests::gui_resolution_cache_preparation`: log14126–14147, child `src/asset_resolver_config.rs:100:64`, parent `:168:9`. Exact raw child failure:

```text
cold preparation: "prepare GUI CASC resolution cache for /syncthing/World of Warcraft: read encoding file 53dcc8beeb4c9e75b69c5f2a8a67c7e4: Content not found: Archive location not found for encoding key: 53dcc8beeb4c9e75b69c5f2a8a67c7e4; key-aware local archive fallback also failed: missing archive location for encoding key 53dcc8beeb4c9e75b69c5f2a8a67c7e4"
```

Parent reports `cold preparation failed:` and embeds/repeats child output. This is one lib failure, not multiple child/parent failures. Observed cache-preparation/content lookup failure is not proof of a code regression or complete diagnosis of installed CASC state.

## Current source equivalence — PASS within tracked scope

`git -C /home/osso/Projects/wow/wow-ui-sim rev-parse HEAD`: `e45f95517cae150bbb71bb937bf487cbf8689c56`.

`git diff ba413e8e177f04dc6fc7491834606ce36c750954 HEAD -- src tests Cargo.toml Cargo.lock build.rs .cargo scripts`: empty, exit0. Complete committed changed-path inspection found134 paths, all `docs/` or `data/patch-api/evidence/`; no source/test/Cargo/build changes. Thus post-ba changes are documentation/retained-evidence only.

Read-only actual suite checkout `/home/osso/Projects/wow/full-suite-checkout` is available: `git -C ... rev-parse HEAD` pins **ba413e8e177f04dc6fc7491834606ce36c750954**, exit0. Saved compile lines9–10 also name that checkout. Both canonical-vs-e45 and suite-checkout-vs-ba tracked working diffs are empty for `src tests crates Cargo.toml Cargo.lock build.rs .cargo scripts build`, exit0. This establishes inspected tracked source equivalence now, not historical cleanliness, untracked-input absence, executable identity, toolchain identity or artifact provenance.

## Acceptance boundary

**Count/source-equivalence authentication PASS. Saved full-suite execution FAIL. Parent OPEN.** `new_failures` repeats the three failures in JSON; it is metadata, not evidence that new source caused them. No binary/artifact identity, native parity, cache provenance/content, or new profile-matrix acceptance inferred. No rerun or operational remediation performed. Machine-readable counts, exact selected names/ordinals, source commands and scope limits: `countproof.json` beside this report.
