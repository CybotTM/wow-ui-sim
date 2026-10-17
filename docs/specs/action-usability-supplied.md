# Supplied spell-action usability

Integration checkpoint (2026-10-16): spell kind explicitly approved; Retail/Wrath behavioral RED → GREEN and preservation observed. See [bounded proof](../baselines/approved-batch-2026-10-16.md); no native-parity claim.

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
- `tests/action_usability_modern_supplied.rs` — active Retail/PTR/Mists/Forever shared ActionButton file and SharedXMLBase closure; unchanged modern mixin query refresh and `ACTION_USABLE_CHANGED` watcher, Rust-supplied spell state, passive supply, per-slot routing, rebind and empty-slot colors. Explicit host event delivery, not automatic notifications. Execution evidence pending.

## Known gaps (current cycle)

- [x] Both profile cases compiled and reached original color assertions before producer; both became GREEN after producer. Full Retail ActionButtonMixin, native and broader profile acceptance remain unproved.
- [ ] Macro/outfit checks preserve occupancy-query/color behavior only, not action execution or broader action-kind acceptance.

## Out of scope

Resource/range/cooldown calculations, casting eligibility/enforcement, automatic notifications, attack/repeat state (G1/G2), UseAction dispatch (G4), vendor edits, native parity and broader profile acceptance. Host supplies state; the fixture explicitly delivers the public usability event.
