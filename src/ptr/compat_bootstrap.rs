const PATCH_12_1_COMPAT_BOOTSTRAP_LUA: &str = include_str!("compat_bootstrap.lua");
const PATCH_12_1_STRICT_REMOVALS_LUA: &str = include_str!("strict_removals.lua");

pub fn init(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(PATCH_12_1_COMPAT_BOOTSTRAP_LUA)?;
    Ok(())
}

pub fn apply_post_load(env: &crate::lua_api::WowLuaEnv) {
    if let Err(err) = env.exec(PATCH_12_1_COMPAT_BOOTSTRAP_LUA) {
        eprintln!("patch 12.1 compat bootstrap failed after load: {err}");
    }
}

pub fn apply_strict_removals(env: &crate::lua_api::WowLuaEnv) {
    if let Err(err) = env.exec(PATCH_12_1_STRICT_REMOVALS_LUA) {
        eprintln!("patch 12.1 strict removals failed after startup events: {err}");
    }
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn patch_12_1_post_load_reapplies_epoch_enums_after_generated_docs_reset() {
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
            .expect("patch 12.1 enums");
        assert_eq!(on_update_mode, "Disabled");
        assert_eq!(discord, "number");
    }
}
