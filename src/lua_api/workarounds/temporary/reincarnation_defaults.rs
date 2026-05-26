//! Temporary `C_Reincarnation` mutable defaults.
//!
//! Reincarnation state is not modeled by simulator state yet. Keep the small
//! compatibility surface in Lua workaround space until a real backing model
//! owns it.

const REINCARNATION_DEFAULTS_LUA: &str = r#"
C_Reincarnation = C_Reincarnation or __wow_namespace()

if rawget(C_Reincarnation, "_state") == nil then
    C_Reincarnation._state = {
        active = false,
        character = nil,
    }
end

local function copyCharacter(character)
    if character == nil then
        return {
            guid = "reincarnation-guid",
            name = "Reincarnating Character",
        }
    end

    if type(character) ~= "table" then
        return nil
    end

    return {
        guid = character.guid ~= nil and tostring(character.guid) or "",
        name = character.name ~= nil and tostring(character.name) or "",
    }
end

if rawget(C_Reincarnation, "IsReincarnating") == nil then
    function C_Reincarnation.IsReincarnating()
        return C_Reincarnation._state.active == true
    end
end

if rawget(C_Reincarnation, "GetReincarnatingCharacter") == nil then
    function C_Reincarnation.GetReincarnatingCharacter()
        return C_Reincarnation._state.character
    end
end

if rawget(C_Reincarnation, "StartReincarnation") == nil then
    function C_Reincarnation.StartReincarnation(character)
        if C_Reincarnation._state.active == true then
            return false
        end

        local nextCharacter = copyCharacter(character)
        if nextCharacter == nil then
            return false
        end

        C_Reincarnation._state.active = true
        C_Reincarnation._state.character = nextCharacter
        return true
    end
end

if rawget(C_Reincarnation, "StopReincarnation") == nil then
    function C_Reincarnation.StopReincarnation()
        if C_Reincarnation._state.active ~= true then
            return false
        end

        C_Reincarnation._state.active = false
        C_Reincarnation._state.character = nil
        return true
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(REINCARNATION_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_inactive_reincarnation_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (bool, bool, bool) = env
            .eval(
                r#"
                return C_Reincarnation.IsReincarnating(),
                       C_Reincarnation.GetReincarnatingCharacter() == nil,
                       C_Reincarnation.StopReincarnation()
                "#,
            )
            .expect("reincarnation defaults should be callable");

        assert_eq!(result, (false, true, false));
    }

    #[test]
    fn tracks_started_and_stopped_reincarnation_state() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (bool, bool, String, String, bool, bool, bool) = env
            .eval(
                r#"
                local started = C_Reincarnation.StartReincarnation({ guid = "guid-1", name = "Ari" })
                local character = C_Reincarnation.GetReincarnatingCharacter()
                local activeBeforeStop = C_Reincarnation.IsReincarnating()
                local duplicate = C_Reincarnation.StartReincarnation({ guid = "guid-2", name = "Bee" })
                local stopped = C_Reincarnation.StopReincarnation()
                return started,
                       activeBeforeStop,
                       character.guid,
                       character.name,
                       duplicate,
                       stopped,
                       C_Reincarnation.GetReincarnatingCharacter() == nil
                "#,
            )
            .expect("reincarnation state should be mutable");

        assert_eq!(
            result,
            (
                true,
                true,
                "guid-1".to_string(),
                "Ari".to_string(),
                false,
                true,
                true,
            )
        );
    }

    #[test]
    fn preserves_existing_reincarnation_provider_methods() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_Reincarnation.IsReincarnating()
                return "provider"
            end
            "#,
        )
        .expect("fixture should install provider method");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: (String, String) = env
            .eval(
                r#"
                return C_Reincarnation.IsReincarnating(),
                       type(C_Reincarnation.StartReincarnation)
                "#,
            )
            .expect("existing provider method should be preserved");

        assert_eq!(result, ("provider".to_string(), "function".to_string()));
    }
}
