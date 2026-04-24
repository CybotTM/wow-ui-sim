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
//! - `C_Club.GetClubMembers(clubId)` — returns an array of member IDs.
//! - `C_Club.GetMemberInfo(clubId, memberId)` — returns a
//!   `ClubMemberInfo`-like table derived from `world.guild_members`.
//! - `C_Club.GetStreams(clubId)` — returns an empty array; the simulator does
//!   not model club chat streams yet.
//! - `C_Club.GetClubCapacity(clubId)` — returns 1000 (hard-coded guild
//!   capacity; retail is unbounded in practice).
//! - `C_Club.IsEnabled()` — returns true unconditionally.
//! - `C_Club.IsRestricted()` — returns `ClubRestrictionReason.None`.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{
    borrow_state, create_string, create_table, table_set, val_to_string,
};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const GUILD_CLUB_ID: &str = "guild-0";
const GUILD_CLUB_TYPE: f64 = 2.0;
const GUILD_CLUB_CAPACITY: f64 = 1000.0;

pub(super) fn register_club_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Club")?;
    register_club_lookup_methods(state, table_ref)?;
    register_club_status_methods(state, table_ref)?;
    Ok(())
}

fn register_club_lookup_methods(state: &mut LuaState, table_ref: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSubscribedClubs",
        c_club_get_subscribed_clubs,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetClubInfo", c_club_get_club_info)?;
    table_set_rust_fn_static(state, table_ref, "GetClubMembers", c_club_get_club_members)?;
    table_set_rust_fn_static(state, table_ref, "GetMemberInfo", c_club_get_member_info)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetMemberInfoForSelf",
        c_club_get_member_info_for_self,
    )?;
    Ok(())
}

fn register_club_status_methods(state: &mut LuaState, table_ref: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetClubPrivileges",
        c_club_get_club_privileges,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetStreams", c_club_get_streams)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetClubCapacity",
        c_club_get_club_capacity,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetGuildClubId", c_club_get_guild_club_id)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "AreMembersReady",
        c_club_are_members_ready,
    )?;
    table_set_rust_fn_static(state, table_ref, "FocusMembers", c_club_focus_members)?;
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

fn c_club_get_club_info(state: &mut LuaState) -> LuaResult<u32> {
    if !is_guild_club_arg(state) {
        state.push(Val::Nil);
        return Ok(1);
    }
    let guild_name = borrow_state(state)?.world.guild_name.clone();
    match guild_name {
        Some(name) => {
            let info = build_club_info_table(state, &name);
            state.push(info);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_club_get_club_members(state: &mut LuaState) -> LuaResult<u32> {
    if !is_guild_club_arg(state) {
        let empty_members = create_table(state);
        state.push(empty_members);
        return Ok(1);
    }

    let member_count = borrow_state(state)?.world.guild_members.len();
    let array = create_table(state);
    for zero_based_index in 0..member_count {
        let member_id = member_id_from_index(zero_based_index);
        set_table_array(state, array, member_id, Val::Num(member_id as f64));
    }
    state.push(array);
    Ok(1)
}

fn c_club_get_member_info(state: &mut LuaState) -> LuaResult<u32> {
    if !is_guild_club_arg(state) {
        state.push(Val::Nil);
        return Ok(1);
    }

    let member_id = i64::from_stack(state, 2)?;
    let Some(zero_based_index) = index_from_member_id(member_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let member = borrow_state(state)?
        .world
        .guild_members
        .get(zero_based_index)
        .cloned();
    match member {
        Some(member) => {
            let member_info = build_member_info_table(
                state,
                member_id,
                member.rank_index,
                &member.name,
                zero_based_index == 0,
                member.online,
            );
            state.push(member_info);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_club_get_member_info_for_self(state: &mut LuaState) -> LuaResult<u32> {
    if !is_guild_club_arg(state) {
        state.push(Val::Nil);
        return Ok(1);
    }
    let member = {
        let sim = borrow_state(state)?;
        sim.world.guild_name.as_ref().map(|_| {
            sim.world
                .guild_members
                .first()
                .map(|member| (member.rank_index, member.name.clone(), member.online))
                .unwrap_or_else(|| (1, sim.player.name.clone(), true))
        })
    };
    let Some((rank_index, name, online)) = member else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let member_info = build_member_info_table(state, 1, rank_index, &name, true, online);
    state.push(member_info);
    Ok(1)
}

fn c_club_get_club_privileges(state: &mut LuaState) -> LuaResult<u32> {
    let privileges = create_table(state);
    for field in CLUB_PRIVILEGE_FIELDS {
        table_set(state, privileges, field, Val::Bool(false));
    }
    state.push(privileges);
    Ok(1)
}

fn c_club_get_streams(state: &mut LuaState) -> LuaResult<u32> {
    let array = create_table(state);
    state.push(array);
    Ok(1)
}

fn c_club_get_club_capacity(_state: &mut LuaState) -> LuaResult<u32> {
    _state.push(Val::Num(GUILD_CLUB_CAPACITY));
    Ok(1)
}

fn c_club_get_guild_club_id(state: &mut LuaState) -> LuaResult<u32> {
    if borrow_state(state)?.world.guild_name.is_some() {
        let club_id = create_string(state, GUILD_CLUB_ID);
        state.push(club_id);
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

fn c_club_are_members_ready(state: &mut LuaState) -> LuaResult<u32> {
    let members_ready = is_guild_club_arg(state);
    state.push(Val::Bool(members_ready));
    Ok(1)
}

fn c_club_focus_members(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_club_is_enabled(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_club_is_restricted(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn is_guild_club_arg(state: &mut LuaState) -> bool {
    let value = stack_val(state, 1);
    matches!(value, Val::Str(_))
        && val_to_string(state, value).is_some_and(|club_id| club_id == GUILD_CLUB_ID)
}

fn member_id_from_index(zero_based_index: usize) -> i64 {
    zero_based_index as i64 + 1
}

fn index_from_member_id(member_id: i64) -> Option<usize> {
    member_id
        .checked_sub(1)
        .and_then(|zero_based| usize::try_from(zero_based).ok())
}

fn build_club_info_table(state: &mut LuaState, name: &str) -> Val {
    let t = create_table(state);
    let club_id = create_string(state, GUILD_CLUB_ID);
    let name_str = create_string(state, name);
    table_set(state, t, "clubId", club_id);
    table_set(state, t, "clubType", Val::Num(GUILD_CLUB_TYPE));
    table_set(state, t, "name", name_str);
    let desc = create_string(state, "");
    let broadcast = create_string(state, "");
    table_set(state, t, "description", desc);
    table_set(state, t, "broadcast", broadcast);
    table_set(state, t, "avatarId", Val::Num(0.0));
    table_set(state, t, "memberCount", Val::Num(GUILD_CLUB_CAPACITY));
    t
}

fn build_member_info_table(
    state: &mut LuaState,
    member_id: i64,
    rank_order: i32,
    name: &str,
    is_self: bool,
    online: bool,
) -> Val {
    let t = create_table(state);
    let name_str = create_string(state, name);
    table_set(state, t, "memberId", Val::Num(member_id as f64));
    table_set(state, t, "name", name_str);
    table_set(state, t, "isSelf", Val::Bool(is_self));
    table_set(state, t, "guildRankOrder", Val::Num(rank_order as f64));
    table_set(state, t, "role", Val::Num(4.0));
    // ClubMemberPresence: 1 = Online, 3 = Offline.
    table_set(
        state,
        t,
        "presence",
        Val::Num(if online { 1.0 } else { 3.0 }),
    );
    t
}

const CLUB_PRIVILEGE_FIELDS: &[&str] = &[
    "canDestroy",
    "canSetName",
    "canSetDescription",
    "canSetAvatar",
    "canSetBroadcast",
    "canSetPrivacyLevel",
    "canSetOwnMemberAttribute",
    "canSetOtherMemberAttribute",
    "canSetOwnMemberNote",
    "canSetOtherMemberNote",
    "canSetOwnVoiceState",
    "canSetOwnPresenceLevel",
    "canUseVoice",
    "canVoiceMuteMemberForAll",
    "canGetInvitation",
    "canSendInvitation",
    "canCreateStream",
    "canDestroyStream",
    "canSetStreamName",
    "canSetStreamSubject",
    "canSetStreamAccess",
    "canSetStreamVoiceLevel",
    "canCreateTicket",
];
