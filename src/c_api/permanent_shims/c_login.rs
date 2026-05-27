//! Permanent `C_Login` glue-account defaults.
//!
//! The simulator does not model Battle.net launcher handoff, reconnect state,
//! first-run account classification, or persisted login error state. Keep those
//! static glue-screen answers separate from state-backed login screen methods.

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_login_defaults(state: &mut LuaState, ns: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, ns, "IsLauncherLogin", c_login_false)?;
    table_set_rust_fn_static(state, ns, "IsReconnectLoginPossible", c_login_false)?;
    table_set_rust_fn_static(state, ns, "GetLastError", c_login_get_last_error)?;
    table_set_rust_fn_static(state, ns, "ClearLastError", c_login_noop)?;
    table_set_rust_fn_static(state, ns, "AttemptedLauncherLogin", c_login_false)?;
    table_set_rust_fn_static(state, ns, "IsNewPlayer", c_login_false)?;
    Ok(())
}

fn c_login_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_login_get_last_error(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn c_login_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
