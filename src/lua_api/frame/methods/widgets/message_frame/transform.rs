//! Message transforms: remove_by_predicate, transform_messages, adjust_message_colors,
//! snapshot/replace helpers, call_function_multi, and low-level value helpers.

use super::callbacks::set_display_dirty_and_fire_callbacks;
use crate::lua_api::message_frame::Message;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, frame_id_from_stack, val_to_string,
};
use crate::lua_bridge::stack_val;
use rilua::vm::callinfo::LUA_MULTRET;
use rilua::vm::execute::{CallResult, execute};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::scroll::message_frame_scroll_limit;

pub(super) fn remove_messages_by_predicate(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let predicate = match stack_val(state, 2) {
        Val::Function(f) => Val::Function(f),
        _ => return Ok(0),
    };
    let snapshot = get_message_snapshot(state, id);
    if snapshot.is_empty() {
        return Ok(0);
    }
    let mut kept = Vec::with_capacity(snapshot.len());
    let mut removed_any = false;
    for message in &snapshot {
        let args = build_message_args(state, message);
        let results = call_function_multi(state, predicate, &args)?;
        if is_truthy(results.into_iter().next().unwrap_or(Val::Nil)) {
            removed_any = true;
        } else {
            kept.push(message.clone());
        }
    }
    if removed_any {
        replace_messages(state, id, kept);
        set_display_dirty_and_fire_callbacks(state, id)?;
    }
    Ok(0)
}

pub(super) fn transform_messages(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let predicate = match stack_val(state, 2) {
        Val::Function(f) => Val::Function(f),
        _ => return Ok(0),
    };
    let transform = match stack_val(state, 3) {
        Val::Function(f) => Val::Function(f),
        _ => return Ok(0),
    };
    let snapshot = get_message_snapshot(state, id);
    if snapshot.is_empty() {
        return Ok(0);
    }
    let mut transformed = Vec::with_capacity(snapshot.len());
    let mut changed_any = false;
    for message in &snapshot {
        let pred_args = build_message_args(state, message);
        let pred_results = call_function_multi(state, predicate, &pred_args)?;
        let passes = is_truthy(pred_results.into_iter().next().unwrap_or(Val::Nil));
        if passes {
            let xform_args = build_message_args(state, message);
            let results = call_function_multi(state, transform, &xform_args)?;
            transformed.push(message_from_transform_results(state, message, results));
            changed_any = true;
        } else {
            transformed.push(message.clone());
        }
    }
    if changed_any {
        replace_messages(state, id, transformed);
        set_display_dirty_and_fire_callbacks(state, id)?;
    }
    Ok(0)
}

pub(super) fn adjust_message_colors(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let transform = match stack_val(state, 2) {
        Val::Function(f) => Val::Function(f),
        _ => return Ok(0),
    };
    let snapshot = get_message_snapshot(state, id);
    if snapshot.is_empty() {
        return Ok(0);
    }
    let mut recolored = Vec::with_capacity(snapshot.len());
    let mut changed_any = false;
    for message in &snapshot {
        let args = build_message_args(state, message);
        let results = call_function_multi(state, transform, &args)?;
        if let Some((r, g, b)) = color_from_results(message, results) {
            let mut updated = message.clone();
            updated.r = r;
            updated.g = g;
            updated.b = b;
            recolored.push(updated);
            changed_any = true;
        } else {
            recolored.push(message.clone());
        }
    }
    if changed_any {
        replace_messages(state, id, recolored);
        set_display_dirty_and_fire_callbacks(state, id)?;
    }
    Ok(0)
}

// ── Message snapshot / replace helpers ───────────────────────────────────────

pub(super) fn get_message_snapshot(state: &mut LuaState, frame_id: u64) -> Vec<Message> {
    let sim = match borrow_state(state) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    sim.message_frames
        .get(&frame_id)
        .map(|d| d.messages.clone())
        .unwrap_or_default()
}

pub(super) fn replace_messages(state: &mut LuaState, frame_id: u64, messages: Vec<Message>) {
    let mut sim = match borrow_state_mut(state) {
        Ok(s) => s,
        Err(_) => return,
    };
    let data = sim.message_frames.entry(frame_id).or_default();
    data.messages = messages;
    data.scroll_offset = data
        .scroll_offset
        .clamp(0, message_frame_scroll_limit(data));
}

// ── Multi-return call helper ──────────────────────────────────────────────────

/// Call a Lua function and collect ALL return values (unlike `call_function_state`
/// which discards all but the first).
pub(super) fn call_function_multi(
    state: &mut LuaState,
    func: Val,
    args: &[Val],
) -> LuaResult<Vec<Val>> {
    let Val::Function(_) = func else {
        return Ok(vec![]);
    };
    let func_idx = state.top;
    state.ensure_stack(func_idx + 1 + args.len());
    state.stack_set(func_idx, func);
    state.top = func_idx + 1;
    for arg in args {
        let top = state.top;
        state.stack_set(top, *arg);
        state.top = top + 1;
    }
    let save_base = state.base;
    state.base = func_idx + 1;
    let result = match state.precall(func_idx, LUA_MULTRET)? {
        CallResult::Lua => execute(state),
        CallResult::Rust => Ok(()),
    };
    let results: Vec<Val> = if result.is_ok() && state.top > func_idx {
        (func_idx..state.top).map(|i| state.stack_get(i)).collect()
    } else {
        vec![]
    };
    state.top = func_idx;
    state.base = save_base;
    result?;
    Ok(results)
}

// ── Message args / transform helpers ─────────────────────────────────────────

pub(super) fn build_message_args(state: &mut LuaState, message: &Message) -> Vec<Val> {
    let text_val = create_string(state, &message.text);
    let msg_id = match message.message_id {
        Some(id) => Val::Num(id as f64),
        None => Val::Nil,
    };
    vec![
        text_val,
        Val::Num(message.r as f64),
        Val::Num(message.g as f64),
        Val::Num(message.b as f64),
        Val::Num(message.a as f64),
        msg_id,
        Val::Num(message.timestamp),
    ]
}

pub(super) fn message_from_transform_results(
    state: &mut LuaState,
    original: &Message,
    results: Vec<Val>,
) -> Message {
    let mut iter = results.into_iter();
    let text = iter
        .next()
        .and_then(|v| val_to_string(state, v))
        .unwrap_or_else(|| original.text.clone());
    let r = val_to_opt_f32(iter.next()).unwrap_or(original.r);
    let g = val_to_opt_f32(iter.next()).unwrap_or(original.g);
    let b = val_to_opt_f32(iter.next()).unwrap_or(original.b);
    let a = val_to_opt_f32(iter.next()).unwrap_or(original.a);
    let message_id = val_to_opt_i64(iter.next()).or(original.message_id);
    let timestamp = val_to_opt_f64(iter.next()).unwrap_or(original.timestamp);
    Message {
        text,
        r,
        g,
        b,
        a,
        message_id,
        timestamp,
    }
}

pub(super) fn color_from_results(original: &Message, results: Vec<Val>) -> Option<(f32, f32, f32)> {
    let mut iter = results.into_iter();
    let change_color = iter.next().unwrap_or(Val::Nil);
    if !is_truthy(change_color) {
        return None;
    }
    let r = val_to_opt_f32(iter.next()).unwrap_or(original.r);
    let g = val_to_opt_f32(iter.next()).unwrap_or(original.g);
    let b = val_to_opt_f32(iter.next()).unwrap_or(original.b);
    Some((r, g, b))
}

pub(super) fn val_to_opt_f32(val: Option<Val>) -> Option<f32> {
    match val {
        Some(Val::Num(n)) => Some(n as f32),
        _ => None,
    }
}

pub(super) fn val_to_opt_f64(val: Option<Val>) -> Option<f64> {
    match val {
        Some(Val::Num(n)) => Some(n),
        _ => None,
    }
}

pub(super) fn val_to_opt_i64(val: Option<Val>) -> Option<i64> {
    match val {
        Some(Val::Num(n)) => Some(n as i64),
        _ => None,
    }
}

pub(super) fn is_truthy(val: Val) -> bool {
    !matches!(val, Val::Nil | Val::Bool(false))
}
