# Currency list field projection

Expose existing currency-list backing fields through `C_CurrencyInfo.GetCurrencyListInfo` and the Mists legacy tuple. Existing listed currencies project supplied `SimState.currency_info` display values; list structure remains static. Source lives in `src/c_api/item_spell/c_currency.rs` and `src/mists/compat_bootstrap.lua`.

## What it must do

- [ ] Return non-nil boolean `isShowInBackpack` from `CurrencyEntry.is_show_in_backpack`, preserving true and false.
- [ ] Return non-nil number `maxQuantity` from `CurrencyEntry.max_quantity`, preserving zero and positive caps.
- [ ] Forward those fields directly through Mists `GetCurrencyListInfo` tuple positions 5 and 8.
- [ ] Preserve all other fields, unsupported tuple slots, catalog data, defaults, and registrations.

### Supplied-state display projection

- [ ] For existing non-header list rows, project current name, quantity, icon, quality and maximum quantity from `SimState.currency_info` selected by the static currency ID.
- [ ] For existing watched rows, backpack info projects current name, quantity and icon from that same supplied row.
- [ ] Observe successive supplied-state updates without changing list size/order, header records, currency IDs, static status flags or watched membership.
- [ ] Do not resurrect static display values when a listed currency is removed from supplied state: list info returns nil and backpack info returns no values. Headers do not require a supplied map row.
- [ ] Preserve invalid-index behavior and existing Mists tuple mapping. This is simulator policy, not a claim about native update timing.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/globals/currency_data.rs` — existing `CurrencyEntry` fields and static list; unchanged.
- `src/c_api/item_spell/c_currency.rs` — namespaced list-info projection with capacity for nine fields.
- `src/mists/compat_bootstrap.lua` — existing Mists legacy tuple adapter.
- `tests/mists_currency_list.rs` — existing currency-list integration test file; owned by the separate fixture actor.

## Tests asserting this spec

Two new test names in `tests/mists_currency_list.rs` (GREEN unverified here):

- `currency_list_info_preserves_watched_flags_in_namespace_and_legacy_tuple`
- `currency_list_info_preserves_max_quantity_in_namespace_and_legacy_tuple`

Four supplied-state regressions in `tests/currency_info_probes.rs` (RED execution pending):

- `get_currency_list_info_reflects_successive_existing_currency_updates`
- `get_backpack_currency_info_reflects_successive_existing_currency_updates`
- `get_currency_list_info_returns_nil_when_existing_currency_row_is_removed`
- `get_backpack_currency_info_returns_nothing_when_existing_currency_row_is_removed`

## Known gaps (current cycle)

- [ ] Supplied-state projection requires authentic RED before producer changes, then targeted GREEN and preservation controls. Historical static-field proof does not cover mutation behavior.

## Out of scope

Native Mists catalog and comprehensive currency parity are not claimed. No catalog/default/registration changes, alternate paths, new fallbacks, vendor edits, or adjacent serializer refactors.
