# Mists corpse map position

The [contract](../../specs/mists-corpse-map-position.md) binds a supplied corpse observation to the original Mists WorldMap provider. No native corpse coordinates are seeded.

## Selected consumer

`Blizzard_WorldMap_Mists.toc:3–5,12–13` selects game/Mists, requires MapCanvas and SharedMapDataProviders, and loads the Cata WorldMap Lua/XML. `Cata/Blizzard_WorldMap.lua:206` installs `DeathMapDataProviderMixin`. `Blizzard_SharedMapDataProviders_Mists.toc:4–5,25–26` selects the original `DeathMapDataProvider.lua` and **Wrath** `DeathMapDataProvider.xml`, not the root XML. That selected XML defines `CorpsePinTemplate` using `CorpsePinMixin`.

The original refresh removes old pins, calls `C_DeathInfo.GetCorpseMapPosition` at line 26, then acquires, positions and shows a corpse pin only for a non-nil vector. The test loads stock startup discovery order and locates the provider installed by original OnLoad; it does not substitute a provider or pin.

## State boundary

Existing `corpse_available` represents corpse availability. The existing death predicate treats player health at or below zero as dead and has no separate ghost state (`group_queries_relationships.rs`). Supplied `corpse_map_positions` stores independent coordinates by UI map. The query returns one nil while alive, without availability, or without an observation for the requested map. `RetrieveCorpse` already clears availability and restores health, so the query observes that transition without changing the verb.

The current player position does not identify corpse position. No map projection, native discovery, timer or ghost lifecycle is inferred. No Retail implementation exists to reuse; existing Era/Anniversary nil defaults remain outside this publication change.

## Proof scope

The selector alternates absent and supplied states, changes maps, removes availability, revives the player and retrieves the corpse. A constant nil/no-op cannot satisfy the supplied original pin assertions; a constant populated producer cannot satisfy initial absence or later removal. Runtime RED/GREEN and stock proof are pending.

## Sources

- Pinned Mists `Blizzard_APIDocumentationGenerated/DeathInfoDocumentation.lua:5,11–22` — namespace, numeric map argument and nilable vector2 result.
- Pinned Mists WorldMap and SharedMapDataProviders files identified above — original consumer and XML selection.
- [Supplied state](../../../src/lua_api/state/sim_state.rs), [death predicate](../../../src/lua_api/globals/group_queries_relationships.rs), [corpse retrieval](../../../src/lua_api/globals/offer_verbs.rs).
- [Original-consumer test](../../../tests/mists_corpse_position.rs).

## See Also

- [Client profiles](client-profiles.md).
- [Consumer selection and RED proof](../investigations/consumer-selection-and-red-proof.md).
