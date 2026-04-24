//! Integration tests for `src/lua_api/globals/guild_probes.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── IsInGuild ─────────────────────────────────────────────────────────────────

#[test]
fn is_in_guild_true_when_guild_name_is_seeded() {
    let env = env();
    // Default seeded world has a guild_name.
    assert!(env.state().borrow().world.guild_name.is_some());
    let b: bool = env.eval("return IsInGuild()").unwrap();
    assert!(b);
}

#[test]
fn is_in_guild_false_after_clearing_guild_name() {
    let env = env();
    env.state().borrow_mut().world.guild_name = None;
    let b: bool = env.eval("return IsInGuild()").unwrap();
    assert!(!b);
}

#[test]
fn can_guild_invite_tracks_guild_membership() {
    let env = env();
    let seeded: bool = env.eval("return CanGuildInvite()").unwrap();
    assert!(seeded);
    env.state().borrow_mut().world.guild_name = None;
    let cleared: bool = env.eval("return CanGuildInvite()").unwrap();
    assert!(!cleared);
}

#[test]
fn is_guild_leader_defaults_false() {
    let env = env();
    let leader: bool = env.eval("return IsGuildLeader()").unwrap();
    assert!(!leader);
}

#[test]
fn guild_recipe_queries_have_safe_defaults() {
    let env = env();
    let can_view: bool = env
        .eval(
            r#"
            QueryGuildRecipes()
            return CanViewGuildRecipes(0)
            "#,
        )
        .unwrap();
    assert!(!can_view);
}

// ── CanReplaceGuildMaster ─────────────────────────────────────────────────────

#[test]
fn can_replace_guild_master_defaults_false() {
    let env = env();
    let b: bool = env.eval("return CanReplaceGuildMaster()").unwrap();
    assert!(!b);
}

#[test]
fn can_replace_guild_master_reads_flag() {
    let env = env();
    env.state().borrow_mut().can_replace_guild_master = true;
    let b: bool = env.eval("return CanReplaceGuildMaster()").unwrap();
    assert!(b);
}

// ── GetAutoDeclineGuildInvites ────────────────────────────────────────────────

#[test]
fn get_auto_decline_guild_invites_defaults_false() {
    let env = env();
    let b: bool = env.eval("return GetAutoDeclineGuildInvites()").unwrap();
    assert!(!b);
}

#[test]
fn get_auto_decline_guild_invites_reads_flag() {
    let env = env();
    env.state().borrow_mut().auto_decline_guild_invites = true;
    let b: bool = env.eval("return GetAutoDeclineGuildInvites()").unwrap();
    assert!(b);
}

// ── GetGuildRosterShowOffline ─────────────────────────────────────────────────

#[test]
fn get_guild_roster_show_offline_defaults_true() {
    let env = env();
    let b: bool = env.eval("return GetGuildRosterShowOffline()").unwrap();
    assert!(b, "retail defaults show-offline to true");
}

#[test]
fn get_guild_roster_show_offline_toggles_off() {
    let env = env();
    env.state().borrow_mut().guild_roster_show_offline = false;
    let b: bool = env.eval("return GetGuildRosterShowOffline()").unwrap();
    assert!(!b);
}

// ── GetNumGuildMembers / GetGuildRosterSize ───────────────────────────────────

#[test]
fn get_num_guild_members_returns_zero_when_roster_empty() {
    let env = env();
    env.state().borrow_mut().world.guild_members.clear();
    let (total, online, online_mobile): (i32, i32, i32) =
        env.eval("return GetNumGuildMembers()").unwrap();
    assert_eq!(total, 0, "cleared roster has no entries");
    assert_eq!(online, 0);
    assert_eq!(online_mobile, 0);
}

#[test]
fn get_num_guild_members_counts_roster() {
    use wow_ui_sim::lua_api::state::GuildMember;
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.world.guild_members = vec![
            GuildMember {
                name: "Alpha".into(),
                rank_index: 1,
                online: true,
            },
            GuildMember {
                name: "Beta".into(),
                rank_index: 2,
                online: false,
            },
        ];
    }
    let (total, online, online_mobile): (i32, i32, i32) =
        env.eval("return GetNumGuildMembers()").unwrap();
    assert_eq!(total, 2);
    assert_eq!(online, 1, "only online roster entries count as online");
    assert_eq!(online_mobile, 0);
}

#[test]
fn seeded_guild_roster_has_one_online_and_one_offline_member() {
    let env = env();
    let (total, online, first_online, second_online): (i32, i32, bool, bool) = env
        .eval(
            r#"
            local total, online = GetNumGuildMembers()
            local _, _, _, _, _, _, _, _, firstOnline = GetGuildRosterInfo(1)
            local _, _, _, _, _, _, _, _, secondOnline = GetGuildRosterInfo(2)
            return total, online, firstOnline, secondOnline
            "#,
        )
        .unwrap();

    assert_eq!(total, 2);
    assert_eq!(online, 1);
    assert!(first_online);
    assert!(!second_online);
}

#[test]
fn get_guild_roster_size_returns_total_count() {
    use wow_ui_sim::lua_api::state::GuildMember;
    let env = env();
    env.state().borrow_mut().world.guild_members = vec![GuildMember {
        name: "Solo".into(),
        rank_index: 1,
        online: true,
    }];
    let n: i32 = env.eval("return GetGuildRosterSize()").unwrap();
    assert_eq!(n, 1);
}

// ── GetGuildRosterMOTD ────────────────────────────────────────────────────────

#[test]
fn get_guild_roster_motd_defaults_empty() {
    let env = env();
    let motd: String = env.eval("return GetGuildRosterMOTD()").unwrap();
    assert!(motd.is_empty());
}

#[test]
fn get_guild_roster_motd_reads_state() {
    let env = env();
    env.state().borrow_mut().world.guild_motd = "Raid Tuesday 9pm".into();
    let motd: String = env.eval("return GetGuildRosterMOTD()").unwrap();
    assert_eq!(motd, "Raid Tuesday 9pm");
}

// ── GetGuildRosterInfo ────────────────────────────────────────────────────────

#[test]
fn get_guild_roster_info_returns_nil_for_out_of_range_index() {
    let env = env();
    let name: Option<String> = env.eval("return (GetGuildRosterInfo(99))").unwrap();
    assert_eq!(name, None);
}

#[test]
fn get_guild_roster_info_synthesises_row_from_member_and_rank() {
    use wow_ui_sim::lua_api::state::{GuildMember, GuildRank};
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.world.guild_ranks = vec![
            GuildRank {
                name: "Guild Master".into(),
                flags: vec![],
            },
            GuildRank {
                name: "Officer".into(),
                flags: vec![],
            },
            GuildRank {
                name: "Veteran".into(),
                flags: vec![],
            },
        ];
        st.world.guild_members = vec![GuildMember {
            name: "Alpha".into(),
            rank_index: 2,
            online: true,
        }];
        st.player.level = 80;
        st.player.class_index = 2; // Paladin
    }

    let (name, rank_name, rank_index, level, class, online, class_file): (
        String,
        String,
        i32,
        i32,
        String,
        bool,
        String,
    ) = env
        .eval(
            r#"
            local name, rankName, rankIndex, level, class,
                  _zone, _note, _officer, online, _status, classFile
                = GetGuildRosterInfo(1)
            return name, rankName, rankIndex, level, class, online, classFile
            "#,
        )
        .unwrap();

    assert_eq!(name, "Alpha");
    assert_eq!(rank_name, "Officer");
    assert_eq!(rank_index, 1, "retail exposes 0-based rankIndex");
    assert_eq!(level, 80);
    assert_eq!(class, "Paladin");
    assert!(online);
    assert_eq!(class_file, "PALADIN");
}

#[test]
fn get_guild_roster_info_reports_offline_member() {
    use wow_ui_sim::lua_api::state::GuildMember;
    let env = env();
    env.state().borrow_mut().world.guild_members = vec![GuildMember {
        name: "Offline".into(),
        rank_index: 1,
        online: false,
    }];

    let online: bool = env
        .eval(
            r#"
            local _, _, _, _, _, _, _, _, online = GetGuildRosterInfo(1)
            return online
            "#,
        )
        .unwrap();

    assert!(!online);
}
