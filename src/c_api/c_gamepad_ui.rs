//! Forever Gamepad views share existing player/pet storage without aliasing.
//!
//! Player slots 4096–4179 reserve three pages of 28 bindable positions in the
//! existing action maps. This simulator allocation is not native numbering;
//! see docs/specs/forever-gamepad-player-storage.md.

use crate::lua_api::state::FIRST_PET_ACTION_SLOT;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const FIRST_PLAYER_ACTION_SLOT: u32 = 4096;

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    let table = super::helpers::ensure_namespace(state, "C_GamepadUI")?;
    table_set_rust_fn_static(
        state,
        table,
        "GetFirstGamepadActionStorageSlotIndex",
        first_player_slot,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetFirstGamepadPetActionStorageSlotIndex",
        first_pet_slot,
    )
}

fn first_player_slot(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(FIRST_PLAYER_ACTION_SLOT as f64));
    Ok(1)
}

fn first_pet_slot(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(FIRST_PET_ACTION_SLOT as f64));
    Ok(1)
}
