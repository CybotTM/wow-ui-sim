use super::helpers::{
    inv_type_to_class_id, inv_type_to_equip_loc, inv_type_to_subclass, item_class_from_inv_type,
    item_subclass_name, quality_color_hex,
};
use crate::items;
use crate::lua_api::globals::missing_surface::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_string, table_get, val_to_string};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn parse_prefixed_id(value: &str, prefix: &str) -> Option<u32> {
    let needle = format!("|H{prefix}:");
    let start = value.find(&needle)? + needle.len();
    let digits: String = value[start..]
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect();
    digits.parse().ok()
}

fn parse_item_id_from_val(state: &LuaState, value: Val) -> Option<u32> {
    match value {
        Val::Num(number) if number > 0.0 => Some(number as u32),
        Val::Str(_) => val_to_string(state, value)
            .and_then(|text| parse_prefixed_id(&text, "item").or_else(|| text.parse().ok())),
        _ => None,
    }
}

pub(super) fn item_link_for_id(item_id: u32) -> Option<String> {
    let item = items::get_item(item_id)?;
    Some(format!(
        "|cff{}|Hitem:{}::::::::80:::::|h[{}]|h|r",
        quality_color_hex(item.quality),
        item_id,
        item.name
    ))
}

pub(crate) fn spell_link_for_id(spell_id: u32) -> Option<String> {
    use crate::spells;
    let spell = spells::get_spell(spell_id)?;
    Some(format!(
        "|cff71d5ff|Hspell:{}|h[{}]|h|r",
        spell_id, spell.name
    ))
}

fn item_guid_for_bag_slot(bag: i32, slot: i32, item_id: u32) -> String {
    format!("Item-{bag}-{slot}-{item_id}")
}

pub(crate) fn parse_item_guid(guid: &str) -> Option<(i32, i32, u32)> {
    let mut parts = guid.split('-');
    if parts.next()? != "Item" {
        return None;
    }
    Some((
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    ))
}

pub(super) fn register_c_item(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Item")?;
    let methods: &[(&str, fn(&mut LuaState) -> LuaResult<u32>)] = &[
        ("GetItemIconByID", c_item_get_item_icon_by_id),
        ("GetItemNameByID", c_item_get_item_name_by_id),
        ("GetItemQualityByID", c_item_get_item_quality_by_id),
        ("GetItemInfoInstant", c_item_get_item_info_instant),
        ("GetItemInfo", c_item_get_item_info),
        (
            "GetDetailedItemLevelInfo",
            c_item_get_detailed_item_level_info,
        ),
        ("GetItemSubClassInfo", c_item_get_item_sub_class_info),
        ("GetItemLink", c_item_get_item_link),
        ("GetItemGUID", c_item_get_item_guid),
        ("GetItemInventorySlotInfo", c_item_get_item_inventory_slot_info),
    ];
    for &(name, func) in methods {
        table_set_rust_fn(state, table_ref, name, func)?;
    }
    Ok(())
}

fn c_item_get_item_icon_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = parse_item_id_from_val(state, stack_val(state, 1)).unwrap_or(0);
    let icon = items::get_item(item_id)
        .map(|item| {
            if item.icon_file_data_id == 0 {
                134400.0
            } else {
                item.icon_file_data_id as f64
            }
        })
        .unwrap_or(134400.0);
    state.push(Val::Num(icon));
    Ok(1)
}

fn c_item_get_item_name_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = parse_item_id_from_val(state, stack_val(state, 1)).unwrap_or(0);
    let name = items::get_item(item_id)
        .map(|item| item.name)
        .unwrap_or("Unknown");
    let name = create_string(state, name);
    state.push(name);
    Ok(1)
}

fn c_item_get_item_quality_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = parse_item_id_from_val(state, stack_val(state, 1)).unwrap_or(0);
    let quality = items::get_item(item_id)
        .map(|item| item.quality as f64)
        .unwrap_or(0.0);
    state.push(Val::Num(quality));
    Ok(1)
}

fn c_item_get_item_info_instant(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = parse_item_id_from_val(state, stack_val(state, 1)).unwrap_or(0);
    let Some(item) = items::get_item(item_id) else {
        return Ok(0);
    };
    let class_name = create_string(state, item_class_from_inv_type(item.inventory_type));
    let subclass_name = create_string(state, inv_type_to_subclass(item.inventory_type));
    let empty = create_string(state, "");
    state.push(Val::Num(item_id as f64));
    state.push(class_name);
    state.push(subclass_name);
    state.push(empty);
    state.push(Val::Num(if item.icon_file_data_id == 0 {
        134400.0
    } else {
        item.icon_file_data_id as f64
    }));
    state.push(Val::Num(inv_type_to_class_id(item.inventory_type) as f64));
    state.push(Val::Num(0.0));
    Ok(7)
}

fn c_item_get_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = parse_item_id_from_val(state, stack_val(state, 1)).unwrap_or(0);
    let Some(item) = items::get_item(item_id) else {
        return Ok(0);
    };
    let item_name = create_string(state, item.name);
    let item_link = create_string(state, &item_link_for_id(item_id).unwrap_or_default());
    let class_name = create_string(state, item_class_from_inv_type(item.inventory_type));
    let subclass_name = create_string(state, inv_type_to_subclass(item.inventory_type));
    let equip_loc = create_string(state, inv_type_to_equip_loc(item.inventory_type));
    state.push(item_name);
    state.push(item_link);
    state.push(Val::Num(item.quality as f64));
    state.push(Val::Num(item.item_level as f64));
    state.push(Val::Num(item.required_level as f64));
    state.push(class_name);
    state.push(subclass_name);
    state.push(Val::Num(item.stackable as f64));
    state.push(equip_loc);
    state.push(Val::Num(if item.icon_file_data_id == 0 {
        134400.0
    } else {
        item.icon_file_data_id as f64
    }));
    state.push(Val::Num(item.sell_price as f64));
    state.push(Val::Num(inv_type_to_class_id(item.inventory_type) as f64));
    state.push(Val::Num(0.0));
    state.push(Val::Num(item.bonding as f64));
    state.push(Val::Num(item.expansion_id as f64));
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    Ok(17)
}

fn c_item_get_detailed_item_level_info(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = parse_item_id_from_val(state, stack_val(state, 1)).unwrap_or(0);
    let level = items::get_item(item_id)
        .map(|item| item.item_level as f64)
        .unwrap_or(0.0);
    state.push(Val::Num(level));
    state.push(Val::Num(0.0));
    state.push(Val::Num(level));
    Ok(3)
}

fn c_item_get_item_sub_class_info(state: &mut LuaState) -> LuaResult<u32> {
    let class_id = i32::from_stack(state, 1)?;
    let subclass_id = i32::from_stack(state, 2)?;
    let name = create_string(state, item_subclass_name(class_id, subclass_id));
    state.push(name);
    Ok(1)
}

fn c_item_get_item_link(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = parse_item_id_from_val(state, stack_val(state, 1)).unwrap_or(0);
    match item_link_for_id(item_id) {
        Some(link) => {
            let link = create_string(state, &link);
            state.push(link);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_item_get_item_guid(state: &mut LuaState) -> LuaResult<u32> {
    let location = stack_val(state, 1);
    let bag = match table_get(state, location, "bagID") {
        Val::Num(value) => value as i32,
        _ => 0,
    };
    let slot = match table_get(state, location, "slotIndex") {
        Val::Num(value) => value as i32,
        _ => 0,
    };
    let item_id = borrow_state(state)?
        .get_bag_item(bag, slot)
        .map(|(item_id, _)| item_id);
    match item_id {
        Some(item_id) => {
            let guid = create_string(state, &item_guid_for_bag_slot(bag, slot, item_id));
            state.push(guid);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_item_get_item_inventory_slot_info(state: &mut LuaState) -> LuaResult<u32> {
    let inv_type = i32::from_stack(state, 1)?;
    let label = create_string(state, inv_type_to_subclass(inv_type.max(0) as u8));
    state.push(label);
    Ok(1)
}
