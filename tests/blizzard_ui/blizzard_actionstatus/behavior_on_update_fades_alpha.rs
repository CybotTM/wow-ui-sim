//! `OnUpdate` fade behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";
const HALF_FADE_ELAPSED: f32 = 1.0;
const DONE_FADE_ELAPSED: f32 = 2.1;
const HALF_ALPHA: f32 = 0.5;
const ALPHA_TOLERANCE: f32 = 0.001;

#[test]
fn on_update_fades_alpha_then_hides_after_fadetime() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (seeded_by_display_message, half_alpha, shown_after_half, shown_after_done): (
            bool,
            f32,
            bool,
            bool,
        ) = env
            .eval(&format!(
                r#"
                ActionStatus:DisplayMessage("fade")
                local seededByDisplayMessage = ActionStatus.startTime >= 0

                ActionStatus.startTime = GetTime() - {HALF_FADE_ELAPSED}
                ActionStatus:OnUpdate()
                local halfAlpha = ActionStatus:GetAlpha()
                local shownAfterHalf = ActionStatus:IsShown()

                ActionStatus:DisplayMessage("fade")
                ActionStatus.startTime = GetTime() - {DONE_FADE_ELAPSED}
                ActionStatus:OnUpdate()

                return seededByDisplayMessage,
                       halfAlpha,
                       shownAfterHalf,
                       ActionStatus:IsShown()
                "#
            ))
            .expect("ActionStatus OnUpdate fade probe must run cleanly");

        assert!(
            seeded_by_display_message,
            "`DisplayMessage` must seed `ActionStatus.startTime` before OnUpdate fades"
        );
        assert!(
            (half_alpha - HALF_ALPHA).abs() <= ALPHA_TOLERANCE,
            "`ActionStatus:OnUpdate()` after 1.0s must fade alpha to ~0.5, got {half_alpha}"
        );
        assert!(
            shown_after_half,
            "`ActionStatus` must remain shown halfway through the 2.0s fade"
        );
        assert!(
            !shown_after_done,
            "`ActionStatus` must hide once elapsed time exceeds the fade duration"
        );
    });
}
