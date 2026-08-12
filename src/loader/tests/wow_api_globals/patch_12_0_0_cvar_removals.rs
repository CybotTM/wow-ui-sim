use crate::lua_api::WowLuaEnv;

#[test]
fn test_patch_12_0_0_force_allow_aero_cvar_removed() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
                if GetCVar("ForceAllowAero") ~= nil then
                    return "value=" .. tostring(GetCVar("ForceAllowAero"))
                end
                if GetCVarDefault("ForceAllowAero") ~= nil then
                    return "default=" .. tostring(GetCVarDefault("ForceAllowAero"))
                end
                return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "retail 12.0.0 removed ForceAllowAero CVar should have no value or default"
    );
}
