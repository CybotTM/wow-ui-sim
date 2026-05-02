//! No-parent fallback behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";

#[test]
fn get_best_parent_without_candidates_returns_nil_and_update_parent_survives() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (best_parent_is_nil, best_parent_scale, update_succeeded, parent_is_nil): (
            bool,
            f32,
            bool,
            bool,
        ) = env
            .eval(
                r#"
                ActionStatus.alternateParentFrame = nil
                GetAppropriateTopLevelParent = function()
                    return nil
                end

                local bestParent, bestParentScale = ActionStatus:GetBestParent()
                local updateSucceeded = pcall(function()
                    ActionStatus:UpdateParent()
                end)

                return bestParent == nil,
                       bestParentScale,
                       updateSucceeded,
                       ActionStatus:GetParent() == nil
                "#,
            )
            .expect("ActionStatus no-parent behavior probe must run cleanly");

        assert!(
            best_parent_is_nil,
            "`GetBestParent()` must return nil when no alternate or top-level parent exists"
        );
        assert_eq!(
            best_parent_scale, 1.0,
            "`GetBestParent()` must return scale 1 when no parent exists"
        );
        assert!(
            update_succeeded,
            "`UpdateParent()` must not crash when `GetBestParent()` returns nil"
        );
        assert!(
            parent_is_nil,
            "`UpdateParent()` must call `SetParent(nil)` when `GetBestParent()` returns nil"
        );
    });
}
