//! Integration tests for the global `GetColorForCurrencyReward` shim
//! installed by `runtime_surface_bootstrap.lua`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

#[test]
fn returns_white_when_no_default_color_passed() {
    let env = env();
    let (r, g, b): (f64, f64, f64) = env
        .eval("return GetColorForCurrencyReward(2245, 35):GetRGB()")
        .unwrap();
    assert_eq!((r, g, b), (1.0, 1.0, 1.0));
}

#[test]
fn returns_default_color_when_one_is_provided() {
    let env = env();
    let (r, g, b): (f64, f64, f64) = env
        .eval(
            r#"
            local override = CreateColor(0.25, 0.5, 0.75, 1)
            return GetColorForCurrencyReward(2245, 1, override):GetRGB()
            "#,
        )
        .unwrap();
    assert_eq!((r, g, b), (0.25, 0.5, 0.75));
}

#[test]
fn return_is_callable_with_get_rgba() {
    let env = env();
    let (r, g, b, a): (f64, f64, f64, f64) = env
        .eval("return GetColorForCurrencyReward(2245, 0):GetRGBA()")
        .unwrap();
    assert_eq!((r, g, b, a), (1.0, 1.0, 1.0, 1.0));
}

#[test]
fn returned_value_is_a_table() {
    let env = env();
    let returned_type: String = env
        .eval("return type(GetColorForCurrencyReward(2245, 0))")
        .unwrap();
    assert_eq!(returned_type, "table");
}

#[test]
fn handles_zero_quantity_without_error() {
    let env = env();
    let r: f64 = env
        .eval("local c = GetColorForCurrencyReward(2245, 0); return ({c:GetRGB()})[1]")
        .unwrap();
    assert_eq!(r, 1.0);
}

#[test]
fn handles_unknown_currency_id_without_error() {
    let env = env();
    let r: f64 = env
        .eval("local c = GetColorForCurrencyReward(99999, 7); return ({c:GetRGB()})[1]")
        .unwrap();
    assert_eq!(r, 1.0);
}

#[test]
fn default_color_is_returned_unchanged_for_red() {
    let env = env();
    let (r, g, b): (f64, f64, f64) = env
        .eval(
            r#"
            local red = CreateColor(1, 0, 0, 1)
            return GetColorForCurrencyReward(2245, 9999, red):GetRGB()
            "#,
        )
        .unwrap();
    assert_eq!((r, g, b), (1.0, 0.0, 0.0));
}
