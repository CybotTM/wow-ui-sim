# Saved PTR runtime audit — 2026-10-13

## Verdict and scope

**Current bounded runtime PASS:** 28/28 selected tests; eight completed integration invocations with actual `systemd-run --wait` exits 0. Separate normal CLI startup exits 0, reaches `SPELL_DISPLAY_NORMAL_PROBE_OK`, stdout ends `[]`, stderr reports `Status: CLEAN`, 0 unique / 0 occurrences / 0 unattributed Lua errors. No pending scopes.

**Historical classification remains FAIL for the nine-test suite:** 38ec has 6 passed / 3 failed, actual wait exit 101. Its separate normal startup PASS (wait 0 / CLEAN) does not erase those three consumer failures.

Read-only saved-artifact audit following verify/SKILL.md as verifier. No runtime, build, hashing, delegation, tracked edits, or old-binary authentication performed. Only requested report/proof directory written. Captured binary identity/source association is the supplied epoch metadata, not independently authenticated binary contents. External libraries/cache/settings explicitly unauthenticated by source-scope.json.

Current epoch: `widget-discrete-progress-current/spell-display-ptr-removal-green-20261013T194351858902Z`, source `844a63adb95c7111e1286fa668e632f7cd237806`, production delta recorded as `621649e56`. Historical epoch: `spell-display-ptr-green-20261013T185451536429Z`, source `38ecbd8ee5dfae4315ca98001b7e794e324b1066`. Source844 includes documentation over the 621/38ec behavior changes; this report does not claim documentation changes independently created runtime proof.

## Exact completed scope matrix

| Current capture | Tests passed/failed | Actual wait exit | stdout / stderr bytes | stderr classification |
|---|---:|---:|---:|---|
| runtime-spell-display-post-removal-retry1 | 9 / 0 | 0 | 1210 / 6411 | 110 informational startup lines |
| runtime-removal-policy-retry1 | 1 / 0 | 0 | 228 / 678 | 11 startup lines + policy completion marker |
| runtime-registration-aspect-retry1 | 5 / 0 | 0 | 672 / 3202 | 55 startup lines |
| runtime-unit-filter-retry1 | 2 / 0 | 0 | 318 / 1371 | 22 startup lines + two modeled instant-spell diagnostics |
| runtime-callback-lifecycle-retry1 | 1 / 0 | 0 | 201 / 640 | 11 startup lines |
| runtime-text-state-preservation | 7 / 0 | 0 | 942 / 5115 | 88 startup lines |
| runtime-speak-input-preservation | 2 / 0 | 0 | 328 / 1921 | 33 startup lines |
| runtime-speak-consumer-preservation | 1 / 0 | 0 | 224 / 6946 | 102 startup/workaround/EditMode timing lines |
| runtime-normal-startup-post-removal | separate probe, no Rust test count | 0 | 33 / 14690 | 211 informational lines, three load breakdown lines, five final CLEAN-summary lines |

Byte counts above refer to complete saved streams, not excerpts. `runtime-proof.json` is authoritative for all counts; each count was checked against actual saved bytes and launcher exit against `launcher.txt`'s terminated process status. No timeout/signal exit or hidden nonzero wait observed.

First five non-retry captures (`runtime-spell-display-post-removal`, `runtime-removal-policy`, `runtime-registration-aspect`, `runtime-unit-filter`, `runtime-callback-lifecycle`) each contain admission.json only, `allowed:false`, no argv/result/stdout/stderr/launcher. **Denied, zero launch**, neither failed tests nor pending runs. Each later retry1 completed. No waiting/polling or new launch performed by this audit.

## Assertions reached — current tests

All named tests are listed verbatim in `test-results.json`. Full source is retained in `source-proof/tests/`; locations below are relative to that root. Passing test results plus inspected straight-line test bodies establish that their required assertions and helper error checks completed, not only that an initial setup returned. This is bounded test-source reasoning, not assertion-level tracing/instrumentation.

### SpellDisplay — 9 tests (3 real cached consumers + 6 query/feed/security tests)

`widget_spell_display/fixtures.rs`: each seeded query boundary checks getter function, supplied table, exactly **14 outer fields / 16 nested spellInfo fields**, every expected field type/value, no unexpected keys. Baseline concrete record uses spell 35395, supplied text/tooltip, stack 3, width 180, order 12.5. `widget_discrete/fixtures.rs:41-49` executes Lua then asserts accumulated Lua errors empty after every helper phase.

| Test suffix | Complete reached behavior |
|---|---|
| queries::spell_display_complete_nested_fresh_replace_delete | Fresh outer AND nested tables; caller mutation cannot affect retained/fresh snapshots or Rust input; absent lookup exactly one nil; replacement text/tint/earned/stack/order/width reflected with prior snapshot unchanged; explicit input deletion one nil, retained snapshot unchanged. |
| queries::spell_display_strict_numeric_ids_without_integer_range_inference | Exact negative/fractional/zero/4294967299 keys retained; -0 equals 0; truncated/colliding keys absent; missing/nil/string/bool/table/function rejected with number-contract error. |
| queries::spell_display_same_id_environment_isolation | Second environment initially absent; same ID can hold independent text; first complete nested snapshot remains unchanged. |
| security::spell_display_real_secret_ids_allow_untainted_deny_tainted_recover | Actual VM secret-number IDs; untainted supplied/absent lookup; complete public outer/nested values and keys; tainted supplied/absent IDs rejected with untainted-caller diagnostic and remain secret; tainted unwrap denied; ordinary IDs still readable; retained snapshot accessible; untainted stack restored; fresh outer/nested result mutation cannot affect prior or later reads. |
| feed::spell_display_feed_commits_before_events_retains_visualization_on_membership_removal | Real event observer sees committed membership/type/set/ID and complete record before notification; two updates, no duplicated membership/full event; distinct outer/nested event snapshots retain first value; first removal true, repeated removal false; one full event sees empty membership; visualization/first snapshot retained. |
| feed::spell_display_feed_rejects_type_mismatch_unknown_set_without_mutation_or_notification | Unknown set and actual wrong type14 rejected; original input preserved; zero notifications/members; unknown set remains absent. |
| consumer::spell_display_cached_activation_update_hidden_restore_remove | Cached addon closure includes Blizzard_UIWidgets; scalar enum/catalog prerequisites and real container registration checked; initial empty pool/layout/timers; event activates real frame with catalog ID, text, stack, tooltip, positive geometry, icon36, no earned mark/rarity pips; update preserves frame identity, updates text/tooltip/stack4, earned/red tint, width206.25-36 while retaining original snapshot; Hidden releases frame; restore uses manager pool; membership removal empties frame map and hides restored frame but retains updated API record and original snapshot; each phase completes actual OnUpdate layout/pool count and no-timer checks. |
| consumer::spell_display_attached_player_context_separate_from_world_loot_policy | Real attached player container/frame context, supplied nested text; API record has no attachedUnit field; actual layout/pool count1; no world-loot behavior claim. |
| consumer::spell_display_zero_spell_polling_uses_real_on_update_without_feed_notification | Event constructs zero-spell frame with hadSpellID false; direct host-map replacement without notification leaves it unchanged until actual XML OnUpdate; polling retains same frame, discovers 35395, flips hadSpellID, updates supplied text. |

### Removal, aspect, unit and callback controls — 9 tests

`frame_unregister_unknown.rs` executes the complete 59-line `fixtures/frame_unregister_unknown.lua`: absent typo `PLAYER_EQUIPED_SPELLS_CHANGED` and synthetic unknown removal do not throw, return exactly one false and do not register; known PLAYER_LOGIN registers/delivers; repeated unknown cleanup preserves it; RegisterEvent/RegisterUnitEvent still reject unknown names without altering known registration; subsequent known delivery increments to2; known removal returns exactly one true and stops delivery; repeated known removal exactly one false. Final `FRAME_UNREGISTER_UNKNOWN_POLICY_DONE` actually appears in stderr.

`forbidden_aspect_creation.rs:154-370`, five selected tests:

| Test suffix | Complete reached behavior |
|---|---|
| event_registration_aspect_rejects_addon_callers_and_permits_secure_callers | Eight operations (RegisterEvent, RegisterUnitEvent, RegisterAllEvents, both callback registrations, UnregisterEvent, UnregisterAllEvents, IsEventRegistered) tested on restricted frames for secure and tainted callers; all secure calls succeed, all tainted calls reject with string error. |
| event_registration_aspect_preserves_existing_listener_delivery | Rejected unregister/clear/unit-replacement preserve individual/unit/all listeners, original player filter, query values and counts1/1/3 after real dispatch. |
| event_registration_aspect_preserves_callbacks_when_replacement_is_rejected | Rejected clear/replacement retain original callback owner/unit filter; ordinary/unit counts1/1, replacement0; registrations remain. |
| event_registration_aspect_leaves_zero_mask_mutations_and_delivery_unchanged | Zero-mask individual/unit/all registrations succeed; actual delivery counts1/1/3; both removal forms succeed; later dispatch changes no counts. |
| event_registration_aspect_leaves_zero_mask_callback_delivery_unchanged | Zero-mask ordinary/unit callbacks receive correct owner/player payload once; target filtered; clear unregisters both; later events change no counts. |

`unit_event_second_unit.rs`, two tests: modeled Word of Glory changes player health50000→70000 and delivers only appropriate first/player filters; Crusader Strike changes target50000→35000 and delivers target plus second-unit listener, not player-only; query retains first unit player. Lifecycle rejects focus, replacing two-unit with single clears second, unregister and unregister-all remove both and prevent later delivery.

`global_event_callbacks.rs:17-77,210-217`, selected Lua lifecycle only: loads actual cached SharedXMLBase/Event.lua; Event.RegisterUnitCallback strips only nil owner and preserves exactly three payload values (unit,17,false); independent player/target callbacks counts advance correctly; unregister handles stop only their own delivery. Rust lifecycle sibling was NOT selected.

### TextWithState — 7 tests

`widget_text_state/fixtures.rs` validates exactly21 fields, types/values and no extras. Three query tests establish fresh tables, immutable retained/host snapshots, unknown/deleted exactly-one-nil outputs, replacement of text/tooltip/padding/width/order/animation/kits/timer without inferred native clamping, strict numeric ID keys/type errors, independent same-ID environment inputs. Security test exercises genuine VM secrets: untainted supplied/absent queries, public keys/values, tainted rejection/no declassification, ordinary-ID access, stack recovery and snapshot isolation.

Two feed tests establish committed ID760801/set428/type8 membership and supplied record visible to event observers, immutable two-update snapshots, no duplicate membership, true/false removal and one full event with empty members but retained visualization; unknown set/type19 rejection produces no mutation/events.

Cached consumer completes real type8 construction/event registration and catalog enums, activation, positive measured font geometry/padding clamp, width180, left/white/font/color/tooltip/scale/layout/order checks; incremental same-frame width206.25/right/green/shadow-font/order9.25 update with old snapshot intact; hidden release, restored text/frame from pool, membership removal hides/empties map while API record remains. Real OnUpdate pool/layout/timer checks complete after all phases. No GPU pixel/native claim.

### Speak input and consumer — 3 tests

`voice_speak_for_me_active_input.rs`: default getter exists and returns exactly one false boolean; retained getter reads live independent host input while retained earlier value remains false; transcription/permission/channel/connection state does not override activity; independent environments remain separate through true→false changes.

`voice_speak_for_me_consumer.rs`: real cached ChatFrame3/editBox/prompt and function prerequisites; no accumulated errors at full constructor/startup or any consumer phase; inactive/unmuted hides edit box, disables activation and clears prompt; active/unmuted enables activation and REMOTE_TEXT_TO_SPEECH prompt; active/muted leaves query true but disables activation and uses muted prompt/red RGB; inactive/unmuted restoration rechecks hidden/disabled/empty. Direct cached function invocation, not event/transport/native/render proof.

## Separate normal startup — current and historical

Both argv captures invoke saved normal `wow-sim`, `--no-addons --no-saved-vars --exec-lua ... lua-errors`; `WOW_SIM_CASC=0`, `WOW_SIM_NO_SOUND=1`, isolated network, source read-only mounted. Both load283 Blizzard addons, record1568/1568 bytecode cache hits with zero replay/store failures, apply the existing workaround/bootstrap paths, restart GC, then finish CLEAN. Sound/third-party/SavedVariables disabling and CASC-disabled font fallback are configuration/informational diagnostics, not runtime failures. Both still load EditMode layout from WTF; `--no-saved-vars` must not be interpreted as no cached configuration.

Exact reached probe: SpellDisplay getter is a function; unseeded761999 returns exactly one nil; TextWithState unseeded761999 exactly one nil; Speak query has boolean type and is false; completion marker printed. This does NOT assert a seeded SpellDisplay consumer during normal startup, whole-parent acceptance, native parity, or native defaults.

Historical normal: stdout33/stderr14698 bytes, wait0, service7.887s, five CLEAN summary lines. Current normal: stdout33/stderr14690 bytes, wait0, service8.574s, five CLEAN summary lines. Full streams independently classified; neither normal capture contains Lua-error emissions or test panics.

## Independent historical 38ec failure analysis

Historical nine-test stdout1598/stderr17766 bytes; 6 pass/3 fail; wait101, service8.958s. Six producer/query/feed/security tests reach all their assertions above. All three cached consumer tests reach seeded complete getter boundary, cache/template prerequisites, real empty container registration and initial flush. They then fail after real event notification:

| Historical consumer | Actual boundary / proof limits |
|---|---|
| attached_player_context | `tests/widget_discrete/fixtures.rs:44:5`, `discrete [container attachedUnit propagation only] Lua errors:`. Its Lua assertion block completed, then accumulated-error check failed. Final flush(count1) not reached. |
| cached_activation_update_hidden_restore_remove | `fixtures.rs:42:33`, `discrete [actual event activation]: Lua error: assertion failed!`. Exact unlabeled assertion inside this Lua block is not identified by saved traceback. All subsequent incremental update/Hidden/restore/removal and final pool assertions NOT proved historically. |
| zero_spell_polling | `fixtures.rs:42:33`, `discrete [zero-spell consumer before poll]: Lua error: assertion failed!`. Later direct replacement/no-event/polling assertions NOT reached historically. |

Full historical stderr contains **six emitted Lua-error diagnostics (two per failing consumer, original plus ScriptErrors-wrapped), three test panics, three assertion/accumulated-error records,108 traceback lines,110 informational startup lines, six blanks and one backtrace hint**. These are not six independently distinct causes. All emitted error headers name `PLAYER_EQUIPED_SPELLS_CHANGED`; real trace goes through `[C]: UnregisterEvent`, cached Base.lua:697 UpdateTypeIcon, Base.lua:596/514, SpellDisplay.lua:35, manager.lua:526/52. The error text says RegisterEvent but operation on traceback is removal.

Historical `events.rs` removal uses `remove_registered_event_checked` → `mutate_registered_event_checked` → `ensure_registerable_event`, applying registration validation to removal. Current epoch `events.rs:81-99` preserves forbidden-aspect check but removes an event from actual registered state without that catalog validation. Runtime control proves strict registration remains; historical/curr source comparison explains the failure path without vendor mutation or native semantics inference.

## Bounded unchanged-source proof and exclusions

Direct byte comparisons, **no hashes**: all22 selected current epoch test/helper files equal canonical current files;20/22 equal historical files. Two removal-policy files do not match historical because they are new/absent there. All five SpellDisplay files, its test root, shared assert helper, all TextState files/root, aspect/unit/callback/Speak tests and retained local harnesses are unchanged across these epochs. This establishes unchanged selected assertions, not whole-source equality or binary identity. `bounded-source-comparison.json` records exact paths/results; full copies preserved.

No native-client test evidence, native unknown-event removal parity, native event alias/default/range admissibility, audio/transport, screenshot/GPU/rendering, full-parent acceptance, or full suite covered. Historical38ec remains a partial failure; current selected PTR tests and normal startup independently pass. No pending capture or further runtime scope needed for this assigned saved audit.

## Proof files

- `runtime-proof.json`: all16 scopes, admissions, actual wait exits, full stream byte checks and diagnostic category counts.
- `test-results.json`: exact28 current and9 historical test names/statuses.
- `bounded-source-comparison.json`: selected22 files, direct byte comparisons.
- `proof/{current,historical}/runtime-*/`: verbatim full stdout/stderr/launcher/argv/result/admission, plus per-line stderr classification; epoch revision/source-scope metadata beside captures.
- `source-proof/`: selected current test/helper source and current/historical event implementation for bounded inspection.
