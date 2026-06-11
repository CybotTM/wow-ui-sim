use super::*;

#[test]
fn apply_system_anchors_hides_profile_hidden_status_tracking_bars() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                StatusTrackingBar = 15,
            },
            EditModeStatusTrackingBarSystemIndices = {
                StatusTrackingBar1 = 1,
            },
        }

        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return false end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local frame = {
            system = Enum.EditModeSystem.StatusTrackingBar,
            systemIndex = Enum.EditModeStatusTrackingBarSystemIndices.StatusTrackingBar1,
            name = "MainStatusTrackingBarContainer",
            shown = true,
            shownBarIndex = 4,
            isInEditMode = false,
            settingsMapUpdates = 0,
        }

        function frame:GetName()
            return self.name
        end
        function frame:IsShown()
            return self.shown
        end
        function frame:Show()
            self.shown = true
        end
        function frame:Hide()
            self.shown = false
            self.hideCalls = (self.hideCalls or 0) + 1
        end
        function frame:SetShown(shown)
            if shown then
                self:Show()
            else
                self:Hide()
            end
        end
        function frame:UpdateShownState()
            self:SetShown(self.shownBarIndex ~= -1 or self.isInEditMode)
        end
        function frame:SetHasActiveChanges(value)
            self.hasActiveChanges = value
        end
        function frame:UpdateSettingMap()
            self.settingsMapUpdates = self.settingsMapUpdates + 1
        end
        function frame:ApplySystemAnchor()
            self.anchorCalls = (self.anchorCalls or 0) + 1
        end
        function frame:UpdateSystem(systemInfo)
            self.updateSystemCalls = (self.updateSystemCalls or 0) + 1
            self.systemInfo = systemInfo
            self:Show()
        end

        EditModeManagerFrame = {
            layoutInfo = {},
            registeredSystemFrames = { frame },
        }

        function EditModeManagerFrame:InitSystemAnchors()
            self.initSystemAnchorsCalled = true
        end
        function EditModeManagerFrame:GetActiveLayoutSystemInfo(system, systemIndex)
            return {
                system = system,
                systemIndex = systemIndex,
                hidden = true,
                isInDefaultPosition = false,
                anchorInfo = {
                    point = "BOTTOM",
                    relativeTo = UIParent,
                    relativePoint = "BOTTOM",
                    offsetX = 0,
                    offsetY = 0,
                },
                settings = {},
            }
        end
        function EditModeManagerFrame:UpdateSystem(systemFrame)
            -- Mirrors EditModeManagerFrameMixin:UpdateSystem -- the manager
            -- resolves the active layout itself; nothing pre-seeds the frame.
            systemFrame:UpdateSystem(self:GetActiveLayoutSystemInfo(systemFrame.system, systemFrame.systemIndex))
        end
        "#,
    )
    .expect("install profile-hidden status tracking stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply singleton anchors with profile-hidden status bar");

    env.exec(
        r#"
        EditModeManagerFrame.registeredSystemFrames[1]:UpdateShownState()
        "#,
    )
    .expect("replay later status tracking visibility update");

    let (shown, update_calls, anchor_calls, hide_calls): (bool, i64, i64, i64) = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            return frame:IsShown(),
                frame.updateSystemCalls or 0,
                frame.anchorCalls or 0,
                frame.hideCalls or 0
            "#,
        )
        .expect("read profile-hidden status bar replay state");

    assert!(
        !shown,
        "startup EditMode replay must apply the active profile hidden flag: \
         shown={shown}, update_calls={update_calls}, anchor_calls={anchor_calls}, \
         hide_calls={hide_calls}"
    );
    assert_eq!(
        update_calls, 1,
        "profile-hidden status bars still need UpdateSystem for layout state"
    );
    assert_eq!(
        anchor_calls, 1,
        "profile-hidden status bars still need anchor replay for later re-show"
    );
    assert_eq!(
        hide_calls, 2,
        "profile-hidden status bars should be hidden after UpdateSystem and \
         remain hidden when later status-tracking updates try to show them"
    );
}
