//! Temporary secure-reference helper defaults.
//!
//! Real WoW stores secure references in restricted environments. The simulator
//! does not model that table yet, so keep this global compatibility helper
//! explicit until secure environments own the reference store.

const SECURE_REFERENCE_DEFAULTS_LUA: &str = r#"
if StoreSecureReference == nil then
  function StoreSecureReference(name, value)
    if type(name) == "string" then
      rawset(_G, name, value)
    end
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(SECURE_REFERENCE_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn stores_secure_reference_by_name() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local frame = CreateFrame("Frame")
                StoreSecureReference("myref", frame)
                if myref ~= frame then return "stored" end
                StoreSecureReference({}, "ignored")
                if rawget(_G, "ignored") ~= nil then return "invalid_name" end
                return "ok"
                "#,
            )
            .expect("StoreSecureReference probe should run");

        assert_eq!(result, "ok");
    }
}
