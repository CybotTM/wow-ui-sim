//! Temporary `C_RestrictedActions` defaults.
//!
//! The simulator does not enforce restricted-action policy yet. These defaults
//! keep Blizzard tooltip and addon code permissive until a real policy model
//! owns the namespace.

const RESTRICTED_ACTIONS_DEFAULTS_LUA: &str = r#"
C_RestrictedActions = C_RestrictedActions or __wow_namespace()

if rawget(C_RestrictedActions, "CheckAllowProtectedFunctions") == nil then
    function C_RestrictedActions.CheckAllowProtectedFunctions()
        return true
    end
end

if rawget(C_RestrictedActions, "GetAddOnRestrictionState") == nil then
    function C_RestrictedActions.GetAddOnRestrictionState(_restrictionType)
        return 0
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(RESTRICTED_ACTIONS_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_permissive_restricted_action_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (bool, i32) = env
            .eval(
                r#"
                return C_RestrictedActions.CheckAllowProtectedFunctions(),
                       C_RestrictedActions.GetAddOnRestrictionState(Enum.AddOnRestrictionType.Combat)
                "#,
            )
            .expect("restricted action defaults should be callable");

        assert_eq!(result, (true, 0));
    }

    #[test]
    fn preserves_existing_restricted_action_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_RestrictedActions.CheckAllowProtectedFunctions()
                return false
            end
            function C_RestrictedActions.GetAddOnRestrictionState()
                return 7
            end
            "#,
        )
        .expect("fixture should install existing restricted-action provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: (bool, i32) = env
            .eval(
                r#"
                return C_RestrictedActions.CheckAllowProtectedFunctions(),
                       C_RestrictedActions.GetAddOnRestrictionState()
                "#,
            )
            .expect("existing restricted-action provider should remain callable");

        assert_eq!(result, (false, 7));
    }
}
