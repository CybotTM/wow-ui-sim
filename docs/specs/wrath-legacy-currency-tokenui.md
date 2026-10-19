# Wrath legacy currency and TokenUI

Wrath 12340 legacy globals expose explicitly supplied one-level categories through mutable displayed indices and independent backpack watches. Laptop batch 5 (2026-10-18) supplies the contract; original Blizzard_TokenUI and HybridScrollFrame Lua drive integration tests. Modern C_CurrencyInfo state remains independent.

## What it must do

- [ ] GetCurrencyListSize counts current display rows: each header plus its children only when expanded.
- [ ] GetCurrencyListInfo uses one-based current displayed indices and returns nine positional fields: name, header, expanded, unused, watched, count, extra type, icon, item ID.
- [ ] ExpandCurrencyList accepts numeric 0/1 and returns no values; subsequent display indices reflect collapse/expansion immediately.
- [ ] SetCurrencyBackpack accepts numeric 0/1 at the current display index and returns no values. Original UI enforces its three-watch cap.
- [ ] GetBackpackCurrencyInfo returns five positional fields for watched slots 1..3 independent of category expansion: name, count, extra type, icon, item ID.
- [ ] Original header clicks and modified watch clicks refresh real widget state. Zero and large counts retain supplied values; original backpack rendering formats them.
- [ ] No Wrath legacy mutation changes modern currency queries/state or other profiles.

## How it works

- [Lua API](../lua-api.md)
- [UI loading](../addon-loading-pipeline.md)

## Implementation inventory

- `src/lua_api/globals/real/wrath_currency.rs` — supplied legacy category model and global API.
- `src/lua_api/state.rs` and `src/lua_api/state/sim_state.rs` — per-environment state.
- `src/c_api/item_spell/c_currency.rs` — registration boundary, without delegating modern queries to legacy state.

## Tests asserting this spec

- `tests/wrath_currency_tokenui.rs` — original complete Lua consumers with byte hashes and concrete supplied categories.
- `tests/fixtures/wrath_currency_tokenui.lua` — widget/localization fixture; no currency API replacements.

## Known gaps (current cycle)

- [ ] SetCurrencyUnused regrouping/order/expansion is unresolved. No XML click handler depending on it is claimed covered.
- [ ] Empty state is intentional: no native Wrath currency catalog is invented. Supplied test item IDs/counts/icons are inputs, not claimed native records.

## Out of scope

Mists equivalence, modern depth-aware list changes, native full-startup compatibility and server currency acquisition.
