//! `OnUpdate` after-fadetime behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";
const AFTER_FADETIME_ELAPSED: f32 = 2.1;

#[test]
fn on_update_after_fadetime_hides_action_status() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (shown_before_update, shown_after_update): (bool, bool) = env
            .eval(&format!(
                r#"
                ActionStatus:DisplayMessage("done")
                local shownBeforeUpdate = ActionStatus:IsShown()

                ActionStatus.startTime = GetTime() - {AFTER_FADETIME_ELAPSED}
                ActionStatus:OnUpdate()

                return shownBeforeUpdate, ActionStatus:IsShown()
                "#
            ))
            .expect("ActionStatus after-fadetime OnUpdate probe must run cleanly");

        assert!(
            shown_before_update,
            "`DisplayMessage` must show `ActionStatus` before OnUpdate runs"
        );
        assert!(
            !shown_after_update,
            "`ActionStatus:OnUpdate()` must hide the frame once elapsed time exceeds 2.0s"
        );
    });
}
