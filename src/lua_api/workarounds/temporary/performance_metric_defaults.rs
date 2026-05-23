//! Temporary framerate and performance metric globals.
//!
//! The simulator does not model addon CPU/memory accounting yet. These globals
//! are safe startup defaults for Blizzard performance UI callers.

const PERFORMANCE_METRIC_DEFAULTS_LUA: &str = r#"
if GetFramerate == nil then
  function GetFramerate()
    return 60
  end
end

if UpdateAddOnMemoryUsage == nil then
  function UpdateAddOnMemoryUsage()
    return 0
  end
end

if UpdateAddOnCPUUsage == nil then
  function UpdateAddOnCPUUsage()
    return 0
  end
end

if ResetCPUUsage == nil then
  function ResetCPUUsage()
    return 0
  end
end

if GetAddOnMemoryUsage == nil then
  function GetAddOnMemoryUsage(_name)
    return 0
  end
end

if GetAddOnCPUUsage == nil then
  function GetAddOnCPUUsage(_name)
    return 0
  end
end

if GetFrameCPUUsage == nil then
  function GetFrameCPUUsage(_frame, _includeChildren)
    return 0, 0
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(PERFORMANCE_METRIC_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_performance_metric_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if GetFramerate() ~= 60 then return "framerate" end
                if UpdateAddOnMemoryUsage() ~= 0 then return "update_memory" end
                if UpdateAddOnCPUUsage() ~= 0 then return "update_cpu" end
                if ResetCPUUsage() ~= 0 then return "reset_cpu" end
                if GetAddOnMemoryUsage("missing") ~= 0 then return "addon_memory" end
                if GetAddOnCPUUsage("missing") ~= 0 then return "addon_cpu" end
                local frame, children = GetFrameCPUUsage(UIParent, true)
                if frame ~= 0 or children ~= 0 then return "frame_cpu" end
                return "ok"
                "#,
            )
            .expect("performance metric defaults probe should run");

        assert_eq!(result, "ok");
    }
}
