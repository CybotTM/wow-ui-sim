# Zone ability spell icon

`C_ZoneAbility.GetZoneAbilityIcon(zoneAbilitySpellID)` returns a numeric icon file ID for a known spell from the existing spell catalog. Cached metadata establishes spell-ID input and numeric file-ID output; the current string placeholder contradicts that shape. [Lua API architecture](../lua-api.md).

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

Unknown/invalid spell behavior, active-ability membership, zone lifecycle, external addon caller acceptance and native parity are not established by this positive test. No new catalog or generated-data edits.
