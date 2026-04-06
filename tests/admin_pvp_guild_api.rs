//! Tests for A_Admin PvP and Guild API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetHonorLevel
// ============================================================================

#[test]
fn test_set_honor_level_unit_honor_level_returns_level() {
    let env = env();
    let level: i32 = env
        .eval(
            r#"
            A_Admin.SetHonorLevel(50)
            return UnitHonorLevel("player")
            "#,
        )
        .unwrap();
    assert_eq!(level, 50);
}

#[test]
fn test_unit_honor_returns_zero() {
    let env = env();
    let honor: i32 = env.eval(r#"return UnitHonor("player")"#).unwrap();
    assert_eq!(honor, 0);
}

#[test]
fn test_unit_power_bar_timer_info_returns_nil() {
    let env = env();
    let is_nil: bool = env
        .eval(r#"return UnitPowerBarTimerInfo("player", 1) == nil"#)
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_get_send_mail_price_returns_30() {
    let env = env();
    let price: i32 = env.eval("return GetSendMailPrice()").unwrap();
    assert_eq!(price, 30);
}

#[test]
fn test_get_web_ticket_returns_nil() {
    let env = env();
    let is_nil: bool = env.eval("return GetWebTicket() == nil").unwrap();
    assert!(is_nil);
}

#[test]
fn test_set_honor_level_zero() {
    let env = env();
    let level: i32 = env
        .eval(
            r#"
            A_Admin.SetHonorLevel(0)
            return UnitHonorLevel("player")
            "#,
        )
        .unwrap();
    assert_eq!(level, 0);
}

#[test]
fn test_set_honor_level_max() {
    let env = env();
    let level: i32 = env
        .eval(
            r#"
            A_Admin.SetHonorLevel(500)
            return UnitHonorLevel("player")
            "#,
        )
        .unwrap();
    assert_eq!(level, 500);
}

#[test]
fn test_set_honor_level_overwrites_previous() {
    let env = env();
    let level: i32 = env
        .eval(
            r#"
            A_Admin.SetHonorLevel(10)
            A_Admin.SetHonorLevel(75)
            return UnitHonorLevel("player")
            "#,
        )
        .unwrap();
    assert_eq!(level, 75);
}

// ============================================================================
// SetGuildInfo
// ============================================================================

#[test]
fn test_set_guild_info_c_guild_get_guild_info_name() {
    let env = env();
    let guild_name: String = env
        .eval(
            r#"
            A_Admin.SetGuildInfo("Epic Guild", "Officer", 150)
            local guildName = C_Guild.GetGuildInfo("player")
            return guildName
            "#,
        )
        .unwrap();
    assert_eq!(guild_name, "Epic Guild");
}

#[test]
fn test_set_guild_info_is_in_guild_returns_true() {
    let env = env();
    let in_guild: bool = env
        .eval(
            r#"
            A_Admin.SetGuildInfo("Epic Guild", "Officer", 150)
            return IsInGuild()
            "#,
        )
        .unwrap();
    assert!(in_guild);
}

#[test]
fn test_set_guild_info_c_guild_is_in_guild_returns_true() {
    let env = env();
    let in_guild: bool = env
        .eval(
            r#"
            A_Admin.SetGuildInfo("Epic Guild", "Officer", 150)
            return C_Guild.IsInGuild()
            "#,
        )
        .unwrap();
    assert!(in_guild);
}

#[test]
fn test_set_guild_info_c_guild_get_guild_info_rank() {
    let env = env();
    let (guild_name, guild_rank): (String, String) = env
        .eval(
            r#"
            A_Admin.SetGuildInfo("Some Guild", "Guild Master", 42)
            local name, rank = C_Guild.GetGuildInfo("player")
            return name, rank
            "#,
        )
        .unwrap();
    assert_eq!(guild_name, "Some Guild");
    assert_eq!(guild_rank, "Guild Master");
}

// ============================================================================
// ClearGuild
// ============================================================================

#[test]
fn test_clear_guild_is_in_guild_returns_false() {
    let env = env();
    let in_guild: bool = env
        .eval(
            r#"
            A_Admin.SetGuildInfo("Epic Guild", "Officer", 150)
            A_Admin.ClearGuild()
            return IsInGuild()
            "#,
        )
        .unwrap();
    assert!(!in_guild);
}

#[test]
fn test_clear_guild_c_guild_is_in_guild_returns_false() {
    let env = env();
    let in_guild: bool = env
        .eval(
            r#"
            A_Admin.SetGuildInfo("Epic Guild", "Officer", 150)
            A_Admin.ClearGuild()
            return C_Guild.IsInGuild()
            "#,
        )
        .unwrap();
    assert!(!in_guild);
}

#[test]
fn test_in_guild_by_default() {
    let env = env();
    let in_guild: bool = env.eval("return IsInGuild()").unwrap();
    assert!(in_guild);
}

#[test]
fn test_default_guild_name() {
    let env = env();
    let name: String = env
        .eval("return (C_Guild.GetGuildInfo('player'))")
        .unwrap();
    assert_eq!(name, "Heroes of Azeroth");
}

// ============================================================================
// GuildControlGetNumRanks
// ============================================================================

#[test]
fn test_guild_control_get_num_ranks_returns_zero_when_not_in_guild() {
    let env = env();
    let ranks: i32 = env.eval("return GuildControlGetNumRanks()").unwrap();
    assert_eq!(ranks, 0);
}

#[test]
fn test_guild_control_set_rank_does_not_error() {
    let env = env();
    env.eval::<()>("GuildControlSetRank(1)").unwrap();
}

// ============================================================================
// GetGuildFactionGroup
// ============================================================================

#[test]
fn test_get_guild_faction_group_nil_when_not_in_guild() {
    let env = env();
    let is_nil: bool = env
        .eval("return GetGuildFactionGroup() == nil")
        .unwrap();
    assert!(is_nil);
}

// ============================================================================
// GetGroupMemberCounts
// ============================================================================

#[test]
fn test_get_group_member_counts_all_zero_when_solo() {
    let env = env();
    let total: i32 = env
        .eval(
            r#"
            local c = GetGroupMemberCounts()
            return c.TANK + c.HEALER + c.DAMAGER + c.NOROLE
            "#,
        )
        .unwrap();
    assert_eq!(total, 0);
}

// ============================================================================
// UnitGroupRolesAssigned
// ============================================================================

#[test]
fn test_unit_group_roles_assigned_returns_none() {
    let env = env();
    let role: String = env
        .eval(r#"return UnitGroupRolesAssigned("player")"#)
        .unwrap();
    assert_eq!(role, "NONE");
}

// ============================================================================
// GetDungeonDifficultyID
// ============================================================================

#[test]
fn test_get_dungeon_difficulty_id_returns_normal() {
    let env = env();
    let diff: i32 = env.eval("return GetDungeonDifficultyID()").unwrap();
    assert_eq!(diff, 1);
}

// ============================================================================
// RequestGuildChallengeInfo
// ============================================================================

#[test]
fn test_request_guild_challenge_info_does_not_error() {
    let env = env();
    env.eval::<()>("RequestGuildChallengeInfo()").unwrap();
}

// ============================================================================
// StoreSecureReference
// ============================================================================

#[test]
fn test_store_secure_reference_does_not_error() {
    let env = env();
    env.eval::<()>(r#"StoreSecureReference("myref", CreateFrame("Frame"))"#)
        .unwrap();
}

// ============================================================================
// GetLFGInfoServer
// ============================================================================

#[test]
fn test_get_lfg_info_server_returns_not_queued() {
    let env = env();
    let (in_party, joined, queued): (bool, bool, bool) = env
        .eval(r#"return GetLFGInfoServer(1, 0)"#)
        .unwrap();
    assert!(!in_party);
    assert!(!joined);
    assert!(!queued);
}

// ============================================================================
// GetAvailableLocaleInfo
// ============================================================================

#[test]
fn test_get_available_locale_info_returns_enus() {
    let env = env();
    let name: String = env
        .eval("return GetAvailableLocaleInfo()[1].localeName")
        .unwrap();
    assert_eq!(name, "enUS");
}

#[test]
fn test_get_available_locale_info_returns_locale_id() {
    let env = env();
    let id: i32 = env
        .eval("return GetAvailableLocaleInfo()[1].localeId")
        .unwrap();
    assert_eq!(id, 1);
}
