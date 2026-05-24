//! Temporary gamepad cursor-control defaults.
//!
//! The simulator has per-frame gamepad button/stick state, but no global
//! gamepad cursor-control model yet. Keep these inert globals explicit until
//! cursor-control input state exists.

const GAMEPAD_CURSOR_CONTROL_DEFAULTS_LUA: &str = r#"
if CanAutoSetGamePadCursorControl == nil then
  function CanAutoSetGamePadCursorControl(_enabled)
    return false
  end
end

if SetGamePadCursorControl == nil then
  function SetGamePadCursorControl(_enabled)
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(GAMEPAD_CURSOR_CONTROL_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_gamepad_cursor_control_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if CanAutoSetGamePadCursorControl(true) ~= false then return "auto" end
                if pcall(SetGamePadCursorControl, true) ~= true then return "set" end
                return "ok"
                "#,
            )
            .expect("gamepad cursor-control defaults probe should run");

        assert_eq!(result, "ok");
    }
}
