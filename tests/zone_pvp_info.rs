//! `C_PvP.GetZonePVPInfo()` + `A_Admin.SetZonePVP` round-trip.

use wow_ui_sim::lua_api::WowLuaEnv;

fn zone_pvp(env: &WowLuaEnv) -> (String, bool, Option<String>) {
    env.eval::<(String, bool, Option<String>)>(
        r#"
        local a, b, c = C_PvP.GetZonePVPInfo()
        return a, b, c
        "#,
    )
    .unwrap()
}

#[test]
fn defaults_are_contested_non_subzone_no_faction() {
    let env = WowLuaEnv::new().unwrap();
    let (pvp_type, is_sub_zone, faction) = zone_pvp(&env);
    assert_eq!(pvp_type, "contested");
    assert!(!is_sub_zone);
    assert_eq!(faction, None);
}

#[test]
fn admin_set_zone_pvp_drives_all_three_returns() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetZonePVP("friendly", true, "Alliance")"#)
        .unwrap();
    let (pvp_type, is_sub_zone, faction) = zone_pvp(&env);
    assert_eq!(pvp_type, "friendly");
    assert!(is_sub_zone);
    assert_eq!(faction.as_deref(), Some("Alliance"));
}

#[test]
fn admin_set_zone_pvp_empty_args_reset_to_defaults() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetZonePVP("arena", true, "Horde")"#)
        .unwrap();
    // No-arg call resets to contested/false/nil.
    env.exec("A_Admin.SetZonePVP()").unwrap();
    let (pvp_type, is_sub_zone, faction) = zone_pvp(&env);
    assert_eq!(pvp_type, "contested");
    assert!(!is_sub_zone);
    assert_eq!(faction, None);
}

#[test]
fn admin_set_zone_pvp_empty_faction_clears_it() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetZonePVP("friendly", false, "Horde")"#)
        .unwrap();
    env.exec(r#"A_Admin.SetZonePVP("contested", false, "")"#)
        .unwrap();
    let (_, _, faction) = zone_pvp(&env);
    assert_eq!(faction, None, "empty faction string should clear to nil");
}

#[test]
fn each_canonical_pvp_type_round_trips() {
    let env = WowLuaEnv::new().unwrap();
    for token in &[
        "contested",
        "sanctuary",
        "arena",
        "friendly",
        "hostile",
        "combat",
    ] {
        env.exec(&format!(r#"A_Admin.SetZonePVP({token:?})"#))
            .unwrap();
        let (pvp_type, _, _) = zone_pvp(&env);
        assert_eq!(pvp_type, *token);
    }
}

#[test]
fn faction_name_can_be_either_side() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetZonePVP("friendly", false, "Alliance")"#)
        .unwrap();
    assert_eq!(zone_pvp(&env).2.as_deref(), Some("Alliance"));
    env.exec(r#"A_Admin.SetZonePVP("friendly", false, "Horde")"#)
        .unwrap();
    assert_eq!(zone_pvp(&env).2.as_deref(), Some("Horde"));
}
