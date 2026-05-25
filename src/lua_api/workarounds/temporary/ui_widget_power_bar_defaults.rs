//! Temporary UI widget power-bar defaults.
//!
//! `C_UIWidgetManager` has a narrow state-backed quest-pin surface. Power-bar
//! widget-set state is not modeled yet, so expose WoW's inert "no widget set"
//! answer as an explicit temporary default.

const UI_WIDGET_POWER_BAR_DEFAULTS_LUA: &str = r#"
C_UIWidgetManager = C_UIWidgetManager or __wow_namespace()
if rawget(C_UIWidgetManager, "GetPowerBarWidgetSetID") == nil then
    function C_UIWidgetManager.GetPowerBarWidgetSetID()
        return 0
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(UI_WIDGET_POWER_BAR_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_no_power_bar_widget_set_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let widget_set_id: i32 = env
            .eval("return C_UIWidgetManager.GetPowerBarWidgetSetID()")
            .expect("power-bar widget-set default should be callable");

        assert_eq!(widget_set_id, 0);
    }

    #[test]
    fn preserves_existing_power_bar_widget_set_function() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_UIWidgetManager.GetPowerBarWidgetSetID()
                return 42
            end
            "#,
        )
        .expect("fixture should install existing function");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let widget_set_id: i32 = env
            .eval("return C_UIWidgetManager.GetPowerBarWidgetSetID()")
            .expect("existing power-bar widget-set function should remain callable");

        assert_eq!(widget_set_id, 42);
    }
}
