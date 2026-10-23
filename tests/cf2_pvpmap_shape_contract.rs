//! Shape-only contracts; native semantics unknown. Source predictions are not runtime proof.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]

#[path = "common/cf2_shape.rs"]
mod shape;

// retail: Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:987 (IsMatchComplete).
// ptr: Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:946 (IsMatchComplete).
// wowforever: Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:987 (IsMatchComplete).
// Source-predicted FAIL: IsMatchComplete missing; boolean and original event acceptance only; not executed; native semantics unknown.
#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]
#[test]
fn original_match_enter_world_shape_blocker() {
    let env = shape::load(&["Blizzard_PVPMatch"]);
    // Inactive is an input prerequisite to avoid the original OR short-circuit.
    env.state().borrow_mut().pvp_match_active = false;
    shape::check(
        &env,
        r#"

scalar('boolean', C_PvP.IsMatchComplete())
original(PVPMatchResultsMixin.OnEvent, 'Blizzard_PVPMatch/PVPMatchResults.lua')
local frame = CreateFrame('Frame', nil, UIParent)
Mixin(frame, PVPMatchResultsMixin)
frame:OnEvent('PLAYER_ENTERING_WORLD')

"#,
    );
}

// retail: Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:300/283/1660 (vehicle getters/record).
// ptr: Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:281/264/1619 (vehicle getters/record).
// wowforever: Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:300/283/1670 (vehicle getters/record).
// era/anniversary: Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:96/80/769 (vehicle getters/record).
// All six profiles declare both queries MayReturnNothing and the same record fields.
// WorldMap requires SharedMapDataProviders; selected Mainline/Classic TOCs include VehicleDataProvider.xml.
// mists: Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:87 (GetBattlefieldVehicles).
// mists: Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:71 (GetBattlefieldVehicleInfo).
// mists: Blizzard_APIDocumentationGenerated/PvpInfoDocumentation.lua:760 (BattlefieldVehicleInfo).
// Source-predicted FAIL: vehicle getters missing; optional table or zero returns, actual records only; not executed; native semantics unknown.
#[test]
fn original_vehicle_provider_shape_blocker() {
    let env = shape::load(&["Blizzard_WorldMap"]);
    shape::check(
        &env,
        r#"

original(VehicleDataProviderMixin.RefreshAllData, 'Blizzard_SharedMapDataProviders/VehicleDataProvider.lua')
WorldMapFrame:SetMapID(84)
local mapID = WorldMapFrame:GetMapID()
assert(type(mapID) == 'number', 'normal map prerequisite')
local mapInfo = C_Map.GetMapInfo(mapID)
assert(mapInfo and mapInfo.mapType >= Enum.UIMapType.Zone, 'normal zone prerequisite')
local schema = {{'x','number',false}, {'y','number',false}, {'name','string',false},
    {'isOccupied','boolean',false}, {'atlas','string',false},
    {'textureWidth','number',false}, {'textureHeight','number',false},
    {'facing','number',false}, {'isPlayer','boolean',false}, {'isAlive','boolean',false},
    {'shouldDrawBelowPlayerBlips','boolean',false}}
local result = pack(C_PvP.GetBattlefieldVehicles(mapID))
assert(result.n == 0 or (result.n == 1 and type(result[1]) == 'table'))
local provider = CreateFromMixins(VehicleDataProviderMixin)
provider:OnAdded(WorldMapFrame)
provider:RefreshAllData()
if result.n == 1 then
    for index, vehicle in ipairs(result[1]) do
        fields(vehicle, schema)
        local indexed = pack(C_PvP.GetBattlefieldVehicleInfo(index, mapID))
        -- Local declaration also marks the indexed query MayReturnNothing.
        assert(indexed.n == 0 or (indexed.n == 1 and type(indexed[1]) == 'table'))
        if indexed.n == 1 then
            fields(indexed[1], schema)
            local pin = WorldMapFrame:AcquirePin('VehiclePinTemplate', index)
            original(pin.Refresh, 'Blizzard_SharedMapDataProviders/VehicleDataProvider.lua')
            original(pin.OnUpdate, 'Blizzard_SharedMapDataProviders/VehicleDataProvider.lua')
            pin:Refresh()
            pin:OnUpdate()
        else
            print('conditional indexed-pin coverage blocked: no indexed result')
        end
    end
end

"#,
    );
}
