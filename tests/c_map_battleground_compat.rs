use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn get_map_info_has_battleground_names_used_by_addons() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let result: String = env
        .eval(
            r#"
        local expected = {
            [93] = "Arathi Basin",
            [91] = "Alterac Valley",
            [121] = "Eye of the Storm",
            [169] = "Isle of Conquest",
            [128] = "Strand of the Ancients",
        }

        for mapID, name in pairs(expected) do
            local info = C_Map.GetMapInfo(mapID)
            if not info then
                return "missing_map=" .. tostring(mapID)
            end
            if info.name ~= name then
                return "map_name=" .. tostring(mapID) .. ":" .. tostring(info.name)
            end
        end

        return "ok"
    "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "C_Map.GetMapInfo should expose battleground names used by addon localization: {result}"
    );
}

#[test]
fn get_map_info_has_bounty_helper_waypoint_maps() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let result: String = env
        .eval(
            r#"
        local expected = {
            23, 42, 50, 70, 75, 108, 109, 114, 117, 118, 120, 122, 123,
            198, 207, 249, 379, 390, 504, 534, 543, 646, 680, 862, 863,
            885, 895, 1462, 1527, 1530, 1533, 1543, 1550, 1970, 2200,
            2214, 2255, 2346, 2371, 2395, 2424,
        }

        for _, mapID in ipairs(expected) do
            local info = C_Map.GetMapInfo(mapID)
            if not info then
                return "missing_map=" .. tostring(mapID)
            end
            if type(info.name) ~= "string" or info.name == "" then
                return "missing_name=" .. tostring(mapID)
            end
        end

        return "ok"
    "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "C_Map.GetMapInfo should expose waypoint maps used by addon startup code: {result}"
    );
}
