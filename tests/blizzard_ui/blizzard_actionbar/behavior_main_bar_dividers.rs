//! Behavior pin: main-bar dividers appear only between adjacent visible buttons.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use rilua::Val;

const ROOT: &str = "Blizzard_ActionBar";
const LEFT_SLOT: u32 = 6;
const RIGHT_SLOT: u32 = 7;
const SPELL_ID: u32 = 853;

#[test]
fn main_bar_divider_between_six_and_seven_requires_both_buttons() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        clear_main_bar_actions_and_grid(env);
        assert_center_divider_count(env, 0, "empty bar has no dividers");

        seed_slot(env, LEFT_SLOT);
        refresh_main_bar(env);
        assert_center_buttons(env, true, false);
        assert_center_divider_count(env, 0, "left half alone has no center divider");

        seed_slot(env, RIGHT_SLOT);
        refresh_main_bar(env);
        assert_center_buttons(env, true, true);
        assert_center_divider_count(env, 1, "adjacent populated halves show center divider");

        clear_slot(env, LEFT_SLOT);
        refresh_main_bar(env);
        assert_center_buttons(env, false, true);
        assert_center_divider_count(env, 0, "right half alone hides center divider");
    });
    }
}

fn clear_main_bar_actions_and_grid(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.state().borrow_mut().action_bars.clear();
    env.exec(
        r#"
        for _, button in pairs(MainActionBar.actionButtons) do
            button:SetAttribute("showgrid", 0)
        end
        MainActionBar:UpdateShownButtons()
        "#,
    )
    .expect("main bar clear fixture must run cleanly");
}

fn seed_slot(env: &wow_ui_sim::lua_api::WowLuaEnv, slot: u32) {
    env.state().borrow_mut().action_bars.insert(slot, SPELL_ID);
    env.fire_event_with_args("ACTIONBAR_SLOT_CHANGED", &[Val::Num(slot as f64)])
        .expect("ACTIONBAR_SLOT_CHANGED must dispatch cleanly");
}

fn clear_slot(env: &wow_ui_sim::lua_api::WowLuaEnv, slot: u32) {
    env.state().borrow_mut().action_bars.remove(&slot);
    env.fire_event_with_args("ACTIONBAR_SLOT_CHANGED", &[Val::Num(slot as f64)])
        .expect("ACTIONBAR_SLOT_CHANGED must dispatch cleanly");
}

fn refresh_main_bar(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.eval::<()>("MainActionBar:UpdateShownButtons()")
        .expect("MainActionBar:UpdateShownButtons must run cleanly");
}

fn assert_center_buttons(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    left_shown: bool,
    right_shown: bool,
) {
    let (actual_left, actual_right): (bool, bool) = env
        .eval("return ActionButton6:IsShown(), ActionButton7:IsShown()")
        .expect("center button visibility probe must run cleanly");
    assert_eq!(actual_left, left_shown, "ActionButton6 visibility mismatch");
    assert_eq!(
        actual_right, right_shown,
        "ActionButton7 visibility mismatch"
    );
}

fn assert_center_divider_count(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    expected_count: i32,
    context: &str,
) {
    let count: i32 = env
        .eval(
            r#"
            return MainActionBar.HorizontalDividersPool
                and MainActionBar.HorizontalDividersPool:GetNumActive()
                or 0
            "#,
        )
        .expect("main bar divider count probe must run cleanly");
    assert_eq!(count, expected_count, "{context}");
}
