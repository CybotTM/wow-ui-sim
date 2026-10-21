//! CVar readiness at Lua publication: SimState loads its store synchronously
//! before registering the API. No deferred CVar-loading phase exists here.

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = super::ensure_namespace(state, "C_CVar")?;
    table_set_rust_fn_static(state, namespace, "AreCVarsLoaded", are_cvars_loaded)
}

fn are_cvars_loaded(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}
