//! Behavior pin: temporary shapeshift bars reuse the main action bar.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";
const SEEDED_TEMP_SHAPESHIFT_BAR_INDEX: i32 = 9;

#[test]
fn update_temp_shapeshift_actionbar_routes_to_main_bar_page() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.has_temp_shapeshift_action_bar = true;
            state.temp_shapeshift_bar_index = SEEDED_TEMP_SHAPESHIFT_BAR_INDEX;
        }

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

        let (state_is_main, action_page, temp_index): (bool, i32, i32) = env
            .eval(
                r#"
                return ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_MAIN,
                    MainActionBar:GetAttribute("actionpage"),
                    C_ActionBar.GetTempShapeshiftBarIndex()
                "#,
            )
            .expect("post temp shapeshift actionpage probe must run cleanly");

        assert!(
            state_is_main,
            "temp shapeshift state must keep CURRENT_ACTION_BAR_STATE at \
             LE_ACTIONBAR_STATE_MAIN"
        );
        assert_eq!(
            action_page, temp_index,
            "temp shapeshift state must route MainActionBar actionpage through \
             C_ActionBar.GetTempShapeshiftBarIndex()"
        );
    });
    }
}
