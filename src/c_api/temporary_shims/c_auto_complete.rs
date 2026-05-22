//! C_AutoComplete temporary shim — realm/name completion data is not modeled.
//!
//! The deprecated global `GetAutoCompleteRealms` forwards to this namespace in
//! Blizzard Lua, so both surfaces return the same empty list until completion
//! data has backing state.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_auto_complete_shims(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_AutoComplete")?;
    table_set_rust_fn_static(state, namespace, "GetAutoCompleteRealms", get_realms)?;
    table_set_rust_fn_static(state, state.global, "GetAutoCompleteRealms", get_realms)
}

fn get_realms(state: &mut LuaState) -> LuaResult<u32> {
    let realms = create_table(state);
    state.push(realms);
    Ok(1)
}
