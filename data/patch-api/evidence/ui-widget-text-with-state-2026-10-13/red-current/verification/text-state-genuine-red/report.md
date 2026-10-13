# TextWithState saved-execution verification

Date: 2026-10-13. **Genuine supplied-record RED established; producer GREEN and parent acceptance remain OPEN.**

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/text-state-retail-red-speak-preservation-20261013T163822997097Z`; execution: `runtime-text-state-red/`. Declared source: `64a2a57b27c5c2b021375cbfddda557ec145507c`. Existing integration receipt authentication is reused (caller reference 3814); no source/archive/binary byte authentication performed here. Revision attribution is metadata, not independently authenticated source provenance.

## Execution evidence

Complete stdout, stderr, launcher, argv, result and admission inspected. Their verbatim contents, plus compile stderr and saved test/helper source, are retained in `proof.json`.

- Rust harness: **FAILED: 1 passed; 6 failed; 0 ignored; 0 measured; 10977 filtered out; finished in 0.72s**. Seven cases counted once, not duplicated from panic or failure listings.
- `systemd-run --wait`: exit **101**; launcher says `Finished with result: exit-code`, main process `code=exited, status=101/n/a`, runtime **753ms**, CPU **732ms**, peak **74M**, swap **0B**. This is test failure, not timeout/OOM/setup failure.
- Unit `wow-speak-retail-green-runtime-20261013t164458020996z.service` contains `green`, but that label is not evidence of GREEN. Actual counts and terminal status above establish RED.
- Saved argv selects copied integration executable, filter `ui_widget_manager_text_state::`, `--nocapture --test-threads=1`, timeout 86s, read-only source overlay, network isolation, CASC disabled and sound disabled. Saved admission says `allowed: true`, worker max 4GiB/high 3GiB, profile Retail. No launcher, build or replay executed by verifier.

## Reached / unreached matrix

| Case | Actual evidence | Unreached acceptance |
|---|---|---|
| `consumer::text_state_cached_type8_activation_update_hidden_restore_membership_removal` | FAIL at shared `tests/widget_discrete/fixtures.rs:42:33`: `missing supplied TextWithState record` | Cached addon loading, enum checks, constructor, pool/layout/geometry, activation/update/Hidden/restore/removal |
| `queries::text_state_complete21_fresh_snapshots_unknown_nil_replace_remove` | Same supplied-record FAIL | Complete21, freshness, unknown nil, replacement/deletion, negative-value preservation |
| `queries::text_state_numeric_ids_and_strict_widget_id_arguments` | Same supplied-record FAIL | Numeric identity and strict argument rejection |
| `queries::text_state_same_id_inputs_are_environment_local` | Same supplied-record FAIL in first environment | Second environment construction and isolation checks |
| `security::text_state_real_secret_ids_allow_untainted_deny_tainted_recover` | Same supplied-record FAIL | Secret wrapper/helper setup, untainted/tainted policy, public recovery |
| `feed::text_state_feed_commits_before_events_retains_visualization_on_membership_removal` | Callback error `missing supplied TextWithState record in update event`; then panic at shared helper line42: `initial event must observe supplied record` | Replacement/event snapshot freshness, membership removal and independent visualization retention |
| `feed::text_state_feed_rejects_type_mismatch_unknown_set_without_mutation_or_notification` | PASS | This is rejected-feed preservation, **not** producer GREEN |

Five direct tests seed the environment-owned `text_with_state` map before `assert_initial_snapshot`. Saved `tests/widget_text_state/fixtures.rs:58-61` first asserts getter publication (`type(get) == 'function'`), invokes it, then asserts a table. Actual missing-record message therefore establishes publication assertion passed and call returned a non-table result; **not** a missing-function/setup failure. Table-type failure alone does not prove literal nil, return arity, or exact backing stub behavior; no such claim made. The stale phase label `(unexecuted RED candidate)` is embedded source wording, not the execution status.

Feed observer reaches payload/membership assertions before getter/table check (`feed.rs:34-38`). Callback cannot append a snapshot after its assertion fails. Immediate length assertion (`feed.rs:60`) prevents later transitions from masking the first boundary. Shared helper executes Lua, panics on exec error, then checks recorded Lua errors only if exec succeeded. Full callback traceback and every panic/startup record are preserved, not reduced to counts. Saved production feed inserts visualization/membership before notification; this source evidence explains intended supplied-input ordering, not acceptance of downstream lifecycle.

## Spec correction / historical gate

Read-only `git show` confirms `1be01af09f584b1b0afdbb15c1c463f82f0759ab` changes only `docs/specs/ui-widget-text-with-state.md` (one insertion/one deletion), **no Rust changes**. New wording excludes negative order from consumer fixtures while explicitly allowing negative consumer padding to exercise unchanged template clamp-to-zero; query snapshots preserve negative input. Saved consumer assigns `-3.75` and expects `textHeight + Clamp(padding, 0, textHeight - 1)`; saved query replacement expects `bottomPadding = -3.75`. Wording contradiction is corrected; runtime clamp/query assertions remain unreached in this RED run.

`verification/text-state-input-source/report.md` source gate (caller reference 3807) remains historical **spec FAIL** at its original scope/revision. It is not rewritten or retroactively called PASS. Prior format evidence is not rerun for a spec-only correction.

## Warnings / limitations

Runtime stderr contains startup diagnostics, five direct panics, one callback Lua error with complete traceback, feed immediate panic and backtrace note. No runtime warning lines observed. Existing compile stderr contains six deprecated Clippy manifest-key warnings plus aggregate `iced_wgpu` six-warning line, three package-cache lock waits and successful compile finish; verbatim preserved in proof, not suppressed or relabeled clean.

Saved execution only. No launches, builds, checks/formats, hashes, replay, operations, canonical source/config edits, delegation, commits or parent closure. No PTR, native, rendered/pixel, secret-policy, complete21 or cached-consumer GREEN claim. Only requested verification artifacts written.
