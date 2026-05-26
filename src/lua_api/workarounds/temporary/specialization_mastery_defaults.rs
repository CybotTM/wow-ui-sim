//! Temporary `C_SpecializationInfo` mastery-spell defaults.
//!
//! Core specialization identity and spell displays are modeled, but mastery
//! spell rows are not seeded yet. Deprecated specialization wrappers expect an
//! iterable table, so keep the empty-table shape explicit here.

const SPECIALIZATION_MASTERY_DEFAULTS_LUA: &str = r#"
C_SpecializationInfo = C_SpecializationInfo or __wow_namespace()

if rawget(C_SpecializationInfo, "GetSpecializationMasterySpells") == nil then
    function C_SpecializationInfo.GetSpecializationMasterySpells(_specIndex)
        return {}
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(SPECIALIZATION_MASTERY_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_mastery_spells_empty_table_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let count: i32 = env
            .eval(
                r#"
                local spells = C_SpecializationInfo.GetSpecializationMasterySpells(2)
                return #spells
                "#,
            )
            .expect("mastery spells should be queryable");

        assert_eq!(count, 0);
    }

    #[test]
    fn preserves_existing_mastery_spells_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_SpecializationInfo = C_SpecializationInfo or __wow_namespace()

            function C_SpecializationInfo.GetSpecializationMasterySpells(_specIndex)
                return { { spellID = 12345 } }
            end
            "#,
        )
        .expect("fixture should install existing C_SpecializationInfo provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let spell_id: i32 = env
            .eval(
                r#"
                local spells = C_SpecializationInfo.GetSpecializationMasterySpells(2)
                return spells[1].spellID
                "#,
            )
            .expect("existing mastery spells provider should remain callable");

        assert_eq!(spell_id, 12345);
    }
}
