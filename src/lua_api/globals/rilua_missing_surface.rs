//! Restored rilua API surface for item, spell, tooltip, and small legacy globals.

use crate::items;
use crate::lua_api::game_data::CLASS_LABELS;
use crate::lua_api::globals::{currency_data, hero_talents, spell_api, spellbook_data};
use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, create_table, frame_ref,
    table_get, table_set, val_to_string,
};
use crate::lua_api::rilua_script_helpers::{get_event_listeners, get_script};
use crate::lua_api::state::RACE_DATA;
use crate::lua_api::talent_state;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn};
use crate::specializations;
use crate::spell_descriptions;
use crate::spells;
use crate::traits::{
    TRAIT_COND_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB, TRAIT_SUBTREE_DB, TRAIT_TREE_DB,
};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val};

const TOOLTIP_TYPE_ITEM: f64 = 0.0;
const TOOLTIP_TYPE_SPELL: f64 = 1.0;
const TOOLTIP_TYPE_UNIT: f64 = 2.0;
const TOOLTIP_TYPE_UNIT_AURA: f64 = 7.0;
const TOOLTIP_TYPE_MINIMAP_MOUSEOVER: f64 = 21.0;

const LINE_TYPE_UNIT_NAME: f64 = 2.0;
const LINE_TYPE_SPELL_NAME: f64 = 13.0;
const LINE_TYPE_ITEM_BINDING: f64 = 20.0;
const LINE_TYPE_EQUIP_SLOT: f64 = 21.0;
const LINE_TYPE_ITEM_NAME: f64 = 22.0;
const LINE_TYPE_ITEM_LEVEL: f64 = 31.0;
const LINE_TYPE_SPELL_DESCRIPTION: f64 = 34.0;

const WORLD_LOOT_TOOLTIP_SPELL_ID: u32 = 19750;
const WORLD_LOOT_TOOLTIP_INVENTORY_TYPE: f64 = 13.0;
const WORLD_CURSOR_GUID: &str = "WorldLootObject-0000-0000C0DE";

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "PlaySound", noop)?;
    LuaApiMut::register_function(lua, "PlaySoundFile", noop)?;
    LuaApiMut::register_function(lua, "StopSound", noop)?;
    LuaApiMut::register_function(lua, "GetSpellLink", get_spell_link_global)?;
    LuaApiMut::register_function(lua, "GetRepairAllCost", get_repair_all_cost)?;
    LuaApiMut::register_function(lua, "SetActionUIButton", set_action_ui_button)?;
    LuaApiMut::register_function(lua, "MapSceneCharacterHighlightStart", noop)?;
    LuaApiMut::register_function(lua, "MapSceneCharacterHighlightEnd", noop)?;
    LuaApiMut::register_function(lua, "CreateAtlasMarkup", create_atlas_markup)?;
    LuaApiMut::register_function(lua, "InGlue", in_glue)?;
    LuaApiMut::register_function(lua, "strsub", strsub)?;

    let state = lua.state_mut();
    ensure_global_table(state, "UISpecialFrames");
    ensure_global_table(state, "StaticPopupDialogs");
    ensure_global_table(state, "UIPanelWindows");
    ensure_global_table(state, "SOUNDKIT");
    register_c_item(state)?;
    register_c_item_upgrade(state)?;
    register_c_container(state)?;
    register_c_currency_info(state)?;
    register_c_equipment_set(state)?;
    register_c_bank(state)?;
    register_c_spell(state)?;
    register_c_spell_book(state)?;
    register_c_traits(state)?;
    register_c_class_talents(state)?;
    register_c_tooltip_info(state)?;
    Ok(())
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn ensure_global_table(state: &mut LuaState, name: &str) {
    let _ = ensure_namespace(state, name);
}

fn set_table_array(state: &mut LuaState, table: Val, index: i64, value: Val) {
    let Val::Table(table_ref) = table else { return };
    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        let _ = table.raw_set(Val::Num(index as f64), value, &state.gc.string_arena);
    }
}

fn get_repair_all_cost(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    Ok(2)
}

fn create_atlas_markup(state: &mut LuaState) -> LuaResult<u32> {
    let atlas_name = match stack_val(state, 1) {
        Val::Str(_) => val_to_string(state, stack_val(state, 1)).unwrap_or_default(),
        _ => String::new(),
    };
    let text = if atlas_name.is_empty() {
        String::new()
    } else {
        format!("|A:{atlas_name}:0:0|a")
    };
    let value = create_string(state, &text);
    state.push(value);
    Ok(1)
}

fn in_glue(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn strsub(state: &mut LuaState) -> LuaResult<u32> {
    let Some(text) = val_to_string(state, stack_val(state, 1)) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let start = i32::from_stack(state, 2).unwrap_or(1);
    let end = i32::from_stack(state, 3).unwrap_or(-1);
    let len = text.chars().count() as i32;
    let normalize = |index: i32| {
        if index < 0 {
            (len + index + 1).max(1)
        } else {
            index.max(1)
        }
    };
    let start = normalize(start);
    let end = normalize(end).min(len);
    let result = if start > end || start > len {
        String::new()
    } else {
        text.chars()
            .skip((start - 1) as usize)
            .take((end - start + 1) as usize)
            .collect::<String>()
    };
    let value = create_string(state, &result);
    state.push(value);
    Ok(1)
}

fn ensure_namespace(
    state: &mut LuaState,
    name: &str,
) -> LuaResult<rilua::vm::gc::arena::GcRef<Table>> {
    let key_ref = state.gc.intern_string(name.as_bytes());
    let current = state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    let table_ref = match current {
        Val::Table(table_ref) => table_ref,
        _ => {
            let table_ref = state.gc.alloc_table(Table::new());
            if let Some(globals) = state.gc.tables.get_mut(state.global) {
                let _ = globals.raw_set(
                    Val::Str(key_ref),
                    Val::Table(table_ref),
                    &state.gc.string_arena,
                );
            }
            table_ref
        }
    };
    Ok(table_ref)
}

fn color_table(state: &mut LuaState, r: f64, g: f64, b: f64, a: f64) -> Val {
    let key = state.gc.intern_string(b"CreateColor");
    let create_color = state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(key, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    match call_function_state(
        state,
        create_color,
        &[Val::Num(r), Val::Num(g), Val::Num(b), Val::Num(a)],
    ) {
        Ok(color) => color,
        Err(_) => create_table(state),
    }
}

fn push_tooltip_line(
    state: &mut LuaState,
    lines: Val,
    index: i64,
    line_type: f64,
    left_text: &str,
    left_color: Option<(f64, f64, f64)>,
    wrap: bool,
) {
    let line = create_table(state);
    table_set(state, line, "type", Val::Num(line_type));
    let left_text_val = create_string(state, left_text);
    table_set(state, line, "leftText", left_text_val);
    if let Some((r, g, b)) = left_color {
        let color = color_table(state, r, g, b, 1.0);
        table_set(state, line, "leftColor", color);
    }
    if wrap {
        table_set(state, line, "wrapText", Val::Bool(true));
    }
    set_table_array(state, lines, index, line);
}

fn empty_tooltip(state: &mut LuaState, tooltip_type: f64) -> Val {
    let tooltip = create_table(state);
    let lines = create_table(state);
    table_set(state, tooltip, "type", Val::Num(tooltip_type));
    table_set(state, tooltip, "lines", lines);
    tooltip
}

fn item_quality_color(quality: u8) -> (f64, f64, f64) {
    match quality {
        0 => (0.62, 0.62, 0.62),
        1 => (1.0, 1.0, 1.0),
        2 => (0.12, 1.0, 0.0),
        3 => (0.0, 0.44, 0.87),
        4 => (0.64, 0.21, 0.93),
        5 => (1.0, 0.5, 0.0),
        _ => (1.0, 1.0, 1.0),
    }
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

fn item_class_name(class_id: i32) -> &'static str {
    match class_id {
        0 => "Consumable",
        1 => "Container",
        2 => "Weapon",
        3 => "Gem",
        4 => "Armor",
        5 => "Reagent",
        7 => "Tradeskill",
        12 => "Quest",
        15 => "Miscellaneous",
        _ => "Unknown",
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

fn item_equip_slot_label(inv_type: u8) -> &'static str {
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
        _ => "",
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

fn parse_prefixed_id(value: &str, prefix: &str) -> Option<u32> {
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

fn item_link_for_id(item_id: u32) -> Option<String> {
    let item = items::get_item(item_id)?;
    Some(format!(
        "|cff{}|Hitem:{}::::::::80:::::|h[{}]|h|r",
        quality_color_hex(item.quality),
        item_id,
        item.name
    ))
}

fn spell_link_for_id(spell_id: u32) -> Option<String> {
    let spell = spells::get_spell(spell_id)?;
    Some(format!(
        "|cff71d5ff|Hspell:{}|h[{}]|h|r",
        spell_id, spell.name
    ))
}

fn item_guid_for_bag_slot(bag: i32, slot: i32, item_id: u32) -> String {
    format!("Item-{bag}-{slot}-{item_id}")
}

fn parse_item_guid(guid: &str) -> Option<(i32, i32, u32)> {
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

fn current_item_upgrade_location(state: &mut LuaState) -> Option<(i32, i32)> {
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
    if let Some(globals) = state.gc.tables.get_mut(state.global) {
        let _ = globals.raw_set(Val::Str(key_ref), table, &state.gc.string_arena);
    }
    table
}

fn push_item_name_line(state: &mut LuaState, lines: Val, item: &items::ItemInfo) {
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_ITEM_NAME,
        item.name,
        Some(item_quality_color(item.quality)),
        false,
    );
}

fn push_item_level_line(state: &mut LuaState, lines: Val, item: &items::ItemInfo) {
    let item_level = format!("Item Level {}", item.item_level);
    push_tooltip_line(
        state,
        lines,
        2,
        LINE_TYPE_ITEM_LEVEL,
        &item_level,
        None,
        false,
    );
}

fn item_binding_text(state: &mut LuaState, bonding: u8) -> Option<String> {
    if bonding != 1 {
        return None;
    }
    let item_bind_key = state.gc.intern_string(b"ITEM_BIND_ON_PICKUP");
    state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(item_bind_key, &state.gc.string_arena))
        .and_then(|value| val_to_string(state, value))
        .or_else(|| Some("Binds when picked up".to_string()))
}

fn push_item_equip_slot_line(
    state: &mut LuaState,
    lines: Val,
    inventory_type: u8,
    next_index: &mut i64,
) {
    let equip_slot = item_equip_slot_label(inventory_type);
    if equip_slot.is_empty() {
        return;
    }
    push_tooltip_line(
        state,
        lines,
        *next_index,
        LINE_TYPE_EQUIP_SLOT,
        equip_slot,
        None,
        false,
    );
    *next_index += 1;
}

fn push_item_binding_line(
    state: &mut LuaState,
    lines: Val,
    next_index: i64,
    item: &items::ItemInfo,
) {
    let Some(binding) = item_binding_text(state, item.bonding) else {
        return;
    };
    push_tooltip_line(
        state,
        lines,
        next_index,
        LINE_TYPE_ITEM_BINDING,
        &binding,
        None,
        false,
    );
}

fn tooltip_for_item_id(state: &mut LuaState, item_id: u32) -> Val {
    let Some(item) = items::get_item(item_id) else {
        return empty_tooltip(state, TOOLTIP_TYPE_ITEM);
    };
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_ITEM);
    populate_item_tooltip_lines(state, tooltip, item);
    tooltip
}

fn populate_item_tooltip_lines(state: &mut LuaState, tooltip: Val, item: &items::ItemInfo) {
    let lines = table_get(state, tooltip, "lines");
    push_item_name_line(state, lines, item);
    push_item_level_line(state, lines, item);

    let mut next_index = 3;
    push_item_equip_slot_line(state, lines, item.inventory_type, &mut next_index);
    push_item_binding_line(state, lines, next_index, item);
}

fn spell_cost_line(spell_id: u32) -> Option<&'static str> {
    match spell_id {
        19750 => Some("10% of Base MANA"),
        _ => None,
    }
}

fn spell_cast_line(spell_id: u32) -> String {
    let cast_ms = spell_api::spell_cast_time(spell_id as i32);
    if cast_ms <= 0 {
        "Instant".to_string()
    } else {
        format!("{:.1} sec cast", cast_ms as f64 / 1000.0)
    }
}

fn tooltip_for_spell_id(state: &mut LuaState, spell_id: u32) -> Val {
    let Some(spell) = spells::get_spell(spell_id) else {
        return empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    };
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    let lines = table_get(state, tooltip, "lines");
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_SPELL_NAME,
        spell.name,
        None,
        false,
    );
    let mut next_index = 2;
    if let Some(cost) = spell_cost_line(spell_id) {
        push_tooltip_line(
            state,
            lines,
            next_index,
            LINE_TYPE_SPELL_NAME,
            cost,
            None,
            false,
        );
        next_index += 1;
    }
    let cast_line = spell_cast_line(spell_id);
    push_tooltip_line(
        state,
        lines,
        next_index,
        LINE_TYPE_SPELL_NAME,
        &cast_line,
        None,
        false,
    );
    next_index += 1;
    let description =
        spell_descriptions::get_spell_description(spell_id).unwrap_or("No description available.");
    push_tooltip_line(
        state,
        lines,
        next_index,
        LINE_TYPE_SPELL_DESCRIPTION,
        description,
        None,
        true,
    );
    table_set(state, tooltip, "id", Val::Num(spell_id as f64));
    tooltip
}

fn tooltip_for_unit_aura(
    state: &mut LuaState,
    aura: Option<crate::lua_api::game_data::AuraInfo>,
) -> Val {
    let Some(aura) = aura else {
        return empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA);
    };
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA);
    let lines = table_get(state, tooltip, "lines");
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_SPELL_NAME,
        &aura.name,
        None,
        false,
    );
    push_tooltip_line(state, lines, 2, LINE_TYPE_SPELL_NAME, "1 hr", None, false);
    let description = spell_descriptions::get_spell_description(aura.spell_id as u32)
        .unwrap_or("No description available.");
    push_tooltip_line(
        state,
        lines,
        3,
        LINE_TYPE_SPELL_DESCRIPTION,
        description,
        None,
        true,
    );
    tooltip
}

fn class_color(class_index: i32) -> (f64, f64, f64) {
    match class_index {
        1 => (0.78, 0.61, 0.43),
        2 => (0.96, 0.55, 0.73),
        3 => (0.67, 0.83, 0.45),
        4 => (1.0, 0.96, 0.41),
        5 => (1.0, 1.0, 1.0),
        6 => (0.77, 0.12, 0.23),
        7 => (0.0, 0.44, 0.87),
        8 => (0.25, 0.78, 0.92),
        9 => (0.53, 0.53, 0.93),
        10 => (0.0, 1.0, 0.6),
        11 => (1.0, 0.49, 0.04),
        12 => (0.64, 0.19, 0.79),
        13 => (0.2, 0.58, 0.5),
        _ => (1.0, 1.0, 1.0),
    }
}

struct UnitTooltipInfo {
    name: String,
    level: i32,
    race: String,
    class_name: String,
    color: (f64, f64, f64),
}

fn class_label(class_index: i32) -> String {
    CLASS_LABELS
        .get((class_index - 1).max(0) as usize)
        .copied()
        .unwrap_or("Unknown")
        .to_string()
}

fn unit_tooltip_info(state: &LuaState, unit: &str) -> Option<UnitTooltipInfo> {
    let sim = borrow_state(state).ok()?;
    match unit {
        "target" => sim.current_target.as_ref().map(|target| UnitTooltipInfo {
            name: target.name.clone(),
            level: target.level,
            race: target.creature_type.clone(),
            class_name: class_label(target.class_index),
            color: class_color(target.class_index),
        }),
        "player" => {
            let player = &sim.player;
            let race = RACE_DATA
                .get(player.race_index)
                .map(|(name, _, _)| (*name).to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            Some(UnitTooltipInfo {
                name: player.name.clone(),
                level: player.level,
                race,
                class_name: class_label(player.class_index),
                color: class_color(player.class_index),
            })
        }
        _ => None,
    }
}

fn push_unit_tooltip_lines(state: &mut LuaState, lines: Val, info: &UnitTooltipInfo) {
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_UNIT_NAME,
        &info.name,
        Some(info.color),
        false,
    );
    let level_text = format!("Level {}", info.level);
    push_tooltip_line(
        state,
        lines,
        2,
        LINE_TYPE_SPELL_NAME,
        &level_text,
        None,
        false,
    );
    push_tooltip_line(
        state,
        lines,
        3,
        LINE_TYPE_SPELL_NAME,
        &info.race,
        None,
        false,
    );
    push_tooltip_line(
        state,
        lines,
        4,
        LINE_TYPE_SPELL_NAME,
        &info.class_name,
        None,
        false,
    );
}

fn tooltip_for_unit(state: &mut LuaState, unit: &str) -> Val {
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_UNIT);
    let lines = table_get(state, tooltip, "lines");
    if let Some(info) = unit_tooltip_info(state, unit) {
        push_unit_tooltip_lines(state, lines, &info);
    }
    tooltip
}

fn tooltip_for_world_loot(state: &mut LuaState) -> Val {
    let tooltip = tooltip_for_spell_id(state, WORLD_LOOT_TOOLTIP_SPELL_ID);
    table_set(
        state,
        tooltip,
        "worldLootObjectInventoryType",
        Val::Num(WORLD_LOOT_TOOLTIP_INVENTORY_TYPE),
    );
    let guid = create_string(state, WORLD_CURSOR_GUID);
    table_set(state, tooltip, "worldLootObjectGUID", guid);
    tooltip
}

fn fire_named_event(state: &mut LuaState, event_name: &str) {
    for widget_id in get_event_listeners(state, event_name) {
        let Some(handler) = get_script(state, widget_id, "OnEvent") else {
            continue;
        };
        let Ok(frame) = frame_ref(state, widget_id) else {
            continue;
        };
        let event_name_val = create_string(state, event_name);
        let _ = call_function_state(state, handler, &[frame, event_name_val]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_api::env::WowLuaEnv;
    use rilua::Lua;

    #[test]
    fn tooltip_for_item_id_populates_name_and_level_lines() {
        let mut lua = Lua::new().expect("should create rilua state");
        let tooltip = {
            let state = lua.state_mut();
            tooltip_for_item_id(state, 6948)
        };
        let mut state = lua.state_mut();
        let lines = table_get(&mut state, tooltip, "lines");

        let name_line = get_array_element(&mut state, lines, 1);
        assert_eq!(line_type(&mut state, name_line), Some(LINE_TYPE_ITEM_NAME));
        assert_eq!(
            line_text(&mut state, name_line).as_deref(),
            Some("Hearthstone")
        );

        let level_line = get_array_element(&mut state, lines, 2);
        assert_eq!(
            line_type(&mut state, level_line),
            Some(LINE_TYPE_ITEM_LEVEL)
        );
        assert_eq!(
            line_text(&mut state, level_line).as_deref(),
            Some("Item Level 1")
        );
    }

    #[test]
    fn tooltip_for_unit_player_shows_name_and_level() {
        let env = WowLuaEnv::new().expect("should create WowLuaEnv");
        {
            let mut sim = env.state().borrow_mut();
            sim.player.name = "Tester".to_string();
            sim.player.level = 99;
            sim.player.class_index = 3;
            sim.player.race_index = 1;
        }
        let tooltip = {
            let mut lua = env.rilua_mut();
            tooltip_for_unit(lua.state_mut(), "player")
        };
        let mut lua = env.rilua_mut();
        let state = lua.state_mut();
        let lines = table_get(state, tooltip, "lines");
        let name_line = get_array_element(state, lines, 1);
        assert_eq!(line_text(state, name_line).as_deref(), Some("Tester"));

        let level_line = get_array_element(state, lines, 2);
        assert_eq!(line_text(state, level_line).as_deref(), Some("Level 99"));
    }

    fn get_array_element(state: &mut LuaState, table: Val, index: i64) -> Val {
        let Val::Table(table_ref) = table else {
            return Val::Nil;
        };
        state
            .gc
            .tables
            .get(table_ref)
            .map(|table| table.get_int(index))
            .unwrap_or(Val::Nil)
    }

    fn line_type(state: &mut LuaState, line: Val) -> Option<f64> {
        match table_get(state, line, "type") {
            Val::Num(value) => Some(value),
            _ => None,
        }
    }

    fn line_text(state: &mut LuaState, line: Val) -> Option<String> {
        let text_val = table_get(state, line, "leftText");
        val_to_string(state, text_val)
    }
}

fn fire_named_event_with_arg(state: &mut LuaState, event_name: &str, arg: Val) {
    for widget_id in get_event_listeners(state, event_name) {
        let Some(handler) = get_script(state, widget_id, "OnEvent") else {
            continue;
        };
        let Ok(frame) = frame_ref(state, widget_id) else {
            continue;
        };
        let event_name_val = create_string(state, event_name);
        let _ = call_function_state(state, handler, &[frame, event_name_val, arg]);
    }
}

fn current_spec_id(state: &LuaState) -> Option<u32> {
    let sim = borrow_state(state).ok()?;
    let class_id = sim.player.class_index as u32;
    let spec_index = sim.player.active_spec_index.max(1) as usize - 1;
    specializations::specs_for_class(class_id)
        .nth(spec_index)
        .map(|spec| spec.id)
}

fn current_spec_set_id(state: &LuaState) -> u32 {
    match current_spec_id(state) {
        Some(65) => 27,
        Some(66) => 28,
        Some(70) => 29,
        _ => 0,
    }
}

fn config_name(config_id: i32) -> &'static str {
    match config_id {
        101 => "Holy Mythic+",
        102 => "Holy Raid",
        201 => "Protection Raid",
        202 => "Protection Mythic+",
        301 => "Retribution Raid",
        302 => "Retribution Mythic+",
        _ => "Default Loadout",
    }
}

fn trait_node_spec_set(cond_ids: &[u32]) -> u32 {
    cond_ids
        .iter()
        .filter_map(|cond_id| TRAIT_COND_DB.get(cond_id))
        .find(|cond| cond.cond_type == 1)
        .map(|cond| cond.spec_set_id)
        .unwrap_or(0)
}

fn trait_node_is_visible(state: &LuaState, node_id: u32) -> bool {
    let Some(node) = TRAIT_NODE_DB.get(&node_id) else {
        return true;
    };
    let required_spec_set = trait_node_spec_set(node.cond_ids);
    required_spec_set == 0 || required_spec_set == current_spec_set_id(state)
}

fn push_u32_array(state: &mut LuaState, values: impl IntoIterator<Item = u32>) -> Val {
    let table = create_table(state);
    for (index, value) in values.into_iter().enumerate() {
        set_table_array(state, table, index as i64 + 1, Val::Num(value as f64));
    }
    table
}

fn push_i32_array(state: &mut LuaState, values: impl IntoIterator<Item = i32>) -> Val {
    let table = create_table(state);
    for (index, value) in values.into_iter().enumerate() {
        set_table_array(state, table, index as i64 + 1, Val::Num(value as f64));
    }
    table
}

fn push_node_info(state: &mut LuaState, node_id: i32) -> Val {
    let info = create_table(state);
    table_set(state, info, "ID", Val::Num(node_id as f64));
    table_set(state, info, "id", Val::Num(node_id as f64));

    let lookup_node_id = u32::try_from(node_id).ok();

    let active_entry = create_table(state);
    let entry_id = borrow_state(state)
        .ok()
        .and_then(|sim| lookup_node_id.and_then(|id| sim.talents.node_selections.get(&id).copied()))
        .unwrap_or(0);
    table_set(state, active_entry, "entryID", Val::Num(entry_id as f64));
    table_set(state, info, "activeEntry", active_entry);

    let ranks_purchased = borrow_state(state)
        .ok()
        .and_then(|sim| lookup_node_id.and_then(|id| sim.talents.node_ranks.get(&id).copied()))
        .unwrap_or(0);
    table_set(
        state,
        info,
        "ranksPurchased",
        Val::Num(ranks_purchased as f64),
    );
    table_set(state, info, "currentRank", Val::Num(ranks_purchased as f64));
    table_set(state, info, "activeRank", Val::Num(ranks_purchased as f64));
    table_set(
        state,
        info,
        "ranksIncreased",
        Val::Num(ranks_purchased as f64),
    );
    let entry_ranks_increased = create_table(state);
    table_set(
        state,
        info,
        "entryIDToRanksIncreased",
        entry_ranks_increased,
    );

    if let Some(node) = lookup_node_id.and_then(|id| TRAIT_NODE_DB.get(&id)) {
        let entry_ids = push_u32_array(state, node.entry_ids.iter().copied());
        table_set(state, info, "entryIDs", entry_ids);
        let total_max_ranks = node
            .entry_ids
            .iter()
            .filter_map(|entry_id| TRAIT_ENTRY_DB.get(entry_id))
            .map(|entry| entry.max_ranks)
            .max()
            .unwrap_or(0);
        table_set(
            state,
            info,
            "totalMaxRanks",
            Val::Num(total_max_ranks as f64),
        );
        table_set(
            state,
            info,
            "isVisible",
            Val::Bool(lookup_node_id.is_some_and(|id| trait_node_is_visible(state, id))),
        );
    } else {
        let entry_ids = create_table(state);
        table_set(state, info, "entryIDs", entry_ids);
        table_set(state, info, "totalMaxRanks", Val::Num(0.0));
        table_set(state, info, "isVisible", Val::Bool(true));
    }

    info
}

fn config_ids_for_spec_id(spec_id: u32) -> Vec<i32> {
    talent_state::seeded_class_talent_configs(spec_id)
        .iter()
        .map(|config| config.id)
        .collect()
}

fn current_config_ids(state: &LuaState) -> Vec<i32> {
    current_spec_id(state)
        .map(config_ids_for_spec_id)
        .unwrap_or_default()
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

fn get_spell_link_global(state: &mut LuaState) -> LuaResult<u32> {
    c_spell_get_spell_link(state)
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

fn register_c_traits(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Traits")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GenerateImportString",
        c_traits_generate_import_string,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetConfigIDBySystemID",
        c_traits_get_config_id_by_system_id,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetConfigIDByTreeID",
        c_traits_get_config_id_by_tree_id,
    )?;
    table_set_rust_fn(state, table_ref, "GetConfigInfo", c_traits_get_config_info)?;
    table_set_rust_fn(state, table_ref, "GetNodeInfo", c_traits_get_node_info)?;
    table_set_rust_fn(state, table_ref, "GetEntryInfo", c_traits_get_entry_info)?;
    table_set_rust_fn(
        state,
        table_ref,
        "InitializeViewLoadout",
        c_traits_initialize_view_loadout,
    )?;
    table_set_rust_fn(state, table_ref, "GetTreeInfo", c_traits_get_tree_info)?;
    table_set_rust_fn(state, table_ref, "GetTreeNodes", c_traits_get_tree_nodes)?;
    table_set_rust_fn(state, table_ref, "GetAllTreeIDs", c_traits_get_all_tree_ids)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetTraitSystemFlags",
        c_traits_get_trait_system_flags,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "CanPurchaseRank",
        c_traits_can_purchase_rank,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetLoadoutSerializationVersion",
        c_traits_get_loadout_serialization_version,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSubTreeInfo",
        c_traits_get_subtree_info,
    )?;
    table_set_rust_fn(state, table_ref, "SetSelection", c_traits_set_selection)?;
    table_set_rust_fn(state, table_ref, "PurchaseRank", c_traits_purchase_rank)?;
    table_set_rust_fn(state, table_ref, "RefundRank", c_traits_refund_rank)?;
    Ok(())
}

fn register_c_class_talents(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_ClassTalents")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetHeroTalentSpecsForClassSpec",
        c_class_talents_get_hero_talent_specs_for_class_spec,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetActiveHeroTalentSpec",
        c_class_talents_get_active_hero_talent_spec,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetConfigIDsBySpecID",
        c_class_talents_get_config_ids_by_spec_id,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetActiveConfigID",
        c_class_talents_get_active_config_id,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetLastSelectedSavedConfigID",
        c_class_talents_get_last_selected_saved_config_id,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "SwitchToLoadoutByName",
        c_class_talents_switch_to_loadout_by_name,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "SwitchToLoadoutByIndex",
        c_class_talents_switch_to_loadout_by_index,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "SwitchToSpecializationByName",
        c_class_talents_switch_to_specialization_by_name,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "SwitchToSpecializationByIndex",
        c_class_talents_switch_to_specialization_by_index,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetTraitTreeForSpec",
        c_class_talents_get_trait_tree_for_spec,
    )?;
    Ok(())
}

fn c_traits_generate_import_string(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let import = create_string(state, &format!("RILUA:PALADIN:{config_id}"));
    state.push(import);
    Ok(1)
}

fn c_traits_get_config_id_by_system_id(state: &mut LuaState) -> LuaResult<u32> {
    let _system_id = i32::from_stack(state, 1)?;
    state.push(Val::Num(1.0));
    Ok(1)
}

fn c_traits_get_config_id_by_tree_id(state: &mut LuaState) -> LuaResult<u32> {
    let _tree_id = i32::from_stack(state, 1)?;
    state.push(Val::Num(1.0));
    Ok(1)
}

fn c_traits_get_config_info(state: &mut LuaState) -> LuaResult<u32> {
    let config_id = i32::from_stack(state, 1)?;
    let info = create_table(state);
    table_set(state, info, "ID", Val::Num(config_id as f64));
    table_set(state, info, "id", Val::Num(config_id as f64));
    let name = create_string(state, config_name(config_id));
    table_set(state, info, "name", name);
    state.push(info);
    Ok(1)
}

fn c_traits_get_node_info(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let node_id = i32::from_stack(state, 2)?;
    let info = push_node_info(state, node_id);
    state.push(info);
    Ok(1)
}

fn c_traits_get_entry_info(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let entry_id = u32::from_stack(state, 2)?;
    let Some(entry) = TRAIT_ENTRY_DB.get(&entry_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    table_set(state, info, "entryID", Val::Num(entry.id as f64));
    table_set(
        state,
        info,
        "definitionID",
        Val::Num(entry.definition_id as f64),
    );
    table_set(state, info, "subTreeID", Val::Num(entry.sub_tree_id as f64));
    state.push(info);
    Ok(1)
}

fn c_traits_initialize_view_loadout(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let _tree_id = i32::from_stack(state, 2)?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_traits_get_tree_info(state: &mut LuaState) -> LuaResult<u32> {
    let tree_id = match stack_val(state, 2) {
        Val::Num(value) => value as u32,
        _ => u32::from_stack(state, 1)?,
    };
    let Some(tree) = TRAIT_TREE_DB.get(&tree_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    table_set(state, info, "ID", Val::Num(tree.id as f64));
    let currency_ids = push_u32_array(state, tree.currency_ids.iter().copied());
    table_set(state, info, "currencyIDs", currency_ids);
    state.push(info);
    Ok(1)
}

fn c_traits_get_tree_nodes(state: &mut LuaState) -> LuaResult<u32> {
    let tree_id = match stack_val(state, 2) {
        Val::Num(value) => value as u32,
        _ => match stack_val(state, 1) {
            Val::Num(value) => value as u32,
            _ => 0,
        },
    };
    let nodes = TRAIT_TREE_DB
        .get(&tree_id)
        .map(|tree| push_u32_array(state, tree.node_ids.iter().copied()))
        .unwrap_or_else(|| create_table(state));
    state.push(nodes);
    Ok(1)
}

fn c_traits_get_all_tree_ids(state: &mut LuaState) -> LuaResult<u32> {
    let tree_ids = push_u32_array(state, [1, 790, 994]);
    state.push(tree_ids);
    Ok(1)
}

fn c_traits_get_trait_system_flags(state: &mut LuaState) -> LuaResult<u32> {
    let _system_id = i32::from_stack(state, 1)?;
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_traits_can_purchase_rank(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let _node_id = u32::from_stack(state, 2)?;
    let _entry_id = u32::from_stack(state, 3)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_traits_get_loadout_serialization_version(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(2.0));
    Ok(1)
}

fn c_traits_get_subtree_info(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let subtree_id = u32::from_stack(state, 2)?;
    let Some(subtree) = TRAIT_SUBTREE_DB.get(&subtree_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = create_table(state);
    table_set(state, info, "ID", Val::Num(subtree.id as f64));
    table_set(state, info, "id", Val::Num(subtree.id as f64));
    let name = create_string(state, subtree.name);
    table_set(state, info, "name", name);
    let description = create_string(state, subtree.description);
    table_set(state, info, "description", description);
    table_set(
        state,
        info,
        "iconElementID",
        Val::Num(subtree.atlas_element_id as f64),
    );
    let selection_node_ids = push_u32_array(
        state,
        hero_talents::selection_node_ids_for_subtree(subtree_id)
            .into_iter()
            .map(|node_id| node_id as u32),
    );
    table_set(state, info, "subTreeSelectionNodeIDs", selection_node_ids);
    let (pos_x, pos_y) = hero_talents::subtree_position(subtree_id);
    table_set(state, info, "posX", Val::Num(pos_x as f64));
    table_set(state, info, "posY", Val::Num(pos_y as f64));
    state.push(info);
    Ok(1)
}

fn c_traits_set_selection(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let node_id = u32::from_stack(state, 2)?;
    let entry_id = match stack_val(state, 3) {
        Val::Nil => None,
        Val::Num(value) => Some(value as u32),
        _ => None,
    };
    {
        let mut sim = borrow_state_mut(state)?;
        sim.talents.set_node_selection(node_id, entry_id);
        sim.talents
            .set_node_rank(node_id, u32::from(entry_id.is_some()));
    }
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_traits_purchase_rank(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let node_id = u32::from_stack(state, 2)?;
    {
        let mut sim = borrow_state_mut(state)?;
        let next_rank = sim.talents.node_ranks.get(&node_id).copied().unwrap_or(0) + 1;
        sim.talents.set_node_rank(node_id, next_rank);
    }
    fire_named_event_with_arg(state, "TRAIT_NODE_CHANGED", Val::Num(node_id as f64));
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_traits_refund_rank(state: &mut LuaState) -> LuaResult<u32> {
    let _config_id = i32::from_stack(state, 1)?;
    let node_id = u32::from_stack(state, 2)?;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.talents.set_node_rank(node_id, 0);
    }
    fire_named_event_with_arg(state, "TRAIT_NODE_CHANGED", Val::Num(node_id as f64));
    state.push(Val::Bool(true));
    Ok(1)
}

fn hero_specs_for_spec(spec_id: u32) -> &'static [u32] {
    match spec_id {
        65 => &[49, 50],
        66 => &[48, 49],
        70 => &[48, 50],
        _ => &[],
    }
}

fn c_class_talents_get_hero_talent_specs_for_class_spec(state: &mut LuaState) -> LuaResult<u32> {
    let _class_id = i32::from_stack(state, 1)?;
    let spec_id = u32::from_stack(state, 2)?;
    let hero_specs = push_u32_array(state, hero_specs_for_spec(spec_id).iter().copied());
    state.push(hero_specs);
    state.push(Val::Num(71.0));
    Ok(2)
}

fn c_class_talents_get_active_hero_talent_spec(state: &mut LuaState) -> LuaResult<u32> {
    match borrow_state(state)
        .ok()
        .and_then(|sim| hero_talents::get_active_hero_subtree(&sim))
    {
        Some(subtree_id) => state.push(Val::Num(subtree_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_class_talents_get_config_ids_by_spec_id(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = u32::from_stack(state, 1)?;
    let config_ids = push_i32_array(state, config_ids_for_spec_id(spec_id));
    state.push(config_ids);
    Ok(1)
}

fn c_class_talents_get_active_config_id(state: &mut LuaState) -> LuaResult<u32> {
    let active_config_id = borrow_state(state)?.talents.active_config_id as f64;
    state.push(Val::Num(active_config_id));
    Ok(1)
}

fn c_class_talents_get_last_selected_saved_config_id(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = u32::from_stack(state, 1)?;
    let config_id = borrow_state(state)?
        .talents
        .last_selected_config_id_by_spec_id
        .get(&spec_id)
        .copied()
        .or_else(|| talent_state::default_class_talent_config_id(spec_id))
        .unwrap_or(0);
    state.push(Val::Num(config_id as f64));
    Ok(1)
}

fn c_class_talents_switch_to_loadout_by_name(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let config_id = current_config_ids(state)
        .into_iter()
        .find(|config_id| config_name(*config_id) == name)
        .unwrap_or_else(|| {
            borrow_state(state)
                .map(|sim| sim.talents.active_config_id)
                .unwrap_or(0)
        });
    if let Some(spec_id) = current_spec_id(state) {
        borrow_state_mut(state)?
            .talents
            .switch_to_loadout(spec_id, config_id);
    }
    Ok(0)
}

fn c_class_talents_switch_to_loadout_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?.max(1) as usize - 1;
    let configs = current_config_ids(state);
    if let Some(config_id) = configs.get(index).copied()
        && let Some(spec_id) = current_spec_id(state)
    {
        borrow_state_mut(state)?
            .talents
            .switch_to_loadout(spec_id, config_id);
    }
    Ok(0)
}

fn c_class_talents_switch_to_specialization_by_name(state: &mut LuaState) -> LuaResult<u32> {
    let spec_name = String::from_stack(state, 1)?;
    let class_id = borrow_state(state)?.player.class_index as u32;
    let Some((index, spec)) = specializations::specs_for_class(class_id)
        .enumerate()
        .find(|(_, spec)| spec.name == spec_name)
    else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    sim.player.active_spec_index = index as i32 + 1;
    sim.talents.switch_to_spec(spec.id);
    Ok(0)
}

fn c_class_talents_switch_to_specialization_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let spec_index = i32::from_stack(state, 1)?.max(1) as usize - 1;
    let class_id = borrow_state(state)?.player.class_index as u32;
    let Some(spec) = specializations::specs_for_class(class_id).nth(spec_index) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    sim.player.active_spec_index = spec_index as i32 + 1;
    sim.talents.switch_to_spec(spec.id);
    Ok(0)
}

fn c_class_talents_get_trait_tree_for_spec(state: &mut LuaState) -> LuaResult<u32> {
    let _spec_id = u32::from_stack(state, 1)?;
    state.push(Val::Num(790.0));
    Ok(1)
}

fn register_c_tooltip_info(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_TooltipInfo")?;
    table_set_rust_fn(state, table_ref, "GetTraitEntry", c_tooltip_get_trait_entry)?;
    table_set_rust_fn(state, table_ref, "GetAction", c_tooltip_get_action)?;
    table_set_rust_fn(state, table_ref, "GetItemByID", c_tooltip_get_item_by_id)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetItemByGUID",
        c_tooltip_get_item_by_guid,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetOwnedItemByID",
        c_tooltip_get_owned_item_by_id,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetRecipeResultItem",
        c_tooltip_get_recipe_result_item,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetRecipeResultItemForOrder",
        c_tooltip_get_recipe_result_item_for_order,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetMinimapMouseover",
        c_tooltip_get_minimap_mouseover,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetUpgradeItem",
        c_tooltip_get_upgrade_item,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetInventoryItem",
        c_tooltip_get_inventory_item,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetSpellBookItem",
        c_tooltip_get_spell_book_item,
    )?;
    table_set_rust_fn(state, table_ref, "GetSpellByID", c_tooltip_get_spell_by_id)?;
    table_set_rust_fn(state, table_ref, "GetUnitBuff", c_tooltip_get_unit_buff)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetUnitBuffByAuraInstanceID",
        c_tooltip_get_unit_buff_by_aura_instance_id,
    )?;
    table_set_rust_fn(state, table_ref, "GetUnitDebuff", c_tooltip_get_unit_debuff)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetUnitDebuffByAuraInstanceID",
        c_tooltip_get_unit_debuff_by_aura_instance_id,
    )?;
    table_set_rust_fn(state, table_ref, "GetUnitAura", c_tooltip_get_unit_aura)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetUnitAuraByAuraInstanceID",
        c_tooltip_get_unit_aura_by_aura_instance_id,
    )?;
    table_set_rust_fn(state, table_ref, "GetHyperlink", c_tooltip_get_hyperlink)?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetWorldCursor",
        c_tooltip_get_world_cursor,
    )?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetWorldLootObject",
        c_tooltip_get_world_loot_object,
    )?;
    table_set_rust_fn(state, table_ref, "GetUnit", c_tooltip_get_unit)?;
    Ok(())
}

fn c_tooltip_get_trait_entry(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip = tooltip_for_spell_id(state, 19750);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_action(state: &mut LuaState) -> LuaResult<u32> {
    let slot = u32::from_stack(state, 1)?;
    let spell_id = borrow_state(state)?.action_bars.get(&slot).copied();
    match spell_id {
        Some(spell_id) => {
            let tooltip = tooltip_for_spell_id(state, spell_id);
            state.push(tooltip);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_tooltip_get_item_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    let tooltip = tooltip_for_item_id(state, item_id);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_item_by_guid(state: &mut LuaState) -> LuaResult<u32> {
    let guid = String::from_stack(state, 1)?;
    let tooltip = if let Some((_, _, item_id)) = parse_item_guid(&guid) {
        tooltip_for_item_id(state, item_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    let guid = create_string(state, &guid);
    table_set(state, tooltip, "guid", guid);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_owned_item_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    let owned = borrow_state(state)?
        .bag_items
        .values()
        .any(|item| item.item_id == item_id);
    let tooltip = if owned {
        tooltip_for_item_id(state, item_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    state.push(tooltip);
    Ok(1)
}

fn recipe_output_item(recipe_id: i32) -> Option<u32> {
    match recipe_id {
        100005 => Some(229181),
        _ => None,
    }
}

fn c_tooltip_get_recipe_result_item(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let tooltip = if let Some(item_id) = recipe_output_item(recipe_id) {
        tooltip_for_item_id(state, item_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_recipe_result_item_for_order(state: &mut LuaState) -> LuaResult<u32> {
    let recipe_id = i32::from_stack(state, 1)?;
    let tooltip = if let Some(item_id) = recipe_output_item(recipe_id) {
        tooltip_for_item_id(state, item_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_minimap_mouseover(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_MINIMAP_MOUSEOVER);
    let lines = table_get(state, tooltip, "lines");
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_SPELL_NAME,
        "Stormwind City",
        None,
        false,
    );
    push_tooltip_line(
        state,
        lines,
        2,
        LINE_TYPE_SPELL_NAME,
        "Trade District",
        None,
        false,
    );
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_upgrade_item(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = current_item_upgrade_location(state).and_then(|(bag, slot)| {
        borrow_state(state)
            .ok()?
            .get_bag_item(bag, slot)
            .map(|(item_id, _)| item_id)
    });
    let tooltip = if let Some(item_id) = item_id {
        tooltip_for_item_id(state, item_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_inventory_item(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let slot = i32::from_stack(state, 2)?;
    let item_id = borrow_state(state)?
        .player
        .equipped_items
        .get(&slot)
        .map(|item| item.item_id)
        .unwrap_or(0);
    let tooltip = tooltip_for_item_id(state, item_id);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_spell_book_item(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1)?;
    match spellbook_data::get_spell_at_slot(slot) {
        Some((_, entry, _)) => {
            let tooltip = tooltip_for_spell_id(state, entry.spell_id);
            state.push(tooltip);
        }
        None => {
            let tooltip = empty_tooltip(state, TOOLTIP_TYPE_SPELL);
            state.push(tooltip);
        }
    }
    Ok(1)
}

fn c_tooltip_get_spell_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    let tooltip = tooltip_for_spell_id(state, spell_id);
    state.push(tooltip);
    Ok(1)
}

fn lookup_player_aura(state: &LuaState, index: i32) -> Option<crate::lua_api::game_data::AuraInfo> {
    let sim = borrow_state(state).ok()?;
    sim.player.buffs.get((index - 1).max(0) as usize).cloned()
}

fn lookup_player_aura_by_instance_id(
    state: &LuaState,
    aura_instance_id: i32,
) -> Option<crate::lua_api::game_data::AuraInfo> {
    let sim = borrow_state(state).ok()?;
    sim.player
        .buffs
        .iter()
        .find(|aura| aura.aura_instance_id == aura_instance_id)
        .cloned()
}

fn c_tooltip_get_unit_buff(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    let aura = lookup_player_aura(state, index);
    let tooltip = tooltip_for_unit_aura(state, aura);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit_buff_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let aura_instance_id = i32::from_stack(state, 2)?;
    let aura = lookup_player_aura_by_instance_id(state, aura_instance_id);
    let tooltip = tooltip_for_unit_aura(state, aura);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit_debuff(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let _index = i32::from_stack(state, 2)?;
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit_debuff_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let _aura_instance_id = i32::from_stack(state, 2)?;
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit_aura(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    let filter = String::from_stack(state, 3).unwrap_or_default();
    let tooltip = if filter.eq_ignore_ascii_case("HARMFUL") {
        empty_tooltip(state, TOOLTIP_TYPE_UNIT_AURA)
    } else {
        let aura = lookup_player_aura(state, index);
        tooltip_for_unit_aura(state, aura)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit_aura_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let aura_instance_id = i32::from_stack(state, 2)?;
    let aura = lookup_player_aura_by_instance_id(state, aura_instance_id);
    let tooltip = tooltip_for_unit_aura(state, aura);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_hyperlink(state: &mut LuaState) -> LuaResult<u32> {
    let link = String::from_stack(state, 1)?;
    let tooltip = if let Some(item_id) = parse_prefixed_id(&link, "item") {
        tooltip_for_item_id(state, item_id)
    } else if let Some(spell_id) = parse_prefixed_id(&link, "spell") {
        tooltip_for_spell_id(state, spell_id)
    } else {
        empty_tooltip(state, TOOLTIP_TYPE_ITEM)
    };
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_world_cursor(state: &mut LuaState) -> LuaResult<u32> {
    let tooltip = tooltip_for_world_loot(state);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_world_loot_object(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = String::from_stack(state, 1)?;
    let tooltip = tooltip_for_world_loot(state);
    state.push(tooltip);
    Ok(1)
}

fn c_tooltip_get_unit(state: &mut LuaState) -> LuaResult<u32> {
    let unit = String::from_stack(state, 1)?;
    let tooltip = tooltip_for_unit(state, &unit);
    state.push(tooltip);
    Ok(1)
}

fn set_action_ui_button(state: &mut LuaState) -> LuaResult<u32> {
    let button = stack_val(state, 1);
    let action = u32::from_stack(state, 2)?;
    let Some(button_id) = crate::lua_api::rilua_methods::extract_frame_id(state, button) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    sim.action_ui_buttons.retain(|(id, _)| *id != button_id);
    sim.action_ui_buttons.push((button_id, action));
    Ok(0)
}
