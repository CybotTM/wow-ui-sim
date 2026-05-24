//! Temporary legacy Battle.net account defaults.
//!
//! Retail Battle.net state is partially modeled through `C_BattleNet`, but the
//! old global `BNGetInfo` is still only needed as a safe startup probe. Keep its
//! inert account shape explicit here until legacy Battle.net globals are backed
//! by the social state model.

const BATTLE_NET_ACCOUNT_DEFAULTS_LUA: &str = r#"
if BNGetInfo == nil then
  function BNGetInfo()
    return nil, "", 0, "", false, false, false
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(BATTLE_NET_ACCOUNT_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_battle_net_account_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local accountName, battleTag, bnetIDAccount, displayName,
                    isRIDEnabled, isInAllowedRegion, isDND = BNGetInfo()
                if accountName ~= nil then return "account" end
                if battleTag ~= "" or bnetIDAccount ~= 0 or displayName ~= "" then return "identity" end
                if isRIDEnabled ~= false or isInAllowedRegion ~= false or isDND ~= false then return "flags" end
                return "ok"
                "#,
            )
            .expect("Battle.net account defaults probe should run");

        assert_eq!(result, "ok");
    }
}
