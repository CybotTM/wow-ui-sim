# Supplied stable active-pet list

Retail and PTR expose supplied `PetInfo` observations through `C_StableInfo.GetActivePetList`. This is separate from [Forever stable reads](forever-stable-read-state.md). The implementation belongs in `src/c_api/c_stable_info/active_list.rs`.

## What it must do

- [ ] Publish the getter and its admin inputs only on Retail/PTR. Retail's pinned `StableInfoDocumentation.lua:15–22,193–214` declares a non-nil `table<PetInfo>` with 16 fields. PTR 70493 declares the getter at `:19–26` and `PetInfo` at `:215–241`, with five additional loyalty/happiness/experience fields. Retail must not publish those additional fields.
- [ ] Return one fresh array of fresh records, including fresh numeric ability arrays. Preserve the supplied observation order; do not infer native sorting from slot IDs or pet identities.
- [ ] `A_Admin.SetActivePetList(records)` replaces the complete ordered observation. An explicitly supplied empty list projects as empty; this does not establish native initial or empty-state policy. Passing nil withdraws the observation.
- [ ] `A_Admin.SetActivePet(index, record)` replaces an existing supplied ordinal or appends at the next ordinal. `A_Admin.RemoveActivePet(index)` removes an existing ordinal. These are simulator input operations, not native pet movement, sorting, stable capacity or event policies.
- [ ] With no supplied observation, the getter raises `native-unknown: C_StableInfo.GetActivePetList has no supplied list or documented default`. Partial mutations of an unknown list fail explicitly. No placeholder pets are created.
- [ ] Populated, add, update, remove and supplied-empty transitions reach the original selected `StableActivePetListMixin:Refresh` and its original XML slot buttons. Two independent environments retain disjoint observations.

## How it works

- [Consumer selection and RED requirements](../wiki/investigations/consumer-selection-and-red-proof.md).

## Implementation inventory

- `src/c_api/c_stable_info/active_list.rs`: supplied list, validation, admin mutations and projection.
- `src/c_api/c_stable_info.rs`: profile-scoped getter registration.
- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs`: environment-local observation storage.
- `src/lua_api/globals/admin.rs`: admin registration.

## Tests asserting this spec

- `tests/stable_active_list.rs`: original selected consumer and two-environment lifecycle.
- Deferred `api_shape_stable_active_list.rs` must be restored only after original consumer behavior passes.

## Known gaps (current cycle)

- [ ] Original-consumer RED/GREEN and stock/check evidence are pending.

## Out of scope

Native acquisition, ordering, default membership, empty-state policy, validation rules and event timing are unknown. Admin validation is a simulator input contract only. The secondary-pet `GetStablePetInfo` query is not implemented by this lane. Forever's Camelot stable reads and Wrath/Mists/Era/Anniversary remain unchanged.
