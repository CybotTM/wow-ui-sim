//! Behavior pin: no override preconditions routes through ResetToDefault.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";
const SEEDED_ACTION_BAR_PAGE: u32 = 4;

#[test]
fn update_override_actionbar_without_overrides_resets_to_default_page() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        clear_override_preconditions(env);

        env.exec(
            r#"
            MainActionBar:SetAttribute("actionpage", 99)
            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "UPDATE_OVERRIDE_ACTIONBAR"
            )
            "#,
        )
        .expect("ActionBarController UPDATE_OVERRIDE_ACTIONBAR dispatch must run cleanly");

        let (state_is_main, action_page, current_page): (bool, i32, i32) = env
            .eval(
                r#"
                return ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_MAIN,
                    MainActionBar:GetAttribute("actionpage"),
                    C_ActionBar.GetActionBarPage()
                "#,
            )
            .expect("post reset-to-default actionpage probe must run cleanly");

        assert!(
            state_is_main,
            "cleared override state must keep CURRENT_ACTION_BAR_STATE at \
             LE_ACTIONBAR_STATE_MAIN"
        );
        assert_eq!(
            action_page, current_page,
            "ActionBarController_ResetToDefault must restore MainActionBar actionpage \
             from C_ActionBar.GetActionBarPage()"
        );
    });
    }
}

fn clear_override_preconditions(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.action_bar_page = SEEDED_ACTION_BAR_PAGE;
    state.has_override_action_bar = false;
    state.override_bar_skin = None;
    state.has_vehicle_action_bar = false;
    state.player.vehicle_skin = None;
    state.has_temp_shapeshift_action_bar = false;
    state.has_bonus_action_bar = false;
    state.pet_battles.battle_state = 0;
}
