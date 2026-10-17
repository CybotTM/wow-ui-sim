# Classic marker active-state query

Era/Anniversary `IsRaidMarkerActive(index)` projects supplied marker indexes. Source: `src/lua_api/globals/real/publication_12_0_0.rs`. Separate root from the [R12 capability/menu contract](classic-startup-a.md).

## What it must do

- [ ] Publish the query before original CompactRaidFrameManager startup evaluates dropdown selections.
- [ ] Return a boolean for the supplied index, matching cached Era/Anniversary `Blizzard_APIDocumentationGenerated/RaidMarkersDocumentation.lua:48-59` (`luaIndex` argument, nonnil boolean result).
- [ ] Empty supplied set returns false; this is approved simulator empty-state policy, not a documented native default. Explicit active index 3 returns true without changing other markers.

## How it works

- [Lua API](../lua-api.md)

## Implementation inventory

- `src/lua_api/globals/real/publication_12_0_0.rs`: host set and query.
- `src/lua_api/globals/register.rs`: narrow Era/Anniversary registration.

## Tests asserting this spec

- `tests/classic_marker_startup.rs`: genuine startup ordering loads the original manager and menu, rejects manager errors, then exercises supplied selection with the original menu initializer.

## Known gaps (current cycle)

- [ ] GREEN and preservation pending. Test commit `505550cda` fails in both profiles on production `9a454578d`; original manager line84 calls missing `IsRaidMarkerActive`. The A fixture captured callbacks without evaluating them, unlike the real dropdown.

## Out of scope

`PlaceRaidMarker` and `ClearRaidMarker` effects are not implemented or claimed. No marker placement, permission/restriction model, invalid-index native semantics, or native default-state claim.
