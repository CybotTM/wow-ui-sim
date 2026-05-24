//! Temporary camera and tutorial setting defaults.
//!
//! The simulator does not model camera console defaults or tutorial account
//! settings yet. Keep these startup defaults explicit until those settings have
//! a backing state model.

const CAMERA_TUTORIAL_DEFAULTS_LUA: &str = r#"
if GetCameraFOVDefaults == nil then
  function GetCameraFOVDefaults()
    return 0, 30, 110
  end
end

if GetTutorialsEnabled == nil then
  function GetTutorialsEnabled()
    return false
  end
end

if IsTutorialFlagged == nil then
  function IsTutorialFlagged()
    return false
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(CAMERA_TUTORIAL_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_camera_tutorial_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local currentFov, minFov, maxFov = GetCameraFOVDefaults()
                if currentFov ~= 0 or minFov ~= 30 or maxFov ~= 110 then return "fov" end
                if GetTutorialsEnabled() ~= false then return "tutorials" end
                if IsTutorialFlagged(1) ~= false then return "tutorial_flagged" end
                return "ok"
                "#,
            )
            .expect("camera/tutorial defaults probe should run");

        assert_eq!(result, "ok");
    }
}
