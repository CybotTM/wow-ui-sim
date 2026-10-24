//! Supplied incoming invitations, separate from outgoing invitations and membership.
use super::club_members::build_member_info;
use super::club_model::Member;
use super::helpers::set_table_array;
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

/// Non-null ClubInfo fields from ClubDocumentation.lua. Nullable metadata is absent.
#[derive(Clone, Debug)]
pub struct InvitedClub {
    pub id: String,
    pub name: String,
    pub description: String,
    pub broadcast: String,
    pub club_type: u8,
    pub avatar_id: u32,
}

/// An active-player invitation snapshot. Opaque tokens use the existing club model
/// representation; no native token encoding or service behavior is inferred here.
#[derive(Clone, Debug)]
pub struct SelfInvitation {
    pub id: String,
    pub club: InvitedClub,
    pub inviter: Member,
    pub leaders: Vec<Member>,
}

/// The documented initialized-clubs query projects only supplied active-player rows.
/// Initialization failure semantics and service refresh/events are not inferred.
pub(super) fn get_invitations_for_self(state: &mut LuaState) -> LuaResult<u32> {
    let invitations = borrow_state(state)?.clubs.self_invitations.clone();
    let result = create_table(state);
    for (index, invitation) in invitations.iter().enumerate() {
        let row = build_invitation(state, invitation);
        set_table_array(state, result, index as i64 + 1, row);
    }
    state.push(result);
    Ok(1)
}

fn build_invitation(state: &mut LuaState, invitation: &SelfInvitation) -> Val {
    let row = create_table(state);
    let id = create_string(state, &invitation.id);
    table_set(state, row, "invitationId", id);
    let club = build_invited_club(state, &invitation.club);
    table_set(state, row, "club", club);
    let inviter = build_member_info(state, &invitation.inviter, invitation.club.club_type);
    table_set(state, row, "inviter", inviter);
    let leaders = create_table(state);
    for (index, member) in invitation.leaders.iter().enumerate() {
        let leader = build_member_info(state, member, invitation.club.club_type);
        set_table_array(state, leaders, index as i64 + 1, leader);
    }
    table_set(state, row, "leaders", leaders);
    row
}

fn build_invited_club(state: &mut LuaState, club: &InvitedClub) -> Val {
    let table = create_table(state);
    for (field, text) in [
        ("clubId", &club.id),
        ("name", &club.name),
        ("description", &club.description),
        ("broadcast", &club.broadcast),
    ] {
        let value = create_string(state, text);
        table_set(state, table, field, value);
    }
    table_set(state, table, "clubType", Val::Num(club.club_type as f64));
    table_set(state, table, "avatarId", Val::Num(club.avatar_id as f64));
    table
}
