//! Behavior pin: shapeshift controller events refresh StanceBar.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn shapeshift_events_update_stance_bar() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            local original = StanceBar.Update
            StanceBar.updateCalls = {}

            function StanceBar:Update()
                table.insert(self.updateCalls, _G.currentShapeshiftEvent)
                if original then
                    return original(self)
                end
            end

            local events = {
                "UPDATE_SHAPESHIFT_FORM",
                "UPDATE_SHAPESHIFT_FORMS",
                "UPDATE_SHAPESHIFT_USABLE",
            }

            for _, eventName in ipairs(events) do
                _G.currentShapeshiftEvent = eventName
                ActionBarController:GetScript("OnEvent")(
                    ActionBarController,
                    eventName
                )
            end
            "#,
        )
        .expect("ActionBarController shapeshift event dispatches must run cleanly");

        let (call_count, first_event, second_event, third_event): (i32, String, String, String) =
            env.eval(
                r#"
                return #StanceBar.updateCalls,
                    StanceBar.updateCalls[1],
                    StanceBar.updateCalls[2],
                    StanceBar.updateCalls[3]
                "#,
            )
            .expect("post shapeshift StanceBar update probe must run cleanly");

        assert_eq!(
            call_count, 3,
            "each shapeshift controller event must call StanceBar:Update exactly once"
        );
        assert_eq!(first_event, "UPDATE_SHAPESHIFT_FORM");
        assert_eq!(second_event, "UPDATE_SHAPESHIFT_FORMS");
        assert_eq!(third_event, "UPDATE_SHAPESHIFT_USABLE");
    });
    }
}
