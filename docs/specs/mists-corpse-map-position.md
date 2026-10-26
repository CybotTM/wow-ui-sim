# Mists corpse map position

Mists exposes the pinned `C_DeathInfo.GetCorpseMapPosition(uiMapID: number)` query, returning a nilable vector2 (`Vector2DMixin`). The declaration is `Blizzard_APIDocumentationGenerated/DeathInfoDocumentation.lua:5,11–22` in the profile cache. [Consumer and state notes](../wiki/systems/mists-corpse-map-position.md) describe selection and data flow.

## What it must do

- [ ] Publish the query only on Mists; do not add other DeathInfo APIs or change other profiles.
- [ ] Return one nil result when alive, without a corpse, or without supplied coordinates for the requested map.
- [ ] Project supplied coordinates without replacing them with the player's current map position.
- [ ] Let the original `DeathMapDataProvider` acquire, position, show and remove its selected `CorpsePinTemplate` across state and map changes.
- [ ] Observe the existing `RetrieveCorpse` transition without changing resurrection behavior.

## How it works

- [Consumer and state notes](../wiki/systems/mists-corpse-map-position.md).

## Implementation inventory

- `src/lua_api/state/sim_state.rs` — explicit corpse map observations alongside existing corpse availability.
- `src/lua_api/state.rs` — empty observation default.
- `src/c_api/c_death_info.rs` — Mists-only state-backed query.
- `src/c_api/mod.rs` — namespace registration.

## Tests asserting this spec

- `tests/mists_corpse_position.rs` — original selected WorldMap provider, supplied pin coordinates and removal.

## Known gaps (current cycle)

- [ ] Execute original-consumer RED/GREEN, producer-only RED, stock delta and warning-free checks.

## Out of scope

Native corpse discovery, automatic coordinate capture, map-space transforms, other DeathInfo APIs, other-profile publication, and native death/ghost timing are not inferred from the signature. Coordinates are explicit supplied observations.
