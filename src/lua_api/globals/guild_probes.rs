//! Guild-state probe globals.
//!
//! Migrates 4 entries off `GLOBAL_FALSE_STUBS`:
//!
//! - `IsInGuild()`                 -> `world.guild_name.is_some()`
//! - `CanReplaceGuildMaster()`     -> `SimState.can_replace_guild_master`
//! - `GetAutoDeclineGuildInvites()` -> `SimState.auto_decline_guild_invites`
//! - `GetGuildRosterShowOffline()` -> `SimState.guild_roster_show_offline`

use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn is_in_guild(state: &mut LuaState) -> LuaResult<u32> {
    let b = borrow_state(state)?.world.guild_name.is_some();
    state.push(Val::Bool(b));
    Ok(1)
}

fn can_replace_guild_master(state: &mut LuaState) -> LuaResult<u32> {
    let b = borrow_state(state)?.can_replace_guild_master;
    state.push(Val::Bool(b));
    Ok(1)
}

fn get_auto_decline_guild_invites(state: &mut LuaState) -> LuaResult<u32> {
    let b = borrow_state(state)?.auto_decline_guild_invites;
    state.push(Val::Bool(b));
    Ok(1)
}

fn get_guild_roster_show_offline(state: &mut LuaState) -> LuaResult<u32> {
    let b = borrow_state(state)?.guild_roster_show_offline;
    state.push(Val::Bool(b));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsInGuild", is_in_guild)?;
    LuaApiMut::register_function(lua, "CanReplaceGuildMaster", can_replace_guild_master)?;
    LuaApiMut::register_function(
        lua,
        "GetAutoDeclineGuildInvites",
        get_auto_decline_guild_invites,
    )?;
    LuaApiMut::register_function(
        lua,
        "GetGuildRosterShowOffline",
        get_guild_roster_show_offline,
    )?;
    Ok(())
}
