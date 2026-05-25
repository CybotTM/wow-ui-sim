//! Temporary `C_Spell` metadata/count defaults.
//!
//! Local spell names, icons, targeting, and cooldowns are backed elsewhere.
//! Passive/ranged/press-hold metadata, display counts, and priority-aura
//! ordering are still unmodeled, so their inert defaults stay explicit here.

const SPELL_METADATA_DEFAULTS_LUA: &str = r#"
C_Spell = C_Spell or __wow_namespace()

local function installSpellDefault(name, fn)
    if rawget(C_Spell, name) == nil then
        C_Spell[name] = fn
    end
end

local function returnFalse(_spellID)
    return false
end

installSpellDefault("IsSpellPassive", returnFalse)
installSpellDefault("IsRangedAutoAttackSpell", returnFalse)
installSpellDefault("IsPressHoldReleaseSpell", returnFalse)

installSpellDefault("GetSpellCastCount", function(_spellID)
    return 0
end)

installSpellDefault("GetSpellDisplayCount", function(_spellID, _maxDisplayCount)
    return 0
end)

installSpellDefault("IsPriorityAura", returnFalse)
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(SPELL_METADATA_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_spell_metadata_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (bool, bool, bool, i32, i32, bool) = env
            .eval(
                r#"
                return C_Spell.IsSpellPassive(116),
                    C_Spell.IsRangedAutoAttackSpell(116),
                    C_Spell.IsPressHoldReleaseSpell(116),
                    C_Spell.GetSpellCastCount(116),
                    C_Spell.GetSpellDisplayCount(116, 99),
                    C_Spell.IsPriorityAura(116)
                "#,
            )
            .expect("spell metadata defaults should be callable");

        assert_eq!(result, (false, false, false, 0, 0, false));
    }

    #[test]
    fn preserves_existing_spell_metadata_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_Spell = C_Spell or __wow_namespace()

            function C_Spell.IsSpellPassive(_spellID)
                return true
            end
            function C_Spell.GetSpellCastCount(_spellID)
                return 7
            end
            function C_Spell.IsPriorityAura(_spellID)
                return true
            end
            "#,
        )
        .expect("fixture should install existing C_Spell providers");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: (bool, i32, bool) = env
            .eval(
                r#"
                return C_Spell.IsSpellPassive(116),
                    C_Spell.GetSpellCastCount(116),
                    C_Spell.IsPriorityAura(116)
                "#,
            )
            .expect("existing spell metadata providers should remain callable");

        assert_eq!(result, (true, 7, true));
    }
}
