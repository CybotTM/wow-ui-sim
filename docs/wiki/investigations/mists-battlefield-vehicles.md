# Mists battlefield vehicle consumer

The selected Mists WorldMap reaches an unpublished `C_PvP.GetBattlefieldVehicles` while refreshing its vehicle data provider. The [contract](../../specs/mists-battlefield-vehicles.md) limits this repair to supplied observations and Mists publication.

## Selected original path

Verified source: 2026-10-25, Mists profile-scoped Blizzard UI cache.

`Blizzard_WorldMap_Mists.toc:3,12–13` requires MapCanvas and SharedMapDataProviders and selects the Cata WorldMap Lua/XML. `Cata/Blizzard_WorldMap.lua:202` adds `VehicleDataProviderMixin`. `Blizzard_SharedMapDataProviders_Mists.toc:4–5,21` selects the game/Mists provider XML, which loads `VehicleDataProvider.lua` and declares `VehiclePinTemplate` with `VehiclePinMixin`.

`Blizzard_MapCanvas.lua:451–454` iterates data providers with `pairs`. Vehicle refresh removes old pins, accepts zone-or-more-detailed map types, calls `GetBattlefieldVehicles(mapID)` at line 31, and acquires pins for alive non-player records with coordinates and an atlas. `VehiclePinMixin:Refresh` then needs `GetBattlefieldVehicleInfo` at line 57 to set rotation, atlas, dimensions, name, frame-level category and coordinates. OnUpdate queries that same record again. Fixing only the plural call would leave the supplied positive consumer incomplete.

## State boundary

No existing plural or singular producer exists in the simulator, including Retail. The existing queue models membership/status, not map vehicle observations. The existing `is_active_battlefield` flag represents battlefield presence. A per-map list of supplied records belongs to `c_api::c_pvp`; the simulator starts with no observations. List order is the 1-based pin index. Queries must not synthesize positions, atlases or names.

## Proof status

The test loads the selected dependency closure, locates the provider installed by the original WorldMap OnLoad, checks cached Lua provenance, and invokes that original refresh directly so dispatch cannot swallow the missing-call failure. Its single selector includes both empty and positive states, rejecting a constant empty/no-op producer. Runtime proof remains pending until recorded in the task report.

## Sources

- Pinned Mists `PvpInfoDocumentation.lua:71–99,760–775` — signatures, `MayReturnNothing`, eleven fields.
- Selected Mists WorldMap and SharedMapDataProviders TOCs, original provider Lua/XML and MapCanvas — consumer chain above.
- [Feature contract](../../specs/mists-battlefield-vehicles.md)
- [Consumer selection and RED proof](consumer-selection-and-red-proof.md)

## See Also

- [Map display input](map-display-info-model.md) — another independent WorldMap provider boundary.
