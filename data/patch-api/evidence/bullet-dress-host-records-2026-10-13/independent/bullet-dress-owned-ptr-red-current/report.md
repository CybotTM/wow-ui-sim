# PTR owned-input RED classification

## Scope and association
Historical executed runs only, associated by their result.json source field with `733399c38f48158206f8b3144cb139a52c537d9f` (PTR). No source-byte identity, artifact integrity, library identity, or parent acceptance established. Epoch: `<epochs>/bullet-dress-owned-ptr-red-20261013T235725549105Z`.

Read all six files (admission.json, argv.json, launcher.txt, result.json, stdout, stderr) in each of the three named runtime directories. PlayerChoice stderr read through its full 65,802 bytes in bounded contiguous chunks. No new builds, runtime execution, authentication, hashing, source archive, library, cache, or old accepted payload reads. Relevant tests/helpers inspected only through Git-selected source 733399c38; existing seed source inspected for the fixture failure. No producer/native/render claims.

User reports main corrected fixture oracle at `402ab5a88` after snapshot sealing. That source was not used by these historical runs; correction is not verified here and does not reclassify their raw results. Epoch label is October 13; recorded admission timestamps are October 14, 2026 UTC.

## Launch results
| Run directory | Aggregate | Launcher / service exit | Runtime | CPU | Printed memory peak / swap |
|---|---|---|---|---|---|
| runtime-bullet-owned-red-2g | 0 passed / 7 failed; 10,188 filtered | 101 / exited 101 | 10.174s | 6.139s | 1.5G / 0B |
| runtime-dress-owned-red-2g | 0 passed / 5 failed; 10,190 filtered | 101 / exited 101 | 1.893s | 1.550s | 101.1M / 0B |
| runtime-player-choice-dress-red-2g | 0 passed / 1 failed; 10,194 filtered | 101 / exited 101 | 7.829s | 4.809s | 1.1G / 0B |

Bullet nested timeout child reports 0 passed / 1 failed; 10,194 filtered. It is the SAME real-consumer case included in the seven-test aggregate, not an eighth case. All three admissions allowed execution, worker max 2,147,483,648 bytes / high 1,610,612,736 bytes. Launch argv sets CPUQuota=200%, outer timeout 86 seconds, network-unshared bwrap, WOW_SIM_CASC=0, WOW_SIM_NO_SOUND=1, one test thread. Unit names contain “green” but observed outcomes are RED. No launcher denial, setup abort, timeout exit, signal termination, or OOM termination reported. Memory peaks are printed rounded values, not exact bytes. Admission pressure and limits are distinct from the assertion failures; no resource-causality conclusion follows.

## Bullet: raw 0/7; six feature failures, one fixture failure
Names below are under `ui_widget_manager_bullet_text_list::`.

| Case | Exact first failure / source-associated boundary | Proven reach; unreached scope |
|---|---|---|
| consumer::bullet_real_cached_consumer_callbacks_order_lines_replace_hide_restore_remove_unregister | fixtures.rs:79:33: `first positive-data callback RED before pooled consumer assertions`: `missing supplied BulletTextList record in real cached-consumer callback`; parent timeout_reexec.rs:184:9 wraps child exit 101 | Cached closure and empty-container registration completed; initial hidden supplied-record event callback reached its table assertion. Callback count gate passed before captured callback error rethrow. Complete snapshot/membership and pooled hidden suppression, shown line acquisition/order/color/height, replacement, hide/restore, removal, unregister and post-unregister behavior NOT proven. Child is not a separate test. |
| feed::bullet_feed_commits_complete_record_and_membership_before_synchronous_event | fixtures.rs:79:33: `first real-callback positive-data RED`: `missing supplied BulletTextList record in real UPDATE_UI_WIDGET callback` | Typed update returned; synchronous event count assertion passed; callback event/id/type assertions preceded failing table assertion. Complete record projection/membership assertion and replacement snapshot/event ordering NOT reached. |
| feed::bullet_feed_rejects_missing_set_and_type_mismatch_without_state_or_events | feed.rs:111:5: `assertion failed: env.state().borrow().ui_widget_inputs.sets.is_empty()` | Missing-set error and empty Bullet record map checks passed. Global set-emptiness assumption conflicts with existing demo seeded sets. FIXTURE failure, NOT getter RED. No-notification assertion, supplied-set/type-mismatch rejection, unchanged membership and absent removal checks NOT reached. No before/after set equality proof. |
| queries::bullet_numeric_keys_do_not_truncate_and_signed_zeroes_share_a_key | fixtures.rs:79:33: `first genuine positive-data getter RED`: `missing supplied BulletTextList record` | Initial ordinary selector 760401 reached table assertion. Fractional, negative, >u32, signed-zero and missing selector checks NOT reached. |
| queries::bullet_replacement_empty_lines_deletion_and_environment_isolation | same fixtures.rs:79:33 getter failure | Initial record supplied; second environment creation, empty lines, replacement/deletion and retained-snapshot isolation NOT reached. |
| queries::bullet_supplied_fifteen_fields_and_fresh_ordered_lines | same fixtures.rs:79:33 getter failure | Getter function assertion passed; initial row table assertion failed. Fifteen-field snapshot, ordered lines, fresh row/lines, caller mutation isolation and unchanged host input assertions NOT reached. |
| security::bullet_secrets_allowed_untainted_denied_tainted_before_present_absent_lookup | same fixtures.rs:79:33 getter failure | Ordinary positive selector fails before installing authentic secret IDs. Secret construction, AllowedWhenUntainted projection, tainted present/absent denial order, public output secrecy, wrapper preservation and stack restoration NOT reached. This is positive-data RED, not security-denial proof. |

Failure text establishes a failed table assertion, not the exact unprinted getter return. Seed evidence at selected source: src/lua_api/state.rs:314 initializes ui_widget_inputs with seed_demo_world_quest_widgets; src/c_api/ui_widget_manager/fixtures.rs:5-19 inserts demo world-quest sets. No native quest association inferred.

## Direct dress: raw 0/5
All cases under `item_dressability::`; references in tests/item_dressability.rs.

| Case | Exact first failure | Reached / unreached |
|---|---|---|
| item_dressability_empty_input_returns_exactly_one_boolean_false | :28:6: `empty supplied classification must return boolean false` | Function and generated-link checks passed; first numeric 6948 result passed exact one-value arity, failed combined boolean/false assertion. Actual value/type NOT printed. Numeric 901001, bare item link and generated-link calls NOT reached. |
| item_dressability_is_independent_of_mount_equipment_and_consumable_inputs | :221:10: `supplied result must be boolean` | Independent equipment/consumable/mount inputs inserted; first numeric 6948 query passed exact arity, failed type check. Actual value/type NOT printed. Bare/generated links, boolean comparison, supplied dressability transitions, independent-API assertions and removals NOT reached. |
| item_dressability_projects_explicit_numeric_and_link_booleans | :221:10: `supplied result must be boolean` | Initial state-empty and uncatalogued-item checks passed; false classifications inserted for 6948/901001. First numeric 6948 type assertion failed after exact arity. Other links, uncatalogued queries and true/false replacement transitions NOT reached. |
| item_dressability_replacement_removal_and_environment_isolation | :221:10: `supplied result must be boolean` | Two environments constructed, true inserted in supplied environment; first numeric 6948 type assertion failed after exact arity. Isolated query, false/true replacements, removal/default and final empty-state checks NOT reached. |
| item_dressability_requires_item_info_and_authenticates_vm_secrets | :44:14: `required ItemInfo must reject input` | First expected=false iteration inserted classification; taint-mode/untainted/function setup reached. First argumentError() (NO ARGUMENTS) fails combined error/string assertion. pcall status/message NOT printed. nil/false/table/function arguments, generated link, authentic secret setup, untainted secret acceptance, malformed-secret validation, tainted denial precedence, ordinary tainted projection, wrapper/taint restoration and expected=true iteration NOT reached. Not a proven security denial. |

Source ordering distinguishes first numeric selector from later supported links. Direct failures do not print nil; do not borrow the consumer's printed nil as their actual return data.

## Real PlayerChoice dress consumer: raw 0/1
`mount_item_player_choice::item_dressability_original_player_choice_tracks_supplied_boolean_and_mount` fails tests/mount_item_player_choice.rs:235:9: equality `left: false`, `right: true`.

Whole cached closure load reached original XML reward/ItemButton construction and unchanged vendor method identity/provenance assertions. Recorded per-addon Lua errors/records/counts and warnings are empty. Nil-symbol and missing-requirement diagnostics are separately printed; they did not abort setup and are not silently converted into feature failures. Original SetItem stores generated Hearthstone item link and item identity. Two original IsDressupReward calls explicitly print boolean false. Two direct generated-link predicate observations explicitly print function / nil / nil; predicate provenance prints Lua, =(string), lines 72-74. These are observed runtime provenance only, not source-byte verification.

Selected test ordering removes mount relation then supplies false, true, false. First false consumer comparison passes. Second, supplied true, returns false and fails. Third false transition NOT reached. Subsequent mount insertion/independent predicate checks/mount-dependent true consumer, mount removal/default false and dressability removal/default false NOT reached. No mount branch, complete state lifecycle, producer, native classification, render or parent acceptance established.

## Verdict
Historical RED classification only: 13 unique cases, 0 passed, 13 failed; Bullet splits six feature failures plus one fixture-oracle failure. All failures are reached assertions, not denied launches/resource exits. Later selectors and state/security/lifecycle contracts remain unproven at these first-failure boundaries. No GREEN or corrected-fixture evidence supplied by these runs.
