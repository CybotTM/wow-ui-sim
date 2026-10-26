# Mists battlefield vehicles

Mists exposes supplied battlefield map observations through `C_PvP`. The pinned `Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:71–99,760–775` declares both vehicle queries and their record fields. See the [consumer investigation](../wiki/investigations/mists-battlefield-vehicles.md).

## What it must do

- [x] Publish `GetBattlefieldVehicles(uiMapID)` and the original pin's `GetBattlefieldVehicleInfo(vehicleIndex, uiMapID)` on Mists only.
- [x] Return no values outside an active battlefield or for an absent map observation; return an array for a supplied list, including an explicit empty list. The declaration permits no result; it does not specify native absence policy beyond `MayReturnNothing`.
- [x] Preserve supplied list order and all eleven documented record fields without generating native vehicle data.
- [x] Let the original selected WorldMap provider filter dead/player records, acquire the original XML pin, project the supplied name, atlas, size and position, and remove stale pins when the map or active state changes.

## How it works

- [Consumer selection and supplied state](../wiki/investigations/mists-battlefield-vehicles.md)

## Implementation inventory

- `src/c_api/c_pvp/battlefield_vehicles.rs` — supplied vehicle records and query projection.
- `src/c_api/c_pvp.rs` — Mists publication boundary.
- `src/lua_api/state/sim_state.rs` and `src/lua_api/state.rs` — per-environment map observations, empty by default.

## Tests asserting this spec

- `tests/mists_battlefield_vehicles.rs` — original Mists WorldMap provider and XML pins, including supplied records and removal.

## Known gaps (current cycle)

None for the bounded supplied-state contract. Mists/Retail all-target checks, formatting and exact paired producer-only RED proof pass.

## Out of scope

- Native vehicle discovery or seeded vehicle data: inputs must be explicitly supplied.
- Publication on other profiles: this change repairs the selected Mists consumer only.
- Queue transitions and unrelated WorldMap startup errors: existing simulator state remains authoritative.
