use wow_ui_sim::lua_api::WowLuaEnv;

pub fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}
