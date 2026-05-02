//! Alternate top-level parent event behavior for `Blizzard_ActionStatus`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ActionStatus";
const ALTERNATE_PARENT_EVENT: &str = "UI.AlternateTopLevelParentChanged";

#[test]
fn alternate_top_level_parent_event_reruns_update_parent() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (old_parent_adopted, new_parent_adopted, new_parent_anchored): (bool, bool, bool) = env
            .eval(
                r#"
                local oldParent = CreateFrame("Frame", "ActionStatusOldTopParentProbe", UIParent)
                oldParent:Show()
                local newParent = CreateFrame("Frame", "ActionStatusNewTopParentProbe", UIParent)
                newParent:Show()

                local currentParent = oldParent
                GetAppropriateTopLevelParent = function()
                    return currentParent
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
                local oldParentAdopted = ActionStatus:GetParent() == oldParent

                currentParent = newParent
                EventRegistry:TriggerEvent("UI.AlternateTopLevelParentChanged")

                return oldParentAdopted,
                       ActionStatus:GetParent() == newParent,
                       allPointsTarget(ActionStatus, newParent)
                "#,
            )
            .expect("ActionStatus alternate top-level parent event probe must run cleanly");

        assert!(
            old_parent_adopted,
            "test setup must first parent ActionStatus to the old top-level parent"
        );
        assert!(
            new_parent_adopted,
            "`EventRegistry:TriggerEvent({ALTERNATE_PARENT_EVENT:?})` must rerun UpdateParent"
        );
        assert!(
            new_parent_anchored,
            "`UpdateParent` from {ALTERNATE_PARENT_EVENT:?} must re-anchor with SetAllPoints"
        );
    });
}
