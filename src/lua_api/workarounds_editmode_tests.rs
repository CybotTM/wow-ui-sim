use super::{
    APPLY_SYSTEM_ANCHORS_LUA, FIX_ACTION_BAR_NAN_SIZE_LUA, SETUP_LAYOUT_INFO_LUA, WowLuaEnv,
};

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
fn setup_layout_info_preserves_distinct_saved_action_bar_profile_settings() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                ActionBar = 0,
            },
            EditModeActionBarSetting = {
                IconSize = 3,
                HideBarArt = 6,
            },
            EditModeActionBarSystemIndices = {
                MainBar = 1,
            },
            EditModeLayoutType = {
                Preset = 0,
                Account = 1,
            },
        }

        C_EditMode = {
            GetLayouts = function()
                return {
                    activeLayout = 9,
                    layouts = {
                        {
                            layoutIndex = 9,
                            layoutName = "Ultrawide",
                            layoutType = Enum.EditModeLayoutType.Account,
                            systems = {
                                {
                                    system = Enum.EditModeSystem.ActionBar,
                                    systemIndex = Enum.EditModeActionBarSystemIndices.MainBar,
                                    isInDefaultPosition = false,
                                    anchorInfo = { point = "BOTTOMRIGHT", offsetX = -407.5, offsetY = 87.3 },
                                    settings = {
                                        { setting = Enum.EditModeActionBarSetting.IconSize, value = 2 },
                                        { setting = Enum.EditModeActionBarSetting.HideBarArt, value = 1 },
                                    },
                                },
                            },
                        },
                        {
                            layoutIndex = 10,
                            layoutName = "Widescreen",
                            layoutType = Enum.EditModeLayoutType.Account,
                            systems = {
                                {
                                    system = Enum.EditModeSystem.ActionBar,
                                    systemIndex = Enum.EditModeActionBarSystemIndices.MainBar,
                                    isInDefaultPosition = false,
                                    anchorInfo = { point = "BOTTOMRIGHT", offsetX = -407.5, offsetY = 87.3 },
                                    settings = {
                                        { setting = Enum.EditModeActionBarSetting.IconSize, value = 4 },
                                        { setting = Enum.EditModeActionBarSetting.HideBarArt, value = 1 },
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
            for _, element in ipairs(addedArray) do
                table.insert(tbl, element)
            end
        end

        EditModeManagerFrame = {}
        "#,
    )
    .expect("install multi-profile edit mode stubs");

    env.exec(SETUP_LAYOUT_INFO_LUA)
        .expect("run setup layout info");

    let (active_name, ultrawide_summary, widescreen_summary): (String, String, String) = env
        .eval(
            r#"
            local layoutInfo = EditModeManagerFrame.layoutInfo
            local function actionBarSummary(layout)
                local systemInfo = layout.systems[1]
                local values = {}
                for _, settingInfo in ipairs(systemInfo.settings) do
                    values[settingInfo.setting] = settingInfo.value
                end
                return table.concat({
                    layout.layoutName,
                    tostring(systemInfo.systemIndex),
                    tostring(values[Enum.EditModeActionBarSetting.IconSize]),
                    tostring(values[Enum.EditModeActionBarSetting.HideBarArt]),
                }, ":")
            end
            return layoutInfo.layouts[layoutInfo.activeLayout].layoutName,
                actionBarSummary(layoutInfo.layouts[3]),
                actionBarSummary(layoutInfo.layouts[4])
            "#,
        )
        .expect("read distinct saved action bar profile settings");

    assert_eq!(
        active_name, "Ultrawide",
        "active layout ID should select the matching saved profile after presets are prepended"
    );
    assert_eq!(ultrawide_summary, "Ultrawide:1:2:1");
    assert_eq!(widescreen_summary, "Widescreen:1:4:1");
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
fn setup_layout_info_initializes_account_settings_from_saved_cache() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeLayoutType = {
                Preset = 0,
                Account = 1,
            },
            EditModeAccountSetting = {
                ShowGrid = 0,
                GridSpacing = 1,
                ShowTimerBars = 25,
                ShowVehicleSeatIndicator = 26,
                ShowArchaeologyBar = 27,
                ShowTotemActionBar = 33,
            },
        }

        C_EditMode = {
            GetLayouts = function()
                return {
                    activeLayout = 1,
                    layouts = {},
                }
            end,
            GetAccountSettings = function()
                return {
                    { setting = Enum.EditModeAccountSetting.ShowGrid, value = 1 },
                    { setting = Enum.EditModeAccountSetting.GridSpacing, value = 42 },
                    { setting = Enum.EditModeAccountSetting.ShowTimerBars, value = 0 },
                    { setting = Enum.EditModeAccountSetting.ShowVehicleSeatIndicator, value = 0 },
                    { setting = Enum.EditModeAccountSetting.ShowArchaeologyBar, value = 0 },
                    { setting = Enum.EditModeAccountSetting.ShowTotemActionBar, value = 0 },
                }
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
        function EditModeManagerFrame:InitializeAccountSettings()
            self.accountSettings = C_EditMode.GetAccountSettings()
            self.accountSettingsInitialized = true
            self.showGrid = self.accountSettings[1].value
            self.gridSpacing = self.accountSettings[2].value
        end

        EditModeManagerFrame.AccountSettings = {
            timerBarsShown = true,
            vehicleSeatIndicatorShown = true,
            archaeologyBarShown = true,
            totemActionBarShown = true,
        }
        function EditModeManagerFrame.AccountSettings:SetTimerBarsShown(value)
            self.timerBarsShown = value
        end
        function EditModeManagerFrame.AccountSettings:SetVehicleSeatIndicatorShown(value)
            self.vehicleSeatIndicatorShown = value
        end
        function EditModeManagerFrame.AccountSettings:SetArchaeologyBarShown(value)
            self.archaeologyBarShown = value
        end
        function EditModeManagerFrame.AccountSettings:SetTotemActionBarShown(value)
            self.totemActionBarShown = value
        end
        "#,
    )
    .expect("install account setting stubs");

    env.exec(SETUP_LAYOUT_INFO_LUA)
        .expect("setup layout info should initialize account settings");

    let (
        initialized,
        show_grid,
        grid_spacing,
        timer_bars_shown,
        vehicle_seat_indicator_shown,
        archaeology_bar_shown,
        totem_action_bar_shown,
    ): (bool, i32, i32, bool, bool, bool, bool) = env
        .eval(
            r#"
            return EditModeManagerFrame.accountSettingsInitialized,
                EditModeManagerFrame.showGrid,
                EditModeManagerFrame.gridSpacing,
                EditModeManagerFrame.AccountSettings.timerBarsShown,
                EditModeManagerFrame.AccountSettings.vehicleSeatIndicatorShown,
                EditModeManagerFrame.AccountSettings.archaeologyBarShown,
                EditModeManagerFrame.AccountSettings.totemActionBarShown
            "#,
        )
        .expect("read account setting state");

    assert!(
        initialized,
        "saved account settings should be applied through Blizzard's initializer"
    );
    assert_eq!(show_grid, 1);
    assert_eq!(grid_spacing, 42);
    assert!(
        !timer_bars_shown,
        "saved timer bars visibility should be applied during account initialization"
    );
    assert!(
        !vehicle_seat_indicator_shown,
        "saved vehicle seat visibility should be applied during account initialization"
    );
    assert!(
        !archaeology_bar_shown,
        "saved archaeology bar visibility should be applied during account initialization"
    );
    assert!(
        !totem_action_bar_shown,
        "saved totem action bar visibility should be applied during account initialization"
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
        "1:1:8:4:2:true:true:false:true:_|2:4:6:5:2:nil:nil:nil:true:Always|3:1:12:2:2:nil:nil:nil:false:Always|4:1:8:4:2:nil:nil:nil:false:Always|5:1:8:4:2:nil:nil:nil:true:Always|6:1:12:3:2:nil:nil:nil:true:Always|7:1:12:5:2:nil:nil:nil:true:Always|8:1:12:5:2:nil:nil:nil:true:Always|11:3:nil:5:2:nil:nil:nil:nil:_|12:1:nil:5:2:nil:nil:nil:false:_|13:1:nil:5:2:nil:nil:nil:nil:_",
        "each active Widescreen action-bar row should replay its saved profile settings"
    );
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
fn apply_system_anchors_replays_buffs_on_top_when_aura_update_exists() {
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
            systemIndex = 2,
            name = "TargetFrame",
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
            self.anchorApplied = true
        end

        function frame:UpdateAuras()
            self.aurasUpdated = true
        end

        function frame:UpdateSystemSetting(setting, entireSystemUpdate)
            table.insert(self.updatedSettings, tostring(setting) .. ":" .. tostring(entireSystemUpdate))
            if setting == Enum.EditModeUnitFrameSetting.BuffsOnTop then
                self:UpdateAuras()
            end
        end

        EditModeManagerFrame = {
            layoutInfo = {},
            registeredSystemFrames = { frame },
        }

        function EditModeManagerFrame:InitSystemAnchors()
            self.initSystemAnchorsCalled = true
        end

        function EditModeManagerFrame:GetActiveLayoutSystemInfo()
            return {
                system = Enum.EditModeSystem.UnitFrame,
                systemIndex = 2,
                isInDefaultPosition = false,
                anchorInfo = {
                    point = "TOPLEFT",
                    relativeTo = UIParent,
                    relativePoint = "TOPLEFT",
                    offsetX = 10,
                    offsetY = -10,
                },
                settings = {
                    { setting = Enum.EditModeUnitFrameSetting.BuffsOnTop, value = 0 },
                    { setting = Enum.EditModeUnitFrameSetting.FrameSize, value = 0 },
                },
            }
        end

        function EditModeManagerFrame:UpdateSystem()
            error("target frame startup should not run the full update path")
        end
        "#,
    )
    .expect("install target frame stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply target frame anchors");

    let (updated_settings, auras_updated): (String, bool) = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            return table.concat(frame.updatedSettings, ","),
                frame.aurasUpdated or false
            "#,
        )
        .expect("read target frame state");

    assert_eq!(
        updated_settings, "2:true,16:true",
        "BuffsOnTop and FrameSize should both replay for unit frames with UpdateAuras"
    );
    assert!(
        auras_updated,
        "BuffsOnTop replay should be able to refresh unit-frame auras"
    );
}

#[test]
fn apply_system_anchors_replays_active_widescreen_unit_frame_settings() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                UnitFrame = 3,
            },
            EditModeUnitFrameSetting = {
                CastBarUnderneath = 1,
                BuffsOnTop = 2,
                UseLargerFrame = 3,
                UseRaidStylePartyFrames = 4,
                ShowPartyFrameBackground = 5,
                UseHorizontalGroups = 6,
                ViewRaidSize = 9,
                FrameWidth = 10,
                FrameHeight = 11,
                DisplayBorder = 12,
                RaidGroupDisplayType = 13,
                SortPlayersBy = 14,
                RowSize = 15,
                FrameSize = 16,
                ViewArenaSize = 17,
            },
        }

        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return false end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local setting = Enum.EditModeUnitFrameSetting
        local activeRows = {
            [1] = { setting.CastBarUnderneath, setting.FrameSize },
            [2] = { setting.BuffsOnTop, setting.FrameSize },
            [3] = { setting.BuffsOnTop, setting.UseLargerFrame, setting.FrameSize },
            [4] = {
                setting.UseRaidStylePartyFrames,
                setting.ShowPartyFrameBackground,
                setting.UseHorizontalGroups,
                setting.FrameWidth,
                setting.FrameHeight,
                setting.DisplayBorder,
                setting.SortPlayersBy,
                setting.FrameSize,
            },
            [5] = {
                setting.ViewRaidSize,
                setting.FrameWidth,
                setting.FrameHeight,
                setting.DisplayBorder,
                setting.RaidGroupDisplayType,
                setting.SortPlayersBy,
                setting.RowSize,
            },
            [7] = {
                setting.FrameWidth,
                setting.FrameHeight,
                setting.DisplayBorder,
                setting.ViewArenaSize,
            },
        }

        local function copiedSettings(systemIndex)
            local copied = {}
            for _, settingId in ipairs(activeRows[systemIndex] or {}) do
                table.insert(copied, { setting = settingId, value = 0 })
            end
            return copied
        end

        local function newFrame(name, systemIndex)
            local frame = {
                system = Enum.EditModeSystem.UnitFrame,
                systemIndex = systemIndex,
                name = name,
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
                self.anchorApplied = true
            end

            function frame:UpdateAuras()
                self.aurasUpdated = true
            end

            function frame:UpdateSystemSetting(settingId, entireSystemUpdate)
                table.insert(self.updatedSettings, tostring(settingId) .. ":" .. tostring(entireSystemUpdate))
            end

            return frame
        end

        EditModeManagerFrame = {
            layoutInfo = {},
            requestedIndices = {},
            registeredSystemFrames = {
                newFrame("PlayerFrame", 1),
                newFrame("TargetFrame", 2),
                newFrame("FocusFrame", 3),
                newFrame("PartyFrame", 4),
                newFrame("CompactRaidFrameContainer", 5),
                newFrame("CompactArenaFrame", 7),
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
                anchorInfo = {
                    point = "CENTER",
                    relativeTo = UIParent,
                    relativePoint = "CENTER",
                    offsetX = 0,
                    offsetY = 0,
                },
                settings = copiedSettings(systemIndex),
            }
        end

        function EditModeManagerFrame:UpdateSystem()
            error("active unit frame shortcut rows should not run full startup UpdateSystem")
        end
        "#,
    )
    .expect("install active unit frame row stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply active unit frame row settings");

    let (requested_indices, replayed_settings): (String, String) = env
        .eval(
            r#"
            local replayed = {}
            for _, frame in ipairs(EditModeManagerFrame.registeredSystemFrames) do
                table.insert(replayed, table.concat(frame.updatedSettings, ","))
            end
            return table.concat(EditModeManagerFrame.requestedIndices, ","),
                table.concat(replayed, "|")
            "#,
        )
        .expect("read active unit frame replayed settings");

    assert_eq!(requested_indices, "1,2,3,4,5,7");
    assert_eq!(
        replayed_settings,
        "1:true,16:true|2:true,16:true|2:true,3:true,16:true|4:true,5:true,6:true,10:true,11:true,12:true,14:true,16:true|9:true,10:true,11:true,12:true,13:true,14:true,15:true|10:true,11:true,12:true,17:true",
        "every active Widescreen unit-frame setting on shortcut frames should replay"
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

#[test]
fn apply_system_anchors_replays_active_widescreen_singleton_settings() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                CastBar = 1,
                Minimap = 2,
                ChatFrame = 8,
                ObjectiveTracker = 12,
                MicroMenu = 13,
                Bags = 14,
                DurabilityFrame = 16,
                TimerBars = 17,
                VehicleSeatIndicator = 18,
                ArchaeologyBar = 19,
            },
        }

        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return false end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local widescreenRows = {
            [Enum.EditModeSystem.CastBar] = { {0, 0}, {1, 0}, {2, 0} },
            [Enum.EditModeSystem.Minimap] = { {0, 0}, {1, 0}, {2, 5} },
            [Enum.EditModeSystem.ChatFrame] = { {0, 3}, {1, 48}, {2, 1}, {3, 20} },
            [Enum.EditModeSystem.ObjectiveTracker] = { {0, 1}, {1, 0}, {2, 0} },
            [Enum.EditModeSystem.MicroMenu] = { {0, 1}, {1, 0}, {2, 6}, {3, 4} },
            [Enum.EditModeSystem.Bags] = { {0, 0}, {1, 0}, {2, 5} },
            [Enum.EditModeSystem.DurabilityFrame] = { {0, 5} },
            [Enum.EditModeSystem.TimerBars] = { {0, 0} },
            [Enum.EditModeSystem.VehicleSeatIndicator] = { {0, 10} },
            [Enum.EditModeSystem.ArchaeologyBar] = { {0, 0} },
        }

        local function settingsFor(system)
            local settings = {}
            for _, pair in ipairs(widescreenRows[system] or {}) do
                table.insert(settings, { setting = pair[1], value = pair[2] })
            end
            return settings
        end

        local function newSingletonFrame(system, name)
            local frame = {
                system = system,
                systemIndex = nil,
                name = name,
                replayedSettings = {},
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
                table.insert(self.replayedSettings, setting)
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
            requestedRows = {},
            registeredSystemFrames = {
                newSingletonFrame(Enum.EditModeSystem.CastBar, "PlayerCastingBarFrame"),
                newSingletonFrame(Enum.EditModeSystem.Minimap, "MinimapCluster"),
                newSingletonFrame(Enum.EditModeSystem.ChatFrame, "ChatFrame1"),
                newSingletonFrame(Enum.EditModeSystem.ObjectiveTracker, "ObjectiveTrackerFrame"),
                newSingletonFrame(Enum.EditModeSystem.MicroMenu, "MicroMenu"),
                newSingletonFrame(Enum.EditModeSystem.Bags, "BagsBar"),
                newSingletonFrame(Enum.EditModeSystem.DurabilityFrame, "DurabilityFrame"),
                newSingletonFrame(Enum.EditModeSystem.TimerBars, "TimerBarsFrame"),
                newSingletonFrame(Enum.EditModeSystem.VehicleSeatIndicator, "VehicleSeatIndicator"),
                newSingletonFrame(Enum.EditModeSystem.ArchaeologyBar, "ArchaeologyBarFrame"),
            },
        }

        function EditModeManagerFrame:InitSystemAnchors()
            self.initSystemAnchorsCalled = true
        end
        function EditModeManagerFrame:GetActiveLayoutSystemInfo(system, systemIndex)
            table.insert(self.requestedRows, tostring(system) .. ":" .. tostring(systemIndex))
            return {
                system = system,
                systemIndex = systemIndex,
                isInDefaultPosition = false,
                anchorInfo = { point = "CENTER", relativeTo = UIParent, relativePoint = "CENTER", offsetX = 0, offsetY = 0 },
                settings = settingsFor(system),
            }
        end
        function EditModeManagerFrame:UpdateSystem(systemFrame)
            systemFrame:UpdateSystem(systemFrame.systemInfo)
        end
        "#,
    )
    .expect("install active singleton stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply active singleton settings");

    let (requested_rows, replayed_values, update_system_calls): (String, String, String) = env
        .eval(
            r#"
            local replayedRows = {}
            local updateRows = {}
            for _, frame in ipairs(EditModeManagerFrame.registeredSystemFrames) do
                table.insert(replayedRows, tostring(frame.system) .. ":" .. table.concat(frame.replayedValues, ","))
                table.insert(updateRows, tostring(frame.updateSystemCalls or 0))
            end
            return table.concat(EditModeManagerFrame.requestedRows, "|"),
                table.concat(replayedRows, "|"),
                table.concat(updateRows, ",")
            "#,
        )
        .expect("read singleton replay state");

    assert_eq!(
        requested_rows, "1:-1|2:-1|8:-1|12:-1|13:-1|14:-1|16:-1|17:-1|18:-1|19:-1",
        "singleton Widescreen systems should request the saved -1 row"
    );
    assert_eq!(
        replayed_values,
        "1:0=0,1=0,2=0|2:0=0,1=0,2=5|8:0=3,1=48,2=1,3=20|12:0=1,1=0,2=0|13:0=1,1=0,2=6,3=4|14:0=0,1=0,2=5|16:0=5|17:0=0|18:0=10|19:0=0",
        "every active Widescreen singleton option row should replay its saved setting values"
    );
    assert_eq!(
        update_system_calls, "0,1,1,1,1,1,1,1,1,1",
        "only the cast bar should use the direct startup replay branch"
    );
}

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
            systemFrame:UpdateSystem(systemFrame.systemInfo)
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

    assert_eq!(requested_indices, "1,2");
    assert_eq!(
        replayed_values, "0=0,1=0,2=0,3=11,5=5,6=5|0=0,1=0,2=0,4=8,5=5,6=5",
        "active Widescreen BuffFrame and DebuffFrame options should replay saved values"
    );
    assert_eq!(
        update_system_calls, "1,1",
        "AuraFrame rows should run through the manager update path"
    );
}

#[test]
fn apply_system_anchors_updates_each_cooldown_viewer_profile_row() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
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
        "#,
    )
    .expect("install cooldown viewer stubs");

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
