# Supplied item cooldowns

Item cooldown queries must project independently supplied start/duration pairs unchanged. Passive input lives in `src/lua_api/state/`; C API queries live in `src/c_api/item_spell/`. See the [API inventory](../wiki/systems/lua-api.md) for architecture. Bounded query producers are source-only; GREEN and profile acceptance remain pending.

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
- `src/c_api/item_spell/helpers.rs`: pure item-map pair projection; absent input yields `(0, 0)` by explicit model policy, not fallback compatibility or native parity.
- `src/c_api/item_spell/c_item.rs`: existing ItemInfo parser selects the passive item-map pair; boolean enabled `true`.
- `src/c_api/item_spell/c_container.rs`: separate numeric item-ID and bag/slot selectors project the same item map; numeric enabled `1`. All three query producers are source-only, not GREEN proof.
- `src/lua_api/globals/cooldown_probes.rs`: unchanged legacy equipment-slot getter.

## Tests asserting this spec

`tests/item_cooldowns.rs`: four route tests, exact return counts/types/values, colliding item/slot IDs, and absent records. Inserts overwrite fixture defaults; removals establish missing records and an empty slot. Genuine preproducer Retail execution at `e4caa4899`: four tests, legacy 1 PASS and query 3 FAIL, exit 101. All three queries fail their first `cooldown start value` assertion. C_Item numeric/hyperlink selector controls and Container bag controls were reached; subsequent hyperlink/collision/missing tuples were not reached. Postproducer GREEN remains pending.

## Known gaps (current cycle)

- [x] Passive input/tests/spec committed as `e4caa4899`; independent full-stream gate confirms genuine Retail RED (legacy 1 PASS, query 3 FAIL, exit 101). First start-value assertions fail; exact actual starts were not printed. Later cooldown tuples remain unreached, not successful selection proof.
- [ ] Bounded producers implemented source-only: unchanged supplied pairs, distinct selectors and namespace-specific enable types, short borrows ending before Lua pushes. Obtain actual GREEN and profile proof; acceptance remains unchecked.
- [ ] Native behavior, expiry/elapsed-time rules, enable derivation, secret validation, name selection, profile preservation, and wider consumer/runtime proof remain pending. These are not exclusions from the parent goal.

## Out of scope

No parent exclusions established by this passive-input stage.
