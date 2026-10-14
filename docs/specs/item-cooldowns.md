# Supplied item cooldowns

Item cooldown queries must project independently supplied start/duration pairs unchanged. Passive input lives in `src/lua_api/state/`; C API queries live in `src/c_api/item_spell/`. See the [API inventory](../wiki/systems/lua-api.md) for architecture. Producers and runtime acceptance remain pending.

## What it must do

- [ ] `C_Item.GetItemCooldown` resolves numeric ItemInfo and item hyperlinks to the same item ID and returns exactly `(number, number, boolean)`, with the supplied pair unchanged and enabled `true`.
- [ ] `C_Container.GetItemCooldown(itemID)` selects a numeric item ID and returns exactly `(number, number, number)`, with the supplied pair unchanged and enabled `1`; it does not select an equipment slot.
- [ ] `C_Container.GetContainerItemCooldown(bag, slot)` resolves the occupied item ID and returns its unchanged pair with numeric enabled `1`; this route is distinct from the single-item-ID query.
- [ ] Explicit simulator assumption, not native parity: missing item records and empty bag slots return three ready values `(0, 0, true)` for C_Item or `(0, 0, 1)` for Container. The Container item-ID declaration permits no returns; this model instead requires the ready tuple.
- [ ] Legacy `GetInventoryItemCooldown(unit, slot)` preserves independent equipment-slot pairs and three numeric returns, including `(0, 0, 1)` for a missing slot record. Item ID 13 must not alias equipment slot 13.

## How it works

- [API inventory](../wiki/systems/lua-api.md).

## Implementation inventory

- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs`: passive item-ID cooldown map and empty initializer, separate from equipment-slot input.
- `src/lua_api/game_data.rs`: existing `SpellCooldownState` start/duration pair.
- `src/c_api/item_spell/c_item.rs`, `src/c_api/item_spell/c_container.rs`: existing constant getters; producers not integrated.
- `src/lua_api/globals/cooldown_probes.rs`: unchanged legacy equipment-slot getter.

## Tests asserting this spec

`tests/item_cooldowns.rs`: four route tests, exact return counts/types/values, colliding item/slot IDs, and absent records. Inserts overwrite fixture defaults; removals establish missing records and an empty slot. Authored only; compile and execution pending.

## Known gaps (current cycle)

- [ ] Main commits passive input/tests/spec, then obtains actual compile and query-boundary RED before producer changes. A constant-stub mismatch is valid preproducer RED, not evidence of successful cooldown selection.
- [ ] Implement producers and obtain GREEN; acceptance remains unchecked.
- [ ] Native behavior, expiry/elapsed-time rules, enable derivation, secret validation, name selection, profile preservation, and wider consumer/runtime proof remain pending. These are not exclusions from the parent goal.

## Out of scope

No parent exclusions established by this passive-input stage.
