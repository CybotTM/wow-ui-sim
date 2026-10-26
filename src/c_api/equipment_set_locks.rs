//! Explicit per-item-ID lock observations for the saved equipment-set model.
//! This model has no item-instance identity or inferred native lock policy.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::{FromStack, TableBuilder};
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn supports_item_lock_observations() -> bool {
    matches!(
        ACTIVE,
        ClientProfile::Retail
            | ClientProfile::Ptr
            | ClientProfile::WowForever
            | ClientProfile::Mists
    )
}

pub(crate) fn register_admin(builder: TableBuilder) -> LuaResult<TableBuilder> {
    if !supports_item_lock_observations() {
        return Ok(builder);
    }
    builder
        .set_function("SetEquipmentItemLock", set_item_lock)?
        .set_function("ClearEquipmentItemLock", clear_item_lock)
}

fn set_item_lock(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    let locked = bool::from_stack(state, 2)?;
    borrow_state_mut(state)?
        .equipment_manager
        .item_lock_observations
        .insert(item_id, locked);
    Ok(0)
}

fn clear_item_lock(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    borrow_state_mut(state)?
        .equipment_manager
        .item_lock_observations
        .remove(&item_id);
    Ok(0)
}
