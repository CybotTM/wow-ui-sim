//! Temporary Collections filter defaults.
//!
//! Toybox, heirloom, and transmog filter state is not modeled yet, so these
//! helpers report the default-filter state expected by Blizzard Collections UI
//! startup.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_collection_filter_default_shims(state: &mut LuaState) -> LuaResult<()> {
    register_default_filter_namespace(state, "C_ToyBoxInfo")?;
    register_default_filter_namespace(state, "C_HeirloomInfo")?;
    register_default_filter_namespace(state, "C_TransmogCollection")
}

fn register_default_filter_namespace(state: &mut LuaState, name: &'static str) -> LuaResult<()> {
    let namespace = ensure_namespace(state, name)?;
    table_set_rust_fn_static(state, namespace, "IsUsingDefaultFilters", return_true)
}

fn return_true(state: &mut LuaState) -> LuaResult<u32> {
    state.push(rilua::Val::Bool(true));
    Ok(1)
}
