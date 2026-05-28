//! Voice chat state probes backed by `SimState.voice_chat`.
//!
//! Migrates 6 entries off `GLOBAL_FALSE_STUBS`:
//!
//! - `IsUsingVoiceChat()`        -> `voice_chat.using`
//! - `IsVoiceEnabled()`          -> `voice_chat.enabled`
//! - `VoiceChat_IsConnecting()`  -> `voice_chat.connecting`
//! - `VoiceChat_IsMuted()`       -> `voice_chat.muted`
//! - `VoiceChat_IsDeafened()`    -> `voice_chat.deafened`
//! - `VoiceChat_IsTalking()`     -> `voice_chat.talking`

use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

macro_rules! define_voice_probe {
    ($fn_name:ident, $field:ident) => {
        fn $fn_name(state: &mut LuaState) -> LuaResult<u32> {
            let v = borrow_state(state)?.voice_chat.$field;
            state.push(Val::Bool(v));
            Ok(1)
        }
    };
}

define_voice_probe!(is_using_voice_chat, using);
define_voice_probe!(is_voice_enabled, enabled);
define_voice_probe!(voice_chat_is_connecting, connecting);
define_voice_probe!(voice_chat_is_muted, muted);
define_voice_probe!(voice_chat_is_deafened, deafened);
define_voice_probe!(voice_chat_is_talking, talking);

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsUsingVoiceChat", is_using_voice_chat)?;
    LuaApiMut::register_function(lua, "IsVoiceEnabled", is_voice_enabled)?;
    LuaApiMut::register_function(lua, "VoiceChat_IsConnecting", voice_chat_is_connecting)?;
    LuaApiMut::register_function(lua, "VoiceChat_IsMuted", voice_chat_is_muted)?;
    LuaApiMut::register_function(lua, "VoiceChat_IsDeafened", voice_chat_is_deafened)?;
    LuaApiMut::register_function(lua, "VoiceChat_IsTalking", voice_chat_is_talking)?;
    Ok(())
}
