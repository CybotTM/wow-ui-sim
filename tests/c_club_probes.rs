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
        }];
    }
    let (name, is_self, presence): (String, bool, i32) = env
        .eval(
            r#"
            local members = C_Club.GetClubMembers('guild-0')
            local m = members[1]
            return m.name, m.isSelf, m.presence
            "#,
        )
        .unwrap();
    assert_eq!(name, "Uther");
    assert!(is_self, "first member should be isSelf=true");
    assert_eq!(presence, 1, "online presence = 1");
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
            },
            GuildMember {
                name: "Jaina".into(),
                rank_index: 2,
            },
        ];
    }
    let (count, first_name, second_name): (i32, String, String) = env
        .eval(
            r#"
            local members = C_Club.GetClubMembers('guild-0')
            return #members, members[1].name, members[2].name
            "#,
        )
        .unwrap();
    assert_eq!(count, 2);
    assert_eq!(first_name, "Arthas");
    assert_eq!(second_name, "Jaina");
}

#[test]
fn get_club_capacity_returns_number() {
    let env = env();
    let capacity: i32 = env
        .eval("return C_Club.GetClubCapacity('guild-0')")
        .unwrap();
    assert!(capacity > 0, "guild capacity should be positive");
}
