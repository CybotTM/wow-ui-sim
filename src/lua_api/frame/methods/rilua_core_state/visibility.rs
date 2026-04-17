//! Visibility, collapse-layout, and menu-open methods.

use super::helpers::{arg_bool, frame_id};
use crate::lua_api::rilua_methods::{borrow_state, borrow_state_mut, frame_ref};
use crate::lua_api::rilua_script_helpers::call_error_handler_state;
use crate::lua_api::rilua_script_helpers::get_script as get_rilua_script;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn show(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let changed = set_frame_visible(state, id, true)?;
    if changed {
        fire_visibility_handler(state, id, "OnShow");
    }
    Ok(0)
}

pub fn hide(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let changed = set_frame_visible(state, id, false)?;
    if changed {
        fire_visibility_handler(state, id, "OnHide");
    }
    Ok(0)
}

pub fn set_shown(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let shown = arg_bool(state, 2);
    let changed = set_frame_visible(state, id, shown)?;
    if changed {
        let handler_name = if shown { "OnShow" } else { "OnHide" };
        fire_visibility_handler(state, id, handler_name);
    }
    Ok(0)
}

fn set_frame_visible(state: &mut LuaState, id: u64, shown: bool) -> LuaResult<bool> {
    let mut sim = borrow_state_mut(state)?;
    let was_visible = sim
        .widgets
        .get(id)
        .map(|frame| frame.visible)
        .unwrap_or(false);
    sim.set_frame_visible(id, shown);
    Ok(was_visible != shown)
}

fn fire_visibility_handler(state: &mut LuaState, frame_id: u64, handler_name: &str) {
    let Some(handler) = get_rilua_script(state, frame_id, handler_name) else {
        return;
    };
    let Ok(frame) = frame_ref(state, frame_id) else {
        return;
    };
    if let Err(error_msg) =
        crate::lua_api::rilua_script_helpers::protected_lua_pcall_state(state, handler, &[frame])
    {
        call_error_handler_state(state, &error_msg);
    }
}

pub fn is_visible(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.is_ancestor_visible(id);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_shown(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.get(id).map(|f| f.visible).unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn set_collapses_layout(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let val = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.collapses_layout = val;
    }
    Ok(0)
}

pub fn collapses_layout(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.collapses_layout)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_collapsed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = is_collapsed_impl(&sim, id);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

/// IsMenuOpen() — returns false (menus are never open in headless mode).
pub fn is_menu_open(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn is_collapsed_impl(state: &crate::lua_api::SimState, id: u64) -> bool {
    let frame = match state.widgets.get(id) {
        Some(f) => f,
        None => return false,
    };
    if !frame.collapses_layout {
        return false;
    }
    let mut visible = frame.visible;
    let mut cur_parent = frame.parent_id;
    while visible {
        match cur_parent.and_then(|pid| state.widgets.get(pid)) {
            Some(p) if p.visible => cur_parent = p.parent_id,
            Some(_) => {
                visible = false;
            }
            None => break,
        }
    }
    !visible
}
