//! Mouse click/motion and hyperlink propagation methods.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn register(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        mt,
        "CanPropagateMouseClicks",
        can_propagate_mouse_clicks,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "CanPropagateMouseMotion",
        can_propagate_mouse_motion,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "DoesHyperlinkPropagateToParent",
        does_hyperlink_propagate_to_parent,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "SetHyperlinkPropagateToParent",
        set_hyperlink_propagate_to_parent,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "SetPropagateMouseClicks",
        set_propagate_mouse_clicks,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "SetPropagateMouseMotion",
        set_propagate_mouse_motion,
    )?;
    Ok(())
}

pub fn can_propagate_mouse_clicks(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = borrow_state(state)?
        .widgets
        .get(id)
        .map(|f| f.propagate_mouse_clicks)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn can_propagate_mouse_motion(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = borrow_state(state)?
        .widgets
        .get(id)
        .map(|f| f.propagate_mouse_motion)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn does_hyperlink_propagate_to_parent(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = borrow_state(state)?
        .widgets
        .get(id)
        .map(|f| f.propagate_hyperlinks_to_parent)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn set_hyperlink_propagate_to_parent(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.propagate_hyperlinks_to_parent = value;
    }
    Ok(0)
}

pub fn set_propagate_mouse_clicks(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.propagate_mouse_clicks = value;
    }
    Ok(0)
}

pub fn set_propagate_mouse_motion(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.propagate_mouse_motion = value;
    }
    Ok(0)
}
