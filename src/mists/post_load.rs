//! Mists post-load Lua workarounds — patches that wrap functions defined
//! by FrameXML / Blizzard_* addons.

const MISTS_POST_LOAD_LUA: &str = include_str!("post_load.lua");

pub fn apply(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(MISTS_POST_LOAD_LUA);
}

pub fn apply_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if addon_name == "Blizzard_Collections" {
        let _ = env.exec(MISTS_POST_LOAD_LUA);
    }
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn post_load_hides_inactive_pvp_ready_dialog() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            PVPReadyDialog = CreateFrame("Frame", "PVPReadyDialog", UIParent)
            PVPReadyDialog:Show()
            "#,
        )
        .expect("dialog setup should run");

        super::apply(&env);

        let shown: bool = env
            .eval("return PVPReadyDialog:IsShown()")
            .expect("visibility probe should run");
        assert!(!shown, "inactive PVPReadyDialog should not show at startup");
    }
}
