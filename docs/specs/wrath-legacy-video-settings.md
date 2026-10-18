# Wrath legacy video settings

Wrath's legacy gamma uses simulator state in `src/lua_api/globals/real/legacy_gamma.rs`. Initial zero is user-approved simulator policy (2026-10-17), not a captured native default. BaseMip default0 is the same explicitly approved policy, not a captured native default. Original Blizzard controls remain unchanged.

## What it must do

- [ ] Wrath `GetGamma` and `SetGamma` share one per-environment scalar, initially zero and directly settable through Rust `SimState::legacy_gamma`.
- [ ] The original Resolution setup traverses SetMinMaxValues → GammaSlider OnValueChanged without errors; original setters round-trip -0.5, 0 and 0.5.
- [ ] Gamma never changes the real display or the separate Gamma CVar. Other client profiles retain their existing surface.

- [ ] Wrath registers BaseMip with factory default `0`, case-insensitive queries and unchanged supplied-current override precedence.
- [ ] Original Effects setup projects unsupplied BaseMip0 to slider1; supplied0/1 project to1/0 and original setters invert those values without changing the factory default.

## How it works

- [Startup consumer investigation](../wiki/investigations/wrath-startup-u-residuals.md)
- [CVar and Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/cvars.rs`: Wrath-only BaseMip entry in the existing default registry.
- `src/lua_api/globals/real/legacy_gamma.rs`: Wrath-only modeled getter/setter registration.
- `src/lua_api/state/sim_state.rs` and `state.rs`: public scalar and neutral initialization.
- `src/lua_api/globals/real/wrath_startup_queries.rs`: existing Wrath registration route; old constant compatibility getter removed.

## Tests asserting this spec

- `tests/wrath_gamma_basemip.rs`: cache-pinned original Resolution/Effects consumers and state/default/override controls.

## Known gaps (current cycle)

- [x] Bounded Wrath normal-Cargo GREEN at `89a915a0c`: 34/34 PASS (four Gamma/BaseMip cases, six Slider step-assignment, five range-lifecycle, eighteen widget-slider and one original Effects control). Exact argv, full streams and exit0: private handoff `wrath-gamma-basemip-fix/wrath-slider-green.*`.
- [ ] This headless Wrath subset does not establish full-feature startup, current-master integration, other-profile preservation, native parity or rendered brightness; those remain separate receipts.

## Out of scope

Host display changes, rendered brightness, native default/mapping claims, Gamma CVar conversion, vendor edits, other profiles' gamma policy and unrelated startup errors.
