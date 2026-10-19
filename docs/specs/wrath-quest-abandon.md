# Wrath quest abandon confirmation

Wrath legacy selection/confirmation and reward presentation use the existing rich quest store and pending target. See [quest state](../wiki/systems/lua-api.md) and [Wrath log projection](wrath-quest-log-projection.md).

## What it must do

- [x] `SetAbandonQuest` captures the selected active quest ID without changing membership, auxiliary state, pending offers, history, economy or notifications. Empty/invalid public selection clears the target. `GetAbandonQuestName` uses that marked identity, independently of later selection.
- [x] Original no-items XML abandon button/popup and real cancel preserve state until confirmation. Simulator `AbandonQuest` stands in for the server response: resolve the retained ID in the current rich log, remove only matching rich/light membership and auxiliary state, clear matching pending offer/selection, preserve unrelated state and completion history. Confirmation and no-match calls retain the target ID. No target/match means no mutation or notification.
- [x] Provisional simulator notification policy: after removal commit, deliver direct `UNIT_QUEST_LOG_CHANGED("player")`, then queue an argumentless `QUEST_LOG_UPDATE`, coalescing with an already queued log update. Original listeners see committed membership; repeated confirmation without a matching row does nothing.
- [x] Original `QuestLog_SetSelection` runs to completion without destructive marking; visible original `QuestLog_OnEvent` refresh renders surviving supplied rows. Direct execution, installed error handler and simulator error collectors remain observable and clean.

### Legacy reads and reward presentation

- [x] `GetQuestLogGroupNum` returns exactly one number from the selected rich entry's `suggested_group_size`, zero for an absent field, missing row or unresolved selection. Original `QuestInfo_ShowGroupSize` renders supplied group text and hides it for a no-group row.
- [x] Wrath reward item count/info read the selected rich entry's existing `reward_items`; the legacy item reader does not require a second quest-ID argument. Spell, arena and talent readers project supplied rich reward data, with absent spell returning nil and absent points returning zero. Original full selection renders supplied items/spells/points and hides empty rewards.
- [x] `ProcessQuestLogRewardFactions` replaces one shared prepared faction-output list from the selected rich entry; no event. Count/info readers use that shared output without auto-refresh on selection. Dialog close preserves it. Count returns one number; out-of-range integral indices return `0, 0`.
- [x] `GetDailyQuestsCompleted` counts nonzero entries in the player's 25-slot daily store, never active/completed quest-log rows; empty store returns zero. `GetMaxDailyQuests` returns 25. Both return exactly one number. Original `QuestLog_UpdateQuestCount` displays supplied progress and hides zero progress. This field/binding addition is Wrath-only.
- [x] Wrath `GetQuestGreenRange` returns one number driven only by current engine player level; ignores arguments. Its client table has 20 level/5 entries, capped at index 19: levels 0–9 yield 4, 10–19 yield 5, 20–29 yield 6, 30–39 yield 7, 40+ yield 8. Original `GetQuestDifficultyColor` consumes it at green/gray boundaries; no `GetGrayLevel`, expansion input or level-cap input is added.

### Cross-profile preservation

- [x] Retail, PTR and Forever retain their existing destructive `SetAbandonQuest` and shared reward-read contracts.

## Consumer grounding and source limits

Tests pin whole original Wrath `FrameXML/QuestInfo.lua` to SHA-256 `3bfe691acdb2a095a6633c3ce7bef4a13f53f464d20781b2e850e442f9b715cd`. `QuestInfo_ShowRewards` at lines 269–281 tests `GetQuestLogRewardSpell()` for Lua truthiness and compares arena/talent rewards numerically at lines 297–299: numeric zero is Lua-truthy, so this model uses nil for an absent spell and zero for empty point amounts. Exact native empty-spell return arity is not inferred from that consumer. Lines 382–384 consume supplied spell texture/name/tradeskill/learned values. Lines 436–440 consume selected item reward info. The original faction-processing call is line 281 in this pinned file; `QuestInfo_DoReputations` at lines 142–153 reads prepared count/list and divides amounts by 100.

Source-derived 3.3.5a build 12340 findings support ID-based target capture, retained target on confirmation/no-match, numeric group reads and shared prepared faction output. These are decompile findings, not native captures. Startup faction count zero is a static-zero-initialization inference; the simulator initializes an empty list. Native `AbandonQuest` sends a remove-by-slot request, with removal arriving later from server updates. This serverless simulator combines request and response. Local commit/delivery timing, exact conditional event delivery and native total order remain provisional; original FrameXML consumption proves bounded UI compatibility, not native sequencing.

Original `FrameXML/QuestLogFrame.lua:515–525` calls `GetDailyQuestsCompleted` during visible log refresh and `GetMaxDailyQuests` only for positive progress. Build 12340 source findings specify a count of nonzero entries across 25 player daily fields, a fixed maximum of 25 and zero with no active player. Current game-mode simulator state always contains a player; absent-player lifecycle is not added or certified here. Daily reset timing, turn-in population and refresh events remain pending; none are invented by these reads.

The green-range table is from independently verified client12340 reconstruction revision `0e4e9fe0` (`QuestLog.cpp:69,111–125`, script return at `QuestLog_Scripts.cpp:87–90`), relayed by the parent; it is source-derived, not a native-client capture. The CMaNGOS server curve is not used. Tests pin whole `UIParent.lua` to `a48687367d05ba9965bdd7a43bdecb92666c8ba74873bd7d9da08b205b34e177` and `Constants.lua` to `367f5024098edda7d8dbcc9f57cf2616c85da4a3a9eb9851d2d4fc0bea8b72fd`. This snapshot's `UIParent.lua:3358–3370` calls `UnitLevel("player")` and `GetQuestGreenRange()` directly; `Constants.lua:403–409` supplies the concrete RGB outputs asserted at band edges. Source reports zero with no active player; that lifecycle remains excluded rather than inferred from `IsLoggedIn`.

The standalone original detail route (`QuestLog_SetSelection` while the main list is hidden) exposes the same real abandon button and popup without main-list difficulty coloring. Its cancel/confirm/retained-target path is separately asserted with a visible original control. This does not replace or disable the main `QuestLog_OnShow`/visible-refresh case.

Quest-dialog reward setup also rebuilds the shared faction output before detail/complete presentation in the source findings. That dialog producer belongs to the acceptance/turn-in slices and is not implemented or certified here. Header selection preserves a prior quest identity natively, but this lane models flat ordinary rich rows only; no header or absent-player/cache lifecycle parity is claimed.

## Implementation inventory

- `src/lua_api/globals/quest_verbs.rs` — Wrath marking, retained-target server-response simulation and notifications.
- `src/lua_api/globals/quest_surface/wrath_quest_log.rs` — selected legacy reads and explicit faction processing; Wrath-only registration in `register.rs`.
- `src/lua_api/sim_substates/quest_log.rs` — existing rich/light-related state, supplied reward data and shared prepared faction output; no parallel quest store.

## Tests asserting this spec

- `tests/wrath_quest_abandon.rs` — whole pinned original consumers/full startup, real XML button/popup, visible log refresh, noncatalog rows, supplied rewards, shared faction output and state/event controls.
- `tests/common/wrath_full_ui.rs` — original FrameXML TOC/startup addon ordering.
- `tests/quest_verbs.rs`, `tests/quest_abandon_membership.rs`, `tests/c_quest_log_rewards.rs` — affected legacy/shared preservation contracts.

## Proof status

Genuine original selection RED first removed supplied rich/light state before confirmation. Separate group-read RED reached `QuestInfo.lua:211`; implementing that read advanced full original selection to a missing reward reader at line 272. Further genuine RED through the original XML popup accept button cleared the marked ID (`None` instead of supplied quest 990101) after selection switched to another quest; the notification consumer also failed because no direct unit-change signal arrived. A supplied-item original selection reproduced the missing reward reader at line 272. After those repairs, 10/11 abandon cases passed; visible log refresh reached a separate genuine missing `GetDailyQuestsCompleted` read at `QuestLogFrame.lua:520`. After the daily repair, 133/134 selected cases passed but main-list coloring reached missing `GetQuestGreenRange` at `UIParent.lua:3367`. A separate original color-consumer RED reproduces that call at player level 9; the visible standalone detail/button/cancel/confirm route passes independently. Final bounded proof at rebased source `f1e2d22ce` (base `5a508d443`): Wrath **136/136 PASS** (14 abandon/standalone/counter/color cases plus 122 controls), actual stock `wow-sim --no-addons --no-saved-vars lua-errors` exits 0 with `[]` and zero unique/total errors. Current-source Wrath `cargo check --lib -j1` exits 0; formatting, explicit standalone formatting and manual readability checks pass. Compiler/Cargo/standalone warning counts are zero. Runtime copies preserve original CVar stat/hash.

Preservation: Retail default **126/126 PASS**; PTR **108/108 PASS**; Forever **121 PASS / 2 FAIL**, with checks passing in all three profiles. The two Forever `quest_readiness` cases (`ql2_original_gossip_refresh_and_click_use_live_readiness`, `ql2_original_progress_button_advances_once_without_grant_or_turn_in`) fail identically in an independently built master-only `5a508d443`: missing `C_QuestOffer` at original Mainline `QuestInfo.lua:655`, secondary missing `C_Log` at `ScriptErrors.lua:83`. They are pre-existing gaps, not this lane's regression; no shim or test disable was added. This is bounded quest-component preservation, not a claim that every profile/suite is clean. Rebase and later Wrath-only read additions leave non-Wrath quest code paths unchanged; no fresh whole-repository CI/native acceptance is inferred.

The delayed Wrath check already covered exact source `f1e2d22ce`; final integration reuses that successful proof instead of rerunning an unchanged broad check. Final receipt-only documentation changes do not invalidate identical source/test bytes. Native server timing, pixel rendering and whole W1 remain outside this proof.

## Out of scope

Item destruction, watch-index modeling, headers, absent-player/cache lifecycles, daily reset/population/refresh producers, acceptance, turn-in, reward acquisition, native event timing and whole W1/profile acceptance. Existing nil `GetAbandonQuestItems` only supports the no-items branch. No Blizzard/vendor code or callback replacement, added workarounds or shims.
