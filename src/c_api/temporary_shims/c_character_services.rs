//! C_CharacterServices temporary shims — service entitlement and assignment
//! state are not modeled.
//!
//! Active boost/trial type probes remain SimState-backed. These helpers expose
//! the inert service/display/assignment shape until character-service
//! entitlement and distribution state exists.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::{create_string, create_table, table_set};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_character_services_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_CharacterServices")?;
    table_set_rust_fn_static(
        state,
        ns,
        "HasRequiredServiceForCharacterUpgrade",
        return_false,
    )?;
    table_set_rust_fn_static(state, ns, "HasRequiredBoostForClassTrial", return_false)?;
    table_set_rust_fn_static(state, ns, "GetCharacterServiceDisplayInfo", empty_table)?;
    table_set_rust_fn_static(state, ns, "GetVASDistributions", empty_table)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetCharacterServiceDisplayData",
        get_character_service_display_data,
    )?;
    table_set_rust_fn_static(state, ns, "AssignUpgradeDistribution", assign_noop)?;
    table_set_rust_fn_static(state, ns, "AssignPCTDistribution", assign_noop)?;
    Ok(())
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn get_character_service_display_data(state: &mut LuaState) -> LuaResult<u32> {
    let info = create_table(state);
    let popup_info = create_table(state);
    let flow_title = create_string(state, "Character Upgrade");
    let texture_kit = create_string(state, "characterupdate");
    table_set(state, popup_info, "textureKit", texture_kit);
    table_set(state, info, "boostLevel", Val::Num(80.0));
    table_set(state, info, "flowTitle", flow_title);
    table_set(state, info, "popupInfo", popup_info);
    state.push(info);
    Ok(1)
}

fn assign_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
