//! Minimal error-collection helpers kept for tests during the rilua migration.

use crate::lua_api::env::WowLuaAppData;
use rilua::LuaApi;
use std::ops::Deref;

pub fn call_error_handler<L>(lua: L, error_msg: &str)
where
    L: Deref<Target = rilua::Lua>,
{
    if collect_lua_error(lua, error_msg) {
        eprintln!("Lua error: {error_msg}");
    }
}

pub fn collect_lua_error<L>(lua: L, msg: &str) -> bool
where
    L: Deref<Target = rilua::Lua>,
{
    let Some(app_data) = lua.state().app_data::<WowLuaAppData>() else {
        return false;
    };
    let mut state = app_data.sim_state.borrow_mut();
    state.lua_errors.push(msg.to_string());
    let normalized = crate::lua_errors::extract_error_message(msg);
    let count = state.lua_error_counts.entry(normalized).or_insert(0);
    let first = *count == 0;
    *count += 1;
    first
}
