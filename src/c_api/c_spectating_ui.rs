//! Scenario-backed spectating query; the native contract is undocumented.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    if !matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::WowForever
    ) {
        return Ok(());
    }
    let namespace = super::ensure_namespace(state, "C_SpectatingUI")?;
    table_set_rust_fn_static(state, namespace, "IsSpectating", is_spectating)
}

fn is_spectating(state: &mut LuaState) -> LuaResult<u32> {
    let spectating = borrow_state(state)?.spectating;
    state.push(Val::Bool(spectating));
    Ok(1)
}
