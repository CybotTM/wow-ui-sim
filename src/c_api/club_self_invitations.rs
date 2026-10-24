//! Supplied incoming invitations, separate from outgoing invitations and membership.
use super::club_model::Member;

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
