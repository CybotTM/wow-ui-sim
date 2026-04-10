use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn game_rules_hardcore_flag_defaults_to_false() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(C_GameRules.IsHardcoreActive) ~= "function" then
                return "missing_is_hardcore_active"
            end
            if C_GameRules.IsHardcoreActive() then
                return "hardcore_should_default_false"
            end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "C_GameRules.IsHardcoreActive should exist and default to false"
    );
}
