# Current house information

`C_Housing.GetCurrentHouseInfo()` exposes a supplied nullable housing snapshot on Retail/PTR. Source: `src/c_api/c_housing/current_house.rs`; [model and evidence](../wiki/systems/housing-spatial-presence.md#current-house-shape-and-ptr-startup).

## What it must do

- [ ] Return exactly one nullable `HouseInfo` with no arguments, as declared by both pinned profiles' `HousingUIDocumentation.lua`.
- [ ] Project supplied `plotID:number` and nullable `houseName:string`, `ownerName:string`, `plotCost:number`, `neighborhoodName:string`, `moveOutTime:time_t`, `plotReserved:bool`, `neighborhoodGUID:WOWGUID`, `houseGUID:WOWGUID`, as declared by `HousingUISharedDocumentation.lua`.
- [ ] Return one nil for an unsupplied simulator snapshot; do not fabricate fields from area, ownership or tracked-house selection. This is a host-input policy, not a native default or native nil-selection claim.
- [ ] Preserve absent optional fields and both boolean values; reflect replacement/clearing of the supplied snapshot.
- [ ] Admit `SHARD_TRANSFER` registration/validity on PTR only through `client_profile::ACTIVE`, matching pinned 12.1.7.70493 `SystemDocumentation.lua`; preserve other profiles' existing event policy.

## How it works

- [Housing shape and startup evidence](../wiki/systems/housing-spatial-presence.md#current-house-shape-and-ptr-startup).

## Implementation inventory

- `src/c_api/c_housing/current_house.rs`: documented shape and projection.
- `src/c_api/c_housing.rs`: Retail/PTR publication.
- `src/lua_api/state/support_types.rs`: optional snapshot in the existing housing state.
- `src/event/valid_events.rs`: PTR-only declared event admission above `sim-event`.

## Tests asserting this spec

`tests/housing_current_house_info.rs` tests nullable shape, concrete supplied rows, replacement/clearing and Retail/PTR event admission. `tests/cf2_housing_plot.rs` retains the original registered world event and housing-controls loader; only missing original tutorial data/utility loads were added to its fixture.

## Known gaps (current cycle)

- [ ] Current merged Retail/PTR GREEN, preservation, mutation and all-target proof pending.

## Out of scope

Native acquisition, validation, location/ownership coherence, transition/event timing and native nil-selection semantics remain **native-unknown**. Fixture values are host inputs, not native return observations. No Classic/Forever query expansion, vendor edits or unrelated PTR event additions.
