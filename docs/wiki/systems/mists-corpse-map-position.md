# Mists corpse map position

The [contract](../../specs/mists-corpse-map-position.md) binds a supplied corpse observation to the original Mists WorldMap provider. No native corpse coordinates are seeded.

## Selected consumer

`Blizzard_WorldMap_Mists.toc:3–5,12–13` selects game/Mists, requires MapCanvas and SharedMapDataProviders, and loads the Cata WorldMap Lua/XML. `Cata/Blizzard_WorldMap.lua:206` installs `DeathMapDataProviderMixin`. `Blizzard_SharedMapDataProviders_Mists.toc:4–5,25–26` selects the original `DeathMapDataProvider.lua` and **Wrath** `DeathMapDataProvider.xml`, not the root XML. That selected XML defines `CorpsePinTemplate` using `CorpsePinMixin`.

The original refresh removes old pins, calls `C_DeathInfo.GetCorpseMapPosition` at line 26, then acquires, positions and shows a corpse pin only for a non-nil vector. The test loads stock startup discovery order and locates the provider installed by original OnLoad; it does not substitute a provider or pin.

## State boundary

Existing `corpse_available` represents corpse availability. The existing death predicate treats player health at or below zero as dead and has no separate ghost state (`group_queries_relationships.rs`). Supplied `corpse_map_positions` stores independent coordinates by UI map. The query returns one nil while alive, without availability, or without an observation for the requested map. `RetrieveCorpse` already clears availability and restores health, so the query observes that transition without changing the verb.

Positive results use the original SharedXML `CreateVector2D` factory, preserving `Vector2DMixin` methods rather than constructing a replacement pin or vector mixin. The current player position does not identify corpse position. No map projection, native discovery, timer or ghost lifecycle is inferred. No Retail implementation exists to reuse; existing Era/Anniversary nil defaults remain outside this publication change.

## Proof scope

The selector alternates absent and supplied states, changes maps, removes availability, revives the player and retrieves the corpse. A constant nil/no-op cannot satisfy the supplied original pin assertions; a constant populated producer cannot satisfy initial absence or later removal. 

## Executed proof

| Boundary | Result |
|---|---|
| Initial test/schema `5ac4d6263` | Exact RED at original `DeathMapDataProvider.lua:26`, missing query |
| Producer `6c425dcab` | Exact GREEN: supplied coordinates reach original XML pin; map/state/retrieval transitions remove it |
| Producer-only RED `d796b7108` | Same original missing-call failure with byte-identical test blob `5fd3355ff281a5426d4aa7dbe1f4b53351caf040` |
| Mists and Retail all-target checks | Both pass with zero warnings in Cargo JSON and plain diagnostics |
| Formatting and changed-Rust readability | Pass |

Raw Mists stock stays at **12 keys / 18 occurrences**: the two corpse-query keys clear and two existing `C_Map.GetMapDisplayInfo` keys become visible, each count one. Original WorldMap OnLoad (`Cata/Blizzard_WorldMap.lua:112–126`) calls SetMapID, which reaches MapCanvas `OnMapChanged:562–565`. That loop uses `pairs`: the death provider inherits the base `OnMapChanged:74–76`, calling its refresh, while GroupMembers `OnMapChanged:22–23` calls the original pin's `OnMapChanged:112–114`, where `GetMapDisplayInfo` is absent. The second added key is ScriptErrors' re-report (`Blizzard_ScriptErrors.lua:71–73`). Before/after maps support this first-failure masking trace; no stable `pairs` ordering is claimed. Other common-key occurrence counts are unchanged. The residual API is outside this contract.

## Sources

- Pinned Mists `Blizzard_APIDocumentationGenerated/DeathInfoDocumentation.lua:5,11–22` — namespace, numeric map argument and nilable vector2 result.
- Pinned Mists WorldMap and SharedMapDataProviders files identified above — original consumer and XML selection.
- [Supplied state](../../../src/lua_api/state/sim_state.rs), [death predicate](../../../src/lua_api/globals/group_queries_relationships.rs), [corpse retrieval](../../../src/lua_api/globals/offer_verbs.rs).
- [Original-consumer test](../../../tests/mists_corpse_position.rs).

## See Also

- [Client profiles](client-profiles.md).
- [Consumer selection and RED proof](../investigations/consumer-selection-and-red-proof.md).
