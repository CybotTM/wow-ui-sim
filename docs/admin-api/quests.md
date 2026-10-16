# Quests

`A_Admin` functions are simulator-only, not native WoW APIs.

## TurnInSelectedQuest

```lua
local changed = A_Admin.TurnInSelectedQuest()
```

Turn in the selected existing rich quest row only when complete and not failed. Returns `true` on commitment; `false` for absent/invalid selection, missing row, incomplete or failed quest. Select through existing `SelectActiveQuest`, `C_GossipInfo.SelectActiveQuest` or quest-log selection paths. Completion must already be supplied in the simulator model; this endpoint does not progress objectives.

Commit removes active rich/light membership and modeled Forever ordinary-watch metadata, records existing completion history, clears the consumed selection/matching pending offer and current matching gossip rows. Unrelated offers/membership/history remain. Original synchronous listeners then receive `QUEST_TURNED_IN(questID, 0, 0)`; zero XP/money is explicit simulator policy, not a native reward amount. No inventory/currency/reward grants occur. Repeating the call or reselecting the removed quest returns `false` without an event.

Task active/map/time projections suppress completion-history IDs; catalog metadata is not erased. [Owning contract and proof status](../specs/quest-turn-in-model.md) distinguish supplied-model fixtures from native behavior and current integration evidence.

Proof status: producer committed as `b5dd5947a`; GREEN has not been run. Tests were authored in `96ba1ba4d`; receipt snapshot `b7e380f2` is not the test-authoring commit or executed runtime source. Accepted shared Retail RED is 0 PASS/3 FAIL, exit 101, against retained pre-producer source `bbeac6213`; later assertions remain unreached. The fourth, Forever-only watch/event-order/reentry/repeat case was not executed. See the owning contract for receipt and accepted audit paths.
