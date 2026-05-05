//! Highlight lock and desaturate-hierarchy methods.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use crate::widget::Frame;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn register(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, mt, "DesaturateHierarchy", desaturate_hierarchy)?;
    table_set_rust_fn_static(state, mt, "IsHighlightLocked", is_highlight_locked)?;
    table_set_rust_fn_static(state, mt, "LockHighlight", lock_highlight)?;
    table_set_rust_fn_static(
        state,
        mt,
        "IsIgnoringChildrenForBounds",
        is_ignoring_children_for_bounds,
    )?;
    table_set_rust_fn_static(state, mt, "SetHighlightLocked", set_highlight_locked)?;
    table_set_rust_fn_static(
        state,
        mt,
        "SetIgnoringChildrenForBounds",
        set_ignoring_children_for_bounds,
    )?;
    table_set_rust_fn_static(state, mt, "UnlockHighlight", unlock_highlight)?;
    Ok(())
}

pub fn desaturate_hierarchy(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let desaturation = f64::from_stack(state, 2)?;
    let exclude_root = Option::<bool>::from_stack(state, 3)?.unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    let ids = collect_descendants(&sim.widgets, id, exclude_root);
    let desaturated = desaturation > 0.0;
    for fid in ids {
        if let Some(f) = sim.widgets.get_mut_visual(fid) {
            f.desaturated = desaturated;
        }
    }
    Ok(0)
}

fn collect_descendants(
    widgets: &crate::widget::WidgetRegistry,
    root_id: u64,
    exclude_root: bool,
) -> Vec<u64> {
    let mut ids = Vec::new();
    let mut stack = vec![root_id];
    while let Some(fid) = stack.pop() {
        if !(exclude_root && fid == root_id) {
            ids.push(fid);
        }
        if let Some(f) = widgets.get(fid) {
            stack.extend(f.children.iter().rev().copied());
        }
    }
    ids
}

pub fn is_highlight_locked(state: &mut LuaState) -> LuaResult<u32> {
    push_frame_bool(state, |frame| frame.highlight_locked)
}

fn push_frame_bool(state: &mut LuaState, read: impl FnOnce(&Frame) -> bool) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = borrow_state(state)?
        .widgets
        .get(id)
        .map(read)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn lock_highlight(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    set_highlight_with_texture(state, id, true)
}

pub fn unlock_highlight(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    set_highlight_with_texture(state, id, false)
}

fn set_highlight_with_texture(state: &mut LuaState, id: u64, locked: bool) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.highlight_locked = locked;
    }
    if let Some(highlight_id) = sim
        .widgets
        .get(id)
        .and_then(|frame| frame.children_keys.get("HighlightTexture").copied())
    {
        sim.widgets.set_visible(highlight_id, locked);
    }
    Ok(0)
}

pub fn is_ignoring_children_for_bounds(state: &mut LuaState) -> LuaResult<u32> {
    push_frame_bool(state, |frame| frame.ignoring_children_for_bounds)
}

pub fn set_highlight_locked(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let locked = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.highlight_locked = locked;
    }
    Ok(0)
}

pub fn set_ignoring_children_for_bounds(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let ignore = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.ignoring_children_for_bounds = ignore;
    }
    Ok(0)
}
