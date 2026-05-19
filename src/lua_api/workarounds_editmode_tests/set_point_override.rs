use super::*;

#[test]
fn edit_mode_set_point_override_syncs_render_anchor_state() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    env.exec(
        r#"
        local frame = CreateFrame("Frame", "MinimapCluster", UIParent)
        frame:SetSize(240, 252)

        local fields = debug.getfenv(frame)[1]
        rawset(fields, "ClearAllPointsBase", function(self)
            self.__editModePoints = {}
        end)
        rawset(fields, "ClearAllPoints", function(self)
            self:ClearAllPointsBase()
        end)
        rawset(fields, "SetPointBase", function(self, point, relativeTo, relativePoint, offsetX, offsetY)
            self.__editModePoints = self.__editModePoints or {}
            table.insert(self.__editModePoints, {
                point,
                relativeTo or UIParent,
                relativePoint or point,
                offsetX or 0,
                offsetY or 0,
            })
        end)
        rawset(fields, "SetPoint", function(self, ...)
            return self:SetPointBase(...)
        end)
        rawset(fields, "GetPoint", function(self, index)
            local point = self.__editModePoints and self.__editModePoints[index or 1]
            if point then
                return unpack(point)
            end
        end)

        EditModeManagerFrame = {
            registeredSystemFrames = { frame },
        }

        function EditModeManagerFrame:OnEditModeSystemAnchorChanged()
            self.anchorChanged = (self.anchorChanged or 0) + 1
        end
        "#,
    )
    .expect("install edit mode frame override stubs");

    fix_set_point_override_3arg(&env);

    let (num_points, right_delta, left): (i32, f64, f64) = env
        .eval(
            r#"
            MinimapCluster:ClearAllPoints()
            MinimapCluster:SetPoint("LEFT", UIParent, "LEFT", 0, 0)
            MinimapCluster:ClearAllPoints()
            MinimapCluster:SetPoint("RIGHT", UIParent, "RIGHT", 0, 0)
            return MinimapCluster:GetNumPoints(),
                math.abs(MinimapCluster:GetRight() - UIParent:GetRight()),
                MinimapCluster:GetLeft()
            "#,
        )
        .expect("read synced Rust anchor state");

    assert_eq!(
        num_points, 1,
        "EditMode ClearAllPoints override must clear Rust anchors too"
    );
    assert!(
        right_delta < 0.01,
        "RIGHT anchor should reach Rust layout state, delta={right_delta}"
    );
    assert!(
        left > 700.0,
        "MinimapCluster should render on the right side after sync, left={left}"
    );
}
