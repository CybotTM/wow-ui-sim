# Simulator selected quest turn-in

Explicitly simulator-authored `A_Admin.TurnInSelectedQuest()` commits a selected completed quest using the existing rich quest-log/history model in `src/lua_api/sim_substates/quest_log.rs` and administrative producer in `src/lua_api/globals/admin_quests.rs`. This is not a native API or native-parity claim. [Implementation notes](../wiki/systems/lua-api.md#simulator-selected-quest-turn-in) describe projections.

## What it must do

- [ ] Consume only the selected positive, i32-representable quest ID whose existing rich row is complete and not failed. Return `true` only for a committed turn-in; no selection, invalid/out-of-range ID, missing row, incomplete or failed row returns `false` without mutation/event. Selection, not a supplied argument or another completed quest, determines the target.
- [ ] Remove target rich/light active membership and, where modeled (Forever), ordinary watch membership/metadata. Preserve unrelated membership and completion history; insert the target into the existing completion set exactly once. A complete represented seed already in history remains eligible; history alone is not an active quest. Repeated calls/reselection of a removed row cannot turn it in again.
- [ ] Clear the consumed selection and matching pending offer, remove matching current gossip rows and refresh counts; preserve unrelated pending offers/rows. A callback retry sees no eligible consumed target.
- [ ] Publish `QUEST_TURNED_IN(questID, 0, 0)` once through existing synchronous dispatch after the full mutation and after releasing the mutable state borrow. Original listeners observe removed active/watch membership and recorded completion, not a staged or manually replayed event. The payload field order is sourced from cached official Lua documentation; zero rewards and this mutation order are explicit simulator policies.
- [ ] Suppress completed IDs in existing task catalog active/map/time projections, including `GetQuestLogTimeLeft`, `GetQuestsOnMap`, the available map alias and `DoesMapShowTaskQuestObjectives`. Rich log/world/map queries reflect removed rows. Preserve unrelated task catalog availability and environment isolation; catalog metadata lookups remain catalog-backed. No second active/completed domain is introduced.

## How it works

- [Quest state and projection implementation](../wiki/systems/lua-api.md#simulator-selected-quest-turn-in)
- [Existing offer/accept/watch/abandon contract](quest-offer-rich-log-coherence.md)
- [Simulator admin API](../admin-api/quests.md)

## Implementation inventory

- `src/lua_api/sim_substates/quest_log.rs`: eligibility, rich-row removal and existing completion-set insertion.
- `src/lua_api/globals/admin_quests.rs`: simulator-only endpoint, shared membership/dialog cleanup and original synchronous event dispatch.
- `src/lua_api/globals/quest_surface/task_quest.rs`: static task availability filtered by existing per-environment completion history; metadata stays catalog-backed.

## Tests asserting this spec

`tests/quest_turn_in_model.rs` contains four authored behavioral cases: selected ordinary completion/watch/event-boundary/reentrant and repeat controls (Forever), rich-only world task/map/time/history/environment projections (shared), invalid/incomplete/failed controls (shared), and represented completed-seed/dialog/pending preservation (shared). Existing completion is supplied by setting the existing rich row's `is_complete` fixture field; no gameplay objective-completion producer is claimed.

Status: producer committed as `b5dd5947a` after actual shared Retail behavioral RED; producer GREEN has not been run. Three shared cases executed: zero PASS/three FAIL, exit 101. The world-task and represented-seed/dialog cases stopped at unchanged rich active membership; the invalid-selection case stopped at selection `None` because the missing endpoint yielded nil instead of false. All later assertions in those cases remain unreached. The fourth authored case is Forever-only and was not executed: ordinary watch cleanup, original synchronous listener visibility, reentry and repeat-event controls have no runtime proof here. Committed tests are `96ba1ba4d`; saved receipt metadata labels the test/source snapshot `b7e380f2fe56ceb820cbbe355c6a6c94aac8d339`, not the test-authoring commit or runtime source. The executed copied fixture used retained normal Retail library source `bbeac62133cfbf678faf3eca07eab27f6b9bcec8` (getter `64a5576b1`), not the subsequently committed producer or unrelated EditBox source. Receipt: `/home/osso/.local/state/wow-ui-sim/verification/wrath-source-current/quest-turnin-retail-red-20261016T172145753836Z`. Independent RED audit accepted this evidence in `/home/osso/.local/state/wow-ui-sim/verification/quest-turnin-next/red-report.md`; no GREEN, Forever-event ordering, native or full-parent claim. Missing endpoint is tolerated only by the test invocation helper, not production fallback.

## Known gaps (current cycle)

- [ ] Focused GREEN, Forever watch/event ordering, formatting/readability/profile checks and independent verification remain pending.
- [ ] Native behavior, reward acquisition, objective progression and broad UI acceptance remain unproven and are not asserted by supplied fixtures.

## Out of scope

Native `GetQuestReward`/`CompleteQuest` publication or parity, native validation/event sequencing beyond the documented payload shape, quest objective progression, automatic watches, inventory/currency/XP/money rewards, transport, native catalogs, persistence, new repeatable/reacceptance/reset policy and whole-parent acceptance. `is_replayable` is not changed or promoted into a repeat-turn-in lifecycle. The existing seed catalog stays intact; only active/map/time projections exclude completion-history IDs under this authored scenario.
