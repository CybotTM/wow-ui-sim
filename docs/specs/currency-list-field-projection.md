# Currency list field projection

Expose existing currency-list backing fields through `C_CurrencyInfo.GetCurrencyListInfo` and the Mists legacy tuple. Existing listed currencies project supplied `SimState.currency_info` display values; list structure remains static. Source lives in `src/c_api/item_spell/c_currency.rs` and `src/mists/compat_bootstrap.lua`.

## What it must do

- [ ] Return non-nil boolean `isShowInBackpack` from `CurrencyEntry.is_show_in_backpack`, preserving true and false.
- [ ] Return non-nil number `maxQuantity`, preserving zero and positive caps. Static header values come from `CurrencyEntry`; ordinary rows use supplied-state values.
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
- `tests/mists_currency_list.rs` — existing Mists namespace and legacy-tuple controls.
- `tests/currency_info_probes.rs` — shared supplied-state display and missing-row regressions.

## Tests asserting this spec

Two existing static-field tests in `tests/mists_currency_list.rs` (historical bounded GREEN, not supplied-state mutation proof):

- `currency_list_info_preserves_watched_flags_in_namespace_and_legacy_tuple`
- `currency_list_info_preserves_max_quantity_in_namespace_and_legacy_tuple`

Saved static Mists audit `20261010T051236Z`, compiled revision `a62f7a95ba22857ec48cd9e115fc15eb77df12e6`: five tests PASS, zero FAIL, exit 0. Field assertions prove strict boolean true/false at namespace `isShowInBackpack` and legacy slot 5, and strict numeric cap 90 at namespace `maxQuantity` and legacy slot 8; they do not prove zero-cap or mutation behavior. The other three tests cover numeric singleton list size, populated real TokenUI update, and injected missing-size failure after dependency loading. Warnings and loader diagnostics remain; no native, catalog, full-profile or full-suite acceptance.

Historical RED `0148437a7868b0a25d0ce2fca504378d4558b5ee` (`20261010T043019Z`) and supplemental RED `5184a8cf8feea9a820a15264604a6af303bb5a9d` (`20261010T045459Z`) each recorded one PASS / four FAIL / exit 101. Their field failures establish nil/false versus true/true and nil/nil versus 90/90. Earlier TokenUI fixture failures occurred before the intended size lookup, not successful missing-size reproductions. Private receipt: `/home/osso/.local/state/wow-ui-sim/verification/mists-currency-green-current/20261010T051236Z/audit/report.md`.

Four supplied-state regressions in `tests/currency_info_probes.rs`, introduced at `085f7d799` and decoder-corrected at `f4a6fb2e4`:

- `get_currency_list_info_reflects_successive_existing_currency_updates`
- `get_backpack_currency_info_reflects_successive_existing_currency_updates`
- `get_currency_list_info_returns_nil_when_existing_currency_row_is_removed`
- `get_backpack_currency_info_returns_nothing_when_existing_currency_row_is_removed`

Mists wrapper preservation probe in `tests/mists_currency_list.rs`:

- `legacy_currency_display_updates_preserve_static_rows_and_backpack_membership` — successive positive/zero display updates, exact legacy tuple arity/slots and static watched membership despite a conflicting supplied backpack flag; execution pending.

## Known gaps (current cycle)

- [ ] Targeted GREEN and preservation controls remain pending. Authentic default Retail RED at `20261011T073849Z`, revision `f4a6fb2e4`, compiled successfully and executed 12 existing controls PASS / four new regressions FAIL / exit 101 with seven scoped inputs unchanged. Initial `085f7d799` attempt failed compilation because the test decoder does not support `u32`; zero cases executed. Historical static-field proof does not cover mutation behavior.

## Out of scope

Native Mists catalog and comprehensive currency parity are not claimed. No catalog/default/registration changes, alternate paths, new fallbacks, vendor edits, or adjacent serializer refactors.
