use super::ensure_namespace;
use crate::items;
use crate::lua_api::globals::{currency_data, spellbook_data};
use crate::lua_api::rilua_methods::{
    borrow_state, create_string, create_table, table_get, table_set, val_to_string,
};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn};
use crate::spell_descriptions;
use crate::spells;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub(super) fn register_item_and_spell_surfaces(state: &mut LuaState) -> LuaResult<()> {
    register_c_item(state)?;
    register_c_item_upgrade(state)?;
    register_c_container(state)?;
    register_c_currency_info(state)?;
    register_c_equipment_set(state)?;
    register_c_bank(state)?;
    register_c_spell(state)?;
    register_c_spell_book(state)?;
    Ok(())
}

fn quality_color_hex(quality: u8) -> &'static str {
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

fn item_class_from_inv_type(inv_type: u8) -> &'static str {
    match inv_type {
        13 | 15 | 17 | 21 | 22 | 25 | 26 => "Weapon",
        1..=12 | 14 | 16 | 23 => "Armor",
        _ => "Miscellaneous",
    }
}

fn inv_type_to_class_id(inv_type: u8) -> i32 {
    match inv_type {
        13 | 15 | 17 | 21 | 22 | 25 | 26 => 2,
        1..=12 | 14 | 16 | 23 => 4,
        _ => 15,
    }
}

fn item_subclass_name(class_id: i32, subclass_id: i32) -> &'static str {
    match (class_id, subclass_id) {
        (4, 1) => "Cloth",
        (4, 2) => "Leather",
        (4, 3) => "Mail",
        (4, 4) => "Plate",
        (4, 6) => "Shield",
        (7, 4) => "Cooking",
        _ => "Unknown",
    }
}

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

fn inv_type_to_equip_loc(inv_type: u8) -> &'static str {
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
        _ => "",
    }
}

pub(super) fn parse_prefixed_id(value: &str, prefix: &str) -> Option<u32> {
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

pub(super) fn spell_link_for_id(spell_id: u32) -> Option<String> {
    let spell = spells::get_spell(spell_id)?;
    Some(format!(
        "|cff71d5ff|Hspell:{}|h[{}]|h|r",
        spell_id, spell.name
    ))
}

fn item_guid_for_bag_slot(bag: i32, slot: i32, item_id: u32) -> String {
    format!("Item-{bag}-{slot}-{item_id}")
}

pub(super) fn parse_item_guid(guid: &str) -> Option<(i32, i32, u32)> {
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

pub(super) fn current_item_upgrade_location(state: &mut LuaState) -> Option<(i32, i32)> {
    let storage = global_table(state, "__item_upgrade_state");
    let location = table_get(state, storage, "location");
    let Val::Table(_) = location else { return None };
    let bag = match table_get(state, location, "bagID") {
        Val::Num(value) => value as i32,
        _ => return None,
    };
    let slot = match table_get(state, location, "slotIndex") {
        Val::Num(value) => value as i32,
        _ => return None,
    };
    Some((bag, slot))
}

fn global_table(state: &mut LuaState, name: &str) -> Val {
    let key_ref = state.gc.intern_string(name.as_bytes());
    let current = state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    if matches!(current, Val::Table(_)) {
        return current;
    }
    let table = create_table(state);
    let global = state.global;
    if let Some(globals) = state.gc.tables.get_mut(global) {
        let _ = globals.raw_set(Val::Str(key_ref), table, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    table
}

fn register_c_item(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Item")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetItemIconByID",
        c_item_get_item_icon_by_id,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetItemNameByID",
        c_item_get_item_name_by_id,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetItemQualityByID",
        c_item_get_item_quality_by_id,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetItemInfoInstant",
        c_item_get_item_info_instant,
    )?;
    table_set_rust_fn(state, table_ref, "GetItemInfo", c_item_get_item_info)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetDetailedItemLevelInfo",
        c_item_get_detailed_item_level_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetItemSubClassInfo",
        c_item_get_item_sub_class_info,
    )?;
    table_set_rust_fn(state, table_ref, "GetItemLink", c_item_get_item_link)?;
    table_set_rust_fn(state, table_ref, "GetItemGUID", c_item_get_item_guid)?;
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

fn register_c_item_upgrade(state: &mut LuaState) -> LuaResult<()> {
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

fn register_c_container(state: &mut LuaState) -> LuaResult<()> {
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

fn register_c_currency_info(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_CurrencyInfo")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetCurrencyListSize",
        c_currency_get_list_size,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetCurrencyListInfo",
        c_currency_get_list_info,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetCoinTextureString",
        c_currency_get_coin_texture_string,
    )?;
    Ok(())
}

fn c_currency_get_list_size(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(currency_data::currency_list_size() as f64));
    Ok(1)
}

fn c_currency_get_list_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let Some(entry) = currency_data::get_currency_list_entry(index) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    let name = create_string(state, entry.name);
    table_set(
        state,
        info,
        "currencyTypesID",
        Val::Num(entry.currency_id as f64),
    );
    table_set(state, info, "name", name);
    table_set(state, info, "quantity", Val::Num(entry.quantity as f64));
    table_set(
        state,
        info,
        "iconFileID",
        Val::Num(entry.icon_file_id as f64),
    );
    table_set(state, info, "isHeader", Val::Bool(entry.is_header));
    table_set(
        state,
        info,
        "isHeaderExpanded",
        Val::Bool(entry.is_header_expanded),
    );
    table_set(state, info, "quality", Val::Num(entry.quality as f64));
    state.push(info);
    Ok(1)
}

fn c_currency_get_coin_texture_string(state: &mut LuaState) -> LuaResult<u32> {
    let amount = i64::from_stack(state, 1)?;
    let amount = create_string(state, &format!("{amount}"));
    state.push(amount);
    Ok(1)
}

fn register_c_equipment_set(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_EquipmentSet")?;
    table_set_rust_fn(state, table_ref, "GetEquipmentSetIDs", c_equipment_set_ids)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetEquipmentSetInfo",
        c_equipment_set_info,
    )?;
    Ok(())
}

fn c_equipment_set_ids(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn c_equipment_set_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn register_c_bank(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Bank")?;
    table_set_rust_fn(
        state,
        table_ref,
        "FetchDepositedMoney",
        c_bank_fetch_deposited_money,
    )?;
    Ok(())
}

fn c_bank_fetch_deposited_money(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn register_c_spell(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Spell")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellDescription",
        c_spell_get_spell_description,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellTexture",
        c_spell_get_spell_texture,
    )?;
    table_set_rust_fn(state, table_ref, "GetSpellLink", c_spell_get_spell_link)?;
    table_set_rust_fn(state, table_ref, "GetSpellName", c_spell_get_spell_name)?;
    Ok(())
}

fn c_spell_get_spell_description(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let description = create_string(
        state,
        spell_descriptions::get_spell_description(spell_id).unwrap_or(""),
    );
    state.push(description);
    Ok(1)
}

fn c_spell_get_spell_texture(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let icon = spells::get_spell(spell_id)
        .map(|spell| spell.icon_file_data_id)
        .unwrap_or(136243);
    let texture = create_string(state, "Interface\\ICONS\\INV_Misc_QuestionMark");
    state.push(texture);
    state.push(Val::Num(icon as f64));
    Ok(2)
}

fn c_spell_get_spell_link(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    match spell_link_for_id(spell_id) {
        Some(link) => {
            let link = create_string(state, &link);
            state.push(link);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_spell_get_spell_name(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let name = spells::get_spell(spell_id)
        .map(|spell| spell.name)
        .unwrap_or("Unknown");
    let name = create_string(state, name);
    state.push(name);
    Ok(1)
}

fn register_c_spell_book(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_SpellBook")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellBookItemName",
        c_spell_book_get_spell_book_item_name,
    )?;
    Ok(())
}

fn c_spell_book_get_spell_book_item_name(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    match spellbook_data::get_spell_at_slot(slot) {
        Some((_, entry, _)) => {
            let name = spells::get_spell(entry.spell_id)
                .map(|spell| spell.name)
                .unwrap_or("Unknown");
            let name = create_string(state, name);
            state.push(name);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}
