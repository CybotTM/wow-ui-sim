# Supplied stable pet slots

Retail/PTR slot queries use explicit environment-local observations. The [contract](../../specs/stable-pet-info.md) defines the supported boundary; Forever retains its separate existing producer.

## State and projection

`SimState.stable_pet_slots` maps a positive integral slot to `Option<PetInfo>`. An absent key means unknown, a present `None` means explicitly empty, and a present record contains complete supplied fields. `A_Admin.SetStablePetInfo` validates before replacing one observation; `ClearStablePetInfo` removes it. Neither reads active-list membership.

`c_stable_info/pet_info.rs` gates registration with `client_profile::ACTIVE`. It reuses `active_list.rs` decoding and projection: input tables and ability arrays are copied, and every query allocates a fresh record and arrays. Empty slots return exactly one nil. Unknown slots raise a named `native-unknown: C_StableInfo.GetStablePetInfo` error containing the slot. This is an explicit simulator uncertainty policy, not a claim about native errors.

## Original consumer boundary

The selected Retail root and PTR Standard StableUI create the original hidden `StableFrame`. Primary tests call its original `RefreshSelectedPetData`; secondary tests observe original `ActivePetList.Refresh` after secondary-button `SetPet`. These are bounded interactions, not full stable-window or 3D acceptance. Retained setup diagnostics remain visible.

## Sources

- [Contract](../../specs/stable-pet-info.md) and `tests/stable_pet_info.rs`.
- Pinned `Blizzard_APIDocumentationGenerated/StableInfoDocumentation.lua`: Retail 70448 function 66–79, record 193–214; PTR 70493 function 88–101, record 215–241. All fields are supplied, including uiModelSceneID.
- [Consumer selection](../investigations/consumer-selection-and-red-proof.md).

## See Also

- [Active-list contract](../../specs/stable-active-list.md) — independent observations, not inferred slot mappings.
