//! Temporary `C_TaxiMap` fallback surface.
//!
//! Taxi-node data is not modeled yet. These methods keep map UI callers on
//! empty lists while preserving the default "taxi nodes can be shown" gate.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_taxi_map_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_TaxiMap")?;
    table_set_rust_fn_static(state, ns, "GetAllTaxiNodes", empty_table)?;
    table_set_rust_fn_static(state, ns, "GetTaxiNodesForMap", empty_table)?;
    table_set_rust_fn_static(state, ns, "ShouldMapShowTaxiNodes", return_true)?;
    Ok(())
}

fn empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn return_true(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}
