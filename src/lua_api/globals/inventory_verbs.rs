//! Inventory pickup verbs that move items through `SimState.cursor_item`
//! and the existing `bag_items` / `equipped_items` maps.
//!
//! Migrates 7 entries off `GLOBAL_NIL_STUBS`:
//!
//! - `PickupContainerItem(bag, slot)` — bag → cursor
//! - `PickupInventoryItem(slot)`      — equip slot → cursor
//! - `PickupBagFromSlot(slot)`        — bag container slot → cursor
//!   (bag container slots are the same equipment slot range covered by
//!    `PickupInventoryItem`; this is a named alias for the bag slots 20-23)
//! - `PickupMerchantItem(index)`      — merchant row → cursor (synthesized)
//! - `EquipCursorItem(slot)`          — cursor → equip slot
//! - `DeleteCursorItem()`             — clears cursor
//! - `PlaceAction(slot)`              — cursor spell/action → action bar slot
//!
//! Registered from `register_tail_globals` after `missing_surface` so the
//! Rust impls supersede any `stub_nil` entries that slipped through.

use crate::lua_api::methods::borrow_state_mut;
use crate::lua_api::state_types::{CursorInfo, CursorItemOrigin, EquippedItem};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &mut LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn stack_u32(state: &mut LuaState, index: i32) -> Option<u32> {
    stack_i32(state, index).and_then(|n| u32::try_from(n).ok())
}

/// `PickupContainerItem(bag, slot)` — take an item out of a bag slot and
/// place it on the cursor. If the slot is empty the call is a silent no-op.
fn pickup_container_item(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(bag), Some(slot)) = (stack_i32(state, 1), stack_i32(state, 2)) else {
        return Ok(0);
    };
    let Ok(mut st) = borrow_state_mut(state) else {
        return Ok(0);
    };
    let Some(item) = st.bag_items.remove(&(bag, slot)) else {
        return Ok(0);
    };
    st.cursor_item = Some(CursorInfo::Item {
        item_id: item.item_id,
        stack_count: item.stack_count,
        origin: CursorItemOrigin::Bag { bag, slot },
    });
    Ok(0)
}

/// `PickupInventoryItem(slot)` — take an equipped item onto the cursor.
/// Silent no-op when the slot is empty.
fn pickup_inventory_item(state: &mut LuaState) -> LuaResult<u32> {
    let Some(slot) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let Ok(mut st) = borrow_state_mut(state) else {
        return Ok(0);
    };
    let Some(equipped) = st.player.equipped_items.remove(&slot) else {
        return Ok(0);
    };
    st.cursor_item = Some(CursorInfo::Item {
        item_id: equipped.item_id,
        stack_count: 1,
        origin: CursorItemOrigin::Equipped { slot },
    });
    Ok(0)
}

/// `PickupBagFromSlot(slot)` — alias for `PickupInventoryItem` targeted at
/// the bag container slot range. Shares the same implementation.
fn pickup_bag_from_slot(state: &mut LuaState) -> LuaResult<u32> {
    pickup_inventory_item(state)
}

/// `PickupMerchantItem(index)` — synthesize a merchant item on the cursor
/// with `item_id = 100_000 + index`. Silent no-op without an index.
fn pickup_merchant_item(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_u32(state, 1) else {
        return Ok(0);
    };
    let Ok(mut st) = borrow_state_mut(state) else {
        return Ok(0);
    };
    st.cursor_item = Some(CursorInfo::Item {
        item_id: 100_000 + index,
        stack_count: 1,
        origin: CursorItemOrigin::Merchant { index },
    });
    Ok(0)
}

/// `EquipCursorItem(slot)` — write the cursor's item into `equipped_items[slot]`.
/// Any previously-equipped item returns to the cursor (WoW's swap behaviour).
/// Silent no-op when the cursor isn't holding an item.
fn equip_cursor_item(state: &mut LuaState) -> LuaResult<u32> {
    let Some(slot) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let Ok(mut st) = borrow_state_mut(state) else {
        return Ok(0);
    };
    let Some(cursor) = st.cursor_item.clone() else {
        return Ok(0);
    };
    let CursorInfo::Item { item_id, .. } = cursor else {
        return Ok(0);
    };
    let displaced = st.player.equipped_items.insert(
        slot,
        EquippedItem {
            item_id,
            enchant_id: 0,
            gem_ids: [0; 3],
        },
    );
    st.cursor_item = displaced.map(|old| CursorInfo::Item {
        item_id: old.item_id,
        stack_count: 1,
        origin: CursorItemOrigin::Equipped { slot },
    });
    Ok(0)
}

/// `DeleteCursorItem()` — clear the cursor. If the cursor was carrying a
/// bag item, it is NOT returned to the bag (deletion semantics).
fn delete_cursor_item(state: &mut LuaState) -> LuaResult<u32> {
    let Ok(mut st) = borrow_state_mut(state) else {
        return Ok(0);
    };
    st.cursor_item = None;
    Ok(0)
}

/// `PlaceAction(slot)` — if the cursor is carrying a spell/action, write
/// the spell id into `action_bars[slot]` and clear the cursor. Items are
/// not placeable on action bars; silent no-op in that case.
fn place_action(state: &mut LuaState) -> LuaResult<u32> {
    let Some(slot) = stack_u32(state, 1) else {
        return Ok(0);
    };
    let Ok(mut st) = borrow_state_mut(state) else {
        return Ok(0);
    };
    let Some(cursor) = st.cursor_item.clone() else {
        return Ok(0);
    };
    let spell_id = match cursor {
        CursorInfo::Action { spell_id, .. }
        | CursorInfo::Spell { spell_id }
        | CursorInfo::PetAction { spell_id, .. } => spell_id,
        CursorInfo::Talent { talent_id, .. } => talent_id,
        CursorInfo::Item { .. } | CursorInfo::Macro { .. } => return Ok(0),
    };
    st.action_bars.insert(slot, spell_id);
    st.cursor_item = None;
    Ok(0)
}

/// Install in the global table. Exposed for tests that want to bypass the
/// full `register_globals` chain.
pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "PickupContainerItem", pickup_container_item)?;
    LuaApiMut::register_function(lua, "PickupInventoryItem", pickup_inventory_item)?;
    LuaApiMut::register_function(lua, "PickupBagFromSlot", pickup_bag_from_slot)?;
    LuaApiMut::register_function(lua, "PickupMerchantItem", pickup_merchant_item)?;
    LuaApiMut::register_function(lua, "EquipCursorItem", equip_cursor_item)?;
    LuaApiMut::register_function(lua, "DeleteCursorItem", delete_cursor_item)?;
    LuaApiMut::register_function(lua, "PlaceAction", place_action)?;
    Ok(())
}
