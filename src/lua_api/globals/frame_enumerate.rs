//! `EnumerateFrames` and `GetClickFrame` global functions.

use crate::lua_api::frame::{frame_lud, get_sim_state, lud_to_id};
use mlua::{Lua, Result, Value};

/// Register `EnumerateFrames` and `GetClickFrame` globals.
pub fn register_frame_enumerate(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("EnumerateFrames", lua.create_function(enumerate_frames)?)?;
    g.set("GetClickFrame", lua.create_function(get_click_frame)?)?;
    Ok(())
}

/// EnumerateFrames(frame?) → next frame or nil.
///
/// Iterates all frames in creation order (ascending ID).
/// Call with nil to get the first frame, then with each returned frame to iterate.
/// Uses O(log N) binary search via `WidgetRegistry::next_id_after`.
fn enumerate_frames(lua: &Lua, arg: Value) -> Result<Value> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();

    let after_id: u64 = match &arg {
        Value::LightUserData(lud) => lud_to_id(*lud),
        Value::Nil => 0,
        _ => return Ok(Value::Nil),
    };

    match state.widgets.next_id_after(after_id) {
        Some(id) => Ok(frame_lud(id)),
        None => Ok(Value::Nil),
    }
}

/// GetClickFrame(name) → frame or nil.
///
/// Looks up a frame by its global name.
fn get_click_frame(lua: &Lua, name: String) -> Result<Value> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    match state.widgets.get_id_by_name(&name) {
        Some(id) => Ok(frame_lud(id)),
        None => Ok(Value::Nil),
    }
}
