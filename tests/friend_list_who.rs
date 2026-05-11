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

            local shownWhos, totalWhos = C_FriendList.GetNumWhoResults()
            if shownWhos ~= 2 or totalWhos ~= 2 then
                return "expected_seeded_who_counts"
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

#[test]
fn friend_list_friend_info_uses_social_friend_state() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local first = C_FriendList.GetFriendInfoByIndex(1)
            if not first then
                return "expected_first_friend"
            end
            if first.name ~= "Arthax" then
                return "expected_social_friend_name:" .. tostring(first.name)
            end
            if first.guid ~= "Player-1-0000A001" then
                return "expected_social_friend_guid:" .. tostring(first.guid)
            end
            if not first.connected then
                return "expected_online_social_friend"
            end

            local byName = C_FriendList.GetFriendInfoByName("Durotan")
            if not byName then
                return "expected_name_lookup"
            end
            if byName.connected then
                return "expected_offline_social_friend"
            end
            if C_FriendList.GetFriendInfoByName("Alyth") ~= nil then
                return "hardcoded_friend_row_leaked"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "C_FriendList friend rows should be backed by SimState.social_friends"
    );
}
