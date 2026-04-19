//! Frame size methods: GetWidth, GetHeight, GetSize, SetSize, SetWidth, SetHeight.

use super::helpers::{
    apply_explicit_height, apply_explicit_size, apply_explicit_width, clear_auto_width_flag,
    current_explicit_size_state, frame_id, frame_size, opt_f32,
};
use crate::lua_api::frame::methods::methods_helpers::{
    can_change_protected_state_for, emit_addon_action_blocked,
};
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn get_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = bool::from_stack(state, 2).ok().unwrap_or(false);
    let (width, _) = frame_size(state, id, ignore)?;
    state.push(Val::Num(width as f64));
    Ok(1)
}

pub fn get_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = bool::from_stack(state, 2).ok().unwrap_or(false);
    let (_, height) = frame_size(state, id, ignore)?;
    state.push(Val::Num(height as f64));
    Ok(1)
}

pub fn get_size(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = bool::from_stack(state, 2).ok().unwrap_or(false);
    let (width, height) = frame_size(state, id, ignore)?;
    state.push(Val::Num(width as f64));
    state.push(Val::Num(height as f64));
    Ok(2)
}

pub fn set_size(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let width = opt_f32(state, 2);
    let height = opt_f32(state, 3);
    if !can_change_protected_state_for(state, id) {
        emit_addon_action_blocked(state, id, "SetSize");
        return Ok(0);
    }
    let mut sim = borrow_state_mut(state)?;
    let Some(current) = current_explicit_size_state(&sim, id) else {
        return Ok(0);
    };

    let size_changed = current.width != width || current.height != height;
    if !size_changed {
        if current.width_is_text_auto {
            clear_auto_width_flag(&mut sim, id);
        }
        return Ok(0);
    }

    apply_explicit_size(&mut sim, id, width, height);
    Ok(0)
}

pub fn set_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let width = opt_f32(state, 2);
    if !can_change_protected_state_for(state, id) {
        emit_addon_action_blocked(state, id, "SetWidth");
        return Ok(0);
    }
    let mut sim = borrow_state_mut(state)?;
    let Some(current) = current_explicit_size_state(&sim, id) else {
        return Ok(0);
    };

    if current.width == width {
        if current.width_is_text_auto {
            clear_auto_width_flag(&mut sim, id);
        }
        return Ok(0);
    }

    apply_explicit_width(&mut sim, id, width);
    Ok(0)
}

pub fn set_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let height = opt_f32(state, 2);
    if !can_change_protected_state_for(state, id) {
        emit_addon_action_blocked(state, id, "SetHeight");
        return Ok(0);
    }
    let mut sim = borrow_state_mut(state)?;
    let Some(current_height) = sim.widgets.get(id).map(|frame| frame.height) else {
        return Ok(0);
    };

    if current_height == height {
        return Ok(0);
    }

    apply_explicit_height(&mut sim, id, height);
    Ok(0)
}
