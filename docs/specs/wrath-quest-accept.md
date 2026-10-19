# Wrath ordinary quest acceptance

Original GossipFrame/greeting selection and QuestFrame accept callback must commit the pending supplied offer to existing light and rich membership, once. No separate quest store or vendor changes.

## Evidence and scope

Authoritative evidence: laptop 3.3.5a build 12340 decompile reading, packet dated 2026-10-18, items 1–2; accepted under AGENTS.md Clean Room. This is source-reading evidence, not a native capture. User decision: `QUEST_ACCEPTED` has log index only. No external API hypothesis overrides this evidence.

Original consumer citations use paths relative to the Wrath runtime source root: `FrameXML/QuestFrame.lua:8–20,339–350`, `FrameXML/QuestFrame.xml:319–332`, `FrameXML/QuestLogFrame.lua:238–265,392–403`. Localized error citation: `FrameXML/GlobalStrings.lua:3425`, `ERR_QUEST_LOG_FULL = "Your quest log is full."`.

## Required behavior

- [ ] Original full startup via `tests/common/wrath_full_ui.rs`; original gossip/greeting buttons select the offer, detail displays supplied title, original fading handler enables Accept. Commit pending offer, not subsequently selected log row. Preserve title/level, existing rows, history, money and XP; consume pending offer on success.
- [ ] **Laptop reading:** `QUEST_ACCEPTED(logIndex)` has exactly one numeric argument, the 1-based displayed index including headers; no quest ID. `GetQuestLogTitle(index)` retains ID at return 9. Cached acceptance follows log rebuild; cache misses/callback failure can delay/suppress acceptance. Existing state-bit transitions and absent lookup index 0 are outside this new-offer slice.
- [ ] **Laptop reading:** `QUEST_LOG_UPDATE` is queued/coalesced, not inline. Deliver actual queued events and prove original visible QuestLogFrame count/title refresh. `UNIT_QUEST_LOG_CHANGED("player")` and `QUEST_ACCEPTED` are direct native signals. Do not impose a total order among these three or exact refresh multiplicity; simulator dispatcher scheduling is not native timing proof.
- [ ] **Laptop reading:** local `AcceptQuest` emits argumentless `QUEST_FINISHED` iff stored giver GUID is nonzero. This closes the dialog, not proof of acceptance, and need not observe committed log state. Zero giver emits no finish; full-log rejection still closes for nonzero giver.
- [ ] **Laptop reading:** no pending offer/wrong state or missing player silently returns, with no events or mutation. Forced original callback after pending consumption is also silent.

## Server-response stand-ins

- [ ] **Simulator stand-in:** accept valid new offers by applying successful server-response effects to both existing memberships. This does not claim native `AcceptQuest` inserts locally.
- [ ] Allow 24 occupied quest slots to become 25. With 25 occupied slots, simulate server `SMSG_QUEST_LOG_FULL`: no insertion or `QUEST_ACCEPTED`, `QUEST_LOG_UPDATE`, `UNIT_QUEST_LOG_CHANGED`; emit `UI_ERROR_MESSAGE` with exactly one string equal to original `ERR_QUEST_LOG_FULL`. No numeric error code. Headers do not occupy quest slots.
- [ ] **Laptop reading, not stand-in:** native client does no capacity check and still sends its request and conditionally emits `QUEST_FINISHED` on the full-log branch. Rejecting locally in the simulator represents the server response, not client validation. No total order between error response and closure is required. Full-log pending disposition is unspecified.

## Test coverage and open gates

`tests/wrath_quest_accept.rs` stages two original offer paths at indices 2/3, no-offer/repeat silence, 24→25 success, 25-slot server-full error and original refresh/closure. These fixtures are flat; header-inclusive indexing is required but not exercised.

- [ ] Preceding abandon/read lane and `tests/common/wrath_full_ui.rs` must land first.
- [x] Explicit `GossipState.giver_guid` supplies stored dialog identity, default zero, with nonzero fixtures set through the existing Rust state input. Authority: laptop `packet-20261018.md`, Q1 (accept closure), Q2 (full-log closure), Q4 (turn-in closure). No inference from `gossip.active` and no fallback giver.
- [ ] Missing-player lifecycle remains unmodeled: `group_queries::unit_exists_in_state("player")` always returns true; `UnitGUID` uses the same resolver and a seeded identity. Reuse that resolver; no parallel presence flag. Header generation, giver acquisition/reset and native identity encoding remain gaps.
- [x] Original-consumer RED on fixture revision `2a284f45f`: missing native AcceptQuest and legacy gossip count producers.
- [ ] Targeted GREEN and Retail/Forever preservation remain gates.

## Out of scope

Party/PvP confirmation, auto-accept, watches, turn-in/rewards, native packet/tick timing, cache failure/state-bit acceptance, header generation and whole-W1 completion.

## Implementation inventory

- `src/lua_api/globals/quest_verbs.rs`: ordinary Wrath producer and existing close/confirm verbs.
- `src/lua_api/globals/quest_surface/legacy_globals.rs`: original offer selection/detail readers.
- `src/lua_api/globals/quest_surface/wrath_quest_log.rs`: supplied-row legacy projection dependency.
- `src/lua_api/sim_substates/quest_log.rs`: ordered rich membership and snapshots.

See [Lua API](../wiki/systems/lua-api.md), [loading](../addon-loading-pipeline.md), [events](../event-system.md).

Continuation RED at `122f26445`: all four full-original tests fail on the legacy `GetGossipText` wrapper before acceptance. Wrath now registers greeting text and active-quest tuples directly from supplied gossip state, independent of the modern namespace. Temporary no-option gossip support remains limited to quest NPC fixtures: legacy option icon-type projection is missing. The temporary progress/ordinary-flag registration must include Wrath even though the modern objective subsystem is disabled there. At `085d2c042`, the no-offer control passes; two original gossip paths next reach missing `CloseGossip`, and greeting reaches missing `GetQuestBackgroundMaterial`. Register modeled legacy closure and the existing supplied-material reader for Wrath. A full-startup diagnostic also reaches nil `GetSuggestedGroupNum`, `GetRewardSpell`, `GetRewardArenaPoints` and `GetRewardTalents`; their legacy detail readers now project the pending rich snapshot (absent rewards remain absent). At `8589e7a21`, 1/4 passes and all accepting paths reach original OnHide → missing `CloseQuest`. Main authorized bringing its basic registration/shared finish path forward: suppress zero-giver finish and clear the stored GUID before emission. **Clear-before-emit is INFERRED ordering, pending native confirmation**, not a supplied native fact. Exactly one FINISHED in this re-entrant acceptance flow depends on that inference. Turn-in dialog-retention rules remain for step B. GREEN remains pending.

Wrath-only temporary ordinary PvP/auto-accept flag defaults live in `src/lua_api/workarounds/temporary/quest_progress_requirements.rs`; retire when supplied quest flag state lands. Acceptance and abandonment share immediate player UNIT / deferred coalesced log notification policy.
