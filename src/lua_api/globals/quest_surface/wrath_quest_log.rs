//! Wrath legacy reads over supplied flat rich rows, without synthetic headers.

use crate::lua_api::methods::{borrow_state, create_string};
use crate::lua_bridge::FromStack;
use rilua::{LuaResult, Val, vm::state::LuaState};

fn supplied_index(state: &LuaState) -> LuaResult<usize> {
    let index = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as usize;
    Ok(index.wrapping_sub(1))
}

pub(super) fn get_abandon_quest_name(state: &mut LuaState) -> LuaResult<u32> {
    let title = {
        let sim = borrow_state(state)?;
        sim.abandon_quest_id.and_then(|id| {
            sim.quest_log_entries
                .entries
                .iter()
                .find(|row| row.quest_id as u32 == id)
                .map(|row| row.title.clone())
        })
    };
    let Some(title) = title else {
        return Ok(0);
    };
    let title = create_string(state, &title);
    state.push(title);
    Ok(1)
}

pub(super) fn get_quest_log_title(state: &mut LuaState) -> LuaResult<u32> {
    let index = supplied_index(state)?;
    let row = borrow_state(state)?
        .quest_log_entries
        .entries
        .get(index)
        .cloned();
    let Some(row) = row else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let title = create_string(state, &row.title);
    let completion = if row.is_failed {
        Val::Num(-1.0)
    } else if row.is_complete {
        Val::Num(1.0)
    } else {
        Val::Nil
    };
    // WotLK: title, level, tag, group, header, collapsed, complete, daily, ID, displayID.
    for value in [
        title,
        Val::Num(row.level as f64),
        Val::Nil,
        Val::Nil,
        Val::Nil,
        Val::Nil,
        completion,
        Val::Nil,
        Val::Num(row.quest_id as f64),
        Val::Nil,
    ] {
        state.push(value);
    }
    Ok(10)
}

pub(super) fn select_quest_log_entry(state: &mut LuaState) -> LuaResult<u32> {
    let index = supplied_index(state)?;
    let quest_id = borrow_state(state)?
        .quest_log_entries
        .entries
        .get(index)
        .map(|row| row.quest_id)
        .unwrap_or(0);
    super::handlers::set_selected_quest_id(state, quest_id)?;
    Ok(0)
}

pub(super) fn get_quest_log_selection(state: &mut LuaState) -> LuaResult<u32> {
    let index = {
        let sim = borrow_state(state)?;
        sim.selected_quest_log_id
            .and_then(|id| {
                sim.quest_log_entries
                    .entries
                    .iter()
                    .position(|row| row.quest_id as u32 == id)
            })
            .map_or(0, |index| index + 1)
    };
    state.push(Val::Num(index as f64));
    Ok(1)
}
