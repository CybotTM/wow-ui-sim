# Wrath supplied quest-log read projection

Under `client-wrath`, the legacy log read surface projects ordered supplied rich ordinary rows. Source: `src/lua_api/globals/quest_surface/wrath_quest_log.rs`. This closes a bounded W1 read seam, not the quest lifecycle. Existing [quest data flow](../wiki/systems/lua-api.md) and [offer contract](quest-offer-rich-log-coherence.md) describe adjacent state.

## What it must do

- [ ] `GetQuestLogTitle`, `SelectQuestLogEntry`, and `GetQuestLogSelection` use the same ordered rich rows counted by `GetNumQuestLogEntries`; no static catalog fallback. Selection retains quest identity and resolves its current one-based position after reorder.
- [ ] Unchanged Wrath `QuestLog_GetFirstSelectableQuest` and `QuestInfo_ShowTitle` enumerate/read supplied rows; selecting indices 1 and 2 independently displays the corresponding real FontString title, including reversed input.
- [ ] Empty rows produce count 0/0, first-selectable 0, selection 0, nil title lookup, and an empty original title display.
- [ ] Represented ordinary rows return the tuple below; active completion/failed flags, not completion history, determine slot 7.
- [ ] Reads/selection preserve rich/light membership, pending offer, gossip, money, XP, completion history, independent environments and notifications. Only selection and existing dialog-readiness invalidation may change. Direct execution, installed error handler, `lua_errors` and `lua_error_counts` remain clean.

### WotLK 3.3.5 return contract

`title, level, questTag, suggestedGroup, isHeader, isCollapsed, isComplete, isDaily, questID, displayQuestID = GetQuestLogTitle(index)`

| Positions | Represented ordinary-row value |
| --- | --- |
| 1–2 | Supplied title and level |
| 3–6 | nil tag, nil suggested group, nil header, nil collapsed |
| 7 | 1 complete, -1 failed, nil otherwise (failed takes precedence) |
| 8–10 | nil daily, supplied quest ID, nil display-ID flag |

False flags are nil, never boolean false or numeric zero. Unsupported metadata stays absent; neither generic frequency nor numeric tag IDs establish legacy classifications. History-only `is_flagged_completed`/`completed_quest_ids` does not make an active row complete. Invalid title lookup has nil first return; extra-return arity and native invalid-selector policy are not certified.

Period provenance: [WoWWiki API_GetQuestLogTitle](https://wowwiki-archive.fandom.com/wiki/API_GetQuestLogTitle), specifically its **Before patch 6.0.1** signature, documents header/collapsed as 1/nil and completion as -1/+1/nil. [Historical corroboration](https://vanilla-wow-archive.fandom.com/wiki/API_GetQuestLogTitle) includes suggestedGroup added 2.0.3, daily added 2.1.0 and quest ID added 3.3.0; its site label is not a Vanilla version pin. These documentation details were cross-checked in the scope-plan investigation; implementation-time fetch attempts were blocked (402/403), not newly fetched confirmation.

Primary consumer evidence is the whole unchanged cached Wrath `FrameXML/QuestLogFrame.lua`: ten-value unpacking in `QuestLog_Update`, original first-selectable header truthiness, and numeric completion comparisons. [Public 3.3.5 original source](https://github.com/wowgaming/3.3.5-interface-files/blob/main/QuestLogFrame.lua) corroborates the ordering and signed-completion checks. `FrameXML/QuestInfo.lua` uses title/selection in `QuestInfo_ShowTitle`. Positive displayQuestID semantics are not established by the historical nine-return documentation; this adapter leaves it absent. No positive header/collapse/daily semantics are claimed.

## How it works

- [Lua API/state architecture](../wiki/systems/lua-api.md)
- [Quest offer/rich log coherence](quest-offer-rich-log-coherence.md)

## Implementation inventory

- `src/lua_api/globals/quest_surface/wrath_quest_log.rs` — Wrath legacy row/title/identity projection.
- `src/lua_api/globals/quest_surface/mod.rs` — Wrath-only module boundary.
- `src/lua_api/globals/quest_surface/register.rs` — profile-selected legacy bindings, leaving shared C namespace bindings intact.
- `src/lua_api/sim_substates/quest_log.rs` — existing ordered rich rows (unchanged).

## Tests asserting this spec

`tests/wrath_quest_log_projection.rs`: `original_index_one`, `original_index_two`, `original_reordered_index_two`, `selection_tracks_identity_after_reordering`, `original_empty_log`, `completion_flags_use_active_rows_not_history`.

Fixtures load whole cached sources, verify hashes, use real simulator widgets and public selector setup, and never substitute quest APIs or Blizzard functions. Rust supplies neutral rich/light membership directly; this is not acceptance proof. Cached source SHA-256:

- `FrameXML/QuestLogFrame.lua`: `6ac6ca92b3036417082b88cc2f8b5c9fc8889f51f82aa726fe0bcd44990bc5e2`
- `FrameXML/QuestInfo.lua`: `3bfe691acdb2a095a6633c3ce7bef4a13f53f464d20781b2e850e442f9b715cd`

## Known gaps (current cycle)

- [ ] Bounded Wrath runtime and preservation proof pending; no native/full-profile acceptance claim.

## Out of scope

Headers, collapse/grouping, daily classification, tag localization, full `QuestLog_Update`, original `QuestLog_SetSelection`, full `QuestInfo_Display`, objectives/progress/readiness, acceptance/events, watches, abandon, rewards, other profiles, C_QuestLog redesign and static fallback catalogs. `GetQuestLogIndexByID`, C namespace indices and static objective/text reads remain separate downstream gaps. `IsCurrentQuestFailed` representation is not certified. Original full selection remains excluded because its `SetAbandonQuest` call currently removes membership; fixing this read seam must not imply that boundary is repaired.
