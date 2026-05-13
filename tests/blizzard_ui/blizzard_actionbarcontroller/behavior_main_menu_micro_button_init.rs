//! Behavior pin: controller OnLoad initializes the micro menu.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn onload_calls_main_menu_micro_button_init_once() {
    test_timeout! {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.mainMenuMicroButtonInitCalls = 0

            function StatusTrackingBarManager:SetBarAnimation()
            end

            function MainMenuMicroButton_Init()
                _G.mainMenuMicroButtonInitCalls =
                    _G.mainMenuMicroButtonInitCalls + 1
            end

            ActionBarController_OnLoad(ActionBarController)
            "#,
        )
        .expect("ActionBarController OnLoad micro-menu probe must run cleanly");

        let init_calls: i32 = env
            .eval("return _G.mainMenuMicroButtonInitCalls")
            .expect("micro-menu OnLoad call count probe must run cleanly");

        assert_eq!(
            init_calls, 1,
            "ActionBarController_OnLoad must call MainMenuMicroButton_Init exactly once"
        );
    });
    }
}
