# Mists battle-pet display identity

Mists publishes `C_PetBattles.GetDisplayID(petOwner, petIndex)` from supplied battle-pet state in `src/c_api/c_pet_battles.rs`. See [implementation and consumer evidence](../wiki/systems/pet-battle-names.md#mists-display-identity).

## What it must do

- [ ] Register the producer only when `client_profile::ACTIVE` is Mists; preserve other profiles' existing surfaces.
- [ ] Read `PetBattlePet.display_id` from the same owner roster and one-based slot as GetName, independently of species ID or journal aliases. Return one number when supplied, without inventing a model/skin ID.
- [ ] Return one nil for unsupplied display identity or absent owner/slot. This is an accepted simulator-unknown representation, not a documented native return contract. Native arity remains a laptop question.
- [ ] Let the unchanged original Mists pet-name selectors proceed past Shared PetBattleUI line 1109 without a missing GetDisplayID call. Do not change Blizzard code or model coercion.

## How it works

- [Pet-battle identity and nil receiver trace](../wiki/systems/pet-battle-names.md#mists-display-identity).

## Implementation inventory

- `src/c_api/c_pet_battles.rs` — Mists-only registration and roster display-ID query.
- `src/lua_api/sim_substates/mod.rs` — optional supplied field beside the per-pet name; seeded records leave it unknown.

## Tests asserting this spec

`tests/cf2_pet_identity_consumers.rs`: exact `original_pet_name_owner_slot_and_refresh` and `stock_pet_name_consumer_refresh` selectors remain byte-identical. They reject a missing member through retained original-consumer errors. They do not supply display IDs or assert model values, so they cannot reject constant/nil producers and do not prove supplied-value projection. Static staging only; no build or runtime acceptance.

## Known gaps (current cycle)

- [ ] Execute Mists producer-minus RED and producer candidate with both exact selectors, then compare full stock maps.
- [ ] Obtain native supplied/unsupplied return arity and value behavior from host aso. Generated Mists PetBattles docs omit GetDisplayID.
- [ ] Supplied display-value, owner/slot switching and constant-producer rejection have no behavioral assertions in the unchanged fixtures.

## Out of scope

3D rendering, other profiles' publication, native constants, species/display mapping, adjacent pet APIs and model-method coercion changes. Six existing exhaustive test literals receive only `display_id: None` under an explicit compile-only exception; no assertions or selector bodies change.
