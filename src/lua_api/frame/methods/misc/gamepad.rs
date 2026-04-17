//! Gamepad button/stick enable methods.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn register(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, mt, "EnableGamePadButton", enable_game_pad_button)?;
    table_set_rust_fn(state, mt, "EnableGamePadStick", enable_game_pad_stick)?;
    table_set_rust_fn(
        state,
        mt,
        "IsGamePadButtonEnabled",
        is_game_pad_button_enabled,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "IsGamePadStickEnabled",
        is_game_pad_stick_enabled,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "ShouldButtonPassThrough",
        should_button_pass_through,
    )?;
    Ok(())
}

pub fn enable_game_pad_button(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.gamepad_button_enabled = enabled;
    }
    Ok(0)
}

pub fn enable_game_pad_stick(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.gamepad_stick_enabled = enabled;
    }
    Ok(0)
}

pub fn is_game_pad_button_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = borrow_state(state)?
        .widgets
        .get(id)
        .map(|f| f.gamepad_button_enabled)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn is_game_pad_stick_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = borrow_state(state)?
        .widgets
        .get(id)
        .map(|f| f.gamepad_stick_enabled)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn should_button_pass_through(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let button = String::from_stack(state, 2)?;
    let normalized = button.to_ascii_lowercase();
    let val = borrow_state(state)?
        .widgets
        .get(id)
        .map(|f| f.pass_through_buttons.contains(&normalized))
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}
