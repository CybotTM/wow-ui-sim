# Era and Anniversary engraving availability

Era and Anniversary expose supplied engraving availability through `src/c_api/c_engraving.rs`, not an invented inactive default. See [the investigation](../wiki/investigations/era-engraving-availability.md) for original consumer selection and proof limits.

## What it must do

- [x] Publish `C_Engraving.IsEngravingEnabled` on Era and Anniversary through the shared C API registrar, selected through `client_profile::ACTIVE`.
- [x] Return one supplied environment-local boolean. Missing input raises `C_Engraving.IsEngravingEnabled: unsupplied input`; supplied false is not missing input.
- [x] Remove the temporary constant-false Lua default without seeding stock availability or inferring seasonal policy.
- [x] Drive the selected original `RuneFrameControlButton` XML-bound OnLoad with supplied true/false and unset, including transitions, recovery and independent environments. Both exact selectors on each profile reject the old constant-false implementation.
- [x] Attempt the complete original settings closure as preservation and report its actual boundary, without claiming full-settings behavior acceptance.

## How it works

- [Selection, state and bounded evidence](../wiki/investigations/era-engraving-availability.md)

## Implementation inventory

- `src/c_api/c_engraving.rs` — shared availability reader; remaining six members stay Mists-only by the registrar's second profile gate.
- `src/c_api/engraving_inputs.rs` — existing environment-local `enabled: Option<bool>` input.
- `src/lua_api/workarounds/temporary/classic_startup_defaults.rs` — no engraving policy remains.

## Tests asserting this spec

- `tests/era_engraving.rs` — original XML OnLoad, supplied-query arity, missing-input error and settings preservation attempt.

## Known gaps (current cycle)

No open implementation task remains in the approved availability-only slice. Both profiles' exact original selectors are RED101/GREEN0; settings false-input preservation completes with no collected closure diagnostics. Each stock map changes0/0→2/2, exposing the named unset error and its ScriptErrors echo; this is not clean startup. Both added raw keys are attributed to Blizzard_CharacterFrame and the removed constant-false shim is their masking predecessor. All seven full-feature all-target checks have zero JSON/plain compiler warnings/errors. Other-profile runtime parity is not established by compiler checks.

## Out of scope

Bags and full LoD EngravingUI are excluded. No native inactive default or complete engraving lifecycle is claimed. Mists' existing seven-member model is retained; no other profile gains publication. The other25 declared members remain outside Era/Anniversary's modeled surface:

| Domain | Excluded members |
|---|---|
| Filters | AddCategoryFilter, AddExclusiveCategoryFilter, ClearAllCategoryFilters, ClearCategoryFilter, ClearExclusiveCategoryFilter, EnableEquippedFilter, GetExclusiveCategoryFilter, HasCategoryFilter, IsEquippedFilterEnabled, SetSearchFilter |
| Catalog | GetNumRunesKnown, GetRuneCategories, GetRunesForCategory, IsKnownRuneSpell, IsRuneEquipped |
| Equipment | GetRuneForEquipmentSlot, IsEquipmentSlotEngravable |
| Inventory | GetRuneForInventorySlot, IsInventorySlotEngravable, IsInventorySlotEngravableByCurrentRuneCast |
| Casting/mode/refresh | CastRune, GetCurrentRuneCast, GetEngravingModeEnabled, RefreshRunesList, SetEngravingModeEnabled |

## Pinned declaration

Both Era and Anniversary `Blizzard_APIDocumentationGenerated/EngravingInfoDocumentation.lua:179–186` independently declare the non-null bool query. Both select `Blizzard_CharacterFrame/Vanilla/PaperDollFrame.xml:716,722` and `PaperDollFrame.lua:950–954`; settings reads availability at `Classic/InterfaceOverrides.lua:136–138`.
