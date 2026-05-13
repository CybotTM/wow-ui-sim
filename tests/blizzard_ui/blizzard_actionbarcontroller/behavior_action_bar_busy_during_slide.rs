//! Behavior pin: running action bar slide animations make transitions busy.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn action_bar_busy_during_override_slide_blocks_validation() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.multiActionBarUpdatesDuringBusy = 0
            function MultiActionBar_Update()
                _G.multiActionBarUpdatesDuringBusy =
                    _G.multiActionBarUpdatesDuringBusy + 1
            end

            StanceBar.numForms = 0
            MainActionBar:Hide()
            OverrideActionBar:Hide()

            BeginActionBarTransition(OverrideActionBar, 1)
            local busyAfterBegin = ActionBarBusy()
            local mainShownBeforeValidate = MainActionBar:IsShown()
            local overrideShownBeforeValidate = OverrideActionBar:IsShown()

            ValidateActionBarTransition()

            _G.busyAfterBegin = busyAfterBegin
            _G.mainShownBeforeValidate = mainShownBeforeValidate
            _G.overrideShownBeforeValidate = overrideShownBeforeValidate
            "#,
        )
        .expect("busy action bar transition probe must run cleanly");

        let (
            busy_after_begin,
            slide_is_playing,
            main_before_validate,
            main_after_validate,
            override_before_validate,
            override_after_validate,
            updates,
        ): (bool, bool, bool, bool, bool, bool, i32) = env
            .eval(
                r#"
                return _G.busyAfterBegin,
                    OverrideActionBar.slideOut:IsPlaying(),
                    _G.mainShownBeforeValidate,
                    MainActionBar:IsShown(),
                    _G.overrideShownBeforeValidate,
                    OverrideActionBar:IsShown(),
                    _G.multiActionBarUpdatesDuringBusy
                "#,
            )
            .expect("post busy transition probe must run cleanly");

        assert!(
            busy_after_begin,
            "BeginActionBarTransition must make ActionBarBusy true while slideOut is playing"
        );
        assert!(
            slide_is_playing,
            "BeginActionBarTransition(OverrideActionBar, 1) must start the slide animation"
        );
        assert_eq!(
            main_before_validate, main_after_validate,
            "ValidateActionBarTransition must not change MainActionBar visibility while busy"
        );
        assert_eq!(
            override_before_validate, override_after_validate,
            "ValidateActionBarTransition must not change OverrideActionBar visibility while busy"
        );
        assert_eq!(
            updates, 0,
            "ValidateActionBarTransition must return before MultiActionBar_Update while busy"
        );
    });
    }
}
