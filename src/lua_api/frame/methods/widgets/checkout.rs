//! Checkout widget lifecycle methods.

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub(super) fn register_checkout(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, mt, "OpenCheckout", open_checkout)?;
    table_set_rust_fn_static(state, mt, "CancelOpenCheckout", no_result)?;
    table_set_rust_fn_static(state, mt, "CloseCheckout", no_result)?;
    table_set_rust_fn_static(state, mt, "OpenExternalLink", no_result)?;
    Ok(())
}

fn open_checkout(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn no_result(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
