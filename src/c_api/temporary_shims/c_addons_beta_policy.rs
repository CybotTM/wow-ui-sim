//! C_AddOns temporary beta-policy shim.
//!
//! The simulator has no beta-realm script policy state. Retail callers only
//! need a boolean probe here, so keep the inert non-beta answer isolated as a
//! temporary compatibility shim.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_addons_beta_policy(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_AddOns")?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetScriptsDisallowedForBeta",
        get_scripts_disallowed_for_beta,
    )?;
    Ok(())
}

fn get_scripts_disallowed_for_beta(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}
