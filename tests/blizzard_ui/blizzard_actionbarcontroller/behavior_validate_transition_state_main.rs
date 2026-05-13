//! Behavior pin: main-state validation restores main and stance bars.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn validate_transition_main_state_shows_main_stance_and_slides_out_override() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.mainStateSlideOutCalls = 0
            _G.mainStateSlideOutArgWasNil = false

            function OverrideActionBar.slideOut:Play(animIn)
                _G.mainStateSlideOutCalls = _G.mainStateSlideOutCalls + 1
                _G.mainStateSlideOutArgWasNil = animIn == nil
            end

            function MultiActionBar_Update()
            end

            function UIParent_ManageFramePositions()
            end

            function StanceBar:ShouldShow()
                return _G.mainStateStanceShouldShow
            end

            MainActionBar:Hide()
            StanceBar:Hide()
            OverrideActionBar:Hide()
            _G.mainStateStanceShouldShow = false
            ValidateActionBarTransition()

            _G.mainShownWhenStanceHidden = MainActionBar:IsShown()
            _G.stanceShownWhenShouldShowFalse = StanceBar:IsShown()
            _G.slideCallsWhenOverrideHidden = _G.mainStateSlideOutCalls

            MainActionBar:Hide()
            StanceBar:Hide()
            OverrideActionBar:Show()
            OverrideActionBar.hideOnFinish = false
            _G.mainStateStanceShouldShow = true
            ValidateActionBarTransition()

            _G.stateIsMain =
                ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_MAIN
            _G.mainShownWhenStanceVisible = MainActionBar:IsShown()
            _G.stanceShownWhenShouldShowTrue = StanceBar:IsShown()
            _G.overrideHideOnFinish = OverrideActionBar.hideOnFinish
            "#,
        )
        .expect("main-state ValidateActionBarTransition probe must run cleanly");

        let (
            state_is_main,
            main_shown_when_stance_hidden,
            stance_shown_when_should_show_false,
            slide_calls_when_override_hidden,
            main_shown_when_stance_visible,
            stance_shown_when_should_show_true,
            slide_out_calls,
            slide_out_arg_was_nil,
            override_hide_on_finish,
        ): (bool, bool, bool, i32, bool, bool, i32, bool, bool) = env
            .eval(
                r#"
                return _G.stateIsMain,
                    _G.mainShownWhenStanceHidden,
                    _G.stanceShownWhenShouldShowFalse,
                    _G.slideCallsWhenOverrideHidden,
                    _G.mainShownWhenStanceVisible,
                    _G.stanceShownWhenShouldShowTrue,
                    _G.mainStateSlideOutCalls,
                    _G.mainStateSlideOutArgWasNil,
                    _G.overrideHideOnFinish
                "#,
            )
            .expect("post main-state ValidateActionBarTransition probe must run cleanly");

        assert!(
            state_is_main,
            "test precondition must leave CURRENT_ACTION_BAR_STATE at LE_ACTIONBAR_STATE_MAIN"
        );
        assert!(
            main_shown_when_stance_hidden,
            "main-state validation must show MainActionBar"
        );
        assert!(
            !stance_shown_when_should_show_false,
            "main-state validation must not show StanceBar when ShouldShow is false"
        );
        assert_eq!(
            slide_calls_when_override_hidden, 0,
            "main-state validation must not slide out an already hidden OverrideActionBar"
        );
        assert!(
            main_shown_when_stance_visible,
            "main-state validation must show MainActionBar when StanceBar is shown"
        );
        assert!(
            stance_shown_when_should_show_true,
            "main-state validation must show StanceBar when ShouldShow is true"
        );
        assert_eq!(
            slide_out_calls, 1,
            "main-state validation must slide out a visible OverrideActionBar once"
        );
        assert!(
            slide_out_arg_was_nil,
            "main-state validation must call BeginActionBarTransition with nil animIn"
        );
        assert!(
            override_hide_on_finish,
            "slide-out transition must mark OverrideActionBar to hide on finish"
        );
    });
    }
}
