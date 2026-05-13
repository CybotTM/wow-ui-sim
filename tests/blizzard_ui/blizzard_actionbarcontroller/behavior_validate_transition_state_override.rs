//! Behavior pin: override-state validation restores the override bar.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn validate_transition_override_state_hides_main_stance_and_slides_in_override() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.has_override_action_bar = true;
            state.override_bar_skin = Some(1);
        }

        env.exec(
            r#"
            _G.overrideStateSlideInCalls = 0
            _G.overrideStateSlideInArgWasOne = false

            function OverrideActionBar:UpdateSkin()
            end

            function OverrideActionBar.slideOut:Play(animIn)
                _G.overrideStateSlideInCalls =
                    _G.overrideStateSlideInCalls + 1
                _G.overrideStateSlideInArgWasOne = animIn == 1
            end

            function MultiActionBar_Update()
            end

            function UIParent_ManageFramePositions()
            end

            ActionBarController_UpdateAll()

            _G.overrideStateAfterUpdate =
                ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_OVERRIDE

            MainActionBar:Show()
            StanceBar:Show()
            OverrideActionBar:Hide()
            OverrideActionBar.hideOnFinish = true
            _G.overrideStateSlideInCalls = 0
            _G.overrideStateSlideInArgWasOne = false

            ValidateActionBarTransition()

            _G.mainShownAfterOverrideValidation = MainActionBar:IsShown()
            _G.stanceShownAfterOverrideValidation = StanceBar:IsShown()
            _G.overrideShownAfterOverrideValidation = OverrideActionBar:IsShown()
            _G.overrideHideOnFinishAfterSlideIn = OverrideActionBar.hideOnFinish
            "#,
        )
        .expect("override-state ValidateActionBarTransition probe must run cleanly");

        let (
            state_is_override,
            main_shown,
            stance_shown,
            override_shown,
            slide_in_calls,
            slide_in_arg_was_one,
            override_hide_on_finish,
        ): (bool, bool, bool, bool, i32, bool, bool) = env
            .eval(
                r#"
                return _G.overrideStateAfterUpdate,
                    _G.mainShownAfterOverrideValidation,
                    _G.stanceShownAfterOverrideValidation,
                    _G.overrideShownAfterOverrideValidation,
                    _G.overrideStateSlideInCalls,
                    _G.overrideStateSlideInArgWasOne,
                    _G.overrideHideOnFinishAfterSlideIn
                "#,
            )
            .expect("post override-state ValidateActionBarTransition probe must run cleanly");

        assert!(
            state_is_override,
            "test setup must put CURRENT_ACTION_BAR_STATE in LE_ACTIONBAR_STATE_OVERRIDE"
        );
        assert!(
            !main_shown,
            "override-state validation must hide MainActionBar"
        );
        assert!(
            !stance_shown,
            "override-state validation must hide StanceBar"
        );
        assert!(
            override_shown,
            "override-state validation must show OverrideActionBar"
        );
        assert_eq!(
            slide_in_calls, 1,
            "override-state validation must slide in a hidden OverrideActionBar once"
        );
        assert!(
            slide_in_arg_was_one,
            "override-state validation must call BeginActionBarTransition with animIn = 1"
        );
        assert!(
            !override_hide_on_finish,
            "slide-in transition must not hide OverrideActionBar on finish"
        );
    });
    }
}
