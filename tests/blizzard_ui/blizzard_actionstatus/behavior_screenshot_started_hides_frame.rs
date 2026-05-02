//! `SCREENSHOT_STARTED` behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";

#[test]
fn screenshot_started_hides_visible_action_status() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (shown_before_event, shown_after_event): (bool, bool) = env
            .eval(
                r#"
                ActionStatus:Show()
                local shownBeforeEvent = ActionStatus:IsShown()

                FireEvent("SCREENSHOT_STARTED")

                return shownBeforeEvent, ActionStatus:IsShown()
                "#,
            )
            .expect("SCREENSHOT_STARTED behavior probe must run cleanly");

        assert!(
            shown_before_event,
            "`ActionStatus:Show()` must make the frame visible before firing SCREENSHOT_STARTED"
        );
        assert!(
            !shown_after_event,
            "`ActionStatusMixin:OnEvent` must call `ActionStatus:Hide()` when \
             SCREENSHOT_STARTED fires"
        );
    });
}
