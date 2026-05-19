use super::*;

const ACTION_BAR_PROFILE_REPLAY_STUBS: &str = r#"
        Enum = {
            EditModeSystem = {
                ActionBar = 0,
            },
            EditModeActionBarSetting = {
                Orientation = 0,
                NumRows = 1,
                NumIcons = 2,
                IconSize = 3,
                IconPadding = 4,
                VisibleSetting = 5,
                HideBarArt = 6,
                HideBarScrolling = 8,
                AlwaysShowButtons = 9,
            },
            ActionBarOrientation = {
                Horizontal = 0,
                Vertical = 1,
            },
            ActionBarVisibleSetting = {
                Always = 0,
                InCombat = 1,
                OutOfCombat = 2,
                Hidden = 3,
            },
        }
        ACTION_BUTTON_SHOW_GRID_REASON_CVAR = 4

        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return true end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local actionButton = {
            container = {},
        }
        function actionButton.container:SetScale(value)
            self.scale = value
        end
        function actionButton:UpdateButtonArt()
            self.buttonArtUpdated = true
        end
        function actionButton:SetShowGrid(showGrid, reason)
            self.showGrid = showGrid
            self.showGridReason = reason
        end

        local frame = {
            system = Enum.EditModeSystem.ActionBar,
            systemIndex = 1,
            name = "MainActionBar",
            anchorCalls = 0,
            actionButtons = { actionButton },
            ActionBarPageNumber = {},
            BorderArt = {},
            Selection = {},
            numButtonsShowable = 12,
        }

        function frame:GetName()
            return self.name
        end

        function frame:SetHasActiveChanges(value)
            self.hasActiveChanges = value
        end

        function frame.Selection:SetVerticalState(value)
            self.verticalState = value
        end

        function frame:UpdateSettingMap()
            self.settingMapUpdated = true
        end

        function frame:GetSettingValue(setting, useRawValue)
            for _, settingInfo in ipairs(self.systemInfo.settings) do
                if settingInfo.setting == setting then
                    if not useRawValue
                        and setting == Enum.EditModeActionBarSetting.IconSize then
                        return 50 + (settingInfo.value * 10)
                    end
                    return settingInfo.value
                end
            end
        end

        function frame:ApplySystemAnchor()
            self.anchorCalls = self.anchorCalls + 1
        end

        function frame:EditModeSetScale(value)
            self.editModeScale = value
        end

        function frame:UpdateShownButtons()
            self.shownButtonsUpdated = true
        end

        function frame:Layout()
            self.layoutUpdated = true
        end

        function frame:UpdateVisibility()
            self.visibilityUpdated = true
        end

        function frame:SetShowGrid(showGrid, reason)
            self.showGrid = showGrid
            self.showGridReason = reason
            for _, button in pairs(self.actionButtons) do
                button:SetShowGrid(showGrid, reason)
            end
        end

        function frame:RefreshGridLayout()
            self.gridRefreshed = true
        end

        function frame:RefreshDividers()
            self.dividersRefreshed = true
        end

        function frame:RefreshBarArt()
            self.barArtRefreshed = true
        end

        function frame.BorderArt:SetShown(value)
            self.shown = value
        end

        function frame.ActionBarPageNumber:SetShown(value)
            self.shown = value
        end

        EditModeManagerFrame = {
            layoutInfo = {},
            registeredSystemFrames = { frame },
            layoutApplyInProgress = false,
        }

        function EditModeManagerFrame:InitSystemAnchors()
            self.initSystemAnchorsCalled = true
        end

        function EditModeManagerFrame:UpdateActionBarLayout(systemFrame)
            self.actionBarLayoutUpdated = systemFrame == frame
        end

        function EditModeManagerFrame:GetActiveLayoutSystemInfo()
            return {
                system = Enum.EditModeSystem.ActionBar,
                systemIndex = 1,
                isInDefaultPosition = false,
                anchorInfo = {
                    point = "BOTTOMLEFT",
                    relativeTo = UIParent,
                    relativePoint = "BOTTOMLEFT",
                    offsetX = 208.2,
                    offsetY = 99.7,
                },
                settings = {
                    { setting = Enum.EditModeActionBarSetting.Orientation, value = Enum.ActionBarOrientation.Vertical },
                    { setting = Enum.EditModeActionBarSetting.NumRows, value = 2 },
                    { setting = Enum.EditModeActionBarSetting.NumIcons, value = 8 },
                    { setting = Enum.EditModeActionBarSetting.IconSize, value = 3 },
                    { setting = Enum.EditModeActionBarSetting.IconPadding, value = 6 },
                    { setting = Enum.EditModeActionBarSetting.VisibleSetting, value = Enum.ActionBarVisibleSetting.Hidden },
                    { setting = Enum.EditModeActionBarSetting.HideBarArt, value = 1 },
                    { setting = Enum.EditModeActionBarSetting.HideBarScrolling, value = 1 },
                    { setting = Enum.EditModeActionBarSetting.AlwaysShowButtons, value = 1 },
                },
            }
        end

        function EditModeManagerFrame:UpdateActionBarPositions()
            error("saved action bar anchors should not be repacked")
        end
"#;

fn install_action_bar_profile_replay_stubs(env: &WowLuaEnv) {
    env.exec(ACTION_BAR_PROFILE_REPLAY_STUBS)
        .expect("install action bar stubs");
}

#[test]
fn apply_system_anchors_does_not_repack_action_bars_after_saved_anchor() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    install_action_bar_profile_replay_stubs(&env);
    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply action bar anchors");

    let (
        anchor_calls,
        grid_refreshed,
        num_rows,
        num_buttons,
        button_padding,
        edit_mode_scale,
        button_scale,
        visibility,
        visibility_updated,
        border_art_shown,
        page_number_shown,
        button_art_updated,
        show_grid,
        show_grid_reason,
        layout_updated,
        action_bar_layout_updated,
        selection_vertical,
    ): (
        i32,
        bool,
        i32,
        i32,
        i32,
        f64,
        f64,
        String,
        bool,
        bool,
        bool,
        bool,
        bool,
        i32,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            local button = frame.actionButtons[1]
            return frame.anchorCalls,
                frame.gridRefreshed,
                frame.numRows,
                frame.numButtonsShowable,
                frame.buttonPadding,
                frame.editModeScale,
                button.container.scale,
                frame.visibility,
                frame.visibilityUpdated,
                frame.BorderArt.shown,
                frame.ActionBarPageNumber.shown,
                button.buttonArtUpdated,
                button.showGrid,
                button.showGridReason,
                frame.layoutUpdated,
                EditModeManagerFrame.actionBarLayoutUpdated,
                frame.Selection.verticalState
            "#,
        )
        .expect("read action bar state");

    assert_eq!(anchor_calls, 1, "saved action bar anchor should apply once");
    assert!(
        grid_refreshed,
        "action bar runtime layout should still refresh"
    );
    assert_eq!(num_rows, 2);
    assert_eq!(num_buttons, 8);
    assert_eq!(button_padding, 6);
    assert_eq!(edit_mode_scale, 0.8);
    assert_eq!(button_scale, 0.8);
    assert_eq!(visibility, "Hidden");
    assert!(visibility_updated);
    assert!(
        !border_art_shown,
        "HideBarArt should hide saved main-bar side art"
    );
    assert!(!page_number_shown);
    assert!(button_art_updated);
    assert!(show_grid);
    assert_eq!(show_grid_reason, 4);
    assert!(layout_updated);
    assert!(action_bar_layout_updated);
    assert!(
        selection_vertical,
        "vertical action bars should update EditMode selection state"
    );
}

#[test]
fn fix_action_bar_size_ignores_hidden_right_anchored_buttons() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        MainActionBar = {
            width = -472,
            height = 45,
        }

        function MainActionBar:GetSize()
            return self.width, self.height
        end

        function MainActionBar:SetSize(width, height)
            self.width = width
            self.height = height
        end

        local function newContainer(width, height, point, offsetX, shown)
            return {
                width = width,
                height = height,
                point = point,
                offsetX = offsetX,
                shown = shown,
            }
        end

        function newContainerMethods(container)
            function container:GetSize()
                return self.width, self.height
            end
            function container:GetNumPoints()
                return 1
            end
            function container:GetPoint()
                return self.point, MainActionBar, self.point, self.offsetX, 0
            end
            function container:IsShown()
                return self.shown
            end
            return container
        end

        for i = 1, 8 do
            _G["MainActionBarButtonContainer" .. i] = newContainerMethods(
                newContainer(40, 40, "BOTTOMLEFT", (i - 1) * 47, true)
            )
        end

        for i = 9, 12 do
            _G["MainActionBarButtonContainer" .. i] = newContainerMethods(
                newContainer(40, 40, "BOTTOMRIGHT", -376 - ((i - 9) * 47), false)
            )
        end
        "#,
    )
    .expect("install main action bar size stubs");

    env.exec(FIX_ACTION_BAR_NAN_SIZE_LUA)
        .expect("fix main action bar size");

    let (width, height): (i32, i32) = env
        .eval("return MainActionBar.width, MainActionBar.height")
        .expect("read main action bar size");

    assert_eq!(width, 369);
    assert_eq!(height, 40);
}

#[test]
fn apply_system_anchors_replays_each_widescreen_action_bar_profile_row() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                ActionBar = 0,
            },
            EditModeActionBarSetting = {
                Orientation = 0,
                NumRows = 1,
                NumIcons = 2,
                IconSize = 3,
                IconPadding = 4,
                VisibleSetting = 5,
                HideBarArt = 6,
                HideBarScrolling = 8,
                AlwaysShowButtons = 9,
            },
            ActionBarOrientation = {
                Horizontal = 0,
                Vertical = 1,
            },
            ActionBarVisibleSetting = {
                Always = 0,
                Hidden = 3,
            },
        }
        ACTION_BUTTON_SHOW_GRID_REASON_CVAR = 4
        UIParent = { name = "UIParent" }

        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return true end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local setting = Enum.EditModeActionBarSetting
        local rows = {
            [1] = { {setting.Orientation, 0}, {setting.NumRows, 1}, {setting.NumIcons, 8}, {setting.IconSize, 4}, {setting.IconPadding, 2}, {setting.HideBarArt, 1}, {setting.HideBarScrolling, 1}, {setting.AlwaysShowButtons, 1} },
            [2] = { {setting.Orientation, 0}, {setting.NumRows, 4}, {setting.NumIcons, 6}, {setting.IconSize, 5}, {setting.IconPadding, 2}, {setting.VisibleSetting, 0}, {setting.AlwaysShowButtons, 1} },
            [3] = { {setting.Orientation, 0}, {setting.NumRows, 1}, {setting.NumIcons, 12}, {setting.IconSize, 2}, {setting.IconPadding, 2}, {setting.VisibleSetting, 0}, {setting.AlwaysShowButtons, 0} },
            [4] = { {setting.Orientation, 0}, {setting.NumRows, 1}, {setting.NumIcons, 8}, {setting.IconSize, 4}, {setting.IconPadding, 2}, {setting.VisibleSetting, 0}, {setting.AlwaysShowButtons, 0} },
            [5] = { {setting.Orientation, 0}, {setting.NumRows, 1}, {setting.NumIcons, 8}, {setting.IconSize, 4}, {setting.IconPadding, 2}, {setting.VisibleSetting, 0}, {setting.AlwaysShowButtons, 1} },
            [6] = { {setting.Orientation, 0}, {setting.NumRows, 1}, {setting.NumIcons, 12}, {setting.IconSize, 3}, {setting.IconPadding, 2}, {setting.VisibleSetting, 0}, {setting.AlwaysShowButtons, 1} },
            [7] = { {setting.Orientation, 0}, {setting.NumRows, 1}, {setting.NumIcons, 12}, {setting.IconSize, 5}, {setting.IconPadding, 2}, {setting.VisibleSetting, 0}, {setting.AlwaysShowButtons, 1} },
            [8] = { {setting.Orientation, 0}, {setting.NumRows, 1}, {setting.NumIcons, 12}, {setting.IconSize, 5}, {setting.IconPadding, 2}, {setting.VisibleSetting, 0}, {setting.AlwaysShowButtons, 1} },
            [11] = { {setting.Orientation, 0}, {setting.NumRows, 3}, {setting.IconSize, 5}, {setting.IconPadding, 2} },
            [12] = { {setting.Orientation, 0}, {setting.NumRows, 1}, {setting.IconSize, 5}, {setting.IconPadding, 2}, {setting.AlwaysShowButtons, 0} },
            [13] = { {setting.Orientation, 0}, {setting.NumRows, 1}, {setting.IconSize, 5}, {setting.IconPadding, 2} },
        }

        local function settingsFor(index)
            local settings = {}
            for _, pair in ipairs(rows[index] or {}) do
                table.insert(settings, { setting = pair[1], value = pair[2] })
            end
            return settings
        end

        local function newActionBar(index)
            local button = { container = {} }
            function button.container:SetScale(value)
                self.scale = value
            end
            function button:UpdateButtonArt()
                self.buttonArtUpdated = true
            end
            function button:SetShowGrid(showGrid, reason)
                self.showGrid = showGrid
                self.showGridReason = reason
            end

            local frame = {
                system = Enum.EditModeSystem.ActionBar,
                systemIndex = index,
                name = "ActionBar" .. tostring(index),
                actionButtons = { button },
                ActionBarPageNumber = {},
                BorderArt = {},
                Selection = {},
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
                self.anchorApplied = true
            end
            function frame.Selection:SetVerticalState(value)
                self.verticalState = value
            end
            function frame:GetSettingValue(settingId)
                for _, settingInfo in ipairs(self.systemInfo.settings) do
                    if settingInfo.setting == settingId then
                        return settingInfo.value
                    end
                end
            end
            function frame:UpdateShownButtons()
                self.shownButtonsUpdated = true
            end
            function frame:EditModeSetScale(value)
                self.editModeScale = value
            end
            function frame:Layout()
                self.layoutUpdated = true
            end
            function frame:UpdateVisibility()
                self.visibilityUpdated = true
            end
            function frame:UpdateEndCaps(forceHide)
                self.endCapsForceHide = forceHide
            end
            function frame:SetShowGrid(showGrid, reason)
                self.showGrid = showGrid
                button:SetShowGrid(showGrid, reason)
            end
            function frame:RefreshGridLayout()
                self.gridRefreshed = true
            end
            function frame:RefreshDividers()
                self.dividersRefreshed = true
            end
            function frame:RefreshBarArt()
                self.barArtRefreshed = true
            end
            function frame.BorderArt:SetShown(value)
                self.shown = value
            end
            function frame.ActionBarPageNumber:SetShown(value)
                self.shown = value
            end

            return frame
        end

        EditModeManagerFrame = {
            layoutInfo = {},
            requestedIndices = {},
            registeredSystemFrames = {
                newActionBar(1), newActionBar(2), newActionBar(3), newActionBar(4),
                newActionBar(5), newActionBar(6), newActionBar(7), newActionBar(8),
                newActionBar(11), newActionBar(12), newActionBar(13),
            },
        }

        function EditModeManagerFrame:InitSystemAnchors()
            self.initSystemAnchorsCalled = true
        end
        function EditModeManagerFrame:GetActiveLayoutSystemInfo(system, systemIndex)
            table.insert(self.requestedIndices, systemIndex)
            return {
                system = system,
                systemIndex = systemIndex,
                isInDefaultPosition = false,
                anchorInfo = { point = "CENTER", relativeTo = UIParent, relativePoint = "CENTER", offsetX = 0, offsetY = 0 },
                settings = settingsFor(systemIndex),
            }
        end
        function EditModeManagerFrame:UpdateActionBarLayout(systemFrame)
            systemFrame.actionBarLayoutUpdated = true
        end
        function EditModeManagerFrame:UpdateSystem()
            error("active action bars should use the startup replay path")
        end
        "#,
    )
    .expect("install active action bar row stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply active action bar row settings");

    let (requested_indices, replayed_rows): (String, String) = env
        .eval(
            r#"
            local rows = {}
            for _, frame in ipairs(EditModeManagerFrame.registeredSystemFrames) do
                local button = frame.actionButtons[1]
                table.insert(rows, table.concat({
                    tostring(frame.systemIndex),
                    tostring(frame.numRows),
                    tostring(frame.numButtonsShowable),
                    tostring(frame.shownButtonsUpdated),
                    tostring(frame.iconSize),
                    tostring(frame.buttonPadding),
                    tostring(frame.hideBarArt),
                    tostring(frame.endCapsForceHide),
                    tostring(frame.ActionBarPageNumber.shown),
                    tostring(button.showGrid),
                    frame.visibility or "_",
                }, ":"))
            end
            return table.concat(EditModeManagerFrame.requestedIndices, ","),
                table.concat(rows, "|")
            "#,
        )
        .expect("read action bar row replay state");

    assert_eq!(requested_indices, "1,2,3,4,5,6,7,8,11,12,13");
    assert_eq!(
        replayed_rows,
        "1:1:8:true:4:2:true:true:false:true:_|2:4:6:true:5:2:nil:nil:nil:true:Always|3:1:12:true:2:2:nil:nil:nil:false:Always|4:1:8:true:4:2:nil:nil:nil:false:Always|5:1:8:true:4:2:nil:nil:nil:true:Always|6:1:12:true:3:2:nil:nil:nil:true:Always|7:1:12:true:5:2:nil:nil:nil:true:Always|8:1:12:true:5:2:nil:nil:nil:true:Always|11:3:nil:nil:5:2:nil:nil:nil:nil:_|12:1:nil:nil:5:2:nil:nil:nil:false:_|13:1:nil:nil:5:2:nil:nil:nil:nil:_",
        "each active Widescreen action-bar row should replay saved settings"
    );
}
