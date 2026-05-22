//! C_SuperTrack temporary shim — super-tracked quest/content/map-pin state is not modeled.
//!
//! Quest navigation and POI callsites need these methods to exist even when the
//! simulator has no active super-tracking target.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;
use rilua::{Val, vm::gc::arena::GcRef, vm::table::Table};

type LuaTableRef = GcRef<Table>;

pub(crate) fn register_c_super_track_shims(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_SuperTrack")?;
    register_super_track_query_shims(state, namespace)?;
    register_super_track_mutation_shims(state, namespace)
}

fn register_super_track_query_shims(state: &mut LuaState, namespace: LuaTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, namespace, "GetSuperTrackedQuestID", return_zero)?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetHighestPrioritySuperTrackingType",
        return_nil,
    )?;
    table_set_rust_fn_static(state, namespace, "GetSuperTrackedMapPin", return_nil)
}

fn register_super_track_mutation_shims(
    state: &mut LuaState,
    namespace: LuaTableRef,
) -> LuaResult<()> {
    table_set_rust_fn_static(state, namespace, "SetSuperTrackedQuestID", noop)?;
    table_set_rust_fn_static(state, namespace, "ClearAllSuperTracked", noop)?;
    table_set_rust_fn_static(state, namespace, "ClearSuperTrackedContent", noop)?;
    table_set_rust_fn_static(state, namespace, "ClearSuperTrackedMapPin", noop)
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn return_nil(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}
