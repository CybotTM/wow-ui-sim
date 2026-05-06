//! Scroll family: ScrollUp/Down, PageUp/Down, ScrollToTop/Bottom, offset, allowed.

use super::super::shared::{val_to_bool, val_to_f64};
use super::callbacks::call_scroll_changed_callback;
use crate::lua_api::message_frame::MessageFrameData;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{IntoStack, stack_val};
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(super) fn scroll_up(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    adjust_scroll(state, id, 1);
    Ok(0)
}

pub(super) fn scroll_down(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    adjust_scroll(state, id, -1);
    Ok(0)
}

pub(super) fn page_up(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    page_scroll(state, id, true);
    Ok(0)
}

pub(super) fn page_down(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    page_scroll(state, id, false);
    Ok(0)
}

pub(super) fn scroll_to_top(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    scroll_to_edge(state, id, true);
    Ok(0)
}

pub(super) fn scroll_to_bottom(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    scroll_to_edge(state, id, false);
    Ok(0)
}

pub(super) fn at_top(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| d.scroll_offset == message_frame_scroll_limit(d))
        .unwrap_or(true);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn at_bottom(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| d.scroll_offset == 0)
        .unwrap_or(true);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn get_max_scroll_range(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| message_frame_scroll_limit(d) as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_scroll_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let offset = val_to_f64(stack_val(state, 2)) as i32;
    let changed = {
        let mut sim = borrow_state_mut(state)?;
        let data = sim.message_frames.entry(id).or_default();
        let changed = data.scroll_offset != offset;
        data.scroll_offset = offset;
        changed
    };
    if changed {
        call_scroll_changed_callback(state, id, offset)?;
    }
    Ok(0)
}

pub(super) fn get_scroll_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| d.scroll_offset as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_scroll_allowed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let v = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    sim.message_frames.entry(id).or_default().scroll_allowed = v;
    Ok(0)
}

pub(super) fn is_scroll_allowed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| d.scroll_allowed)
        .unwrap_or(true);
    drop(sim);
    v.into_stack(state)
}

// ── Scroll helpers ────────────────────────────────────────────────────────────

fn adjust_scroll(state: &mut LuaState, id: u64, delta: i32) {
    scroll_by(state, id, |_| delta);
}

fn page_scroll(state: &mut LuaState, id: u64, towards_top: bool) {
    scroll_by(state, id, |data| {
        let page = data.max_lines.max(1) as i32;
        if towards_top { page } else { -page }
    });
}

fn scroll_by(state: &mut LuaState, id: u64, delta_for_data: impl FnOnce(&MessageFrameData) -> i32) {
    let mut sim = match borrow_state_mut(state) {
        Ok(s) => s,
        Err(_) => return,
    };
    let Some(data) = sim.message_frames.get_mut(&id) else {
        return;
    };
    if !data.scroll_allowed {
        return;
    }
    let delta = delta_for_data(data);
    let max = message_frame_scroll_limit(data);
    data.scroll_offset = (data.scroll_offset + delta).clamp(0, max);
}

fn scroll_to_edge(state: &mut LuaState, id: u64, top: bool) {
    let mut sim = match borrow_state_mut(state) {
        Ok(s) => s,
        Err(_) => return,
    };
    let Some(data) = sim.message_frames.get_mut(&id) else {
        return;
    };
    if !data.scroll_allowed {
        return;
    }
    data.scroll_offset = if top {
        message_frame_scroll_limit(data)
    } else {
        0
    };
}

pub(super) fn message_frame_scroll_limit(data: &MessageFrameData) -> i32 {
    data.messages.len().min(data.max_lines).saturating_sub(1) as i32
}

pub(super) fn truncate_messages(data: &mut MessageFrameData) {
    while data.messages.len() > data.max_lines {
        if data.insert_mode == "TOP" {
            data.messages.pop();
        } else {
            data.messages.remove(0);
        }
    }
    data.scroll_offset = data
        .scroll_offset
        .clamp(0, message_frame_scroll_limit(data));
}
