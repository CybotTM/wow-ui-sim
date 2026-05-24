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
use crate::lua_bridge::FromStack;
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

fn is_submerged(state: &mut LuaState) -> LuaResult<u32> {
    let submerged = borrow_state(state)?.player.movement.swimming;
    state.push(Val::Bool(submerged));
    Ok(1)
}

fn is_stealthed(state: &mut LuaState) -> LuaResult<u32> {
    let _ = borrow_state(state)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn is_advanced_flyable_area(state: &mut LuaState) -> LuaResult<u32> {
    let flyable = borrow_state(state)?.world.flyable_area;
    state.push(Val::Bool(flyable));
    Ok(1)
}

fn is_drivable_area(state: &mut LuaState) -> LuaResult<u32> {
    let _ = borrow_state(state)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn is_indoors(state: &mut LuaState) -> LuaResult<u32> {
    let _ = borrow_state(state)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn is_outdoors(state: &mut LuaState) -> LuaResult<u32> {
    let _ = borrow_state(state)?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn unit_position(state: &mut LuaState) -> LuaResult<u32> {
    let _ = Option::<String>::from_stack(state, 1)?;
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(4)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsPlayerMoving", is_player_moving)?;
    LuaApiMut::register_function(lua, "IsMounted", is_mounted)?;
    LuaApiMut::register_function(lua, "IsFlying", is_flying)?;
    LuaApiMut::register_function(lua, "IsFalling", is_falling)?;
    LuaApiMut::register_function(lua, "IsSwimming", is_swimming)?;
    LuaApiMut::register_function(lua, "IsSubmerged", is_submerged)?;
    LuaApiMut::register_function(lua, "IsStealthed", is_stealthed)?;
    LuaApiMut::register_function(lua, "IsAdvancedFlyableArea", is_advanced_flyable_area)?;
    LuaApiMut::register_function(lua, "IsDrivableArea", is_drivable_area)?;
    LuaApiMut::register_function(lua, "IsIndoors", is_indoors)?;
    LuaApiMut::register_function(lua, "IsOutdoors", is_outdoors)?;
    LuaApiMut::register_function(lua, "UnitPosition", unit_position)?;
    Ok(())
}
