//! Rilua A_Admin handlers — Movement.
//!
//! Extracted from rilua_admin_extras.rs per the 750-line file cap and to keep
//! each sub-module focused on a single concern. The parent entry
//! point in admin.rs imports these as pub(super) and weaves
//! them into the A_Admin TableBuilder chain.

use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

// ── Movement ──────────────────────────────────────────────────────────────────

pub(super) fn set_moving(state: &mut LuaState) -> LuaResult<u32> {
    let v = bool::from_stack(state, 1)?;
    borrow_state_mut(state)?.player.movement.moving = v;
    Ok(0)
}

pub(super) fn set_mounted(state: &mut LuaState) -> LuaResult<u32> {
    let v = bool::from_stack(state, 1)?;
    borrow_state_mut(state)?.player.movement.mounted = v;
    Ok(0)
}

pub(super) fn set_flying(state: &mut LuaState) -> LuaResult<u32> {
    let v = bool::from_stack(state, 1)?;
    borrow_state_mut(state)?.player.movement.flying = v;
    Ok(0)
}

pub(super) fn set_falling(state: &mut LuaState) -> LuaResult<u32> {
    let v = bool::from_stack(state, 1)?;
    borrow_state_mut(state)?.player.movement.falling = v;
    Ok(0)
}

pub(super) fn set_swimming(state: &mut LuaState) -> LuaResult<u32> {
    let v = bool::from_stack(state, 1)?;
    borrow_state_mut(state)?.player.movement.swimming = v;
    Ok(0)
}
