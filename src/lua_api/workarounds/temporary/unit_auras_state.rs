//! Temporary `C_UnitAuras` compatibility surface.
//!
//! Aura lookup and blocked-aura/provider-switch state are Rust-backed in
//! `globals::auras`. The private warning text anchor is still a no-op
//! compatibility surface, so keep it explicit as temporary behavior.

const UNIT_AURAS_STATE_LUA: &str = r#"
if type(C_UnitAuras) ~= "table" then
    C_UnitAuras = {}
end

if rawget(C_UnitAuras, "SetPrivateWarningTextAnchor") == nil then
    function C_UnitAuras.SetPrivateWarningTextAnchor()
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(UNIT_AURAS_STATE_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_private_warning_text_anchor_without_replacing_aura_state() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if type(C_UnitAuras.SetPrivateWarningTextAnchor) ~= "function" then
                    return "missing_warning_anchor"
                end
                if type(C_UnitAuras.AddBlockedAura) ~= "function" then
                    return "missing_blocked_aura"
                end
                if type(C_UnitAuras._blockedAuras) ~= "table" then
                    return "missing_blocked_state"
                end
                if C_UnitAuras._providerSwitched ~= false then
                    return "bad_provider_state"
                end
                C_UnitAuras.SetPrivateWarningTextAnchor("frame", "TOP", 1, 2)
                return "ok"
                "#,
            )
            .expect("unit aura compatibility probe should run");

        assert_eq!(result, "ok");
    }
}
