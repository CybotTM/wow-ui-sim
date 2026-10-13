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

### Host updates and consumer lifecycle

- [ ] Host `replace_ui_widget_set`, `update_ui_widget`, and `remove_ui_widget` publish new state before synchronous events. Unknown-set/type-mismatched updates fail without mutation or notification; membership removal preserves independent visualization snapshots.
- [ ] Host state updates become visible through the existing cached manager: `UPDATE_UI_WIDGET` performs incremental updates and `UPDATE_ALL_UI_WIDGETS` performs full reconciliation against current supplied state.
- [ ] Incremental updates activate newly available widgets and update existing consumers using their supplied type, unit token, and visualization record.
- [ ] Full reconciliation activates current members and removes consumers whose membership has been removed, preserving supplied ordering.
- [ ] Hidden visualization state and subsequent activation are handled by actual manager consumers, rather than only by query snapshots.
- [ ] Unregistering a consumer stops its updates; removal and reactivation leave no stale consumer state.
- [ ] Behavioral tests exercise activation, removal, hidden-state transitions, and unregister through the actual cached Blizzard manager consumer path.

## How it works

- [Widget architecture](../widget-system.md) — API data is consumed by Lua to create ordinary frames.
- [Event system](../event-system.md) — event dispatch architecture.

## Implementation inventory

- `src/c_api/ui_widget_manager.rs` — modeled widget identities and existing ButtonHeader records; exports manager input records.
- `src/c_api/ui_widget_manager/inputs.rs` — currently referenced supplied set membership and IconAndText input records.
- `src/c_api/ui_widget_manager/{queries,feed,fixtures}.rs` — fresh snapshots, synchronous host updates and existing demo initialization.
- `src/lua_api/state/sim_state.rs` — environment-owned manager, ButtonHeader, and Prey input state.
- `src/lua_api/globals/quest_surface/task_quest.rs` — exact supplied quest/type lookup.

## Tests asserting this spec

At `d11908dcf`, five supplied-state cases execute 0 PASS / 5 query-boundary FAIL, without fixture/setup failures. At `6a852c322`, actual cached XML container registration fails because supplied set417 is not returned; host-secret query fails on the old numeric extractor. Later lifecycle and secret-recovery assertions remain unreached in RED. A sixth state case covers host feed publication/error handling; no current producer GREEN is claimed.

- `tests/ui_widget_manager_state.rs` — supplied-state isolation, metadata, ordered membership, complete records, and exact mappings.
- `tests/ui_widget_manager_lifecycle.rs` — cached manager incremental/full reconciliation and actual consumer lifecycle behavior.
- `tests/ui_widget_manager_security.rs` — actual host-secret caller policy and public recovery.

## Known gaps (current cycle)

- [ ] Connect the pending host input producer to environment-local manager state.
- [ ] Prove the supplied-state contract with the staged state tests.
- [ ] Prove incremental/full reconciliation and consumer lifecycle with the staged lifecycle tests.
- [ ] Verify existing complete ButtonHeader and Prey preservation and explicit demo world-quest behavior.

## Out of scope

- Native DB2 widget catalogs: unknown; no catalog completeness or fabricated catalog entries.
- Native server wire protocol: unknown; supplied host state is not a claim of native transport compatibility.
- Whole native visualization coverage: this cycle models manager state and full supplied IconAndText while preserving existing ButtonHeader/Prey support.
- Unrelated demo seeds or replacement Blizzard manager behavior: neither is needed for this bounded model contract.
