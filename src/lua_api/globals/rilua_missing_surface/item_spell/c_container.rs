use super::c_item::item_link_for_id;
use super::helpers::global_table;
use crate::lua_api::globals::rilua_missing_surface::ensure_namespace;
use crate::lua_api::rilua_methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub(super) fn register_c_item_upgrade(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_ItemUpgrade")?;
    table_set_rust_fn(
        state,
        table_ref,
        "SetItemUpgradeFromLocation",
        c_item_upgrade_set_location,
    )?;
    table_set_rust_fn(state, table_ref, "ClearItemUpgrade", c_item_upgrade_clear)?;
    Ok(())
}

fn c_item_upgrade_set_location(state: &mut LuaState) -> LuaResult<u32> {
    let location = stack_val(state, 1);
    let storage = global_table(state, "__item_upgrade_state");
    table_set(state, storage, "location", location);
    Ok(0)
}

fn c_item_upgrade_clear(state: &mut LuaState) -> LuaResult<u32> {
    let storage = global_table(state, "__item_upgrade_state");
    table_set(state, storage, "location", Val::Nil);
    Ok(0)
}

pub(super) fn register_c_container(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Container")?;
    register_container_methods(
        state,
        table_ref,
        &[
            ("GetContainerNumSlots", c_container_get_num_slots),
            ("GetContainerNumFreeSlots", c_container_get_num_free_slots),
            ("GetContainerItemInfo", c_container_get_item_info),
            ("GetContainerItemID", c_container_get_item_id),
            ("GetContainerItemLink", c_container_get_item_link),
            ("ContainerIDToInventoryID", c_container_id_to_inventory_id),
            ("GetBagName", c_container_get_bag_name),
            (
                "GetContainerItemPurchaseInfo",
                c_container_get_item_purchase_info,
            ),
            ("GetContainerItemQuestInfo", c_container_get_item_quest_info),
            ("IsBattlePayItem", c_container_is_battle_pay_item),
        ],
    )?;
    register_container_methods(
        state,
        table_ref,
        &[
            ("UseContainerItem", c_container_noop),
            ("PickupContainerItem", c_container_noop),
            ("SplitContainerItem", c_container_noop),
        ],
    )?;
    Ok(())
}

type ContainerScriptFn = fn(&mut LuaState) -> LuaResult<u32>;

fn register_container_methods(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
    entries: &[(&str, ContainerScriptFn)],
) -> LuaResult<()> {
    for &(name, func) in entries {
        table_set_rust_fn(state, table_ref, name, func)?;
    }
    Ok(())
}

fn c_container_get_num_slots(state: &mut LuaState) -> LuaResult<u32> {
    let bag = i32::from_stack(state, 1)?;
    let slots = if bag == 0 { 16.0 } else { 0.0 };
    state.push(Val::Num(slots));
    Ok(1)
}

fn c_container_get_num_free_slots(state: &mut LuaState) -> LuaResult<u32> {
    let bag = i32::from_stack(state, 1)?;
    let occupied = borrow_state(state)?.bag_occupied_slots(bag) as f64;
    let free = if bag == 0 {
        (16.0 - occupied).max(0.0)
    } else {
        0.0
    };
    state.push(Val::Num(free));
    Ok(1)
}

fn c_container_get_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let bag = i32::from_stack(state, 1)?;
    let slot = i32::from_stack(state, 2)?;
    let Some((item_id, stack_count)) = borrow_state(state)?.get_bag_item(bag, slot) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    table_set(state, info, "itemID", Val::Num(item_id as f64));
    table_set(state, info, "stackCount", Val::Num(stack_count as f64));
    state.push(info);
    Ok(1)
}

fn c_container_get_item_id(state: &mut LuaState) -> LuaResult<u32> {
    let bag = i32::from_stack(state, 1)?;
    let slot = i32::from_stack(state, 2)?;
    let item_id = borrow_state(state)?
        .get_bag_item(bag, slot)
        .map(|(item_id, _)| item_id);
    match item_id {
        Some(item_id) => state.push(Val::Num(item_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_container_get_item_link(state: &mut LuaState) -> LuaResult<u32> {
    let bag = i32::from_stack(state, 1)?;
    let slot = i32::from_stack(state, 2)?;
    let link = borrow_state(state)?
        .get_bag_item(bag, slot)
        .and_then(|(item_id, _)| item_link_for_id(item_id));
    match link {
        Some(link) => {
            let link = create_string(state, &link);
            state.push(link);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_container_id_to_inventory_id(state: &mut LuaState) -> LuaResult<u32> {
    let bag = i32::from_stack(state, 1)?;
    state.push(Val::Num((20 + bag).max(0) as f64));
    Ok(1)
}

fn c_container_get_bag_name(state: &mut LuaState) -> LuaResult<u32> {
    let bag = i32::from_stack(state, 1)?;
    if bag == 0 {
        let name = create_string(state, "Backpack");
        state.push(name);
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

fn c_container_get_item_purchase_info(state: &mut LuaState) -> LuaResult<u32> {
    let _bag = i32::from_stack(state, 1)?;
    let _slot = i32::from_stack(state, 2)?;
    state.push(Val::Nil);
    Ok(1)
}

fn c_container_get_item_quest_info(state: &mut LuaState) -> LuaResult<u32> {
    let _bag = i32::from_stack(state, 1)?;
    let _slot = i32::from_stack(state, 2)?;
    state.push(Val::Nil);
    Ok(1)
}

fn c_container_is_battle_pay_item(state: &mut LuaState) -> LuaResult<u32> {
    let _bag = i32::from_stack(state, 1)?;
    let _slot = i32::from_stack(state, 2)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_container_noop(state: &mut LuaState) -> LuaResult<u32> {
    let _ = state;
    Ok(0)
}
