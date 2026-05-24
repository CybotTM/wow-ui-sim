//! Temporary unit stagger defaults.
//!
//! The simulator does not model monk stagger damage yet. Keep the legacy
//! `UnitStagger` global explicit here until unit combat state owns stagger
//! amounts alongside the PaperDoll stagger percentage shim.

const UNIT_STAGGER_DEFAULTS_LUA: &str = r#"
if UnitStagger == nil then
  function UnitStagger(_unit)
    return 0
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(UNIT_STAGGER_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_unit_stagger_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let stagger: i32 = env
            .eval(r#"return UnitStagger("player")"#)
            .expect("UnitStagger probe should run");

        assert_eq!(stagger, 0);
    }
}
