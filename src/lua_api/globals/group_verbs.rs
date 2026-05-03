//! Group/party verbs that mutate `SimState.party_members` /
//! `party_group_active` and dispatch `GROUP_ROSTER_UPDATE` /
//! `READY_CHECK` events.
//!
//! Migrates 7 entries off `GLOBAL_NIL_STUBS` and adds one new global
//! (`InviteToGroup` was unregistered):
//!
//! - `InviteToGroup(name)`     — appends a synthesized member, sets
//!                                `party_group_active`, fires ROSTER.
//! - `AcceptGroup()`           — sets `party_group_active`, fires ROSTER.
//! - `DeclineGroup()`          — silent no-op (dialog UI decision only).
//! - `LeaveParty()`            — clears members + `party_group_active`,
//!                                fires ROSTER.
//! - `RemoveFromParty(name)`   — remove by name, fires ROSTER.
//! - `UninviteUnit(unit)`      — remove by unit token (partyN or name),
//!                                fires ROSTER.
//! - `KickUnit(unit)`          — alias of `UninviteUnit`.
//! - `ReadyCheck()`            — fires `READY_CHECK`.
//!
//! Registered from `register_tail_globals` after `missing_surface`.

use crate::event::Event;
use crate::lua_api::game_data::PartyMember;
use crate::lua_api::globals::unit_api::parse_party_index;
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult};

const DEFAULT_INVITE_HEALTH: i32 = 100_000;
const DEFAULT_INVITE_POWER: i32 = 10_000;

pub(crate) fn push_event(state: &mut LuaState, name: &str) -> LuaResult<()> {
    borrow_state_mut(state)?.events.push(Event {
        name: name.to_string(),
        args: Vec::new(),
    });
    Ok(())
}

fn required_string(state: &mut LuaState, index: i32) -> Option<String> {
    Option::<String>::from_stack(state, index)
        .ok()
        .flatten()
        .filter(|s| !s.is_empty())
}

fn synthesize_party_member(name: String) -> PartyMember {
    PartyMember {
        name,
        class_index: 1,
        level: 80,
        health: DEFAULT_INVITE_HEALTH,
        health_max: DEFAULT_INVITE_HEALTH,
        power: DEFAULT_INVITE_POWER,
        power_max: DEFAULT_INVITE_POWER,
        power_type: 0,
        power_type_name: "MANA".to_string(),
        is_leader: false,
        dead_since: None,
        buffs: vec![],
        debuffs: vec![],
    }
}

/// `InviteToGroup(name)` — append a synthesized party member, flag the
/// group active. Silent no-op on empty / non-string input.
fn invite_to_group(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = required_string(state, 1) else {
        return Ok(0);
    };
    {
        let mut st = borrow_state_mut(state)?;
        st.party_members.push(synthesize_party_member(name));
        st.party_group_active = true;
    }
    push_event(state, "GROUP_ROSTER_UPDATE")?;
    Ok(0)
}

/// `AcceptGroup()` — flag the group active. Keeps existing roster (if any).
fn accept_group(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.party_group_active = true;
    push_event(state, "GROUP_ROSTER_UPDATE")?;
    Ok(0)
}

/// `DeclineGroup()` — silent no-op (retail just dismisses the popup).
fn decline_group(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// Clear roster and group metadata when the player leaves the group.
pub(crate) fn clear_party_roster(state: &mut LuaState) -> LuaResult<()> {
    let mut st = borrow_state_mut(state)?;
    st.party_members.clear();
    st.party_group_active = false;
    st.party_leader_index = None;
    st.is_party_lfg = false;
    st.everyone_assistant = false;
    Ok(())
}

/// `LeaveParty()` — clear members + active flag.
fn leave_party(state: &mut LuaState) -> LuaResult<u32> {
    clear_party_roster(state)?;
    push_event(state, "GROUP_ROSTER_UPDATE")?;
    Ok(0)
}

/// `RemoveFromParty(name)` — drop the first member whose name matches.
/// Silent no-op when missing or not present.
fn remove_from_party(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = required_string(state, 1) else {
        return Ok(0);
    };
    let removed = {
        let mut st = borrow_state_mut(state)?;
        let before = st.party_members.len();
        st.party_members.retain(|m| m.name != name);
        before != st.party_members.len()
    };
    if removed {
        push_event(state, "GROUP_ROSTER_UPDATE")?;
    }
    Ok(0)
}

/// `UninviteUnit(unit)` — drop by unit token (`party1`..`party4`) or by
/// player name. Silent no-op when input is missing or unknown.
fn uninvite_unit(state: &mut LuaState) -> LuaResult<u32> {
    let Some(token) = required_string(state, 1) else {
        return Ok(0);
    };
    let removed = {
        let mut st = borrow_state_mut(state)?;
        if let Some(index) = parse_party_index(&token)
            && index < st.party_members.len()
        {
            st.party_members.remove(index);
            true
        } else {
            let before = st.party_members.len();
            st.party_members.retain(|m| m.name != token);
            before != st.party_members.len()
        }
    };
    if removed {
        push_event(state, "GROUP_ROSTER_UPDATE")?;
    }
    Ok(0)
}

/// `KickUnit(unit)` — alias for `UninviteUnit`.
fn kick_unit(state: &mut LuaState) -> LuaResult<u32> {
    uninvite_unit(state)
}

/// `ReadyCheck()` — fire `READY_CHECK`.
fn ready_check(state: &mut LuaState) -> LuaResult<u32> {
    push_event(state, "READY_CHECK")?;
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "InviteToGroup", invite_to_group)?;
    LuaApiMut::register_function(lua, "AcceptGroup", accept_group)?;
    LuaApiMut::register_function(lua, "DeclineGroup", decline_group)?;
    LuaApiMut::register_function(lua, "LeaveParty", leave_party)?;
    LuaApiMut::register_function(lua, "RemoveFromParty", remove_from_party)?;
    LuaApiMut::register_function(lua, "UninviteUnit", uninvite_unit)?;
    LuaApiMut::register_function(lua, "KickUnit", kick_unit)?;
    LuaApiMut::register_function(lua, "ReadyCheck", ready_check)?;
    Ok(())
}
