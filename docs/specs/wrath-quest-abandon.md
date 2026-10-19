# Wrath quest abandon confirmation

Wrath legacy abandonment separates selection from confirmation in `src/lua_api/globals/quest_verbs.rs` and `src/lua_api/globals/quest_surface/`. It reuses the existing quest stores and target field. See [quest state architecture](../wiki/systems/lua-api.md) and [Wrath log projection](wrath-quest-log-projection.md).

## What it must do

- [ ] Under `client-wrath`, `SetAbandonQuest` records the selected active rich quest identity without removing membership, auxiliary state, pending offers or completion history, granting rewards or publishing success notifications. Empty/invalid selection clears the target.
- [ ] `GetAbandonQuestName` reads the marked rich quest title, even if current selection changes. The unchanged original XML abandon button opens the original no-items `ABANDON_QUEST` dialog with that title; its real cancel button preserves quest state.
- [ ] As a simulator stand-in for the server response, `AbandonQuest` consumes the marked target once, removes only that quest from rich/light membership and objective/background/readiness stores, clears a matching pending offer and selected identity, and preserves unrelated rows, pending offers, history, money and XP.
- [ ] Provisional simulator notification policy: successful Wrath confirmation queues one argumentless `QUEST_LOG_UPDATE` after the modeled removal commit, not the modern `QUEST_REMOVED` event. Missing/stale/consumed targets do not remove another quest or publish another success notification.
- [ ] Wrath `GetQuestLogGroupNum` reads the selected rich entry's supplied suggested group size, or zero when absent. Original `QuestInfo_ShowGroupSize` renders supplied group text and hides it for a no-group row.
- [ ] Original `QuestLog_SetSelection` remains nondestructive. Original `QuestLog_OnEvent` observes the committed log through its `QUEST_LOG_UPDATE` path. Original consumer errors remain visible rather than replaced with test implementations.
- [ ] Retail, PTR and Forever retain their existing destructive `SetAbandonQuest` contracts.

## How it works

- [Lua API and quest state](../wiki/systems/lua-api.md)
- [Legacy read projection](wrath-quest-log-projection.md)

### Preliminary 3.3.5a evidence and limits

Source-derived 3.3.5a build 12340 findings support nondestructive `SetAbandonQuest` target capture and a single numeric `GetQuestLogGroupNum` return, zero when no entry matches selection. These are preliminary source findings, not native captures.

Native `AbandonQuest` sends a remove-by-slot request; it does not remove local membership or fire events itself. Removal arrives through a server quest-log update. This simulator has no server, so its confirmation path stands in for that response. Local commit, target consumption, notification count/payload and delivery order are provisional simulator policy, not proven native API sequencing. Original FrameXML consumption establishes UI compatibility, not native event order; exact conditional event delivery remains under investigation. No request/server architecture redesign is included.

## Implementation inventory

- `src/lua_api/globals/quest_verbs.rs` — profile-specific marking/confirmation over existing state.
- `src/lua_api/globals/quest_surface/wrath_quest_log.rs` — marked supplied-title projection and selected rich-row group recommendation, registered only for Wrath.
- `src/lua_api/sim_substates/quest_log.rs` — existing rich membership and auxiliary stores; no parallel quest store.

## Tests asserting this spec

- `tests/wrath_quest_abandon.rs` — hash-pinned original consumers loaded through the real full Wrath startup, original XML button and popup, supplied noncatalog rows and state/notification controls.
- `tests/common/wrath_full_ui.rs` — shared original FrameXML TOC/startup addon ordering.
- `tests/quest_verbs.rs`, `tests/quest_abandon_membership.rs` — existing cross-profile preservation contracts.

## Known gaps (current cycle)

- Consumer RED: original `QuestLog_SetSelection(1)` removes supplied rich/light membership and auxiliary state before confirmation. Original full startup records no Lua errors in this probe.
- Group-size RED: original `QuestLog_SetSelection(1)` reaches `QuestInfo_ShowGroupSize` and fails at its missing `GetQuestLogGroupNum` call. The selected-row read is implemented; original full-selection GREEN remains pending.
- [ ] Confirmation/log-refresh GREEN and cross-profile preservation pending.

## Out of scope

Items-to-destroy, watch-index modeling, acceptance, turn-in, reward acquisition, native event timing and whole W1/profile acceptance. `GetAbandonQuestItems` remains the existing nil no-items branch; that does not establish item destruction support. No Blizzard/vendor code or callback replacement is permitted.
