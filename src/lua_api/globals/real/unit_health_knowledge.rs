//! Explicit Classic exact-health knowledge, separate from vitals and secrecy.
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn should_know_unit_health(state: &mut LuaState) -> LuaResult<u32> {
    let unit = String::from_stack(state, 1)?;
    let known = borrow_state(state)?
        .unit_health_knowledge
        .get(&unit)
        .copied()
        .unwrap_or(false);
    state.push(Val::Bool(known));
    Ok(1)
}

pub(crate) fn register(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "ShouldKnowUnitHealth", should_know_unit_health)?;
    Ok(())
}
