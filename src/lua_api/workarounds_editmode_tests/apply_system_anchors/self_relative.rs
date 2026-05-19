use super::*;

#[test]
fn apply_system_anchors_skips_self_relative_saved_anchor() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                Buffs = 6,
            },
            EditModeAuraFrameSetting = {
                IconSize = 5,
            },
        }

        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return false end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local frame = {
            system = Enum.EditModeSystem.Buffs,
            systemIndex = 1,
            name = "BuffFrame",
            anchorCalls = 0,
            settingCalls = 0,
            gridLayoutCalls = 0,
        }

        function frame:GetName()
            return self.name
        end

        function frame:SetHasActiveChanges(value)
            self.hasActiveChanges = value
        end

        function frame:UpdateSettingMap()
            self.settingMapUpdated = true
        end

        function frame:ApplySystemAnchor()
            self.anchorCalls = self.anchorCalls + 1
            error("self-relative anchor should not be applied")
        end

        function frame:UpdateSystemSetting(setting, entireSystemUpdate)
            self.settingCalls = self.settingCalls + 1
            self.lastSetting = setting
            self.lastEntireSystemUpdate = entireSystemUpdate
        end

        function frame:UpdateGridLayout()
            self.gridLayoutCalls = self.gridLayoutCalls + 1
        end

        EditModeManagerFrame = {
            layoutInfo = {},
            registeredSystemFrames = { frame },
            layoutApplyInProgress = false,
        }

        function EditModeManagerFrame:InitSystemAnchors()
            self.initSystemAnchorsCalled = true
        end

        function EditModeManagerFrame:GetActiveLayoutSystemInfo()
            return {
                system = Enum.EditModeSystem.Buffs,
                systemIndex = 1,
                isInDefaultPosition = true,
                anchorInfo = {
                    point = "RIGHT",
                    relativeTo = "BuffFrame",
                    relativePoint = "BOTTOMRIGHT",
                    offsetX = -13,
                    offsetY = -15,
                },
                settings = {
                    { setting = Enum.EditModeAuraFrameSetting.IconSize, value = 5 },
                },
            }
        end

        function EditModeManagerFrame:UpdateSystem()
            error("self-relative anchor should not use the full update path")
        end

        function EditModeManagerFrame:UpdateActionBarPositions()
            self.updateActionBarPositionsCalled = true
        end
        "#,
    )
    .expect("install edit mode anchor stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply system anchors");

    let (
        anchor_calls,
        has_active_changes,
        setting_map_updated,
        setting_calls,
        last_setting,
        last_entire_system_update,
        grid_layout_calls,
    ): (i32, bool, bool, i32, i32, bool, i32) = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            return frame.anchorCalls,
                frame.hasActiveChanges,
                frame.settingMapUpdated,
                frame.settingCalls,
                frame.lastSetting,
                frame.lastEntireSystemUpdate,
                frame.gridLayoutCalls
            "#,
        )
        .expect("read self-relative anchor state");

    assert_eq!(anchor_calls, 0, "self-relative anchors should be skipped");
    assert!(
        !has_active_changes,
        "system should still be seeded as clean"
    );
    assert!(
        setting_map_updated,
        "system settings should still be mapped"
    );
    assert_eq!(setting_calls, 1, "saved aura settings should replay");
    assert_eq!(last_setting, 5);
    assert!(
        last_entire_system_update,
        "replayed aura settings should use entire-system semantics"
    );
    assert_eq!(
        grid_layout_calls, 1,
        "self-relative aura systems should still run their final grid layout refresh"
    );
}
