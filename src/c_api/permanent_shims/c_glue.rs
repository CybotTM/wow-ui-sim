//! Permanent `C_Glue` session defaults.
//!
//! The simulator creates a fresh Lua/runtime environment per run, so it has no
//! cross-session glue first-load tracker to expose here.

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_glue_defaults(state: &mut LuaState, ns: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        ns,
        "IsFirstLoadThisSession",
        c_glue_is_first_load_this_session,
    )
}

fn c_glue_is_first_load_this_session(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}
