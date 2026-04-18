//! MessageFrame widget methods: AddMessage, scrolling, fading, message history.
//!
//! Consolidates the three master-branch files:
//! - widget_message_frame.rs
//! - widget_message_frame_callbacks.rs
//! - widget_message_frame_scroll.rs

use super::shared::{opt_f32, opt_string, val_to_bool, val_to_f64};
use crate::lua_api::message_frame::{Message, MessageFrameData};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, frame_id_from_stack,
    frame_ref, get_or_create_frame_fields, table_get, table_set, val_to_string,
};
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn};
use crate::widget::{Frame, WidgetType};
use rilua::vm::callinfo::LUA_MULTRET;
use rilua::vm::execute::{CallResult, execute};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

// ── Field name constants ──────────────────────────────────────────────────────

const ON_SCROLL_CHANGED_CB: &str = "onScrollChangedCallback";
const ON_LINE_RIGHT_CLICKED_CB: &str = "onLineRightClickedCallback";
const ON_DISPLAY_REFRESHED_CBS: &str = "onDisplayRefreshedCallbacks";
const ON_TEXT_COPIED_CB: &str = "onTextCopiedCallback";
const ON_TEXT_COPIED_ORIG: &str = "_onTextCopiedCallback_orig";
const INDENTED_WORD_WRAP_FIELD: &str = "_mf_indented_word_wrap";

// ── Add / backfill ────────────────────────────────────────────────────────────

fn add_message(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    add_message_core_from_stack(state, id, true);
    Ok(0)
}

fn add_msg(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    add_message_core_from_stack(state, id, true);
    Ok(0)
}

fn add_message_silent(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    add_message_core_from_stack(state, id, false);
    Ok(0)
}

fn backfill_message(state: &mut LuaState) -> LuaResult<u32> {
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
            timestamp,
        },
    );
    if data.messages.len() > data.max_lines {
        data.messages.pop();
    }
    data.display_dirty = true;
    Ok(0)
}

// ── Core add helper ───────────────────────────────────────────────────────────

/// Read args from the Lua stack (positions 2+) and insert a message.
///
/// This borrows `sim` mutably via `borrow_state_mut` before calling, so the
/// caller must NOT hold any borrow of `SimState` when calling this.
///
/// Stack layout expected: arg1=self(idx1), arg2=text(idx2), arg3=r, arg4=g,
/// arg5=b, arg6=a, arg7=message_id.
fn add_message_core_from_stack(state: &mut LuaState, id: u64, log: bool) {
    let text = match opt_string(state, 2) {
        Some(t) => t,
        None => return,
    };
    let r = opt_f32(state, 3).unwrap_or(1.0);
    let g = opt_f32(state, 4).unwrap_or(1.0);
    let b = opt_f32(state, 5).unwrap_or(1.0);
    let a = opt_f32(state, 6).unwrap_or(1.0);
    let message_id = match stack_val(state, 7) {
        Val::Num(n) => Some(n as i64),
        _ => None,
    };

    let mut sim = match borrow_state_mut(state) {
        Ok(s) => s,
        Err(_) => return,
    };
    if log {
        log_message_sim(&sim, id, &text);
    }
    let timestamp = sim.start_time.elapsed().as_secs_f64();
    let data = sim.message_frames.entry(id).or_default();
    insert_message(data, text, r, g, b, a, message_id, timestamp);
    truncate_messages(data);
    data.display_dirty = true;
}

fn insert_message(
    data: &mut MessageFrameData,
    text: String,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    message_id: Option<i64>,
    timestamp: f64,
) {
    let msg = Message {
        text,
        r,
        g,
        b,
        a,
        message_id,
        timestamp,
    };
    if data.insert_mode == "TOP" {
        data.messages.insert(0, msg);
    } else {
        data.messages.push(msg);
    }
}

fn log_message_sim(sim: &crate::lua_api::SimState, id: u64, text: &str) {
    let name = sim
        .widgets
        .get(id)
        .and_then(|w| w.name.as_deref())
        .unwrap_or("?");
    let clean = crate::dump::strip_wow_escapes(text);
    eprintln!("[{name}] {clean}");
}

// ── Clear ─────────────────────────────────────────────────────────────────────

fn clear(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(data) = sim.message_frames.get_mut(&id) {
        data.messages.clear();
        data.scroll_offset = 0;
    }
    Ok(0)
}

fn clear_text(state: &mut LuaState) -> LuaResult<u32> {
    clear(state)
}

// ── Count / max lines ─────────────────────────────────────────────────────────

fn get_num_messages(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let count = sim
        .message_frames
        .get(&id)
        .map(|d| d.messages.len())
        .unwrap_or(0) as f64;
    drop(sim);
    count.into_stack(state)
}

fn set_max_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let max = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    let data = sim.message_frames.entry(id).or_default();
    data.max_lines = max.max(1) as usize;
    while data.messages.len() > data.max_lines {
        truncate_messages(data);
    }
    data.scroll_offset = data
        .scroll_offset
        .clamp(0, message_frame_scroll_limit(data));
    data.display_dirty = true;
    Ok(0)
}

fn get_max_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| d.max_lines)
        .unwrap_or(120) as f64;
    drop(sim);
    v.into_stack(state)
}

// ── Fading ────────────────────────────────────────────────────────────────────

fn set_fading(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    sim.message_frames.entry(id).or_default().fading = value;
    Ok(0)
}

fn get_fading(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| d.fading)
        .unwrap_or(true);
    drop(sim);
    v.into_stack(state)
}

fn set_time_visible(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let v = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    sim.message_frames.entry(id).or_default().time_visible = v;
    Ok(0)
}

fn get_time_visible(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| d.time_visible)
        .unwrap_or(10.0);
    drop(sim);
    v.into_stack(state)
}

fn set_fade_duration(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let v = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    sim.message_frames.entry(id).or_default().fade_duration = v;
    Ok(0)
}

fn get_fade_duration(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| d.fade_duration)
        .unwrap_or(3.0);
    drop(sim);
    v.into_stack(state)
}

fn set_fade_power(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let v = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    sim.message_frames.entry(id).or_default().fade_power = v;
    Ok(0)
}

fn get_fade_power(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| d.fade_power)
        .unwrap_or(1.0);
    drop(sim);
    v.into_stack(state)
}

// ── Insert mode ───────────────────────────────────────────────────────────────

fn set_insert_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mode = opt_string(state, 2).unwrap_or_else(|| "BOTTOM".to_string());
    let mut sim = borrow_state_mut(state)?;
    sim.message_frames.entry(id).or_default().insert_mode = mode;
    Ok(0)
}

fn get_insert_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mode = {
        let sim = borrow_state(state)?;
        sim.message_frames
            .get(&id)
            .map(|d| d.insert_mode.clone())
            .unwrap_or_else(|| "BOTTOM".to_string())
    };
    let v = create_string(state, &mode);
    v.into_stack(state)
}

// ── Misc ──────────────────────────────────────────────────────────────────────

fn set_text_copyable(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let v = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    sim.message_frames.entry(id).or_default().text_copyable = v;
    Ok(0)
}

fn is_text_copyable(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .message_frames
        .get(&id)
        .map(|d| d.text_copyable)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn has_message_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let msg_id = val_to_f64(stack_val(state, 2)) as i64;
    let sim = borrow_state(state)?;
    let found = sim
        .message_frames
        .get(&id)
        .map(|d| d.messages.iter().any(|m| m.message_id == Some(msg_id)))
        .unwrap_or(false);
    drop(sim);
    found.into_stack(state)
}

fn get_message_info(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let index = val_to_f64(stack_val(state, 2)) as i32;
    let sim = borrow_state(state)?;
    let (text, r, g, b, a, ts) = sim
        .message_frames
        .get(&id)
        .and_then(|d| d.messages.get((index - 1) as usize))
        .map(|m| {
            (
                m.text.clone(),
                m.r as f64,
                m.g as f64,
                m.b as f64,
                m.a as f64,
                m.timestamp,
            )
        })
        .unwrap_or((String::new(), 1.0, 1.0, 1.0, 1.0, 0.0));
    drop(sim);
    let text_val = create_string(state, &text);
    text_val.into_stack(state)?;
    state.push(Val::Num(r));
    state.push(Val::Num(g));
    state.push(Val::Num(b));
    state.push(Val::Num(a));
    state.push(Val::Num(ts));
    Ok(6)
}

// ── Scroll ────────────────────────────────────────────────────────────────────

fn scroll_up(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    adjust_scroll(state, id, 1);
    Ok(0)
}

fn scroll_down(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    adjust_scroll(state, id, -1);
    Ok(0)
}

fn page_up(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    page_scroll(state, id, true);
    Ok(0)
}

fn page_down(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    page_scroll(state, id, false);
    Ok(0)
}

fn scroll_to_top(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    scroll_to_edge(state, id, true);
    Ok(0)
}

fn scroll_to_bottom(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    scroll_to_edge(state, id, false);
    Ok(0)
}

fn at_top(state: &mut LuaState) -> LuaResult<u32> {
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

fn at_bottom(state: &mut LuaState) -> LuaResult<u32> {
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

fn get_max_scroll_range(state: &mut LuaState) -> LuaResult<u32> {
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

fn set_scroll_offset(state: &mut LuaState) -> LuaResult<u32> {
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

fn get_scroll_offset(state: &mut LuaState) -> LuaResult<u32> {
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

fn set_scroll_allowed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let v = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    sim.message_frames.entry(id).or_default().scroll_allowed = v;
    Ok(0)
}

fn is_scroll_allowed(state: &mut LuaState) -> LuaResult<u32> {
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
    let max = message_frame_scroll_limit(data);
    data.scroll_offset = (data.scroll_offset + delta).clamp(0, max);
}

fn page_scroll(state: &mut LuaState, id: u64, towards_top: bool) {
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
    let page = data.max_lines.max(1) as i32;
    let delta = if towards_top { page } else { -page };
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

fn message_frame_scroll_limit(data: &MessageFrameData) -> i32 {
    data.messages.len().min(data.max_lines).saturating_sub(1) as i32
}

fn truncate_messages(data: &mut MessageFrameData) {
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

// ── Callbacks ─────────────────────────────────────────────────────────────────

fn set_indented_word_wrap(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let v = val_to_bool(stack_val(state, 2));
    let fields = get_or_create_frame_fields(state, id);
    table_set(state, fields, INDENTED_WORD_WRAP_FIELD, Val::Bool(v));
    Ok(0)
}

fn get_indented_word_wrap(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let fields = get_or_create_frame_fields(state, id);
    let v = match table_get(state, fields, INDENTED_WORD_WRAP_FIELD) {
        Val::Bool(b) => b,
        _ => false,
    };
    v.into_stack(state)
}

fn set_on_scroll_changed_callback(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let func = stack_val(state, 2);
    let fields = get_or_create_frame_fields(state, id);
    let stored = match func {
        Val::Function(_) => func,
        _ => Val::Nil,
    };
    table_set(state, fields, ON_SCROLL_CHANGED_CB, stored);
    Ok(0)
}

fn set_on_line_right_clicked_callback(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let func = stack_val(state, 2);
    let fields = get_or_create_frame_fields(state, id);
    let stored = match func {
        Val::Function(_) => func,
        _ => Val::Nil,
    };
    table_set(state, fields, ON_LINE_RIGHT_CLICKED_CB, stored);
    Ok(0)
}

fn add_on_display_refreshed_callback(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let func = stack_val(state, 2);
    let Val::Function(_) = func else {
        return Ok(0);
    };
    let fields = get_or_create_frame_fields(state, id);
    let callbacks = get_or_create_display_refreshed_callbacks(state, fields);
    let Val::Table(cbs_ref) = callbacks else {
        return Ok(0);
    };
    let next_idx = {
        let arena = &state.gc.string_arena;
        state
            .gc
            .tables
            .get(cbs_ref)
            .map(|t| t.len(arena) as i64 + 1)
            .unwrap_or(1)
    };
    if let Some(t) = state.gc.tables.get_mut(cbs_ref) {
        let _ = t.raw_set(Val::Num(next_idx as f64), func, &state.gc.string_arena);
    }
    state.gc.barrier_back(cbs_ref);
    Ok(0)
}

fn set_on_text_copied_callback(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let func = stack_val(state, 2);
    let fields = get_or_create_frame_fields(state, id);
    match func {
        Val::Function(_) => {
            table_set(state, fields, ON_TEXT_COPIED_ORIG, func);
            // Store an alias under the public key too (the original does a wrapper,
            // but we simplify: both keys point to the same function).
            table_set(state, fields, ON_TEXT_COPIED_CB, func);
        }
        _ => {
            table_set(state, fields, ON_TEXT_COPIED_CB, Val::Nil);
            table_set(state, fields, ON_TEXT_COPIED_ORIG, Val::Nil);
        }
    }
    Ok(0)
}

fn mark_display_dirty(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    set_display_dirty_and_fire_callbacks(state, id)
}

fn reset_all_fade_times(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    {
        let mut sim = borrow_state_mut(state)?;
        let now = sim.start_time.elapsed().as_secs_f64();
        sim.message_frames
            .entry(id)
            .or_default()
            .override_fade_timestamp = now;
    }
    set_display_dirty_and_fire_callbacks(state, id)
}

fn reset_message_fade_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let msg_id = val_to_f64(stack_val(state, 2)) as i64;
    let changed = {
        let mut sim = borrow_state_mut(state)?;
        let now = sim.start_time.elapsed().as_secs_f64();
        let Some(data) = sim.message_frames.get_mut(&id) else {
            return Ok(0);
        };
        let Some(message) = data
            .messages
            .iter_mut()
            .rev()
            .find(|m| m.message_id == Some(msg_id))
        else {
            return Ok(0);
        };
        message.timestamp = now;
        true
    };
    if changed {
        set_display_dirty_and_fire_callbacks(state, id)?;
    }
    Ok(0)
}

fn get_font_string_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let msg_id = val_to_f64(stack_val(state, 2)) as i64;
    let (font_string_id, message) = match resolve_message_font_string(state, id, msg_id) {
        Some(pair) => pair,
        None => {
            state.push(Val::Nil);
            return Ok(1);
        }
    };
    update_message_font_string(state, font_string_id, &message);
    let val = frame_ref(state, font_string_id)?;
    state.push(val);
    Ok(1)
}

fn remove_messages_by_predicate(state: &mut LuaState) -> LuaResult<u32> {
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

fn transform_messages(state: &mut LuaState) -> LuaResult<u32> {
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

fn adjust_message_colors(state: &mut LuaState) -> LuaResult<u32> {
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

// ── Callback/display-dirty helpers ────────────────────────────────────────────

fn set_display_dirty_and_fire_callbacks(state: &mut LuaState, frame_id: u64) -> LuaResult<u32> {
    {
        let mut sim = borrow_state_mut(state)?;
        sim.message_frames
            .entry(frame_id)
            .or_default()
            .display_dirty = true;
    }
    call_display_refreshed_callbacks(state, frame_id)?;
    Ok(0)
}

fn call_display_refreshed_callbacks(state: &mut LuaState, frame_id: u64) -> LuaResult<()> {
    let fields = get_or_create_frame_fields(state, frame_id);
    let callbacks = get_or_create_display_refreshed_callbacks(state, fields);
    let Val::Table(cbs_ref) = callbacks else {
        return Ok(());
    };
    let len = {
        let arena = &state.gc.string_arena;
        state
            .gc
            .tables
            .get(cbs_ref)
            .map(|t| t.len(arena))
            .unwrap_or(0)
    };
    if len == 0 {
        return Ok(());
    }
    let self_val = frame_ref(state, frame_id)?;
    for i in 1..=len {
        let cb = state
            .gc
            .tables
            .get(cbs_ref)
            .map(|t| t.get_int(i as i64))
            .unwrap_or(Val::Nil);
        if matches!(cb, Val::Function(_)) {
            let _ = call_function_state(state, cb, &[self_val]);
        }
    }
    Ok(())
}

fn call_scroll_changed_callback(state: &mut LuaState, frame_id: u64, offset: i32) -> LuaResult<()> {
    let fields = get_or_create_frame_fields(state, frame_id);
    let cb = table_get(state, fields, ON_SCROLL_CHANGED_CB);
    if !matches!(cb, Val::Function(_)) {
        return Ok(());
    }
    let self_val = frame_ref(state, frame_id)?;
    let _ = call_function_state(state, cb, &[self_val, Val::Num(offset as f64)]);
    Ok(())
}

fn get_or_create_display_refreshed_callbacks(state: &mut LuaState, fields: Val) -> Val {
    let existing = table_get(state, fields, ON_DISPLAY_REFRESHED_CBS);
    if matches!(existing, Val::Table(_)) {
        return existing;
    }
    let new_table = Val::Table(state.gc.alloc_table(rilua::vm::table::Table::new()));
    table_set(state, fields, ON_DISPLAY_REFRESHED_CBS, new_table);
    new_table
}

// ── Message snapshot / replace helpers ───────────────────────────────────────

fn get_message_snapshot(state: &mut LuaState, frame_id: u64) -> Vec<Message> {
    let sim = match borrow_state(state) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    sim.message_frames
        .get(&frame_id)
        .map(|d| d.messages.clone())
        .unwrap_or_default()
}

fn replace_messages(state: &mut LuaState, frame_id: u64, messages: Vec<Message>) {
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
fn call_function_multi(state: &mut LuaState, func: Val, args: &[Val]) -> LuaResult<Vec<Val>> {
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

fn build_message_args(state: &mut LuaState, message: &Message) -> Vec<Val> {
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

fn message_from_transform_results(
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

fn color_from_results(original: &Message, results: Vec<Val>) -> Option<(f32, f32, f32)> {
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

fn val_to_opt_f32(val: Option<Val>) -> Option<f32> {
    match val {
        Some(Val::Num(n)) => Some(n as f32),
        _ => None,
    }
}

fn val_to_opt_f64(val: Option<Val>) -> Option<f64> {
    match val {
        Some(Val::Num(n)) => Some(n),
        _ => None,
    }
}

fn val_to_opt_i64(val: Option<Val>) -> Option<i64> {
    match val {
        Some(Val::Num(n)) => Some(n as i64),
        _ => None,
    }
}

fn is_truthy(val: Val) -> bool {
    !matches!(val, Val::Nil | Val::Bool(false))
}

// ── FontString-by-ID ─────────────────────────────────────────────────────────

fn resolve_message_font_string(
    state: &mut LuaState,
    frame_id: u64,
    message_id: i64,
) -> Option<(u64, Message)> {
    let (message, existing_fs_id) = {
        let sim = borrow_state(state).ok()?;
        let data = sim.message_frames.get(&frame_id)?;
        let message = data
            .messages
            .iter()
            .rev()
            .find(|m| m.message_id == Some(message_id))
            .cloned()?;
        let existing = data.message_font_strings.get(&message_id).copied();
        (message, existing)
    };

    let font_string_id = existing_fs_id.unwrap_or_else(|| {
        let mut sim = borrow_state_mut(state).unwrap();
        create_message_font_string(&mut sim, frame_id, message_id)
    });

    {
        let mut sim = borrow_state_mut(state).ok()?;
        sim.message_frames
            .get_mut(&frame_id)?
            .message_font_strings
            .insert(message_id, font_string_id);
    }
    Some((font_string_id, message))
}

fn create_message_font_string(
    sim: &mut crate::lua_api::SimState,
    parent_id: u64,
    message_id: i64,
) -> u64 {
    let mut font_string = Frame::new(WidgetType::FontString, None, Some(parent_id));
    font_string.visible = false;
    font_string.object_type_name = Some("FontString".to_string());
    font_string.parent_key = Some(format!("MessageID{message_id}"));
    let fs_id = font_string.id;
    sim.widgets.register(font_string);
    sim.widgets.add_child(parent_id, fs_id);
    let parent_props = sim
        .widgets
        .get(parent_id)
        .map(|p| (p.frame_strata, p.frame_level));
    if let Some((strata, level)) = parent_props
        && let Some(frame) = sim.widgets.get_mut_visual(fs_id)
    {
        frame.frame_strata = strata;
        frame.frame_level = level + 1;
    }
    fs_id
}

fn update_message_font_string(state: &mut LuaState, font_string_id: u64, message: &Message) {
    let mut sim = match borrow_state_mut(state) {
        Ok(s) => s,
        Err(_) => return,
    };
    let Some(frame) = sim.widgets.get_mut_visual(font_string_id) else {
        return;
    };
    frame.text = Some(message.text.clone());
    frame.text_stripped = Some(crate::dump::strip_wow_escapes(&message.text));
    frame.text_color = crate::widget::Color::new(message.r, message.g, message.b, message.a);
}

// ── Register ─────────────────────────────────────────────────────────────────

const METHODS: &[(&str, rilua::vm::closure::RustFn)] = &[
    // Add / backfill
    ("AddMessage", add_message),
    ("AddMsg", add_msg),
    ("_AddMessageSilent", add_message_silent),
    ("BackFillMessage", backfill_message),
    // Clear
    ("Clear", clear),
    ("ClearText", clear_text),
    // Count / max lines
    ("GetNumMessages", get_num_messages),
    ("SetMaxLines", set_max_lines),
    ("GetMaxLines", get_max_lines),
    // Fading
    ("SetFading", set_fading),
    ("GetFading", get_fading),
    ("SetTimeVisible", set_time_visible),
    ("GetTimeVisible", get_time_visible),
    ("SetFadeDuration", set_fade_duration),
    ("GetFadeDuration", get_fade_duration),
    ("SetFadePower", set_fade_power),
    ("GetFadePower", get_fade_power),
    // Insert mode
    ("SetInsertMode", set_insert_mode),
    ("GetInsertMode", get_insert_mode),
    // Misc
    ("SetTextCopyable", set_text_copyable),
    ("IsTextCopyable", is_text_copyable),
    ("HasMessageByID", has_message_by_id),
    ("GetMessageInfo", get_message_info),
    // Scroll
    ("ScrollUp", scroll_up),
    ("ScrollDown", scroll_down),
    ("PageUp", page_up),
    ("PageDown", page_down),
    ("ScrollToTop", scroll_to_top),
    ("ScrollToBottom", scroll_to_bottom),
    ("AtTop", at_top),
    ("AtBottom", at_bottom),
    ("GetMaxScrollRange", get_max_scroll_range),
    ("SetScrollOffset", set_scroll_offset),
    ("GetScrollOffset", get_scroll_offset),
    ("SetScrollAllowed", set_scroll_allowed),
    ("IsScrollAllowed", is_scroll_allowed),
    // Callbacks
    ("SetIndentedWordWrap", set_indented_word_wrap),
    ("GetIndentedWordWrap", get_indented_word_wrap),
    ("SetOnScrollChangedCallback", set_on_scroll_changed_callback),
    (
        "SetOnLineRightClickedCallback",
        set_on_line_right_clicked_callback,
    ),
    (
        "AddOnDisplayRefreshedCallback",
        add_on_display_refreshed_callback,
    ),
    ("SetOnTextCopiedCallback", set_on_text_copied_callback),
    ("MarkDisplayDirty", mark_display_dirty),
    ("ResetAllFadeTimes", reset_all_fade_times),
    ("ResetMessageFadeByID", reset_message_fade_by_id),
    ("GetFontStringByID", get_font_string_by_id),
    ("RemoveMessagesByPredicate", remove_messages_by_predicate),
    ("TransformMessages", transform_messages),
    ("AdjustMessageColors", adjust_message_colors),
];

pub fn register_message_frame(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    for (name, func) in METHODS {
        table_set_rust_fn(state, metatable, name, *func)?;
    }
    Ok(())
}
