use super::{SETUP_LAYOUT_INFO_LUA, WowLuaEnv};

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
