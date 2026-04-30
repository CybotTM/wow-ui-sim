//! Wrath post-load Lua workarounds — patches that wrap functions defined
//! by FrameXML / Blizzard_* addons. Runs from the post-load workaround
//! chain in `src/lua_api/workarounds.rs::apply`, not the early bootstrap
//! (which runs before any addon code defines those functions).

const WRATH_POST_LOAD_LUA: &str = include_str!("post_load.lua");

pub fn apply(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(WRATH_POST_LOAD_LUA);
}
