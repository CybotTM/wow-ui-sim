//! Temporary autocomplete realm defaults and legacy global forwarders.
//!
//! Realm completion data is not modeled yet, so realm probes return an empty
//! list. Character/name completion is modeled in `c_api::c_auto_complete`; this
//! bootstrap only exposes the deprecated global forwarder in raw `WowLuaEnv`
//! tests where Blizzard_DeprecatedAutoComplete has not been loaded.

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
if GetAutoCompleteResults == nil then
    function GetAutoCompleteResults(name, numResults, cursorPosition, allowFullMatch, includeFlags, excludeFlags)
        return C_AutoComplete.GetAutoCompleteResults(name, numResults, cursorPosition, not not allowFullMatch, includeFlags, excludeFlags)
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
    fn installs_legacy_results_forwarder() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        env.exec(
            r#"
            GetAutoCompleteResults = nil
            function C_AutoComplete.GetAutoCompleteResults(_name, _numResults, _cursorPosition, allowFullMatch)
                return { allowFullMatch and "coerced" or "not-coerced" }
            end
            "#,
        )
        .expect("fixture should install modeled autocomplete result function");
        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: String = env
            .eval(
                r#"
                return GetAutoCompleteResults("name", 1, 0, 1, nil, nil)[1]
                "#,
            )
            .expect("legacy autocomplete result function should forward to C_AutoComplete");

        assert_eq!(result, "coerced");
    }

    #[test]
    fn preserves_existing_legacy_results_forwarder() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        env.exec(
            r#"
            function GetAutoCompleteResults()
                return { "legacy-modeled-result" }
            end
            "#,
        )
        .expect("fixture should install legacy autocomplete result function");
        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: String = env
            .eval(
                r#"
                return GetAutoCompleteResults("name", 1, 0, true, nil, nil)[1]
                "#,
            )
            .expect("existing legacy autocomplete result function should remain callable");

        assert_eq!(result, "legacy-modeled-result");
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
