# Bullet / dressability final bounded existing-runtime gate

**PASS — 34 unique names × Retail/PTR = 68 passing profile-target outcomes.**
Existing evidence only. No rebuild, test/check/fmt rerun, authentication, services, delegation, Bash, cwd tools, or repository edits.
Recorded runtime timestamps are **October 14, 2026 UTC**; epoch names retained literally (local session date supplied as October 13).

## Coverage matrix

| Bounded scope | Source | Retail | PTR | Proof level |
|---|---|---:|---:|---|
| Corrected Bullet producer/query/security + cached consumer | bde7ea277 | 7 PASS | 7 PASS | Full six-file evidence; outer 7/0; nested consumer 1/0 included |
| Original Bullet public API | 69f2569cb | 2 PASS | 2 PASS | Separate producer gate + original six-file evidence |
| Dressability producer | 69f2569cb | 5 PASS | 5 PASS | Separate producer gate + original six-file evidence |
| Whole cached PlayerChoice dressability/mount consumer | 69f2569cb | 1 PASS | 1 PASS | Separate producer gate; complete 72,107 / 71,662-byte stderr |
| Widget state | 69f2569cb | 6 PASS | 6 PASS | Full six-file preservation evidence |
| Widget security | 69f2569cb | 1 PASS | 1 PASS | Full six-file preservation evidence |
| Widget lifecycle | 69f2569cb | 2 PASS | 2 PASS | Outer 2/0; two child 1/0 summaries included, not extra |
| Mount relations | 69f2569cb | 8 PASS | 8 PASS | Full six-file preservation evidence |
| PlayerChoice boolean / live mount relation | 69f2569cb | 1 + 1 PASS | 1 + 1 PASS | Complete 64,427 / 67,450 and 63,979 / 67,007-byte consumer stderr |
| **Bounded total** | **Mixed revisions, explicit reuse** | **34 PASS** | **34 PASS** | **68 outcomes, no duplicate names per profile** |
| Normal startup (separate, not part of 68) | 69f2569cb | PASS | PASS | One nil Bullet return; one false dress return; marker; [] JSON errors; CLEAN 0 unique / 0 occurrences |

All 20 selected test launches: admitted, launcher success/exit 0, matching argv/result/receipt metadata, independently parsed stdout counts, zero failed/ignored/measured. Filtered tests are excluded, not PASS. Nested child execution is not another logical target. Two startup launches are not test-suite passes (expected_cases=0).

## Corrected failure boundary

Both bde cached fixture helpers call `env.exec(code)` and inspect collected Lua errors. Feed test PASS now includes the single-line reset `bulletRejectedEvents = 0`, type-mismatch rejection, unchanged Bullet map/set/membership, no-notification assertion and absent-membership removal; these assertions follow the previously failing reset. Missing-set rejection and synchronous event-before-observation checks also pass. Source inspection plus recorded passing test outcome establishes reachability; no separate per-assertion tracing claimed.

Prior Retail source69 remains **6 PASS + 1 fixture parser failure, exit 101**, including child consumer 1 PASS already counted in outer 6. Its later rejection assertions were unreached then. Later bde evidence is additive, not a rewrite of historical failure. Three earlier Retail denial-only attempts remain ADMISSION DENIED, not test failures or passes.

## Full diagnostics, not globally clean compatibility

Every full stdout/stderr/argv/result/admission/launcher file consumed. `proof.json` embeds originals and every selected stderr line with category/line number; no head/tail sampling of 64–72 KiB consumer logs.

Each of six PlayerChoice consumer runs (boolean, live mount, dressability across two profiles) retains **77 NilSymbolObservation records across 20 addon rows** and **18 MissingRequirement records across 9 rows**. These are nonempty compatibility diagnostics, not loader warnings or Lua exceptions. Keep their full names, attribution, source and line records in proof; no globally clean-runtime claim.

Each consumer has 59 empty loader-warning snapshots. Boolean/live-mount/dressability respectively have 60/62/65 empty snapshots for each Lua-error, Lua-error-record and Lua-error-count category. Supplied behavior passes despite retained nil/missing diagnostics. Remaining selected test stderr contains startup timings and corrected Bullet cache paths only; no logged nonempty Lua errors/warnings, skipped tests, timeout, OOM, abort or kill failure indicators. Successful launch metadata does not prove resource headroom outside these executions.

Normal startup is independently inspected and separately gated: 46 stdout bytes each, 14,702 / 14,697 stderr bytes, 219 lines each, zero recorded Lua errors. Headless, third-party addons/SavedVariables disabled, CASC off, sound off, recorded 86-second timeout; Blizzard UI loaded and EditMode cache/workarounds remain. Not general cache-free/native startup. Separate startup report retains compile-manifest deprecations; no warning-free build claim here.

## Reuse boundary

Exact `git -C <repo> diff 69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c bde7ea277e86f9ad17f7135b3cd676f80e73057b --` observed once, exit 0: **only `tests/widget_bullet/fixtures.rs`, `env.eval::<()>(code)` → `env.exec(code)`**. Production/config/manifests and selected preservation test files unchanged. Reusing source69 preservation/original/startup evidence is therefore bounded to those unchanged surfaces; source69 binaries are still source69, not current-revision binaries.

Four integration receipts reused by stat only: existing seal/receipt/revision source/profile/path fields and current receipt stat fields match all corresponding selected test result/argv records. No binary, library or archive contents read, no rehash, no new build/source-tree authentication. Recorded feature/profile/cache association and launch constraints are metadata, not freshly authenticated provenance. Startup uses its own separate gate/receipt evidence.

## Actual evidence paths

### Retail

Corrected Bullet: `<epochs>/bullet-fixture-retail-green-retry1-20261014T005621584814Z/runtime-bullet-corrected-green-2g`

Preservation: `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z` with all six `runtime-preserve-*` directories listed individually below and in proof.

- `<epochs>/bullet-fixture-retail-green-retry1-20261014T005621584814Z/runtime-bullet-corrected-green-2g` — 7
- `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-preserve-mount-relations-2g` — 8
- `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-preserve-playerchoice-boolean-2g` — 1
- `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-preserve-playerchoice-mount-live-2g` — 1
- `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-preserve-widget-lifecycle-2g` — 2
- `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-preserve-widget-security-2g` — 1
- `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-preserve-widget-state-2g` — 6
- `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-bullet-initial-api-green-2g-retry1` — 2
- `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-dress-producer-green-2g-retry1` — 5
- `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-playerchoice-dress-green-2g-retry1` — 1
- `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/runtime-normal-bullet-dress-defaults` — separate startup
### Ptr

Corrected Bullet: `<epochs>/bullet-fixture-ptr-green-20261014T005940095211Z/runtime-bullet-corrected-green-2g`

Preservation: `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z` with all six `runtime-preserve-*` directories listed individually below and in proof.

- `<epochs>/bullet-fixture-ptr-green-20261014T005940095211Z/runtime-bullet-corrected-green-2g` — 7
- `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-preserve-mount-relations-2g` — 8
- `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-preserve-playerchoice-boolean-2g` — 1
- `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-preserve-playerchoice-mount-live-2g` — 1
- `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-preserve-widget-lifecycle-2g` — 2
- `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-preserve-widget-security-2g` — 1
- `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-preserve-widget-state-2g` — 6
- `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-bullet-initial-api-green-2g` — 2
- `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-dress-producer-green-2g` — 5
- `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-playerchoice-dress-green-2g` — 1
- `<epochs>/bullet-dress-ptr-green-20261014T004551190529Z/runtime-normal-bullet-dress-defaults` — separate startup

### Existing separate gates (read, not assumed PASS)

- `<verification>/bullet-dress-retail-producer-runtime-current/report-20261014T005450827302Z.md`; `<verification>/bullet-dress-retail-producer-runtime-current/proof-20261014T005450827302Z.json`
- `<verification>/bullet-dress-ptr-producer-runtime-current/report.md`; `<verification>/bullet-dress-ptr-producer-runtime-current/proof.json`
- `<verification>/bullet-dress-normal-startup-current/report.md`; `<verification>/bullet-dress-normal-startup-current/proof.json`

## Unproved / pending

| Requirement | Status |
|---|---|
| Native WoW parity / rendering | UNPROVED |
| Earlier profiles / full suite | UNPROVED |
| Full parent acceptance | UNPROVED, not promoted |
| Final check / fmt | PENDING OTHER GATE; not run here |

## Exact unique-name ledger

Each name appears once per profile; paths and revisions per entry are in proof.

| Test name | Source | Retail | PTR |
|---|---|---|---|
| `item_dressability::item_dressability_empty_input_returns_exactly_one_boolean_false` | 69f2569cb | PASS | PASS |
| `item_dressability::item_dressability_is_independent_of_mount_equipment_and_consumable_inputs` | 69f2569cb | PASS | PASS |
| `item_dressability::item_dressability_projects_explicit_numeric_and_link_booleans` | 69f2569cb | PASS | PASS |
| `item_dressability::item_dressability_replacement_removal_and_environment_isolation` | 69f2569cb | PASS | PASS |
| `item_dressability::item_dressability_requires_item_info_and_authenticates_vm_secrets` | 69f2569cb | PASS | PASS |
| `mount_from_item::mount_item_authenticates_secret_input_before_type_or_lookup` | 69f2569cb | PASS | PASS |
| `mount_from_item::mount_item_empty_relation_returns_exactly_one_nil` | 69f2569cb | PASS | PASS |
| `mount_from_item::mount_item_requires_one_numeric_identity` | 69f2569cb | PASS | PASS |
| `mount_from_item::supplied_mount_item_mapping_does_not_leak_into_independent_fresh_environment` | 69f2569cb | PASS | PASS |
| `mount_from_item::supplied_mount_item_mapping_needs_no_catalog_target_or_ownership` | 69f2569cb | PASS | PASS |
| `mount_from_item::supplied_mount_item_mapping_replacement_and_removal_are_live` | 69f2569cb | PASS | PASS |
| `mount_from_item::supplied_mount_item_numeric_identities_are_distinct_without_truncation_aliases` | 69f2569cb | PASS | PASS |
| `mount_from_item::supplied_mount_item_secret_identity_hits_and_recovers_across_callers` | 69f2569cb | PASS | PASS |
| `mount_item_player_choice::item_dressability_original_player_choice_tracks_supplied_boolean_and_mount` | 69f2569cb | PASS | PASS |
| `mount_item_player_choice::mount_item_player_choice_original_consumer_tracks_live_supplied_relation` | 69f2569cb | PASS | PASS |
| `mount_item_player_choice::mount_item_player_choice_real_item_consumer_returns_boolean` | 69f2569cb | PASS | PASS |
| `ui_widget_bullet_text_list_api_red::bullet_text_list_empty_default_unknown_numeric_id_returns_one_nil_simulator_policy` | 69f2569cb | PASS | PASS |
| `ui_widget_bullet_text_list_api_red::bullet_text_list_public_api_rejects_non_numeric_widget_id_with_context` | 69f2569cb | PASS | PASS |
| `ui_widget_manager_bullet_text_list::consumer::bullet_real_cached_consumer_callbacks_order_lines_replace_hide_restore_remove_unregister` | bde7ea277 | PASS | PASS |
| `ui_widget_manager_bullet_text_list::feed::bullet_feed_commits_complete_record_and_membership_before_synchronous_event` | bde7ea277 | PASS | PASS |
| `ui_widget_manager_bullet_text_list::feed::bullet_feed_rejects_missing_set_and_type_mismatch_without_state_or_events` | bde7ea277 | PASS | PASS |
| `ui_widget_manager_bullet_text_list::queries::bullet_numeric_keys_do_not_truncate_and_signed_zeroes_share_a_key` | bde7ea277 | PASS | PASS |
| `ui_widget_manager_bullet_text_list::queries::bullet_replacement_empty_lines_deletion_and_environment_isolation` | bde7ea277 | PASS | PASS |
| `ui_widget_manager_bullet_text_list::queries::bullet_supplied_fifteen_fields_and_fresh_ordered_lines` | bde7ea277 | PASS | PASS |
| `ui_widget_manager_bullet_text_list::security::bullet_secrets_allowed_untainted_denied_tainted_before_present_absent_lookup` | bde7ea277 | PASS | PASS |
| `ui_widget_manager_lifecycle::cached_ui_widget_container_filters_supplied_unit_updates` | 69f2569cb | PASS | PASS |
| `ui_widget_manager_lifecycle::cached_ui_widget_container_refreshes_removes_and_unregisters_supplied_widgets` | 69f2569cb | PASS | PASS |
| `ui_widget_manager_security::widget_queries_deny_tainted_secrets_and_recover_public_isolated_snapshots` | 69f2569cb | PASS | PASS |
| `ui_widget_manager_state::absent_high_ids_do_not_invent_widget_sets_or_icons` | 69f2569cb | PASS | PASS |
| `ui_widget_manager_state::host_widget_feed_publishes_state_before_events_and_rejects_invalid_updates` | 69f2569cb | PASS | PASS |
| `ui_widget_manager_state::icon_and_text_returns_complete_fresh_isolated_supplied_snapshots` | 69f2569cb | PASS | PASS |
| `ui_widget_manager_state::quest_widget_sets_use_exact_supplied_quest_and_type_mapping` | 69f2569cb | PASS | PASS |
| `ui_widget_manager_state::widget_set_info_returns_exact_supplied_fields_for_nonthreshold_id` | 69f2569cb | PASS | PASS |
| `ui_widget_manager_state::widgets_by_set_preserve_supplied_order_and_documented_entry_fields` | 69f2569cb | PASS | PASS |

Output: new exclusive-create mode 0600 report/proof; file fsync, directory fsync and byte readback. Existing evidence untouched.
