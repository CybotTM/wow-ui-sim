# Mount item relations

Current Retail/PTR `C_MountJournal.GetMountFromItem(itemID)` queries an explicit simulator-owned item-to-mount relation. Cached API declarations establish one required numeric input, a nullable numeric mount ID return, and `AllowedWhenUntainted` secret arguments. They do not establish catalog contents, ownership filtering or dressability rules.

## Required behavior

- [ ] An unseeded environment returns exactly one nil for an unmapped numeric item ID. No normal-session catalog seeding or inferred spell/name/link relation.
- [ ] A supplied mapping returns its exact numeric mount ID, without requiring a matching ordinary mount-journal record or ownership. Updates and separate environments remain independent.
- [ ] Missing/nil and nonnumeric inputs fail the required-number contract. Item links and numeric strings are not this API's numeric argument. Numeric identity representation follows the existing supplied-relation policy; no guessed native range rule.
- [ ] Authenticate the original VM secret argument before type validation or lookup. Secure callers can query wrapped numeric inputs; tainted callers cannot inspect wrapped inputs, including malformed values. Ordinary numbers remain usable from tainted callers. Preserve wrapper identity and caller taint.
- [ ] Preserve existing mount spell lookup and earlier-profile surfaces; do not transfer `GetMountFromSpell`'s separate `SpellIdentifier`/`AllowedWhenTainted` contract.

## Tests

`tests/mount_from_item.rs` contains initial pre-producer empty-result, required-number and authentic secret/taint controls. No execution proof yet. Positive supplied-state/update/isolation coverage requires a real backing model and an expanded pre-producer RED stage; neither is implemented yet.

## Real consumer boundary

Unchanged `PlayerChoiceBaseOptionItemRewardMixin:IsDressupReward` checks `C_Item.IsDressableItemByID(itemButton.itemLink)` first. Only a false result reaches `GetMountFromItem(itemRewardInfo.itemId)`; a nonnil mount result returns before the pet branch. The real dressability API is currently missing. Its `ItemInfo` input and nonnil boolean return do not define native item classifications.

Whole-template/caller proof must use the actual vendor method and ItemButton input, not mocked getters, sliced handlers or an inventory-type guess. Initial query tests do not prove this consumer. Any dressability backing behavior needs its own modeled contract and genuine failure evidence; no normal-session seeding authorization follows from a fixture.

## Evidence gaps

- Initial/expanded RED, producer, current runtime/profile/startup verification and retention pending.
- Real dressability prerequisite and unsliced PlayerChoice mount branch unproved.
- Native catalog/parity and full handoff acceptance remain separate and open.
