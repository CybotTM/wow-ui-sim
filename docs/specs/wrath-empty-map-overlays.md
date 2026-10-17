# Wrath empty map overlays

Wrath's temporary `GetNumMapOverlays()` count lives in `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`. No discovered-map overlay catalog is modeled; the approved empty-state policy returns zero, not native-default evidence.

## What it must do

- [ ] Return one numeric zero when no overlay records are supplied.
- [ ] Unchanged `WorldMapFrame_Update` at WorldMapFrame.lua:304 skips the empty overlay loop and hides previously shown WorldMapOverlay textures at :357–359.

## How it works

- [Lua API](../lua-api.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

- `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`: Wrath-only count; retire when a supplied overlay model owns both records and count.

## Tests asserting this spec

`tests/wrath_unmasked_residuals.rs`: load complete unchanged WorldMapFrame.lua; ordinary widget inputs include two stale overlay textures. No count or map callback is replaced. Original-consumer RED reaches :304.

## Known gaps (current cycle)

- [ ] GREEN/startup evidence pending.
- [ ] The next debug-map capability query `HasDebugZoneMap()` at :363 has no backing model or approved boolean default. The bounded test records that downstream failure explicitly; it is not a complete WorldMap update or clean-startup claim.

## Out of scope

Populated overlays, debug-map capability policy, other profiles, vendor edits and native parity.
