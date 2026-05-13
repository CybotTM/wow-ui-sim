//! Behavior pin: unskinned vehicle bars reuse the main action bar.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";
const SEEDED_VEHICLE_BAR_INDEX: i32 = 7;

#[test]
fn update_vehicle_actionbar_unskinned_routes_to_main_bar_page() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.has_vehicle_action_bar = true;
            state.player.vehicle_skin = None;
            state.vehicle_bar_index = SEEDED_VEHICLE_BAR_INDEX;
        }

        env.exec(
            r#"
            MainActionBar:SetAttribute("actionpage", 99)
            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "UPDATE_VEHICLE_ACTIONBAR"
            )
            "#,
        )
        .expect("ActionBarController UPDATE_VEHICLE_ACTIONBAR dispatch must run cleanly");

        let (state_is_main, action_page, vehicle_index): (bool, i32, i32) = env
            .eval(
                r#"
                return ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_MAIN,
                    MainActionBar:GetAttribute("actionpage"),
                    C_ActionBar.GetVehicleBarIndex()
                "#,
            )
            .expect("post unskinned vehicle actionpage probe must run cleanly");

        assert!(
            state_is_main,
            "unskinned vehicle state must keep CURRENT_ACTION_BAR_STATE at \
             LE_ACTIONBAR_STATE_MAIN"
        );
        assert_eq!(
            action_page, vehicle_index,
            "unskinned vehicle state must route MainActionBar actionpage through \
             C_ActionBar.GetVehicleBarIndex()"
        );
    });
    }
}
