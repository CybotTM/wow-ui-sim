//! Explicit Mists transmog availability; no native default is inferred.
//! Declaration: Mists TransmogOutfitInfoDocumentation.lua:586–593.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

/// Host-supplied availability for one environment.
#[derive(Debug, Default)]
pub struct MistsTransmogAvailability {
    pub enabled: Option<bool>,
}

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    if ACTIVE != ClientProfile::Mists {
        return Ok(());
    }
    let namespace = super::ensure_namespace(state, "C_TransmogOutfitInfo")?;
    table_set_rust_fn_static(state, namespace, "IsTransmogEnabled", is_transmog_enabled)
}

fn is_transmog_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let input = borrow_state(state)?.mists_transmog.enabled;
    let enabled = input.ok_or_else(|| {
        runtime_error(
            "C_TransmogOutfitInfo.IsTransmogEnabled: transmog availability has not been supplied",
        )
    })?;
    state.push(Val::Bool(enabled));
    Ok(1)
}
