//! Event query and dispatch globals.

use crate::lua_api::frame::{frame_lud, get_sim_state};
use mlua::{Lua, MultiValue, Result, Value};

pub fn register(lua: &Lua) -> Result<()> {
    register_get_frames_registered(lua)?;
    register_send_system_message(lua)?;
    Ok(())
}

/// `GetFramesRegisteredForEvent(event)` - returns all frames registered for an event.
fn register_get_frames_registered(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "GetFramesRegisteredForEvent",
        lua.create_function(|lua, event: String| {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            let values: Vec<Value> = state
                .widgets
                .get_event_listeners(&event)
                .into_iter()
                .map(frame_lud)
                .collect();
            Ok(MultiValue::from_vec(values))
        })?,
    )
}

/// `SendSystemMessage(msg)` - fires CHAT_MSG_SYSTEM event with the given message.
fn register_send_system_message(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "SendSystemMessage",
        lua.create_function(|lua, msg: String| {
            let fire: mlua::Function = lua.globals().get("FireEvent")?;
            fire.call::<()>((
                "CHAT_MSG_SYSTEM",
                msg,
                "",  // sender (empty for system)
            ))
        })?,
    )
}
