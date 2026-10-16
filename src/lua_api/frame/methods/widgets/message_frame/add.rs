//! AddMessage / BackFillMessage / add helpers.

use super::super::shared::{opt_f32, opt_string};
use super::scroll::truncate_messages;
use crate::lua_api::message_frame::{Message, MessageFrameData};
use crate::lua_api::methods::{borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn add_message(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    add_message_core_from_stack(state, id, true);
    Ok(0)
}

pub(super) fn add_msg(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    add_message_core_from_stack(state, id, true);
    Ok(0)
}

pub(super) fn add_message_silent(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    add_message_core_from_stack(state, id, false);
    Ok(0)
}

pub(super) fn backfill_message(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = match opt_string(state, 2) {
        Some(t) => t,
        None => return Ok(0),
    };
    let r = opt_f32(state, 3).unwrap_or(1.0);
    let g = opt_f32(state, 4).unwrap_or(1.0);
    let b = opt_f32(state, 5).unwrap_or(1.0);
    let a = opt_f32(state, 6).unwrap_or(1.0);

    let mut sim = borrow_state_mut(state)?;
    log_message_sim(&sim, id, &text);
    let timestamp = sim.start_time.elapsed().as_secs_f64();
    let data = sim.message_frames.entry(id).or_default();
    data.messages.insert(
        0,
        Message {
            text,
            r,
            g,
            b,
            a,
            message_id: None,
            access_id: None,
            extra_data: None,
            timestamp,
        },
    );
    if data.messages.len() > data.max_lines {
        data.messages.pop();
    }
    data.display_dirty = true;
    Ok(0)
}

/// Add the existing simulator message shape or the Wrath history shape.
pub(super) fn add_message_core_from_stack(state: &mut LuaState, id: u64, log: bool) {
    let Some(text) = opt_string(state, 2) else {
        return;
    };
    let mut message = read_message_from_stack(state, text);
    let mut sim = match borrow_state_mut(state) {
        Ok(s) => s,
        Err(_) => return,
    };
    if log {
        log_message_sim(&sim, id, &message.text);
    }
    message.timestamp = sim.start_time.elapsed().as_secs_f64();
    let data = sim.message_frames.entry(id).or_default();
    insert_message(data, message);
    truncate_messages(data);
    data.display_dirty = true;
}

fn read_message_from_stack(state: &LuaState, text: String) -> Message {
    let mut message = Message {
        text,
        r: opt_f32(state, 3).unwrap_or(1.0),
        g: opt_f32(state, 4).unwrap_or(1.0),
        b: opt_f32(state, 5).unwrap_or(1.0),
        a: opt_f32(state, 6).unwrap_or(1.0),
        message_id: read_optional_id(state, 7),
        access_id: None,
        extra_data: None,
        timestamp: 0.0,
    };
    // Public Wrath consumer: text, r, g, b, lineID, false, accessID, extraData.
    let wrath_history =
        cfg!(feature = "client-wrath") && matches!(stack_val(state, 7), Val::Bool(false));
    if wrath_history {
        message.a = 1.0;
        message.message_id = read_optional_id(state, 6);
        message.access_id = read_optional_id(state, 8);
        message.extra_data = read_optional_id(state, 9);
    }
    message
}

pub(super) fn read_optional_id(state: &LuaState, index: i32) -> Option<i64> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i64),
        _ => None,
    }
}

fn insert_message(data: &mut MessageFrameData, message: Message) {
    if data.insert_mode == "TOP" {
        data.messages.insert(0, message);
    } else {
        data.messages.push(message);
    }
}

pub(super) fn log_message_sim(sim: &crate::lua_api::SimState, id: u64, text: &str) {
    let name = sim
        .widgets
        .get(id)
        .and_then(|w| w.name.as_deref())
        .unwrap_or("?");
    let clean = crate::dump::strip_wow_escapes(text);
    eprintln!("[{name}] {clean}");
}
