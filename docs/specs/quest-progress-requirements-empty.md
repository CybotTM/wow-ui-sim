# Temporary empty quest progress-dialogue / requirements inputs

## Bounded contract

Retail/Forever original Mainline QuestFrame calls five unmodeled native globals:
`GetProgressText` returns `""`; `GetQuestMoneyToGet`, `GetNumQuestItems`,
`GetNumQuestCurrencies` each return numeric0; `GetQuestPortraitTurnIn` returns
`0, "", ""` (no display ID, empty portrait text/name). No required rows, costs, dialogue or portrait IDs are invented. These are
explicit temporary compatibility gaps, not supplied-state implementations.

No NPC progress-dialogue, required-item/currency/cost, or turn-in portrait-input
model exists in the inspected quest state. Existing `reward_items` and
`currency_rewards` remain authoritative for rewards; criteria-spell metadata is
not a reward-spell or portrait model. No parallel store is introduced.

Implementation: `workarounds/temporary/quest_progress_requirements.rs`, registered
only under QL-1's Retail/Forever gate. Retirement condition: supplied quest-progress-
requirements/dialogue/portrait-input model replaces these typed empties.

## Original consumers and proof

Cached Retail `Blizzard_UIPanels_Game/Mainline/QuestFrame.lua:205,218-225,129`
calls progress text, requirement counts/cost and turn-in portrait respectively.
`tests/quest_dialog_requirements.rs` invokes those original handlers and asserts
empty text, hidden requirement rows/cost, an empty reward panel and a hidden,
detached portrait frame with the exact empty portrait tuple, with no new swallowed Lua errors. Explicit fixture mapView=false
establishes the first-dialog presentation state; automatic startup is not claimed.

Test commit `7c7d9d8ff` at saved producer `9d1430bf3` reached three actual nil-call
REDs: progress text line205, required-item count line218, portrait line129. Required-
currency and money calls occur later in the blocked same original refresh; their
individual nil-call boundaries were not reached and are not claimed as distinct
REDs. At saved `24a076873`, text and requirement-refresh cases passed; the first
no-values portrait draft instead reached original `QuestFrame.lua:523`'s nil-number
comparison. The original `:129-136` tests displayID against0 and calls its hide branch
at`:535-540`; text/name are consumed at`:459,499-504`. The approved correction uses
numeric0 plus empty strings, never a portrait ID. Revised test `4f601b63e` reproduced
the same original nil-comparison RED before the temporary-producer fix. Final
no-portrait branch GREEN remains pending.

## Inventory exclusions

`GetNumRewardSpells`, `GetRewardSpell`, `GetQuestCurrencyInfo` were absent in the
same closure but are not called by these Mainline consumers; they are explicitly
excluded, with no new registration. Reward-spell data remains unmodeled. Original
reward currency rows use [the separate supplied currency projection](quest-reward-currency-panel.md).
See [QL-2](quest-readiness-dialog.md) for readiness/dialog scope.
