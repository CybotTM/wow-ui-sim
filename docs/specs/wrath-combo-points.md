# Wrath target-bound combo points

Wrath publishes the existing target-bound simulator combo pool in `src/lua_api/globals/real/combo_points.rs`. Main authorized this projection and the zero-when-unsupplied policy on 2026-10-17. This is a simulator contract, not native parity.

## What it must do

- [ ] Original `ComboFrame.lua:20` reads `GetComboPoints(PlayerFrame.unit, "target")`; absent/unsupplied targets return zero and hide the frame and its first highlight/shine.
- [ ] `A_Admin.SetPlayerPower(count, max, 4)` binds the existing secondary pool to the selected target GUID, as on [Forever](forever-combo-points.md). Original ComboFrame shows supplied points; swapping targets hides stale points without changing UnitPower.
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

- [ ] GREEN and final startup/check evidence pending; original absent-global RED established on both original ComboFrame paths.

## Out of scope

Native parity, multi-target accumulation, combat-generated gains, new ownership models and vendor edits. Existing [Retail](retail-combo-points.md) and [Forever](forever-combo-points.md) policies are not redefined.
