# Mists battlefield vehicle consumer

The selected Mists WorldMap reaches an unpublished `C_PvP.GetBattlefieldVehicles` while refreshing its vehicle data provider. The [contract](../../specs/mists-battlefield-vehicles.md) limits this repair to supplied observations and Mists publication.

## Selected original path

Verified source: 2026-10-25, Mists profile-scoped Blizzard UI cache.

`Blizzard_WorldMap_Mists.toc:3,12–13` requires MapCanvas and SharedMapDataProviders and selects the Cata WorldMap Lua/XML. `Cata/Blizzard_WorldMap.lua:202` adds `VehicleDataProviderMixin`. `Blizzard_SharedMapDataProviders_Mists.toc:4–5,21` selects the game/Mists provider XML, which loads `VehicleDataProvider.lua` and declares `VehiclePinTemplate` with `VehiclePinMixin`.

`Blizzard_MapCanvas.lua:451–454` iterates data providers with `pairs`. Vehicle refresh removes old pins, accepts zone-or-more-detailed map types, calls `GetBattlefieldVehicles(mapID)` at line 31, and acquires pins for alive non-player records with coordinates and an atlas. `VehiclePinMixin:Refresh` then needs `GetBattlefieldVehicleInfo` at line 58 to set rotation, atlas, dimensions, name, frame-level category and coordinates. OnUpdate queries that same record again. Fixing only the plural call would leave the supplied positive consumer incomplete.

## State boundary

No existing plural or singular producer exists in the simulator, including Retail. The existing queue models membership/status, not map vehicle observations. The existing `is_active_battlefield` flag represents battlefield presence. A per-map list of supplied records belongs to `c_api::c_pvp`; the simulator starts with no observations. List order is the 1-based pin index. Queries must not synthesize positions, atlases or names.

## Proof status

The test confirms selected dependency TOCs, loads stock startup discovery order, locates the provider installed by the original WorldMap OnLoad, checks cached Lua provenance, and invokes that original refresh directly so dispatch cannot swallow the missing-call failure. Loading only the dependency closure omitted implicit SharedXML startup dependencies and failed before the vehicle boundary; that first attempt is setup failure, not API RED. The corrected fixture reaches the original missing plural call. Its single selector includes empty and positive states: a constant empty/no-op producer contradicts the one-pin supplied state, while a constant populated producer contradicts the initial empty state. The original-consumer GREEN passes with all eleven projected fields and real XML pin acquisition/removal. The raw supplied-state before/after captures are both 12 keys / 18 occurrences: two missing-vehicle keys disappear and two existing `C_DeathInfo.GetCorpseMapPosition` keys appear. Each added key traces to `DeathMapDataProvider.lua:26`, selected at Cata WorldMap:206 and visited by MapCanvas:451–454; the second key is Blizzard ScriptErrors' re-report. No common-key count changes. This is not a zero-additions claim. A fresh build of exact base `05f95ab1c` also produces 12/18, with the raw message-count map identical to the supplied-schema BEFORE. The caller's historical 14/20 is not reproduced with these current inputs; no counts are normalized. The paired producer-only RED removes only the two C API source producers, retains identical test bytes, and fails again at the original plural call. Mists all-target checking passes with only the three inherited `fast_handler_late_bind` warnings. Retail all-target checking passes without warnings. Formatting and the changed-Rust readability audit pass. These checks cover unchanged runtime/test bytes across the later documentation commits; no full-suite or native vehicle-population claim is made.

## Sources

- Pinned Mists `PvpInfoDocumentation.lua:71–99,760–775` — signatures, `MayReturnNothing`, eleven fields.
- Selected Mists WorldMap and SharedMapDataProviders TOCs, original provider Lua/XML and MapCanvas — consumer chain above.
- [Feature contract](../../specs/mists-battlefield-vehicles.md)
- [Consumer selection and RED proof](consumer-selection-and-red-proof.md)

## See Also

- [Map display input](map-display-info-model.md) — another independent WorldMap provider boundary.
