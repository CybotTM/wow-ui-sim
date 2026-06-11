use super::*;

#[test]
fn apply_system_anchors_replays_active_widescreen_aura_frame_settings() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                AuraFrame = 6,
            },
            EditModeAuraFrameSystemIndices = {
                BuffFrame = 1,
                DebuffFrame = 2,
                ExternalDefensivesFrame = 3,
            },
        }

        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return false end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local rows = {
            [Enum.EditModeAuraFrameSystemIndices.BuffFrame] = {
                {0, 0}, {1, 0}, {2, 0}, {3, 11}, {5, 5}, {6, 5},
            },
            [Enum.EditModeAuraFrameSystemIndices.DebuffFrame] = {
                {0, 0}, {1, 0}, {2, 0}, {4, 8}, {5, 5}, {6, 5},
            },
            [Enum.EditModeAuraFrameSystemIndices.ExternalDefensivesFrame] = {
                {0, 0}, {1, 0}, {2, 1}, {3, 7}, {5, 5}, {6, 5}, {9, 70},
            },
        }

        local function settingsFor(index)
            local settings = {}
            for _, pair in ipairs(rows[index] or {}) do
                table.insert(settings, { setting = pair[1], value = pair[2] })
            end
            return settings
        end

        local function newAuraFrame(index, name)
            local frame = {
                system = Enum.EditModeSystem.AuraFrame,
                systemIndex = index,
                name = name,
                replayedValues = {},
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
            function frame:GetSettingValue(setting)
                for _, settingInfo in ipairs(self.systemInfo.settings or {}) do
                    if settingInfo.setting == setting then
                        return settingInfo.value
                    end
                end
            end
            function frame:UpdateSystemSetting(setting, entireSystemUpdate)
                table.insert(self.replayedValues, tostring(setting) .. "=" .. tostring(self:GetSettingValue(setting)))
                self.entireSystemUpdate = entireSystemUpdate
            end
            function frame:UpdateSystem(systemInfo)
                self.updateSystemCalls = (self.updateSystemCalls or 0) + 1
                self.systemInfo = systemInfo
                for _, settingInfo in ipairs(systemInfo.settings or {}) do
                    self:UpdateSystemSetting(settingInfo.setting, true)
                end
            end

            return frame
        end

        EditModeManagerFrame = {
            layoutInfo = {},
            requestedIndices = {},
            registeredSystemFrames = {
                newAuraFrame(Enum.EditModeAuraFrameSystemIndices.BuffFrame, "BuffFrame"),
                newAuraFrame(Enum.EditModeAuraFrameSystemIndices.DebuffFrame, "DebuffFrame"),
                newAuraFrame(Enum.EditModeAuraFrameSystemIndices.ExternalDefensivesFrame, "ExternalDefensivesFrame"),
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
        function EditModeManagerFrame:UpdateSystem(systemFrame)
            -- Mirrors EditModeManagerFrameMixin:UpdateSystem -- the manager
            -- resolves the active layout itself; nothing pre-seeds the frame.
            systemFrame:UpdateSystem(self:GetActiveLayoutSystemInfo(systemFrame.system, systemFrame.systemIndex))
        end
        "#,
    )
    .expect("install aura frame stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply aura frame settings");

    let (requested_indices, replayed_values, update_system_calls): (String, String, String) = env
        .eval(
            r#"
            local replayedRows = {}
            local updateRows = {}
            for _, frame in ipairs(EditModeManagerFrame.registeredSystemFrames) do
                table.insert(replayedRows, table.concat(frame.replayedValues, ","))
                table.insert(updateRows, tostring(frame.updateSystemCalls or 0))
            end
            return table.concat(EditModeManagerFrame.requestedIndices, ","),
                table.concat(replayedRows, "|"),
                table.concat(updateRows, ",")
            "#,
        )
        .expect("read aura frame replay state");

    assert_eq!(requested_indices, "1,2,3");
    assert_eq!(
        replayed_values,
        "0=0,1=0,2=0,3=11,5=5,6=5|0=0,1=0,2=0,4=8,5=5,6=5|0=0,1=0,2=1,3=7,5=5,6=5,9=70",
        "active Widescreen aura-frame options should replay saved values"
    );
    assert_eq!(
        update_system_calls, "1,1,1",
        "AuraFrame rows should run through the manager update path"
    );
}

const COOLDOWN_VIEWER_STUBS: &str = r#"
        Enum = {
            EditModeSystem = {
                CooldownViewer = 20,
            },
            EditModeCooldownViewerSetting = {
                Orientation = 0,
                IconLimit = 1,
                IconDirection = 2,
                IconSize = 3,
                IconPadding = 4,
                Opacity = 5,
                VisibleSetting = 6,
                BarContent = 7,
                HideWhenInactive = 8,
                ShowTimer = 9,
                ShowTooltips = 10,
                BarWidthScale = 11,
            },
        }

        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return false end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local setting = Enum.EditModeCooldownViewerSetting
        local layoutRows = {
            [1] = {
                system = Enum.EditModeSystem.CooldownViewer,
                systemIndex = 1,
                isInDefaultPosition = false,
                anchorInfo = { point = "CENTER", relativeTo = UIParent, relativePoint = "CENTER", offsetX = 0, offsetY = 0 },
                settings = {
                    { setting = setting.Orientation, value = 0 },
                    { setting = setting.IconLimit, value = 12 },
                    { setting = setting.IconDirection, value = 1 },
                    { setting = setting.IconSize, value = 5 },
                    { setting = setting.IconPadding, value = 2 },
                    { setting = setting.Opacity, value = 100 },
                    { setting = setting.VisibleSetting, value = 0 },
                    { setting = setting.HideWhenInactive, value = 1 },
                    { setting = setting.ShowTimer, value = 1 },
                    { setting = setting.ShowTooltips, value = 1 },
                },
            },
            [2] = {
                system = Enum.EditModeSystem.CooldownViewer,
                systemIndex = 2,
                isInDefaultPosition = false,
                anchorInfo = { point = "CENTER", relativeTo = UIParent, relativePoint = "CENTER", offsetX = 10, offsetY = 0 },
                settings = {
                    { setting = setting.Orientation, value = 0 },
                    { setting = setting.IconLimit, value = 7 },
                    { setting = setting.IconDirection, value = 1 },
                    { setting = setting.IconSize, value = 5 },
                    { setting = setting.IconPadding, value = 2 },
                    { setting = setting.Opacity, value = 100 },
                    { setting = setting.VisibleSetting, value = 0 },
                    { setting = setting.HideWhenInactive, value = 1 },
                    { setting = setting.ShowTimer, value = 1 },
                    { setting = setting.ShowTooltips, value = 1 },
                },
            },
            [3] = {
                system = Enum.EditModeSystem.CooldownViewer,
                systemIndex = 3,
                isInDefaultPosition = false,
                anchorInfo = { point = "CENTER", relativeTo = UIParent, relativePoint = "CENTER", offsetX = 20, offsetY = 0 },
                settings = {
                    { setting = setting.Orientation, value = 0 },
                    { setting = setting.IconLimit, value = 1 },
                    { setting = setting.IconDirection, value = 1 },
                    { setting = setting.IconSize, value = 5 },
                    { setting = setting.IconPadding, value = 5 },
                    { setting = setting.Opacity, value = 100 },
                    { setting = setting.VisibleSetting, value = 0 },
                    { setting = setting.HideWhenInactive, value = 1 },
                    { setting = setting.ShowTimer, value = 1 },
                    { setting = setting.ShowTooltips, value = 1 },
                },
            },
            [4] = {
                system = Enum.EditModeSystem.CooldownViewer,
                systemIndex = 4,
                isInDefaultPosition = false,
                anchorInfo = { point = "CENTER", relativeTo = UIParent, relativePoint = "CENTER", offsetX = 30, offsetY = 0 },
                settings = {
                    { setting = setting.Orientation, value = 1 },
                    { setting = setting.IconLimit, value = 1 },
                    { setting = setting.IconDirection, value = 0 },
                    { setting = setting.IconSize, value = 5 },
                    { setting = setting.IconPadding, value = 5 },
                    { setting = setting.Opacity, value = 100 },
                    { setting = setting.VisibleSetting, value = 0 },
                    { setting = setting.BarContent, value = 0 },
                    { setting = setting.HideWhenInactive, value = 1 },
                    { setting = setting.ShowTimer, value = 1 },
                    { setting = setting.ShowTooltips, value = 1 },
                },
            },
        }

        local function copySettings(settings)
            local copied = {}
            for index, settingInfo in ipairs(settings or {}) do
                copied[index] = { setting = settingInfo.setting, value = settingInfo.value }
            end
            return copied
        end

        local function copySystemInfo(row)
            return {
                system = row.system,
                systemIndex = row.systemIndex,
                isInDefaultPosition = row.isInDefaultPosition,
                anchorInfo = row.anchorInfo,
                settings = copySettings(row.settings),
            }
        end

        local function settingValue(systemInfo, settingId)
            for _, settingInfo in ipairs(systemInfo.settings or {}) do
                if settingInfo.setting == settingId then
                    return settingInfo.value
                end
            end
            return nil
        end

        local function newCooldownViewer(index)
            local frame = {
                system = Enum.EditModeSystem.CooldownViewer,
                systemIndex = index,
                name = "CooldownViewer" .. index,
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

            function frame:UpdateSystem(systemInfo)
                self.updateSystemCalls = (self.updateSystemCalls or 0) + 1
                self.appliedSettings = {}
                for _, settingInfo in ipairs(systemInfo.settings or {}) do
                    self.appliedSettings[settingInfo.setting] = settingInfo.value
                end
                self.orientation = settingValue(systemInfo, setting.Orientation)
                self.iconLimit = settingValue(systemInfo, setting.IconLimit)
                self.iconDirection = settingValue(systemInfo, setting.IconDirection)
                self.iconSize = settingValue(systemInfo, setting.IconSize)
                self.iconPadding = settingValue(systemInfo, setting.IconPadding)
                self.opacity = settingValue(systemInfo, setting.Opacity)
                self.visibleSetting = settingValue(systemInfo, setting.VisibleSetting)
                self.barContent = settingValue(systemInfo, setting.BarContent)
                self.hideWhenInactive = settingValue(systemInfo, setting.HideWhenInactive)
                self.showTimer = settingValue(systemInfo, setting.ShowTimer)
                self.showTooltips = settingValue(systemInfo, setting.ShowTooltips)
                self.barWidthScale = settingValue(systemInfo, setting.BarWidthScale)
            end

            return frame
        end

        EditModeManagerFrame = {
            layoutInfo = {},
            registeredSystemFrames = {
                newCooldownViewer(1),
                newCooldownViewer(2),
                newCooldownViewer(3),
                newCooldownViewer(4),
            },
            requestedIndices = {},
        }

        function EditModeManagerFrame:InitSystemAnchors()
            self.initSystemAnchorsCalled = true
        end

        function EditModeManagerFrame:GetActiveLayoutSystemInfo(system, systemIndex)
            table.insert(self.requestedIndices, systemIndex)
            if system ~= Enum.EditModeSystem.CooldownViewer then
                return nil
            end
            local row = layoutRows[systemIndex]
            if not row then
                return nil
            end
            return copySystemInfo(row)
        end

        function EditModeManagerFrame:UpdateSystem(systemFrame)
            local systemInfo = systemFrame.systemInfo or self:GetActiveLayoutSystemInfo(systemFrame.system, systemFrame.systemIndex)
            systemFrame:UpdateSystem(systemInfo)
        end
        "#;

fn install_cooldown_viewer_stubs(env: &WowLuaEnv) {
    env.exec(COOLDOWN_VIEWER_STUBS)
        .expect("install cooldown viewer stubs");
}

#[test]
fn apply_system_anchors_updates_each_cooldown_viewer_profile_row() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    install_cooldown_viewer_stubs(&env);

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply cooldown viewer rows");

    let (
        requested_indices,
        calls_0,
        calls_1,
        calls_2,
        calls_3,
        icon_limit_0,
        icon_limit_1,
        icon_padding_2,
        orientation_3,
        bar_content_3,
        show_timer_3,
        bar_width_scale_3,
    ): (
        String,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        Option<i32>,
    ) = env
        .eval(
            r#"
            local frames = EditModeManagerFrame.registeredSystemFrames
            return table.concat(EditModeManagerFrame.requestedIndices, ","),
                frames[1].updateSystemCalls,
                frames[2].updateSystemCalls,
                frames[3].updateSystemCalls,
                frames[4].updateSystemCalls,
                frames[1].iconLimit,
                frames[2].iconLimit,
                frames[3].iconPadding,
                frames[4].orientation,
                frames[4].barContent,
                frames[4].showTimer,
                frames[4].barWidthScale
            "#,
        )
        .expect("read cooldown viewer state");

    assert_eq!(
        requested_indices, "1,2,3,4",
        "each CooldownViewer systemIndex should request its matching saved row"
    );
    assert_eq!(calls_0, 1);
    assert_eq!(calls_1, 1);
    assert_eq!(calls_2, 1);
    assert_eq!(calls_3, 1);
    assert_eq!(icon_limit_0, 12);
    assert_eq!(icon_limit_1, 7);
    assert_eq!(icon_padding_2, 5);
    assert_eq!(orientation_3, 1);
    assert_eq!(bar_content_3, 0);
    assert_eq!(show_timer_3, 1);
    assert_eq!(
        bar_width_scale_3, None,
        "active Widescreen row should not invent absent BarWidthScale"
    );
}

#[test]
fn apply_system_anchors_preserves_first_apply_dirtiness_for_manager_update() {
    // Regression: seed_system_frame builds the settingMap before the fallback
    // emm.UpdateSystem runs. UpdateSystem's own UpdateSettingMap(true) then
    // diffed against that identical map, reset dirtySettings to {}, and every
    // dirty-gated per-setting handler was skipped — DebuffFrame's
    // ShowDispelType never reached UpdateSystemSettingShowDispelType, so
    // dispellable player debuffs rendered the plain border instead of the
    // dispel-icon atlas.
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = { AuraFrame = 6 },
            EditModeAuraFrameSystemIndices = { DebuffFrame = 2 },
            EditModeAuraFrameSetting = { ShowDispelType = 10 },
        }
        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return false end,
            IsRightAnchoredActionBar = function() return false end,
        }

        -- Faithful Blizzard-style dirty gating: handlers only run for settings
        -- the UpdateSettingMap diff marked dirty.
        local frame = {
            system = Enum.EditModeSystem.AuraFrame,
            systemIndex = Enum.EditModeAuraFrameSystemIndices.DebuffFrame,
            name = "DebuffFrame",
            dirtySettings = {},
            appliedSettings = {},
        }
        function frame:GetName() return self.name end
        function frame:SetHasActiveChanges(value) self.hasActiveChanges = value end
        function frame:MarkAllSettingsDirty() self.settingMap = nil end
        function frame:IsSettingDirty(setting) return self.dirtySettings[setting] end
        function frame:ClearDirtySetting(setting) self.dirtySettings[setting] = nil end
        function frame:UpdateSettingMap(updateDirtySettings)
            local old = self.settingMap
            self.settingMap = {}
            for _, info in ipairs(self.systemInfo.settings or {}) do
                self.settingMap[info.setting] = { value = info.value }
            end
            if updateDirtySettings then
                self.dirtySettings = {}
                for setting, info in pairs(self.settingMap) do
                    if not old or not old[setting] or old[setting].value ~= info.value then
                        self.dirtySettings[setting] = true
                    end
                end
            end
        end
        function frame:GetSettingValue(setting)
            local info = self.settingMap and self.settingMap[setting]
            return info and info.value
        end
        function frame:UpdateSystemSetting(setting, entireSystemUpdate)
            if not self:IsSettingDirty(setting) then return end
            self.appliedSettings[setting] = self:GetSettingValue(setting)
            self:ClearDirtySetting(setting)
        end
        function frame:UpdateSystem(systemInfo)
            self.systemInfo = systemInfo
            self:UpdateSettingMap(true)
            for _, info in ipairs(systemInfo.settings or {}) do
                self:UpdateSystemSetting(info.setting, true)
            end
        end

        EditModeManagerFrame = {
            layoutInfo = {},
            registeredSystemFrames = { frame },
        }
        function EditModeManagerFrame:InitSystemAnchors() end
        function EditModeManagerFrame:GetActiveLayoutSystemInfo(system, systemIndex)
            return {
                system = system,
                systemIndex = systemIndex,
                isInDefaultPosition = false,
                anchorInfo = { point = "TOPRIGHT", relativeTo = UIParent, relativePoint = "TOPRIGHT", offsetX = -270, offsetY = -155 },
                settings = {
                    { setting = Enum.EditModeAuraFrameSetting.ShowDispelType, value = 1 },
                },
            }
        end
        function EditModeManagerFrame:UpdateSystem(systemFrame)
            -- Mirrors EditModeManagerFrameMixin:UpdateSystem -- the manager
            -- resolves the active layout itself; nothing pre-seeds the frame.
            systemFrame:UpdateSystem(self:GetActiveLayoutSystemInfo(systemFrame.system, systemFrame.systemIndex))
        end
        "#,
    )
    .expect("install dirty-gated aura frame stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply system anchors");

    let applied: f64 = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            return frame.appliedSettings[Enum.EditModeAuraFrameSetting.ShowDispelType] or -1
            "#,
        )
        .expect("read applied ShowDispelType");
    assert_eq!(
        applied, 1.0,
        "seeded settingMap must not swallow first-apply dirtiness for dirty-gated setting handlers"
    );
}
