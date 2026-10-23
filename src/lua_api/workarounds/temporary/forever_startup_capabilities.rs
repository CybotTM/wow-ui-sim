//! Forever-only named M2 capabilities whose backing systems are absent.
//! Retire each default with the corresponding authoritative capability model.

const STARTUP_CAPABILITIES: &str = r#"
-- Missing system: keyring UI capability policy, distinct from bag/key contents.
if rawget(C_ActionBar, "ShouldShowKeyring") == nil then
    function C_ActionBar.ShouldShowKeyring()
        return false
    end
end

-- Missing system: lifetime player talent-point award history.
-- Current unspent/spent talent state does not establish whether one was earned.
if rawget(C_SpecializationInfo, "HasPlayerEarnedATalentPoint") == nil then
    function C_SpecializationInfo.HasPlayerEarnedATalentPoint()
        return false
    end
end

-- Missing system: active targeting spell/item-condition metadata.
if rawget(C_Spell, "TargetSpellChecksItemCondition") == nil then
    function C_Spell.TargetSpellChecksItemCondition()
        return false
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(STARTUP_CAPABILITIES)?;
    Ok(())
}
