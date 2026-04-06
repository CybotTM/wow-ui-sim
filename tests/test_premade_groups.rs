use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

#[test]
fn get_search_result_info_returns_table() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local info = C_LFGList.GetSearchResultInfo(1)
            if type(info) ~= "table" then return "type=" .. type(info) end
            if info.name ~= "+15 Mists chill run" then return "name=" .. tostring(info.name) end
            if info.leaderName ~= "Thrallx" then return "leader=" .. tostring(info.leaderName) end
            if info.numMembers ~= 3 then return "num=" .. tostring(info.numMembers) end
            if info.maxMembers ~= 5 then return "max=" .. tostring(info.maxMembers) end
            if info.activityID ~= 1195 then return "activity=" .. tostring(info.activityID) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetSearchResultInfo: {result}");
}

#[test]
fn get_search_result_info_invalid_returns_nil() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local info = C_LFGList.GetSearchResultInfo(9999)
            return info == nil and "nil" or "not_nil"
            "#,
        )
        .unwrap();
    assert_eq!(result, "nil");
}
