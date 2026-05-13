//! Behavior pin: ActionBarController_UpdateAll fans out to button event frames.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn update_all_forces_each_action_bar_button_event_frame() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.has_bonus_action_bar = true;
            state.action_bar_page = 1;
            state.bonus_bar_index = 5;
        }

        env.exec(
            r#"
            _G.updateAllEventFrameCalls = 0
            _G.updateAllEventFrameForceValues = {}

            local firstFrame = {}
            function firstFrame:UpdateAction(force)
                _G.updateAllEventFrameCalls = _G.updateAllEventFrameCalls + 1
                table.insert(_G.updateAllEventFrameForceValues, force)
            end

            local secondFrame = {}
            function secondFrame:UpdateAction(force)
                _G.updateAllEventFrameCalls = _G.updateAllEventFrameCalls + 1
                table.insert(_G.updateAllEventFrameForceValues, force)
            end

            ActionBarButtonEventsFrame.frames = {
                First = firstFrame,
                Second = secondFrame,
            }

            ActionBarController_UpdateAll(true)
            "#,
        )
        .expect("ActionBarController_UpdateAll(true) must run cleanly");

        let (calls, first_force, second_force): (i32, bool, bool) = env
            .eval(
                r#"
                return _G.updateAllEventFrameCalls,
                    _G.updateAllEventFrameForceValues[1],
                    _G.updateAllEventFrameForceValues[2]
                "#,
            )
            .expect("post ActionBarController_UpdateAll force probe must run cleanly");

        assert_eq!(
            calls, 2,
            "ActionBarController_UpdateAll must update every button event frame"
        );
        assert!(
            first_force,
            "ActionBarController_UpdateAll(true) must pass true to the first frame"
        );
        assert!(
            second_force,
            "ActionBarController_UpdateAll(true) must pass true to the second frame"
        );
    });
    }
}
