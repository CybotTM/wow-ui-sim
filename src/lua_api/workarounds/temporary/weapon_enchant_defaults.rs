//! Temporary weapon enchant state defaults.
//!
//! The simulator does not model temporary weapon enchants yet. Keep the empty
//! enchant tuple explicit here until weapon enchant state is represented.

const WEAPON_ENCHANT_DEFAULTS_LUA: &str = r#"
if GetWeaponEnchantInfo == nil then
  function GetWeaponEnchantInfo()
    return false, 0, 0, 0, false, 0, 0, 0
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(WEAPON_ENCHANT_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_weapon_enchant_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local hasMainHandEnchant, mainHandExpiration, mainHandCharges, mainHandEnchantID,
                    hasOffHandEnchant, offHandExpiration, offHandCharges, offHandEnchantID =
                    GetWeaponEnchantInfo()
                if hasMainHandEnchant ~= false then return "mainhand_has" end
                if mainHandExpiration ~= 0 or mainHandCharges ~= 0 or mainHandEnchantID ~= 0 then return "mainhand_values" end
                if hasOffHandEnchant ~= false then return "offhand_has" end
                if offHandExpiration ~= 0 or offHandCharges ~= 0 or offHandEnchantID ~= 0 then return "offhand_values" end
                return "ok"
                "#,
            )
            .expect("weapon enchant defaults probe should run");

        assert_eq!(result, "ok");
    }
}
