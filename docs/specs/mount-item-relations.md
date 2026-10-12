# Mount item relations

Current Retail/PTR `C_MountJournal.GetMountFromItem(itemID)` queries an explicit simulator-owned item-to-mount relation. Cached API declarations establish one required numeric input, a nullable numeric mount ID return, and `AllowedWhenUntainted` secret arguments. Catalog contents, ownership filtering and dressability rules are not established by those declarations.

## What it must do

- [ ] An unseeded environment returns exactly one nil for an unmapped numeric item ID. No normal-session relation seeding or inferred spell/name/link mapping.
- [ ] A supplied mapping returns its exact numeric mount ID without requiring an ordinary journal record or ownership. Replacements/removals are live; separate environments remain independent.
- [ ] Missing/nil and nonnumeric inputs fail the required-number contract. Item links and numeric strings are not this API's numeric argument. Simulator policy reuses existing supplied-relation identities: finite integral signed32-bit values may match, while fractional/nonfinite/out-of-range numbers miss without truncation or saturation. This is not a native range claim.
- [ ] Authenticate the original VM secret argument before type validation or lookup. Secure callers can query wrapped numeric inputs; tainted callers cannot inspect wrapped inputs, including malformed values. Ordinary numbers remain usable from tainted callers. Preserve wrapper identity and caller taint.
- [ ] Preserve existing mount spell lookup and earlier-profile surfaces; do not transfer `GetMountFromSpell`'s separate `SpellIdentifier`/`AllowedWhenTainted` contract.

## How it works

- [Lua/C API surface](../lua-api.md) — existing runtime integration context; relation query implementation pending.

## Implementation inventory

- `src/lua_api/state/sim_state.rs` — explicit environment-owned `mount_item_relations` field.
- `src/lua_api/state.rs` — empty default; no normal catalog population.
- `tests/mount_from_item.rs` — initial/expanded direct queries and mapped-secret recovery coverage.
- `tests/mount_item_player_choice.rs` — original whole-template baseline and live relation contrast; captures actual supplier/result console observations without guessing classification.
- `src/c_api/c_mount_item.rs` — current-profile secret-authenticated supplied-relation query.
- `src/c_api/mod.rs` and `src/c_api/registration.rs` — Retail/PTR module and namespace wiring.

## Tests asserting this spec

`tests/mount_from_item.rs` has three initial empty-result, required-number and authentic secret/taint cases, four supplied-state/no-catalog, replacement/removal, environment-isolation and numeric-non-aliasing cases, and one mapped-secret hit/denial/recovery control. The additional mapped-secret case has no separate prior RED credit. Initial `0808b3cd0` execution independently verified0PASS/3FAIL at missing-API guards; downstream assertions were not reached. Empty field/default scaffolding follows that genuine RED. Expanded `3cc6d36af` execution independently verified0PASS/7FAIL at missing-API guards after fixture setup. Query producer follows that actual RED; GREEN remains pending.

## Known gaps (current cycle)

- [ ] Current Retail/PTR query runtime/startup verification and evidence retention.
- [ ] Actual supplier observations and unsliced PlayerChoice live-mapping contrast; baseline alone proves no mount branch.

Unchanged `PlayerChoiceBaseOptionItemRewardMixin:IsDressupReward` checks `C_Item.IsDressableItemByID(itemButton.itemLink)` first. Only false reaches `GetMountFromItem(itemRewardInfo.itemId)`; a nonnil mount result returns before the pet branch. The observed Retail whole-template baseline passes, contrary to the anticipated missing-predicate failure; it does not identify the actual dressability provider or branch. Required `ItemInfo` and nonnil boolean declarations do not classify items. Whole-template proof must use the actual vendor method and ItemButton input, not mocked getters, sliced handlers or an inventory-type guess. Direct query proof does not establish this consumer. Any dressability backing needs its own modeled contract and genuine failure evidence; fixtures do not authorize normal-session seeding.

## Out of scope

Native catalog/range/classification rules, ownership inference and normal relation population: absent source authority or user seeding authorization. Native/external/full-handoff acceptance remains separate and open; this query slice cannot close the parent goal.
