//! Direct `DisplayMessage` behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";
const MESSAGE: &str = "hello";

#[test]
fn display_message_direct_call_shows_text_and_resets_fade_state() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (is_shown, text, alpha, start_time_was_reseeded): (bool, String, f32, bool) = env
            .eval(&format!(
                r#"
                ActionStatus:Hide()
                ActionStatus.Text:SetText("")
                ActionStatus:SetAlpha(0.25)
                ActionStatus.startTime = -1

                local beforeCall = GetTime()
                ActionStatus:DisplayMessage({MESSAGE:?})
                local afterCall = GetTime()

                return ActionStatus:IsShown(),
                       ActionStatus.Text:GetText(),
                       ActionStatus:GetAlpha(),
                       ActionStatus.startTime >= beforeCall and ActionStatus.startTime <= afterCall
                "#
            ))
            .expect("DisplayMessage direct-call behavior probe must run cleanly");

        assert!(is_shown, "`DisplayMessage` must show `ActionStatus`");
        assert_eq!(text, MESSAGE, "`DisplayMessage` must set ActionStatus.Text");
        assert_eq!(alpha, 1.0, "`DisplayMessage` must reset alpha to 1.0");
        assert!(
            start_time_was_reseeded,
            "`DisplayMessage` must seed `ActionStatus.startTime` from GetTime()"
        );
    });
}
