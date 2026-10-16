# Retail watched currency mutation — optional B5 sub-slice

Integration checkpoint (2026-10-16): B5-watch producer admitted after observed behavioral RED in the approved profile scope. GREEN, preservation and startup observations remain pending. No native-parity claim.

Status: staged, not selected by base B5 acceptance; uncompiled/unexecuted. Proposed simulator contract only.

## What it must do

- [ ] SetCurrencyBackpack resolves current displayed index; SetCurrencyBackpackByID resolves an existing catalog currency independent of visibility. Headers, unknown IDs and invalid indices do nothing.
- [ ] Record environment-local watch overrides, initially matching static membership. Update both list status and the corresponding supplied CurrencyInfo flag; backpack enumeration uses catalog order, independent of collapse/unused placement. Repeated operations are idempotent. No persistence, new events, cap enforcement or capacity policy in C API; original TokenFrame enforces its own capacity.
- [ ] Missing supplied currency records cannot be mutated. Existing watched slots with removed records preserve the shared no-values behavior; unrelated direct supplied display edits do not change watch membership.
- [ ] Quantities and non-Retail behavior remain unchanged.

## How it works

- [C API and Lua boundary](../lua-api.md)

## Implementation inventory

- `src/c_api/currency_list.rs` — watch membership overrides.
- `src/c_api/item_spell/c_currency.rs` — public mutations and list/backpack projections.

## Tests asserting this spec

`tests/currency_watched_mutations.rs` reuses original cached TokenUI loading. Original TokenFrame:SetTokenWatched (Lua:441-456) drives removal and addition after index shifting; direct ByID removal exercises the API form consumed by original BackpackTokenMixin:OnClick (Lua:728-735). Original list refresh and backpack rendering run for the TokenFrame calls; the modified-click branch itself is not exercised.

## Known gaps (current cycle)

- [ ] Compilation, reached RED/GREEN, startup and profile-preservation gates remain unexecuted.

Predicted RED on base-only producer: missing watch mutation leaves Honor watched and/or other removals unchanged, so backpack-empty assertion fails; an earlier missing-function error is registration evidence only. Run the test addition without this patch's producer first; setup errors do not qualify. No RED/GREEN execution here.

## Out of scope

No transfer, filtering, persistence or new catalog. This intentionally supersedes static watch status/membership clauses of [field projection](currency-list-field-projection.md) for Retail public watch mutators only; non-Retail/static and missing-row preservation tests remain applicable. Independent of the unused sub-slice; unused classification does not implicitly watch/unwatch.
