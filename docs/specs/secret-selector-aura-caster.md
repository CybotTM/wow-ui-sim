# Numeric-secret aura caster identity

Retail/PTR 12.1.5 caster-selector compatibility in `src/c_api/c_unit_auras.rs`; [aura runtime](../wiki/systems/lua-api.md).

## What it must do

- [ ] `C_UnitAuras.GetAuraCasterGUID` accepts authentic numeric-secret aura instance IDs under public unit tokens in secure/tainted callers, selecting the same stored aura and configured source GUID as a plain instance.
- [ ] Exactly one secret-wrapped GUID is returned for a secret numeric selector; exactly one ordinary GUID for a public selector. One ordinary nil remains the result for missing aura/unit/caster identity. Secret in → secret identity out is explicitly approved simulator policy, not native-parity evidence.
- [ ] The unchanged original AuraContainerPublicAuraSourceMixin:GetAuraCasterGUID reaches the real producer and forwards its result with all four selector/caller controls.
- [ ] Input opacity, caller taint and source/model rows remain unchanged. Public f64→i32 instance coercion is preserved for numeric-secret selection; non-numeric-secret instances and secret unit tokens remain rejected. INFERRED: native invalid-instance conversion/failure rules are not established by these tests.
- [ ] The existing public-selector output and unit-aura-restriction behavior remain unchanged. Implementing the documented RequiresUnitAuraAccess/SecretWhenUnitAuraRestricted layer for this endpoint is separate follow-up scope, not silently bundled into numeric-selector support.

## How it works

- [C API overview](../wiki/systems/lua-api.md)

## Implementation inventory

- `src/c_api/c_unit_auras.rs`: endpoint-local host-only instance inspection and secret identity publication.

## Tests asserting this spec

- `tests/secret_selector_aura_caster.rs`
- `tests/unit_exists_state.rs`

## Known gaps (current cycle)

- [ ] Compiled original-consumer RED→GREEN and preservation receipts.

## Out of scope

Secret unit/spell-string selectors, Retail CarryOver, aura restriction model changes, private-aura endpoints, new GUID model and native invalid-instance ReturnNothing parity. Natural secret-instance producer provenance remains INFERRED; tests inject authentic VM wrappers at the original consumer's selector boundary.
