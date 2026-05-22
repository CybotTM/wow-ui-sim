//! C_RestrictedActions temporary shim — addon restriction state is not modeled.
//!
//! The simulator does not enforce restricted-action policy yet, so default to
//! allowing protected calls and reporting no addon restriction.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_restricted_actions_shims(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_RestrictedActions")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "CheckAllowProtectedFunctions",
        allow_protected_functions,
    )?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetAddOnRestrictionState",
        get_addon_restriction_state,
    )
}

fn allow_protected_functions(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn get_addon_restriction_state(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}
