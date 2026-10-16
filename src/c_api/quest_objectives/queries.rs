//! Public C_QuestLog queries and legacy wrappers over the same snapshots.

use super::{ENABLED, objectives_for_quest};
use crate::c_api::helpers::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    if !ENABLED {
        return Ok(());
    }
    let namespace = ensure_namespace(state, "C_QuestLog")?;
    table_set_rust_fn_static(state, namespace, "GetQuestObjectives", get_objectives)
}

fn get_objectives(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let objectives = borrow_state(state)?
        .quest_log_entries
        .objective_progress
        .get(&quest_id)
        .filter(|progress| progress.declared_objective_count.is_none())
        .map(|progress| progress.objectives.clone());
    let Some(objectives) = objectives else {
        return Ok(0);
    };
    let array = create_table(state);
    // Root parent before allocating its children.
    state.push(array);
    for (index, objective) in objectives.iter().enumerate() {
        let row = create_table(state);
        state.push(row);
        write_objective(state, row, objective);
        set_table_array(state, array, index as i64 + 1, row);
        state.top -= 1;
    }
    Ok(1)
}

fn write_objective(state: &mut LuaState, row: Val, objective: &super::QuestObjective) {
    let text = create_string(state, &objective.text);
    table_set(state, row, "text", text);
    let kind = create_string(state, &objective.objective_type);
    table_set(state, row, "type", kind);
    table_set(state, row, "finished", Val::Bool(objective.finished));
    table_set(
        state,
        row,
        "numFulfilled",
        Val::Num(f64::from(objective.num_fulfilled)),
    );
    table_set(
        state,
        row,
        "numRequired",
        Val::Num(f64::from(objective.num_required)),
    );
}

fn read_legacy_quest_id(state: &LuaState, index_arg: i32) -> LuaResult<Option<i32>> {
    let index = Option::<f64>::from_stack(state, index_arg)?.map(|value| value as i32);
    let sim = borrow_state(state)?;
    Ok(match index {
        Some(index) if index >= 2 => sim
            .quest_log_entries
            .entries
            .get((index - 2) as usize)
            .map(|entry| entry.quest_id),
        Some(_) => None,
        None => sim
            .selected_quest_log_id
            .and_then(|id| i32::try_from(id).ok()),
    })
}

pub(crate) fn get_legacy_objective_count(state: &mut LuaState) -> LuaResult<u32> {
    let id = read_legacy_quest_id(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        id.map(|id| objectives_for_quest(&sim.quest_log_entries, id).len())
            .unwrap_or(0)
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

pub(crate) fn get_legacy_leaderboard(state: &mut LuaState) -> LuaResult<u32> {
    let objective_index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let Some(quest_id) = read_legacy_quest_id(state, 2)? else {
        return Ok(0);
    };
    let objective = {
        let sim = borrow_state(state)?;
        let rows = objectives_for_quest(&sim.quest_log_entries, quest_id);
        objective_index
            .checked_sub(1)
            .and_then(|index| usize::try_from(index).ok())
            .and_then(|index| rows.get(index))
            .cloned()
    };
    let Some(objective) = objective else {
        return Ok(0);
    };
    let text = create_string(state, &objective.text);
    state.push(text);
    let kind = create_string(state, &objective.objective_type);
    state.push(kind);
    state.push(Val::Bool(objective.finished));
    Ok(3)
}

pub(crate) fn get_percentage(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = i32::from_stack(state, 1)?;
    let percentage = borrow_state(state)?
        .quest_log_entries
        .objective_progress
        .get(&quest_id)
        .and_then(|progress| progress.percentage)
        .unwrap_or(0.0);
    state.push(Val::Num(percentage));
    Ok(1)
}

pub(crate) fn read_selected_objective_text(state: &LuaState) -> LuaResult<String> {
    let sim = borrow_state(state)?;
    let quest_id = sim
        .selected_quest_log_id
        .and_then(|id| i32::try_from(id).ok())
        .unwrap_or(0);
    Ok(join_objective_text(&sim.quest_log_entries, quest_id))
}

pub(crate) fn get_legacy_quest_text(state: &mut LuaState) -> LuaResult<u32> {
    let quest_id = read_legacy_quest_id(state, 1)?.unwrap_or(0);
    let objectives = {
        let sim = borrow_state(state)?;
        join_objective_text(&sim.quest_log_entries, quest_id)
    };
    let description = create_string(state, find_catalog_description(quest_id));
    state.push(description);
    let objectives = create_string(state, &objectives);
    state.push(objectives);
    Ok(2)
}

fn find_catalog_description(quest_id: i32) -> &'static str {
    use crate::lua_api::globals::quest_surface::data::{QUEST_LOG, QuestLogEntry};
    QUEST_LOG
        .iter()
        .find_map(|entry| match entry {
            QuestLogEntry::Quest {
                quest_id: id,
                description,
                ..
            } if *id == quest_id => Some(*description),
            _ => None,
        })
        .unwrap_or("")
}

fn join_objective_text(
    log: &crate::lua_api::sim_substates::QuestLogState,
    quest_id: i32,
) -> String {
    objectives_for_quest(log, quest_id)
        .iter()
        .map(|objective| objective.text.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}
