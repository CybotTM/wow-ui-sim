//! Tests for A_Admin zone and instance API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetZone
// ============================================================================

#[test]
fn test_set_zone_updates_get_zone_text() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.SetZone("Stormwind City", 1519)
            return GetZoneText()
            "#,
        )
        .unwrap();
    assert_eq!(name, "Stormwind City");
}

#[test]
fn test_set_zone_updates_get_real_zone_text() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.SetZone("Stormwind City", 1519)
            return GetRealZoneText()
            "#,
        )
        .unwrap();
    assert_eq!(name, "Stormwind City");
}

#[test]
fn test_set_zone_both_zone_functions_agree() {
    let env = env();
    let (zone, real): (String, String) = env
        .eval(
            r#"
            A_Admin.SetZone("Durotar", 14)
            return GetZoneText(), GetRealZoneText()
            "#,
        )
        .unwrap();
    assert_eq!(zone, "Durotar");
    assert_eq!(real, "Durotar");
}

// ============================================================================
// SetSubZone
// ============================================================================

#[test]
fn test_set_sub_zone_updates_get_sub_zone_text() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.SetSubZone("Trade District")
            return GetSubZoneText()
            "#,
        )
        .unwrap();
    assert_eq!(name, "Trade District");
}

#[test]
fn test_set_sub_zone_independent_of_zone() {
    let env = env();
    let (zone, sub): (String, String) = env
        .eval(
            r#"
            A_Admin.SetZone("Stormwind City", 1519)
            A_Admin.SetSubZone("Old Town")
            return GetZoneText(), GetSubZoneText()
            "#,
        )
        .unwrap();
    assert_eq!(zone, "Stormwind City");
    assert_eq!(sub, "Old Town");
}

// ============================================================================
// SetInstanceInfo
// ============================================================================

#[test]
fn test_set_instance_info_get_instance_info_name() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.SetInstanceInfo("The Necrotic Wake", "party", 23, 5)
            local n = GetInstanceInfo()
            return n
            "#,
        )
        .unwrap();
    assert_eq!(name, "The Necrotic Wake");
}

#[test]
fn test_set_instance_info_get_instance_info_type() {
    let env = env();
    let itype: String = env
        .eval(
            r#"
            A_Admin.SetInstanceInfo("The Necrotic Wake", "party", 23, 5)
            local name, itype = GetInstanceInfo()
            return itype
            "#,
        )
        .unwrap();
    assert_eq!(itype, "party");
}

#[test]
fn test_set_instance_info_sets_in_instance_true() {
    let env = env();
    let in_inst: bool = env
        .eval(
            r#"
            A_Admin.SetInstanceInfo("The Necrotic Wake", "party", 23, 5)
            local inInst = IsInInstance()
            return inInst
            "#,
        )
        .unwrap();
    assert!(in_inst);
}

#[test]
fn test_set_instance_info_difficulty_id() {
    let env = env();
    let diff_id: i32 = env
        .eval(
            r#"
            A_Admin.SetInstanceInfo("Vault of the Incarnates", "raid", 16, 20)
            local name, itype, diffId = GetInstanceInfo()
            return diffId
            "#,
        )
        .unwrap();
    assert_eq!(diff_id, 16);
}

// ============================================================================
// SetInInstance
// ============================================================================

#[test]
fn test_set_in_instance_false_overrides_instance_info() {
    let env = env();
    let in_inst: bool = env
        .eval(
            r#"
            A_Admin.SetInstanceInfo("Dungeon", "party", 23, 5)
            A_Admin.SetInInstance(false)
            local inInst = IsInInstance()
            return inInst
            "#,
        )
        .unwrap();
    assert!(!in_inst);
}

#[test]
fn test_set_in_instance_true() {
    let env = env();
    let in_inst: bool = env
        .eval(
            r#"
            A_Admin.SetInInstance(true)
            local inInst = IsInInstance()
            return inInst
            "#,
        )
        .unwrap();
    assert!(in_inst);
}
