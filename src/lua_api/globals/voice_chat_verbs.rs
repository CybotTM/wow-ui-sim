//! Voice chat verbs that drive `SimState.voice_chat` and fire
//! `VOICE_CHAT_*` events.
//!
//! Migrates 4 entries off `GLOBAL_NIL_STUBS`:
//!
//! - `VoiceChat_GetMicrophoneVolume()`  — returns current mic volume.
//! - `VoiceChat_SetMicrophoneVolume(v)` — set mic volume ∈ [0, 1], fires
//!                                         `VOICE_CHAT_MICROPHONE_VOLUME_CHANGED`.
//! - `VoiceChat_SetOutputVolume(v)`     — set output volume ∈ [0, 1],
//!                                         fires `VOICE_CHAT_OUTPUT_VOLUME_CHANGED`.
//! - `VoiceChatHeadsetModeCheck()`      — flag `headset_mode = true`,
//!                                         fires `VOICE_CHAT_HEADSET_MODE_CHANGED`.
//!
//! Registered from `register_tail_globals` after `missing_surface`.

use crate::event::Event;
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn push_event(state: &mut LuaState, name: &str) -> LuaResult<()> {
    borrow_state_mut(state)?.events.push(Event {
        name: name.to_string(),
        args: Vec::new(),
    });
    Ok(())
}

fn stack_f32(state: &mut LuaState, index: i32) -> Option<f32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as f32),
        _ => None,
    }
}

/// `VoiceChat_GetMicrophoneVolume()` — return `voice_chat.microphone_volume`.
fn get_microphone_volume(state: &mut LuaState) -> LuaResult<u32> {
    let volume = borrow_state_mut(state)?.voice_chat.microphone_volume;
    state.push(Val::Num(volume as f64));
    Ok(1)
}

/// `VoiceChat_SetMicrophoneVolume(v)` — clamped to [0, 1], fires mic event.
fn set_microphone_volume(state: &mut LuaState) -> LuaResult<u32> {
    let Some(v) = stack_f32(state, 1) else {
        return Ok(0);
    };
    borrow_state_mut(state)?.voice_chat.microphone_volume = v.clamp(0.0, 1.0);
    push_event(state, "VOICE_CHAT_MICROPHONE_VOLUME_CHANGED")?;
    Ok(0)
}

/// `VoiceChat_SetOutputVolume(v)` — clamped to [0, 1], fires output event.
fn set_output_volume(state: &mut LuaState) -> LuaResult<u32> {
    let Some(v) = stack_f32(state, 1) else {
        return Ok(0);
    };
    borrow_state_mut(state)?.voice_chat.output_volume = v.clamp(0.0, 1.0);
    push_event(state, "VOICE_CHAT_OUTPUT_VOLUME_CHANGED")?;
    Ok(0)
}

/// `VoiceChatHeadsetModeCheck()` — flag `headset_mode = true`, fire event.
fn headset_mode_check(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.voice_chat.headset_mode = true;
    push_event(state, "VOICE_CHAT_HEADSET_MODE_CHANGED")?;
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "VoiceChat_GetMicrophoneVolume", get_microphone_volume)?;
    LuaApiMut::register_function(lua, "VoiceChat_SetMicrophoneVolume", set_microphone_volume)?;
    LuaApiMut::register_function(lua, "VoiceChat_SetOutputVolume", set_output_volume)?;
    LuaApiMut::register_function(lua, "VoiceChatHeadsetModeCheck", headset_mode_check)?;
    Ok(())
}
