//! Temporary `AssistedCombatManager` table defaults.
//!
//! Assisted combat rotation state is not modeled yet. These defaults keep
//! Blizzard action-bar and spellbook code loading while preserving the
//! table-shaped manager surface behind an explicit workaround boundary.

const ASSISTED_COMBAT_MANAGER_DEFAULTS_LUA: &str = r#"
AssistedCombatManager = type(AssistedCombatManager) == "table" and AssistedCombatManager or {}

if rawget(AssistedCombatManager, "HasActionSpell") == nil then
  function AssistedCombatManager:HasActionSpell()
    return false
  end
end

if rawget(AssistedCombatManager, "GetActionSpellID") == nil then
  function AssistedCombatManager:GetActionSpellID()
    return 0
  end
end

if rawget(AssistedCombatManager, "GetActionSpellDescription") == nil then
  function AssistedCombatManager:GetActionSpellDescription()
    return ""
  end
end

if rawget(AssistedCombatManager, "SetCanHighlightSpellbookSpells") == nil then
  function AssistedCombatManager:SetCanHighlightSpellbookSpells(_enabled)
  end
end

if rawget(AssistedCombatManager, "ShouldHighlightSpellbookSpell") == nil then
  function AssistedCombatManager:ShouldHighlightSpellbookSpell(_spellID)
    return false
  end
end

if rawget(AssistedCombatManager, "AddSpellTooltipLine") == nil then
  function AssistedCombatManager:AddSpellTooltipLine(_tooltip, _spellID, _overriddenSpellID)
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(ASSISTED_COMBAT_MANAGER_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_assisted_combat_manager_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("assisted combat defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                if type(AssistedCombatManager) ~= "table" then return "missing_manager" end
                if AssistedCombatManager:HasActionSpell() ~= false then return "has_action_spell" end
                if AssistedCombatManager:GetActionSpellID() ~= 0 then return "spell_id" end
                if AssistedCombatManager:GetActionSpellDescription() ~= "" then return "description" end
                if AssistedCombatManager:ShouldHighlightSpellbookSpell(116) ~= false then return "highlight" end
                AssistedCombatManager:SetCanHighlightSpellbookSpells(true)
                AssistedCombatManager:AddSpellTooltipLine({}, 116, nil)
                return "ok"
                "#,
            )
            .expect("assisted combat defaults should be callable");

        assert_eq!(result, "ok");
    }
}
