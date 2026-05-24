//! Temporary glue character-select defaults.
//!
//! Character-select startup still expects a handful of glue globals before the
//! simulator has a full account/character-service state model. Keep those
//! defaults explicit here until that backing model owns them.

const GLUE_CHARACTER_SELECT_DEFAULTS_LUA: &str = r#"
if GetSpecializationInfoForSpecID == nil then
  function GetSpecializationInfoForSpecID(_specID)
    return nil, ""
  end
end

if GetCharacterUndeleteStatus == nil then
  function GetCharacterUndeleteStatus()
    return false, false, 0, 0
  end
end

if IsCharacterTimerunning == nil then
  function IsCharacterTimerunning(_characterIndex)
    return false
  end
end

if ShouldShowExpansionUpgradeBanner == nil then
  function ShouldShowExpansionUpgradeBanner()
    return false
  end
end

if GetCharacterListGroupsInfo == nil then
  function GetCharacterListGroupsInfo()
    return {}
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(GLUE_CHARACTER_SELECT_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_glue_character_select_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local specName, specDescription = GetSpecializationInfoForSpecID(999999)
                if specName ~= nil or specDescription ~= "" then return "spec" end

                local canUndelete, cooldownActive, cooldownRemaining, cooldownSeconds = GetCharacterUndeleteStatus()
                if canUndelete ~= false or cooldownActive ~= false then return "undelete_flags" end
                if cooldownRemaining ~= 0 or cooldownSeconds ~= 0 then return "undelete_cooldown" end

                if IsCharacterTimerunning(1) ~= false then return "timerunning" end
                if ShouldShowExpansionUpgradeBanner() ~= false then return "upgrade_banner" end

                local groups = GetCharacterListGroupsInfo()
                if type(groups) ~= "table" or next(groups) ~= nil then return "groups" end

                return "ok"
                "#,
            )
            .expect("glue character-select defaults probe should run");

        assert_eq!(result, "ok");
    }
}
