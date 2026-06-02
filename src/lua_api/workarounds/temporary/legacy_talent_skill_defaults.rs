//! Temporary legacy talent, PvP talent, arena, and skill-window global defaults.
//!
//! These globals are explicit workarounds because the simulator does not yet
//! model pre-MoP talent trees, retail PvP talent selections, arena rosters, or
//! the removed legacy skill window.

const LEGACY_TALENT_SKILL_DEFAULTS_LUA: &str = r#"
local function numberArgument(value)
    if type(value) == "number" then
        return value
    end

    return nil
end

local function isValidPvpTalentSlot(slotIndex)
    return slotIndex == 1 or slotIndex == 2 or slotIndex == 3
end

if rawget(_G, "GetNumTalentTabs") == nil then
    function GetNumTalentTabs()
        return 0
    end
end

if rawget(_G, "GetTalentInfo") == nil then
    function GetTalentInfo(_tabIndex, _talentIndex)
        return nil
    end
end

if rawget(_G, "GetTalentInfoBySpecialization") == nil then
    function GetTalentInfoBySpecialization(_specIndex, _tier, _column)
        return nil
    end
end

if rawget(_G, "GetPvpTalentSlotInfo") == nil then
    function GetPvpTalentSlotInfo(slotIndex)
        slotIndex = numberArgument(slotIndex)
        if slotIndex == nil or not isValidPvpTalentSlot(slotIndex) then
            return nil
        end

        return {
            enabled = true,
            locked = false,
            selectedTalentID = 0,
            slotIndex = slotIndex,
        }
    end
end

if rawget(_G, "GetArenaOpponentSpec") == nil then
    function GetArenaOpponentSpec(_opponentIndex)
        return 0
    end
end

if rawget(_G, "GetNumSkillLines") == nil then
    function GetNumSkillLines()
        return 0
    end
end

if rawget(_G, "GetSkillLineInfo") == nil then
    function GetSkillLineInfo(_index)
        return nil
    end
end

if rawget(_G, "GetSelectedSkill") == nil then
    function GetSelectedSkill()
        return 0
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(LEGACY_TALENT_SKILL_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_legacy_talent_skill_default_shapes() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (i32, bool, bool, i32, i32, i32, bool, i32) = env
            .eval(
                r#"
                local talent = GetTalentInfo(1, 1)
                local specTalent = GetTalentInfoBySpecialization(1, 1, 1)
                local slot = GetPvpTalentSlotInfo(2)
                local missingSlot = GetPvpTalentSlotInfo(4)
                local skill = GetSkillLineInfo(1)
                return GetNumTalentTabs(),
                       talent == nil,
                       specTalent == nil,
                       slot.slotIndex,
                       slot.selectedTalentID,
                       GetArenaOpponentSpec(1),
                       missingSlot == nil and skill == nil,
                       GetSelectedSkill()
                "#,
            )
            .expect("legacy talent/skill defaults should be callable");

        assert_eq!(result, (0, true, true, 2, 0, 0, true, 0));
    }

    #[test]
    fn preserves_existing_legacy_talent_skill_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function GetArenaOpponentSpec(_opponentIndex)
                return 70
            end
            "#,
        )
        .expect("fixture should install existing provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let spec_id: i32 = env
            .eval("return GetArenaOpponentSpec(1)")
            .expect("existing provider should remain callable");

        assert_eq!(spec_id, 70);
    }
}
