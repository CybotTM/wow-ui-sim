use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn quest_log_watch_surface_remains_state_backed_after_bootstrap_defaults_removed() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_QuestLog.ReadyForTurnIn(1) ~= false then return "turn-in" end
            if type(C_QuestLog.GetNumWorldQuestWatches()) ~= "number" then return "world-count" end
            if C_QuestLog.GetQuestIDForWorldQuestWatchIndex(1) ~= nil then return "world-id" end
            if type(C_QuestLog.GetNumQuestWatches()) ~= "number" then return "quest-count" end
            for index = 1, C_QuestLog.GetNumQuestWatches() do
                if type(C_QuestLog.GetQuestIDForQuestWatchIndex(index)) ~= "number" then
                    return "quest-id"
                end
            end

            local shown, total = C_QuestLog.GetNumQuestLogEntries()
            if type(shown) ~= "number" or type(total) ~= "number" then return "state-backed" end
            return "ok"
            "#,
        )
        .expect("quest log defaults should be callable");

    assert_eq!(result, "ok");
}
