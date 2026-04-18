//! Visibility, collapse-layout, and menu-open methods.

use super::helpers::{arg_bool, frame_id};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_ref};
use crate::lua_api::script_helpers::call_error_handler_state;
use crate::lua_api::script_helpers::get_script as get_rilua_script;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::collections::HashSet;

pub fn show(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    for frame_id in update_frame_visibility(state, id, true)? {
        fire_visibility_handler(state, frame_id, "OnShow");
    }
    Ok(0)
}

pub fn hide(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    for frame_id in update_frame_visibility(state, id, false)? {
        fire_visibility_handler(state, frame_id, "OnHide");
    }
    Ok(0)
}

pub fn set_shown(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let shown = arg_bool(state, 2);
    let handler_name = if shown { "OnShow" } else { "OnHide" };
    for frame_id in update_frame_visibility(state, id, shown)? {
        fire_visibility_handler(state, frame_id, handler_name);
    }
    Ok(0)
}

fn update_frame_visibility(state: &mut LuaState, id: u64, shown: bool) -> LuaResult<Vec<u64>> {
    let mut sim = borrow_state_mut(state)?;
    let subtree_ids = collect_subtree_ids(&sim, id);
    let previously_visible = visible_subtree_ids(&sim, &subtree_ids);
    sim.set_frame_visible(id, shown);
    let currently_visible = visible_subtree_ids(&sim, &subtree_ids);
    drop(sim);

    let transitioned = if shown {
        subtree_ids
            .into_iter()
            .filter(|frame_id| {
                currently_visible.contains(frame_id) && !previously_visible.contains(frame_id)
            })
            .collect()
    } else {
        subtree_ids
            .into_iter()
            .rev()
            .filter(|frame_id| {
                previously_visible.contains(frame_id) && !currently_visible.contains(frame_id)
            })
            .collect()
    };
    Ok(transitioned)
}

fn collect_subtree_ids(state: &crate::lua_api::SimState, root_id: u64) -> Vec<u64> {
    let mut ids = Vec::new();
    let mut stack = vec![root_id];
    while let Some(frame_id) = stack.pop() {
        ids.push(frame_id);
        if let Some(frame) = state.widgets.get(frame_id) {
            for child_id in frame.children.iter().rev().copied() {
                stack.push(child_id);
            }
        }
    }
    ids
}

fn visible_subtree_ids(state: &crate::lua_api::SimState, ids: &[u64]) -> HashSet<u64> {
    ids.iter()
        .copied()
        .filter(|frame_id| state.widgets.is_ancestor_visible(*frame_id))
        .collect()
}

fn fire_visibility_handler(state: &mut LuaState, frame_id: u64, handler_name: &str) {
    let Some(handler) = get_rilua_script(state, frame_id, handler_name) else {
        return;
    };
    let Ok(frame) = frame_ref(state, frame_id) else {
        return;
    };
    if let Err(error_msg) =
        crate::lua_api::script_helpers::protected_lua_pcall_state(state, handler, &[frame])
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
