//! `SCREENSHOT_SUCCEEDED` behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";

#[test]
fn screenshot_succeeded_shows_success_text_and_resets_fade_state() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (is_shown, text, expected_text, alpha, start_time_was_reseeded): (
            bool,
            String,
            String,
            f32,
            bool,
        ) = env
            .eval(
                r#"
                ActionStatus:Hide()
                ActionStatus.Text:SetText("")
                ActionStatus:SetAlpha(0.25)
                ActionStatus.startTime = -1

                local beforeEvent = GetTime()
                FireEvent("SCREENSHOT_SUCCEEDED")
                local afterEvent = GetTime()

                return ActionStatus:IsShown(),
                       ActionStatus.Text:GetText(),
                       SCREENSHOT_SUCCESS,
                       ActionStatus:GetAlpha(),
                       ActionStatus.startTime >= beforeEvent and ActionStatus.startTime <= afterEvent
                "#,
            )
            .expect("SCREENSHOT_SUCCEEDED behavior probe must run cleanly");

        assert!(is_shown, "`SCREENSHOT_SUCCEEDED` must show `ActionStatus`");
        assert_eq!(
            text, expected_text,
            "`SCREENSHOT_SUCCEEDED` must display the localized SCREENSHOT_SUCCESS string"
        );
        assert_eq!(
            alpha, 1.0,
            "`SCREENSHOT_SUCCEEDED` must reset `ActionStatus` alpha to 1.0"
        );
        assert!(
            start_time_was_reseeded,
            "`SCREENSHOT_SUCCEEDED` must seed `ActionStatus.startTime` from GetTime()"
        );
    });
}
