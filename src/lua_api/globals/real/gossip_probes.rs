//! Gossip probe globals backed by `SimState.gossip`.
//!
//! Migrates 3 entries off `GLOBAL_ZERO_STUBS`:
//!
//! - `GetGossipNumOptions()`         → `gossip.num_options`
//! - `GetGossipNumAvailableQuests()` → `gossip.num_available_quests`
//! - `GetGossipNumActiveQuests()`    → `gossip.num_active_quests`
//!
//! Each returns 0 while no dialog is open. Dialog open/close is
//! signalled via the `GOSSIP_SHOW` / `GOSSIP_CLOSED` events — tests
//! can seed the substate and fire the event directly through
//! `WowLuaEnv::fire_event`.

use crate::lua_api::methods::borrow_state;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn push_count(state: &mut LuaState, n: i32) -> LuaResult<u32> {
    state.push(Val::Num(n as f64));
    Ok(1)
}

fn get_gossip_num_options(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.gossip.num_options;
    push_count(state, n)
}

fn get_gossip_num_available_quests(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.gossip.num_available_quests;
    push_count(state, n)
}

fn get_gossip_num_active_quests(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.gossip.num_active_quests;
    push_count(state, n)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetGossipNumOptions", get_gossip_num_options)?;
    LuaApiMut::register_function(
        lua,
        "GetGossipNumAvailableQuests",
        get_gossip_num_available_quests,
    )?;
    LuaApiMut::register_function(
        lua,
        "GetGossipNumActiveQuests",
        get_gossip_num_active_quests,
    )?;
    Ok(())
}
