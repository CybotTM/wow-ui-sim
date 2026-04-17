//! `C_GuildInfo.GetClubId` / `IsGuildOfficer` / `CanSpeakInGuildChat`.
//!
//! NOTE: `tests/guild_info.rs` is a separate aspirational test for seeded
//! MOTD / InfoText, which are unimplemented and were failing before this
//! PLAN item. This file scopes to the three probes actually in the PLAN.

use wow_ui_sim::lua_api::WowLuaEnv;

fn probes(env: &WowLuaEnv) -> (Option<String>, bool, bool) {
    env.eval(
        r#"
        return C_GuildInfo.GetClubId(),
               C_GuildInfo.IsGuildOfficer(),
               C_GuildInfo.CanSpeakInGuildChat()
        "#,
    )
    .unwrap()
}

#[test]
fn defaults_no_club_not_officer_can_speak() {
    let env = WowLuaEnv::new().unwrap();
    let (club, officer, can_speak) = probes(&env);
    assert_eq!(club, None, "no guild → no club id");
    assert!(!officer);
    assert!(can_speak, "chat default is true (no mute)");
}

#[test]
fn admin_sets_club_id() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetGuildClubId("42-alliance-heroes")"#)
        .unwrap();
    let (club, _, _) = probes(&env);
    assert_eq!(club.as_deref(), Some("42-alliance-heroes"));
}

#[test]
fn admin_clears_club_id_on_nil_or_empty() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetGuildClubId("X")"#).unwrap();
    env.exec(r#"A_Admin.SetGuildClubId("")"#).unwrap();
    assert_eq!(probes(&env).0, None);

    env.exec(r#"A_Admin.SetGuildClubId("X")"#).unwrap();
    env.exec(r#"A_Admin.SetGuildClubId(nil)"#).unwrap();
    assert_eq!(probes(&env).0, None);
}

#[test]
fn admin_toggles_officer_flag() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetGuildIsOfficer(true)").unwrap();
    assert!(probes(&env).1);
    env.exec("A_Admin.SetGuildIsOfficer(false)").unwrap();
    assert!(!probes(&env).1);
}

#[test]
fn admin_toggles_can_speak_flag() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetGuildCanSpeakInChat(false)").unwrap();
    assert!(!probes(&env).2);
    env.exec("A_Admin.SetGuildCanSpeakInChat(true)").unwrap();
    assert!(probes(&env).2);
}

#[test]
fn no_arg_setters_default_to_true() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.SetGuildCanSpeakInChat(false)").unwrap();
    env.exec("A_Admin.SetGuildIsOfficer()").unwrap();
    env.exec("A_Admin.SetGuildCanSpeakInChat()").unwrap();
    let (_, officer, can_speak) = probes(&env);
    assert!(officer);
    assert!(can_speak);
}

#[test]
fn other_c_guild_info_members_fall_through_to_namespace_metamethod() {
    // Unimplemented C_GuildInfo.* should still resolve via the stub
    // namespace's __index fallback (returns a no-op fn).
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            if type(C_GuildInfo.SomeUnimplementedMember) ~= "function" then
                return "missing"
            end
            if C_GuildInfo.SomeUnimplementedMember() ~= nil then
                return "non_nil"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}
