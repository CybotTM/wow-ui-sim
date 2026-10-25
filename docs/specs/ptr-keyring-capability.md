# PTR keyring capability

`C_ActionBar.ShouldShowKeyring()` exposes a supplied keyring UI capability on PTR. Source: `src/c_api/c_action_bar.rs`; [original consumer and proof](../wiki/systems/housing-spatial-presence.md#ptr-keyring-startup-boundary).

## What it must do

- [x] On PTR, return exactly one non-nil boolean with no arguments, matching authenticated 12.1.7.70493 `ActionBarFrameDocumentation.lua:1022–1028`.
- [x] Project `SimState.action_bar_state.keyring_ui_enabled`, including changes false → true → false. The empty host sets false; this is simulator policy, not an observed native constant/default.
- [x] Gate the addition with `client_profile::ACTIVE`; do not expand Retail publication or create a `C_Container` alias.
- [x] Let original `ContainerFrame.lua:10` continue past its keyring query. Later publication of `CloseAllBags` also depends on the separate input-interface-style API.

## How it works

- [PTR startup boundary](../wiki/systems/housing-spatial-presence.md#ptr-keyring-startup-boundary).

## Implementation inventory

- `src/c_api/c_action_bar.rs`: state-backed query and PTR publication.
- `src/lua_api/state/support_types.rs`: explicit capability in the existing action-bar state.
- `src/lua_api/globals/action_bar_api/registration.rs`: invokes the C API registration.

## Tests asserting this spec

`tests/action_bar_keyring_capability.rs` tests supplied transitions, return count/type and Retail absence. `tests/cf2_housing_plot.rs` exercises original `Blizzard_Game` routing and its original housing loader without assertion changes.

## Known gaps (current cycle)

- [ ] PTR original eight-state rerun waits for the separate `ptr-input-interface-style` merge: `ContainerFrame.lua:14` now reaches missing `C_InputInterfaceStyle` through original `InputUtil.lua:12`. This dependency is not a defect of the keyring query.

## Out of scope

Native keyring eligibility, expansion/faction rules and bag-content inference remain **native-unknown**. This API is not inferred from keys or the `showKeyring` preference. Existing Forever workaround is unchanged. Further unrelated missing APIs are not part of this lane.
