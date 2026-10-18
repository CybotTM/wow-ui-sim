//! Wrath's simulated signed gamma setting, independent of CVars and host display.
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetGamma", get_gamma)?;
    LuaApiMut::register_function(lua, "SetGamma", set_gamma)?;
    Ok(())
}

fn get_gamma(state: &mut LuaState) -> LuaResult<u32> {
    let gamma = borrow_state(state)?.legacy_gamma;
    state.push(Val::Num(gamma));
    Ok(1)
}

fn set_gamma(state: &mut LuaState) -> LuaResult<u32> {
    let gamma = f64::from_stack(state, 1)?;
    borrow_state_mut(state)?.legacy_gamma = gamma;
    Ok(0)
}
