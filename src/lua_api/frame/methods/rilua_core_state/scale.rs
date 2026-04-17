//! Scale methods: GetScale, GetEffectiveScale, SetScale,
//! SetIgnoreParentScale, GetIgnoreParentScale, IsIgnoringParentScale.

use super::helpers::{arg_bool, frame_id};
use crate::lua_api::rilua_methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

pub fn get_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.get(id).map(|f| f.scale).unwrap_or(1.0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn get_effective_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.effective_scale)
        .unwrap_or(1.0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn set_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let scale = match stack_val(state, 2) {
        Val::Num(n) => n as f32,
        _ => return Ok(0),
    };
    if scale <= 0.0 {
        return Err(runtime_error("Frame:SetScale(): Scale must be > 0"));
    }
    let mut sim = borrow_state_mut(state)?;
    let changed = sim
        .widgets
        .get(id)
        .map(|f| f.scale != scale)
        .unwrap_or(false);
    if !changed {
        return Ok(0);
    }
    let parent_eff_scale = sim
        .widgets
        .get(id)
        .and_then(|f| f.parent_id)
        .and_then(|pid| sim.widgets.get(pid))
        .map(|p| p.effective_scale)
        .unwrap_or(1.0);
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.scale = scale;
    }
    sim.widgets.propagate_effective_scale(id, parent_eff_scale);
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

pub fn set_ignore_parent_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    let changed = sim
        .widgets
        .get(id)
        .map(|f| f.ignore_parent_scale != ignore)
        .unwrap_or(false);
    if !changed {
        return Ok(0);
    }
    let parent_eff_scale = sim
        .widgets
        .get(id)
        .and_then(|f| f.parent_id)
        .and_then(|pid| sim.widgets.get(pid))
        .map(|p| p.effective_scale)
        .unwrap_or(1.0);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.ignore_parent_scale = ignore;
    }
    sim.widgets.propagate_effective_scale(id, parent_eff_scale);
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

pub fn get_ignore_parent_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.ignore_parent_scale)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_ignoring_parent_scale(state: &mut LuaState) -> LuaResult<u32> {
    get_ignore_parent_scale(state)
}
