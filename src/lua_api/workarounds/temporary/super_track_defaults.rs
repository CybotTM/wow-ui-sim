//! Temporary `C_SuperTrack` no-active-target defaults.
//!
//! Super-tracked quest/content/map-pin state is not modeled yet. Keep the
//! no-op and empty-target compatibility shape explicit here until quest
//! navigation owns the backing state.

const SUPER_TRACK_DEFAULTS_LUA: &str = r#"
C_SuperTrack = C_SuperTrack or __wow_namespace()

local function installSuperTrackDefault(name, fn)
    if rawget(C_SuperTrack, name) == nil then
        C_SuperTrack[name] = fn
    end
end

installSuperTrackDefault("GetSuperTrackedQuestID", function()
    return 0
end)

installSuperTrackDefault("GetHighestPrioritySuperTrackingType", function()
    return nil
end)

installSuperTrackDefault("GetSuperTrackedMapPin", function()
    return nil
end)

installSuperTrackDefault("SetSuperTrackedQuestID", function(_questID)
end)

installSuperTrackDefault("ClearAllSuperTracked", function()
end)

installSuperTrackDefault("ClearSuperTrackedContent", function()
end)

installSuperTrackDefault("ClearSuperTrackedMapPin", function()
end)
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(SUPER_TRACK_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_super_track_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if C_SuperTrack.GetSuperTrackedQuestID() ~= 0 then return "quest" end
                if C_SuperTrack.GetHighestPrioritySuperTrackingType() ~= nil then return "type" end
                if C_SuperTrack.GetSuperTrackedMapPin() ~= nil then return "map-pin" end
                if C_SuperTrack.SetSuperTrackedQuestID(42) ~= nil then return "set" end
                if C_SuperTrack.ClearAllSuperTracked() ~= nil then return "clear-all" end
                if C_SuperTrack.ClearSuperTrackedContent() ~= nil then return "clear-content" end
                if C_SuperTrack.ClearSuperTrackedMapPin() ~= nil then return "clear-pin" end
                return "ok"
                "#,
            )
            .expect("super-track defaults should be callable");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_super_track_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_SuperTrack = C_SuperTrack or __wow_namespace()

            function C_SuperTrack.GetSuperTrackedQuestID()
                return 42
            end
            function C_SuperTrack.GetHighestPrioritySuperTrackingType()
                return "quest"
            end
            "#,
        )
        .expect("fixture should install existing C_SuperTrack providers");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: (i32, String) = env
            .eval(
                r#"
                return C_SuperTrack.GetSuperTrackedQuestID(),
                    C_SuperTrack.GetHighestPrioritySuperTrackingType()
                "#,
            )
            .expect("existing C_SuperTrack providers should remain callable");

        assert_eq!(result, (42, "quest".to_string()));
    }
}
