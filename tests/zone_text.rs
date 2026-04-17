//! Zone-text getters — canonical WoW fallback behaviour, SimState-backed.

use wow_ui_sim::lua_api::WowLuaEnv;

fn zone_text(env: &WowLuaEnv) -> (String, String, String, String) {
    env.eval(
        r#"
        return GetZoneText(), GetSubZoneText(),
               GetMinimapZoneText(), GetRealZoneText()
        "#,
    )
    .unwrap()
}

#[test]
fn defaults_match_sim_seed_state() {
    // SimState::default() seeds Stormwind City / Trade District. The four
    // zone probes should reflect that out of the box.
    let env = WowLuaEnv::new().unwrap();
    let (zone, sub, mini, real) = zone_text(&env);
    assert_eq!(zone, "Stormwind City");
    assert_eq!(sub, "Trade District");
    assert_eq!(
        mini, "Trade District",
        "minimap prefers sub-zone when populated"
    );
    assert_eq!(real, "Stormwind City");
}

#[test]
fn set_zone_updates_zone_and_real_zone() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetZone("Durotar", 14)"#).unwrap();
    env.exec(r#"A_Admin.SetSubZone("")"#).unwrap();
    let (zone, sub, mini, real) = zone_text(&env);
    assert_eq!(zone, "Durotar");
    assert_eq!(sub, "");
    assert_eq!(mini, "Durotar", "minimap falls back to zone when sub empty");
    assert_eq!(real, "Durotar", "not in instance → real matches zone");
}

#[test]
fn set_sub_zone_drives_subzone_and_minimap() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetZone("Durotar", 14)"#).unwrap();
    env.exec(r#"A_Admin.SetSubZone("Razor Hill")"#).unwrap();
    let (zone, sub, mini, real) = zone_text(&env);
    assert_eq!(zone, "Durotar");
    assert_eq!(sub, "Razor Hill");
    assert_eq!(
        mini, "Razor Hill",
        "minimap prefers sub-zone when populated"
    );
    assert_eq!(real, "Durotar");
}

#[test]
fn in_instance_real_zone_uses_instance_name() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetZone("Stranglethorn Vale", 33)"#)
        .unwrap();
    env.exec(r#"A_Admin.SetInstanceInfo("Deadmines", "party", 1, 5)"#)
        .unwrap();
    let (zone, _sub, _mini, real) = zone_text(&env);
    assert_eq!(zone, "Stranglethorn Vale");
    assert_eq!(
        real, "Deadmines",
        "GetRealZoneText should show the instance name while in_instance",
    );
}

#[test]
fn leaving_instance_restores_zone_as_real() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetZone("Durotar", 14)"#).unwrap();
    env.exec(r#"A_Admin.SetInstanceInfo("Ragefire Chasm", "party", 1, 5)"#)
        .unwrap();
    env.exec(r#"A_Admin.SetInInstance(false)"#).unwrap();
    let (_, _, _, real) = zone_text(&env);
    assert_eq!(
        real, "Durotar",
        "after leaving the instance, real zone should fall back to world zone",
    );
}

#[test]
fn clearing_sub_zone_restores_zone_to_minimap() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetZone("Durotar", 14)"#).unwrap();
    env.exec(r#"A_Admin.SetSubZone("Razor Hill")"#).unwrap();
    env.exec(r#"A_Admin.SetSubZone("")"#).unwrap();
    let (_, sub, mini, _) = zone_text(&env);
    assert_eq!(sub, "");
    assert_eq!(mini, "Durotar");
}

#[test]
fn zone_text_reflects_live_admin_changes() {
    // Consecutive admin updates should propagate through.
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"A_Admin.SetZone("Zone A", 1)"#).unwrap();
    let (a, _, _, _) = zone_text(&env);
    env.exec(r#"A_Admin.SetZone("Zone B", 2)"#).unwrap();
    let (b, _, _, _) = zone_text(&env);
    assert_eq!(a, "Zone A");
    assert_eq!(b, "Zone B");
}
