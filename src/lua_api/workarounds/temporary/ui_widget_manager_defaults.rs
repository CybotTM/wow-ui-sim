//! Temporary UIWidgetManager defaults for partial addon loads.
//!
//! Full game loads get the real singleton frame and mixin from Blizzard_UIWidgets.
//! This fallback only keeps isolated loads from failing before that addon has
//! published `UIWidgetManager`.

const UI_WIDGET_MANAGER_DEFAULTS_LUA: &str = r#"
if UIWidgetManager == nil then
    UIWidgetManager = {}
end

if type(UIWidgetManager.RegisterWidgetVisTypeTemplate) ~= "function" then
    function UIWidgetManager:RegisterWidgetVisTypeTemplate()
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(UI_WIDGET_MANAGER_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    fn apply_again(env: &WowLuaEnv) {
        let mut lua = env.lua.borrow_mut();
        super::apply_bootstrap(&mut lua).expect("UIWidgetManager defaults should apply");
    }

    #[test]
    fn installs_minimal_ui_widget_manager_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (String, String, bool) = env
            .eval(
                r#"
                return type(UIWidgetManager),
                       type(UIWidgetManager.RegisterWidgetVisTypeTemplate),
                       UIWidgetManager:RegisterWidgetVisTypeTemplate(1, {}) == nil
                "#,
            )
            .expect("UIWidgetManager default probe should run");

        assert_eq!(result, ("table".to_string(), "function".to_string(), true));
    }

    #[test]
    fn preserves_existing_ui_widget_manager() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            UIWidgetManager = {
                RegisterWidgetVisTypeTemplate = function()
                    return "existing"
                end,
            }
            "#,
        )
        .expect("fixture should install existing UIWidgetManager");

        apply_again(&env);

        let value: String = env
            .eval("return UIWidgetManager:RegisterWidgetVisTypeTemplate(1, {})")
            .expect("UIWidgetManager preservation probe should run");

        assert_eq!(value, "existing");
    }
}
