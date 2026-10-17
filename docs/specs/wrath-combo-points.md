# Wrath target-bound combo points

Wrath publishes the existing target-bound simulator combo pool in `src/lua_api/globals/real/combo_points.rs`. Main authorized this projection and the zero-when-unsupplied policy on 2026-10-17. This is a simulator contract, not native parity.

## What it must do

- [x] Original `ComboFrame.lua:20` reads `GetComboPoints(PlayerFrame.unit, "target")`; absent/unsupplied targets return zero and hide the frame and its first highlight/shine.
- [x] `A_Admin.SetPlayerPower(count, max, 4)` binds the existing secondary pool to the selected target GUID, as on [Forever](forever-combo-points.md). Original ComboFrame shows supplied points; swapping targets hides stale points without changing UnitPower.
- [ ] Resolved nonplayer owners without supplied combo state return zero on Wrath, not an ownership error. Retail's player-owned and Forever's existing query/error policies stay unchanged.

## How it works

- [Target-bound ownership](../wiki/investigations/forever-combo-points.md)
- [Admin health/power inputs](../admin-api/health-power.md)

## Implementation inventory

- `src/lua_api/globals/real/combo_points.rs`: shared profile selection, assignment and query.
- `src/lua_api/globals/admin.rs`: forwards combo inputs to that model; player-pool profiles remain unbound.
- `src/lua_api/state_types/character_world.rs`: passive optional target GUID; unused outside target-bound profiles.
- `src/lua_api/globals/register.rs`: Wrath publication.

## Tests asserting this spec

`tests/wrath_unmasked_residuals.rs`: full unchanged ComboFrame.lua plus ordinary frame fixtures, no replacement count or fade functions; supplied tests load the unchanged UIParent.lua fade implementation.

## Known gaps (current cycle)

- [x] Original absent-global RED at ComboFrame.lua:20 on empty and supplied paths; both GREEN in the d11a25154 sealed Wrath43 gate. The resolved nonplayer zero control also passes separately after its fixture assertion was added. Startup no longer fails at ComboFrame:20; it next exposes UnitIsTalking at PlayerFrame.lua:150.
- [ ] Retail/Forever preservation and final checks pending; whole Wrath startup remains failing (18 records / 24 occurrences).

## Out of scope

Native parity, multi-target accumulation, combat-generated gains, new ownership models and vendor edits. Existing [Retail](retail-combo-points.md) and [Forever](forever-combo-points.md) policies are not redefined.
