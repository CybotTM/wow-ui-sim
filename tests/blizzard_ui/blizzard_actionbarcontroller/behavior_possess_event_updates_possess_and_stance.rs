//! Behavior pin: UPDATE_POSSESS_BAR refreshes possess and stance bars.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn update_possess_bar_updates_possess_and_stance_bars() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            PossessActionBar.updateCalls = 0
            StanceBar.updateCalls = 0

            function PossessActionBar:Update()
                self.updateCalls = self.updateCalls + 1
            end

            function StanceBar:Update()
                self.updateCalls = self.updateCalls + 1
            end

            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "UPDATE_POSSESS_BAR"
            )
            "#,
        )
        .expect("ActionBarController UPDATE_POSSESS_BAR dispatch must run cleanly");

        let (possess_calls, stance_calls): (i32, i32) = env
            .eval(
                r#"
                return PossessActionBar.updateCalls,
                    StanceBar.updateCalls
                "#,
            )
            .expect("post UPDATE_POSSESS_BAR update probe must run cleanly");

        assert_eq!(
            possess_calls, 1,
            "UPDATE_POSSESS_BAR must call PossessActionBar:Update exactly once"
        );
        assert_eq!(
            stance_calls, 1,
            "UPDATE_POSSESS_BAR must call StanceBar:Update exactly once"
        );
    });
    }
}
