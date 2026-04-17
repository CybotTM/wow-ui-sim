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
