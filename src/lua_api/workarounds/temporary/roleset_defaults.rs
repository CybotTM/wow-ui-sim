//! Temporary `C_Roleset` defaults.
//!
//! Frame roleset membership is stored on frames, but active role filtering is
//! not modeled yet. Keep the 12.1 namespace callable without changing frame
//! visibility until a roleset backend owns filtering semantics.

const ROLESET_DEFAULTS_LUA: &str = r#"
C_Roleset = C_Roleset or __wow_namespace()

if rawget(C_Roleset, "ApplyRolesetFilters") == nil then
    function C_Roleset.ApplyRolesetFilters()
        return true
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(ROLESET_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_roleset_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: bool = env
            .eval("return C_Roleset.ApplyRolesetFilters({})")
            .expect("roleset default should be callable");

        assert!(result);
    }

    #[test]
    fn preserves_existing_roleset_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_Roleset = {
                ApplyRolesetFilters = function()
                    return "existing"
                end,
            }
            "#,
        )
        .expect("fixture should install existing roleset provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: String = env
            .eval("return C_Roleset.ApplyRolesetFilters({})")
            .expect("existing roleset provider should remain callable");

        assert_eq!(result, "existing");
    }
}
