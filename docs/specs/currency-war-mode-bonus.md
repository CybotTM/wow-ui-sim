# Currency War Mode bonus flags

`C_CurrencyInfo.DoesWarModeBonusApply` queries explicitly supplied per-currency flags. Current cached Retail/PTR `CurrencyInfoDocumentation.lua` declares required numeric `currencyID`, two nullable booleans (`warModeApplies`, `limitOncePerTooltip`) and `AllowedWhenUntainted`. `QuestUtils` consumes both flags separately; its additional PvP/quest/faction gates are not this query's model.

## What it must do

- [ ] Return exactly two values, preserving both independently nullable boolean inputs for the requested modeled currency.
- [ ] Return `(nil, nil)` for absent modeled records without seeding normal eligibility. This is selected simulator policy, not native absence/catalog proof.
- [ ] Reflect owned updates/removal on subsequent calls without changing previously returned scalar observations or other currency records.
- [ ] Require numeric IDs without string coercion or lossy identity aliasing; authenticate secret inputs before validation/lookup and preserve wrappers/caller taint.
- [ ] Publish the modeled query on current Retail/PTR surfaces without inferring earlier-profile backports or deriving flags from quantities, factions or global PvP state.

## How it works

- [C API boundary](../../AGENTS.md#c-api-boundary)
- [Currency projection, separate scope](currency-list-field-projection.md)

## Implementation inventory

- `tests/currency_war_mode_bonus.rs`: initial public absence/type/security controls; getter and owned flag inputs not implemented at this stage.
- `src/c_api/item_spell/c_currency.rs`: existing currency registration seam.

## Tests asserting this spec

`tests/currency_war_mode_bonus.rs`. Initial controls do not prove positive identity mapping or updates; supplied-state tests follow actual RED.

## Known gaps (current cycle)

- [ ] Authentic initial/positive RED, producer, current-profile GREEN, normal publication/startup and final Rust/independent proof pending.

## Out of scope

Native eligibility catalogs, generated normal-session memberships, PvP desired/active state, quest eligibility, faction-grant relationships and tooltip first-instance production. These are separate caller dependencies: no mocked getter/vendor rewrite or end-to-end quest-display claim substitutes for those models. Completed currency field-projection proof remains separate; parent handoff OPEN.
