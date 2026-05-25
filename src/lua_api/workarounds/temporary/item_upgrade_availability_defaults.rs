//! Temporary item-upgrade availability defaults.
//!
//! `C_ItemUpgrade.SetItemUpgradeFromLocation` / `ClearItemUpgrade` keep real
//! simulator state for tooltip probes. Upgrade-cost and eligibility data is not
//! modeled yet, so `CanUpgradeItem` remains a false compatibility default here.

const ITEM_UPGRADE_AVAILABILITY_DEFAULTS_LUA: &str = r#"
C_ItemUpgrade = C_ItemUpgrade or __wow_namespace()
if rawget(C_ItemUpgrade, "CanUpgradeItem") == nil then
    function C_ItemUpgrade.CanUpgradeItem(_location)
        return false
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(ITEM_UPGRADE_AVAILABILITY_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_false_availability_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let can_upgrade: bool = env
            .eval("return C_ItemUpgrade.CanUpgradeItem({ bagID = 0, slotIndex = 1 })")
            .expect("item upgrade availability default should be callable");

        assert!(!can_upgrade);
    }

    #[test]
    fn preserves_existing_availability_function() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_ItemUpgrade.CanUpgradeItem()
                return true
            end
            "#,
        )
        .expect("fixture should install existing function");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let can_upgrade: bool = env
            .eval("return C_ItemUpgrade.CanUpgradeItem({ bagID = 0, slotIndex = 1 })")
            .expect("existing item upgrade availability should remain callable");

        assert!(can_upgrade);
    }
}
