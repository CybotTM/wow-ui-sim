//! C_Map temporary map-group shims: sibling/floor map groups are not modeled.
//!
//! The world map probes these optional APIs to decide whether to show the
//! floor dropdown. Until the simulator has seeded map-group state, keep the
//! live startup-compatible "no group" shape isolated here.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_map_group_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Map")?;
    table_set_rust_fn_static(state, ns, "GetMapGroupID", get_map_group_id)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetMapGroupMembersInfo",
        get_map_group_members_info,
    )?;
    Ok(())
}

fn get_map_group_id(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn get_map_group_members_info(state: &mut LuaState) -> LuaResult<u32> {
    let members = create_table(state);
    state.push(members);
    Ok(1)
}
