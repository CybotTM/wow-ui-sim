//! Mouse and keyboard input-enable methods.

use super::helpers::{arg_bool, frame_id};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, val_to_string};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn enable_mouse(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let enable = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.mouse_enabled = enable;
    }
    Ok(0)
}

pub fn is_mouse_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.mouse_enabled)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn enable_mouse_wheel(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let enable = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.mouse_wheel_enabled = enable;
    }
    Ok(0)
}

pub fn is_mouse_wheel_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.mouse_wheel_enabled)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn enable_keyboard(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let enable = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut(id) {
        f.keyboard_enabled = enable;
    }
    Ok(0)
}

pub fn is_keyboard_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.keyboard_enabled)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn register_for_mouse(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let buttons = collect_mouse_registration_args(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.registered_mouse_buttons = buttons;
    }
    Ok(0)
}

fn collect_mouse_registration_args(
    state: &mut LuaState,
    start: i32,
) -> std::collections::HashSet<String> {
    let mut buttons = std::collections::HashSet::new();
    let mut index = start;
    loop {
        let value = stack_val(state, index);
        if value == Val::Nil {
            break;
        }
        if let Some(button) = val_to_string(state, value) {
            buttons.insert(button);
        }
        index += 1;
    }
    buttons
}

pub fn enable_mouse_motion(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let enable = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.mouse_motion_enabled = enable;
    }
    Ok(0)
}

pub fn is_mouse_motion_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.mouse_motion_enabled)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn set_mouse_motion_enabled(state: &mut LuaState) -> LuaResult<u32> {
    enable_mouse_motion(state)
}

pub fn set_mouse_click_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let enable = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.mouse_enabled = enable;
    }
    Ok(0)
}

pub fn is_mouse_click_enabled(state: &mut LuaState) -> LuaResult<u32> {
    is_mouse_enabled(state)
}
