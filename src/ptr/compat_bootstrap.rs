const PTR_COMPAT_BOOTSTRAP_LUA: &str = include_str!("compat_bootstrap.lua");
const PTR_STRICT_REMOVALS_LUA: &str = include_str!("strict_removals.lua");

pub fn init(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(PTR_COMPAT_BOOTSTRAP_LUA)?;
    Ok(())
}

pub fn apply_post_load(env: &crate::lua_api::WowLuaEnv) {
    if let Err(err) = env.exec(PTR_COMPAT_BOOTSTRAP_LUA) {
        eprintln!("PTR compat bootstrap failed after load: {err}");
    }
    if let Err(err) = env.exec(PTR_STRICT_REMOVALS_LUA) {
        eprintln!("PTR strict removals failed after load: {err}");
    }
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn patch_12_1_post_load_reapplies_ptr_enums_after_generated_docs_reset() {
        let env = WowLuaEnv::new().expect("env");
        env.exec("Enum.OnUpdateMode = nil; Enum.ClubStreamType.Discord = nil")
            .expect("reset enums");

        super::apply_post_load(&env);

        let (on_update_mode, discord): (String, String) = env
            .eval(
                r#"
                return Enum.OnUpdateMode.Disabled,
                    type(Enum.ClubStreamType.Discord)
                "#,
            )
            .expect("ptr enums");
        assert_eq!(on_update_mode, "Disabled");
        assert_eq!(discord, "number");
    }
}
