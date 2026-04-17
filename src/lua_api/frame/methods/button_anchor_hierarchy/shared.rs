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

pub(super) fn resolve_anchor_target_id(state: &mut LuaState, value: Val) -> Option<usize> {
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
    extract_frame_id(state, global).map(|id| id as usize)
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
            crate::widget::AnchorPoint::from_str(&point_name).ok_or_else(|| {
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
        if global != Val::Nil {
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
