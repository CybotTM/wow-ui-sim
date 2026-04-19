//! Mouse-input probe globals.
//!
//! Migrates 2 entries off `GLOBAL_ZERO_STUBS`:
//!
//! - `GetCursorPosition()` → `(x, y)` in unscaled screen pixels from
//!   `SimState.mouse_position`, defaulting to `(0, 0)` when the cursor
//!   is unknown.
//! - `GetMouseFocus()`     → FrameRef for `SimState.hovered_frame`,
//!   or `nil` when the cursor isn't over any widget.

use crate::lua_api::methods::{borrow_state, frame_ref};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn get_cursor_position(state: &mut LuaState) -> LuaResult<u32> {
    let (x, y) = {
        let sim = borrow_state(state)?;
        sim.mouse_position.unwrap_or((0.0, 0.0))
    };
    state.push(Val::Num(x as f64));
    state.push(Val::Num(y as f64));
    Ok(2)
}

fn get_mouse_focus(state: &mut LuaState) -> LuaResult<u32> {
    let hovered_id = { borrow_state(state)?.hovered_frame };
    match hovered_id {
        Some(id) => {
            let frame = frame_ref(state, id)?;
            state.push(frame);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetCursorPosition", get_cursor_position)?;
    LuaApiMut::register_function(lua, "GetMouseFocus", get_mouse_focus)?;
    Ok(())
}
