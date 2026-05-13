//! Behavior pin: main-state transition resets the micro menu only when override is visible.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn validate_transition_main_state_resets_micro_menu_only_while_leaving_override() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.microMenuResetCalls = 0

            function MicroMenu:ResetMicroMenuPosition()
                _G.microMenuResetCalls = _G.microMenuResetCalls + 1
            end

            function MultiActionBar_Update()
            end

            function UIParent_ManageFramePositions()
            end

            function StanceBar:ShouldShow()
                return false
            end

            OverrideActionBar:Hide()
            ValidateActionBarTransition()
            local resetCallsWhenOverrideHidden = _G.microMenuResetCalls

            OverrideActionBar:Show()
            OverrideActionBar.hideOnFinish = false
            ValidateActionBarTransition()

            _G.stateIsMain =
                ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_MAIN
            _G.resetCallsWhenOverrideHidden = resetCallsWhenOverrideHidden
            _G.overrideHideOnFinish = OverrideActionBar.hideOnFinish
            "#,
        )
        .expect("main-state micro-menu reset probe must run cleanly");

        let (state_is_main, reset_calls_when_hidden, total_reset_calls, override_hide_on_finish): (
            bool,
            i32,
            i32,
            bool,
        ) = env
            .eval(
                r#"
                return _G.stateIsMain,
                    _G.resetCallsWhenOverrideHidden,
                    _G.microMenuResetCalls,
                    _G.overrideHideOnFinish
                "#,
            )
            .expect("post main-state micro-menu reset probe must run cleanly");

        assert!(
            state_is_main,
            "test setup must leave CURRENT_ACTION_BAR_STATE at LE_ACTIONBAR_STATE_MAIN"
        );
        assert_eq!(
            reset_calls_when_hidden, 0,
            "ValidateActionBarTransition must not reset MicroMenu when OverrideActionBar is hidden"
        );
        assert_eq!(
            total_reset_calls, 1,
            "ValidateActionBarTransition must reset MicroMenu once when OverrideActionBar is shown"
        );
        assert!(
            override_hide_on_finish,
            "visible override bar must start sliding out after the micro-menu reset"
        );
    });
    }
}
