//! Build-12340 manual watches: ordered identities, indexed through current rows.

use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_api::state::SimState;
use crate::lua_bridge::stack_val;
use rilua::{LuaResult, Val, vm::state::LuaState};
use std::ffi::{CStr, CString};

const MAX_MANUAL_WATCHES: usize = 25;

pub(super) const MANUAL_WATCH_FUNCTIONS: &[(&str, super::SurfaceFn)] = &[
    ("AddQuestWatch", add_quest_watch),
    ("RemoveQuestWatch", remove_quest_watch),
    ("GetNumQuestWatches", get_num_quest_watches),
    ("IsQuestWatched", is_quest_watched),
    ("GetQuestIndexForWatch", get_quest_index_for_watch),
];

fn parse_numeric_string(value: &[u8]) -> Option<f64> {
    let value = CString::new(value).ok()?;
    let mut end = std::ptr::null_mut();
    // CString supplies a live NUL-terminated input; strtod sets end within it.
    let number = unsafe { libc::strtod(value.as_ptr(), &mut end) };
    if std::ptr::eq(end, value.as_ptr()) {
        return None;
    }
    // end remains inside value, including its terminating NUL.
    let remaining = unsafe { CStr::from_ptr(end) }.to_bytes();
    remaining
        .iter()
        .all(u8::is_ascii_whitespace)
        .then_some(number)
}

fn numeric_argument(state: &LuaState) -> Option<f64> {
    match stack_val(state, 1) {
        Val::Num(number) => Some(number),
        Val::Str(string) => {
            let string = state.gc.string_arena.get(string)?;
            parse_numeric_string(string.data())
        }
        _ => None,
    }
}

fn quest_id_at_row(sim: &SimState, row: f64) -> Option<i32> {
    sim.quest_log_entries
        .entries
        .get((row as usize).wrapping_sub(1))
        .map(|entry| entry.quest_id)
        .filter(|id| *id > 0)
}

fn mutation_quest_id(state: &LuaState, sim: &SimState) -> Option<i32> {
    if let Some(row) = numeric_argument(state) {
        return quest_id_at_row(sim, row);
    }
    let selected = sim.selected_quest_log_id?;
    sim.quest_log_entries
        .entries
        .iter()
        .find(|entry| entry.quest_id as u32 == selected)
        .map(|entry| entry.quest_id)
        .filter(|id| *id > 0)
}

fn add_quest_watch(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    let Some(id) = mutation_quest_id(state, &sim) else {
        return Ok(0);
    };
    let watches = &mut sim.quest_log_entries.manual_watch_ids;
    // At most 25 identities; one ordered vector retains native append order.
    if watches.len() < MAX_MANUAL_WATCHES && !watches.contains(&id) {
        watches.push(id);
    }
    Ok(0)
}

fn remove_quest_watch(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    let Some(id) = mutation_quest_id(state, &sim) else {
        return Ok(0);
    };
    sim.quest_log_entries
        .manual_watch_ids
        .retain(|watched| *watched != id);
    Ok(0)
}

fn get_num_quest_watches(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?
        .quest_log_entries
        .manual_watch_ids
        .len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn is_quest_watched(state: &mut LuaState) -> LuaResult<u32> {
    let row = numeric_argument(state)
        .ok_or_else(|| rilua::runtime_error("Usage: IsQuestWatched(index)"))?;
    let watched = {
        let sim = borrow_state(state)?;
        quest_id_at_row(&sim, row)
            .is_some_and(|id| sim.quest_log_entries.manual_watch_ids.contains(&id))
    };
    state.push(if watched { Val::Num(1.0) } else { Val::Nil });
    Ok(1)
}

fn get_quest_index_for_watch(state: &mut LuaState) -> LuaResult<u32> {
    let watch = numeric_argument(state)
        .ok_or_else(|| rilua::runtime_error("Usage: GetQuestIndexForWatch(index)"))?;
    let row = {
        let sim = borrow_state(state)?;
        sim.quest_log_entries
            .manual_watch_ids
            .get((watch as usize).wrapping_sub(1))
            .and_then(|id| {
                sim.quest_log_entries
                    .entries
                    .iter()
                    .position(|entry| entry.quest_id == *id)
            })
    };
    state.push(row.map_or(Val::Nil, |index| Val::Num((index + 1) as f64)));
    Ok(1)
}
