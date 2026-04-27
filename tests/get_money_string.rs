//! Integration tests for the `GetMoneyString` global installed by
//! `runtime_surface_bootstrap.lua`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

#[test]
fn formats_full_gold_silver_copper() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(1234567)").unwrap();
    assert_eq!(s, "123g 45s 67c");
}

#[test]
fn pure_copper_returns_copper_only() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(42)").unwrap();
    assert_eq!(s, "42c");
}

#[test]
fn pure_silver_returns_silver_only() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(500)").unwrap();
    assert_eq!(s, "5s");
}

#[test]
fn pure_gold_returns_gold_only() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(70000)").unwrap();
    assert_eq!(s, "7g");
}

#[test]
fn zero_money_returns_zero_copper() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(0)").unwrap();
    assert_eq!(s, "0c");
}

#[test]
fn elides_zero_silver_segment() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(10005)").unwrap();
    assert_eq!(s, "1g 5c");
}

#[test]
fn elides_zero_copper_segment() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(10500)").unwrap();
    assert_eq!(s, "1g 5s");
}

#[test]
fn separate_thousands_inserts_commas_in_gold() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(123456789, true)").unwrap();
    assert_eq!(s, "12,345g 67s 89c");
}

#[test]
fn separate_thousands_omitted_for_small_gold() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(1234567, true)").unwrap();
    assert_eq!(s, "123g 45s 67c");
}

#[test]
fn handles_nil_input_as_zero() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(nil)").unwrap();
    assert_eq!(s, "0c");
}

#[test]
fn negative_input_is_clamped_to_zero() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(-500)").unwrap();
    assert_eq!(s, "0c");
}

#[test]
fn fractional_money_is_floored() {
    let env = env();
    let s: String = env.eval("return GetMoneyString(105.9)").unwrap();
    assert_eq!(s, "1s 5c");
}
