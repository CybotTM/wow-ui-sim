//! Chat window presentation + subscription verbs.
//!
//! Migrates chat-window entries off `GLOBAL_NIL_STUBS`:
//!
//! - `SetChatWindowAlpha(index, a)`           — alpha ∈ [0,1]
//! - `SetChatWindowSize(index, fontSize)`     — store chat font size
//! - `SetChatWindowColor(index, r, g, b)`     — RGB ∈ [0,1]
//! - `SetChatWindowLocked(index, locked?)`    — default true when arg omitted
//! - `SetChatWindowUninteractable(i, u?)`     — default true when arg omitted
//! - `AddChatWindowChannel(index, channel)`   — subscribe window to channel
//! - `AddChatWindowMessages(index, group)`    — subscribe window to group
//! - `RemoveChatWindowMessages(index, group)` — unsubscribe window from group
//! - `ChangeChatColor(channel, r, g, b)`      — per-chat-type color override
//! - `GetChatWindowChannels(index)`           — returns a flat list
//! - `GetChatWindowMessages(index)`           — returns a flat list
//!
//! Chat windows are keyed by 1-based chat-frame index (`ChatFrame1` → 1).
//! Windows are lazily created on first touch.
//!
//! Registered from `register_tail_globals` after `missing_surface`.

use crate::lua_api::methods::{borrow_state_mut, create_string, create_table};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &mut LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn stack_f32(state: &mut LuaState, index: i32) -> Option<f32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as f32),
        _ => None,
    }
}

fn required_string(state: &mut LuaState, index: i32) -> Option<String> {
    Option::<String>::from_stack(state, index)
        .ok()
        .flatten()
        .filter(|s| !s.is_empty())
}

fn stack_truthy(state: &mut LuaState, index: i32, default: bool) -> bool {
    match stack_val(state, index) {
        Val::Nil => default,
        Val::Bool(b) => b,
        Val::Num(n) => n != 0.0,
        _ => true,
    }
}

fn window_entry<'a>(
    state: &'a mut LuaState,
    index: i32,
) -> LuaResult<std::cell::RefMut<'a, crate::lua_api::SimState>> {
    let mut st = borrow_state_mut(state)?;
    st.chat_windows.entry(index).or_default();
    Ok(st)
}

/// `SetChatWindowAlpha(index, alpha)` — alpha clamped to [0, 1].
fn set_chat_window_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let alpha = stack_f32(state, 2).unwrap_or(1.0).clamp(0.0, 1.0);
    let mut st = window_entry(state, index)?;
    if let Some(w) = st.chat_windows.get_mut(&index) {
        w.alpha = alpha;
    }
    Ok(0)
}

/// `SetChatWindowSize(index, fontSize)` — store the chat frame font size.
fn set_chat_window_size(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let font_size = stack_f32(state, 2).unwrap_or(12.0).max(1.0);
    let mut st = window_entry(state, index)?;
    if let Some(w) = st.chat_windows.get_mut(&index) {
        w.font_size = font_size;
    }
    Ok(0)
}

/// `SetChatWindowColor(index, r, g, b)` — channel values clamped to [0, 1].
fn set_chat_window_color(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let r = stack_f32(state, 2).unwrap_or(1.0).clamp(0.0, 1.0);
    let g = stack_f32(state, 3).unwrap_or(1.0).clamp(0.0, 1.0);
    let b = stack_f32(state, 4).unwrap_or(1.0).clamp(0.0, 1.0);
    let mut st = window_entry(state, index)?;
    if let Some(w) = st.chat_windows.get_mut(&index) {
        w.r = r;
        w.g = g;
        w.b = b;
    }
    Ok(0)
}

/// `SetChatWindowLocked(index, locked)` — defaults to true when omitted
/// (matches WoW: calling with no arg treats the window as locked).
fn set_chat_window_locked(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let locked = stack_truthy(state, 2, true);
    let mut st = window_entry(state, index)?;
    if let Some(w) = st.chat_windows.get_mut(&index) {
        w.locked = locked;
    }
    Ok(0)
}

/// `SetChatWindowUninteractable(index, uninteractable)` — defaults to true.
fn set_chat_window_uninteractable(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_i32(state, 1) else {
        return Ok(0);
    };
    let uninteractable = stack_truthy(state, 2, true);
    let mut st = window_entry(state, index)?;
    if let Some(w) = st.chat_windows.get_mut(&index) {
        w.uninteractable = uninteractable;
    }
    Ok(0)
}

/// `AddChatWindowChannel(index, channel)` — subscribe window to channel.
/// Idempotent: re-adding a channel is a silent no-op.
fn add_chat_window_channel(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(index), Some(channel)) = (stack_i32(state, 1), required_string(state, 2)) else {
        return Ok(0);
    };
    let mut st = window_entry(state, index)?;
    if let Some(w) = st.chat_windows.get_mut(&index) {
        let is_new_channel = w.channel_names.insert(channel.clone());
        if is_new_channel {
            w.channels.push(channel);
        }
    }
    Ok(0)
}

/// `AddChatWindowMessages(index, group)` — subscribe window to message group.
/// Idempotent: re-adding a group is a silent no-op.
fn add_chat_window_messages(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(index), Some(group)) = (stack_i32(state, 1), required_string(state, 2)) else {
        return Ok(0);
    };
    let mut st = window_entry(state, index)?;
    if let Some(w) = st.chat_windows.get_mut(&index)
        && !w.messages.iter().any(|message| message == &group)
    {
        w.messages.push(group);
    }
    Ok(0)
}

/// `RemoveChatWindowMessages(index, group)` — unsubscribe window from group.
fn remove_chat_window_messages(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(index), Some(group)) = (stack_i32(state, 1), required_string(state, 2)) else {
        return Ok(0);
    };
    let mut st = window_entry(state, index)?;
    if let Some(w) = st.chat_windows.get_mut(&index) {
        w.messages.retain(|message| message != &group);
    }
    Ok(0)
}

/// `ChangeChatColor(channel, r, g, b)` — record per-chat-type color
/// override. Keyed by uppercase channel/type name.
fn change_chat_color(state: &mut LuaState) -> LuaResult<u32> {
    let Some(channel) = required_string(state, 1) else {
        return Ok(0);
    };
    let r = stack_f32(state, 2).unwrap_or(1.0).clamp(0.0, 1.0);
    let g = stack_f32(state, 3).unwrap_or(1.0).clamp(0.0, 1.0);
    let b = stack_f32(state, 4).unwrap_or(1.0).clamp(0.0, 1.0);
    let mut st = borrow_state_mut(state)?;
    st.chat_type_colors
        .insert(channel.to_uppercase(), (r, g, b));
    Ok(0)
}

fn push_string_list(state: &mut LuaState, items: Vec<String>) -> LuaResult<u32> {
    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        return Ok(0);
    };
    for (i, s) in items.into_iter().enumerate() {
        let value = create_string(state, &s);
        if let Some(t) = state.gc.tables.get_mut(table_ref) {
            let _ = t.raw_set(Val::Num((i + 1) as f64), value, &state.gc.string_arena);
        }
    }
    state.gc.barrier_back(table_ref);
    state.push(table);
    Ok(1)
}

/// `GetChatWindowChannels(index)` — return a flat list of channel names.
fn get_chat_window_channels(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_i32(state, 1) else {
        let empty = create_table(state);
        state.push(empty);
        return Ok(1);
    };
    let channels = {
        let st = borrow_state_mut(state)?;
        st.chat_windows
            .get(&index)
            .map(|w| w.channels.clone())
            .unwrap_or_default()
    };
    push_string_list(state, channels)
}

/// `GetChatWindowMessages(index)` — return a flat list of message types.
fn get_chat_window_messages(state: &mut LuaState) -> LuaResult<u32> {
    let Some(index) = stack_i32(state, 1) else {
        let empty = create_table(state);
        state.push(empty);
        return Ok(1);
    };
    let messages = {
        let st = borrow_state_mut(state)?;
        st.chat_windows
            .get(&index)
            .map(|w| w.messages.clone())
            .unwrap_or_default()
    };
    push_string_list(state, messages)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "SetChatWindowAlpha", set_chat_window_alpha)?;
    LuaApiMut::register_function(lua, "SetChatWindowSize", set_chat_window_size)?;
    LuaApiMut::register_function(lua, "SetChatWindowColor", set_chat_window_color)?;
    LuaApiMut::register_function(lua, "SetChatWindowLocked", set_chat_window_locked)?;
    LuaApiMut::register_function(
        lua,
        "SetChatWindowUninteractable",
        set_chat_window_uninteractable,
    )?;
    LuaApiMut::register_function(lua, "AddChatWindowChannel", add_chat_window_channel)?;
    LuaApiMut::register_function(lua, "AddChatWindowMessages", add_chat_window_messages)?;
    LuaApiMut::register_function(lua, "RemoveChatWindowMessages", remove_chat_window_messages)?;
    LuaApiMut::register_function(lua, "ChangeChatColor", change_chat_color)?;
    LuaApiMut::register_function(lua, "GetChatWindowChannels", get_chat_window_channels)?;
    LuaApiMut::register_function(lua, "GetChatWindowMessages", get_chat_window_messages)?;
    Ok(())
}
