//! Temporary unit stagger defaults.
//!
//! The simulator does not model monk stagger damage yet. Keep the legacy
//! `UnitStagger` global and PaperDoll stagger percentage explicit here until
//! unit combat state owns stagger amounts.

const UNIT_STAGGER_DEFAULTS_LUA: &str = r#"
if UnitStagger == nil then
  function UnitStagger(_unit)
    return 0
  end
end

C_PaperDollInfo = C_PaperDollInfo or __wow_namespace()

if rawget(C_PaperDollInfo, "GetStaggerPercentage") == nil then
  function C_PaperDollInfo.GetStaggerPercentage(_unit)
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

    #[test]
    fn installs_paper_doll_stagger_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let (stagger, against_target_is_nil): (f64, bool) = env
            .eval(
                r#"
                local stagger, staggerAgainstTarget = C_PaperDollInfo.GetStaggerPercentage("player")
                return stagger, staggerAgainstTarget == nil
                "#,
            )
            .expect("PaperDoll stagger default should be callable");

        assert_eq!(stagger, 0.0);
        assert!(against_target_is_nil);
    }

    #[test]
    fn preserves_existing_paper_doll_stagger_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_PaperDollInfo = C_PaperDollInfo or __wow_namespace()

            function C_PaperDollInfo.GetStaggerPercentage(_unit)
                return 42, 17
            end
            "#,
        )
        .expect("fixture should install existing PaperDoll stagger provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: (i32, i32) = env
            .eval(r#"return C_PaperDollInfo.GetStaggerPercentage("player")"#)
            .expect("existing PaperDoll stagger provider should remain callable");

        assert_eq!(result, (42, 17));
    }
}
