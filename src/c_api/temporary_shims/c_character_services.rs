//! C_CharacterServices temporary shims — service entitlement and assignment
//! state are not modeled.
//!
//! Active boost/trial type probes remain SimState-backed. These helpers expose
//! the inert service/display/assignment shape until character-service
//! entitlement and distribution state exists.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_character_services_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_CharacterServices")?;
    table_set_rust_fn_static(
        state,
        ns,
        "HasRequiredServiceForCharacterUpgrade",
        has_required_service_for_character_upgrade,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetCharacterServiceDisplayInfo",
        get_character_service_display_info,
    )?;
    table_set_rust_fn_static(state, ns, "AssignUpgradeDistribution", assign_noop)?;
    table_set_rust_fn_static(state, ns, "AssignPCTDistribution", assign_noop)?;
    Ok(())
}

fn has_required_service_for_character_upgrade(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn get_character_service_display_info(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn assign_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
