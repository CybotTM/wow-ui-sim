# Mists supplied transmog availability

Mists `C_TransmogOutfitInfo.IsTransmogEnabled()` reads an explicitly supplied environment-local boolean. [Implementation and evidence](../wiki/systems/mists-transmog-enabled.md) describe the boundary.

## What it must do

- [ ] Publish only on Mists through `client_profile::ACTIVE`, without changing modern registration.
- [ ] Initialize the Mists input to `None`; supplied `Some(true)` and `Some(false)` return exactly one corresponding boolean.
- [ ] Raise `C_TransmogOutfitInfo.IsTransmogEnabled: transmog availability has not been supplied` for `None`.
- [ ] The original selected Mists `InterfaceOverrides.AdjustDisplaySettings` omits helm/cloak controls for true and creates both boolean proxy checkboxes for false.
- [ ] Preserve independent environments, transitions and unset recovery through that original caller.

## How it works

- [Supplied input and consumer boundary](../wiki/systems/mists-transmog-enabled.md)

## Implementation inventory

- `src/c_api/mists_transmog.rs`: Mists-specific supplied availability model, ACTIVE gate and query.
- `src/c_api/registration.rs`: registers the query without ungating the modern outfit module.
- `src/lua_api/state/sim_state.rs` and `state.rs`: per-environment model storage and construction.
- `tests/mists_transmog_enabled.rs`: original selected consumer and real Settings controls.

## Tests asserting this spec

`tests/mists_transmog_enabled.rs`: enabled-first and disabled-first selectors each observe both branches, transitions, independent environments and named unset recovery. Both exact producer-free RED cases reach the missing original query at `InterfaceOverrides.lua:4`. GREEN execution is pending.

## Known gaps (current cycle)

- [ ] Observe producer-subtracted RED and producer GREEN; complete bound stock and seven-profile warning checks.

## Out of scope

Retail/PTR retain the existing default-true `SimState.transmog_enabled`; that default is not established native policy. Forever's selected Camelot spellbook query remains a separate missing producer. No entitlement policy, account state, persistence, inferred events or native default is supplied. Later display-settings branches are not accepted by this bounded query proof.
