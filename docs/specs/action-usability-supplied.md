# Supplied spell-action usability

Integration checkpoint (2026-10-16): G3 producer admitted after observed behavioral RED in the approved profile scope. GREEN, preservation and startup observations remain pending. No native-parity claim.

G3 supplies usability for existing spell actions to the namespace query and public button consumer. State belongs to `src/c_api/action_usability.rs`; see [Lua API architecture](../lua-api.md). This is a simulator contract, not a native resource-rule claim.

## What it must do

- [ ] Accept explicit usable, insufficient-power and other-unusable classifications keyed by spell ID. Missing input means usable for an occupied spell action, preserving the current baseline.
- [ ] Return `(true,false)`, `(false,true)` and `(false,false)` respectively through `C_ActionBar.IsUsableAction` and the Wrath legacy query / Retail deprecated wrapper.
- [ ] Drive the original cached Wrath `ActionButton_OnEvent` usability branch to white `(1,1,1)`, blue `(0.5,0.5,1)` and gray `(0.4,0.4,0.4)` icon colors. Normal texture is white except in the blue branch. Returning to usable restores white.
- [ ] Resolve supplied inputs through the current spell binding: duplicate bindings share classification; a different spell does not inherit it; an empty slot returns `(false,false)` despite retained spell input.
- [ ] Preserve existing macro/outfit occupancy behavior; do not infer their usability from spell inputs.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/c_api/action_usability.rs` — supplied classification and query projection.
- `src/lua_api/state/sim_state.rs`, `src/lua_api/state.rs` — input storage / empty initialization.
- `src/lua_api/globals/action_bar_api.rs` — namespace query registration import.
- `src/wrath/compat_bootstrap.lua` — legacy query delegation.

## Tests asserting this spec

- `tests/action_usability_supplied.rs` — full cached Wrath file, actual textures, original event consumer; Retail additionally loads its complete deprecated wrapper.

## Known gaps (current cycle)

- [ ] Staged only: compile, observed behavioral RED before producer, GREEN and profile preservation remain unexecuted. Patch applicability is not behavior proof.
- [ ] Macro/outfit checks preserve occupancy-query/color behavior only, not action execution or broader action-kind acceptance.

## Out of scope

Resource/range/cooldown calculations, casting eligibility/enforcement, automatic notifications, attack/repeat state (G1/G2), UseAction dispatch (G4), vendor edits, native parity and broader profile acceptance. Host supplies state; the fixture explicitly delivers the public usability event.
