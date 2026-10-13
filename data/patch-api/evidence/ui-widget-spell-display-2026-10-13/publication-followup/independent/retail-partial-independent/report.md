# Retail producer38ec — saved partial-runtime verification

**Verdict: PARTIAL / consumer FAIL. Not GREEN, not all-consumer proof, not native acceptance.**

## Scope and evidence

Artifact-only verifier, following `<HOME>/AgentConfig/skills/verify/SKILL.md`; no delegation, replay, build, hashing, reauthentication, operations, or model change. Read complete saved stdout, stderr, result.json, argv.json, admission.json, launcher.txt. Source reads explain boundaries only; they do not authenticate the executable, code archive, libraries, or cached vendor contents.

Absolute epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-green-20261013T184746921697Z/runtime-spell-display-green`.

Saved result labels source `38ecbd8ee5dfae4315ca98001b7e794e324b1066`, profile `retail`, filter `ui_widget_manager_spell_display::`, expected cases 9. This is an attributed producer revision, not a new authenticated source-to-binary association. Receipt reuse is stat-only: device 66306, inode 142233202, size 470894624, mtime_ns 1791917428680394237.

stdout: 1598 bytes; `FAILED. 6 passed; 3 failed; 0 ignored; 0 measured; 10984 filtered out; finished in 5.46s`. Saved launcher: exited status 101, service runtime 5.545s, CPU 5.242s, peak memory 1.7G, swap 0B. No timeout reported.

stderr: full 17739 bytes read in bounded contiguous chunks, including all three panics and final startup records. Direct byte inspection counts 237 newline terminators / 237 splitlines, ending in newline; read-tool display calls it 238 lines. User's 239-line description is not reproduced by that byte count; no content skipped based on the discrepancy.

argv records systemd-run --user --wait, agents.slice, MemoryMax=4G, MemoryHigh=3G, CPUQuota=200%, KillMode=control-group; timeout TERM 86 seconds / kill-after 2; bwrap with unshared network, read-only root, saved source overlay, read-only canonical target, tmpfs /tmp, and WOW_SIM_CASC=0 / WOW_SIM_NO_SOUND=1. Executable is the epoch's artifacts/deps/integration-9f07e4a55f2bbbf7, --nocapture, --test-threads=1. Admission records allowed=true, worker_max=4294967296, worker_high=3221225472, profile retail. These describe the saved launch, not current authentication or fresh execution.

## Runtime coverage matrix

Names below are under `ui_widget_manager_spell_display::`. Archived test paths below are relative to the epoch's `source/`.

| Case | Saved outcome | Reached proof | Missing proof |
|---|---|---|---|
| queries::spell_display_complete_nested_fresh_replace_delete | PASS | Complete 14-field outer / 16-field nested snapshot; fresh nested identity; caller mutation isolation; retained original; absent single-nil return; replacement and deletion | Native/catalog/render acceptance |
| queries::spell_display_same_id_environment_isolation | PASS | Independent empty second map; second record; first unchanged | Cross-process or native behavior |
| queries::spell_display_strict_numeric_ids_without_integer_range_inference | PASS | Supplied negative/fractional/zero/>u32 keys; -0 normalization; distinct absent keys; rejects missing/nil/string/bool/table/function | General numeric-range/native contract |
| feed::spell_display_feed_commits_before_events_retains_visualization_on_membership_removal | PASS | Real frame observer sees committed membership and nested snapshots; replacement without duplicate membership; immutable previous snapshot; removal notification once; absent removal false; visualization retained | Cached consumer lifecycle |
| feed::spell_display_feed_rejects_type_mismatch_unknown_set_without_mutation_or_notification | PASS | Unknown set/type mismatch rejected; record/membership unchanged; no notification | Cached consumer recovery |
| security::spell_display_real_secret_ids_allow_untainted_deny_tainted_recover | PASS | Actual host-wrapped secret numbers; untainted supplied/absent reads; public result fields; tainted secret rejection/no unwrap; ordinary reads allowed; return to untainted fresh reads and mutation isolation | Native acceptance or broader security surface |
| consumer::spell_display_attached_player_context_separate_from_world_loot_policy | FAIL | Entire attached-context Lua block returned successfully; subsequent error-vector check failed | Post-supply layout/pool flush; completed/error-free Setup; world-loot policy |
| consumer::spell_display_cached_activation_update_hidden_restore_remove | FAIL | Cached prerequisites and empty-container flush completed; feed reached cached Setup; partial activation assertion block | Full initial presentation, updates, hidden release, restore, removal/reconciliation |
| consumer::spell_display_zero_spell_polling_uses_real_on_update_without_feed_notification | FAIL | Cached prerequisites and empty-container flush completed; zero-spell feed reached cached Setup; pre-poll assertion block | Host replacement, actual 0.01 OnUpdate poll, transition to supplied spell |

PASS means this saved runner reported that case passed. Mapping to assertions comes from reading the archived tests, not executable reauthentication.

## Exact consumer boundaries

`tests/widget_discrete/fixtures.rs:40-48`: `assert_lua` first calls `env.exec(code)`; an execution failure panics at line 42, before checking errors. Successful exec then clones lua_errors and asserts empty at line 44. Thus Lua assertions and an error-free event path are separate gates.

All three cases returned through `load_and_register_spell_container_env`: initial supplied snapshot, cached addon closure including Blizzard_UIWidgets, enum/catalog/template checks, real empty widget-set registration, UPDATE_UI_WIDGET / UPDATE_ALL_UI_WIDGETS subscriptions, empty widgetFrames, and initial flush asserting no dirtyLayout, showing=0, active pool=0, numTimers=0 and ticker=nil. This does not prove post-supply layout.

### Attached player context

Saved stderr panic at fixture line 44, phase `container attachedUnit propagation only`, with two collected error entries (original handler error plus ScriptErrors wrapper), not two independent consumers. Because exec returned, all four Lua assertions in `tests/widget_spell_display/consumer.rs:198-202` completed:

- widgetFrames[761301] exists;
- container.attachedUnit and frame.attachedUnit are both player;
- frame.Spell.spellInfo.text equals Supplied spell;
- fresh API result attachedUnit is nil (context is not an API field).

The next `flush(&env, 1)` was not reached. Passing these assertions does not establish completed Setup, error-free activation, layout/pool counts after activation, or world-loot policy. This case failed despite successful eval.

### Activation/update/hidden/restore/removal

Saved panic at fixture line 42, phase `actual event activation`, bare `Lua error: assertion failed!`. The block's first assertion assigns SpellFrame from widgetFrames[761301]; second tests `SpellFrame:IsShown() and SpellFrame.hadSpellID`. Saved traceback shows inner Spell Setup aborted before outer Setup could set hadSpellID true. With the inspected cache's execution order, the second assertion is the inferred failure, and the frame-existence assertion is inferred completed; the bare panic itself does not identify the individual Lua assertion.

Unreached assertions in that block: spellID=35395, text, stack count, tooltip, outer/inner width and height, icon width=36, earned-check hidden, rarityPipPool active=0. All subsequent flush(1), incremental nested presentation (same frame, updated text/tooltip/count/earned/red tint/text width/retained snapshot), hidden release and getter Hidden state, flush(0), restore from pool, membership removal/reconciliation/retained visualization, and final flush were not reached. No coverage is credited to them. In particular, the later rarityPipPool expression was never evaluated; it cannot be treated as passed or as this run's failure.

### Zero-spell polling

Saved panic at fixture line 42, phase `zero-spell consumer before poll`, bare assertion failure. First assertion assigns PollingSpell from widgetFrames[761301]; second tests `not PollingSpell.hadSpellID and PollingSpell.Spell.spellID == 0`. Inspected cache sets hadSpellID=false at outer Setup entry, but inner Setup errors before assigning self.spellID. Thus second assertion is the inferred failure and frame existence inferred completed, not individually localized by the saved panic.

Unreached: explicit host map replacement with supplied_record, `poll has not run` assertion, actual fire_on_update(0.01), same-frame check after poll, hadSpellID=true/spellID=35395, and supplied text after poll. Initial empty-container fire_on_update(0.0) is not zero-spell polling proof.

## Failure boundary, not a native contract decision

All three recorded traces share:
`UnregisterEvent` -> cached `Blizzard_UIWidgetTemplateBase.lua:697` UpdateTypeIcon -> :596 SetIconAndBorderDisplay -> :514 Setup -> SpellDisplay.lua:35 Setup -> UIWidgetManager.lua:526 ProcessWidget.

Exact error: `Attempt to register unknown event "PLAYER_EQUIPED_SPELLS_CHANGED"`. Preserve the spelling EQUIPED. The trace's operation is UnregisterEvent, although the error string labels RegisterEvent twice.

Archived `src/lua_api/frame/methods/text_attribute_event/events.rs:78-135`: UnregisterEvent -> remove_registered_event_checked -> mutate_registered_event_checked(Remove) -> ensure_registerable_event. The same checked mutation handles Insert and Remove; the validator formats its error with RegisterEvent regardless of operation. This explains the label without converting the observed operation into RegisterEvent.

Inspected cached Base.lua:697 unregisters this event in the non-world-loot branch. Base.lua:501-514 stores spellInfo before calling SetIconAndBorderDisplay; its self.spellID assignment is later at :558. SpellDisplay.lua:31-41 sets hadSpellID=false and attachedUnit before inner Setup; dimensions and hadSpellID=true are after the failing call. UIWidgetManager.lua:467 stores widgetFrames before :526 Setup. These explain partial state and the inferred assertion boundaries. Current cache reads are explanatory, not proof of historical cache identity.

Not determined: whether the event is valid in native Retail; whether native UnregisterEvent accepts unknown names; whether the validator, event catalog, cached UI revision, or a different model should change. No permissive unregister contract inferred. Main owns operations and model decision.

## Separate evidence and acceptance limits

User identifies separate full compile proof (741-second terminal / exit 0), six manifest deprecations, source formatting/readability, and 3862 PASS. Those artifacts were not independently examined in this runtime-only scope. They do not turn these three consumer failures into passes or authenticate the code archive/libraries. Existing receipt is stat-only; no new hash or authentication performed.

No all-consumer proof, GREEN, native acceptance, renderer acceptance, native widget catalog proof, full-parent acceptance, or deployment claim. Saved epoch naming containing green does not alter the FAIL result.

Private deliverables: `<PRIVATE_VERIFICATION_ROOT>/spell-display-retail-producer-partial-current/report.md` and `<PRIVATE_VERIFICATION_ROOT>/spell-display-retail-producer-partial-current/proof.json`.
