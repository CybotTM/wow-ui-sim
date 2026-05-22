//! C_PetBattles temporary static fallbacks — pet journal and player-NPC
//! ownership state are not modeled.
//!
//! The main pet-battle surface is backed by `SimState.pet_battles`. These
//! helpers cover APIs that require a broader battle-pet journal or PvP/NPC
//! ownership model.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_pet_battles_static_fallbacks(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_PetBattles")?;
    table_set_rust_fn_static(state, ns, "GetAllEffectNames", no_results)?;
    table_set_rust_fn_static(state, ns, "GetPetInfoByPetID", get_pet_info_by_pet_id)?;
    table_set_rust_fn_static(state, ns, "IsTrapAvailable", is_trap_available)?;
    table_set_rust_fn_static(state, ns, "IsPlayerNPC", is_player_npc)?;
    table_set_rust_fn_static(state, ns, "ShouldShowPetSelect", return_false)?;
    Ok(())
}

fn no_results(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn get_pet_info_by_pet_id(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn is_trap_available(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    state.push(Val::Num(0.0));
    Ok(2)
}

fn is_player_npc(state: &mut LuaState) -> LuaResult<u32> {
    return_false(state)
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}
