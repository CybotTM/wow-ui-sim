# Mists engraving query inputs

The normal Mists bag and settings consumers read explicit environment-local engraving inputs. The C API implementation belongs in `src/c_api`; [the investigation](../wiki/investigations/mists-engraving-inputs.md) records consumer selection and proof limits.

## What it must do

- [ ] Publish only the seven members reached by the normal Mists bag/settings flow, gated by `client_profile::ACTIVE`. Do not change other profiles.
- [ ] Project supplied availability, engraving mode, optional current rune, per-inventory-slot eligibility, optional per-slot rune, and current-cast per-slot eligibility. An unsupplied input must raise an explicit error naming the query; supplied false/nil is not missing input.
- [ ] Return the pinned `EngravingData` fields and independent Lua tables. `RefreshRunesList` records an observable simulator refresh request with no Lua results; it does not invent native catalog refresh effects.
- [ ] Drive the original selected `ContainerFrame_Shared.lua` overlay and desaturation functions, with input supplied before addon loading. Cover true/false, present/nil, slot discrimination, repeated refresh requests and environment isolation. The complete original settings consumer remains blocked before engraving; no settings-path proof is claimed.
- [ ] Keep the old storage-flow test unchanged. Without supplied input its named unsupplied-state error is boundary advancement, not a passing storage-flow claim.

## How it works

- [Consumer and declaration inventory](../wiki/investigations/mists-engraving-inputs.md)

## Implementation inventory

- `src/c_api/c_engraving.rs` — ACTIVE-gated Mists projections, named missing-input errors, fresh rune tables and local refresh-request recording.
- `src/c_api/engraving_inputs.rs` — explicit state and pinned record shape.
- `src/lua_api/state.rs`, `src/lua_api/state/sim_state.rs` — environment-local state ownership.

## Tests asserting this spec

- `tests/mists_engraving.rs` — supplied original consumers, direct projections, missing inputs and isolation.
- `tests/mists_bank_storage_panel.rs` — unchanged historical storage-flow boundary.

## Known gaps (current cycle)

- [ ] Execute producer-subtracted RED and supplied GREEN with byte-identical tests.
- [ ] The complete original `InterfaceOverrides.AdjustDisplaySettings` stops at line4 because `C_TransmogOutfitInfo.IsTransmogEnabled` is absent. That separate producer is outside this scope. Do not substitute copied vendor excerpts or claim settings-path GREEN.
- [ ] Bind stock changes to original emitters and finish seven-profile zero-warning all-target checks.

## Out of scope

- LoD `Blizzard_EngravingUI`: loadable on Mists but no normal Mists UI entry.
- Era/Anniversary engraving and their existing temporary availability default; Wrath has no pinned engraving callers.
- Invented native inactive defaults, catalog/filter policy, casting transitions and refresh side effects.

## Pinned declarations

Each citation is relative to Mists `Blizzard_APIDocumentationGenerated/EngravingInfoDocumentation.lua`: `GetEngravingModeEnabled` :73–79; `GetCurrentRuneCast` :64–70; `IsEngravingEnabled` :179–185; `RefreshRunesList` :269–271; `IsInventorySlotEngravable` :211–223; `GetRuneForInventorySlot` :135–147; `IsInventorySlotEngravableByCurrentRuneCast` :226–238; `EngravingData` :329–340. These declare results and fields, not native defaults.
