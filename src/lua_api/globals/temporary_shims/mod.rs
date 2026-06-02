//! Temporary-shim namespace wrapper for lua_api globals.
//!
//! Broad generated defaults still live in `super::missing_surface`; narrower
//! unsupported compatibility surfaces live beside this module.

use rilua::LuaResult;

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    super::missing_surface::register_all(lua)
}

pub fn register_quest_log_overrides(lua: &mut rilua::Lua) -> LuaResult<()> {
    super::missing_surface::register_quest_log_overrides(lua)
}
