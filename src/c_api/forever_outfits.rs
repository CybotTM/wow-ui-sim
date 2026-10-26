//! Supplied Forever 70532 GetOutfitsInfo observation.
//! Declaration: TransmogOutfitInfoDocumentation.lua:383–392, entry:896–907.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, runtime_error};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    if ACTIVE != ClientProfile::WowForever {
        return Ok(());
    }
    let namespace = super::ensure_namespace(state, "C_TransmogOutfitInfo")?;
    table_set_rust_fn_static(state, namespace, "GetOutfitsInfo", get_outfits_info)
}

fn get_outfits_info(state: &mut LuaState) -> LuaResult<u32> {
    let observation = borrow_state(state)?
        .forever_outfits
        .clone()
        .ok_or_else(|| {
            runtime_error("C_TransmogOutfitInfo.GetOutfitsInfo: outfit list has not been supplied")
        })?;
    let Some(entries) = observation else {
        return Ok(0);
    };
    let snapshot = super::outfit_entry::build_entry_array(state, &entries);
    state.push(snapshot);
    Ok(1)
}
