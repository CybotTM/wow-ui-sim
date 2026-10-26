//! Wrath inventory candidates for the original paper-doll flyout.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::items;
use crate::lua_api::methods::{borrow_state, table_set_num};
use crate::lua_api::state::SimState;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

// Wrath FrameXML/Constants.lua defines the location flags and bag shift.
const PLAYER_LOCATION: i32 = 0x0010_0000;
const BAG_LOCATION: i32 = 0x0020_0000;
const BAG_SHIFT: u32 = 8;
const FINGER_INVENTORY_TYPE: u8 = 11;

fn is_ring_candidate(sim: &SimState, item_id: u32) -> bool {
    sim.equippable_items.contains(&item_id)
        && items::get_item(item_id).is_some_and(|item| item.inventory_type == FINGER_INVENTORY_TYPE)
}

fn ring_candidates(sim: &SimState) -> Vec<(i32, u32)> {
    let equipped = sim
        .player
        .equipped_items
        .iter()
        .filter_map(|(&slot, item)| {
            (matches!(slot, 11 | 12) && is_ring_candidate(sim, item.item_id))
                .then_some((PLAYER_LOCATION + slot, item.item_id))
        });
    let carried = sim.bag_items.iter().filter_map(|(&(bag, slot), item)| {
        ((0..=4).contains(&bag) && is_ring_candidate(sim, item.item_id)).then_some((
            PLAYER_LOCATION + BAG_LOCATION + (bag << BAG_SHIFT) + slot,
            item.item_id,
        ))
    });
    equipped.chain(carried).collect()
}

fn get_inventory_items_for_slot(state: &mut LuaState) -> LuaResult<u32> {
    let Val::Num(slot) = stack_val(state, 1) else {
        return Err(runtime_error(
            "GetInventoryItemsForSlot requires a numeric slot",
        ));
    };
    if !(1.0..=19.0).contains(&slot) {
        return Ok(0);
    }
    let Val::Table(table) = stack_val(state, 2) else {
        return Err(runtime_error(
            "GetInventoryItemsForSlot requires an item table",
        ));
    };
    let candidates = if slot == 11.0 || slot == 12.0 {
        let sim = borrow_state(state)?;
        ring_candidates(&sim)
    } else {
        Vec::new()
    };
    for (location, item_id) in candidates {
        table_set_num(
            state,
            table,
            f64::from(location),
            Val::Num(f64::from(item_id)),
        );
    }
    state.push(Val::Table(table));
    Ok(1)
}

pub(crate) fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    if ACTIVE == ClientProfile::Wrath {
        LuaApiMut::register_function(
            lua,
            "GetInventoryItemsForSlot",
            get_inventory_items_for_slot,
        )?;
    }
    Ok(())
}
