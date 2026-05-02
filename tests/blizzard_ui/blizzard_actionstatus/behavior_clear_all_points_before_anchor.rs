//! Anchor reset behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";

#[test]
fn update_parent_resets_points_before_anchoring_to_parent() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (seed_point_count, update_point_count, update_anchors_target_parent): (i32, i32, bool) =
            env.eval(
                r#"
                local parent = CreateFrame("Frame", "ActionStatusClearPointsParentProbe", UIParent)
                parent:Show()

                GetAppropriateTopLevelParent = function()
                    return parent
                end

                local function allPointsTarget(frame, target)
                    local point1, relativeTo1, relativePoint1 = frame:GetPoint(1)
                    local point2, relativeTo2, relativePoint2 = frame:GetPoint(2)
                    return frame:GetNumPoints() == 2
                       and point1 == "TOPLEFT"
                       and relativeTo1 == target
                       and relativePoint1 == "TOPLEFT"
                       and point2 == "BOTTOMRIGHT"
                       and relativeTo2 == target
                       and relativePoint2 == "BOTTOMRIGHT"
                end

                ActionStatus:ClearAllPoints()
                ActionStatus:SetPoint("CENTER", UIParent, "CENTER", 7, 9)
                local seedPointCount = ActionStatus:GetNumPoints()

                ActionStatus:UpdateParent()

                return seedPointCount,
                       ActionStatus:GetNumPoints(),
                       allPointsTarget(ActionStatus, parent)
                "#,
            )
            .expect("ActionStatus clear-points behavior probe must run cleanly");

        assert_eq!(
            seed_point_count, 1,
            "test setup must seed exactly one stale ActionStatus point"
        );
        assert_eq!(
            update_point_count, 2,
            "`UpdateParent()` must replace stale points with the two SetAllPoints anchors"
        );
        assert!(
            update_anchors_target_parent,
            "`UpdateParent()` must anchor only to the selected parent"
        );
    });
}
