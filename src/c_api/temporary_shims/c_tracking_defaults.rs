//! Temporary tracker defaults for unmodeled UI systems.
//!
//! Content tracking and neighborhood initiatives are queried by objective
//! tracker startup code. Until their backing state exists, they expose empty
//! tracked-list shapes instead of living in the Lua bootstrap.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::{create_table, create_table_with_fields};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_tracking_default_shims(state: &mut LuaState) -> LuaResult<()> {
    register_content_tracking(state)?;
    register_neighborhood_initiative(state)
}

fn register_content_tracking(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_ContentTracking")?;
    table_set_rust_fn_static(state, namespace, "GetTrackedIDs", return_empty_table)?;
    table_set_rust_fn_static(state, namespace, "IsTracking", return_false)
}

fn register_neighborhood_initiative(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_NeighborhoodInitiative")?;
    table_set_rust_fn_static(state, namespace, "IsInitiativeEnabled", return_false)?;
    table_set_rust_fn_static(state, namespace, "GetAvailableHouseXP", return_zero)?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetTrackedInitiativeTasks",
        get_tracked_initiative_tasks,
    )?;
    table_set_rust_fn_static(state, namespace, "GetInitiativeTaskInfo", return_nil)?;
    table_set_rust_fn_static(
        state,
        namespace,
        "RemoveTrackedInitiativeTask",
        return_no_values,
    )?;
    table_set_rust_fn_static(
        state,
        namespace,
        "AddTrackedInitiativeTask",
        return_no_values,
    )
}

fn return_empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn return_nil(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn return_no_values(_: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn get_tracked_initiative_tasks(state: &mut LuaState) -> LuaResult<u32> {
    let tracked_ids = create_table(state);
    let tasks = create_table_with_fields(state, &[("trackedIDs", tracked_ids)]);
    state.push(tasks);
    Ok(1)
}
