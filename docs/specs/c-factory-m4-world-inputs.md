# M4 world startup inputs

M4 resolves original cached Blizzard consumers recorded by [factory removal](c-namespace-nil-members.md), without restoring arbitrary member synthesis. C API state belongs in `src/c_api/`; named service defaults remain temporary. [Lua API notes](../wiki/systems/lua-api.md) describe simulator ownership.

## What it must do

- [ ] Documented non-Wrath combat-audio throttle reads reuse independent numeric settings, with unset zero; the existing Retail setter round-trip remains intact without publishing an unrouted setter on other profiles. Unmodeled playback enablement reports false.
- [ ] Modern `C_PvP.IsMatchActive` projects existing match state. PTR quest objectives project existing supplied objective snapshots, including unknown-objective absence.
- [ ] Modern private-aura callbacks retain registrations until removed for that unit and callback identity. Other units/callbacks survive removal; repeated removal is harmless. Raid-boss callback replacement and clearing retain only the current registration. Original AuraContainer callback handles can register/unregister and receive the update-info object as their first argument, as consumed by the original cached callback. Update and raid-message registrations use FunctionContainers, matching all original cached consumers; no alternate plain-function dispatch is supported.
- [ ] Forever exposes existing loaded CVar state, hidden-group-buff snapshots and unit-identity secrecy policy through the missing public queries, without enabling unrelated visual-alert APIs or an unrouted hidden-buff setter. Hidden-buff compatibility snapshots retain their existing Lua-owned input field.
- [ ] Named temporary policies return no supertracked vignette/target, disable hybrid minimap and expansion landing unlocks, report unknown honor rewards as nil and no calling membership. Classic map highlights return no values, PvP wait estimates are zero and numeric quest-reward-spell lists are empty. Forever experience preset is nil and quest timers are empty. Each policy names its missing backing system and retirement condition; none claims native defaults.

## How it works

- [C namespace diagnostics](../wiki/systems/addon-loading.md#missing-c-members-f0-2026-10-20)

## Implementation inventory

- `src/c_api/combat_audio_throttle.rs`, `c_pvp.rs`, `quest_objectives/queries.rs`, `c_secrets.rs` — existing setting/snapshot projections and profile publication; the throttle setter retains its original Retail epoch gate.
- `src/c_api/private_aura_callbacks.{rs,lua}` — rooted environment-local callback lifecycle and authored dispatch; replaces the temporary update-callback implementation.
- `src/c_api/c_cvar_readiness.rs` — synchronous CVar initialization invariant, formerly published by the legacy CVar registrar.
- `src/lua_api/workarounds/temporary/world_service_defaults.rs` — named absent-world-service policies.
- `src/lua_api/workarounds/temporary/unit_auras_state.rs` — existing hidden-buff snapshot read extended to Forever; existing visual-alert and setter publication stays at its original Retail epoch.

## Tests asserting this spec

- `tests/c_factory_m4_contracts.rs` — documented return contracts, existing-state reads, callback lifetime/removal and original AuraContainer handle.
- `tests/c_namespace_nil_contract.rs` — unknown names remain nil.

## Known gaps (current cycle)

- [ ] Capture targeted RED/GREEN per affected profile.
- [ ] Capture one post-M3/M4 stock profile matrix and attribute rows against the frozen F0 inventory, preserving Wrath's empty startup.

## Out of scope

- Vendor/rilua changes, native policy inference, M1/M2/M5/T6 producers and generic fabrication.
- New authoritative services beyond the observed gap-list requirements; temporary defaults retire when those services supply their queries.
- Newly exposed members absent from the frozen gap-list: retain them as residuals with profile, original consumer location, stderr receipt and causal root fix; do not expand this batch.
