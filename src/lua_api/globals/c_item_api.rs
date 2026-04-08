//! C_Item namespace and item-related API functions.
//!
//! Contains item information, container, encoding utilities, and inventory slot functions.

use crate::lua_api::state::SimState;
use mlua::{Lua, MultiValue, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Register item-related C_* namespaces and global functions.
pub fn register_c_item_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_c_item(lua)?;
    super::c_item_location_api::register(lua, state.clone())?;
    super::c_container_api::register_c_container_api(lua, state)?;
    super::c_item_api_globals::register_item_global_apis(lua)?;
    Ok(())
}

/// Register the C_Item namespace.
fn register_c_item(lua: &Lua) -> Result<()> {
    let c_item = lua.create_table()?;
    register_c_item_info_methods(lua, &c_item)?;
    register_c_item_query_methods(lua, &c_item)?;
    register_c_item_link_methods(lua, &c_item)?;
    register_c_item_stub_methods(lua, &c_item)?;
    lua.globals().set("C_Item", c_item)?;
    Ok(())
}

/// C_Item methods: GetItemInfo, GetItemInfoInstant, GetItemIDForItemInfo.
fn register_c_item_info_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetItemInfo", make_c_item_get_item_info(lua)?)?;
    add_c_item_info_instant(lua, t)?;
    add_c_item_id_for_item_info(lua, t)?;
    Ok(())
}

fn add_c_item_info_instant(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetItemInfoInstant",
        lua.create_function(|lua, item_id: Value| {
            let id = parse_item_id_from_value(&item_id);
            if id == 0 {
                return Ok(mlua::MultiValue::new());
            }
            Ok(item_info_instant_multi_value(lua, id)?)
        })?,
    )?;
    Ok(())
}

fn add_c_item_id_for_item_info(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetItemIDForItemInfo",
        lua.create_function(|_, item_id: Value| {
            let id = parse_item_id_from_value(&item_id);
            if id == 0 {
                Ok(Value::Nil)
            } else {
                Ok(Value::Integer(id as i64))
            }
        })?,
    )?;
    Ok(())
}

/// Build the C_Item.GetItemInfo closure.
///
/// Returns 17 values matching the real WoW API signature:
///   itemName, itemLink, itemQuality, itemLevel, itemMinLevel,
///   itemType, itemSubType, itemStackCount, itemEquipLoc, itemTexture,
///   sellPrice, classID, subclassID, bindType, expacID, setID, isCraftingReagent
fn make_c_item_get_item_info(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|lua, item_id: Value| {
        let id = parse_item_id_from_value(&item_id);
        if id == 0 {
            return Ok(mlua::MultiValue::new());
        }
        let Some(item) = crate::items::get_item(id as u32) else {
            return Ok(mlua::MultiValue::new());
        };
        Ok(item_info_multi_value(lua, id, item)?)
    })
}

/// Build the 17-value MultiValue return for GetItemInfo.
fn item_info_multi_value(
    lua: &Lua,
    id: i32,
    item: &crate::items::ItemInfo,
) -> Result<mlua::MultiValue> {
    let color = quality_color(item.quality);
    let link = format!(
        "|cff{}|Hitem:{}::::::::80:::::|h[{}]|h|r",
        color, id, item.name
    );
    Ok(mlua::MultiValue::from_vec(vec![
        Value::String(lua.create_string(item.name)?), // 1  itemName
        Value::String(lua.create_string(&link)?),     // 2  itemLink
        Value::Integer(item.quality as i64),          // 3  itemQuality
        Value::Integer(item.item_level as i64),       // 4  itemLevel
        Value::Integer(item.required_level as i64),   // 5  itemMinLevel
        Value::String(lua.create_string(item_class_from_inv_type(item.inventory_type))?), // 6 itemType
        Value::String(lua.create_string(inv_type_to_subclass(item.inventory_type))?), // 7 itemSubType
        Value::Integer(item.stackable as i64), // 8  itemStackCount
        Value::String(lua.create_string(inv_type_to_equip_loc(item.inventory_type))?), // 9 itemEquipLoc
        Value::Integer(if item.icon_file_data_id != 0 {
            item.icon_file_data_id as i64
        } else {
            134400
        }), // 10 itemTexture
        Value::Integer(item.sell_price as i64), // 11 sellPrice
        Value::Integer(inv_type_to_class_id(item.inventory_type) as i64), // 12 classID
        Value::Integer(0),                      // 13 subclassID
        Value::Integer(item.bonding as i64),    // 14 bindType
        Value::Integer(item.expansion_id as i64), // 15 expacID
        Value::Integer(0),                      // 16 setID
        Value::Boolean(false),                  // 17 isCraftingReagent
    ]))
}

fn item_info_instant_multi_value(lua: &Lua, id: i32) -> Result<mlua::MultiValue> {
    let (class_name, subclass_name) = item_info_instant_names(id);
    let icon = crate::items::get_item(id as u32)
        .map(|i| i.icon_file_data_id)
        .unwrap_or(0);
    let icon = if icon != 0 { icon as i64 } else { 134400 };
    Ok(mlua::MultiValue::from_vec(vec![
        Value::Integer(id as i64),
        Value::String(lua.create_string(class_name)?),
        Value::String(lua.create_string(subclass_name)?),
        Value::String(lua.create_string("")?),
        Value::Integer(icon),
        Value::Integer(15),
        Value::Integer(0),
    ]))
}

fn item_info_instant_names(id: i32) -> (&'static str, &'static str) {
    if let Some(item) = crate::items::get_item(id as u32) {
        (
            item_class_from_inv_type(item.inventory_type),
            inv_type_to_subclass(item.inventory_type),
        )
    } else {
        ("Miscellaneous", "Junk")
    }
}

/// C_Item query methods: icon, subclass, count, class, spec, name, level.
fn register_c_item_query_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    add_item_icon_by_id(lua, t)?;
    add_item_subclass_info(lua, t)?;
    add_item_count(lua, t)?;
    add_item_class_info(lua, t)?;
    add_item_spec_info(lua, t)?;
    add_item_name_by_id(lua, t)?;
    add_detailed_item_level_info(lua, t)?;
    Ok(())
}

fn add_item_icon_by_id(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetItemIconByID",
        lua.create_function(|_, id: i32| {
            let icon = crate::items::get_item(id as u32)
                .map(|i| i.icon_file_data_id)
                .unwrap_or(0);
            Ok(if icon != 0 { icon } else { 134400u32 })
        })?,
    )?;
    Ok(())
}

fn add_item_subclass_info(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetItemSubClassInfo",
        lua.create_function(|lua, (class_id, subclass_id): (i32, i32)| {
            Ok(Value::String(lua.create_string(item_subclass_name(
                class_id,
                subclass_id,
            ))?))
        })?,
    )?;
    Ok(())
}

fn add_item_count(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetItemCount",
        lua.create_function(
            |_, (_id, _b, _c, _r): (Value, Option<bool>, Option<bool>, Option<bool>)| Ok(0),
        )?,
    )?;
    Ok(())
}

fn add_item_class_info(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetItemClassInfo",
        lua.create_function(|lua, class_id: i32| {
            Ok(Value::String(lua.create_string(item_class_name(class_id))?))
        })?,
    )?;
    Ok(())
}

fn add_item_spec_info(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetItemSpecInfo",
        lua.create_function(|lua, _id: Value| lua.create_table())?,
    )?;
    Ok(())
}

fn add_item_name_by_id(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetItemNameByID",
        lua.create_function(|lua, item_id: i32| {
            let name = item_name(item_id);
            Ok(Value::String(lua.create_string(name)?))
        })?,
    )?;
    Ok(())
}

fn add_detailed_item_level_info(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetDetailedItemLevelInfo",
        lua.create_function(|_, item_link: Value| {
            let level = item_level_for_value(&item_link);
            Ok((level, 0i32, level))
        })?,
    )?;
    Ok(())
}

fn item_name(item_id: i32) -> &'static str {
    crate::items::get_item(item_id as u32)
        .map(|i| i.name)
        .unwrap_or("Unknown")
}

fn item_level_for_value(item_link: &Value) -> i32 {
    let id = parse_item_id_from_value(item_link);
    crate::items::get_item(id as u32)
        .map(|i| i.item_level as i32)
        .unwrap_or(0)
}

/// C_Item link and quality methods.
fn register_c_item_link_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "IsItemBindToAccountUntilEquip",
        lua.create_function(|_, _v: Value| Ok(false))?,
    )?;
    t.set(
        "GetItemLink",
        lua.create_function(|lua, item_id: i32| {
            let (name, color) = if let Some(item) = crate::items::get_item(item_id as u32) {
                (item.name, quality_color(item.quality))
            } else {
                ("Unknown", "ffffff")
            };
            let link = format!(
                "|cff{}|Hitem:{}::::::::80:::::|h[{}]|h|r",
                color, item_id, name
            );
            Ok(Value::String(lua.create_string(&link)?))
        })?,
    )?;
    t.set(
        "GetItemQualityByID",
        lua.create_function(|_, item_id: i32| {
            Ok(crate::items::get_item(item_id as u32)
                .map(|i| i.quality as i32)
                .unwrap_or(1))
        })?,
    )?;
    Ok(())
}

/// C_Item stub methods (transmog, load, sockets).
/// DoesItemExist, IsBound, etc. are in c_item_location_api.rs (state-aware).
fn register_c_item_stub_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    add_nil_value_stub(lua, t, "GetItemLearnTransmogSet")?;
    t.set(
        "RequestLoadItemDataByID",
        lua.create_function(|lua, item_id: i32| {
            let success = crate::items::get_item(item_id as u32).is_some();
            fire_event(
                lua,
                "ITEM_DATA_LOAD_RESULT",
                &[Value::Integer(i64::from(item_id)), Value::Boolean(success)],
            )
        })?,
    )?;
    add_bool_value_stub(lua, t, "CanViewItemPowers")?;
    add_i32_value_stub(lua, t, "GetItemNumSockets", 0)?;
    add_i32_multivalue_stub(lua, t, "GetItemGemID", 0)?;
    add_bool_value_stub(lua, t, "IsCorruptedItem")?;
    add_bool_value_stub(lua, t, "IsCosmeticItem")?;
    add_bool_value_stub(lua, t, "IsCurioItem")?;
    add_bool_value_stub(lua, t, "IsRelicItem")?;
    add_bool_value_stub(lua, t, "IsDecorItem")?;
    add_bool_value_stub(lua, t, "IsBoundToAccountUntilEquip")?;
    Ok(())
}

fn add_nil_value_stub(lua: &Lua, t: &mlua::Table, name: &str) -> Result<()> {
    t.set(name, lua.create_function(|_, _id: i32| Ok(Value::Nil))?)?;
    Ok(())
}

fn fire_event(lua: &Lua, event_name: &str, args: &[Value]) -> Result<()> {
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    let mut call_args = vec![Value::String(lua.create_string(event_name)?)];
    call_args.extend(args.iter().cloned());
    fire.call(MultiValue::from_vec(call_args))
}

fn add_bool_value_stub(lua: &Lua, t: &mlua::Table, name: &str) -> Result<()> {
    t.set(name, lua.create_function(|_, _value: Value| Ok(false))?)?;
    Ok(())
}

fn add_i32_value_stub(lua: &Lua, t: &mlua::Table, name: &str, value: i32) -> Result<()> {
    t.set(
        name,
        lua.create_function(move |_, _value: Value| Ok(value))?,
    )?;
    Ok(())
}

fn add_i32_multivalue_stub(lua: &Lua, t: &mlua::Table, name: &str, value: i32) -> Result<()> {
    t.set(
        name,
        lua.create_function(move |_, _args: mlua::MultiValue| Ok(value))?,
    )?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse an item ID from a WoW item link string (e.g. "|Hitem:12345:...|h").
fn parse_item_id_from_link(link: &str) -> Option<i32> {
    let start = link.find("|Hitem:")? + 7;
    let rest = &link[start..];
    let end = rest.find(':')?;
    rest[..end].parse().ok()
}

/// Parse an item ID from a Lua Value (integer, number, or item link string).
/// Public accessor for sibling modules.
pub fn parse_item_id(value: &Value) -> i32 {
    parse_item_id_from_value(value)
}

fn parse_item_id_from_value(value: &Value) -> i32 {
    match value {
        Value::Integer(n) => *n as i32,
        Value::Number(n) => *n as i32,
        Value::String(s) => {
            if let Ok(s) = s.to_str() {
                parse_item_id_from_link(&s).unwrap_or(0)
            } else {
                0
            }
        }
        _ => 0,
    }
}

/// Map item class ID to name (C_Item version — always returns a value).
fn item_class_name(class_id: i32) -> &'static str {
    match class_id {
        0 => "Consumable",
        1 => "Container",
        2 => "Weapon",
        3 => "Gem",
        4 => "Armor",
        5 => "Reagent",
        6 => "Projectile",
        7 => "Tradeskill",
        8 => "Item Enhancement",
        9 => "Recipe",
        10 => "Currency (Obsolete)",
        11 => "Quiver",
        12 => "Quest",
        13 => "Key",
        14 => "Permanent (Obsolete)",
        15 => "Miscellaneous",
        16 => "Glyph",
        17 => "Battle Pets",
        18 => "WoW Token",
        _ => "Unknown",
    }
}

/// Map item class ID to name (legacy global version — includes Profession, returns empty for unknown).
pub(super) fn item_class_name_extended(class_id: i32) -> &'static str {
    match class_id {
        0 => "Consumable",
        1 => "Container",
        2 => "Weapon",
        3 => "Gem",
        4 => "Armor",
        5 => "Reagent",
        6 => "Projectile",
        7 => "Tradeskill",
        8 => "Item Enhancement",
        9 => "Recipe",
        10 => "Currency (deprecated)",
        11 => "Quiver",
        12 => "Quest",
        13 => "Key",
        14 => "Permanent (deprecated)",
        15 => "Miscellaneous",
        16 => "Glyph",
        17 => "Battle Pets",
        18 => "WoW Token",
        19 => "Profession",
        _ => "",
    }
}

/// Map item subclass to name for weapon/armor classes.
fn item_subclass_name(class_id: i32, subclass_id: i32) -> &'static str {
    match (class_id, subclass_id) {
        (2, 0) => "One-Handed Axes",
        (2, 1) => "Two-Handed Axes",
        (2, 2) => "Bows",
        (2, 3) => "Guns",
        (2, 4) => "One-Handed Maces",
        (2, 5) => "Two-Handed Maces",
        (2, 6) => "Polearms",
        (2, 7) => "One-Handed Swords",
        (2, 8) => "Two-Handed Swords",
        (2, 9) => "Warglaives",
        (2, 10) => "Staves",
        (2, 13) => "Fist Weapons",
        (2, 14) => "Miscellaneous",
        (2, 15) => "Daggers",
        (2, 16) => "Thrown",
        (2, 18) => "Crossbows",
        (2, 19) => "Wands",
        (2, 20) => "Fishing Poles",
        (4, 0) => "Miscellaneous",
        (4, 1) => "Cloth",
        (4, 2) => "Leather",
        (4, 3) => "Mail",
        (4, 4) => "Plate",
        (4, 6) => "Shield",
        _ => "Unknown",
    }
}

/// Quality ID to color hex string.
pub(super) fn quality_color(quality: u8) -> &'static str {
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

/// Map inventory type to a rough item class name.
pub(super) fn item_class_from_inv_type(inv_type: u8) -> &'static str {
    match inv_type {
        13 | 15 | 17 | 21 | 22 | 25 | 26 => "Weapon",
        1..=12 | 14 | 16 | 23 => "Armor",
        _ => "Miscellaneous",
    }
}

/// Map inventory type to Enum.ItemClass numeric ID.
/// Weapon=2, Armor=4, Miscellaneous=15.
fn inv_type_to_class_id(inv_type: u8) -> u8 {
    match inv_type {
        13 | 15 | 17 | 21 | 22 | 25 | 26 => 2,
        1..=12 | 14 | 16 | 23 => 4,
        _ => 15,
    }
}

/// Map inventory type to a rough subclass name.
fn inv_type_to_subclass(inv_type: u8) -> &'static str {
    match inv_type {
        1 => "Head",
        2 => "Neck",
        3 => "Shoulder",
        4 => "Shirt",
        5 => "Chest",
        6 => "Waist",
        7 => "Legs",
        8 => "Feet",
        9 => "Wrist",
        10 => "Hands",
        11 => "Finger",
        12 => "Trinket",
        14 => "Shield",
        16 => "Back",
        _ => "Junk",
    }
}

/// Map inventory type to a human-readable equip slot label.
pub(super) fn item_equip_slot_label(inv_type: u8) -> &'static str {
    match inv_type {
        1 => "Head",
        2 => "Neck",
        3 => "Shoulder",
        4 => "Shirt",
        5 | 20 => "Chest",
        6 => "Waist",
        7 => "Legs",
        8 => "Feet",
        9 => "Wrist",
        10 => "Hands",
        11 => "Finger",
        12 => "Trinket",
        13 => "One-Hand",
        14 => "Shield",
        15 => "Ranged",
        16 => "Back",
        17 => "Two-Hand",
        21 => "Main Hand",
        22 => "Off Hand",
        23 => "Held In Off-hand",
        25 => "Thrown",
        26 => "Ranged",
        _ => "",
    }
}

/// Map inventory type to WoW equip location string.
pub(super) fn inv_type_to_equip_loc(inv_type: u8) -> &'static str {
    match inv_type {
        1 => "INVTYPE_HEAD",
        2 => "INVTYPE_NECK",
        3 => "INVTYPE_SHOULDER",
        4 => "INVTYPE_BODY",
        5 => "INVTYPE_CHEST",
        6 => "INVTYPE_WAIST",
        7 => "INVTYPE_LEGS",
        8 => "INVTYPE_FEET",
        9 => "INVTYPE_WRIST",
        10 => "INVTYPE_HAND",
        11 => "INVTYPE_FINGER",
        12 => "INVTYPE_TRINKET",
        13 => "INVTYPE_WEAPON",
        14 => "INVTYPE_SHIELD",
        15 => "INVTYPE_RANGED",
        16 => "INVTYPE_CLOAK",
        17 => "INVTYPE_2HWEAPON",
        20 => "INVTYPE_ROBE",
        21 => "INVTYPE_WEAPONMAINHAND",
        22 => "INVTYPE_WEAPONOFFHAND",
        23 => "INVTYPE_HOLDABLE",
        25 => "INVTYPE_THROWN",
        26 => "INVTYPE_RANGEDRIGHT",
        _ => "",
    }
}
