//! Wrath12340 history binding; other profiles retain existing list semantics.

use super::{opt_string, val_to_f64};
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{IntoStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, runtime_error};

pub(in super::super) fn add_history_line(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if ACTIVE == ClientProfile::Wrath {
        return append_wrath_history_from_stack(state, id);
    }
    let text = opt_string(state, 2).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.editbox_history.push(Some(text));
        let max = frame.editbox_history_max;
        if max > 0 && frame.editbox_history.len() > max as usize {
            frame.editbox_history.remove(0);
        }
    }
    Ok(0)
}

fn append_wrath_history_from_stack(state: &mut LuaState, id: u64) -> LuaResult<u32> {
    if state.top.saturating_sub(state.base) != 2 {
        return history_usage_error(state, id, "AddHistoryLine(\"text\")");
    }
    let enabled = borrow_state(state)?
        .widgets
        .get(id)
        .is_some_and(|frame| frame.editbox_history_max > 0);
    if !enabled {
        return Ok(0);
    }
    let value = stack_val(state, 2);
    let Some(text) = crate::lua_api::wrath_arguments::convert_string(state, value)? else {
        // 12340's null-string conversion reaches undefined uninitialized memory.
        // Intentionally unmodeled: do not create a blank entry or a Lua rejection.
        tracing::warn!(
            "Wrath AddHistoryLine null-string native behavior is undefined; not modeled"
        );
        return Ok(0);
    };
    if let Some(frame) = borrow_state_mut(state)?.widgets.get_mut_visual(id) {
        frame.append_wrath_history(text);
    }
    Ok(0)
}

pub(in super::super) fn get_history_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let count = sim.widgets.get(id).map_or(0, |frame| {
        if ACTIVE == ClientProfile::Wrath {
            frame.editbox_history_max as usize
        } else {
            frame.editbox_history.len()
        }
    });
    drop(sim);
    (count as f64).into_stack(state)
}

pub(in super::super) fn set_history_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if ACTIVE == ClientProfile::Wrath {
        return resize_wrath_history_from_stack(state, id);
    }
    let max = val_to_f64(stack_val(state, 2)) as i32;
    if let Some(frame) = borrow_state_mut(state)?.widgets.get_mut_visual(id) {
        frame.editbox_history_max = max;
    }
    Ok(0)
}

fn resize_wrath_history_from_stack(state: &mut LuaState, id: u64) -> LuaResult<u32> {
    let value = stack_val(state, 2);
    let capacity = crate::lua_api::wrath_arguments::convert_number(state, value)?
        .map(|number| number as i32)
        .filter(|capacity| *capacity > 0);
    let Some(capacity) = capacity else {
        return history_usage_error(state, id, "SetHistoryLines(numLines)");
    };
    if let Some(frame) = borrow_state_mut(state)?.widgets.get_mut_visual(id) {
        frame.resize_wrath_history(capacity as usize);
    }
    Ok(0)
}

pub(in super::super) fn clear_history(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if let Some(frame) = borrow_state_mut(state)?.widgets.get_mut_visual(id) {
        if ACTIVE == ClientProfile::Wrath {
            frame.clear_wrath_history();
        } else {
            frame.editbox_history.clear();
        }
    }
    Ok(0)
}

fn history_usage_error(state: &LuaState, id: u64, signature: &str) -> LuaResult<u32> {
    let sim = borrow_state(state)?;
    let name = sim
        .widgets
        .get(id)
        .and_then(|frame| frame.name.as_deref())
        .unwrap_or("<unnamed>");
    Err(runtime_error(format!("Usage: {name}:{signature}")))
}
