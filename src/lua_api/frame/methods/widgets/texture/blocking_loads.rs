//! Blocking loads methods.

use super::super::shared::val_to_bool;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn set_blocking_loads_requested(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let blocking = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.blocking_loads_requested = blocking;
    }
    Ok(0)
}

pub(super) fn is_blocking_load_requested(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let blocking = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.blocking_loads_requested)
        .unwrap_or(false);
    state.push(Val::Bool(blocking));
    Ok(1)
}
