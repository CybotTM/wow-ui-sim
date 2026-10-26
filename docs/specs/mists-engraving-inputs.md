# Mists engraving query inputs

The normal Mists bag and settings consumers read explicit environment-local engraving inputs. The C API implementation belongs in `src/c_api`; [the investigation](../wiki/investigations/mists-engraving-inputs.md) records consumer selection and proof limits.

## What it must do

- [x] Publish only the seven members reached by the normal Mists bag/settings flow, gated by `client_profile::ACTIVE`. Do not change other profiles.
- [x] Project supplied availability, engraving mode, optional current rune, per-inventory-slot eligibility, optional per-slot rune, and current-cast per-slot eligibility. An unsupplied input must raise an explicit error naming the query; supplied false/nil is not missing input.
- [x] Return the pinned `EngravingData` fields and independent Lua tables. `RefreshRunesList` records an observable simulator refresh request with no Lua results; it does not invent native catalog refresh effects.
- [x] Drive the original selected `ContainerFrame_Shared.lua` overlay and desaturation functions, with input supplied before addon loading. Cover true/false, present/nil, slot discrimination, repeated refresh requests and environment isolation. The complete original settings consumer remains blocked before engraving; no settings-path proof is claimed.
- [x] Keep the old storage-flow test unchanged. Without supplied input its named unsupplied-state error is boundary advancement, not a passing storage-flow claim.

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

- [x] Execute producer-subtracted RED and supplied GREEN with byte-identical tests: corrected RED `e97b841e7`, GREEN `eb3dca9dc`; all three exact GREEN cases pass.
- [ ] The complete original `InterfaceOverrides.AdjustDisplaySettings` stops at line4 because `C_TransmogOutfitInfo.IsTransmogEnabled` is absent. That separate producer is outside this scope. Do not substitute copied vendor excerpts or claim settings-path GREEN.
- [x] Compare bound stock maps: scaffold/no-producer `c50e6386a` and producer `f7a9227b8` both have 6 raw keys/12 occurrences, with no raw-key/count changes.
- [ ] Finish seven-profile zero-warning all-target checks; corrected-source Mists passes, six profiles remain.

## Out of scope

- LoD `Blizzard_EngravingUI`: loadable on Mists but no normal Mists UI entry.
- Era/Anniversary engraving and their existing temporary availability default; Wrath has no pinned engraving callers.
- Invented native inactive defaults, catalog/filter policy, casting transitions and refresh side effects.

## Pinned declarations

Each citation is relative to Mists `Blizzard_APIDocumentationGenerated/EngravingInfoDocumentation.lua`: `GetEngravingModeEnabled` :73–79; `GetCurrentRuneCast` :64–70; `IsEngravingEnabled` :179–185; `RefreshRunesList` :269–271; `IsInventorySlotEngravable` :211–223; `GetRuneForInventorySlot` :135–147; `IsInventorySlotEngravableByCurrentRuneCast` :226–238; `EngravingData` :329–340. These declare results and fields, not native defaults.
