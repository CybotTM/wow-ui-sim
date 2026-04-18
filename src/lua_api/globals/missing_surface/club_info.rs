//! `C_Club` probe surface backed by `WorldState.guild_*` fields.
//!
//! Synthesises one club entry representing the player's guild.
//! No new SimState fields are added — all data is derived from
//! the existing `world.guild_name`, `world.guild_members`, and
//! `world.guild_num_members`.
//!
//! Migrates 4 entries off the namespace stub tables:
//!
//! - `C_Club.GetSubscribedClubs()` — returns a single-element array
//!   containing a `ClubInfo`-like table for the guild when
//!   `world.guild_name` is set, else an empty array.
//! - `C_Club.GetClubMembers(clubId)` — returns an array of
//!   `ClubMemberInfo`-like tables derived from `world.guild_members`.
//! - `C_Club.GetClubCapacity(clubId)` — returns 1000 (hard-coded guild
//!   capacity; retail is unbounded in practice).
//! - `C_Club.IsEnabled()` — returns true unconditionally.
//! - `C_Club.IsRestricted()` — returns `ClubRestrictionReason.None`.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const GUILD_CLUB_ID: &str = "guild-0";
const GUILD_CLUB_TYPE: &str = "Guild";
const GUILD_CLUB_CAPACITY: f64 = 1000.0;

pub(super) fn register_club_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Club")?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSubscribedClubs",
        c_club_get_subscribed_clubs,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetClubMembers", c_club_get_club_members)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetClubCapacity",
        c_club_get_club_capacity,
    )?;
    table_set_rust_fn_static(state, table_ref, "IsEnabled", c_club_is_enabled)?;
    table_set_rust_fn_static(state, table_ref, "IsRestricted", c_club_is_restricted)?;
    Ok(())
}

fn c_club_get_subscribed_clubs(state: &mut LuaState) -> LuaResult<u32> {
    let guild_name = borrow_state(state)?.world.guild_name.clone();
    let array = create_table(state);
    if let Some(name) = guild_name {
        let entry = build_club_info_table(state, &name);
        set_table_array(state, array, 1, entry);
    }
    state.push(array);
    Ok(1)
}

fn c_club_get_club_members(state: &mut LuaState) -> LuaResult<u32> {
    // Accept any clubId — the sim only has one synthetic guild club.
    let members = borrow_state(state)?.world.guild_members.clone();
    let array = create_table(state);
    for (index, member) in members.iter().enumerate() {
        let entry = build_member_info_table(state, member.rank_index, &member.name, index == 0);
        set_table_array(state, array, index as i64 + 1, entry);
    }
    state.push(array);
    Ok(1)
}

fn c_club_get_club_capacity(_state: &mut LuaState) -> LuaResult<u32> {
    _state.push(Val::Num(GUILD_CLUB_CAPACITY));
    Ok(1)
}

fn c_club_is_enabled(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_club_is_restricted(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn build_club_info_table(state: &mut LuaState, name: &str) -> Val {
    let t = create_table(state);
    let club_id = create_string(state, GUILD_CLUB_ID);
    let club_type = create_string(state, GUILD_CLUB_TYPE);
    let name_str = create_string(state, name);
    table_set(state, t, "clubId", club_id);
    table_set(state, t, "clubType", club_type);
    table_set(state, t, "name", name_str);
    let desc = create_string(state, "");
    let broadcast = create_string(state, "");
    table_set(state, t, "description", desc);
    table_set(state, t, "broadcast", broadcast);
    table_set(state, t, "avatarId", Val::Num(0.0));
    t
}

fn build_member_info_table(
    state: &mut LuaState,
    rank_order: i32,
    name: &str,
    is_self: bool,
) -> Val {
    let t = create_table(state);
    let name_str = create_string(state, name);
    // memberId: use rank_order as a simple unique-ish integer id.
    table_set(state, t, "memberId", Val::Num(rank_order as f64));
    table_set(state, t, "name", name_str);
    table_set(state, t, "isSelf", Val::Bool(is_self));
    table_set(state, t, "guildRankOrder", Val::Num(rank_order as f64));
    // presence: 1 = online (ClubMemberPresence.Online in retail enum).
    table_set(state, t, "presence", Val::Num(1.0));
    t
}
