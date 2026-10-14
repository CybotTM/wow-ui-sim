# Macro vehicle-UI conditions

`SecureCmdOptionParse` must distinguish the player's `vehicleui` fact from the selected unit snapshot's `unithasvehicleui` fact. State lives in `src/lua_api/game_data.rs`; condition evaluation lives in `src/lua_api/globals/security/cmd_option.rs`. See the [historical 3.3.0 audit](../wiki/investigations/patch-3-3-0-api-audit.md).

Historical wording distinguishes player and macro-target predicates; it does not establish native defaults, absent-target behavior or negation grammar. Existing NPC/admin/party snapshots initialize the new fact to false as explicit simulator policy, not native evidence. A snapshot of the actual player copies the player's existing fact; arbitrary targets do not inherit player state or hostility.

## What it must do

- [ ] Player true, explicit target false: `[vehicleui]yes;no` selects `yes`; `[@target,unithasvehicleui]yes;no` selects `no`.
- [ ] Player false, explicit target true: those clauses select `no` and `yes`.
- [ ] Both explicit facts true: both clauses select `yes`.
- [ ] Both explicit facts false: both clauses select `no`.

## How it works

- [Historical contract and coverage](../wiki/investigations/patch-3-3-0-api-audit.md)
- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/game_data.rs`: explicit `TargetInfo.has_vehicle_ui` snapshot input; clones retain it.
- `src/lua_api/globals/admin_party_target_helpers.rs` and `admin_api/units.rs`: current and legacy admin target constructors initialize the simulator fact.
- `src/lua_api/globals/targeting_verbs.rs`: player/NPC/party target snapshot initialization.
- `src/lua_api/globals/security/cmd_option.rs`: existing public parser; vehicle predicates not yet implemented in this passive checkpoint.

## Tests asserting this spec

- `tests/cmd_option_vehicle_ui.rs`: four concrete public parser cases with independently supplied player/target facts and constant target identity/hostility. Retail 12.0.5 feature gate; authored, not compiled or executed.
- `tests/cmd_option_selected_unit.rs`: existing selected-clause/unit and unchanged cached slash-handler preservation scope; not newly executed here.

## Known gaps (current cycle)

- [ ] Compile passive state/fixture; execute behavioral RED before adding the condition producer.
- [ ] Implement predicates, execute targeted GREEN and selected-unit preservation with full diagnostics.
- [ ] Verify affected profiles, formatting/checks, readability and parent acceptance separately.

## Out of scope

This bounded slice does not establish absent-target/negation/native/default semantics, full macro grammar, native UI, or a change to the separate `UnitHasVehicleUI` API. No vendor edits, getter overrides or inferred vehicle state from hostility.
