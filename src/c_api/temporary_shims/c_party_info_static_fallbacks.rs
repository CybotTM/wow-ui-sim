//! C_PartyInfo temporary static fallbacks — invite and Torghast state are not
//! modeled.
//!
//! These methods expose the inert baseline used by addon startup code until
//! the simulator has pending-invite and Torghast/Jailer's Tower backing state.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_party_info_static_fallbacks(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_PartyInfo")?;
    table_set_rust_fn_static(
        state,
        ns,
        "IsPartyInJailersTower",
        is_party_in_jailers_tower,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetInviteConfirmationInfo",
        get_invite_confirmation_info,
    )?;
    Ok(())
}

fn is_party_in_jailers_tower(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn get_invite_confirmation_info(state: &mut LuaState) -> LuaResult<u32> {
    let _guid = Option::<String>::from_stack(state, 1)?;
    Ok(0)
}
