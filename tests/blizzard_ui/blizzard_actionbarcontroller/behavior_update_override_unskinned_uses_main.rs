//! Behavior pin: unskinned override bars reuse the main action bar.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";
const SEEDED_OVERRIDE_BAR_INDEX: i32 = 5;

#[test]
fn update_override_actionbar_unskinned_routes_to_main_bar_page() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.has_override_action_bar = true;
            state.override_bar_skin = Some(0);
            state.has_vehicle_action_bar = false;
            state.override_bar_index = SEEDED_OVERRIDE_BAR_INDEX;
        }

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

        let (state_is_main, action_page, override_index): (bool, i32, i32) = env
            .eval(
                r#"
                return ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_MAIN,
                    MainActionBar:GetAttribute("actionpage"),
                    C_ActionBar.GetOverrideBarIndex()
                "#,
            )
            .expect("post unskinned override actionpage probe must run cleanly");

        assert!(
            state_is_main,
            "unskinned override state must keep CURRENT_ACTION_BAR_STATE at \
             LE_ACTIONBAR_STATE_MAIN"
        );
        assert_eq!(
            action_page, override_index,
            "unskinned override state must route MainActionBar actionpage through \
             C_ActionBar.GetOverrideBarIndex()"
        );
    });
    }
}
