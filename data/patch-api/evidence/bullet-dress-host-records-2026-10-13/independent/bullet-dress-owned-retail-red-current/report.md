# Owned-input Retail RED classification

Revision reported by launch results: `76e8b830931d1489ad5a89a75306bcbaa2bc61f7`.
Epoch: `<epochs>/bullet-dress-owned-retail-red-20261013T234617677640Z`.

## Verdict
FAIL: 0/13 unique tests passed. Bullet 0/7 (six feature RED, one fixture-assumption failure); dressability 0/5 feature RED; whole cached consumer 0/1 feature RED. Bullet child-reexec 0/1 is already included in aggregate 0/7, not an eighth test.

## Executed boundaries / resources
Original 4 GiB Bullet admission denied (`allowed=false`); directory contains admission.json only. Zero launch; preserved unchanged, not behavioral RED.
All three actual runs admitted at MemoryMax=2G / MemoryHigh=1536M / CPUQuota=200%, with outer timeout 86s. Each exited 101 with launcher result exit-code, not timeout or reported OOM-kill. Launcher memory peaks are rounded display values, not exact byte measurements. No OOM counter authentication undertaken.
- `runtime-bullet-owned-red-2g-retry1`: peak 1.5G (swap: 0B); runtime 8.786s; exit 101; admission 2026-10-14T00:00:36.477396+00:00.
- `runtime-dress-owned-red-2g`: peak 112.8M (swap: 0B); runtime 1.654s; exit 101; admission 2026-10-14T00:00:57.089453+00:00.
- `runtime-player-choice-dress-red-2g`: peak 1.5G (swap: 0B); runtime 9.264s; exit 101; admission 2026-10-14T00:01:43.053673+00:00.

Admissions crossed UTC midnight: actual retries occurred October 14, 2026 UTC; epoch name retains October 13. Full admission/argv/result/launcher/stdout/stderr read for every actual run. Absolute directories and exact byte counts are in proof.json.

## Exact first failures and unreached assertions
| Test (module prefix abbreviated) | Classification / first failure | Not reached |
|---|---|---|
| `bullet::consumer::bullet_real_cached_consumer_callbacks_order_lines_replace_hide_restore_remove_unregister` | feature-red; `tests/widget_bullet/fixtures.rs:79`: missing supplied BulletTextList record in real cached-consumer callback | Pooled acquisition, ordered visible lines, colors/anchors/heights, replacement, hide/restore, removal and unregister unreached. |
| `bullet::feed::bullet_feed_commits_complete_record_and_membership_before_synchronous_event` | feature-red; `tests/widget_bullet/fixtures.rs:79`: missing supplied BulletTextList record in real UPDATE_UI_WIDGET callback | Complete snapshot/membership and replacement callback assertions unreached; first synchronous callback count passed. |
| `bullet::feed::bullet_feed_rejects_missing_set_and_type_mismatch_without_state_or_events` | fixture-assumption-failure-not-getter-red; `tests/widget_bullet/feed.rs:111`: assertion failed: env.state().borrow().ui_widget_inputs.sets.is_empty() | Missing-set error text and empty bullet records passed; notification assertion, type mismatch and absent removal unreached. Complete baseline-set unchanged comparison is pending, not proved. |
| `bullet::queries::bullet_numeric_keys_do_not_truncate_and_signed_zeroes_share_a_key` | feature-red; `tests/widget_bullet/fixtures.rs:79`: missing supplied BulletTextList record | Numeric key cases and signed-zero assertions unreached. |
| `bullet::queries::bullet_replacement_empty_lines_deletion_and_environment_isolation` | feature-red; `tests/widget_bullet/fixtures.rs:79`: missing supplied BulletTextList record | Replacement/empty lines/deletion and second environment unreached. |
| `bullet::queries::bullet_supplied_fifteen_fields_and_fresh_ordered_lines` | feature-red; `tests/widget_bullet/fixtures.rs:79`: missing supplied BulletTextList record | Fifteen-field snapshot, fresh lines and mutation isolation unreached. |
| `bullet::security::bullet_secrets_allowed_untainted_denied_tainted_before_present_absent_lookup` | feature-red; `tests/widget_bullet/fixtures.rs:79`: missing supplied BulletTextList record | Secret ID installation and all secret/taint/public snapshot assertions unreached. |
| `dress::empty_input_returns_exactly_one_boolean_false` | feature-red; `tests/item_dressability.rs:28`: empty supplied classification must return boolean false | First query(6948) passed one-value arity but failed boolean-false; later numeric/link misses unreached. |
| `dress::is_independent_of_mount_equipment_and_consumable_inputs` | feature-red; `tests/item_dressability.rs:221`: supplied result must be boolean | First query(6948) passed one-value arity; link queries, explicit true/false and independent relation assertions unreached. |
| `dress::projects_explicit_numeric_and_link_booleans` | feature-red; `tests/item_dressability.rs:221`: supplied result must be boolean | First supplied false numeric query failed type after arity; bare/generated links, uncatalogued queries and true/false transitions unreached. |
| `dress::replacement_removal_and_environment_isolation` | feature-red; `tests/item_dressability.rs:221`: supplied result must be boolean | Two environments created; initial supplied true numeric query failed type after arity. Isolated query/replacement/removal assertions unreached. |
| `dress::requires_item_info_and_authenticates_vm_secrets` | feature-red; `tests/item_dressability.rs:44`: required ItemInfo must reject input | First argumentError() failed rejection contract; nil/other malformed arguments, wrappers, untainted projection, tainted denial ordering and restoration unreached; second expected=true iteration unreached. |
| `whole::supplied_boolean_and_mount` | feature-red; `tests/mount_item_player_choice.rs:235`: assertion left == right failed: left false, right true | Initial supplied false consumer result passed; supplied true returned false. Third false, mount insertion/removal, independent predicate and dressability removal assertions unreached. No security assertions in this case. |

## Fixture assumption, not getter RED
`tests/widget_bullet/feed.rs:111` assumes all widget sets start empty. Git text at this revision shows `src/lua_api/state.rs:314` initializes ui_widget_inputs through `seed_demo_world_quest_widgets`; `src/c_api/ui_widget_manager/fixtures.rs:5-21` inserts demo sets from nonempty WORLD_QUESTS (`src/lua_api/globals/quest_surface/data.rs:80-95`, quests 90101/90102). Existing sets therefore invalidate sets.is_empty independently of Bullet projection. Missing-set rejection and empty Bullet records passed before this failure, but unchanged complete baseline sets and zero notifications were not proved. Main plans complete baseline sets unchanged comparison after snapshot sealing; no weakening, correction execution or producer implementation accepted by this report.

## Proof separation
Observed API init succeeded sufficiently to reach first contract failures. Cached Bullet roots, empty container registration and real callback were reached, but pooled line/lifecycle assertions were not. Whole cached PlayerChoice closure loaded and original SetItem/IsDressupReward calls reached two iterations. Initial supplied false returned false; supplied true still returned false. Both logged direct predicate value nil (function, nil, nil), with predicate source Lua =(string):72-74. Vendor method provenance/identity assertions passed before that contrast failure. Whole closure warning/Lua-error records were empty at observed boundaries, while nil-symbol/missing-requirement diagnostics were present: setup reachability is not complete surface acceptance.
Selected Git test/helper and existing seed text only. No build/runtime reruns, no executable/archive/library/cache reads or authentication, no snapshot source-tree authentication and no old accepted payload reads. Result revision is metadata, not independently authenticated binary binding. No native parity, parent acceptance, producer acceptance or completed implementation claim.

Outputs: `<verification>/bullet-dress-owned-retail-red-current/report.md` and `<verification>/bullet-dress-owned-retail-red-current/proof.json`; private 0600, file fsync and immediate full readback.
