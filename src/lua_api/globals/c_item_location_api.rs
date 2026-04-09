//! C_Item methods that query item existence and properties via ItemLocation.
//!
//! These need SimState access to check bag_items. Registered on the C_Item table
//! after it's created in c_item_api.rs, overriding the generated stubs.

use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Extract (bagID, slotIndex) from an ItemLocation table.
fn extract_bag_slot(loc: &Value) -> Option<(i32, i32)> {
    if let Value::Table(t) = loc {
        let bag: Option<i32> = t.get("bagID").ok();
        let slot: Option<i32> = t.get("slotIndex").ok();
        if let (Some(b), Some(s)) = (bag, slot) {
            return Some((b, s));
        }
    }
    None
}

pub(crate) fn item_guid_for_bag_slot(bag: i32, slot: i32, item_id: u32) -> String {
    format!("item-{bag}-{slot}-{item_id}")
}

pub(crate) fn parse_item_guid(guid: &str) -> Option<(i32, i32, u32)> {
    let mut parts = guid.split('-');
    let prefix = parts.next()?;
    let bag = parts.next()?.parse().ok()?;
    let slot = parts.next()?.parse().ok()?;
    let item_id = parts.next()?.parse().ok()?;
    if prefix != "item" || parts.next().is_some() {
        return None;
    }
    Some((bag, slot, item_id))
}

/// Check if an item exists at the given location in state.
fn item_exists_at(state: &SimState, loc: &Value) -> bool {
    extract_bag_slot(loc).is_some_and(|(b, s)| state.get_bag_item(b, s).is_some())
}

/// Register state-aware C_Item methods on an existing C_Item table.
pub fn register(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let c_item: mlua::Table = lua.globals().get("C_Item")?;
    register_does_item_exist(lua, &c_item, state.clone())?;
    register_does_item_exist_by_id(lua, &c_item)?;
    register_is_item_data_cached(lua, &c_item, state.clone())?;
    register_is_item_data_cached_by_id(lua, &c_item)?;
    register_get_item_guid(lua, &c_item, state.clone())?;
    register_is_bound(lua, &c_item, state.clone())?;
    register_get_stack_count(lua, &c_item, state.clone())?;
    register_get_item_quality(lua, &c_item, state.clone())?;
    register_get_current_item_level(lua, &c_item, state)?;
    Ok(())
}

fn register_does_item_exist(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "DoesItemExist",
        lua.create_function(move |_, loc: Value| Ok(item_exists_at(&state.borrow(), &loc)))?,
    )
}

fn register_does_item_exist_by_id(lua: &Lua, t: &mlua::Table) -> Result<()> {
    register_item_id_existence_query(lua, t, "DoesItemExistByID")
}

fn register_is_item_data_cached(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "IsItemDataCached",
        lua.create_function(move |_, loc: Value| Ok(item_exists_at(&state.borrow(), &loc)))?,
    )
}

fn register_is_item_data_cached_by_id(lua: &Lua, t: &mlua::Table) -> Result<()> {
    register_item_id_existence_query(lua, t, "IsItemDataCachedByID")
}

fn register_item_id_existence_query(lua: &Lua, t: &mlua::Table, name: &str) -> Result<()> {
    t.set(
        name,
        lua.create_function(|_, item_id: Value| {
            let id = super::c_item_api::parse_item_id(&item_id);
            Ok(id > 0 && crate::items::get_item(id as u32).is_some())
        })?,
    )
}

fn register_get_item_guid(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set(
        "GetItemGUID",
        lua.create_function(move |lua, loc: Value| {
            let guid = extract_bag_slot(&loc)
                .and_then(|(b, s)| {
                    state
                        .borrow()
                        .get_bag_item(b, s)
                        .map(|(id, _)| item_guid_for_bag_slot(b, s, id))
                })
                .unwrap_or_default();
            Ok(Value::String(lua.create_string(&guid)?))
        })?,
    )
}

fn register_is_bound(lua: &Lua, t: &mlua::Table, state: Rc<RefCell<SimState>>) -> Result<()> {
    t.set(
        "IsBound",
        lua.create_function(move |_, loc: Value| Ok(item_exists_at(&state.borrow(), &loc)))?,
    )
}

fn register_get_stack_count(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetStackCount",
        lua.create_function(move |_, loc: Value| {
            let count = extract_bag_slot(&loc)
                .and_then(|(b, s)| state.borrow().get_bag_item(b, s).map(|(_, c)| c))
                .unwrap_or(0);
            Ok(count)
        })?,
    )
}

fn register_get_item_quality(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetItemQuality",
        lua.create_function(move |_, loc: Value| {
            let quality = extract_bag_slot(&loc)
                .and_then(|(b, s)| {
                    state
                        .borrow()
                        .get_bag_item(b, s)
                        .and_then(|(id, _)| crate::items::get_item(id).map(|i| i.quality as i32))
                })
                .unwrap_or(1);
            Ok(quality)
        })?,
    )
}

fn register_get_current_item_level(
    lua: &Lua,
    t: &mlua::Table,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    t.set(
        "GetCurrentItemLevel",
        lua.create_function(move |_, loc: Value| {
            let level = extract_bag_slot(&loc)
                .and_then(|(b, s)| {
                    state
                        .borrow()
                        .get_bag_item(b, s)
                        .and_then(|(id, _)| crate::items::get_item(id).map(|i| i.item_level as i32))
                })
                .unwrap_or(0);
            Ok(level)
        })?,
    )
}
