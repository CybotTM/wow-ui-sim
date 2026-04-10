use wow_ui_sim::lua_api::WowLuaEnv;

const SCENARIO_INFO_SCRIPT: &str = r#"
    if C_ScenarioInfo.IsTieredEntranceScenario() ~= false then
        return "wrong_tiered_entrance_flag"
    end

    if C_ScenarioInfo.GetDisplayInfo() ~= nil then
        return "wrong_display_info"
    end

    if C_ScenarioInfo.GetTieredEntranceActiveSpells() ~= nil then
        return "wrong_active_spells"
    end

    return "ok"
"#;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn scenario_info_tiered_entrance_methods_match_no_active_scenario_state() {
    let env = env();
    let result: String = env
        .eval(SCENARIO_INFO_SCRIPT)
        .expect("C_ScenarioInfo tiered entrance methods should be callable");
    assert_eq!(result, "ok");
}
