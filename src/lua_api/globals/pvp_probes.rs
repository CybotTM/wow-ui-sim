//! PvP probe globals reading from `WorldState.pvp_*` and
//! `SimState.pvp_last_honor_gain` / `player.pvp_enabled`.
//!
//! Migrates 4 entries off `GLOBAL_FALSE_STUBS`:
//!
//! - `IsInActiveWorldPVP()` — true when `world.pvp_type` is one of the
//!                              active-combat tokens (`"combat"`,
//!                              `"hostile"`, `"arena"`).
//! - `GetPVPDesired()`      — `player.pvp_enabled`.
//! - `GetPVPLastHonorGain()` — `pvp_last_honor_gain` (i32).
//! - `IsSubZonePVP()`       — `world.is_sub_zone_pvp`.

use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn is_in_active_world_pvp(state: &mut LuaState) -> LuaResult<u32> {
    let active = {
        let st = borrow_state(state)?;
        matches!(st.world.pvp_type.as_str(), "combat" | "hostile" | "arena")
    };
    state.push(Val::Bool(active));
    Ok(1)
}

fn get_pvp_desired(state: &mut LuaState) -> LuaResult<u32> {
    let desired = borrow_state(state)?.player.pvp_enabled;
    state.push(Val::Bool(desired));
    Ok(1)
}

fn get_pvp_last_honor_gain(state: &mut LuaState) -> LuaResult<u32> {
    let honor = borrow_state(state)?.pvp_last_honor_gain;
    state.push(Val::Num(honor as f64));
    Ok(1)
}

fn is_sub_zone_pvp(state: &mut LuaState) -> LuaResult<u32> {
    let flag = borrow_state(state)?.world.is_sub_zone_pvp;
    state.push(Val::Bool(flag));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsInActiveWorldPVP", is_in_active_world_pvp)?;
    LuaApiMut::register_function(lua, "GetPVPDesired", get_pvp_desired)?;
    LuaApiMut::register_function(lua, "GetPVPLastHonorGain", get_pvp_last_honor_gain)?;
    LuaApiMut::register_function(lua, "IsSubZonePVP", is_sub_zone_pvp)?;
    Ok(())
}
