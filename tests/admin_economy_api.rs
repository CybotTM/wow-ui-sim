//! Tests for A_Admin economy API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetMoney
// ============================================================================

#[test]
fn test_set_money_get_money_returns_copper() {
    let env = env();
    let money: i64 = env
        .eval(
            r#"
            A_Admin.SetMoney(1234567)
            return GetMoney()
            "#,
        )
        .unwrap();
    assert_eq!(money, 1234567);
}

#[test]
fn test_set_money_zero() {
    let env = env();
    let money: i64 = env
        .eval(
            r#"
            A_Admin.SetMoney(0)
            return GetMoney()
            "#,
        )
        .unwrap();
    assert_eq!(money, 0);
}

#[test]
fn test_set_money_large_value() {
    let env = env();
    let money: i64 = env
        .eval(
            r#"
            A_Admin.SetMoney(9999999999)
            return GetMoney()
            "#,
        )
        .unwrap();
    assert_eq!(money, 9_999_999_999);
}

#[test]
fn test_set_money_overwrites_previous() {
    let env = env();
    let money: i64 = env
        .eval(
            r#"
            A_Admin.SetMoney(5000000)
            A_Admin.SetMoney(100)
            return GetMoney()
            "#,
        )
        .unwrap();
    assert_eq!(money, 100);
}

// ============================================================================
// SetItemLevel
// ============================================================================

#[test]
fn test_set_item_level_get_average_item_level_overall() {
    let env = env();
    let overall: f64 = env
        .eval(
            r#"
            A_Admin.SetItemLevel(489.5)
            local overall, equipped, pvp = GetAverageItemLevel()
            return overall
            "#,
        )
        .unwrap();
    assert!((overall - 489.5).abs() < 0.1, "expected ~489.5, got {}", overall);
}

#[test]
fn test_set_item_level_all_three_values_match() {
    let env = env();
    let (overall, equipped, pvp): (f64, f64, f64) = env
        .eval(
            r#"
            A_Admin.SetItemLevel(512.0)
            return GetAverageItemLevel()
            "#,
        )
        .unwrap();
    assert!((overall - 512.0).abs() < 0.1, "overall mismatch: {}", overall);
    assert!((equipped - 512.0).abs() < 0.1, "equipped mismatch: {}", equipped);
    assert!((pvp - 512.0).abs() < 0.1, "pvp mismatch: {}", pvp);
}

#[test]
fn test_set_item_level_integer_value() {
    let env = env();
    let overall: f64 = env
        .eval(
            r#"
            A_Admin.SetItemLevel(639)
            local overall = GetAverageItemLevel()
            return overall
            "#,
        )
        .unwrap();
    assert!((overall - 639.0).abs() < 0.1, "expected ~639.0, got {}", overall);
}

#[test]
fn test_set_item_level_overwrites_previous() {
    let env = env();
    let overall: f64 = env
        .eval(
            r#"
            A_Admin.SetItemLevel(400.0)
            A_Admin.SetItemLevel(600.0)
            local overall = GetAverageItemLevel()
            return overall
            "#,
        )
        .unwrap();
    assert!((overall - 600.0).abs() < 0.1, "expected ~600.0, got {}", overall);
}
