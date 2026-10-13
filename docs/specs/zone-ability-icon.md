# Zone ability spell icon

`C_ZoneAbility.GetZoneAbilityIcon(zoneAbilitySpellID)` must return a numeric icon file ID for the concrete known spell below from the existing spell catalog. Cached metadata establishes spell-ID input and numeric file-ID output; the current string placeholder contradicts that shape. [C API architecture](../lua-api.md) describes query ownership; current implementation remains in the legacy Lua surface path.

## What it must do

- [ ] For concrete catalog-backed Fireball spell 133, return exactly one numeric value matching its catalog `icon_file_data_id`, not a texture path string.

## How it works

- [Existing spell API](../lua-api.md).

## Implementation inventory

- `src/lua_api/globals/missing_surface/zone_ability.rs`: existing public query, still placeholder.
- `data/spells.rs`: existing generated catalog, not edited by this slice.

## Tests asserting this spec

- `tests/zone_ability_icon.rs`: positive Lua-facing catalog lookup contract.

## Known gaps (current cycle)

- [ ] Test staged; genuine behavioral RED and producer fix unrun.
- [ ] Known-spell output remains a string in current source.

## Out of scope

Native unknown/invalid spell behavior and native parity remain unknown. Active-ability membership, zone lifecycle and external addon caller acceptance are not established by this staged positive test. No new catalog or generated-data edits.
