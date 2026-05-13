//! Behavior pin: UNIT_DISPLAYPOWER refreshes the override vehicle power bar.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn unit_displaypower_updates_override_vehicle_power_bar() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            local original = UnitFrameManaBar_Update
            _G.unitFrameManaBarUpdateCalls = 0
            _G.unitFrameManaBarUpdateFrameMatches = false
            _G.unitFrameManaBarUpdateUnit = nil

            function UnitFrameManaBar_Update(frame, unit)
                _G.unitFrameManaBarUpdateCalls = _G.unitFrameManaBarUpdateCalls + 1
                _G.unitFrameManaBarUpdateFrameMatches = frame == OverrideActionBarPowerBar
                _G.unitFrameManaBarUpdateUnit = unit
                if original then
                    return original(frame, unit)
                end
            end

            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "UNIT_DISPLAYPOWER",
                "player"
            )
            "#,
        )
        .expect("ActionBarController UNIT_DISPLAYPOWER dispatch must run cleanly");

        let (calls, frame_matches, unit): (i32, bool, String) = env
            .eval(
                r#"
                return _G.unitFrameManaBarUpdateCalls,
                    _G.unitFrameManaBarUpdateFrameMatches,
                    _G.unitFrameManaBarUpdateUnit
                "#,
            )
            .expect("post UNIT_DISPLAYPOWER mana update probe must run cleanly");

        assert_eq!(
            calls, 1,
            "UNIT_DISPLAYPOWER must call UnitFrameManaBar_Update exactly once"
        );
        assert!(
            frame_matches,
            "UNIT_DISPLAYPOWER must update OverrideActionBarPowerBar"
        );
        assert_eq!(
            unit, "vehicle",
            "UNIT_DISPLAYPOWER must refresh the vehicle power unit"
        );
    });
    }
}
