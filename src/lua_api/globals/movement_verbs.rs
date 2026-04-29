//! Movement verbs that flip `SimState.player.movement.moving`.
//!
//! Each Start verb sets `movement.moving = true` and fires
//! `PLAYER_STARTED_MOVING`; each Stop verb sets `movement.moving = false`
//! and fires `PLAYER_STOPPED_MOVING`. Rotation verbs (TurnLeft / TurnRight)
//! do not affect `moving` in retail (rotating in place is not movement);
//! they no-op at this layer to satisfy the symbol lookup contract that
//! Blizzard_MovePad's XML KeyValues resolve at template-instantiation
//! time.
//!
//! Registered from `register_tail_globals` after `missing_surface`.

use crate::event::Event;
use crate::lua_api::methods::borrow_state_mut;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult};

fn push_event(state: &mut LuaState, name: &str) -> LuaResult<()> {
    borrow_state_mut(state)?.events.push(Event {
        name: name.to_string(),
        args: Vec::new(),
    });
    Ok(())
}

fn start_translation(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.player.movement.moving = true;
    push_event(state, "PLAYER_STARTED_MOVING")?;
    Ok(0)
}

fn stop_translation(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.player.movement.moving = false;
    push_event(state, "PLAYER_STOPPED_MOVING")?;
    Ok(0)
}

fn rotate_no_op(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

const TRANSLATION_START_VERBS: &[&str] = &[
    "MoveForwardStart",
    "MoveBackwardStart",
    "StrafeLeftStart",
    "StrafeRightStart",
    "JumpOrAscendStart",
];

const TRANSLATION_STOP_VERBS: &[&str] = &[
    "MoveForwardStop",
    "MoveBackwardStop",
    "StrafeLeftStop",
    "StrafeRightStop",
    "AscendStop",
];

const ROTATION_VERBS: &[&str] = &[
    "TurnLeftStart",
    "TurnLeftStop",
    "TurnRightStart",
    "TurnRightStop",
];

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    for &verb in TRANSLATION_START_VERBS {
        LuaApiMut::register_function(lua, verb, start_translation)?;
    }
    for &verb in TRANSLATION_STOP_VERBS {
        LuaApiMut::register_function(lua, verb, stop_translation)?;
    }
    for &verb in ROTATION_VERBS {
        LuaApiMut::register_function(lua, verb, rotate_no_op)?;
    }
    Ok(())
}
