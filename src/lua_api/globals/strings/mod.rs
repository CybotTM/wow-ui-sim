//! UI string registration for the rilua global surface.

pub mod string_data;

use crate::loader::helpers::resolve_lua_escapes;
use crate::lua_api::rilua_methods::{create_string, table_set};
use rilua::LuaApiMut;
use rilua::Val;

pub fn register_all_ui_strings(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let global = Val::Table(state.global);
    for (name, value) in crate::global_strings::GLOBAL_STRINGS.entries() {
        let resolved = resolve_lua_escapes(value);
        let lua_value = create_string(state, &resolved);
        table_set(state, global, name, lua_value);
    }
    Ok(())
}
