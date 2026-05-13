//! Behavior pin: UPDATE_EXTRA_ACTIONBAR refreshes the extra action bar.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn update_extra_actionbar_invokes_extra_actionbar_update() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.extraActionBarUpdateCalls = 0

            function ExtraActionBar_Update()
                _G.extraActionBarUpdateCalls = _G.extraActionBarUpdateCalls + 1
            end

            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "UPDATE_EXTRA_ACTIONBAR"
            )
            "#,
        )
        .expect("ActionBarController UPDATE_EXTRA_ACTIONBAR dispatch must run cleanly");

        let calls: i32 = env
            .eval("return _G.extraActionBarUpdateCalls")
            .expect("post UPDATE_EXTRA_ACTIONBAR update probe must run cleanly");

        assert_eq!(
            calls, 1,
            "UPDATE_EXTRA_ACTIONBAR must call ExtraActionBar_Update exactly once"
        );
    });
    }
}
