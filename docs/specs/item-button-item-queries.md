# Item button item queries

The Retail and PTR ItemButton consumer uses supplied item observations for Azerite classification and reagent quality. See [Lua API architecture](../lua-api.md).

## What it must do

- [ ] `C_AzeriteEmpoweredItem.IsAzeriteEmpoweredItemByID` reads the existing supplied empowered-item ID set and returns a boolean.
- [ ] `C_TradeSkillUI.GetItemReagentQualityInfo` reads item-keyed supplied reagent quality and returns a detached `CraftingQualityInfo` snapshot or nil when no record is supplied.
- [ ] Both APIs authenticate secret selectors before parsing, and accept the existing item-ID and link parser forms.
- [ ] Publish these queries on Retail and PTR only in this lane. Pinned declarations are `AzeriteEmpoweredItemDocumentation.lua:172` in both profiles and `TradeSkillUIDocumentation.lua:364` (Retail) / `:379` (PTR).

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/c_api/c_azerite_empowered_item.rs`: supplied Azerite classification.
- `src/c_api/c_trade_skill_quality.rs`: supplied reagent records and quality snapshots.

## Tests asserting this spec

- `tests/test_keybindings.rs`: original ItemButton overlay consumers with distinct supplied items.

## Known gaps (current cycle)

- [ ] Retail and PTR GREEN, secret-input and profile coverage remain unverified.
- [ ] Check the next character-toggle failure after these queries work.

## Out of scope

- Native item classification and catalog inference; other profile publication; modifier-click dispatch and spell-target APIs.
