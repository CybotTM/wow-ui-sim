//! Temporary `C_AddOns` beta-policy defaults.
//!
//! The simulator has no beta-realm script policy state yet. Retail startup only
//! needs the non-beta answer, so keep this inert compatibility default outside
//! the state-backed `C_AddOns` API implementation.

const ADDONS_BETA_POLICY_DEFAULTS_LUA: &str = r#"
C_AddOns = C_AddOns or __wow_namespace()
if rawget(C_AddOns, "GetScriptsDisallowedForBeta") == nil then
    function C_AddOns.GetScriptsDisallowedForBeta()
        return false
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(ADDONS_BETA_POLICY_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_non_beta_policy_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: bool = env
            .eval("return C_AddOns.GetScriptsDisallowedForBeta()")
            .expect("beta policy default should be callable");

        assert!(!result);
    }

    #[test]
    fn preserves_existing_beta_policy_function() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_AddOns.GetScriptsDisallowedForBeta()
                return true
            end
            "#,
        )
        .expect("fixture should install existing function");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: bool = env
            .eval("return C_AddOns.GetScriptsDisallowedForBeta()")
            .expect("existing beta policy function should remain callable");

        assert!(result);
    }
}
