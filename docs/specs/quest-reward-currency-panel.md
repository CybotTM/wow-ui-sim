# Shared quest reward currency projection into the original panel

## Contract

Retail/Forever `C_QuestInfoSystem.GetQuestRewardCurrencies(questID?)` reads the
existing selected/rich quest's `currency_rewards`, using the exact same serializer
as `C_QuestLog.GetQuestRewardCurrencies(questID)`. No parallel store, invented rows,
or reward acquisition is added. The existing known fields and optional supplied
base amount remain unchanged; quality/bonus/context metadata not represented by
that record are not synthesized.

Cached Retail `Blizzard_APIDocumentationGenerated/QuestInfoSystemDocumentation.lua:58-72`
declares AllowedWhenUntainted, nullable questID and MayReturnNothing.
`QuestLogDocumentation.lua:493-506` instead requires questID. The new wrapper
unwraps/authenticates the original argument before validation, and permits nil.
Authored context policy: nil uses `selected_quest_log_id`; absent/unrepresentable
context yields no values. A representable explicit ID uses the existing list
projection, including its empty-list miss behavior. Native context parity is not
claimed.

## Original consumer and implementation

`Blizzard_UIPanels_Game/Mainline/QuestInfo.lua:608,963-975` reads this namespace and
creates actual reward currency rows. `c_api/quest_reward_currencies.rs` now owns the
shared projection (the existing favor module is Retail-epoch-only); the pre-existing C_QuestLog handler delegates to it. Publication
uses QL-1's Retail/Forever gate.

`tests/quest_reward_currency_panel.rs` invokes original `QuestInfo_ShowRewards` with
explicit existing reward-frame context. Empty-state control checks a hidden rewards
frame. A supplied1166/5 record must produce the original Name/Count and currencyInfo,
then both namespace outputs and nullable selection behavior are checked.

Test `6d8417000` at saved `9d1430bf3` produced genuine Retail RED: original panel
had no supplied currency row (None versus supplied name), while the empty-panel
control passed. Later row/count/context assertions were unreached. GREEN and separate
Retail startup remain pending. Automatic panel scheduling, native/schema-complete
reward metadata, reward spells and grants are excluded.

See [QL-2](quest-readiness-dialog.md) and [temporary requirements inputs](quest-progress-requirements-empty.md).
