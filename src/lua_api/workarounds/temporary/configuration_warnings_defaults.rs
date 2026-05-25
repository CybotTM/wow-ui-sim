//! Temporary `C_ConfigurationWarnings` defaults.
//!
//! Configuration warning state is not modeled yet. Keep the empty warning list
//! and local seen cache explicit here until a real settings/system-warning
//! backend owns this namespace.

const CONFIGURATION_WARNINGS_DEFAULTS_LUA: &str = r#"
C_ConfigurationWarnings = C_ConfigurationWarnings or __wow_namespace()
if type(rawget(C_ConfigurationWarnings, "__wow_seen_warnings")) ~= "table" then
    C_ConfigurationWarnings.__wow_seen_warnings = {}
end

local function installConfigurationWarningsDefault(name, fn)
    if rawget(C_ConfigurationWarnings, name) == nil then
        C_ConfigurationWarnings[name] = fn
    end
end

local function warningKey(warning)
    if warning == nil then
        return nil
    end
    return tostring(warning)
end

installConfigurationWarningsDefault("GetConfigurationWarningSeen", function(warning)
    local key = warningKey(warning)
    local seenWarnings = rawget(C_ConfigurationWarnings, "__wow_seen_warnings")
    return key ~= nil and seenWarnings[key] == true
end)

installConfigurationWarningsDefault("GetConfigurationWarningString", function()
    return nil
end)

installConfigurationWarningsDefault("GetConfigurationWarnings", function()
    return {}
end)

installConfigurationWarningsDefault("SetConfigurationWarningSeen", function(warning)
    local key = warningKey(warning)
    if key ~= nil then
        local seenWarnings = rawget(C_ConfigurationWarnings, "__wow_seen_warnings")
        seenWarnings[key] = true
    end
end)
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(CONFIGURATION_WARNINGS_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_empty_configuration_warning_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (String, Option<String>, bool) = env
            .eval(
                r#"
                local warnings = C_ConfigurationWarnings.GetConfigurationWarnings(false)
                return type(warnings),
                    C_ConfigurationWarnings.GetConfigurationWarningString("warning"),
                    C_ConfigurationWarnings.GetConfigurationWarningSeen("warning")
                "#,
            )
            .expect("configuration warning defaults should be callable");

        assert_eq!(result, ("table".to_string(), None, false));
    }

    #[test]
    fn stores_seen_warnings_in_local_cache() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let seen: bool = env
            .eval(
                r#"
                C_ConfigurationWarnings.SetConfigurationWarningSeen("warning")
                return C_ConfigurationWarnings.GetConfigurationWarningSeen("warning")
                "#,
            )
            .expect("configuration warning seen cache should be callable");

        assert!(seen);
    }

    #[test]
    fn preserves_existing_configuration_warning_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_ConfigurationWarnings.GetConfigurationWarnings()
                return { "existing" }
            end
            "#,
        )
        .expect("fixture should install existing configuration warning provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let warning: String = env
            .eval("return C_ConfigurationWarnings.GetConfigurationWarnings()[1]")
            .expect("existing configuration warning provider should remain callable");

        assert_eq!(warning, "existing");
    }
}
