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
use crate::lua_api::methods::{create_table, table_set_num};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn get_cursor_position(state: &mut LuaState) -> LuaResult<u32> {
    let cursor_position = {
        let sim = borrow_state(state)?;
        sim.mouse_position
            .map(|(x, renderer_y)| (x, sim.screen_height - renderer_y))
            .unwrap_or((0.0, 0.0))
    };
    let (x, y) = cursor_position;
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

fn get_mouse_foci(state: &mut LuaState) -> LuaResult<u32> {
    let hovered_id = { borrow_state(state)?.hovered_frame };
    let table = create_table(state);
    if let (Some(id), Val::Table(table_ref)) = (hovered_id, table) {
        let frame = frame_ref(state, id)?;
        table_set_num(state, table_ref, 1.0, frame);
        state.push(Val::Table(table_ref));
    } else {
        state.push(table);
    }
    Ok(1)
}

fn is_mouse_button_down(state: &mut LuaState) -> LuaResult<u32> {
    let button = Option::<String>::from_stack(state, 1)?;
    let down = borrow_state(state)?
        .mouse_buttons
        .is_down(button.as_deref());
    state.push(Val::Bool(down));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetCursorPosition", get_cursor_position)?;
    LuaApiMut::register_function(lua, "GetMouseFocus", get_mouse_focus)?;
    LuaApiMut::register_function(lua, "GetMouseFoci", get_mouse_foci)?;
    LuaApiMut::register_function(lua, "IsMouseButtonDown", is_mouse_button_down)?;
    Ok(())
}
