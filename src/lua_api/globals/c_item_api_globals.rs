//! Legacy item, spell, inventory, and encoding globals related to item APIs.

use super::c_item_api::{
    inv_type_to_equip_loc, item_class_from_inv_type, item_class_name_extended, parse_item_id,
    quality_color,
};
use crate::lua_api::state::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_item_global_apis(lua: &Lua) -> Result<()> {
    register_c_encoding_util(lua)?;
    register_legacy_item_globals(lua)?;
    register_spell_globals(lua)?;
    register_inventory_globals(lua)?;
    Ok(())
}

/// Register the C_EncodingUtil namespace (stub compression/encoding).
fn register_c_encoding_util(lua: &Lua) -> Result<()> {
    let c_encoding = lua.create_table()?;
    register_identity_string_codec(lua, &c_encoding, "CompressString")?;
    register_identity_string_codec(lua, &c_encoding, "DecompressString")?;
    register_base64_stub(lua, &c_encoding, "EncodeBase64")?;
    register_base64_stub(lua, &c_encoding, "DecodeBase64")?;
    lua.globals().set("C_EncodingUtil", c_encoding)?;
    Ok(())
}

fn register_identity_string_codec(lua: &Lua, t: &mlua::Table, name: &str) -> Result<()> {
    t.set(
        name,
        lua.create_function(|lua, (data, _method): (String, Option<i32>)| {
            Ok(Value::String(lua.create_string(&data)?))
        })?,
    )?;
    Ok(())
}

fn register_base64_stub(lua: &Lua, t: &mlua::Table, name: &str) -> Result<()> {
    t.set(
        name,
        lua.create_function(|lua, data: String| Ok(Value::String(lua.create_string(&data)?)))?,
    )?;
    Ok(())
}

/// Register legacy global item functions (GetItemInfo, GetItemID, etc.).
fn register_legacy_item_globals(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set("GetItemInfo", make_legacy_get_item_info(lua)?)?;
    globals.set(
        "GetItemID",
        lua.create_function(|_, item_link: Option<String>| {
            Ok(item_link.and_then(|link| parse_item_id_from_link(&link)))
        })?,
    )?;
    globals.set(
        "GetItemCount",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(0))?,
    )?;
    register_legacy_item_stubs(lua)?;
    Ok(())
}

/// Legacy global stubs: GetItemClassInfo, GetItemSpecInfo, IsArtifactRelicItem, etc.
fn register_legacy_item_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetItemClassInfo",
        lua.create_function(|lua, class_id: i32| {
            let name = item_class_name_extended(class_id);
            if name.is_empty() {
                Ok(Value::Nil)
            } else {
                Ok(Value::String(lua.create_string(name)?))
            }
        })?,
    )?;
    globals.set(
        "GetItemSpecInfo",
        lua.create_function(|_, _item_id: i32| Ok(Value::Nil))?,
    )?;
    globals.set(
        "IsArtifactRelicItem",
        lua.create_function(|_, _item_id: i32| Ok(false))?,
    )?;
    globals.set(
        "GetTradeSkillTexture",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    Ok(())
}

/// Build the legacy global GetItemInfo closure (returns 17 positional values).
fn make_legacy_get_item_info(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|lua, item_id: Value| {
        let id = parse_item_id(&item_id);
        if id == 0 {
            return Ok(mlua::MultiValue::new());
        }
        let Some(item) = crate::items::get_item(id as u32) else {
            return Ok(mlua::MultiValue::new());
        };
        let color = quality_color(item.quality);
        let link = format!(
            "|cff{}|Hitem:{}::::::::80:::::|h[{}]|h|r",
            color, id, item.name
        );
        Ok(mlua::MultiValue::from_vec(vec![
            Value::String(lua.create_string(item.name)?),
            Value::String(lua.create_string(&link)?),
            Value::Integer(item.quality as i64),
            Value::Integer(item.item_level as i64),
            Value::Integer(item.required_level as i64),
            Value::String(lua.create_string(item_class_from_inv_type(item.inventory_type))?),
            Value::String(lua.create_string("")?),
            Value::Integer(item.stackable as i64),
            Value::String(lua.create_string(inv_type_to_equip_loc(item.inventory_type))?),
            Value::Integer(134400),
            Value::Integer(item.sell_price as i64),
            Value::Integer(15),
            Value::Integer(0),
            Value::Integer(item.bonding as i64),
            Value::Integer(item.expansion_id as i64),
            Value::Nil,
            Value::Boolean(false),
        ]))
    })
}

/// Register spell-related global functions.
fn register_spell_globals(lua: &Lua) -> Result<()> {
    register_spell_query_globals(lua)?;
    register_spell_stub_globals(lua)?;
    Ok(())
}

/// Spell query globals: link, icon, texture.
fn register_spell_query_globals(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "GetSpellLink",
        lua.create_function(|lua, spell_id: i32| {
            let name = crate::spells::get_spell(spell_id as u32)
                .map(|s| s.name)
                .unwrap_or("Unknown");
            let link = format!("|cff71d5ff|Hspell:{}|h[{}]|h|r", spell_id, name);
            Ok(Value::String(lua.create_string(&link)?))
        })?,
    )?;

    globals.set(
        "GetSpellIcon",
        lua.create_function(|_, spell_id: i32| {
            let icon = crate::spells::get_spell(spell_id as u32)
                .map(|s| s.icon_file_data_id)
                .unwrap_or(136243);
            Ok(icon)
        })?,
    )?;

    globals.set(
        "GetSpellTexture",
        lua.create_function(|_, spell_id: i32| {
            let file_id = crate::spells::get_spell(spell_id as u32)
                .map(|s| s.icon_file_data_id)
                .unwrap_or(136243);
            Ok(crate::manifest_interface_data::get_texture_path(file_id).unwrap_or(""))
        })?,
    )?;

    Ok(())
}

/// Spell stub globals: cooldown, known checks, chat.
fn register_spell_stub_globals(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "GetSpellCooldown",
        lua.create_function(|_, _spell_id: Value| Ok((0.0_f64, 0.0_f64, 1, 1.0_f64)))?,
    )?;

    globals.set(
        "IsSpellKnown",
        lua.create_function(|_, args: mlua::MultiValue| {
            let spell_id = args
                .iter()
                .next()
                .and_then(|v| match v {
                    mlua::Value::Integer(n) => Some(*n as u32),
                    _ => None,
                })
                .unwrap_or(0);
            Ok(super::spellbook_data::is_spell_known(spell_id))
        })?,
    )?;

    globals.set(
        "IsPlayerSpell",
        lua.create_function(|_, spell_id: i32| {
            Ok(super::spellbook_data::is_spell_known(spell_id as u32))
        })?,
    )?;

    globals.set(
        "IsSpellKnownOrOverridesKnown",
        lua.create_function(|_, spell_id: i32| {
            Ok(super::spellbook_data::find_spell_slot(spell_id as u32).is_some())
        })?,
    )?;

    globals.set(
        "SpellCanTargetItem",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?,
    )?;
    globals.set(
        "SpellCanTargetItemID",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?,
    )?;
    globals.set(
        "SendChatMessage",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    globals.set(
        "SpellGetVisibilityInfo",
        lua.create_function(|_, (_spell_id, _ctx): (i32, String)| Ok((false, false, false)))?,
    )?;

    Ok(())
}

/// Register inventory slot functions.
fn register_inventory_globals(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "GetInventorySlotInfo",
        lua.create_function(|_, slot_name: String| {
            Ok((
                inventory_slot_id(&slot_name),
                slot_texture_file_data_id(&slot_name),
            ))
        })?,
    )?;
    globals.set(
        "GetInventoryItemLink",
        lua.create_function(|lua, (_unit, slot): (String, i32)| {
            let item_id = get_equipped_item_id(lua, slot);
            match item_id {
                Some(id) if id > 0 => {
                    let item = crate::items::get_item(id);
                    let name = item.map_or("Unknown", |i| i.name);
                    let quality = item.map_or(4, |i| i.quality);
                    let color = quality_color(quality);
                    let link = format!(
                        "|c{color}|Hitem:{id}::::::::80:::::::::|h[{name}]|h|r"
                    );
                    Ok(Value::String(lua.create_string(&link)?))
                }
                _ => Ok(Value::Nil),
            }
        })?,
    )?;
    globals.set(
        "GetInventoryItemID",
        lua.create_function(|lua, (_unit, slot): (String, i32)| {
            match get_equipped_item_id(lua, slot) {
                Some(id) if id > 0 => Ok(Value::Integer(id as i64)),
                _ => Ok(Value::Nil),
            }
        })?,
    )?;
    globals.set(
        "GetInventoryItemTexture",
        lua.create_function(|lua, (_unit, slot): (String, i32)| {
            if (20..=24).contains(&slot) {
                return Ok(Value::String(
                    lua.create_string("Interface\\Icons\\INV_Misc_Bag_08")?,
                ));
            }
            match get_equipped_item_id(lua, slot) {
                Some(id) if id > 0 => {
                    // Return a generic icon fileDataID (items don't store icon yet)
                    Ok(Value::Integer(134400))
                }
                _ => Ok(Value::Nil),
            }
        })?,
    )?;
    globals.set(
        "GetInventoryItemCount",
        lua.create_function(|_, (_unit, _slot): (String, i32)| Ok(0))?,
    )?;
    globals.set(
        "GetInventoryItemBroken",
        lua.create_function(|_, (_unit, _slot): (String, i32)| Ok(false))?,
    )?;
    globals.set(
        "GetInventoryItemEquippedUnusable",
        lua.create_function(|_, (_unit, _slot): (String, i32)| Ok(false))?,
    )?;
    globals.set(
        "GetInventoryItemCooldown",
        lua.create_function(|_, (_unit, _slot): (String, i32)| Ok((0.0_f64, 0.0_f64, 1i32)))?,
    )?;

    Ok(())
}

fn get_equipped_item_id(lua: &Lua, slot: i32) -> Option<u32> {
    let state_rc = lua.app_data_ref::<Rc<RefCell<SimState>>>()?;
    let state = state_rc.borrow();
    state.player.equipped_items.get(&slot).map(|e| e.item_id)
}

fn parse_item_id_from_link(link: &str) -> Option<i32> {
    let start = link.find("|Hitem:")? + 7;
    let rest = &link[start..];
    let end = rest.find(':')?;
    rest[..end].parse().ok()
}

fn inventory_slot_id(slot_name: &str) -> i32 {
    match slot_name {
        "HeadSlot" => 1,
        "NeckSlot" => 2,
        "ShoulderSlot" => 3,
        "BackSlot" => 15,
        "ChestSlot" => 5,
        "ShirtSlot" => 4,
        "TabardSlot" => 19,
        "WristSlot" => 9,
        "HandsSlot" => 10,
        "WaistSlot" => 6,
        "LegsSlot" => 7,
        "FeetSlot" => 8,
        "Finger0Slot" => 11,
        "Finger1Slot" => 12,
        "Trinket0Slot" => 13,
        "Trinket1Slot" => 14,
        "MainHandSlot" => 16,
        "SecondaryHandSlot" => 17,
        "RangedSlot" => 18,
        "AmmoSlot" => 0,
        "Bag0Slot" => 20,
        "Bag1Slot" => 21,
        "Bag2Slot" => 22,
        "Bag3Slot" => 23,
        "ReagentBag0Slot" => 24,
        _ => 0,
    }
}

fn slot_texture_file_data_id(slot_name: &str) -> i32 {
    match slot_name {
        "HeadSlot" => 136516,
        "NeckSlot" => 136519,
        "ShoulderSlot" => 136526,
        "ShirtSlot" => 136525,
        "ChestSlot" => 136512,
        "WaistSlot" => 136529,
        "LegsSlot" => 136517,
        "FeetSlot" => 136513,
        "WristSlot" => 136530,
        "HandsSlot" => 136515,
        "Finger0Slot" | "Finger1Slot" => 136514,
        "Trinket0Slot" | "Trinket1Slot" => 136528,
        "BackSlot" => 136521,
        "MainHandSlot" => 136518,
        "SecondaryHandSlot" => 136524,
        "RangedSlot" => 136520,
        "TabardSlot" => 136527,
        "AmmoSlot" => 136510,
        "Bag0Slot" | "Bag1Slot" | "Bag2Slot" | "Bag3Slot" | "ReagentBag0Slot" => 136511,
        _ => 136516,
    }
}
