# Independent saved full-suite469 audit

**Saved suite: FAIL. Count/identity reconciliation: PASS.**

Read and followed verify/SKILL.md as assigned independent verifier. Entire original JSON and all 16,513 log lines read in memory; only sanitized derived counts, selectors and references retained. No execution or operational checks.

## Scope

Originals: `/home/osso/Projects/wow/full-suite-results/469a92d27bd2c45e590b1d22bf9137ce4c70f3d1.json` and `/home/osso/Projects/wow/full-suite-results/469a92d27bd2c45e590b1d22bf9137ce4c70f3d1.log`. Exact saved source **469a92d27bd2c45e590b1d22bf9137ce4c70f3d1**, default-feature Retail simulator scope. Started 2026-10-10T19:20:41-0500; finished 2026-10-10T19:42:20-0500 (October 11, 2026, 00:42:20 UTC). Saved status `unit2-fullsuite-current/status-20261011-next.json` exits 0 and contains finished original JSON, equal to original on inspection. Status-command success is not suite success.

Later current **b4eecb511** is the docs/evidence-only accepted retention successor, not the recorded suite revision. This distinction uses accepted retention evidence; no source/binary rehash or new revision-map gate. Do not relabel source469 executions as fresh b4 execution.

## Exact executed outcomes

| Segment | PASS | FAIL | Skipped | Executed | Evidence (log lines) |
|---|---:|---:|---:|---:|---|
| Integration | 10,696 | 2 | 19 | 10,698 | 13, 11411; exit100 |
| Prefork main | 2,320 | 1 | 0 recorded | 2,321 | 11417, 13988 |
| Prefork auxiliary | 4 | 0 | 0 recorded | 4 | 13989–14001 |
| Lib | 2,001 | 0 | 0 | 2,001 | 14509, 16512; exit0 |
| **Total** | **15,021** | **3** | **19** | **15,024** | **FAIL** |

Prefork combined: **2,324 PASS / 1 FAIL / 2,325 executed**, overall exit1. Auxiliary batch sizes 2+1+1, all pass. **15,043** selected/reported slots including skips, not executions. Prefork summaries expose no skip field; no ignored event recorded.

Integration ordinals 1–10,698 and lib 1–2,001 each occur exactly once before their summaries. Two repeated nextest FAIL recaps at 11412–11413 excluded. Count executions per batch, not unique names; diagnostics and internal child output are not additional runner slots. Parsed failure sets match original JSON.

## Current unit2 two plus Boolean: 3/3 PASS

- `frame_creation::visibility_scripts::test_set_shown_boolean_transitions_update_frame_state` — log:5856.
- `unit_event_second_unit::reregistering_and_unregistering_clear_the_second_unit_filter` — log:10757.
- `unit_event_second_unit::second_unit_filter_receives_modeled_player_and_target_health_events` — log:10758.

Both unit2 cases and Boolean transition control are actual integration events, not discovery/filter counts. No prefork execution of these three selectors found.

## Eight Retail legacy-aura selectors: 8/8 PASS

- `aura_api::test_unit_aura_harmful_returns_nil` — log:853.
- `aura_api::test_unit_buff_non_player_returns_nil` — log:854.
- `aura_api::test_unit_buff_count_between_4_and_8` — log:855.
- `aura_api::test_unit_aura_helpful_returns_data` — log:856.
- `aura_api::test_unit_buff_past_end_returns_nil` — log:857.
- `aura_api::test_unit_buff_returns_data_for_player` — log:858.
- `aura_api::test_unit_buff_returns_all_fields` — log:859.
- `aura_api::test_unit_debuff_returns_nil` — log:860.

These eight integration executions are already included in totals. Existing assertions in `tests/aura_api.rs:14–98` cover nonempty default player buff/HELPFUL names; non-nil name/spellId/duration only (not full tuple equality); nil first results for index100, target buff, default HARMFUL/debuff; contiguous buff count4–8. No native/caster fallback/arena/player-owned-ordering/populated-harmful claim. Separate historical-profile accepted receipts are not added to this Retail suite count.

## Four original prefork auxiliary bodies: 4/4 PASS

- `chat_frame::test_chat_editbox_click_type_and_submit` — log:13990; `tests/chat_frame.rs:353–393`. Actual fixture body: initial unfocused state, click, typed text, ENTER, sent message and SAY type, cleared editbox.
- `chat_frame::test_chat_editbox_text_color_after_activation` — log:13991; `tests/chat_frame.rs:442–463`. Actual fixture body: activation, white RGB and alpha 1 within 0.01.
- `spell_casting::cast_bar_respects_edit_mode_lock_setting_after_startup_fix` — log:13995; `tests/spell_casting.rs:447–561`. Retail branch: unlocked setting 0, frame-manager parent and detached state, player-anchor success without attachment, locked cast-anchor success with PlayerFrame attachment. Mists-only branch not proved.
- `blizzard_player_spells_loads::mainline_spellbook_keybind_opens_and_closes_without_runtime_errors` — log:13999; `tests/blizzard_player_spells_loads.rs:210–248`. Actual fixture body: clear Lua errors, S then 10 ticks opens published spellbook, S then 10 ticks closes it, no collected Lua errors. Other profiles not proved.

Inspected fixture bodies, not merely no-argument wrappers. Spellbook execution is present in its own one-case batch, not inferred from profile cfg or registry listing. No widening to Mists/PTR/Forever execution.

## Three failures and accepted714/956 comparison

- `method_diff_coverage::diff_methods_extra_snapshot_matches_current_metatable_surface` — FAIL log:6844; diagnostic log:6870, `tests/method_diff_coverage.rs:264:5`.
- `method_diff_coverage::diff_methods_missing_snapshot_matches_current_metatable_surface` — FAIL log:7465; diagnostic log:7491, `tests/method_diff_coverage.rs:264:5`.
- `blizzard_garrison_ui_loads::blizzard_garrison_ui_loads_explicitly_via_load_addon_without_errors` — FAIL log:12061; diagnostic log:12062, `Blizzard_GarrisonUI/Mainline/Blizzard_AdventuresCombatLog.lua:90`.

Method failures are extra/missing metatable-snapshot disagreements; this count audit does not establish semantic correctness of either side. Garrison retains the non-model nil-table `ipairs` failure at the referenced vendor location; unrelated model diagnostics do not erase it.

**All three identities already failed in both accepted714 and accepted956 reports.** Current failure set equals956; no newly observed failure identity versus either accepted audit. JSON's mutable `new_failures.prefork` Garrison label is **not regression evidence**.

714 also failed `addon_loading_perf::heaviest_blizzard_addons_stay_within_per_addon_load_budgets`; current actual integration PASS log:2252. One passing execution is not a timing-stability or causal-fix claim. Executed total grows714→469 by3 (15,021→15,024), and956→469 by9 (15,015→15,024); counts do not establish causality. Accepted reports used only for failure/count comparison, not map/provenance revalidation.

## Boundary

No replay/submission/wait/poll/build/test/live operations/delegation/Bash/rg/fd. No source or binary hashing. No native parity, all-profile acceptance, hermetic closure, current-HEAD fresh execution, startup-cleanliness or overall handoff completion. Full suite remains **FAIL**, despite requested selectors passing.
