//! Explicit per-item-ID lock observations for the saved equipment-set model.
//! This model has no item-instance identity or inferred native lock policy.

use crate::c_api::helpers::ensure_namespace;
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_api::state_types::EquipmentManagerState;
use crate::lua_bridge::{FromStack, TableBuilder, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn supports_item_lock_observations() -> bool {
    matches!(
        ACTIVE,
        ClientProfile::Retail
            | ClientProfile::Ptr
            | ClientProfile::WowForever
            | ClientProfile::Mists
    )
}

// Pinned EquipmentManagerDocumentation.lua:56-69 on Retail/PTR/Forever,
// :53-65 on Mists: required numeric saved-set ID -> non-nil bool.
pub(crate) fn register_query(state: &mut LuaState) -> LuaResult<()> {
    if !supports_item_lock_observations() {
        return Ok(());
    }
    let namespace = ensure_namespace(state, "C_EquipmentSet")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "EquipmentSetContainsLockedItems",
        contains_locked_items,
    )
}

fn contains_locked_items(state: &mut LuaState) -> LuaResult<u32> {
    let set_id = i32::from_stack(state, 1)?;
    let locked =
        observed_set_contains_locked_items(&borrow_state(state)?.equipment_manager, set_id)?;
    state.push(Val::Bool(locked));
    Ok(1)
}

fn observed_set_contains_locked_items(
    manager: &EquipmentManagerState,
    set_id: i32,
) -> LuaResult<bool> {
    let set = manager.sets.iter().find(|set| set.id == set_id).ok_or_else(|| {
        rilua::runtime_error(format!(
            "native-unknown: C_EquipmentSet.EquipmentSetContainsLockedItems has no saved set for ID {set_id}; native unknown-set policy is not established"
        ))
    })?;
    set.item_ids.values().try_fold(false, |has_locked, item_id| {
        let locked = manager.item_lock_observations.get(item_id).ok_or_else(|| {
            rilua::runtime_error(format!(
                "native-unknown: C_EquipmentSet.EquipmentSetContainsLockedItems has no supplied lock observation for saved item ID {item_id} in set {set_id}"
            ))
        })?;
        Ok(has_locked || *locked)
    })
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
    let Val::Bool(locked) = stack_val(state, 2) else {
        return Err(rilua::runtime_error(
            "A_Admin.SetEquipmentItemLock requires an explicit boolean lock observation",
        ));
    };
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
