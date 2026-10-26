# Supplied stable slot observations

Retail/PTR `C_StableInfo.GetStablePetInfo(index)` reads explicit per-slot observations. Source: `src/c_api/c_stable_info/pet_info.rs`. See the [system page](../wiki/systems/stable-pet-info.md).

## What it must do

- [ ] Publish the slot query and simulator inputs only on Retail/PTR via ACTIVE; preserve Forever's existing independent producer and other profiles' absence.
- [ ] Accept a complete profile-specific PetInfo or explicit empty observation per slot. Missing/cleared observations raise a named native-unknown error, not an inferred empty result.
- [ ] Return fresh records/ability arrays for populated observations and exactly one nil for explicit empty observations. Never infer a slot observation from the active-pet list.
- [ ] Preserve independent environment state, copied inputs, transactional input validation, replacement and clearing.
- [ ] Feed installed original selected StableFrame.RefreshSelectedPetData and bounded secondary-slot SetPet control flow; changing supplied slots and values changes what the consumers receive.

Pinned declarations: each profile's `Blizzard_APIDocumentationGenerated/StableInfoDocumentation.lua`: Retail70448 function66–79 and PetInfo193–214; PTR70493 function88–101 and PetInfo215–241; Forever70532 agrees with PTR but is unchanged. All declare required luaIndex and nullable PetInfo; none declares MayReturnNothing. Retail has16 fields and PTR21. Numeric/string/boolean/array values are supplied, including documented-default uiModelSceneID; no native values are guessed.

Simulator inputs: `A_Admin.SetStablePetInfo(slot, record_or_nil)` supplies a record or known empty; `A_Admin.ClearStablePetInfo(slot)` restores unknown. Positive integral representable slot and matching record.slotID are simulator validation policies, not native argument-validation claims.

## How it works

- [Per-slot state and shared codec](../wiki/systems/stable-pet-info.md)

## Implementation inventory

- `src/c_api/c_stable_info/pet_info.rs`: supplied-state query and simulator inputs.
- `src/c_api/c_stable_info/active_list.rs`: existing PetInfo decoder/projector reused without policy refactor.
- `src/c_api/c_stable_info.rs`, `src/lua_api/globals/admin.rs`: profile-gated registration wiring.
- `src/lua_api/state.rs`, `state/sim_state.rs`: environment-local observations.

## Tests asserting this spec

- `tests/stable_pet_info.rs`: original selection/secondary consumers and snapshot/input behavior.
- `tests/stable_active_list.rs`: existing original active-slot preservation with explicit empty secondary observation.

## Known gaps (current cycle)

- [ ] Executed RED/GREEN, paired stock maps and seven-profile zero-warning check receipts.

## Out of scope

Native ownership/defaults/acquisition, swaps/events, active-list synchronization, full stable-window lifecycle, specialization catalog, classic legacy globals, Forever producer changes and 3D pet rendering are separate contracts.
