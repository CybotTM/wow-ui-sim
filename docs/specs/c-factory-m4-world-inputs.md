# M4 world startup inputs

M4 resolves original cached Blizzard consumers recorded by [factory removal](c-namespace-nil-members.md), without restoring arbitrary member synthesis. C API state belongs in `src/c_api/`; named service defaults remain temporary. [Lua API notes](../wiki/systems/lua-api.md) describe simulator ownership.

## What it must do

- [ ] Documented non-Wrath combat-audio throttles reuse independent numeric settings, with unset zero and setter round-trip; unmodeled playback enablement reports false.
- [ ] Modern `C_PvP.IsMatchActive` projects existing match state. PTR quest objectives project existing supplied objective snapshots, including unknown-objective absence.
- [ ] Modern private-aura callbacks retain registrations until removed for that unit and callback identity. Other units/callbacks survive removal; repeated removal is harmless. Raid-boss callback replacement and clearing retain only the current registration. Original AuraContainer callback handles can register/unregister.
- [ ] Forever exposes existing loaded CVar state, hidden-group-buff settings and unit-identity secrecy policy through the missing public queries.
- [ ] Named temporary policies return no supertracked vignette/target, disable hybrid minimap and expansion landing unlocks, report unknown honor rewards as nil and no calling membership. Classic map highlights return no values, PvP wait estimates are zero and numeric quest-reward-spell lists are empty. Forever experience preset is nil and quest timers are empty. Each policy names its missing backing system and retirement condition; none claims native defaults.

## How it works

- [C namespace diagnostics](../wiki/systems/addon-loading.md#missing-c-members-f0-2026-10-20)

## Implementation inventory

- `src/c_api/` — existing combat-audio, PvP, objective and identity state integration points; callback lifecycle ownership.
- `src/lua_api/workarounds/temporary/` — explicit defaults for absent world services and existing hidden-buff setting bootstrap.
- `src/lua_api/globals/set_cvar_verb.rs` — existing loaded CVar publication.

## Tests asserting this spec

- `tests/c_factory_m4_contracts.rs` — documented return contracts, existing-state reads, callback lifetime/removal and original AuraContainer handle.
- `tests/c_namespace_nil_contract.rs` — unknown names remain nil.

## Known gaps (current cycle)

- [ ] Capture targeted RED/GREEN per affected profile.
- [ ] Capture one post-M3/M4 stock profile matrix and attribute rows against the frozen F0 inventory, preserving Wrath's empty startup.

## Out of scope

- Vendor/rilua changes, native policy inference, M1/M2/M5/T6 producers and generic fabrication.
- New authoritative services beyond the observed gap-list requirements; temporary defaults retire when those services supply their queries.
