//! Behavior pin: PET_BATTLE_OPENING_START hides a visible override bar.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn pet_battle_opening_start_animates_visible_override_bar_out() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.petBattleOpeningSlideOutCalls = 0
            _G.petBattleOpeningSlideOutArgWasNil = false

            function OverrideActionBar.slideOut:Play(animIn)
                _G.petBattleOpeningSlideOutCalls =
                    _G.petBattleOpeningSlideOutCalls + 1
                _G.petBattleOpeningSlideOutArgWasNil = animIn == nil
            end

            OverrideActionBar:Show()
            OverrideActionBar.hideOnFinish = false

            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "PET_BATTLE_OPENING_START"
            )
            "#,
        )
        .expect("ActionBarController PET_BATTLE_OPENING_START dispatch must run cleanly");

        let (slide_out_calls, slide_out_arg_was_nil, hide_on_finish): (i32, bool, bool) = env
            .eval(
                r#"
                return _G.petBattleOpeningSlideOutCalls,
                    _G.petBattleOpeningSlideOutArgWasNil,
                    OverrideActionBar.hideOnFinish
                "#,
            )
            .expect("post PET_BATTLE_OPENING_START transition probe must run cleanly");

        assert_eq!(
            slide_out_calls, 1,
            "PET_BATTLE_OPENING_START must play the override bar slide-out once"
        );
        assert!(
            slide_out_arg_was_nil,
            "PET_BATTLE_OPENING_START must call BeginActionBarTransition with nil animIn"
        );
        assert!(
            hide_on_finish,
            "BeginActionBarTransition(OverrideActionBar, nil) must mark the bar to hide on finish"
        );
    });
    }
}
