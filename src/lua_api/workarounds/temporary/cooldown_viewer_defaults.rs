//! Temporary `C_CooldownViewer` defaults.
//!
//! Cooldown viewer category/cooldown state is not modeled yet. These empty
//! defaults keep the Blizzard cooldown-viewer UI loadable until a real backend
//! owns the namespace.

const COOLDOWN_VIEWER_DEFAULTS_LUA: &str = r#"
C_CooldownViewer = C_CooldownViewer or __wow_namespace()

local function installCooldownViewerDefault(name, fn)
    if rawget(C_CooldownViewer, name) == nil then
        C_CooldownViewer[name] = fn
    end
end

installCooldownViewerDefault("GetCooldownViewerCategorySet", function()
    return {}
end)

installCooldownViewerDefault("GetCooldownViewerCooldownInfo", function()
    return nil
end)

if type(GetBuildInfo) == "function" and select(4, GetBuildInfo()) >= 120100 then
    installCooldownViewerDefault("GetGroupBuffItems", function()
        return {}
    end)
end

installCooldownViewerDefault("GetCooldownID", function()
    return nil
end)
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(COOLDOWN_VIEWER_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_empty_cooldown_viewer_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (i32, bool, bool) = env
            .eval(
                r#"
                return #C_CooldownViewer.GetCooldownViewerCategorySet(),
                    C_CooldownViewer.GetCooldownViewerCooldownInfo() == nil,
                    C_CooldownViewer.GetCooldownID() == nil
                "#,
            )
            .expect("cooldown viewer defaults should be callable");

        assert_eq!(result, (0, true, true));
    }

    #[test]
    fn preserves_existing_cooldown_viewer_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_CooldownViewer.GetCooldownViewerCategorySet()
                return { "existing" }
            end
            "#,
        )
        .expect("fixture should install existing cooldown viewer provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let first_category: String = env
            .eval("return C_CooldownViewer.GetCooldownViewerCategorySet()[1]")
            .expect("existing cooldown viewer provider should remain callable");

        assert_eq!(first_category, "existing");
    }
}
