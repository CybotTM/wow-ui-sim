//! Shared helper functions used across submodules.

use crate::lua_api::methods::{borrow_state, extract_frame_id, frame_ref, val_to_string};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

/// Extract an optional f32 number from the stack (accepts Num).
pub(super) fn opt_f32(state: &LuaState, index: i32) -> Option<f32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as f32),
        _ => None,
    }
}

/// Extract an optional String from the stack, returns None for non-string.
pub(super) fn opt_string(state: &LuaState, index: i32) -> Option<String> {
    match stack_val(state, index) {
        Val::Str(_) => String::from_stack(state, index).ok(),
        _ => None,
    }
}

pub(super) fn resolve_anchor_target_id(
    state: &mut LuaState,
    frame_id: u64,
    value: Val,
) -> Option<usize> {
    if let Some(id) = extract_frame_id(state, value) {
        return Some(id as usize);
    }

    let name = val_to_string(state, value)?;
    let key_ref = state.gc.intern_string(name.as_bytes());
    let global = state
        .gc
        .tables
        .get(state.global)
        .map(|table| table.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    if let Some(id) = extract_frame_id(state, global) {
        return Some(id as usize);
    }

    resolve_parent_key_target(state, frame_id, &name)
}

fn resolve_parent_key_target(state: &LuaState, frame_id: u64, name: &str) -> Option<usize> {
    let sim = borrow_state(state).ok()?;
    let parent_id = sim.widgets.get(frame_id)?.parent_id?;
    let parent = sim.widgets.get(parent_id)?;
    if let Some(parent_name) = parent.name.as_deref()
        && let Some(suffix) = name.strip_prefix(parent_name)
    {
        let suffix = suffix.strip_prefix('.').unwrap_or(suffix);
        if suffix.is_empty() {
            return Some(parent_id as usize);
        }
        if let Some(child_id) = parent.children_keys.get(suffix) {
            return Some(*child_id as usize);
        }
    }

    let mut current_id = parent_id;
    let trimmed = name
        .strip_prefix("$parent")
        .or_else(|| name.strip_prefix("$Parent"))
        .or_else(|| name.strip_prefix("$parentKey"))?;
    let path = trimmed.strip_prefix('.').unwrap_or(trimmed);
    if path.is_empty() {
        return Some(current_id as usize);
    }

    for segment in path.split('.') {
        if segment.is_empty() {
            continue;
        }
        let next_id = sim
            .widgets
            .get(current_id)
            .and_then(|frame| frame.children_keys.get(segment).copied())?;
        current_id = next_id;
    }
    Some(current_id as usize)
}

pub(super) fn resolve_relative_point_from_val(
    state: &mut LuaState,
    value: Val,
    default: crate::widget::AnchorPoint,
) -> LuaResult<crate::widget::AnchorPoint> {
    match value {
        Val::Nil => Ok(default),
        Val::Str(_) => {
            let point_name = val_to_string(state, value).unwrap_or_default();
            let normalized = point_name
                .split(['"', ',', ' '])
                .next()
                .unwrap_or(point_name.as_str());
            crate::widget::AnchorPoint::from_str(normalized).ok_or_else(|| {
                runtime_error(format!(
                    "Frame:SetPoint(): Unknown region point {point_name}"
                ))
            })
        }
        _ => Ok(default),
    }
}

pub(super) fn frame_global_or_ref(state: &mut LuaState, id: u64) -> LuaResult<Val> {
    let frame_name = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|frame| frame.name.clone())
    };
    if let Some(name) = frame_name {
        let key_ref = state.gc.intern_string(name.as_bytes());
        let global = state
            .gc
            .tables
            .get(state.global)
            .map(|table| table.get_str(key_ref, &state.gc.string_arena))
            .unwrap_or(Val::Nil);
        if extract_frame_id(state, global) == Some(id) {
            return Ok(global);
        }
    }
    frame_ref(state, id)
}

pub(super) fn bind_named_child_global(
    state: &mut LuaState,
    name: &str,
    child_id: u64,
) -> LuaResult<()> {
    let child_ref = frame_ref(state, child_id)?;
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    if let Some(globals) = state.gc.tables.get_mut(global) {
        let _ = globals.raw_set(Val::Str(key), child_ref, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    Ok(())
}
