//! Tests for A_Admin targeting API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetTarget / ClearTarget
// ============================================================================

#[test]
fn test_set_target_unit_exists() {
    let env = env();
    let exists: bool = env
        .eval(
            r#"
            A_Admin.SetTarget("Boss", 63, 1, true)
            return UnitExists("target")
            "#,
        )
        .unwrap();
    assert!(exists);
}

#[test]
fn test_set_target_unit_name() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            A_Admin.SetTarget("Kyveza", 63, 1, true)
            return UnitName("target")
            "#,
        )
        .unwrap();
    assert_eq!(name, "Kyveza");
}

#[test]
fn test_set_target_unit_level() {
    let env = env();
    let level: i32 = env
        .eval(
            r#"
            A_Admin.SetTarget("Boss", 63, 1, true)
            return UnitLevel("target")
            "#,
        )
        .unwrap();
    assert_eq!(level, 63);
}

#[test]
fn test_set_target_friendly_unit() {
    let env = env();
    let (exists, level): (bool, i32) = env
        .eval(
            r#"
            A_Admin.SetTarget("Healbot", 80, 5, false)
            return UnitExists("target"), UnitLevel("target")
            "#,
        )
        .unwrap();
    assert!(exists);
    assert_eq!(level, 80);
}

#[test]
fn test_clear_target_removes_unit() {
    let env = env();
    let exists: bool = env
        .eval(
            r#"
            A_Admin.SetTarget("Boss", 63, 1, true)
            A_Admin.ClearTarget()
            return UnitExists("target")
            "#,
        )
        .unwrap();
    assert!(!exists);
}

#[test]
fn test_clear_target_without_target_is_noop() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.ClearTarget()
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

// ============================================================================
// SetFocus / ClearFocus
// ============================================================================

#[test]
fn test_set_focus_is_noop_safe() {
    let env = env();
    // SetFocus sets internal state; no crash expected
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetFocus("Flanker", 50, 2, true)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

#[test]
fn test_clear_focus_is_noop_safe() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetFocus("Flanker", 50, 2, true)
            A_Admin.ClearFocus()
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

// ============================================================================
// SetTargetPower
// ============================================================================

#[test]
fn test_set_target_power_current() {
    let env = env();
    let power: i32 = env
        .eval(
            r#"
            A_Admin.SetTarget("Boss", 63, 1, true)
            A_Admin.SetTargetPower(3000, 5000)
            return UnitPower("target")
            "#,
        )
        .unwrap();
    assert_eq!(power, 3000);
}

#[test]
fn test_set_target_power_max() {
    let env = env();
    let power_max: i32 = env
        .eval(
            r#"
            A_Admin.SetTarget("Boss", 63, 1, true)
            A_Admin.SetTargetPower(3000, 5000)
            return UnitPowerMax("target")
            "#,
        )
        .unwrap();
    assert_eq!(power_max, 5000);
}

#[test]
fn test_set_target_power_silently_ignored_without_target() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetTargetPower(3000, 5000)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

// ============================================================================
// SetFocusPower
// ============================================================================

#[test]
fn test_set_focus_power_silently_ignored_without_focus() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetFocusPower(2000, 4000)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

#[test]
fn test_set_focus_power_after_set_focus() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetFocus("Flanker", 50, 2, true)
            A_Admin.SetFocusPower(2000, 4000)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

// ============================================================================
// SetTargetType
// ============================================================================

#[test]
fn test_set_target_type_creature_type() {
    let env = env();
    let creature_type: String = env
        .eval(
            r#"
            A_Admin.SetTarget("Boss", 63, 1, true)
            A_Admin.SetTargetType(nil, "Demon", nil)
            return UnitCreatureType("target")
            "#,
        )
        .unwrap();
    assert_eq!(creature_type, "Demon");
}

#[test]
fn test_set_target_type_undead() {
    let env = env();
    let creature_type: String = env
        .eval(
            r#"
            A_Admin.SetTarget("Skeleton", 40, 1, true)
            A_Admin.SetTargetType(nil, "Undead", nil)
            return UnitCreatureType("target")
            "#,
        )
        .unwrap();
    assert_eq!(creature_type, "Undead");
}

#[test]
fn test_set_target_type_classification() {
    let env = env();
    let classification: String = env
        .eval(
            r#"
            A_Admin.SetTarget("Rareboss", 63, 1, true)
            A_Admin.SetTargetType("elite", nil, nil)
            return UnitClassification("target")
            "#,
        )
        .unwrap();
    assert_eq!(classification, "elite");
}

#[test]
fn test_set_target_type_silently_ignored_without_target() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetTargetType(nil, "Demon", nil)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

// ============================================================================
// SetFocusType
// ============================================================================

#[test]
fn test_set_focus_type_creature_type() {
    let env = env();
    let creature_type: String = env
        .eval(
            r#"
            A_Admin.SetFocus("Imp", 10, 9, true)
            A_Admin.SetFocusType(nil, "Demon", nil)
            return UnitCreatureType("focus")
            "#,
        )
        .unwrap();
    assert_eq!(creature_type, "Demon");
}

#[test]
fn test_set_focus_type_silently_ignored_without_focus() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetFocusType(nil, "Beast", nil)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

// ============================================================================
// SetFocusHealth
// ============================================================================

#[test]
fn test_set_focus_health_after_set_focus() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetFocus("Tank", 80, 2, false)
            A_Admin.SetFocusHealth(60000, 120000)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}

#[test]
fn test_set_focus_health_silently_ignored_without_focus() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            A_Admin.SetFocusHealth(60000, 120000)
            return true
            "#,
        )
        .unwrap();
    assert!(ok);
}
