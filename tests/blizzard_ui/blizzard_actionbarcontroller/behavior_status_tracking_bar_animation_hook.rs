//! Behavior pin: status bars defer animations while action bars transition.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn onload_installs_action_bar_busy_as_status_bar_animation_gate() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.statusTrackingSetBarAnimationCalls = 0
            _G.statusTrackingCapturedAnimation = nil

            function StatusTrackingBarManager:SetBarAnimation(animation)
                _G.statusTrackingSetBarAnimationCalls =
                    _G.statusTrackingSetBarAnimationCalls + 1
                _G.statusTrackingCapturedAnimation = animation
            end

            function MainMenuMicroButton_Init()
            end

            ActionBarController_OnLoad(ActionBarController)

            _G.statusTrackingCapturedIsActionBarBusy =
                _G.statusTrackingCapturedAnimation == ActionBarBusy
            "#,
        )
        .expect("ActionBarController OnLoad animation hook probe must run cleanly");

        let (set_bar_animation_calls, captured_is_action_bar_busy, idle_gate): (i32, bool, bool) =
            env.eval(
                r#"
                return _G.statusTrackingSetBarAnimationCalls,
                    _G.statusTrackingCapturedIsActionBarBusy,
                    _G.statusTrackingCapturedAnimation()
                "#,
            )
            .expect("captured status bar animation callback probe must run cleanly");

        env.exec("BeginActionBarTransition(OverrideActionBar, 1)")
            .expect("override action bar transition probe must run cleanly");

        let busy_gate: bool = env
            .eval("return _G.statusTrackingCapturedAnimation()")
            .expect("busy status bar animation callback probe must run cleanly");

        assert_eq!(
            set_bar_animation_calls, 1,
            "ActionBarController_OnLoad must install exactly one status bar animation gate"
        );
        assert!(
            captured_is_action_bar_busy,
            "ActionBarController_OnLoad must pass the ActionBarBusy function"
        );
        assert!(
            !idle_gate,
            "ActionBarBusy gate must allow status bar animations while action bars are idle"
        );
        assert!(
            busy_gate,
            "ActionBarBusy gate must defer status bar animations while action bars are busy"
        );
    });
    }
}
