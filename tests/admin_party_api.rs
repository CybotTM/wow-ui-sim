//! Tests for A_Admin party simulation API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetPartySize
// ============================================================================

#[test]
fn test_set_party_size_nonzero_means_in_group() {
    let env = env();
    let in_group: bool = env
        .eval(
            r#"
            A_Admin.SetPartySize(3)
            return IsInGroup()
            "#,
        )
        .unwrap();
    assert!(in_group, "IsInGroup() should return true when party size > 0");
}

#[test]
fn test_set_party_size_zero_means_not_in_group() {
    let env = env();
    let in_group: bool = env
        .eval(
            r#"
            A_Admin.SetPartySize(0)
            return IsInGroup()
            "#,
        )
        .unwrap();
    assert!(!in_group, "IsInGroup() should return false when party size == 0");
}

#[test]
fn test_get_num_group_members_includes_player() {
    let env = env();
    // GetNumGroupMembers returns party count + 1 (for the player) when in a group.
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(3)
            return GetNumGroupMembers()
            "#,
        )
        .unwrap();
    assert_eq!(count, 4, "GetNumGroupMembers() should return party size + 1 (player)");
}

#[test]
fn test_get_num_group_members_zero_when_no_party() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(0)
            return GetNumGroupMembers()
            "#,
        )
        .unwrap();
    assert_eq!(count, 0, "GetNumGroupMembers() should return 0 when not in a group");
}

// ============================================================================
// SetPartyMember
// ============================================================================

#[test]
fn test_set_party_member_name_readable_via_unit_name() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMember(1, "Tank", 1, 80)
            return UnitName("party1")
            "#,
        )
        .unwrap();
    assert_eq!(name, "Tank");
}

#[test]
fn test_set_party_member_class_readable_via_unit_class() {
    let env = env();
    // class_index 2 = Paladin
    let (class_name, class_file, class_id): (String, String, i32) = env
        .eval(
            r#"
            A_Admin.SetPartySize(1)
            A_Admin.SetPartyMember(1, "Holydin", 2, 80)
            return UnitClass("party1")
            "#,
        )
        .unwrap();
    assert_eq!(class_name, "Paladin");
    assert_eq!(class_file, "PALADIN");
    assert_eq!(class_id, 2);
}

#[test]
fn test_set_party_member_level_readable_via_unit_level() {
    let env = env();
    let level: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(1)
            A_Admin.SetPartyMember(1, "Tanker", 1, 70)
            return UnitLevel("party1")
            "#,
        )
        .unwrap();
    assert_eq!(level, 70);
}

#[test]
fn test_set_party_member_second_slot() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMember(1, "Alpha", 1, 80)
            A_Admin.SetPartyMember(2, "Beta", 5, 80)
            return UnitName("party2")
            "#,
        )
        .unwrap();
    assert_eq!(name, "Beta");
}

#[test]
fn test_set_party_member_does_not_affect_other_slots() {
    let env = env();
    let name1: String = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMember(1, "Alpha", 1, 80)
            A_Admin.SetPartyMember(2, "Beta", 5, 80)
            return UnitName("party1")
            "#,
        )
        .unwrap();
    assert_eq!(name1, "Alpha");
}

// ============================================================================
// SetPartyMemberHealth
// ============================================================================

#[test]
fn test_set_party_member_health_current() {
    let env = env();
    let hp: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMemberHealth(1, 5000, 10000)
            return UnitHealth("party1")
            "#,
        )
        .unwrap();
    assert_eq!(hp, 5000);
}

#[test]
fn test_set_party_member_health_max() {
    let env = env();
    let hp_max: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMemberHealth(1, 5000, 10000)
            return UnitHealthMax("party1")
            "#,
        )
        .unwrap();
    assert_eq!(hp_max, 10000);
}

#[test]
fn test_set_party_member_health_second_member() {
    let env = env();
    let (hp, hp_max): (i32, i32) = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMemberHealth(1, 1000, 2000)
            A_Admin.SetPartyMemberHealth(2, 3000, 6000)
            return UnitHealth("party2"), UnitHealthMax("party2")
            "#,
        )
        .unwrap();
    assert_eq!(hp, 3000);
    assert_eq!(hp_max, 6000);
}

#[test]
fn test_set_party_member_health_does_not_affect_other_member() {
    let env = env();
    // Set health for member 2, member 1 retains its value.
    let hp1: i32 = env
        .eval(
            r#"
            A_Admin.SetPartySize(2)
            A_Admin.SetPartyMemberHealth(1, 9000, 10000)
            A_Admin.SetPartyMemberHealth(2, 3000, 6000)
            return UnitHealth("party1")
            "#,
        )
        .unwrap();
    assert_eq!(hp1, 9000);
}
