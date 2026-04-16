use super::{
    LINE_TYPE_EQUIP_SLOT, LINE_TYPE_ITEM_BINDING, LINE_TYPE_ITEM_LEVEL, LINE_TYPE_ITEM_NAME,
    LINE_TYPE_SPELL_DESCRIPTION, LINE_TYPE_SPELL_NAME, LINE_TYPE_UNIT_NAME, TOOLTIP_TYPE_ITEM,
    TOOLTIP_TYPE_MINIMAP_MOUSEOVER, TOOLTIP_TYPE_SPELL, TOOLTIP_TYPE_UNIT, TOOLTIP_TYPE_UNIT_AURA,
    WORLD_CURSOR_GUID, WORLD_LOOT_TOOLTIP_INVENTORY_TYPE, WORLD_LOOT_TOOLTIP_SPELL_ID,
    ensure_namespace, set_table_array,
};
use crate::items;
use crate::lua_api::game_data::CLASS_LABELS;
use crate::lua_api::globals::{spell_api, spellbook_data};
use crate::lua_api::rilua_methods::{
    borrow_state, call_function_state, create_string, create_table, table_get, table_set,
    val_to_string,
};
use crate::lua_api::state::RACE_DATA;
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use crate::spell_descriptions;
use crate::spells;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

use super::item_spell::{current_item_upgrade_location, parse_item_guid, parse_prefixed_id};

pub(super) fn register_tooltip_surface(state: &mut LuaState) -> LuaResult<()> {
    register_c_tooltip_info(state)
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

fn push_spell_tooltip_lines(state: &mut LuaState, lines: Val, spell_id: u32, spell_name: &str) {
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_SPELL_NAME,
        spell_name,
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
}

fn tooltip_for_spell_id(state: &mut LuaState, spell_id: u32) -> Val {
    let Some(spell) = spells::get_spell(spell_id) else {
        return empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    };
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    let lines = table_get(state, tooltip, "lines");
    push_spell_tooltip_lines(state, lines, spell_id, spell.name);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_api::env::WowLuaEnv;
    use rilua::{Lua, LuaApiMut};

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

fn register_item_spell_aura_methods(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
) -> LuaResult<()> {
    register_tooltip_methods(
        state,
        table_ref,
        &[
            ("GetTraitEntry", c_tooltip_get_trait_entry),
            ("GetAction", c_tooltip_get_action),
            ("GetItemByID", c_tooltip_get_item_by_id),
            ("GetItemByGUID", c_tooltip_get_item_by_guid),
            ("GetOwnedItemByID", c_tooltip_get_owned_item_by_id),
            ("GetRecipeResultItem", c_tooltip_get_recipe_result_item),
            (
                "GetRecipeResultItemForOrder",
                c_tooltip_get_recipe_result_item_for_order,
            ),
            ("GetMinimapMouseover", c_tooltip_get_minimap_mouseover),
            ("GetUpgradeItem", c_tooltip_get_upgrade_item),
            ("GetInventoryItem", c_tooltip_get_inventory_item),
        ],
    )
}

fn register_spell_aura_unit_methods(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
) -> LuaResult<()> {
    register_tooltip_methods(
        state,
        table_ref,
        &[
            ("GetSpellBookItem", c_tooltip_get_spell_book_item),
            ("GetSpellByID", c_tooltip_get_spell_by_id),
            ("GetUnitBuff", c_tooltip_get_unit_buff),
            (
                "GetUnitBuffByAuraInstanceID",
                c_tooltip_get_unit_buff_by_aura_instance_id,
            ),
            ("GetUnitDebuff", c_tooltip_get_unit_debuff),
            (
                "GetUnitDebuffByAuraInstanceID",
                c_tooltip_get_unit_debuff_by_aura_instance_id,
            ),
            ("GetUnitAura", c_tooltip_get_unit_aura),
            (
                "GetUnitAuraByAuraInstanceID",
                c_tooltip_get_unit_aura_by_aura_instance_id,
            ),
            ("GetHyperlink", c_tooltip_get_hyperlink),
            ("GetWorldCursor", c_tooltip_get_world_cursor),
            ("GetWorldLootObject", c_tooltip_get_world_loot_object),
            ("GetUnit", c_tooltip_get_unit),
        ],
    )
}

fn register_c_tooltip_info(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_TooltipInfo")?;
    register_item_spell_aura_methods(state, table_ref)?;
    register_spell_aura_unit_methods(state, table_ref)?;
    Ok(())
}

type TooltipScriptFn = fn(&mut LuaState) -> LuaResult<u32>;

fn register_tooltip_methods(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
    entries: &[(&str, TooltipScriptFn)],
) -> LuaResult<()> {
    for &(name, func) in entries {
        table_set_rust_fn(state, table_ref, name, func)?;
    }
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
