//! Behavior pin: bonus bars use the bonus page only from main page one.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";
const SEEDED_BONUS_BAR_INDEX: i32 = 5;

#[test]
fn update_bonus_actionbar_uses_bonus_index_only_when_page_one() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_bonus_bar(env, 1);
        dispatch_bonus_actionbar_update(env);
        assert_main_bar_actionpage(env, SEEDED_BONUS_BAR_INDEX);

        seed_bonus_bar(env, 2);
        dispatch_bonus_actionbar_update(env);
        assert_main_bar_actionpage(env, 2);
    });
    }
}

fn seed_bonus_bar(env: &wow_ui_sim::lua_api::WowLuaEnv, action_bar_page: u32) {
    let mut state = env.state().borrow_mut();
    state.has_bonus_action_bar = true;
    state.action_bar_page = action_bar_page;
    state.bonus_bar_index = SEEDED_BONUS_BAR_INDEX;
}

fn dispatch_bonus_actionbar_update(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        MainActionBar:SetAttribute("actionpage", 99)
        ActionBarController:GetScript("OnEvent")(
            ActionBarController,
            "UPDATE_BONUS_ACTIONBAR"
        )
        "#,
    )
    .expect("ActionBarController UPDATE_BONUS_ACTIONBAR dispatch must run cleanly");
}

fn assert_main_bar_actionpage(env: &wow_ui_sim::lua_api::WowLuaEnv, expected_page: i32) {
    let (state_is_main, action_page, bonus_index, current_page): (bool, i32, i32, i32) = env
        .eval(
            r#"
            return ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_MAIN,
                MainActionBar:GetAttribute("actionpage"),
                C_ActionBar.GetBonusBarIndex(),
                C_ActionBar.GetActionBarPage()
            "#,
        )
        .expect("post bonus actionpage probe must run cleanly");

    assert!(
        state_is_main,
        "bonus action bar state must keep CURRENT_ACTION_BAR_STATE at \
         LE_ACTIONBAR_STATE_MAIN"
    );
    assert_eq!(
        action_page, expected_page,
        "bonus branch chose wrong actionpage; bonus_index={bonus_index}, \
         current_page={current_page}"
    );
}
