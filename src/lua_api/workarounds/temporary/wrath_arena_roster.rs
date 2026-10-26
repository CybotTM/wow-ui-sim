//! Native 12340 no-team path. Retire when arena-team membership/query state exists.
//! No team slots are modeled: native silently returns without query or event.
use crate::lua_api::globals::real::wrath_spellbook::numeric_argument;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, runtime_error};

pub fn register(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "ArenaTeamRoster", arena_team_roster)
}

fn arena_team_roster(state: &mut LuaState) -> LuaResult<u32> {
    if numeric_argument(state, 1).is_none() {
        return Err(runtime_error("Usage: ArenaTeamRoster(team)"));
    }
    Ok(0)
}
