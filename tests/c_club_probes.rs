//! Tests for `C_Club` probes backed by `WorldState.guild_*` fields.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::GuildMember;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn is_enabled_returns_true() {
    let env = env();
    let enabled: bool = env.eval("return C_Club.IsEnabled()").unwrap();
    assert!(enabled);
}

#[test]
fn get_subscribed_clubs_returns_guild_entry() {
    let env = env();
    let (count, name, club_type): (i32, String, i32) = env
        .eval(
            r#"
            local clubs = C_Club.GetSubscribedClubs()
            return #clubs, clubs[1].name, clubs[1].clubType
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(name, "Heroes of Azeroth");
    assert_eq!(club_type, 2);
}

#[test]
fn get_subscribed_clubs_club_id_is_string() {
    let env = env();
    let club_id: String = env
        .eval("return C_Club.GetSubscribedClubs()[1].clubId")
        .unwrap();
    assert!(!club_id.is_empty(), "clubId should be a non-empty string");
}

#[test]
fn get_club_info_returns_guild_shape() {
    let env = env();
    let (name, club_type, member_count): (String, i32, i32) = env
        .eval(
            r#"
            local club = C_Club.GetClubInfo("guild-0")
            return club.name, club.clubType, club.memberCount
            "#,
        )
        .unwrap();
    assert_eq!(name, "Heroes of Azeroth");
    assert_eq!(club_type, 2);
    assert!(member_count > 0);
}

#[test]
fn get_subscribed_clubs_empty_when_no_guild() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.guild_name = None;
    }
    let count: i32 = env.eval("return #C_Club.GetSubscribedClubs()").unwrap();
    assert_eq!(count, 0, "no guild → empty subscribed clubs list");
}

#[test]
fn get_club_members_returns_guild_roster() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.guild_members = vec![GuildMember {
            name: "Uther".into(),
            rank_index: 1,
            online: true,
        }];
    }
    let count: i32 = env
        .eval("return #C_Club.GetClubMembers('guild-0')")
        .unwrap();
    assert_eq!(count, 1, "should return seeded guild member");
}

#[test]
fn get_club_members_entry_has_required_fields() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.guild_members = vec![GuildMember {
            name: "Uther".into(),
            rank_index: 1,
            online: true,
        }];
    }
    let (member_id, name, is_self, presence): (i32, String, bool, i32) = env
        .eval(
            r#"
            local members = C_Club.GetClubMembers('guild-0')
            local memberId = members[1]
            local m = C_Club.GetMemberInfo('guild-0', memberId)
            return memberId, m.name, m.isSelf, m.presence
            "#,
        )
        .unwrap();
    assert_eq!(member_id, 1);
    assert_eq!(name, "Uther");
    assert!(is_self, "first member should be isSelf=true");
    assert_eq!(presence, 1, "online presence = 1");
}

#[test]
fn get_club_members_returns_member_ids_for_member_info_lookup() {
    let env = env();
    let (count, first_id, first_name, second_id, second_name): (i32, i32, String, i32, String) =
        env.eval(
            r#"
            local members = C_Club.GetClubMembers('guild-0')
            local first = C_Club.GetMemberInfo('guild-0', members[1])
            local second = C_Club.GetMemberInfo('guild-0', members[2])
            return #members, members[1], first.name, members[2], second.name
            "#,
        )
        .unwrap();

    assert_eq!(count, 2);
    assert_eq!(first_id, 1);
    assert_eq!(first_name, "Uther");
    assert_eq!(second_id, 2);
    assert_eq!(second_name, "Jaina");
}

#[test]
fn get_member_info_for_self_has_role() {
    let env = env();
    let (name, is_self, role): (String, bool, i32) = env
        .eval(
            r#"
            local member = C_Club.GetMemberInfoForSelf("guild-0")
            return member.name, member.isSelf, member.role
            "#,
        )
        .unwrap();
    assert!(!name.is_empty());
    assert!(is_self);
    assert_eq!(role, 4);
}

#[test]
fn get_club_privileges_returns_flags_table() {
    let env = env();
    let (kind, can_invite): (String, bool) = env
        .eval(
            r#"
            local privileges = C_Club.GetClubPrivileges("guild-0")
            return type(privileges), privileges.canSendInvitation
            "#,
        )
        .unwrap();
    assert_eq!(kind, "table");
    assert!(!can_invite);
}

#[test]
fn get_club_members_reflects_state_mutation() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.world.guild_members = vec![
            GuildMember {
                name: "Arthas".into(),
                rank_index: 1,
                online: true,
            },
            GuildMember {
                name: "Jaina".into(),
                rank_index: 2,
                online: false,
            },
        ];
    }
    let (count, first_name, first_presence, second_name, second_presence): (
        i32,
        String,
        i32,
        String,
        i32,
    ) = env
        .eval(
            r#"
            local members = C_Club.GetClubMembers('guild-0')
            local first = C_Club.GetMemberInfo('guild-0', members[1])
            local second = C_Club.GetMemberInfo('guild-0', members[2])
            return #members, first.name, first.presence, second.name, second.presence
            "#,
        )
        .unwrap();
    assert_eq!(count, 2);
    assert_eq!(first_name, "Arthas");
    assert_eq!(first_presence, 1);
    assert_eq!(second_name, "Jaina");
    assert_eq!(second_presence, 3);
}

#[test]
fn members_are_ready_for_seeded_guild_club() {
    let env = env();
    let (club_id, ready): (String, bool) = env
        .eval(
            r#"
            local clubId = C_Club.GetGuildClubId()
            C_Club.FocusMembers(clubId)
            return clubId, C_Club.AreMembersReady(clubId)
            "#,
        )
        .unwrap();

    assert_eq!(club_id, "guild-0");
    assert!(ready);
}

#[test]
fn get_club_capacity_returns_number() {
    let env = env();
    let capacity: i32 = env
        .eval("return C_Club.GetClubCapacity('guild-0')")
        .unwrap();
    assert!(capacity > 0, "guild capacity should be positive");
}

#[test]
fn get_streams_returns_sortable_table() {
    let env = env();
    let (stream_type, count): (String, i32) = env
        .eval(
            r#"
            local streams = C_Club.GetStreams('guild-0')
            table.sort(streams, function(lhs, rhs)
                return lhs.creationTime < rhs.creationTime
            end)
            return type(streams), #streams
            "#,
        )
        .unwrap();
    assert_eq!(stream_type, "table");
    assert_eq!(count, 0);
}

#[test]
fn club_unread_message_queries_have_safe_defaults() {
    let env = env();
    let (any_unread, stream_marker_type, settings_count): (bool, String, i32) = env
        .eval(
            r#"
            local settings = C_Club.GetClubStreamNotificationSettings('guild-0')
            return
                C_Club.DoesAnyCommunityHaveUnreadMessages(),
                type(C_Club.GetStreamViewMarker('guild-0', 1)),
                #settings
            "#,
        )
        .unwrap();
    assert!(!any_unread);
    assert_eq!(stream_marker_type, "nil");
    assert_eq!(settings_count, 0);
}

#[test]
fn club_finder_queries_have_safe_empty_defaults() {
    let env = env();
    let (enabled, invites, applicants, pending, status_flags, guild_total): (
        bool,
        i32,
        i32,
        i32,
        i32,
        i32,
    ) = env
        .eval(
            r#"
            return
                C_ClubFinder.IsEnabled(),
                #C_ClubFinder.PlayerGetClubInvitationList(),
                #C_ClubFinder.ReturnClubApplicantList("guild-0"),
                #C_ClubFinder.ReturnPendingClubApplicantList("guild-0"),
                #C_ClubFinder.GetStatusOfPostingFromClubId("guild-0"),
                C_ClubFinder.GetTotalMatchingGuildListSize()
            "#,
        )
        .unwrap();
    assert!(enabled);
    assert_eq!(invites, 0);
    assert_eq!(applicants, 0);
    assert_eq!(pending, 0);
    assert_eq!(status_flags, 0);
    assert_eq!(guild_total, 0);
}
