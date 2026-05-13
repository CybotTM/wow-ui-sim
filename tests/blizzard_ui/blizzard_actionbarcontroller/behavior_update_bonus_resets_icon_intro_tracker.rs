//! Behavior pin: UPDATE_BONUS_ACTIONBAR resets icon intro tracking.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";
const SEEDED_BONUS_BAR_INDEX: i32 = 5;

#[test]
fn update_bonus_actionbar_updates_all_and_resets_icon_intro_tracker() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.has_bonus_action_bar = true;
            state.action_bar_page = 1;
            state.bonus_bar_index = SEEDED_BONUS_BAR_INDEX;
        }

        env.exec(
            r#"
            _G.bonusIconIntroResetCalls = 0

            function IconIntroTracker:ResetAll()
                _G.bonusIconIntroResetCalls = _G.bonusIconIntroResetCalls + 1
            end

            MainActionBar:SetAttribute("actionpage", 99)
            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "UPDATE_BONUS_ACTIONBAR"
            )
            "#,
        )
        .expect("ActionBarController UPDATE_BONUS_ACTIONBAR dispatch must run cleanly");

        let (reset_calls, action_page, bonus_index): (i32, i32, i32) = env
            .eval(
                r#"
                return _G.bonusIconIntroResetCalls,
                    MainActionBar:GetAttribute("actionpage"),
                    C_ActionBar.GetBonusBarIndex()
                "#,
            )
            .expect("post UPDATE_BONUS_ACTIONBAR reset probe must run cleanly");

        assert_eq!(
            action_page, bonus_index,
            "UPDATE_BONUS_ACTIONBAR must run ActionBarController_UpdateAll"
        );
        assert_eq!(
            reset_calls, 1,
            "UPDATE_BONUS_ACTIONBAR must call IconIntroTracker:ResetAll exactly once"
        );
    });
    }
}
