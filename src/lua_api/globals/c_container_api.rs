//! C_Container namespace and legacy container global functions.
//!
//! Provides inventory data for bag slots and the full C_Container API.
//! Bag contents are stored in `SimState::bag_items` and populated via
//! the admin API (`A_AddBagItem` / `A_RemoveBagItem`).

use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Register C_Container namespace, C_NewItems, and legacy container globals.
pub fn register_c_container_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_c_container(lua, Rc::clone(&state))?;
    register_c_new_items(lua)?;
    register_container_globals(lua, state)?;
    Ok(())
}

/// Register the C_NewItems namespace (new item indicators).
fn register_c_new_items(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsNewItem", lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?)?;
    t.set("RemoveNewItem", lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?)?;
    t.set("ClearAll", lua.create_function(|_, _: ()| Ok(()))?)?;
    lua.globals().set("C_NewItems", t)?;
    Ok(())
}

/// Slot count per bag index (0=backpack, 1–4=equipped bags, 5=reagent bag).
pub(super) fn bag_slot_count(bag: i32) -> i32 {
    match bag {
        0 => 16,     // backpack
        1..=4 => 16, // equipped bags
        _ => 0,      // reagent bag and others not equipped
    }
}

/// Build the `containerInfo` table returned by `C_Container.GetContainerItemInfo`.
fn build_container_item_info(lua: &Lua, item_id: u32, stack_count: i32) -> Result<Value> {
    let (name, quality) = if let Some(item) = crate::items::get_item(item_id) {
        (item.name, item.quality)
    } else {
        ("Unknown", 1u8)
    };
    let color = super::c_item_api::quality_color(quality);
    let link = format!(
        "|cff{}|Hitem:{}::::::::80:::::|h[{}]|h|r",
        color, item_id, name
    );
    let t = lua.create_table()?;
    t.set("itemID", item_id)?;
    t.set("iconFileID", 134400)?;
    t.set("stackCount", stack_count)?;
    t.set("quality", quality as i32)?;
    t.set("hyperlink", lua.create_string(&link)?)?;
    t.set("isLocked", false)?;
    t.set("isBound", false)?;
    t.set("isFiltered", false)?;
    t.set("isReadable", false)?;
    t.set("hasNoValue", false)?;
    t.set("hasLoot", false)?;
    Ok(Value::Table(t))
}

/// Build an item link string for a given item_id.
fn build_item_link(lua: &Lua, item_id: u32) -> Result<Value> {
    let name = crate::items::get_item(item_id)
        .map(|i| i.name)
        .unwrap_or("Unknown");
    let link = format!(
        "|cffffffff|Hitem:{}::::::::80:::::|h[{}]|h|r",
        item_id, name
    );
    Ok(Value::String(lua.create_string(&link)?))
}

/// Register C_Container item query methods.
fn register_c_container_item_methods(
    lua: &Lua, t: &mlua::Table, state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    let s = Rc::clone(state);
    t.set("GetContainerItemID", lua.create_function(move |_, (bag, slot): (i32, i32)| {
        Ok(s.borrow().get_bag_item(bag, slot).map(|(id, _)| id as i64))
    })?)?;
    let s = Rc::clone(state);
    t.set("GetContainerItemLink", lua.create_function(move |lua, (bag, slot): (i32, i32)| {
        let Some((item_id, _)) = s.borrow().get_bag_item(bag, slot) else {
            return Ok(Value::Nil);
        };
        build_item_link(lua, item_id)
    })?)?;
    register_c_container_info_methods(lua, t, state)?;
    Ok(())
}

/// Register GetContainerItemInfo, QuestInfo, and Cooldown.
fn register_c_container_info_methods(
    lua: &Lua, t: &mlua::Table, state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    let s = Rc::clone(state);
    t.set("GetContainerItemInfo", lua.create_function(move |lua, (bag, slot): (i32, i32)| {
        let Some((item_id, stack_count)) = s.borrow().get_bag_item(bag, slot) else {
            return Ok(Value::Nil);
        };
        build_container_item_info(lua, item_id, stack_count)
    })?)?;
    t.set("GetContainerItemQuestInfo", lua.create_function(|lua, (_bag, _slot): (i32, i32)| {
        let t = lua.create_table()?;
        t.set("isQuestItem", false)?;
        t.set("questID", Value::Nil)?;
        t.set("isActive", false)?;
        Ok(Value::Table(t))
    })?)?;
    t.set(
        "GetContainerItemCooldown",
        lua.create_function(|_, (_bag, _slot): (i32, i32)| Ok((0.0, 0.0, 1)))?,
    )?;
    Ok(())
}

/// Register C_Container stub methods used by ContainerFrame.lua.
fn register_c_container_stubs(
    lua: &Lua, t: &mlua::Table, state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set("IsContainerFiltered", lua.create_function(|_, _bag: i32| Ok(false))?)?;
    t.set("GetBagName", lua.create_function(|lua, bag: i32| {
        let name = if bag == 0 { "Backpack" } else { "Bag" };
        Ok(Value::String(lua.create_string(name)?))
    })?)?;
    t.set("ContainerIDToInventoryID", lua.create_function(|_, bag: i32| {
        Ok(if bag > 0 { 19 + bag } else { 0 })
    })?)?;
    let s = Rc::clone(state);
    t.set("HasContainerItem", lua.create_function(move |_, (bag, slot): (i32, i32)| {
        Ok(s.borrow().get_bag_item(bag, slot).is_some())
    })?)?;
    t.set("GetBagSlotFlag", lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?)?;
    t.set("SetBagSlotFlag", lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?)?;
    t.set("GetBackpackAutosortDisabled", lua.create_function(|_, _: ()| Ok(false))?)?;
    t.set("SetBackpackAutosortDisabled", lua.create_function(|_, _: mlua::MultiValue| Ok(()))?)?;
    t.set("GetBackpackSellJunkDisabled", lua.create_function(|_, _: ()| Ok(false))?)?;
    t.set("SetBackpackSellJunkDisabled", lua.create_function(|_, _: mlua::MultiValue| Ok(()))?)?;
    t.set("GetContainerItemPurchaseInfo", lua.create_function(|_, _: mlua::MultiValue| Ok(Value::Nil))?)?;
    t.set("UseContainerItem", lua.create_function(|_, _: mlua::MultiValue| Ok(()))?)?;
    t.set("PickupContainerItem", lua.create_function(|_, _: mlua::MultiValue| Ok(()))?)?;
    t.set("SplitContainerItem", lua.create_function(|_, _: mlua::MultiValue| Ok(()))?)?;
    t.set("IsBattlePayItem", lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?)?;
    t.set("SetBagPortraitTexture", lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?)?;
    let s = Rc::clone(state);
    t.set("GetContainerNumFreeSlots", lua.create_function(move |_, bag: i32| {
        let total = bag_slot_count(bag);
        let occupied = s.borrow().bag_occupied_slots(bag);
        Ok((total - occupied, 0i32))
    })?)?;
    Ok(())
}

/// Register the C_Container namespace.
fn register_c_container(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let c_container = lua.create_table()?;
    c_container.set(
        "GetContainerNumSlots",
        lua.create_function(|_, bag: i32| Ok(bag_slot_count(bag)))?,
    )?;
    register_c_container_item_methods(lua, &c_container, &state)?;
    register_c_container_stubs(lua, &c_container, &state)?;
    lua.globals().set("C_Container", c_container)?;
    Ok(())
}

/// Register legacy global container functions (GetContainerNumSlots, etc.).
fn register_container_globals(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetContainerNumSlots",
        lua.create_function(|_, bag: i32| Ok(bag_slot_count(bag)))?,
    )?;
    globals.set(
        "IsInventoryItemProfessionBag",
        lua.create_function(|_, (_unit, _slot): (Value, Value)| Ok(false))?,
    )?;
    let s = Rc::clone(&state);
    globals.set("GetContainerItemID", lua.create_function(move |_, (bag, slot): (i32, i32)| {
        Ok(s.borrow().get_bag_item(bag, slot).map(|(id, _)| id as i64))
    })?)?;
    let s = Rc::clone(&state);
    globals.set("GetContainerItemLink", lua.create_function(move |lua, (bag, slot): (i32, i32)| {
        let Some((item_id, _)) = s.borrow().get_bag_item(bag, slot) else {
            return Ok(Value::Nil);
        };
        build_item_link(lua, item_id)
    })?)?;
    Ok(())
}
