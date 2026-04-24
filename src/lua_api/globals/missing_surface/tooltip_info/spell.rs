use super::super::{
    LINE_TYPE_ITEM_NAME, LINE_TYPE_SPELL_DESCRIPTION, LINE_TYPE_SPELL_NAME, TOOLTIP_TYPE_ITEM,
    TOOLTIP_TYPE_SPELL, TOOLTIP_TYPE_UNIT_AURA,
};
use super::builders::{empty_tooltip, item_quality_color, push_tooltip_line, tooltip_for_item_id};
use crate::lua_api::game_data;
use crate::lua_api::globals::spell_api;
use crate::lua_api::methods::{borrow_state, table_get, table_set};
use crate::spells;
use crate::traits::{TRAIT_DEFINITION_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB};
use rilua::Val;
use rilua::vm::state::LuaState;

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

fn push_spell_name_line(state: &mut LuaState, lines: Val, index: i64, spell_name: &str) {
    push_tooltip_line(
        state,
        lines,
        index,
        LINE_TYPE_SPELL_NAME,
        spell_name,
        None,
        false,
    );
}

/// Returns true when a cost line was written, so the caller can advance
/// the running index.
fn push_spell_cost_line(state: &mut LuaState, lines: Val, index: i64, spell_id: u32) -> bool {
    let Some(cost) = spell_cost_line(spell_id) else {
        return false;
    };
    push_tooltip_line(state, lines, index, LINE_TYPE_SPELL_NAME, cost, None, false);
    true
}

fn push_spell_cast_line(state: &mut LuaState, lines: Val, index: i64, spell_id: u32) {
    let cast_line = spell_cast_line(spell_id);
    push_tooltip_line(
        state,
        lines,
        index,
        LINE_TYPE_SPELL_NAME,
        &cast_line,
        None,
        false,
    );
}

fn push_spell_description_line(state: &mut LuaState, lines: Val, index: i64, spell_id: u32) {
    let description = resolved_spell_description(state, spell_id);
    push_tooltip_line(
        state,
        lines,
        index,
        LINE_TYPE_SPELL_DESCRIPTION,
        &description,
        None,
        true,
    );
}

fn resolved_spell_description(state: &LuaState, spell_id: u32) -> String {
    match borrow_state(state) {
        Ok(sim) => crate::spell_description_resolver::resolve_spell_description(&sim, spell_id),
        Err(_) => "No description available.".to_string(),
    }
}

fn push_spell_tooltip_lines(state: &mut LuaState, lines: Val, spell_id: u32, spell_name: &str) {
    let mut index = 1;
    push_spell_name_line(state, lines, index, spell_name);
    index += 1;
    if push_spell_cost_line(state, lines, index, spell_id) {
        index += 1;
    }
    push_spell_cast_line(state, lines, index, spell_id);
    index += 1;
    push_spell_description_line(state, lines, index, spell_id);
}

pub(super) fn tooltip_for_spell_id(state: &mut LuaState, spell_id: u32) -> Val {
    let Some(spell) = spells::get_spell(spell_id) else {
        return empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    };
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    let lines = table_get(state, tooltip, "lines");
    push_spell_tooltip_lines(state, lines, spell_id, spell.name);
    table_set(state, tooltip, "id", Val::Num(spell_id as f64));
    tooltip
}

pub(super) fn tooltip_for_unit_aura(
    state: &mut LuaState,
    aura: Option<game_data::AuraInfo>,
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
    let description = resolved_spell_description(state, aura.spell_id as u32);
    push_tooltip_line(
        state,
        lines,
        3,
        LINE_TYPE_SPELL_DESCRIPTION,
        &description,
        None,
        true,
    );
    tooltip
}

pub(super) fn tooltip_for_toy_item_id(state: &mut LuaState, item_id: u32) -> Val {
    if crate::items::get_item(item_id).is_some() {
        return tooltip_for_item_id(state, item_id);
    }

    let toy_name = borrow_state(state).ok().and_then(|st| {
        st.world
            .toys
            .iter()
            .find(|toy| toy.item_id == item_id)
            .map(|toy| toy.name.clone())
    });
    let Some(toy_name) = toy_name else {
        return empty_tooltip(state, TOOLTIP_TYPE_ITEM);
    };

    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_ITEM);
    let lines = table_get(state, tooltip, "lines");
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_ITEM_NAME,
        &toy_name,
        Some(item_quality_color(1)),
        false,
    );
    tooltip
}

pub(super) fn tooltip_for_mount_spell_id(state: &mut LuaState, spell_id: u32) -> Val {
    if spells::get_spell(spell_id).is_some() {
        return tooltip_for_spell_id(state, spell_id);
    }
    let Some(mount_name) = find_mount_name_for_spell(state, spell_id) else {
        return empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    };
    build_mount_tooltip(state, spell_id, &mount_name)
}

fn find_mount_name_for_spell(state: &mut LuaState, spell_id: u32) -> Option<String> {
    borrow_state(state).ok().and_then(|st| {
        st.world
            .mounts
            .iter()
            .find(|mount| mount.spell_id == spell_id)
            .map(|mount| mount.name.clone())
    })
}

fn build_mount_tooltip(state: &mut LuaState, spell_id: u32, mount_name: &str) -> Val {
    let tooltip = empty_tooltip(state, TOOLTIP_TYPE_SPELL);
    let lines = table_get(state, tooltip, "lines");
    push_tooltip_line(
        state,
        lines,
        1,
        LINE_TYPE_SPELL_NAME,
        mount_name,
        None,
        false,
    );
    push_tooltip_line(
        state,
        lines,
        3,
        LINE_TYPE_SPELL_DESCRIPTION,
        "Summons this mount.",
        None,
        true,
    );
    table_set(state, tooltip, "id", Val::Num(spell_id as f64));
    tooltip
}

fn preferred_trait_spell_id(definition_id: u32) -> Option<u32> {
    let definition = TRAIT_DEFINITION_DB.get(&definition_id)?;
    [
        definition.visible_spell_id,
        definition.overrides_spell_id,
        definition.spell_id,
    ]
    .into_iter()
    .find(|spell_id| *spell_id != 0)
}

fn spell_id_for_trait_entry(entry_id: u32) -> Option<u32> {
    let mut current_id = entry_id;
    for _ in 0..8 {
        if let Some(spell_id) = preferred_trait_spell_id(current_id) {
            return Some(spell_id);
        }
        current_id = TRAIT_ENTRY_DB.get(&current_id)?.definition_id;
    }
    None
}

pub(super) fn spell_id_for_talent_id(state: &LuaState, talent_id: u32) -> Option<u32> {
    if let Some(node) = TRAIT_NODE_DB.get(&talent_id) {
        let selected_entry_id = borrow_state(state)
            .ok()
            .and_then(|sim| sim.talents.node_selections.get(&talent_id).copied());
        let entry_id = selected_entry_id.or_else(|| node.entry_ids.first().copied())?;
        return spell_id_for_trait_entry(entry_id);
    }

    spell_id_for_trait_entry(talent_id).or_else(|| preferred_trait_spell_id(talent_id))
}

pub(super) fn lookup_player_aura(state: &LuaState, index: i32) -> Option<game_data::AuraInfo> {
    let sim = borrow_state(state).ok()?;
    sim.player.buffs.get((index - 1).max(0) as usize).cloned()
}

pub(super) fn lookup_player_aura_by_instance_id(
    state: &LuaState,
    aura_instance_id: i32,
) -> Option<game_data::AuraInfo> {
    let sim = borrow_state(state).ok()?;
    sim.player
        .buffs
        .iter()
        .find(|aura| aura.aura_instance_id == aura_instance_id)
        .cloned()
}
