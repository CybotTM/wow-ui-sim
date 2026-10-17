//! Wrath per-unit/channel-mode voice presentation inputs.
//! Existing voice snapshots have no unit-to-member mapping with both flags.
//! Missing entries mean false/false; no voice transport or native-parity claim.

use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

#[derive(Debug, Clone, Copy, Default)]
pub struct LegacyUnitVoice {
    pub voice_enabled: bool,
    pub muted: bool,
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetVoiceStatus", get_voice_status)?;
    LuaApiMut::register_function(lua, "GetMuteStatus", get_mute_status)?;
    Ok(())
}

fn read_legacy_voice(state: &LuaState) -> LuaResult<LegacyUnitVoice> {
    let unit = String::from_stack(state, 1)?;
    let mode = String::from_stack(state, 2)?;
    let voice = borrow_state(state)?
        .legacy_unit_voice
        .get(&(unit, mode))
        .copied()
        .unwrap_or_default();
    Ok(voice)
}

fn get_voice_status(state: &mut LuaState) -> LuaResult<u32> {
    let voice = read_legacy_voice(state)?;
    state.push(Val::Bool(voice.voice_enabled));
    Ok(1)
}

fn get_mute_status(state: &mut LuaState) -> LuaResult<u32> {
    let voice = read_legacy_voice(state)?;
    state.push(Val::Bool(voice.muted));
    Ok(1)
}
