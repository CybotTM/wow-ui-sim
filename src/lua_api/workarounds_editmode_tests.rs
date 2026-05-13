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

#[test]
fn apply_system_anchors_does_not_repack_action_bars_after_saved_anchor() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        Enum = {
            EditModeSystem = {
                ActionBar = 0,
            },
        }

        UIParent = { name = "UIParent" }
        EditModeUtil = {
            IsBottomAnchoredActionBar = function() return true end,
            IsRightAnchoredActionBar = function() return false end,
        }

        local frame = {
            system = Enum.EditModeSystem.ActionBar,
            systemIndex = 1,
            name = "MainActionBar",
            anchorCalls = 0,
            actionButtons = { {} },
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

        function frame:RefreshGridLayout()
            self.gridRefreshed = true
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
                settings = {},
            }
        end

        function EditModeManagerFrame:UpdateActionBarPositions()
            error("saved action bar anchors should not be repacked")
        end
        "#,
    )
    .expect("install action bar stubs");

    env.exec(APPLY_SYSTEM_ANCHORS_LUA)
        .expect("apply action bar anchors");

    let (anchor_calls, grid_refreshed): (i32, bool) = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            return frame.anchorCalls, frame.gridRefreshed
            "#,
        )
        .expect("read action bar state");

    assert_eq!(anchor_calls, 1, "saved action bar anchor should apply once");
    assert!(
        grid_refreshed,
        "action bar runtime layout should still refresh"
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
                    { setting = 16, value = 0 },
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

    let (anchor_calls, has_active_changes, setting_map_updated): (i32, bool, bool) = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            return frame.anchorCalls, frame.hasActiveChanges, frame.settingMapUpdated
            "#,
        )
        .expect("read unit frame state");

    assert_eq!(anchor_calls, 1, "saved unit frame anchor should apply");
    assert!(!has_active_changes, "system should be seeded as clean");
    assert!(
        setting_map_updated,
        "system settings should still be mapped"
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

    let (requested_system_index, has_active_changes, setting_map_updated): (i32, bool, bool) = env
        .eval(
            r#"
            local frame = EditModeManagerFrame.registeredSystemFrames[1]
            return EditModeManagerFrame.requestedSystemIndex,
                frame.hasActiveChanges,
                frame.settingMapUpdated
            "#,
        )
        .expect("read nil-index singleton state");

    assert_eq!(requested_system_index, -1);
    assert!(!has_active_changes, "system should be seeded as clean");
    assert!(
        setting_map_updated,
        "system settings should still be mapped"
    );
}
