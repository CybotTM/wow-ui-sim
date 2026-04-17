//! Inventory / interaction probe globals.
//!
//! Migrates 6 entries off `GLOBAL_FALSE_STUBS` onto real Rust impls:
//!
//! - `IsInventoryItemLocked(slot)` — cursor is carrying that equipped slot.
//! - `IsEquippableItem(itemId)`    — `equippable_items.contains(id)`.
//! - `IsConsumableItem(itemId)`    — `consumable_items.contains(id)`.
//! - `CanLootUnit(unit)`           — `unit is dead AND is_enemy`.
//! - `CanMerchant()`               — `SimState.merchant_frame_open`.
//! - `CanInspect(unit, showError?)` — unit resolves to a player-like entity.

use crate::lua_api::methods::borrow_state;
use crate::lua_api::state_types::CursorInfo;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &mut LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn stack_u32(state: &mut LuaState, index: i32) -> Option<u32> {
    match stack_val(state, index) {
        Val::Num(n) if n >= 0.0 => Some(n as u32),
        _ => None,
    }
}

/// `IsInventoryItemLocked(slot)` — true when the cursor is carrying the
/// item from that equipment slot (i.e. the player is dragging it).
fn is_inventory_item_locked(state: &mut LuaState) -> LuaResult<u32> {
    let Some(slot) = stack_i32(state, 1) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let locked = {
        let st = borrow_state(state)?;
        matches!(
            &st.cursor_item,
            Some(CursorInfo::Item {
                origin: crate::lua_api::state_types::CursorItemOrigin::Equipped { slot: s },
                ..
            }) if *s == slot,
        )
    };
    state.push(Val::Bool(locked));
    Ok(1)
}

fn is_equippable_item(state: &mut LuaState) -> LuaResult<u32> {
    let Some(id) = stack_u32(state, 1) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let b = borrow_state(state)?.equippable_items.contains(&id);
    state.push(Val::Bool(b));
    Ok(1)
}

fn is_consumable_item(state: &mut LuaState) -> LuaResult<u32> {
    let Some(id) = stack_u32(state, 1) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let b = borrow_state(state)?.consumable_items.contains(&id);
    state.push(Val::Bool(b));
    Ok(1)
}

/// `CanLootUnit(unit)` — true when the unit is dead and is an enemy.
/// Sim only models combat-relevant loot windows on current_target.
fn can_loot_unit(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let can = {
        let st = borrow_state(state)?;
        match unit.as_str() {
            "target" => st
                .current_target
                .as_ref()
                .is_some_and(|t| t.health <= 0 && t.is_enemy),
            "focus" => st
                .current_focus
                .as_ref()
                .is_some_and(|t| t.health <= 0 && t.is_enemy),
            _ => false,
        }
    };
    state.push(Val::Bool(can));
    Ok(1)
}

/// `CanMerchant()` — true when a merchant frame is open.
fn can_merchant(state: &mut LuaState) -> LuaResult<u32> {
    let b = borrow_state(state)?.merchant_frame_open;
    state.push(Val::Bool(b));
    Ok(1)
}

/// `CanInspect(unit, showError?)` — true for player-like units (player,
/// party members, friendly targets that are players).
fn can_inspect(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let can = {
        let st = borrow_state(state)?;
        match unit.as_str() {
            "player" | "pet" | "vehicle" => true,
            "target" => st.current_target.as_ref().is_some_and(|t| t.is_player),
            "focus" => st.current_focus.as_ref().is_some_and(|t| t.is_player),
            other => crate::lua_api::globals::unit_api::parse_party_index(other)
                .is_some_and(|idx| st.party_group_active && idx < st.party_members.len()),
        }
    };
    state.push(Val::Bool(can));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsInventoryItemLocked", is_inventory_item_locked)?;
    LuaApiMut::register_function(lua, "IsEquippableItem", is_equippable_item)?;
    LuaApiMut::register_function(lua, "IsConsumableItem", is_consumable_item)?;
    LuaApiMut::register_function(lua, "CanLootUnit", can_loot_unit)?;
    LuaApiMut::register_function(lua, "CanMerchant", can_merchant)?;
    LuaApiMut::register_function(lua, "CanInspect", can_inspect)?;
    Ok(())
}
