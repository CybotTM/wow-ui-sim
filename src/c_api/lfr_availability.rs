//! Mists LFR availability supplied by the simulator environment, not native policy.

use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::{gc::arena::GcRef, state::LuaState, table::Table};
use rilua::{LuaResult, Val};

pub(super) fn register(state: &mut LuaState, namespace: GcRef<Table>) -> LuaResult<()> {
    // Mists 5.5.4 LFGInfoDocumentation.lua:179 declares one non-nil boolean.
    table_set_rust_fn_static(state, namespace, "IsLFREnabled", is_lfr_enabled)
}

fn is_lfr_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let availability = borrow_state(state)?.lfr_availability;
    let enabled = availability.ok_or_else(|| {
        rilua::runtime_error(
            "native-unknown: C_LFGInfo.IsLFREnabled has no supplied availability or documented default",
        )
    })?;
    state.push(Val::Bool(enabled));
    Ok(1)
}
