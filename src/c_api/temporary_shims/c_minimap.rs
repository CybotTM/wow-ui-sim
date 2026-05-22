//! Temporary `C_Minimap` tracking fallback surface.
//!
//! Minimap tracking data is not modeled yet. Widget-specific minimap methods
//! remain on the Minimap frame; this shim only owns namespace-level defaults.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::{create_table, table_set};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_minimap_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Minimap")?;
    table_set_rust_fn_static(state, ns, "GetNumTrackingTypes", return_zero)?;
    table_set_rust_fn_static(state, ns, "GetTrackingInfo", no_results)?;
    table_set_rust_fn_static(state, ns, "GetTrackingFilter", tracking_filter)?;
    table_set_rust_fn_static(state, ns, "SetTracking", no_results)?;
    table_set_rust_fn_static(state, ns, "ClearAllTracking", no_results)?;
    table_set_rust_fn_static(state, ns, "GetViewRadius", get_view_radius)?;
    Ok(())
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn no_results(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn tracking_filter(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    table_set(state, table, "filterID", Val::Num(0.0));
    table_set(state, table, "spellID", Val::Num(0.0));
    state.push(table);
    Ok(1)
}

fn get_view_radius(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(200.0));
    Ok(1)
}
