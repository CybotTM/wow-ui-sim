use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn friend_list_get_who_info_returns_seeded_who_rows() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local info = C_FriendList.GetWhoInfo(1)
            if not info then
                return "expected_first_who_row"
            end
            if info.fullName ~= "Alyth" then
                return "expected_seeded_full_name"
            end
            if info.level ~= 80 then
                return "expected_seeded_level"
            end
            if info.classStr ~= "Paladin" or info.filename ~= "PALADIN" then
                return "expected_seeded_class_info"
            end
            if info.area ~= "Stormwind City" then
                return "expected_seeded_area"
            end
            if info.raceStr ~= "Human" or info.gender ~= 2 then
                return "expected_seeded_race_and_gender"
            end
            if info.fullGuildName ~= "Heroes of Azeroth" then
                return "expected_seeded_guild_name"
            end

            if C_FriendList.GetWhoInfo(99) ~= nil then
                return "out_of_range_who_row_should_be_nil"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "C_FriendList.GetWhoInfo should expose seeded WhoInfo rows"
    );
}
