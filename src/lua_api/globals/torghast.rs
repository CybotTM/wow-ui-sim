//! Torghast (Jailer's Tower) probe globals backed by `SimState.torghast`.
//!
//! Migrates 1 entry off `GLOBAL_FALSE_STUBS`:
//!
//! - `IsInJailersTower()` → true when `torghast.active`
//! - `IsOnGroundFloorInJailersTower()` → true when `torghast.active && torghast.floor == 1`

use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn is_in_jailers_tower(state: &mut LuaState) -> LuaResult<u32> {
    let result = borrow_state(state)?.torghast.active;
    state.push(Val::Bool(result));
    Ok(1)
}

fn is_on_ground_floor_in_jailers_tower(state: &mut LuaState) -> LuaResult<u32> {
    let result = {
        let sim = borrow_state(state)?;
        sim.torghast.active && sim.torghast.floor == 1
    };
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsInJailersTower", is_in_jailers_tower)?;
    LuaApiMut::register_function(
        lua,
        "IsOnGroundFloorInJailersTower",
        is_on_ground_floor_in_jailers_tower,
    )?;
    Ok(())
}
