//! `EnumerateFrames` and `GetClickFrame` global functions.

use crate::lua_api::frame::{extract_frame_id, frame_ref};
use crate::lua_api::frame::get_sim_state;
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
        ref v @ Value::LightUserData(_) | ref v @ Value::UserData(_) => {
            extract_frame_id(v).unwrap_or(0)
        }
        Value::Nil => 0,
        _ => return Ok(Value::Nil),
    };

    match state.widgets.next_id_after(after_id) {
        Some(id) => frame_ref(lua, id),
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
        Some(id) => {
            drop(state);
            frame_ref(lua, id)
        }
        None => Ok(Value::Nil),
    }
}
