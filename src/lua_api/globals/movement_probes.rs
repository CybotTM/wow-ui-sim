//! Movement probe globals reading from `PlayerState.movement`.
//!
//! Migrates 5 entries off generic stubs:
//!
//! - `IsPlayerMoving()` — `player.movement.moving`
//! - `IsMounted()`      — `player.movement.mounted`
//! - `IsFlying()`       — `player.movement.flying`
//! - `IsFalling()`      — `player.movement.falling`
//! - `IsSwimming()`     — `player.movement.swimming`

use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn is_player_moving(state: &mut LuaState) -> LuaResult<u32> {
    let moving = borrow_state(state)?.player.movement.moving;
    state.push(Val::Bool(moving));
    Ok(1)
}

fn is_mounted(state: &mut LuaState) -> LuaResult<u32> {
    let mounted = borrow_state(state)?.player.movement.mounted;
    state.push(Val::Bool(mounted));
    Ok(1)
}

fn is_flying(state: &mut LuaState) -> LuaResult<u32> {
    let flying = borrow_state(state)?.player.movement.flying;
    state.push(Val::Bool(flying));
    Ok(1)
}

fn is_falling(state: &mut LuaState) -> LuaResult<u32> {
    let falling = borrow_state(state)?.player.movement.falling;
    state.push(Val::Bool(falling));
    Ok(1)
}

fn is_swimming(state: &mut LuaState) -> LuaResult<u32> {
    let swimming = borrow_state(state)?.player.movement.swimming;
    state.push(Val::Bool(swimming));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsPlayerMoving", is_player_moving)?;
    LuaApiMut::register_function(lua, "IsMounted", is_mounted)?;
    LuaApiMut::register_function(lua, "IsFlying", is_flying)?;
    LuaApiMut::register_function(lua, "IsFalling", is_falling)?;
    LuaApiMut::register_function(lua, "IsSwimming", is_swimming)?;
    Ok(())
}
