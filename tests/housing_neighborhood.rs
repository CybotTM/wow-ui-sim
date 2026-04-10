use wow_ui_sim::lua_api::WowLuaEnv;

const HOUSING_NEIGHBORHOOD_SCRIPT: &str = r#"
    local houseInfo = C_HousingNeighborhood.GetCornerstoneHouseInfo()
    if not houseInfo or houseInfo.plotID ~= 27 or houseInfo.ownerName ~= "Simhero" or houseInfo.houseName ~= "Sunspire Retreat" then
        return "wrong_initial_house_info"
    end

    local neighborhoodInfo = C_HousingNeighborhood.GetCornerstoneNeighborhoodInfo()
    if not neighborhoodInfo or neighborhoodInfo.neighborhoodName ~= "Dawnmeadow" or neighborhoodInfo.neighborhoodType ~= "Public" then
        return "wrong_initial_neighborhood_info"
    end

    C_HousingNeighborhood.OnCornerstoneClosed()

    if C_HousingNeighborhood.GetCornerstoneHouseInfo() ~= nil then
        return "house_info_not_cleared"
    end

    if C_HousingNeighborhood.GetCornerstoneNeighborhoodInfo() ~= nil then
        return "neighborhood_info_not_cleared"
    end

    C_HousingNeighborhood.OnCornerstoneClosed()

    if C_HousingNeighborhood.GetCornerstoneHouseInfo() ~= nil then
        return "house_info_should_stay_cleared"
    end

    return "ok"
"#;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn housing_neighborhood_cornerstone_close_clears_cornerstone_state() {
    let env = env();
    let result: String = env
        .eval(HOUSING_NEIGHBORHOOD_SCRIPT)
        .expect("seeded C_HousingNeighborhood methods should be queryable");
    assert_eq!(result, "ok");
}
