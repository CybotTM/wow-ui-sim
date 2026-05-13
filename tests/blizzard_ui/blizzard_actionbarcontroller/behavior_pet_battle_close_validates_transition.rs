//! Behavior pin: PET_BATTLE_CLOSE validates the active action bar transition.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn pet_battle_close_validates_stale_override_transition() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.has_override_action_bar = true;
            state.override_bar_skin = Some(1);
        }

        env.exec(
            r#"
            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "UPDATE_OVERRIDE_ACTIONBAR"
            )

            local originalValidateActionBarTransition = ValidateActionBarTransition
            _G.petBattleCloseValidationCalls = 0

            function ValidateActionBarTransition()
                _G.petBattleCloseValidationCalls =
                    _G.petBattleCloseValidationCalls + 1
                return originalValidateActionBarTransition()
            end

            MainActionBar:Show()
            OverrideActionBar:Hide()

            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "PET_BATTLE_CLOSE"
            )
            "#,
        )
        .expect("ActionBarController PET_BATTLE_CLOSE dispatch must run cleanly");

        let (validation_calls, state_is_override, main_bar_shown, override_bar_shown): (
            i32,
            bool,
            bool,
            bool,
        ) = env
            .eval(
                r#"
                return _G.petBattleCloseValidationCalls,
                    ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_OVERRIDE,
                    MainActionBar:IsShown(),
                    OverrideActionBar:IsShown()
                "#,
            )
            .expect("post PET_BATTLE_CLOSE transition probe must run cleanly");

        assert_eq!(
            validation_calls, 1,
            "PET_BATTLE_CLOSE must call ValidateActionBarTransition exactly once"
        );
        assert!(
            state_is_override,
            "test setup must leave CURRENT_ACTION_BAR_STATE stale at LE_ACTIONBAR_STATE_OVERRIDE"
        );
        assert!(
            !main_bar_shown,
            "PET_BATTLE_CLOSE validation must hide MainActionBar for stale override state"
        );
        assert!(
            override_bar_shown,
            "PET_BATTLE_CLOSE validation must show OverrideActionBar for stale override state"
        );
    });
    }
}
