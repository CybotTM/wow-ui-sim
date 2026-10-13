# UIWidgetManager state

UIWidgetManager must expose environment-local supplied widget-set state and drive the existing cached Blizzard manager consumers. The modeled surface lives in `src/c_api/ui_widget_manager.rs` and its input module; this contract prioritizes the missing manager state system, not complete native widget coverage. See [widget architecture](../widget-system.md) and [events](../event-system.md) for the existing runtime boundaries.

## What it must do

### Supplied state

- [ ] Each environment owns independent widget-set metadata and ordered membership records containing explicit `widgetID`, widget type, and `unitToken`; changes in one environment do not affect another.
- [ ] Set queries return supplied metadata and membership in supplied order, without synthesizing membership from widget-ID ranges.
- [ ] IconAndText queries return the full supplied record, without filling missing fields with plausible defaults.
- [ ] Existing complete supplied ButtonHeader and Prey behavior remains intact; manager state does not replace or truncate those records.
- [ ] Quest-to-set and widget-to-type mappings use exact supplied associations, never numeric thresholds or inferred ID ranges.
- [ ] Existing twelve demo world quests retain explicitly stored Tooltip/BehindIcon/AdventureMapDetails sets, with their previous fixture IDs and Worldquest texture kit. Initialization derives only these fixture associations; queries never infer associations or invent IDs. No unrelated records are seeded.

### Next slice: supplied bars and HUD selection

Source-input-only staging follows core `e887`; query producers remain unchanged and next-slice RED has not run. These requirements are not passing claims.

- [ ] `GetStatusBarWidgetVisualizationInfo` and `GetCaptureBarWidgetVisualizationInfo` return complete supplied 35-field StatusBar and 22-field CaptureBar records respectively, preserving field names, Lua types, enum scalars, and values from the current Retail/PTR documented schemas (equivalent except a blank line). Absent IDs return nil; IDs never infer records or catalog entries.
- [ ] Every bar query returns a fresh record. StatusBar `partitionValues` preserves supplied array order and is a fresh nested array: caller element edits, insertion/removal, and outer-record mutation cannot alter host inputs, retained snapshots, or later queries. Supplied replacements affect later queries only; environments remain independent.
- [ ] Four environment-local `f64` inputs select TopCenter, BelowMinimap, PowerBar, and ObjectiveTracker widget sets. Each corresponding query returns exactly one number from its input, including distinct explicit IDs without catalog membership. Default zero means no selected set by simulator model policy, not a proven native default. Later input changes affect queries without changing another environment.
- [ ] Active standard HUD containers execute their actual cached OnLoad using selections supplied before loading and explicit set/widget IDs. Assert selected-set registration and actual acquired consumers; default-zero loading leaves no active widget set. Query freshness does not imply dynamic rebinding of already loaded containers.
- [ ] Actual cached StatusBar/CaptureBar templates consume supplied bar fields through the manager path, without mock providers, templates, Setup, or pools. Assert concrete min/max/value, text/tooltip, partition order and geometry, capture neutral-zone geometry (including zero size), pool counts, hidden/restore transitions, incremental updates, membership removal, and unregister cleanup. Flush actual OnUpdate before asserting deferred layout/count cleanup.

### Host updates and consumer lifecycle

- [ ] Host `replace_ui_widget_set`, `update_ui_widget`, and `remove_ui_widget` publish new state before synchronous events. Unknown-set/type-mismatched updates fail without mutation or notification; membership removal preserves independent visualization snapshots.
- [ ] Host state updates become visible through the existing cached manager: `UPDATE_UI_WIDGET` performs incremental updates and `UPDATE_ALL_UI_WIDGETS` performs full reconciliation against current supplied state.
- [ ] Incremental updates activate newly available widgets and update existing consumers using their supplied type and visualization record. **Modeled event adapter:** preserve documented `unitToken` and also publish its supplied string as `unit`, because the current cached container filters that field against `attachedUnit`. Remove this adapter when the cached consumer uses `unitToken`; native event aliases and unit registration/data isolation are not established.
- [ ] Full reconciliation activates current members and removes consumers whose membership has been removed, preserving supplied ordering.
- [ ] Hidden visualization state and subsequent activation are handled by actual manager consumers, rather than only by query snapshots.
- [ ] Unregistering a consumer stops its updates; removal and reactivation leave no stale consumer state.
- [ ] Behavioral tests exercise activation, removal, hidden-state transitions, and unregister through the actual cached Blizzard manager consumer path.

## How it works

- [Widget architecture](../widget-system.md) — API data is consumed by Lua to create ordinary frames.
- [Event system](../event-system.md) — event dispatch architecture.

## Implementation inventory

- `src/c_api/ui_widget_manager.rs` — modeled widget identities and existing ButtonHeader records; exports manager input records.
- `src/c_api/ui_widget_manager/inputs.rs` — supplied set membership and IconAndText inputs; staged StatusBar/CaptureBar maps and four `f64` HUD selections with default-zero model policy.
- `src/c_api/ui_widget_manager/bar_records.rs` — staged complete 35-field StatusBarRecord and 22-field CaptureBarRecord; source inputs only, not implemented query producers.
- `src/c_api/ui_widget_manager/{queries,feed,fixtures}.rs` — fresh snapshots, synchronous host updates and existing demo initialization.
- `src/lua_api/state/sim_state.rs` — environment-owned manager, ButtonHeader, and Prey input state.
- `src/lua_api/globals/quest_surface/task_quest.rs` — exact supplied quest/type lookup.

## Tests asserting this spec

At `d11908dcf`, five supplied-state cases execute 0 PASS / 5 query-boundary FAIL, without fixture/setup failures. At `6a852c322`, actual cached XML container registration fails because supplied set417 is not returned; host-secret query fails on the old numeric extractor. Later lifecycle and secret-recovery assertions remain unreached in RED. The six state cases, one security case, nine ButtonHeader controls, six Path cases and one Prey control pass at `67775404d` (23/24 selected). Actual cached lifecycle initially fails a pre-unregister count assertion: incremental updates mark layout dirty, but the fixture expected a recomputed count before OnUpdate. Diagnostic execution at `3a7c3650d` identifies `restore showing count=1`; cached source defers layout. The fixture subsequently flushes actual OnUpdate before count-dependent cleanup assertions; retain this failed attempt as deferred-layout evidence. Normal startup has zero Lua errors, runtime warnings and AnimSetup diagnostics. At `96a44bb1f`, a separate actual-container unit notification test fails target-text preservation while player refresh passes; the event adapter correction follows that failed attempt. The unit fixture does not prove unit registration or context-data isolation.

Latest bounded core evidence: two lifecycle cases physically pass at `e887` (scoped PASS 2); the prior 23 passing state/security/control cases at `67775404d` are reused unchanged, not rerun at `e887`. This is 25 logical cases with separate proof scopes, not a fresh 25-case execution. No bar/HUD next-slice result is included. Requirement checkboxes remain conservative: only assertions backed by those tests qualify; unit registration/context-data isolation remains unproven.

- `tests/ui_widget_manager_state.rs` — supplied-state isolation, metadata, ordered membership, complete records, and exact mappings.
- `tests/ui_widget_manager_lifecycle.rs` — cached manager incremental/full reconciliation and actual consumer lifecycle behavior.
- `tests/ui_widget_manager_security.rs` — actual host-secret caller policy and public recovery.

## Next-slice tests and proof gaps

- `tests/ui_widget_manager_bars.rs` — two staged bar-query cases; `tests/ui_widget_manager_hud_selections.rs` — three staged HUD-query cases. Next RED unrun; query producers unchanged.
- `tests/ui_widget_manager_bars_consumer.rs` and `tests/ui_widget_manager_hud_containers.rs` — actual cached-consumer drafts being integrated, not execution proof.
- [ ] Implement and prove complete fresh bar queries and supplied HUD-selection queries against the staged requirements.
- [ ] Integrate and execute actual cached bar/HUD consumer tests; distinguish query-boundary failure from later consumer assertions not reached.
- [ ] Prove unit registration and context-data isolation separately if brought into scope; the current unit notification fixture is insufficient.

## Out of scope

- Native DB2 widget catalogs: unknown; no catalog completeness or fabricated catalog entries.
- Native server wire protocol: unknown; supplied host state is not a claim of native transport compatibility.
- Whole native visualization coverage: core models manager state and full supplied IconAndText while preserving existing ButtonHeader/Prey support; the next slice adds only supplied StatusBar/CaptureBar records and HUD selections.
- Dynamic HUD rebinding events, native DB2 catalogs, and native fixed HUD/set/widget IDs: not established or required by this slice.
- Unrelated demo seeds or replacement Blizzard manager behavior: neither is needed for this bounded model contract.
