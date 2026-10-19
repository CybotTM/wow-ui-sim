# Wrath 12340 quest turn-in

Wrath 3.3.5a (build 12340) dialog requests consumed by the unchanged cached `FrameXML/QuestFrame.lua` and XML. Backing state lives in `src/c_api/quest_readiness.rs` and `src/lua_api/sim_substates/quest_log.rs`; native readers/request registration and simulator response adapters are being implemented through the staged A/B proof below. See [quest implementation notes](../wiki/systems/lua-api.md#simulator-selected-quest-turn-in).

Binding evidence artifact identifiers: `laptop-labels-answers-20261018:turnin-guards-12340` (Q-T1–Q-T3) and `laptop-labels-answers-20261018:getquestreward-choice-12340` (choice conversion/error details), laptop answers dated 2026-10-19. These identifiers are not repository files or filesystem paths. This packet uses that answer, not decompiled source or a native capture. Requirement checkboxes remain open until final GREEN. The integrated fixture compiled and reached shared feature-absence RED at `84f7067d6`: original progress UI calls missing `IsQuestCompletable`; no distinct guard/conversion/event RED is credited from that setup failure.

## What it must do

### Native request contract

- [ ] `CompleteQuest` sends only in dialog state 2 (Progress); `GetQuestReward` sends only in state 3 (Rewards). Both require an active player and an unsent response. Wrong state, absent player or prior response is ignored with zero Lua results and no immediate or queued events. Guard rejection must precede reward-choice validation.
- [ ] Eligible invalid choices raise an error ending in the exact literal body `Invalid reward choice in GetQuestReward([choice])` without consuming the response opportunity. `[choice]` is literal, not substituted. `luaL_error` level 1 may prepend `<short_src>:<line>: ` when the caller has a line; do not require a bare whole message at an uncontrolled call site. Sending alone neither removes the quest nor records acceptance.
- [ ] Convert the argument to int32 by truncation toward zero, then subtract one. Omitted/nil/non-numeric input becomes internal -1 without a type error; numeric strings convert and extra arguments are ignored. With N>0, accept internal 0 through N-1: omitted/nil/0/-1/N+1 error; 1.5 and "1" choose the first reward; N+0.5 chooses the last. Successful calls set responseSent; each test input starts a fresh eligible dialog. Guard-rejected calls return before validation, with no error or send.
- [ ] With N=0 skip bounds validation for ordinary inputs: internal<=0 sends index 0, positive internal values are preserved (3 sends index 2). Sending is not server acceptance. Do not invent successful-call result arity from the ignored-call zero-result rule.
- [ ] Requests use the current dialog quest, never the selected quest-log row. Neither missing NPC/zero giver nor incomplete/failed rich rows is an additional native send guard. UI-disabled-button suppression and server acceptance are separate from native sending.
- [ ] `CloseQuest` clears the NPC GUID and emits argumentless `QUEST_FINISHED`, without clearing dialog state/current quest/response flag. An unanswered closed dialog still sends; a sent dialog stays blocked after close. A new original active-title selection resets the response latch through SetState.

### Completion notifications

- [ ] At the completed quest-cache callback boundary, completion emits `QUEST_FINISHED()` only when next quest ID is zero and giver GUID is nonzero. Concrete GUID zero must not fall back to an interacting NPC. A closed dialog's giver must not be resurrected by offer/claim. Native cache hits may complete inline and misses defer; acceptance plus a generic event-queue drain does not establish callback completion.
- [ ] `UNIT_QUEST_LOG_CHANGED("player")` is immediate, one token per emission; additional aliases are permitted. `QUEST_LOG_UPDATE()` is deferred and coalesced. Tests constrain only UNIT before the deferred LOG_UPDATE flush, never FINISHED versus UNIT.
- [ ] No turn-in path emits `QUEST_TURNED_IN`: it does not exist in build 12340.

### Explicit simulator policy, not native server acceptance

- [ ] Proposed `A_Admin.OfferQuestReward()` consumes a pending native presentation request and supplies the server offer, publishing `QUEST_COMPLETE` for the original reward UI. The test must not fabricate that event or replace a native verb/callback.
- [ ] Wrath `A_Admin.TurnInSelectedQuest()` supplies acceptance for a pending dialog claim, regardless of unrelated log selection; no pending claim returns false with no mutation/events. Completed/nonfailed supplied rows are accepted in these fixtures. Actual server acceptance of arbitrary/stale/closed/incomplete/failed requests is unknown. Fixtures explicitly supply `completion_callback_cache_hit = true`: this response policy completes the quest-cache callback inline within acceptance. FINISHED counts are checked only for that supplied completed-callback boundary, not as a native timing guarantee. False leaves callback scheduling unspecified; no deferred-cache implementation is demanded by these cases.
- [ ] Accepted target 990301 leaves active log/history coherent while complete control 991300 and historical ID 998888 survive. Supplied choice receipts are recorded only at acceptance and preserve unrelated receipt `(998888, 1)`; no-choice index zero adds no choice receipt. These receipts are not inventory grants.
- [ ] Two accepted pending claims in one notification pass each increase the observed player-token emission count before any drain and produce one later argumentless log update. Open the second dialog through original active-title selection, never by resetting the latch in the fixture. Original QuestLogFrame renders the final count. Callback repetition cannot create a second accepted claim without a new dialog.
- [ ] The original UI's no-choice button path accepts without a choice receipt under supplied simulator policy. This does not establish native omitted/nil/zero normalization or successful-call result arity.

The original happy-path receipt-preservation control now explicitly supplies unrelated receipt `(998888, 1)` before any request. A2 reached its final unchanged oracle and exposed that missing input (`{}` versus `{998888: 1}`); the correction is fixture-only. Empty-receipt no-choice controls retain their original empty input/oracle.

## Staged producer proof

Step A implements current-dialog selection/readers, response-latch reset on new dialog, ordinary-integer claims and pending-request-consuming simulator offer/acceptance. Wrath log selection no longer erases the dialog target. It deliberately does not implement native state/player/sent guards, reward conversion/validation, CloseQuest retention, completion FINISHED, or turn-in notifications. Basic shared CloseQuest/acceptance finish plumbing was brought forward to unblock acceptance under Main's explicit disposition. Its clear-before-emit order is **INFERRED pending native confirmation**; zero-giver suppression prevents original OnHide re-entry from publishing a second finish. The first full A run at `f321476f4` reached all 30 cases but each stopped at the shared missing legacy `IsQuestFlaggedCompleted` reader in membership/history checks. Those are not R3 boundary receipts. The legacy reader now projects the same existing completion-ID set as `C_QuestLog`; no assertion is weakened. Corrected step-A runtime receipts are still required before B.

## How it works

- [Quest model notes](../wiki/systems/lua-api.md#simulator-selected-quest-turn-in)
- [Event delivery](../event-system.md)
- [Existing authored admin scope](quest-turn-in-model.md)

## Fixture integration amendment

Main approved using the existing `GossipState.giver_guid` as the single concrete giver input instead of v4's additional `QuestReadinessState.giver_guid`. All supplied GUID values and behavioral oracles are unchanged; there is no inferred/fallback giver. This representation-only departure is committed separately from the v4 fixture/test/spec patches.

## Implementation inventory

- `src/c_api/quest_readiness.rs`: existing dialog phase/id plus staged supplied player/response/next/request fields; concrete giver remains in `GossipState`. Fields alone implement no behavior; Progress/Rewards represent native states 2/3, not numeric enum discriminants.
- `src/lua_api/sim_substates/quest_log.rs`: supplied choice records and simulator acceptance receipts; existing rich/light/history model remains authoritative.
- `src/lua_api/globals/quest_surface/legacy_globals.rs`: existing active selection and dialog reader entry points; native Wrath adaptation still required.
- `src/lua_api/globals/quest_surface/register.rs`: existing registration; Wrath verbs/readers still require adaptation.
- `src/lua_api/globals/admin_quests.rs`: existing selected-quest admin behavior; proposed Wrath offer/completion response policy still required. Preserve existing Retail/Forever policy.
- `tests/common/wrath_turn_in_consumer.rs`: original full-startup fixture, concrete inputs and producer-only notification drain; no callback/API replacement.

`Cargo.toml` explicitly registers `wrath_quest_turn_in` because automatic integration-test discovery is disabled. The v4 packet omitted this fixture prerequisite; the initial attempted RED build therefore did not execute a test. Standalone targets also expose unchanged Wrath abandon/log-read tests for bounded preservation. Fixture compilation uses the public `lua_api::state::QuestRewardItem` re-export and supported `i64` Lua counter conversion; v4's private-module import and unsupported `usize` conversion were compile errors, not behavioral RED.

## Tests asserting this spec

`tests/wrath_quest_turn_in.rs`: 30 authored cases, not 30 reached REDs. Restaging evidence artifact identifier: `w1-turnin-restage-12340-v4` (external, not a tracked report). Its report classifies 12 cases as step-A controls and 18 as step-B named assertion boundaries under Main's staged producer disposition; no step-A receipts have executed. `accepted_response_remains_answered_after_close` is an A post-completion negative acceptance control, not response-sent guard RED: the target row is already consumed. Pending-claim cases `answered_dialog_ignores_both_verbs_and_close_does_not_reset_latch` and `answered_guard_precedes_invalid_choice_after_successful_send` retain the related guard-boundary expectations. `tests/common/wrath_full_ui.rs` is reused unchanged from base 122f26445. Cached original QuestFrame/QuestInfo/QuestLogFrame Lua/XML hashes are provenance preconditions, not substitutes for behavioral assertions.

## Known gaps (current cycle)

- [ ] Step-A per-test boundaries, step-B GREEN and preservation remain pending. Shared RED is executed at `84f7067d6`, with startup `[]|{}` and unchanged original CVar stat/hash. Earlier target-registration and fixture compilation failures are not behavioral RED. Acceptance is 4/4 GREEN at `7ea9065be`, conditional on the documented finish-order inference.
- [ ] NaN, infinity, out-of-int32 values and subtraction overflow remain unverified; add no expected-result oracle. Successful-call result arity also remains unspecified. Extra-argument and non-numeric-string cases are source-grounded above but not separately tested in this packet.
- [ ] No-choice input 3 has a no-error test, but exact sent index 2 is not publicly observable through the current receipt/acceptance seam. Do not assert private request fields or infer a wire index from receipt absence. Wire/send-index observability remains a gap; zero-index input cases use the existing simulator-policy acceptance seam.
- [ ] Both response adapters and Wrath native verb/reader/event implementations are absent or incompatible at the base; staged fixture additions alone do not make tests runnable/GREEN.
- [ ] Outbound network packet capture/duplicate-send counts are not observed. Pending response acceptance is a simulator-policy request oracle, not proof of native server behavior.
- [ ] Native cache-hit/miss scheduling beyond the supplied completed-callback fixture, FINISHED versus UNIT ordering, unit-alias multiplicity and native packet acceptance/rejection remain unknown. Do not fill these gaps with a fixed total order or extra client guards.
- [ ] Inform GUID clearing has no distinct observable fixture here; only NPC GUID clearing is covered by suppression of later completion FINISHED. Player lifecycle outside supplied presence and the states other than 2/3 are not modeled exhaustively.
- [ ] Full rewards (inventory/money/XP), native catalogs, transport and exact post-mutation snapshot timing inside every notification remain unproved.

## Out of scope

Retail/Forever policy changes, vendor Lua/XML edits, native/decompiled source access, gameplay objective progression, real server emulation and full-profile compatibility. No existing success checkbox is changed by authored, unexecuted candidates.
