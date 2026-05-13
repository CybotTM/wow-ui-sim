//! Behavior pin: skinned override bars transition away from the main bar.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn update_override_actionbar_mounts_skinned_override_bar() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.has_override_action_bar = true;
            state.override_bar_skin = Some(1);
        }
        assert_override_preconditions(env);

        env.exec(
            r#"
            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "UPDATE_OVERRIDE_ACTIONBAR"
            )
            "#,
        )
        .expect("ActionBarController UPDATE_OVERRIDE_ACTIONBAR dispatch must run cleanly");

        let (state_is_override, main_bar_shown, override_bar_shown): (bool, bool, bool) = env
            .eval(
                r#"
                return ActionBarController_GetCurrentActionBarState() == LE_ACTIONBAR_STATE_OVERRIDE,
                    MainActionBar:IsShown(),
                    OverrideActionBar:IsShown()
                "#,
            )
            .expect("post UPDATE_OVERRIDE_ACTIONBAR transition probe must run cleanly");

        assert!(
            state_is_override,
            "skinned override state must flip CURRENT_ACTION_BAR_STATE to \
             LE_ACTIONBAR_STATE_OVERRIDE"
        );
        assert!(
            !main_bar_shown,
            "ValidateActionBarTransition must hide MainActionBar for skinned override state"
        );
        assert!(
            override_bar_shown,
            "ValidateActionBarTransition must show OverrideActionBar for skinned override state"
        );
    });
    }
}

fn assert_override_preconditions(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (has_override, skin): (bool, i32) = env
        .eval(
            r#"
            return C_ActionBar.HasOverrideActionBar(),
                C_ActionBar.GetOverrideBarSkin()
            "#,
        )
        .expect("override C_ActionBar precondition probe must run cleanly");

    assert!(
        has_override,
        "seeded state must expose an override action bar"
    );
    assert_eq!(skin, 1, "seeded state must expose override bar skin 1");
}
