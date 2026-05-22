use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn restricted_actions_defaults_are_permissive() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_RestrictedActions.CheckAllowProtectedFunctions() ~= true then
                return "protected"
            end
            if C_RestrictedActions.GetAddOnRestrictionState(Enum.AddOnRestrictionType.Combat) ~= 0 then
                return "combat"
            end
            return "ok"
            "#,
        )
        .expect("restricted action defaults should be callable");

    assert_eq!(result, "ok");
}
