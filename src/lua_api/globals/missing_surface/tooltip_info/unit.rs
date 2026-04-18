use super::builders::{empty_tooltip, push_plain_line, push_tooltip_line};
use super::spell::tooltip_for_spell_id;
use super::super::{
    LINE_TYPE_UNIT_NAME, TOOLTIP_TYPE_UNIT, WORLD_CURSOR_GUID, WORLD_LOOT_TOOLTIP_INVENTORY_TYPE,
    WORLD_LOOT_TOOLTIP_SPELL_ID,
};
use crate::lua_api::game_data::CLASS_LABELS;
use crate::lua_api::methods::{borrow_state, create_string, table_get, table_set};
use crate::lua_api::state::RACE_DATA;
use rilua::vm::state::LuaState;
use rilua::Val;

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

pub(super) struct UnitTooltipInfo {
    pub(super) name: String,
    pub(super) level: i32,
    pub(super) race: String,
    pub(super) class_name: String,
    pub(super) color: (f64, f64, f64),
}

fn class_label(class_index: i32) -> String {
    CLASS_LABELS
        .get((class_index - 1).max(0) as usize)
        .copied()
        .unwrap_or("Unknown")
        .to_string()
}

pub(super) fn unit_tooltip_info(state: &LuaState, unit: &str) -> Option<UnitTooltipInfo> {
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
    push_plain_line(state, lines, 2, &level_text);
    push_plain_line(state, lines, 3, &info.race);
    push_plain_line(state, lines, 4, &info.class_name);
}

pub(super) fn tooltip_for_unit(state: &mut LuaState, unit: &str) -> Val {
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_UNIT);
    let lines = table_get(state, tooltip, "lines");
    if let Some(info) = unit_tooltip_info(state, unit) {
        push_unit_tooltip_lines(state, lines, &info);
    }
    tooltip
}

pub(super) fn tooltip_for_world_loot(state: &mut LuaState) -> Val {
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
