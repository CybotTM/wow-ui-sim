//! Alternate-parent behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";

#[test]
fn alternate_parent_frame_reparents_then_clear_reverts_to_best_parent() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (
            custom_parent_adopted,
            custom_scale,
            custom_anchor_targeted,
            default_parent_restored,
            default_anchor_targeted,
        ): (bool, f32, bool, bool, bool) = env
            .eval(
                r#"
                local defaultParent = CreateFrame("Frame", "ActionStatusDefaultParentProbe", UIParent)
                defaultParent:Show()
                GetAppropriateTopLevelParent = function()
                    return defaultParent
                end
                ActionStatus:UpdateParent()

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

                local customParent = CreateFrame("Frame", "ActionStatusCustomParentProbe", UIParent)
                customParent:SetScale(1.75)
                customParent:Show()

                ActionStatus:SetAlternateParentFrame(customParent)
                local customParentAdopted = ActionStatus:GetParent() == customParent
                local customScale = ActionStatus:GetScale()
                local customAnchorTargeted = allPointsTarget(ActionStatus, customParent)

                ActionStatus:ClearAlternateParentFrame()

                return customParentAdopted,
                       customScale,
                       customAnchorTargeted,
                       ActionStatus:GetParent() == defaultParent,
                       allPointsTarget(ActionStatus, defaultParent)
                "#,
            )
            .expect("ActionStatus alternate-parent behavior probe must run cleanly");

        assert!(
            custom_parent_adopted,
            "`SetAlternateParentFrame(custom)` must reparent ActionStatus to custom"
        );
        assert_eq!(
            custom_scale, 1.0,
            "`SetAlternateParentFrame(custom)` must force ActionStatus scale to 1"
        );
        assert!(
            custom_anchor_targeted,
            "`SetAlternateParentFrame(custom)` must re-anchor ActionStatus with SetAllPoints(custom)"
        );
        assert!(
            default_parent_restored,
            "`ClearAlternateParentFrame()` must restore GetAppropriateTopLevelParent()"
        );
        assert!(
            default_anchor_targeted,
            "`ClearAlternateParentFrame()` must re-anchor ActionStatus with SetAllPoints(bestParent)"
        );
    });
}
