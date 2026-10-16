//! Explicit repeat-active slot input; no inference from casting or attacks.

use crate::lua_api::SimState;
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

/// Slot-owned simulator input, independent of assigned spell and current action.
pub fn is_active(sim: &SimState, slot: u32) -> bool {
    sim.action_repeat_active_slots.contains(&slot)
}

/// Supply state only; callers explicitly dispatch start/stop/state events.
pub(crate) fn set_active_admin(state: &mut LuaState) -> LuaResult<u32> {
    let slot = u32::from_stack(state, 1)?;
    if slot == 0 {
        return Err(runtime_error(
            "repeat-active input requires a positive action slot",
        ));
    }
    let Val::Bool(active) = stack_val(state, 2) else {
        return Err(runtime_error("repeat-active input requires a boolean"));
    };
    let mut sim = borrow_state_mut(state)?;
    if active {
        sim.action_repeat_active_slots.insert(slot);
    } else {
        sim.action_repeat_active_slots.remove(&slot);
    }
    Ok(0)
}
