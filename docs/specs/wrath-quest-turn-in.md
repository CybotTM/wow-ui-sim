# Wrath 12340 quest turn-in

Wrath 3.3.5a (build 12340) dialog requests consumed by the unchanged cached `FrameXML/QuestFrame.lua` and XML. Backing state lives in `src/c_api/quest_readiness.rs` and `src/lua_api/sim_substates/quest_log.rs`; native readers/request registration and simulator response adapters are implemented with the bounded staged A/B proof below. See [quest implementation notes](../wiki/systems/lua-api.md#simulator-selected-quest-turn-in).

Binding evidence artifact identifiers: `laptop-labels-answers-20261018:turnin-guards-12340` (Q-T1–Q-T3), `laptop-labels-answers-20261018:getquestreward-choice-12340` (choice conversion/error details), `laptop-labels-answers-20261018:quest-finished-reentry-12340` (authenticated cleanup/reentry), and `laptop-labels-answers-20261018:quest-choose-reward-error-12340` (native feedback), laptop answers dated 2026-10-19. These identifiers are not repository files or filesystem paths. This packet uses that answer, not decompiled source or a native capture. Requirement checkboxes record implemented bounded behavior, not unrestricted native parity; proof limits remain explicit below. The integrated fixture compiled and reached shared feature-absence RED at `84f7067d6`: original progress UI calls missing `IsQuestCompletable`; no distinct guard/conversion/event RED is credited from that setup failure.

## What it must do

### Native request contract

- [x] `CompleteQuest` sends only in dialog state 2 (Progress); `GetQuestReward` sends only in state 3 (Rewards). Both require an active player and an unsent response. Wrong state, absent player or prior response is ignored with zero Lua results and no immediate or queued events. Guard rejection must precede reward-choice validation.
- [x] Eligible invalid choices raise an error ending in the exact literal body `Invalid reward choice in GetQuestReward([choice])` without consuming the response opportunity. `[choice]` is literal, not substituted. `luaL_error` level 1 may prepend `<short_src>:<line>: ` when the caller has a line; do not require a bare whole message at an uncontrolled call site. Sending alone neither removes the quest nor records acceptance.
- [x] Convert the argument to int32 by truncation toward zero, then subtract one. Omitted/nil/non-numeric input becomes internal -1 without a type error; numeric strings convert and extra arguments are ignored. With N>0, accept internal 0 through N-1: omitted/nil/0/-1/N+1 error; 1.5 and "1" choose the first reward; N+0.5 chooses the last. Successful calls set responseSent; each test input starts a fresh eligible dialog. Guard-rejected calls return before validation, with no error or send.
- [x] With N=0 skip bounds validation for ordinary inputs: internal<=0 sends index 0, positive internal values are preserved (3 sends index 2). Sending is not server acceptance. Do not invent successful-call result arity from the ignored-call zero-result rule.
- [x] Requests use the current dialog quest, never the selected quest-log row. Neither missing NPC/zero giver nor incomplete/failed rich rows is an additional native send guard. UI-disabled-button suppression and server acceptance are separate from native sending.
- [x] `CloseQuest` and QuestGiverFinished do nothing when the NPC GUID is zero, including retaining any pending detail snapshot and other identities. With nonzero NPC GUID, clear NPC/inform GUIDs, then the quest interact target, then dispatch argumentless `QUEST_FINISHED` synchronously. Retain dialog state/current quest/response flag and pending requests. An unanswered closed dialog still sends; a sent dialog stays blocked after close. A new original active-title selection resets the response latch through SetState. Original FINISHED → Hide → OnHide → CloseQuest produces exactly one FINISHED because reentry sees NPC zero. Arbitrary callbacks that open a new dialog are not protected by a generation guard.
- [ ] `QuestChooseRewardError` is a real native global: ignore all arguments and return zero values, without quest/player/response gating. Request named sound kit `igQuestFailed` through the existing sound request model, format/read global `ERR_QUEST_MUST_CHOOSE`, record the last message before synchronous listeners, and emit `UI_ERROR_MESSAGE(message)` with exactly one string argument only when nonempty. Original Wrath UIErrorsFrame reads arg1, not a messageType/message pair. Empty text is recorded but not emitted. No quest mutation is performed by the producer; synchronous listeners can mutate state. Numeric sound ID/audio playback and malformed/missing localization behavior are not authenticated here.

### Completion notifications

- [x] At the completed quest-cache callback boundary, completion emits `QUEST_FINISHED()` only when next quest ID is zero and giver GUID is nonzero. Concrete GUID zero must not fall back to an interacting NPC. A closed dialog's giver must not be resurrected by offer/claim. Native cache hits may complete inline and misses defer; acceptance plus a generic event-queue drain does not establish callback completion.
- [x] `UNIT_QUEST_LOG_CHANGED("player")` is immediate, one token per emission; additional aliases are permitted. `QUEST_LOG_UPDATE()` is deferred and coalesced. Tests constrain only UNIT before the deferred LOG_UPDATE flush, never FINISHED versus UNIT.
- [x] No turn-in path emits `QUEST_TURNED_IN`: it does not exist in build 12340.

### Explicit simulator policy, not native server acceptance

- [x] `A_Admin.OfferQuestReward()` consumes a pending native presentation request and supplies the server offer, publishing `QUEST_COMPLETE` for the original reward UI. The test must not fabricate that event or replace a native verb/callback.
- [x] Wrath `A_Admin.TurnInSelectedQuest()` supplies acceptance for a pending dialog claim, regardless of unrelated log selection; no pending claim returns false with no mutation/events. Completed/nonfailed supplied rows are accepted in these fixtures. Actual server acceptance of arbitrary/stale/closed/incomplete/failed requests is unknown. Fixtures explicitly supply `completion_callback_cache_hit = true`: this response policy completes the quest-cache callback inline within acceptance. FINISHED counts are checked only for that supplied completed-callback boundary, not as a native timing guarantee. False leaves callback scheduling unspecified; no deferred-cache implementation is demanded by these cases.
- [x] Accepted target 990301 leaves active log/history coherent while complete control 991300 and historical ID 998888 survive. Supplied choice receipts are recorded only at acceptance and preserve unrelated receipt `(998888, 1)`; no-choice index zero adds no choice receipt. These receipts are not inventory grants.
- [x] Two accepted pending claims in one notification pass each increase the observed player-token emission count before any drain and produce one later argumentless log update. Open the second dialog through original active-title selection, never by resetting the latch in the fixture. Original QuestLogFrame renders the final count. Callback repetition cannot create a second accepted claim without a new dialog.
- [x] The original UI's no-choice button path accepts without a choice receipt under supplied simulator policy. This does not establish native omitted/nil/zero normalization or successful-call result arity.

The original happy-path receipt-preservation control now explicitly supplies unrelated receipt `(998888, 1)` before any request. A2 reached its final unchanged oracle and exposed that missing input (`{}` versus `{998888: 1}`); the correction is fixture-only. Empty-receipt no-choice controls retain their original empty input/oracle.

## Staged producer proof

Step A checkpoint implemented current-dialog selection/readers, response-latch reset on new dialog, ordinary-integer claims and pending-request-consuming simulator offer/acceptance. Wrath log selection no longer erases the dialog target. It deliberately does not implement native state/player/sent guards, reward conversion/validation, CloseQuest retention, completion FINISHED, or turn-in notifications. Basic shared CloseQuest/acceptance finish plumbing was brought forward to unblock acceptance under Main's explicit disposition. Its clear-before-emit order was **INFERRED at W1**, then authenticated by the 2026-10-19 laptop reentry answer; zero-giver suppression prevents original OnHide re-entry from publishing a second finish. The first full A run at `f321476f4` reached all 30 cases but each stopped at the shared missing legacy `IsQuestFlaggedCompleted` reader in membership/history checks. Those are not R3 boundary receipts. The legacy reader now projects the same existing completion-ID set as `C_QuestLog`; no assertion is weakened. Corrected A3 receipts at `b6374eded` satisfy the gate: 12/12 A controls pass and 18/18 B cases fail at their own post-stimulus guard/conversion/retention/finish/event boundary. The reclassified post-completion acceptance control passes, without being credited as sent-guard proof. Direct conversion cases fail at their supplied fractional/string input, not a common prerequisite; post-Close failures occur only after that specific stimulus. No oracle was weakened.

Step B now implements phase/player/sent guards before argument handling, finite ordinary int32-domain truncation and numeric strings, the exact literal invalid-choice error, CloseQuest retention, conditional supplied-cache-hit finish and shared abandon-notifier delivery (immediate player UNIT, deferred coalesced LOG_UPDATE). The inline completion callback and mirror notifier remain distinct publisher paths; their invocation in the simulator is not a native FINISHED-versus-UNIT ordering contract. Initial B runtime at `b01a3725f` passes 26/30 turn-in cases, all three Wrath preservation targets and standalone startup error checks. Four remaining failures are the observer's rendered-count assertion after accepting from progress-only fixtures: unchanged `FrameXML/QuestLogFrame.lua:327–329` returns immediately while the log is hidden. The fixture now shows the original log consumer after opening progress; every count/title, timing, event and error oracle remains unchanged. No vendor/producer behavior is patched to refresh hidden UI. Full turn-in GREEN is re-proved after this fixture-only correction: 30/30 at `6431fdc35d125e6978d591589e75033d6ce62f53`.

## How it works

- [Quest model notes](../wiki/systems/lua-api.md#simulator-selected-quest-turn-in)
- [Event delivery](../event-system.md)
- [Existing authored admin scope](quest-turn-in-model.md)

## Fixture integration amendment

Main approved using the existing `GossipState.giver_guid` as the single concrete giver input instead of v4's additional `QuestReadinessState.giver_guid`. All supplied GUID values and behavioral oracles are unchanged; there is no inferred/fallback giver. This representation-only departure is committed separately from the v4 fixture/test/spec patches.

## Implementation inventory

- `src/c_api/quest_readiness.rs`: existing dialog phase/id plus staged supplied player/response/next/request fields; concrete giver remains in `GossipState`. Fields alone implement no behavior; Progress/Rewards represent native states 2/3, not numeric enum discriminants.
- `src/lua_api/sim_substates/quest_log.rs`: supplied choice records and simulator acceptance receipts; existing rich/light/history model remains authoritative.
- `src/lua_api/globals/real/wrath_quest_dialog.rs`: native current-dialog readers, guarded requests and explicit simulator offer/acceptance responses.
- `src/lua_api/globals/quest_verbs.rs`: retained CloseQuest state, shared giver-clearing/finish path and immediate/deferred log notifier.
- `src/lua_api/globals/admin_quests.rs`: Wrath response registration remains separate from unchanged Retail/Forever selected-quest policy.
- `tests/common/wrath_turn_in_consumer.rs`: original full-startup fixture, concrete inputs and producer-only notification drain; no callback/API replacement.

`Cargo.toml` explicitly registers `wrath_quest_turn_in` because automatic integration-test discovery is disabled. The v4 packet omitted this fixture prerequisite; the initial attempted RED build therefore did not execute a test. Standalone targets also expose unchanged Wrath abandon/log-read tests for bounded preservation. Fixture compilation uses the public `lua_api::state::QuestRewardItem` re-export and supported `i64` Lua counter conversion; v4's private-module import and unsupported `usize` conversion were compile errors, not behavioral RED.

## Tests asserting this spec

`tests/wrath_quest_turn_in.rs`: 30 behavioral cases; shared feature-absence RED is not credited as 30 distinct boundaries. Restaging evidence artifact identifier: `w1-turnin-restage-12340-v4` (external, not a tracked report). Its report classifies 12 cases as step-A controls and 18 as step-B named assertion boundaries under Main's staged producer disposition; A3 executed the full inventory at `b6374eded` with 12 A pass / 18 B boundary fail. `accepted_response_remains_answered_after_close` is an A post-completion negative acceptance control, not response-sent guard RED: the target row is already consumed. Pending-claim cases `answered_dialog_ignores_both_verbs_and_close_does_not_reset_latch` and `answered_guard_precedes_invalid_choice_after_successful_send` retain the related guard-boundary expectations. `tests/common/wrath_full_ui.rs` is reused unchanged from base 122f26445. Cached original QuestFrame/QuestInfo/QuestLogFrame Lua/XML hashes are provenance preconditions, not substitutes for behavioral assertions.

## Bounded proof — 2026-10-19

External evidence artifact: `w1-accept-turnin` (handoff, not a repository path).

| Scope | Result | Source revision / receipt |
|---|---|---|
| Shared feature-absence RED | Original progress reaches missing IsQuestCompletable | `84f7067d6`, `turnin-red3-00-shared-red-receipt.json` |
| A staged boundary gate | 12 A PASS / 18 B boundary FAIL; reclassified control PASS | `b6374eded`, `turnin-a3-runtime-summary.json`, `turnin-a3-boundaries.json` |
| Same complete turn-in inventory | 30/30 PASS, each named test executed; startup `[]` | `6431fdc35`, `turnin-green2-runtime-summary.json`, `turnin-green2-proof.json` |
| Wrath preservation | Accept 4/4, abandon 14/14, log reads 6/6; standalone startup `[]` | `b01a3725f`, `turnin-green-runtime-summary.json`; later changes are turn-in fixture-only |
| Final format/type gate | cargo fmt --check PASS; Wrath cargo check PASS, zero warnings | `6431fdc35`, `final-fmt-receipt.json`, `final-check-build-receipt.json` |

Build/check commands use `--no-default-features --features sound,gui,casc,client-wrath -j1`. Exact commands, artifact hashes, bounded systemd properties and byte-copy child-XDG CVar stat/hash receipts remain in the evidence artifact. Every runtime leaves the original CVar unchanged. No native capture, complete server/reward emulation, all-profile or independent integration acceptance is claimed.

## Known gaps (current cycle)

- [ ] W1b feedback implementation and current RED/GREEN proof are pending. W1's no-op shim is superseded by the authenticated native contract above; historical W1 receipts do not establish feedback behavior.
- [x] Clear-before-emit and synchronous zero-NPC reentry suppression are authenticated by laptop source reading, not native runtime capture. Earlier target-registration/fixture compilation failures remain non-RED; only executed receipts are credited.
- [ ] NaN, infinity, out-of-int32 values and subtraction overflow remain unverified; add no expected-result oracle. Successful-call result arity also remains unspecified. Extra-argument and non-numeric-string cases are source-grounded above but not separately tested in this packet.
- [ ] No-choice input 3 has a no-error test, but exact sent index 2 is not publicly observable through the current receipt/acceptance seam. Do not assert private request fields or infer a wire index from receipt absence. Wire/send-index observability remains a gap; zero-index input cases use the existing simulator-policy acceptance seam.
- [ ] Player presence beyond the explicitly supplied dialog input is not a complete player lifecycle model.
- [ ] Outbound network packet capture/duplicate-send counts are not observed. Pending response acceptance is a simulator-policy request oracle, not proof of native server behavior.
- [ ] Native cache-hit/miss scheduling beyond the supplied completed-callback fixture, FINISHED versus UNIT ordering, unit-alias multiplicity and native packet acceptance/rejection remain unknown. Do not fill these gaps with a fixed total order or extra client guards.
- [ ] Inform/interact identities are explicit supplied inputs, not acquired NPC lifecycle or native GUID encoding. Player lifecycle outside supplied presence and the states other than 2/3 are not modeled exhaustively.
- [ ] Full rewards (inventory/money/XP), native catalogs, transport and exact post-mutation snapshot timing inside every notification remain unproved.

## Out of scope

Retail/Forever policy changes, vendor Lua/XML edits, native/decompiled source access, gameplay objective progression, real server emulation and full-profile compatibility. The bounded GREEN above does not close the native/model gaps listed here.
