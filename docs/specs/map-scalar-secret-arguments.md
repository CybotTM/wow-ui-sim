# Map scalar secret arguments

`C_Map.GetMapInfo(uiMapID)` and `C_Map.GetMapArtID(uiMapID)` must accept authentic secret-number selectors from untainted callers in current Retail/PTR while preserving existing ordinary-input and historical-profile behavior. Their source lives in `src/c_api/c_map.rs`; see the [existing map investigation](../wiki/investigations/map-display-info-model.md) for related secret-input behavior and evidence boundaries. Implementation is user-approved; this document records the contract, not implementation or runtime proof.

Cached Retail and PTR `AddOns/Blizzard_APIDocumentationGenerated/MapDocumentation.lua` both declare these two functions with required numeric `uiMapID`, `MayReturnNothing = true`, and `SecretArguments = "AllowedWhenUntainted"`. This is declaration evidence, not native parity. `client-retail` enables `retail-12-1-0`; `client-ptr` enables `retail-12-1-5`, which includes `retail-12-1-0`. Historical builds without that capability retain their existing behavior; this contract does not infer historical native secret policy.

## What it must do

All checkboxes below are **UNVERIFIED** until actual main-thread RED/GREEN evidence covers them. Existing MapDisplay proof is not proof for these two functions.

### Current Retail/PTR secret arguments

- [ ] UNVERIFIED: For each target API, an untainted caller supplying an authentic host-VM secret number containing an accepted integer map ID receives the same result values and return count as with the corresponding ordinary selector.
- [ ] UNVERIFIED: For each target API, a tainted caller's authentic secret selector is rejected under `pcall`; an ordinary selector remains accepted with unchanged results.
- [ ] UNVERIFIED: Successful and rejected calls preserve the original secret wrapper. Untainted callers remain untainted; addon-tainted callers retain their taint, and parent caller taint is restored after the addon call.

### Existing compatibility

- [ ] UNVERIFIED: `GetMapArtID` returns exactly one existing catalog art ID for a present map and zero values for an absent map, regardless of permitted ordinary/secret representation of the same selector.
- [ ] UNVERIFIED: `GetMapInfo` returns exactly one unchanged details table for a present map, retains existing generic details for absent IDs in 1–10,000, and returns zero values for absent IDs outside that range. Permitted secret selectors do not alter these branches or their values.
- [ ] UNVERIFIED: Ordinary parsing remains unchanged: the current `i32::from_stack` exact-integer parser rejects fractional values, strings, and i32 overflow. Secret acceptance does not broaden selector validation or change ordinary diagnostics.
- [ ] UNVERIFIED: Historical profiles/epochs without `retail-12-1-0` retain existing parsing, registrations, catalog/default handling, and zero-return behavior; no secret-policy backport is introduced.

## How it works

- [Map display model and bounded evidence](../wiki/investigations/map-display-info-model.md)
- [Related MapDisplay contract](map-display-info.md)
- [Lua API architecture](../lua-api.md)
- [Client profile architecture](../wiki/systems/client-profiles.md)

## Implementation inventory

- `src/c_api/c_map.rs` — existing target registrations, scalar selector parsing, catalog lookups, generic map details, and related MapDisplay secret-aware precedent.
- `src/lua_bridge/from_stack.rs` — existing typed selector conversion contract; parser-validation changes are excluded.
- `Cargo.toml` — current Retail/PTR capability selection and historical epoch feature boundaries.
- `tests/c_map_probes.rs` — existing ordinary map controls and authentic host-secret/tainted-caller precedent.
- `tests/map_scalar_secret_inputs.rs` — four current-epoch secret-input tests and two ungated ordinary compatibility tests; execution remains unverified.

## Tests asserting this spec

`tests/map_scalar_secret_inputs.rs` supplies new target coverage. Existing `tests/c_map_probes.rs` ordinary target tests and MapDisplay security tests are controls/precedent, not admitted RED/GREEN for this slice. No new target test execution is claimed.

Required main-thread evidence must use a deliberately seeded map row with distinctive art ID/name and an authentic host-VM secret numeric selector, not a Lua lookalike. For both APIs, assert exact result count and fixture values, untainted secret acceptance, tainted ordinary acceptance, tainted secret rejection, and wrapper/caller preservation. Retain known/absent-map controls, `GetMapInfo` generic-range controls, ordinary invalid-selector controls, and historical-profile compatibility coverage. Fixture values are simulator test data, not native catalog facts.

## Known gaps (current cycle)

- [ ] UNVERIFIED: Main observes targeted authentic-secret RED before production changes, then GREEN with the same behavioral assertions and compatibility controls.
- [ ] UNVERIFIED: Current Retail and PTR target execution and historical-profile preservation receive explicit bounded evidence; compilation or related API proof alone does not close these requirements.

## Out of scope

Native parity and historical native secret-policy claims; guessed or refreshed map catalogs; parser-validation changes; changes to established generic/default/zero-return behavior; other `C_Map` members or other APIs; generic bridge redesign. This docs-only stage runs no builds/tests and supplies no implementation-completion claim.
