use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn c_tutorial_flags_round_trip_and_documented_helpers_exist() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(C_Tutorial) ~= "table" then
                return "missing_namespace"
            end
            if type(C_Tutorial.GetTutorialStatus) ~= "function" then
                return "missing_get_tutorial_status"
            end
            if type(C_Tutorial.SetTutorialFlag) ~= "function" then
                return "missing_set_tutorial_flag"
            end
            if type(C_Tutorial.AbandonTutorialArea) ~= "function" then
                return "missing_abandon_tutorial_area"
            end
            if type(C_Tutorial.ReturnToTutorialArea) ~= "function" then
                return "missing_return_to_tutorial_area"
            end
            if type(C_Tutorial.GetCombatEventInfo) ~= "function" then
                return "missing_get_combat_event_info"
            end

            if C_Tutorial.HasSeenTutorial(17) then
                return "fresh_env_should_start_unseen"
            end
            if C_Tutorial.GetTutorialStatus(17) then
                return "status_should_start_false"
            end

            C_Tutorial.SetTutorialFlag(17)
            if not C_Tutorial.HasSeenTutorial(17) or not C_Tutorial.GetTutorialStatus(17) then
                return "set_flag_should_mark_seen"
            end

            C_Tutorial.SetTutorialFlag(17, false)
            if C_Tutorial.HasSeenTutorial(17) or C_Tutorial.GetTutorialStatus(17) then
                return "set_flag_false_should_clear"
            end

            C_Tutorial.AcknowledgeTutorial(42)
            if not C_Tutorial.HasSeenTutorial(42) or not C_Tutorial.GetTutorialStatus(42) then
                return "acknowledge_should_mark_seen"
            end

            C_Tutorial.ReturnToTutorialArea()
            C_Tutorial.AbandonTutorialArea()

            local combatInfo = { C_Tutorial.GetCombatEventInfo() }
            if #combatInfo ~= 0 then
                return "combat_event_info_should_default_empty"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "C_Tutorial should expose state-backed tutorial flags"
    );
}
