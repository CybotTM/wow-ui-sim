//! Temporary `C_Navigation` fallback surface.
//!
//! Quest/navigation target state is not modeled yet, so these namespace methods
//! expose the safe empty defaults Blizzard navigation code expects at startup.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_navigation_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Navigation")?;
    table_set_rust_fn_static(state, ns, "WasClampedToScreen", return_false)?;
    table_set_rust_fn_static(state, ns, "GetTargetState", return_zero)?;
    table_set_rust_fn_static(state, ns, "HasValidScreenPosition", return_false)?;
    table_set_rust_fn_static(state, ns, "GetDistance", return_zero)?;
    table_set_rust_fn_static(state, ns, "GetNearestPartyMemberToken", no_results)?;
    table_set_rust_fn_static(state, ns, "GetFrame", no_results)?;
    Ok(())
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn no_results(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
