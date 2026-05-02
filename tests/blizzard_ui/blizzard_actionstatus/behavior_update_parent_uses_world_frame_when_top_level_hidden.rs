//! Hidden top-level parent behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";

#[test]
fn update_parent_uses_world_frame_when_best_parent_is_hidden() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (world_frame_parented, action_status_scale, expected_scale, world_frame_anchored): (
            bool,
            f32,
            f32,
            bool,
        ) = env
            .eval(
                r#"
                local hiddenParent = CreateFrame("Frame", "ActionStatusHiddenTopParentProbe", UIParent)
                hiddenParent:SetScale(1.75)
                hiddenParent:Hide()

                GetAppropriateTopLevelParent = function()
                    return hiddenParent
                end

                local function allPointsTarget(frame, parent)
                    local point1, relativeTo1, relativePoint1 = frame:GetPoint(1)
                    local point2, relativeTo2, relativePoint2 = frame:GetPoint(2)
                    return frame:GetNumPoints() == 2
                       and point1 == "TOPLEFT"
                       and relativeTo1 == parent
                       and relativePoint1 == "TOPLEFT"
                       and point2 == "BOTTOMRIGHT"
                       and relativeTo2 == parent
                       and relativePoint2 == "BOTTOMRIGHT"
                end

                ActionStatus:UpdateParent()

                return ActionStatus:GetParent() == WorldFrame,
                       ActionStatus:GetScale(),
                       hiddenParent:GetEffectiveScale(),
                       allPointsTarget(ActionStatus, WorldFrame)
                "#,
            )
            .expect("ActionStatus hidden-parent fallback probe must run cleanly");

        assert!(
            world_frame_parented,
            "`UpdateParent()` must reparent ActionStatus to WorldFrame when the top-level parent is hidden"
        );
        assert_eq!(
            action_status_scale, expected_scale,
            "`UpdateParent()` must apply the hidden top-level parent's effective scale"
        );
        assert!(
            world_frame_anchored,
            "`UpdateParent()` must re-anchor ActionStatus with SetAllPoints(WorldFrame)"
        );
    });
}
