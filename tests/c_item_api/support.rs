use wow_ui_sim::lua_api::WowLuaEnv;

pub(crate) fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}
