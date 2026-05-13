use super::{APPLY_SYSTEM_ANCHORS_LUA, SETUP_LAYOUT_INFO_LUA, WowLuaEnv};

#[test]
fn setup_layout_info_clones_preset_layouts_without_copytable() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                CastBar = 1,
                UnitFrame = 2,
            },
            EditModeCastBarSetting = {
                LockToPlayerFrame = 101,
            },
            EditModeLayoutType = {
                Preset = 0,
                Account = 1,
            },
            EditModeUnitFrameSetting = {
                CastBarUnderneath = 201,
                UseRaidStylePartyFrames = 202,
            },
            EditModeUnitFrameSystemIndices = {
                Player = 301,
                Party = 302,
            },
        }

        C_EditMode = {
            GetLayouts = function()
                return {
                    layouts = {
                        {
                            layoutIndex = 99,
                            layoutName = "Saved",
                            layoutType = Enum.EditModeLayoutType.Account,
                            systems = {
                                {
                                    system = 77,
                                    systemIndex = 88,
                                    isInDefaultPosition = false,
                                    anchorInfo = { point = "BOTTOM" },
                                    settings = {
                                        { setting = 501, value = 601 },
                                    },
                                },
                                {
                                    system = Enum.EditModeSystem.UnitFrame,
                                    systemIndex = Enum.EditModeUnitFrameSystemIndices.Party,
                                    isInDefaultPosition = false,
                                    anchorInfo = { point = "RIGHT" },
                                    settings = {
                                        { setting = Enum.EditModeUnitFrameSetting.UseRaidStylePartyFrames, value = 1 },
                                    },
                                },
                            },
                        },
                    },
                    activeLayout = 1,
                }
            end,
            GetAccountSettings = function()
                return {
                    { setting = 1, value = 0 },
                }
            end,
        }

        EditModePresetLayoutManager = {
            presetLayoutInfo = {
                {
                    layoutIndex = 1,
                    layoutName = "Preset",
                    layoutType = 0,
                    systems = {
                        {
                            system = Enum.EditModeSystem.CastBar,
                            systemIndex = 1,
                            isInDefaultPosition = true,
                            anchorInfo = { point = "TOP" },
                            settings = {
                                { setting = Enum.EditModeCastBarSetting.LockToPlayerFrame, value = 0 },
                            },
                        },
                        {
                            system = Enum.EditModeSystem.UnitFrame,
                            systemIndex = Enum.EditModeUnitFrameSystemIndices.Player,
                            isInDefaultPosition = true,
                            anchorInfo = { point = "LEFT" },
                            settings = {
                                { setting = Enum.EditModeUnitFrameSetting.CastBarUnderneath, value = 0 },
                            },
                        },
                    },
                },
            },
        }

        function tAppendAll(tbl, addedArray)
            for i, element in ipairs(addedArray) do
                table.insert(tbl, element)
            end
        end

        EditModeManagerFrame = {}
        "#,
    )
    .expect("install edit mode stubs");

    env.exec(SETUP_LAYOUT_INFO_LUA)
        .expect("run setup layout info");

    let (layout_count, lock_to_player, cast_bar_underneath, saved_layout_name): (
        i32,
        i32,
        i32,
        String,
    ) = env
        .eval(
            r#"
            local layouts = EditModeManagerFrame.layoutInfo.layouts
            local presetSystems = layouts[1].systems
            local savedLayout = layouts[2]
            return #layouts,
                presetSystems[1].settings[1].value,
                presetSystems[2].settings[1].value,
                savedLayout.layoutName
            "#,
        )
        .expect("read cloned layout info");

    assert_eq!(
        layout_count, 2,
        "preset and saved layouts should both be present"
    );
    assert_eq!(
        lock_to_player, 0,
        "inactive preset cast bar settings should not be rewritten when a saved layout is active"
    );
    assert_eq!(
        cast_bar_underneath, 0,
        "inactive preset player frame settings should not be rewritten when a saved layout is active"
    );
    assert_eq!(
        saved_layout_name, "Saved",
        "saved layouts should be appended"
    );

    env.exec(
        r#"
        EditModePresetLayoutManager.presetLayoutInfo[1].systems[1].settings[1].value = 999
        EditModePresetLayoutManager.presetLayoutInfo[1].systems[1].anchorInfo.point = "BROKEN"
        "#,
    )
    .expect("mutate original preset");

    let (copied_value, copied_point): (i32, String) = env
        .eval(
            r#"
            local system = EditModeManagerFrame.layoutInfo.layouts[1].systems[1]
            return system.settings[1].value, system.anchorInfo.point
            "#,
        )
        .expect("read cloned preset after source mutation");

    assert_eq!(
        copied_value, 0,
        "cloned settings must not alias preset source"
    );
    assert_eq!(
        copied_point, "TOP",
        "cloned anchor info must not alias preset source"
    );
}

#[test]
fn setup_layout_info_preserves_saved_cast_bar_lock_settings() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                CastBar = 1,
                UnitFrame = 2,
            },
            EditModeCastBarSetting = {
                LockToPlayerFrame = 101,
            },
            EditModeLayoutType = {
                Preset = 0,
                Account = 1,
            },
            EditModeUnitFrameSetting = {
                CastBarUnderneath = 201,
            },
            EditModeUnitFrameSystemIndices = {
                Player = 301,
            },
        }

        C_EditMode = {
            GetLayouts = function()
                return {
                    activeLayout = 1,
                    layouts = {
                        {
                            layoutIndex = 99,
                            layoutName = "Detached Cast Bar",
                            layoutType = Enum.EditModeLayoutType.Account,
                            systems = {
                                {
                                    system = Enum.EditModeSystem.CastBar,
                                    systemIndex = -1,
                                    isInDefaultPosition = false,
                                    anchorInfo = { point = "CENTER" },
                                    settings = {
                                        { setting = Enum.EditModeCastBarSetting.LockToPlayerFrame, value = 0 },
                                    },
                                },
                                {
                                    system = Enum.EditModeSystem.UnitFrame,
                                    systemIndex = Enum.EditModeUnitFrameSystemIndices.Player,
                                    isInDefaultPosition = false,
                                    anchorInfo = { point = "LEFT" },
                                    settings = {
                                        { setting = Enum.EditModeUnitFrameSetting.CastBarUnderneath, value = 0 },
                                    },
                                },
                            },
                        },
                    },
                }
            end,
            GetAccountSettings = function()
                return {}
            end,
        }

        EditModePresetLayoutManager = {
            presetLayoutInfo = {},
        }

        function tAppendAll(tbl, addedArray)
            for i, element in ipairs(addedArray) do
                table.insert(tbl, element)
            end
        end

        EditModeManagerFrame = {}
        "#,
    )
    .expect("install detached cast bar layout stubs");

    env.exec(SETUP_LAYOUT_INFO_LUA)
        .expect("run setup layout info");

    let (lock_to_player, cast_bar_underneath): (i32, i32) = env
        .eval(
            r#"
            local values = {}
            for _, systemInfo in ipairs(EditModeManagerFrame.layoutInfo.layouts[1].systems) do
                for _, settingInfo in ipairs(systemInfo.settings) do
                    values[settingInfo.setting] = settingInfo.value
                end
            end
            return values[Enum.EditModeCastBarSetting.LockToPlayerFrame],
                values[Enum.EditModeUnitFrameSetting.CastBarUnderneath]
            "#,
        )
        .expect("read active saved cast bar settings");

    assert_eq!(
        lock_to_player, 0,
        "active saved cast bar lock setting must not be overwritten"
    );
    assert_eq!(
        cast_bar_underneath, 0,
        "active saved player frame cast bar setting must not be overwritten"
    );
}

#[test]
fn setup_layout_info_remaps_active_saved_layout_after_prepending_presets() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                CastBar = 1,
                UnitFrame = 2,
            },
            EditModeCastBarSetting = {
                LockToPlayerFrame = 101,
            },
            EditModeLayoutType = {
                Preset = 0,
                Account = 1,
            },
            EditModeUnitFrameSetting = {
                CastBarUnderneath = 201,
                UseRaidStylePartyFrames = 202,
            },
            EditModeUnitFrameSystemIndices = {
                Player = 301,
                Party = 302,
            },
        }

        C_EditMode = {
            GetLayouts = function()
                return {
                    activeLayout = 2,
                    layouts = {
                        {
                            layoutIndex = 99,
                            layoutName = "Saved Narrow",
                            layoutType = Enum.EditModeLayoutType.Account,
                            systems = {},
                        },
                        {
                            layoutIndex = 100,
                            layoutName = "Saved Wide",
                            layoutType = Enum.EditModeLayoutType.Account,
                            systems = {
                                {
                                    system = Enum.EditModeSystem.UnitFrame,
                                    systemIndex = Enum.EditModeUnitFrameSystemIndices.Party,
                                    isInDefaultPosition = false,
                                    anchorInfo = { point = "RIGHT" },
                                    settings = {
                                        { setting = Enum.EditModeUnitFrameSetting.UseRaidStylePartyFrames, value = 1 },
                                    },
                                },
                            },
                        },
                    },
                }
            end,
            GetAccountSettings = function()
                return {}
            end,
        }

        EditModePresetLayoutManager = {
            presetLayoutInfo = {
                {
                    layoutIndex = 1,
                    layoutName = "Modern",
                    layoutType = Enum.EditModeLayoutType.Preset,
                    systems = {},
                },
                {
                    layoutIndex = 2,
                    layoutName = "Classic",
                    layoutType = Enum.EditModeLayoutType.Preset,
                    systems = {},
                },
            },
        }

        function tAppendAll(tbl, addedArray)
            for i, element in ipairs(addedArray) do
                table.insert(tbl, element)
            end
        end

        EditModeManagerFrame = {}
        "#,
    )
    .expect("install edit mode stubs");

    env.exec(SETUP_LAYOUT_INFO_LUA)
        .expect("run setup layout info");

    let (active_layout, active_name, saved_party_value): (i32, String, i32) = env
        .eval(
            r#"
            local layoutInfo = EditModeManagerFrame.layoutInfo
            local active = layoutInfo.layouts[layoutInfo.activeLayout]
            local party = active.systems[1]
            return layoutInfo.activeLayout,
                active.layoutName,
                party.settings[1].value
            "#,
        )
        .expect("read remapped active layout");

    assert_eq!(
        active_layout, 4,
        "second saved layout should be active after two presets are prepended"
    );
    assert_eq!(active_name, "Saved Wide");
    assert_eq!(
        saved_party_value, 1,
        "saved party-frame settings must not be overwritten by preset startup compatibility"
    );
}

#[test]
fn setup_layout_info_merges_default_action_bar_settings_into_saved_layout() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                CastBar = 1,
                UnitFrame = 2,
                ActionBar = 3,
            },
            EditModeCastBarSetting = {
                LockToPlayerFrame = 101,
            },
            EditModeLayoutType = {
                Preset = 0,
                Account = 1,
            },
            EditModeUnitFrameSetting = {
                CastBarUnderneath = 201,
                UseRaidStylePartyFrames = 202,
            },
            EditModeUnitFrameSystemIndices = {
                Player = 301,
                Party = 302,
            },
            EditModeActionBarSetting = {
                HideBarArt = 6,
                AlwaysShowButtons = 9,
            },
        }

        C_EditMode = {
            GetLayouts = function()
                return {
                    activeLayout = 1,
                    layouts = {
                        {
                            layoutIndex = 77,
                            layoutName = "Saved Sparse",
                            layoutType = Enum.EditModeLayoutType.Account,
                            systems = {
                                {
                                    system = Enum.EditModeSystem.ActionBar,
                                    systemIndex = 1,
                                    isInDefaultPosition = false,
                                    anchorInfo = { point = "BOTTOM" },
                                    settings = {
                                        { setting = Enum.EditModeActionBarSetting.AlwaysShowButtons, value = 1 },
                                    },
                                },
                            },
                        },
                    },
                }
            end,
            GetAccountSettings = function()
                return {}
            end,
        }

        EditModePresetLayoutManager = {
            presetLayoutInfo = {},
        }

        function EditModePresetLayoutManager:GetAllDefaultSettingsForSystem(system, systemIndex)
            if system == Enum.EditModeSystem.ActionBar and systemIndex == 1 then
                return {
                    [Enum.EditModeActionBarSetting.HideBarArt] = 0,
                    [Enum.EditModeActionBarSetting.AlwaysShowButtons] = 0,
                }
            end
            return {}
        end

        function tAppendAll(tbl, addedArray)
            for i, element in ipairs(addedArray) do
                table.insert(tbl, element)
            end
        end

        EditModeManagerFrame = {}
        "#,
    )
    .expect("install sparse saved layout stubs");

    env.exec(SETUP_LAYOUT_INFO_LUA)
        .expect("run setup layout info");

    let (hide_bar_art, always_show_buttons): (i32, i32) = env
        .eval(
            r#"
            local settings = EditModeManagerFrame.layoutInfo.layouts[1].systems[1].settings
            local values = {}
            for _, settingInfo in ipairs(settings) do
                values[settingInfo.setting] = settingInfo.value
            end
            return values[Enum.EditModeActionBarSetting.HideBarArt],
                values[Enum.EditModeActionBarSetting.AlwaysShowButtons]
            "#,
        )
        .expect("read merged action bar settings");

    assert_eq!(hide_bar_art, 0, "default side-art setting should be merged");
    assert_eq!(
        always_show_buttons, 1,
        "saved values must override default settings"
    );
}

#[test]
fn apply_system_anchors_skips_self_relative_saved_anchor() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                Buffs = 6,
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
                settings = {},
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

    let (anchor_calls, has_active_changes, setting_map_updated): (i32, bool, bool) = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            return frame.anchorCalls, frame.hasActiveChanges, frame.settingMapUpdated
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
}

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
                    { setting = Enum.EditModeActionBarSetting.IconSize, value = 80 },
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
                EditModeManagerFrame.actionBarLayoutUpdated
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
}

#[test]
fn apply_system_anchors_seeds_unit_frame_without_full_startup_update() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                UnitFrame = 3,
            },
            EditModeUnitFrameSetting = {
                BuffsOnTop = 2,
                FrameSize = 16,
            },
        }

        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return false end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local frame = {
            system = Enum.EditModeSystem.UnitFrame,
            systemIndex = 1,
            name = "PlayerFrame",
            anchorCalls = 0,
            updatedSettings = {},
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
        end

        function frame:UpdateSystemSetting(setting, entireSystemUpdate)
            if setting == Enum.EditModeUnitFrameSetting.BuffsOnTop then
                self.buffsAttempted = true
                error("BuffsOnTop should not replay without UpdateAuras")
            end
            table.insert(self.updatedSettings, {
                setting = setting,
                entireSystemUpdate = entireSystemUpdate,
            })
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
                system = Enum.EditModeSystem.UnitFrame,
                systemIndex = 1,
                isInDefaultPosition = false,
                anchorInfo = {
                    point = "TOPLEFT",
                    relativeTo = UIParent,
                    relativePoint = "TOPLEFT",
                    offsetX = -292.4,
                    offsetY = -144.4,
                },
                settings = {
                    { setting = Enum.EditModeUnitFrameSetting.BuffsOnTop, value = 1 },
                    { setting = Enum.EditModeUnitFrameSetting.FrameSize, value = 0 },
                },
            }
        end

        function EditModeManagerFrame:UpdateSystem()
            error("unit frame startup should not run the full update path")
        end
        "#,
    )
    .expect("install unit frame stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply unit frame anchors");

    let (
        anchor_calls,
        has_active_changes,
        setting_map_updated,
        updated_setting,
        entire_update,
        buffs_attempted,
    ): (i32, bool, bool, i32, bool, bool) = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            return frame.anchorCalls,
                frame.hasActiveChanges,
                frame.settingMapUpdated,
                frame.updatedSettings[1] and frame.updatedSettings[1].setting,
                frame.updatedSettings[1] and frame.updatedSettings[1].entireSystemUpdate,
                frame.buffsAttempted or false
            "#,
        )
        .expect("read unit frame state");

    assert_eq!(anchor_calls, 1, "saved unit frame anchor should apply");
    assert!(!has_active_changes, "system should be seeded as clean");
    assert!(
        setting_map_updated,
        "system settings should still be mapped"
    );
    assert_eq!(
        updated_setting, 16,
        "saved unit-frame settings should be applied even when full UpdateSystem is skipped"
    );
    assert!(
        entire_update,
        "startup setting application should use the full-update flag"
    );
    assert!(
        !buffs_attempted,
        "BuffsOnTop should wait until an aura update method exists"
    );
}

#[test]
fn apply_system_anchors_maps_nil_system_index_to_saved_singleton_index() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                CastBar = 1,
            },
        }

        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return false end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local frame = {
            system = Enum.EditModeSystem.CastBar,
            systemIndex = nil,
            name = "PlayerCastingBarFrame",
            updatedSettings = {},
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
            error("cast bar startup should not apply a scale-affecting anchor")
        end

        function frame:UpdateSystemSetting(setting, entireSystemUpdate)
            table.insert(self.updatedSettings, {
                setting = setting,
                entireSystemUpdate = entireSystemUpdate,
            })
        end

        EditModeManagerFrame = {
            layoutInfo = {},
            registeredSystemFrames = { frame },
            layoutApplyInProgress = false,
            requestedSystemIndex = nil,
        }

        function EditModeManagerFrame:InitSystemAnchors()
            self.initSystemAnchorsCalled = true
        end

        function EditModeManagerFrame:GetActiveLayoutSystemInfo(_system, systemIndex)
            self.requestedSystemIndex = systemIndex
            if systemIndex ~= -1 then
                return nil
            end
            return {
                system = Enum.EditModeSystem.CastBar,
                systemIndex = -1,
                isInDefaultPosition = false,
                anchorInfo = {
                    point = "CENTER",
                    relativeTo = UIParent,
                    relativePoint = "CENTER",
                    offsetX = 0,
                    offsetY = -174,
                },
                settings = {
                    { setting = 1, value = 0 },
                },
            }
        end

        function EditModeManagerFrame:UpdateSystem()
            error("nil-index singleton should be seeded directly")
        end
        "#,
    )
    .expect("install nil-index singleton stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply nil-index singleton anchors");

    let (requested_system_index, has_active_changes, setting_map_updated, updated_setting): (
        i32,
        bool,
        bool,
        i32,
    ) = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            return EditModeManagerFrame.requestedSystemIndex,
                frame.hasActiveChanges,
                frame.settingMapUpdated,
                frame.updatedSettings[1] and frame.updatedSettings[1].setting
            "#,
        )
        .expect("read nil-index singleton state");

    assert_eq!(requested_system_index, -1);
    assert!(!has_active_changes, "system should be seeded as clean");
    assert!(
        setting_map_updated,
        "system settings should still be mapped"
    );
    assert_eq!(
        updated_setting, 1,
        "nil-index singleton settings should be applied after seeding"
    );
}
