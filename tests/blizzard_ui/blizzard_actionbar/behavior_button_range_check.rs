//! Behavior pin: action range-check events tint registered button hotkeys.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use rilua::Val;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";
const ACTION_SLOT: f64 = 1.0;

#[test]
fn range_check_event_updates_registered_button_hotkey_color() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        register_action_button_for_range_checks(env);

        fire_range_check_update(env, false, true);
        assert_hotkey_matches_red_font_color(env);

        fire_range_check_update(env, true, true);
        assert_hotkey_matches_actionbar_font_color(env);

        fire_range_check_update(env, false, false);
        assert_hotkey_matches_actionbar_font_color(env);
    });
    }
}

fn register_action_button_for_range_checks(env: &WowLuaEnv) {
    env.exec(
        r#"
        ActionButton1.HotKey:SetText("1")
        ActionButton1.HotKey:Show()
        ActionBarButtonRangeCheckFrame:RegisterFrame(1, ActionButton1)
        "#,
    )
    .expect("range-check registration fixture must run cleanly");
}

fn fire_range_check_update(env: &WowLuaEnv, in_range: bool, checks_range: bool) {
    env.fire_event_with_args(
        "ACTION_RANGE_CHECK_UPDATE",
        &[
            Val::Num(ACTION_SLOT),
            Val::Bool(in_range),
            Val::Bool(checks_range),
        ],
    )
    .expect("ACTION_RANGE_CHECK_UPDATE must dispatch cleanly");
}

fn assert_hotkey_matches_red_font_color(env: &WowLuaEnv) {
    let (r, g, b, expected_r, expected_g, expected_b): (f64, f64, f64, f64, f64, f64) = env
        .eval(
            r#"
            local r, g, b = ActionButton1.HotKey:GetVertexColor()
            local er, eg, eb = RED_FONT_COLOR:GetRGB()
            return r, g, b, er, eg, eb
            "#,
        )
        .expect("red hotkey color probe must run cleanly");
    assert_color_matches(
        (r, g, b),
        (expected_r, expected_g, expected_b),
        "out-of-range action must tint the hotkey red",
    );
}

fn assert_hotkey_matches_actionbar_font_color(env: &WowLuaEnv) {
    let (r, g, b, expected_r, expected_g, expected_b): (f64, f64, f64, f64, f64, f64) = env
        .eval(
            r#"
            local r, g, b = ActionButton1.HotKey:GetVertexColor()
            local er, eg, eb = ACTIONBAR_HOTKEY_FONT_COLOR:GetRGB()
            return r, g, b, er, eg, eb
            "#,
        )
        .expect("normal hotkey color probe must run cleanly");
    assert_color_matches(
        (r, g, b),
        (expected_r, expected_g, expected_b),
        "in-range or unchecked action must restore the actionbar hotkey color",
    );
}

fn assert_color_matches(actual: (f64, f64, f64), expected: (f64, f64, f64), context: &str) {
    let epsilon = 0.001;
    let matches = (actual.0 - expected.0).abs() < epsilon
        && (actual.1 - expected.1).abs() < epsilon
        && (actual.2 - expected.2).abs() < epsilon;
    assert!(
        matches,
        "{context}: expected rgb {:?}, got {:?}",
        expected, actual
    );
}
