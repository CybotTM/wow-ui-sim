# Saved-receipt independent audit — FAIL

## Scope and evidence

Audited only saved receipts; no builds, test execution, status polling, repository operations, source edits, or delegation. Read and followed the assigned verify skill as the assigned independent verifier, adapted to the explicitly requested receipt-only evidence boundary.

Receipt directory remains local; only sanitized small evidence is retained.

- Current: `956b699a580c8e85ac42d31229b5b58ab463a52b.{json,log}`. JSON records exact SHA, start `2026-10-10T15:24:37-0500`, finish `2026-10-10T15:49:58-0500`.
- Historical comparison: `b048251f38b2d53e33b3365590cd227d40fc07d0.{json,log}`. Finished `2026-10-10T14:01:43-0500`.
- Full logs read and parsed: current 16,506 lines; historical 16,513 lines. References below use 1-based log lines.
- Current log SHA-256: `46748fa7fd716fdd576199e5370f6445c24ef7b2996d19d19faf9ed57c63471c`.
- Historical log SHA-256: `e326b07a64ef4d723d7a75f0cd759f3fa8f84b7685188994ee3f5b4e24670398`.

## Actual recorded outcomes

| Recorded batch | PASS | FAIL | Skipped | Executions | Step exit | Current log evidence |
|---|---:|---:|---:|---:|---:|---|
| Integration | 10,687 | 2 | 19 | 10,689 | 100 | 1, 13, 11404 |
| Prefork primary | 2,320 | 1 | 0 recorded | 2,321 | 1 overall | 11409–11410, 13981 |
| Prefork auxiliary chat batch | 2 | 0 | 0 recorded | 2 | included above | 13982–13986 |
| Prefork auxiliary cast-bar batch | 1 | 0 | 0 recorded | 1 | included above | 13987–13990 |
| Prefork auxiliary spellbook batch | 1 | 0 | 0 recorded | 1 | included above | 13991–13994 |
| Lib | 2,001 | 0 | 0 | 2,001 | 0 | 14490, 14502, 16505 |
| **Combined, without selector deduplication** | **15,012** | **3** | **19** | **15,015** | **FAIL** | sum of recorded batches |

Prefork total including auxiliary batches: **2,324 PASS, 1 FAIL, 2,325 executions**. Its summaries do not expose a skip field; zero skipped outcomes are recorded, not an independent assertion about excluded selectors.

Counting method: parse per-execution nextest PASS/FAIL lines before each Summary and prefork `test … ... ok/FAILED/ignored` lines; independently reconcile summaries. Retain every execution across batches and targets even when selectors overlap. Do not count nextest's repeated final failure recap or embedded failed-test stdout as additional executions. Integration has four printed nextest FAIL lines but only two failed executions. Skips are not executions: combined executed plus recorded skipped is **15,034**, not 15,034 executed tests. Integration's 19 skips are reported in its summary, not enumerated by this receipt.

Auxiliary selectors, all recorded `ok`:

- `chat_frame::test_chat_editbox_click_type_and_submit` and `chat_frame::test_chat_editbox_text_color_after_activation` (13983–13984).
- `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix` (13988).
- `blizzard_player_spells_loads::mainline_spellbook_keybind_opens_and_closes_without_runtime_errors` (13992).

## Failure selectors and historical comparison

| Selector | b048 recorded result | 956 recorded result | Evidence |
|---|---|---|---|
| `method_diff_coverage::diff_methods_extra_snapshot_matches_current_metatable_surface` | FAIL | FAIL | historical 6827, 11392; current 6891, 11406 |
| `method_diff_coverage::diff_methods_missing_snapshot_matches_current_metatable_surface` | FAIL | FAIL | historical 7448, 11393; current 6836, 11405 |
| `blizzard_garrison_ui_loads::blizzard_garrison_ui_loads_explicitly_via_load_addon_without_errors` | FAIL | FAIL | historical 12041; current 12054 |
| `iced_app::update::update_tests::main_thread_cpu_time_advances_with_busy_work` | FAIL | **PASS** | historical 14852, 14867; current 14865 |

Current method failures panic at `tests/method_diff_coverage.rs:264:5` (6862, 6917). Messages: `diff_methods_missing.txt is out of sync with the current metatable surface.` (6863) and `diff_methods_extra.txt is out of sync with the current metatable surface.` (6918).

Current Garrison failure reports `bad argument #1 to 'ipairs' (table expected, got nil)` at `Blizzard_GarrisonUI/Mainline/Blizzard_AdventuresCombatLog.lua:90` (12055). Historical log records the same error location and message (12042). This establishes a previously recorded failure, not its underlying cause or introduction revision.

Historical totals, also retaining auxiliary executions: integration 10,682 PASS / 2 FAIL / 19 skipped; prefork 2,324 PASS / 1 FAIL; lib 2,000 PASS / 1 FAIL. Combined **15,006 PASS / 4 FAIL / 19 skipped / 15,010 executions** (historical summaries 11391, 13968, 13973, 13977, 13981, 16510). Current receipt has five more integration executions and the CPU selector now records PASS. This is one saved passing execution, not proof that the CPU test cannot recur or that a specific change fixed it.

## JSON reconciliation and proof limits

Current JSON matches the recorded integration exit 100 with two method-snapshot failures, prefork exit 1 with Garrison failure, and lib exit 0 with no failures. No additional failed executions were found in the outcome records. This does not imply absence of other diagnostic errors or warnings.

JSON `new_failures.prefork` lists Garrison, but its mutable-baseline comparison does **not** establish a new regression. The b048 receipt already records the same failed selector and diagnostic. Do not reinterpret `new_failures` as immutable historical provenance.

Evidence applies only to exact recorded revision **956b699a580c8e85ac42d31229b5b58ab463a52b**. As explicitly scoped by the assignment, it excludes later d97 scalar-secret probes and any future producer; neither is validated by these receipts. No later checkout or producer state was inspected.

**Overall saved-suite result: FAIL.** This audit provides recorded simulator-test evidence only: no suite-passing claim, whole-goal completion claim, native-client behavior proof, or proof of later revisions.
