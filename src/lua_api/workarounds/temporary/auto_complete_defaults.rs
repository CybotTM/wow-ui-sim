//! Temporary autocomplete realm defaults.
//!
//! Realm/name completion data is not modeled yet. Blizzard's deprecated global
//! forwards to `C_AutoComplete`, so both surfaces return empty realm lists until
//! completion data has backing simulator state.

const AUTO_COMPLETE_DEFAULTS_LUA: &str = r#"
C_AutoComplete = C_AutoComplete or __wow_namespace()
if rawget(C_AutoComplete, "GetAutoCompleteRealms") == nil then
    function C_AutoComplete.GetAutoCompleteRealms()
        return {}
    end
end
if GetAutoCompleteRealms == nil then
    function GetAutoCompleteRealms()
        return C_AutoComplete.GetAutoCompleteRealms()
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(AUTO_COMPLETE_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_empty_realm_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let (namespace_count, global_count): (i32, i32) = env
            .eval(
                r#"
                local namespaceRealms = C_AutoComplete.GetAutoCompleteRealms()
                local globalRealms = GetAutoCompleteRealms()
                return #namespaceRealms, #globalRealms
                "#,
            )
            .expect("autocomplete realm defaults should be callable");

        assert_eq!(namespace_count, 0);
        assert_eq!(global_count, 0);
    }

    #[test]
    fn preserves_existing_autocomplete_realms() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_AutoComplete.GetAutoCompleteRealms()
                return { "modeled-realm" }
            end
            function GetAutoCompleteRealms()
                return { "legacy-modeled-realm" }
            end
            "#,
        )
        .expect("fixture should install existing functions");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let (namespace_realm, global_realm): (String, String) = env
            .eval(
                r#"
                return C_AutoComplete.GetAutoCompleteRealms()[1],
                    GetAutoCompleteRealms()[1]
                "#,
            )
            .expect("existing autocomplete realm functions should remain callable");

        assert_eq!(namespace_realm, "modeled-realm");
        assert_eq!(global_realm, "legacy-modeled-realm");
    }
}
