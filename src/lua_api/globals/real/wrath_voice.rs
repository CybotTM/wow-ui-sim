//! Wrath per-unit/channel-mode voice presentation and named speaking queries.
//! Existing voice snapshots have no unit-to-member mapping with both flags.
//! Missing entries mean false/false; no voice transport or native-parity claim.

use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

#[derive(Debug, Clone, Copy, Default)]
pub struct LegacyUnitVoice {
    pub voice_enabled: bool,
    pub muted: bool,
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetVoiceStatus", get_voice_status)?;
    LuaApiMut::register_function(lua, "GetMuteStatus", get_mute_status)?;
    LuaApiMut::register_function(lua, "UnitIsTalking", unit_is_talking)?;
    Ok(())
}

fn unit_is_talking(state: &mut LuaState) -> LuaResult<u32> {
    let Val::Str(reference) = Val::from_stack(state, 1)? else {
        return Err(runtime_error(
            "UnitIsTalking expects a character name string",
        ));
    };
    let talking = {
        let name = state
            .gc
            .string_arena
            .get(reference)
            .ok_or_else(|| runtime_error("UnitIsTalking character name has been collected"))?;
        borrow_state(state)?
            .voice_chat
            .channels
            .iter()
            .flat_map(|channel| &channel.members)
            .any(|member| member.is_active_speaker && member.name.as_bytes() == name.data())
    };
    state.push(Val::Bool(talking));
    Ok(1)
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
