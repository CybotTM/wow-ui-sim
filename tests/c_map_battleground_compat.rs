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
