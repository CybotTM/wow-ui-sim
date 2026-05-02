//! `SCREENSHOT_FAILED` behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";

#[test]
fn screenshot_failed_shows_failure_text() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (is_shown, text, expected_text): (bool, String, String) = env
            .eval(
                r#"
                ActionStatus:Hide()
                ActionStatus.Text:SetText("")

                FireEvent("SCREENSHOT_FAILED")

                return ActionStatus:IsShown(),
                       ActionStatus.Text:GetText(),
                       SCREENSHOT_FAILURE
                "#,
            )
            .expect("SCREENSHOT_FAILED behavior probe must run cleanly");

        assert!(is_shown, "`SCREENSHOT_FAILED` must show `ActionStatus`");
        assert_eq!(
            text, expected_text,
            "`SCREENSHOT_FAILED` must display the localized SCREENSHOT_FAILURE string"
        );
    });
}
