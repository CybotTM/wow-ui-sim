//! Behavior pin: default startup stays in the main action-bar state.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn startup_defaults_to_main_action_bar_state_with_main_bar_shown() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let current_state_is_main: bool = env
            .eval(
                "return ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_MAIN",
            )
            .expect("current action bar state probe must run cleanly");

        assert!(
            current_state_is_main,
            "startup without override, vehicle, or possess state must leave \
             ActionBarController in LE_ACTIONBAR_STATE_MAIN"
        );

        let main_bar_shown: bool = env
            .eval("return MainActionBar:IsShown()")
            .expect("MainActionBar visibility probe must run cleanly");

        assert!(
            main_bar_shown,
            "startup without override, vehicle, or possess state must show MainActionBar"
        );
    });
    }
}
