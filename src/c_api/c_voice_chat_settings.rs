//! Supplied voice settings access, independent of transport and device catalogs.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    if ACTIVE == ClientProfile::Wrath {
        return Ok(());
    }
    let namespace = super::ensure_namespace(state, "C_VoiceChat")?;
    table_set_rust_fn_static(state, namespace, "CanAccessSettings", query_settings_access)
}

fn query_settings_access(state: &mut LuaState) -> LuaResult<u32> {
    let access = borrow_state(state)?.voice_chat.can_access_settings;
    let access = access.ok_or_else(|| {
        runtime_error("C_VoiceChat.CanAccessSettings: settings access has not been supplied")
    })?;
    state.push(Val::Bool(access));
    Ok(1)
}
