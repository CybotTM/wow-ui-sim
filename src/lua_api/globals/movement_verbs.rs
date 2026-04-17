//! Movement verbs that flip `SimState.player.movement.moving`.
//!
//! Migrates 2 entries off `GLOBAL_NIL_STUBS`:
//!
//! - `MoveForwardStart()` — set `movement.moving = true`. Fires
//!                           `PLAYER_STARTED_MOVING`.
//! - `MoveForwardStop()`  — set `movement.moving = false`. Fires
//!                           `PLAYER_STOPPED_MOVING`.
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

/// `MoveForwardStart()` — flip `moving = true`, fire `PLAYER_STARTED_MOVING`.
/// Fires the event even when already moving (matches retail: each start
/// keypress emits the event).
fn move_forward_start(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.player.movement.moving = true;
    push_event(state, "PLAYER_STARTED_MOVING")?;
    Ok(0)
}

/// `MoveForwardStop()` — flip `moving = false`, fire `PLAYER_STOPPED_MOVING`.
fn move_forward_stop(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.player.movement.moving = false;
    push_event(state, "PLAYER_STOPPED_MOVING")?;
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "MoveForwardStart", move_forward_start)?;
    LuaApiMut::register_function(lua, "MoveForwardStop", move_forward_stop)?;
    Ok(())
}
