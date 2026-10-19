# Wrath supplied quest objectives

Wrath ordinary active quests project supplied identity-keyed objective snapshots through legacy reads and unchanged `QuestInfo.lua` / `QuestLogFrame.lua` consumers. Reuse the [quest-log state and selected identity](wrath-quest-log-projection.md); individual objective completion is not quest acceptance, readiness or turn-in. See [legacy surface architecture](../wiki/systems/lua-api.md#wrath-selected-quest-legacy-surface).

## What it must do

- [ ] **Supplied state:** count, text, type and individual finished state come from the existing snapshot. Preserve authored row order and explicit type labels. Empty and missing snapshots both render no objectives, without catalog fallback; their stored states remain distinct. Queries are read-only and environment-local.
- [ ] **Identity:** implicit selection and explicit one-based rich-log indices identify the same quest after reorder. Native indices count headers; this slice's supplied rows contain no headers, so flat rich positions equal displayed positions. Valid explicit indices work without selection.
- [ ] **12340 tuple and arguments:** `GetQuestLogLeaderBoard(index [, questIndex])` returns exactly three values: text, type, finished (`1` or nil); unavailable rows return three nils. Missing/nonnumeric objective index raises `Usage: GetQuestLogLeaderBoard(index)`. Finite numeric indices truncate; numeric strings use Lua 5.1 conversion. Omitted, nonnumeric or invalid explicit leaderboard quest index uses selection. `GetNumQuestLeaderBoards` instead returns zero for an invalid numeric explicit index; omitted/nonnumeric input uses selection. It returns one number.
- [ ] **Original rendering:** unchanged `QuestInfo_ShowObjectives` and `QuestLog_UpdateQuestDetails` populate actual XML FontStrings with supplied text, black unfinished rows and grey finished rows plus localized `COMPLETE`. Preserve returned last widget and correct visibility. Shorter/empty snapshots hide stale rows. Empty text uses the original consumer's type substitution. Individual finished transitions leave rich completion/failure, history, membership, money, XP and other quests unchanged.
- [ ] **Authored summary:** `GetQuestLogQuestText` projects supplied text joined with newlines, not a claim of native narrative-summary equality. Preserve the existing catalog description policy; custom quests have empty descriptions. Consumer execution adds no direct, handler or simulator errors to the exact frozen startup baseline; never clear/filter existing errors.

12340 authority: laptop source findings `objective-readers-12340.md` (2026-10-19, aso session 01a140f8; exact private artifact location in lane handoff). This is source-derived evidence, not native runtime capture. Native types are `event`, `monster`, `object`, `item`, `player`, `reputation`, `log`; this slice accepts authored labels rather than deriving categories from native quest records.

## How it works

- [Legacy selected-quest surface](../wiki/systems/lua-api.md#wrath-selected-quest-legacy-surface)
- [Original quest-log projection boundary](wrath-quest-log-projection.md)
- [Shared supplied-objective model](quest-objective-progress.md)

## Implementation inventory

- `src/c_api/quest_objectives.rs`: existing supplied snapshot types and mutation boundary; Retail/Forever capability gate remains unchanged.
- `src/lua_api/sim_substates/quest_log.rs`: existing rich membership, identity-keyed snapshots and abandon/turn-in cleanup.
- `src/lua_api/globals/quest_surface/wrath_objectives.rs`: supplied legacy count/row/summary adapters and 12340 argument/tuple conversion.
- `src/lua_api/globals/quest_surface/wrath_quest_log.rs`: Wrath-only registration overrides; no shared registration-order change.

## Tests asserting this spec

`tests/wrath_quest_objectives.rs` loads whole original cached UI sources and real widgets via `tests/common/wrath_full_ui.rs`; `tests/wrath_quest_objectives_fixture.lua` asserts consumer output. Source seals authenticate the whole consumers. Required cases: direct objective body, full detail entry, two noncatalog quests, reorder/explicit reads, individual completion, shorter/empty lists, catalog no-fallback, type substitution, read purity, independent environments and bounded direct 12340 tuples/selectors.

Preserve `tests/wrath_quest_log_projection.rs`, `tests/wrath_quest_abandon.rs`, `tests/wrath_quest_offer_select.rs` and `tests/wrath_quest_alpha_gradient.rs`. Combined W1 integration and Retail/Forever preservation remain integration gates, not claims from Wrath-only proof.

## Known gaps (current cycle)

- [ ] Reproduce consumer RED, then supplied-producer GREEN; retain exact revision-bound receipts in lane report.
- [ ] Preserve existing Wrath quest tests; run formatting and warning-free Wrath profile check.
- [ ] Rebase after the parent4 clean-startup merge and refresh the frozen startup baseline separately.

## Out of scope

- **Native category traversal order:** per `objective-readers-12340.md`, native traversal is event, creature/object slots, qualifying collection slots, intermediate slots, player, reputation, complete-log fallback. Flat supplied input lacks category-slot metadata; authored order remains unchanged.
- **Intermediate-item rows and starting-item exclusion:** native intermediates appear only for incomplete quests and are excluded from the count. Neither intermediate identity nor starting-item metadata is represented by this bounded input.
- **Unverified objective-index tail anomaly:** the reported missing overall bounds check may expose player/reputation/log tails for zero, negative or huge indices. Do not assert this anomaly. NaN, infinity and integer-overflow behavior are unmodeled.
- **Acquisition and scheduling:** no absent-player/cache-acquisition model, description acquisition, header insertion, producer notifications, watch automation or automatic refresh policy. Manual unchanged detail refresh establishes rendering only.
- **Quest lifecycle:** no new acceptance/progress/reward/turn-in behavior or inferred readiness/history/reward grants. Do not expand the Retail/Forever objective capability gate or enable Wrath admin setters/modern readiness.
