use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_get_map_group_members_info_defaults_empty() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let (group_returns, members_type, members_count): (i32, String, i32) = env
        .eval(
            r##"
        local members = C_Map.GetMapGroupMembersInfo(12345)
        return select("#", C_Map.GetMapGroupID(2248)), type(members), #members
    "##,
        )
        .unwrap();
    assert_eq!(group_returns, 0);
    assert_eq!(members_type, "table");
    assert_eq!(members_count, 0);
}
