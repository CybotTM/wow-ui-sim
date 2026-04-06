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

#[test]
fn get_search_results_returns_ids() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local numResults, results = C_LFGList.GetSearchResults()
            if numResults ~= 8 then return "count=" .. tostring(numResults) end
            if type(results) ~= "table" then return "type=" .. type(results) end
            if results[1] ~= 1 then return "first=" .. tostring(results[1]) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetSearchResults: {result}");
}

#[test]
fn search_fires_event() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("LFG_LIST_SEARCH_RESULTS_RECEIVED")
            f:SetScript("OnEvent", function(self, event)
                if event == "LFG_LIST_SEARCH_RESULTS_RECEIVED" then fired = true end
            end)
            C_LFGList.Search(2)
            return fired and "ok" or "not_fired"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Search should fire event: {result}");
}

#[test]
fn get_activity_info_table() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local info = C_LFGList.GetActivityInfoTable(1195)
            if type(info) ~= "table" then return "type=" .. type(info) end
            if info.activityID ~= 1195 then return "id=" .. tostring(info.activityID) end
            if info.maxPlayers ~= 5 then return "max=" .. tostring(info.maxPlayers) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetActivityInfoTable: {result}");
}
