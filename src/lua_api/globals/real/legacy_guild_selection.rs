//! Legacy guild roster selection over the existing guild member vector.
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetGuildRosterSelection", get_selection)?;
    LuaApiMut::register_function(lua, "SetGuildRosterSelection", set_selection)?;
    Ok(())
}
fn get_selection(state: &mut LuaState) -> LuaResult<u32> {
    let sim = borrow_state(state)?;
    let selection = if sim.guild_roster_selection <= sim.world.guild_members.len() {
        sim.guild_roster_selection
    } else {
        0
    };
    drop(sim);
    state.push(Val::Num(selection as f64));
    Ok(1)
}
fn set_selection(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    sim.guild_roster_selection = usize::try_from(index)
        .ok()
        .filter(|index| *index <= sim.world.guild_members.len())
        .unwrap_or(0);
    Ok(0)
}
