//! Temporary debug/environment globals for partial Blizzard loads.
//!
//! These helpers are compatibility defaults for Blizzard Lua that expects the
//! client debug environment and widget metatable helpers to exist. They are not
//! modeled simulator state, so keep them out of the central runtime bootstrap.

const DEBUG_ENVIRONMENT_DEFAULTS_LUA: &str = r#"
if AddSourceLocationExclude == nil then
  function AddSourceLocationExclude()
  end
end

if GetGlobalEnvironment == nil then
  function GetGlobalEnvironment()
    return _G
  end
end

if GetButtonMetatable == nil then
  function GetButtonMetatable()
    if CreateFrame == nil then
      return nil
    end
    local frame = CreateFrame("Button")
    return frame and getmetatable(frame) or nil
  end
end

if GetEditBoxMetatable == nil then
  function GetEditBoxMetatable()
    if CreateFrame == nil then
      return nil
    end
    local frame = CreateFrame("EditBox")
    return frame and getmetatable(frame) or nil
  end
end

if secretwrap == nil then
  function secretwrap(fn)
    return fn
  end
end

if GetCallstackHeight == nil then
  function GetCallstackHeight()
    return 0
  end
end

if SetErrorCallstackHeight == nil then
  function SetErrorCallstackHeight()
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(DEBUG_ENVIRONMENT_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_debug_environment_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if GetGlobalEnvironment() ~= _G then return "global_environment" end
                if type(GetButtonMetatable()) ~= "table" then return "button_metatable" end
                if type(GetEditBoxMetatable()) ~= "table" then return "editbox_metatable" end
                local marker = function() return "wrapped" end
                if secretwrap(marker)() ~= "wrapped" then return "secretwrap" end
                if GetCallstackHeight() ~= 0 then return "callstack_height" end
                SetErrorCallstackHeight(4)
                AddSourceLocationExclude("example.lua")
                return "ok"
                "#,
            )
            .expect("debug environment defaults probe should run");

        assert_eq!(result, "ok");
    }
}
