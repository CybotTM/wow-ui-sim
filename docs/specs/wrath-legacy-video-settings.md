# Wrath legacy video settings

Wrath's legacy gamma uses simulator state in `src/lua_api/globals/real/legacy_gamma.rs`. Initial zero is user-approved simulator policy (2026-10-17), not a captured native default. Original Blizzard controls remain unchanged.

## What it must do

- [ ] Wrath `GetGamma` and `SetGamma` share one per-environment scalar, initially zero and directly settable through Rust `SimState::legacy_gamma`.
- [ ] The original Resolution setup traverses SetMinMaxValues → GammaSlider OnValueChanged without errors; original setters round-trip -0.5, 0 and 0.5.
- [ ] Gamma never changes the real display or the separate Gamma CVar. Other client profiles retain their existing surface.

## How it works

- [Startup consumer investigation](../wiki/investigations/wrath-startup-u-residuals.md)
- [CVar and Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/globals/real/legacy_gamma.rs`: Wrath-only modeled getter/setter registration.
- `src/lua_api/state/sim_state.rs` and `state.rs`: public scalar and neutral initialization.
- `src/lua_api/globals/real/wrath_startup_queries.rs`: existing Wrath registration route, preceding shared compatibility bootstrap.

## Tests asserting this spec

- `tests/wrath_gamma_basemip.rs`: cache-pinned original Resolution/Effects consumers and state/default/override controls.

## Known gaps (current cycle)

- [ ] GREEN and requested profile preservation/startup checks pending; original-consumer RED recorded before producers.

## Out of scope

Host display changes, rendered brightness, native default/mapping claims, Gamma CVar conversion, vendor edits, other profiles' gamma policy and unrelated startup errors.
