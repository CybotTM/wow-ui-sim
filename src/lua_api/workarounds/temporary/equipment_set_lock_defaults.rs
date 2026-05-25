//! Temporary equipment-set lock defaults.
//!
//! Equipment set creation and contents are state-backed. Transient item lock
//! state from inventory/server operations is not modeled yet, so expose the
//! inert unlocked answer as an explicit temporary default.

const EQUIPMENT_SET_LOCK_DEFAULTS_LUA: &str = r#"
C_EquipmentSet = C_EquipmentSet or __wow_namespace()
if rawget(C_EquipmentSet, "EquipmentSetContainsLockedItems") == nil then
    function C_EquipmentSet.EquipmentSetContainsLockedItems(_setID)
        return false
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(EQUIPMENT_SET_LOCK_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_unlocked_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let locked: bool = env
            .eval("return C_EquipmentSet.EquipmentSetContainsLockedItems(1)")
            .expect("equipment set lock default should be callable");

        assert!(!locked);
    }

    #[test]
    fn preserves_existing_lock_state_function() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_EquipmentSet.EquipmentSetContainsLockedItems()
                return true
            end
            "#,
        )
        .expect("fixture should install existing function");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let locked: bool = env
            .eval("return C_EquipmentSet.EquipmentSetContainsLockedItems(1)")
            .expect("existing equipment set lock function should remain callable");

        assert!(locked);
    }
}
