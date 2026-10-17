//! State-backed loot-method probe + refresh globals.
//!
//! Migrates 3 entries off `GLOBAL_ZERO_STUBS`:
//!
//! - `GetLootMethod()`             → `(method, partyMasterIndex,
//!   raidMasterIndex)` from `SimState.loot_method`.
//! - `GetMasterLooterThreshold()`  → `loot_method.threshold` (item
//!   quality enum, 0..=4).
//! - `RequestPartyLootMethod()`    → fires `PARTY_LOOT_METHOD_CHANGED`
//!   so listeners re-poll on demand.

use crate::event::Event;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

/// `GetLootMethod()` — retail returns `(method, partyMasterIndex,
/// raidMasterIndex)`. Master-looter indices stay 0 when `method` is
/// anything other than `"master"`.
fn get_loot_method(state: &mut LuaState) -> LuaResult<u32> {
    let (method, party_idx, raid_idx) = {
        let sim = borrow_state(state)?;
        (
            sim.loot_method.method.clone(),
            sim.loot_method.party_master_index,
            sim.loot_method.raid_master_index,
        )
    };
    let method_val = create_string(state, &method);
    state.push(method_val);
    state.push(Val::Num(party_idx as f64));
    state.push(Val::Num(raid_idx as f64));
    Ok(3)
}

/// `GetMasterLooterThreshold()` — retail returns the item quality
/// cutoff (enum value 0..=4).
fn get_master_looter_threshold(state: &mut LuaState) -> LuaResult<u32> {
    let threshold = borrow_state(state)?.loot_method.threshold;
    state.push(Val::Num(threshold as f64));
    Ok(1)
}

/// Wrath loot opt-out selection; unsupplied state is false (not opted out).
#[cfg(feature = "client-wrath")]
fn get_opt_out_of_loot(state: &mut LuaState) -> LuaResult<u32> {
    let opt_out = borrow_state(state)?.loot_method.opt_out;
    state.push(Val::Bool(opt_out));
    Ok(1)
}

/// `RequestPartyLootMethod()` — retail asks the server to re-send the
/// party's current loot method, which fires `PARTY_LOOT_METHOD_CHANGED`.
/// The sim already holds authoritative state, so we just replay the
/// event for listeners.
fn request_party_loot_method(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.events.push(Event {
        name: "PARTY_LOOT_METHOD_CHANGED".to_string(),
        args: Vec::new(),
    });
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetLootMethod", get_loot_method)?;
    LuaApiMut::register_function(lua, "GetMasterLooterThreshold", get_master_looter_threshold)?;
    #[cfg(feature = "client-wrath")]
    LuaApiMut::register_function(lua, "GetLootThreshold", get_master_looter_threshold)?;
    #[cfg(feature = "client-wrath")]
    LuaApiMut::register_function(lua, "GetOptOutOfLoot", get_opt_out_of_loot)?;
    LuaApiMut::register_function(lua, "RequestPartyLootMethod", request_party_loot_method)?;
    Ok(())
}
