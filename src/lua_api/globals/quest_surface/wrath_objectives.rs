//! Wrath 12340 legacy objective reads over supplied snapshots, not catalog rows.

use std::ffi::{CStr, CString};

use crate::c_api::quest_objectives::{objectives_for_quest, read_selected_objective_text};
use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_api::state::SimState;
use crate::lua_bridge::stack_val;
use rilua::{LuaError, LuaResult, RuntimeError, Val, vm::state::LuaState};

fn read_numeric_argument(state: &LuaState, index: i32) -> Option<f64> {
    match stack_val(state, index) {
        Val::Num(number) => Some(number),
        Val::Str(key) => parse_numeric_string(state.gc.string_arena.get(key)?.data()),
        _ => None,
    }
}

fn parse_numeric_string(bytes: &[u8]) -> Option<f64> {
    let text = CString::new(bytes).ok()?;
    let mut end = std::ptr::null_mut();
    // SAFETY: text is NUL-terminated and end receives a pointer into that text.
    let number = unsafe { libc::strtod(text.as_ptr(), &mut end) };
    if end == text.as_ptr().cast_mut() {
        return None;
    }
    // SAFETY: strtod's end pointer remains within the live CString allocation.
    let remainder = unsafe { CStr::from_ptr(end) }.to_bytes();
    remainder
        .iter()
        .all(u8::is_ascii_whitespace)
        .then_some(number)
}

fn indexed_quest_id(sim: &SimState, index: f64) -> Option<i32> {
    let position = (index as i32).checked_sub(1)?;
    let position = usize::try_from(position).ok()?;
    sim.quest_log_entries
        .entries
        .get(position)
        .map(|entry| entry.quest_id)
}

fn selected_active_quest_id(sim: &SimState) -> Option<i32> {
    let selected = sim.selected_quest_log_id?;
    sim.quest_log_entries
        .entries
        .iter()
        .find(|entry| entry.quest_id as u32 == selected)
        .map(|entry| entry.quest_id)
}

pub(super) fn get_num_quest_leaderboards(state: &mut LuaState) -> LuaResult<u32> {
    let explicit = read_numeric_argument(state, 1);
    let count = {
        let sim = borrow_state(state)?;
        let id = match explicit {
            Some(index) => indexed_quest_id(&sim, index),
            None => selected_active_quest_id(&sim),
        };
        id.map_or(0, |id| {
            objectives_for_quest(&sim.quest_log_entries, id).len()
        })
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

pub(super) fn get_quest_log_leaderboard(state: &mut LuaState) -> LuaResult<u32> {
    let index = read_numeric_argument(state, 1).ok_or_else(|| {
        LuaError::Runtime(RuntimeError {
            message: "Usage: GetQuestLogLeaderBoard(index)".into(),
            level: 1,
            traceback: vec![],
        })
    })?;
    let explicit = read_numeric_argument(state, 2);
    let objective = {
        let sim = borrow_state(state)?;
        let id = explicit
            .and_then(|index| indexed_quest_id(&sim, index))
            .or_else(|| selected_active_quest_id(&sim));
        id.and_then(|id| {
            let position = (index as i32).checked_sub(1)?;
            let position = usize::try_from(position).ok()?;
            objectives_for_quest(&sim.quest_log_entries, id)
                .get(position)
                .cloned()
        })
    };
    push_objective(state, objective.as_ref());
    Ok(3)
}

fn push_objective(
    state: &mut LuaState,
    objective: Option<&crate::c_api::quest_objectives::QuestObjective>,
) {
    let Some(objective) = objective else {
        for _ in 0..3 {
            state.push(Val::Nil);
        }
        return;
    };
    let text = create_string(state, &objective.text);
    state.push(text);
    let kind = create_string(state, &objective.objective_type);
    state.push(kind);
    let finished = if objective.finished {
        Val::Num(1.0)
    } else {
        Val::Nil
    };
    state.push(finished);
}

pub(super) fn get_quest_log_quest_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = selected_active_quest_id(&*borrow_state(state)?).unwrap_or(0);
    let description = match super::handlers::find_quest_by_id(id) {
        Some((_, super::data::QuestLogEntry::Quest { description, .. })) => *description,
        _ => "",
    };
    let objectives = read_selected_objective_text(state)?;
    let description = create_string(state, description);
    state.push(description);
    let objectives = create_string(state, &objectives);
    state.push(objectives);
    Ok(2)
}
