//! Temporary-shim namespace wrapper for lua_api globals.
//!
//! Real implementations live in `super::missing_surface`; this module exists
//! so the directory tree makes the shim split explicit.

use rilua::LuaResult;

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    super::missing_surface::register_all(lua)
}

pub fn register_quest_log_overrides(lua: &mut rilua::Lua) -> LuaResult<()> {
    super::missing_surface::register_quest_log_overrides(lua)
}
