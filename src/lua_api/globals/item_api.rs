//! Item class for async item loading
//!
//! Provides Item.CreateFromItemID and Item.CreateFromItemLink for addons
//! that need to load item data asynchronously (LegionRemixHelper, etc.)

use mlua::{Lua, Result, Value};

pub fn register_item_api(lua: &Lua) -> Result<()> {
    let item_class = lua.create_table()?;

    item_class.set(
        "CreateFromItemID",
        lua.create_function(|lua, (_self, item_id): (Value, i32)| create_item_table(lua, item_id))?,
    )?;
    item_class.set(
        "CreateFromItemLink",
        lua.create_function(|lua, (_self, item_link): (Value, String)| {
            let item_id = parse_item_id_from_link(&item_link);
            create_item_table(lua, item_id)
        })?,
    )?;

    lua.globals().set("Item", item_class)?;
    Ok(())
}

/// Parse item ID from a WoW item link string.
fn parse_item_id_from_link(link: &str) -> i32 {
    link.split("item:")
        .nth(1)
        .and_then(|s| s.split(':').next())
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0)
}

/// Quality ID to color hex string.
fn quality_color(quality: u8) -> &'static str {
    match quality {
        0 => "9d9d9d",
        1 => "ffffff",
        2 => "1eff00",
        3 => "0070dd",
        4 => "a335ee",
        5 => "ff8000",
        6 => "e6cc80",
        7 => "00ccff",
        _ => "ffffff",
    }
}

/// Create an item table with standard methods for the given item ID.
fn create_item_table(lua: &Lua, item_id: i32) -> Result<mlua::Table> {
    let item = lua.create_table()?;
    item.set("itemID", item_id)?;
    register_item_methods(lua, &item)?;
    Ok(item)
}

fn register_item_methods(lua: &Lua, item: &mlua::Table) -> Result<()> {
    item.set(
        "ContinueOnItemLoad",
        lua.create_function(|_, (_this, callback): (mlua::Table, mlua::Function)| {
            callback.call::<()>(())?;
            Ok(())
        })?,
    )?;

    item.set(
        "GetItemID",
        lua.create_function(|_, this: mlua::Table| this.get::<i32>("itemID"))?,
    )?;

    item.set(
        "GetItemName",
        lua.create_function(|lua, this: mlua::Table| {
            let id: i32 = this.get("itemID")?;
            let name = item_name(id);
            Ok(Value::String(lua.create_string(name)?))
        })?,
    )?;

    item.set(
        "GetItemLink",
        lua.create_function(|lua, this: mlua::Table| {
            let id: i32 = this.get("itemID")?;
            let link = item_link(id);
            Ok(Value::String(lua.create_string(&link)?))
        })?,
    )?;

    item.set(
        "GetItemIcon",
        lua.create_function(|_, _this: mlua::Table| Ok(134400i32))?,
    )?;

    item.set(
        "GetItemQuality",
        lua.create_function(|_, this: mlua::Table| {
            let id: i32 = this.get("itemID")?;
            Ok(item_quality(id))
        })?,
    )?;

    item.set(
        "IsItemDataCached",
        lua.create_function(|_, _this: mlua::Table| Ok(true))?,
    )?;
    Ok(())
}

fn item_name(item_id: i32) -> &'static str {
    crate::items::get_item(item_id as u32)
        .map(|item| item.name)
        .unwrap_or("Unknown")
}

fn item_quality(item_id: i32) -> i32 {
    crate::items::get_item(item_id as u32)
        .map(|item| item.quality as i32)
        .unwrap_or(1)
}

fn item_link(item_id: i32) -> String {
    let color = quality_color(item_quality(item_id) as u8);
    let name = item_name(item_id);
    format!(
        "|cff{}|Hitem:{}::::::::60:::::|h[{}]|h|r",
        color, item_id, name
    )
}
