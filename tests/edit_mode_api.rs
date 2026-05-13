use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::{SavedVariablesManager, WtfConfig};

#[test]
fn edit_mode_layout_api_persists_active_layout_and_saved_layouts() {
    let env = WowLuaEnv::new().expect("create Lua environment");

    let (initial_active, initial_count): (i32, i32) = env
        .eval(
            r#"
            local info = C_EditMode.GetLayouts()
            return info.activeLayout, #info.layouts
            "#,
        )
        .expect("read initial edit mode layouts");

    assert_eq!(initial_active, 1);
    assert_eq!(initial_count, 0);

    env.exec(
        r#"
        C_EditMode.SaveLayouts({
            activeLayout = 3,
            layouts = {
                {
                    layoutName = "Custom",
                    layoutType = Enum.EditModeLayoutType.Account,
                    systems = {
                        {
                            system = Enum.EditModeSystem.ActionBar,
                            systemIndex = Enum.EditModeActionBarSystemIndices.MainBar,
                            isInDefaultPosition = false,
                            anchorInfo = {
                                point = "CENTER",
                                relativeTo = "UIParent",
                                relativePoint = "CENTER",
                                offsetX = 42,
                                offsetY = -7,
                            },
                            settings = {
                                {
                                    setting = Enum.EditModeActionBarSetting.IconSize,
                                    value = 4,
                                },
                            },
                        },
                    },
                },
            },
        })
        C_EditMode.SetActiveLayout(4)
        "#,
    )
    .expect("save edit mode layout state");

    let (active, count, name, offset_x, icon_size): (i32, i32, String, i32, i32) = env
        .eval(
            r#"
            local info = C_EditMode.GetLayouts()
            local system = info.layouts[1].systems[1]
            return info.activeLayout,
                #info.layouts,
                info.layouts[1].layoutName,
                system.anchorInfo.offsetX,
                system.settings[1].value
            "#,
        )
        .expect("read saved edit mode layout state");

    assert_eq!(active, 4);
    assert_eq!(count, 1);
    assert_eq!(name, "Custom");
    assert_eq!(offset_x, 42);
    assert_eq!(icon_size, 4);
}

#[test]
fn edit_mode_layout_api_loads_wtf_cache_files() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let wtf_path = temp.path().join("WTF");
    let account_path = wtf_path.join("Account/TestAccount");
    let character_path = account_path.join("Test Realm/Testchar");
    std::fs::create_dir_all(&character_path).expect("create WTF dirs");
    std::fs::write(
        account_path.join("edit-mode-cache-account.txt"),
        concat!(
            "1 2 1 100 ",
            "7 Custom 1 ",
            "0 0 0 4 4 UIParent 12.5 -34.0 -1 ##$$",
            "\0"
        ),
    )
    .expect("write account edit mode cache");
    std::fs::write(
        character_path.join("edit-mode-cache-character.txt"),
        "3 3 3 3 3 3\0",
    )
    .expect("write character edit mode cache");

    let env = WowLuaEnv::new().expect("create Lua environment");
    let mut saved_vars = SavedVariablesManager::with_storage_dir(temp.path().join("local-sv"));
    saved_vars.set_wtf_config(WtfConfig::new(
        &wtf_path,
        "TestAccount",
        "Test Realm",
        "Testchar",
    ));
    env.loader_env()
        .with_state(|state| saved_vars.load_edit_mode_cache(state, 2))
        .expect("load edit mode cache");

    let (
        active,
        layout_count,
        grid_spacing,
        damage_meter_default,
        external_defensives_default,
        totem_action_bar_default,
        name,
        system,
        system_index,
        point,
        offset_x,
        setting_value,
    ): (i32, i32, i32, i32, i32, i32, String, i32, i32, String, f64, i32) = env
        .eval(
            r#"
            local info = C_EditMode.GetLayouts()
            local settings = C_EditMode.GetAccountSettings()
            local accountSettingMap = {}
            for _, settingInfo in ipairs(settings) do
                accountSettingMap[settingInfo.setting] = settingInfo.value
            end
            local system = info.layouts[1].systems[1]
            return info.activeLayout,
                #info.layouts,
                accountSettingMap[Enum.EditModeAccountSetting.GridSpacing],
                accountSettingMap[Enum.EditModeAccountSetting.ShowDamageMeter],
                accountSettingMap[Enum.EditModeAccountSetting.ShowExternalDefensives],
                accountSettingMap[Enum.EditModeAccountSetting.ShowTotemActionBar],
                info.layouts[1].layoutName,
                system.system,
                system.systemIndex,
                system.anchorInfo.point,
                system.anchorInfo.offsetX,
                system.settings[2].value
            "#,
        )
        .expect("read imported edit mode cache");

    assert_eq!(active, 3);
    assert_eq!(layout_count, 1);
    assert_eq!(grid_spacing, 100);
    assert_eq!(
        damage_meter_default, 1,
        "missing newer account settings should be filled from defaults"
    );
    assert_eq!(
        external_defensives_default, 1,
        "missing newer account settings should be filled from defaults"
    );
    assert_eq!(
        totem_action_bar_default, 1,
        "missing latest account settings should be filled from defaults"
    );
    assert_eq!(name, "Custom");
    assert_eq!(system, 0);
    assert_eq!(
        system_index, 1,
        "WTF cache stores system indices zero-based, but Lua layout state must use EditMode enum values"
    );
    assert_eq!(point, "BOTTOM");
    assert_eq!(offset_x, 12.5);
    assert_eq!(setting_value, 1);
}

#[test]
fn edit_mode_wtf_cache_normalizes_indexed_system_rows() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let wtf_path = temp.path().join("WTF");
    let account_path = wtf_path.join("Account/TestAccount");
    let character_path = account_path.join("Test Realm/Testchar");
    std::fs::create_dir_all(&character_path).expect("create WTF dirs");
    std::fs::write(
        account_path.join("edit-mode-cache-account.txt"),
        concat!(
            "1 0 ",
            "10 Widescreen 6 ",
            "0 0 0 4 4 UIParent 0.0 0.0 -1 ## ",
            "3 7 0 4 4 UIParent 0.0 0.0 -1 ## ",
            "6 1 0 4 4 UIParent 0.0 0.0 -1 ## ",
            "15 1 0 4 4 UIParent 0.0 0.0 -1 ## ",
            "20 3 0 4 4 UIParent 0.0 0.0 -1 ## ",
            "1 -1 0 4 4 UIParent 0.0 0.0 -1 ##",
            "\0"
        ),
    )
    .expect("write account edit mode cache");
    std::fs::write(
        character_path.join("edit-mode-cache-character.txt"),
        "1 1 1 1 1 1\0",
    )
    .expect("write character edit mode cache");

    let env = WowLuaEnv::new().expect("create Lua environment");
    let mut saved_vars = SavedVariablesManager::with_storage_dir(temp.path().join("local-sv"));
    saved_vars.set_wtf_config(WtfConfig::new(
        &wtf_path,
        "TestAccount",
        "Test Realm",
        "Testchar",
    ));
    env.loader_env()
        .with_state(|state| saved_vars.load_edit_mode_cache(state, 2))
        .expect("load edit mode cache");

    let (layout_name, indices): (String, String) = env
        .eval(
            r#"
            local info = C_EditMode.GetLayouts()
            local layout = info.layouts[1]
            local values = {}
            for _, systemInfo in ipairs(layout.systems) do
                table.insert(values, tostring(systemInfo.systemIndex))
            end
            return layout.layoutName, table.concat(values, ",")
            "#,
        )
        .expect("read normalized edit mode indices");

    assert_eq!(layout_name, "Widescreen");
    assert_eq!(
        indices, "1,8,2,2,4,-1",
        "indexed WTF rows are zero-based, but singleton rows must remain -1"
    );
}

#[test]
fn edit_mode_cache_decodes_repeated_setting_chunks_as_large_value() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let wtf_path = temp.path().join("WTF");
    let account_path = wtf_path.join("Account/TestAccount");
    let character_path = account_path.join("Test Realm/Testchar");
    std::fs::create_dir_all(&character_path).expect("create WTF dirs");
    std::fs::write(
        account_path.join("edit-mode-cache-account.txt"),
        concat!(
            "1 0 ",
            "1 Custom 1 ",
            "20 0 0 0 0 UIParent 0.0 0.0 -1 (-($",
            "\0"
        ),
    )
    .expect("write account edit mode cache");
    std::fs::write(
        character_path.join("edit-mode-cache-character.txt"),
        "1 1 1 1 1 1\0",
    )
    .expect("write character edit mode cache");

    let env = WowLuaEnv::new().expect("create Lua environment");
    let mut saved_vars = SavedVariablesManager::with_storage_dir(temp.path().join("local-sv"));
    saved_vars.set_wtf_config(WtfConfig::new(
        &wtf_path,
        "TestAccount",
        "Test Realm",
        "Testchar",
    ));
    env.loader_env()
        .with_state(|state| saved_vars.load_edit_mode_cache(state, 2))
        .expect("load edit mode cache");

    let (settings_count, setting, value): (i32, i32, i32) = env
        .eval(
            r#"
            local info = C_EditMode.GetLayouts()
            local settings = info.layouts[1].systems[1].settings
            return #settings, settings[1].setting, settings[1].value
            "#,
        )
        .expect("read decoded edit mode settings");

    assert_eq!(settings_count, 1);
    assert_eq!(setting, 5);
    assert_eq!(value, 100);
}

#[test]
fn compact_raid_group_type_enum_matches_edit_mode_unit_frame_indices() {
    let env = WowLuaEnv::new().expect("create Lua environment");

    let (party, raid, arena, edit_party, edit_raid, edit_arena): (i32, i32, i32, i32, i32, i32) =
        env.eval(
            r#"
            return CompactRaidGroupTypeEnum.Party,
                CompactRaidGroupTypeEnum.Raid,
                CompactRaidGroupTypeEnum.Arena,
                Enum.EditModeUnitFrameSystemIndices.Party,
                Enum.EditModeUnitFrameSystemIndices.Raid,
                Enum.EditModeUnitFrameSystemIndices.Arena
            "#,
        )
        .expect("read compact raid group type enum");

    assert_eq!(party, edit_party);
    assert_eq!(raid, edit_raid);
    assert_eq!(arena, edit_arena);
}

#[test]
fn unit_frame_edit_mode_setting_meta_includes_big_defensive_icon_size() {
    let env = WowLuaEnv::new().expect("create Lua environment");

    let (big_defensive_icon_size, min_value, max_value, num_values): (i32, i32, i32, i32) = env
        .eval(
            r#"
            local setting = Enum.EditModeUnitFrameSetting
            local meta = Enum.EditModeUnitFrameSettingMeta
            return setting.BigDefensiveIconSize,
                meta.MinValue,
                meta.MaxValue,
                meta.NumValues
            "#,
        )
        .expect("read unit frame edit mode setting enum");

    assert_eq!(big_defensive_icon_size, 21);
    assert_eq!(min_value, 0);
    assert_eq!(max_value, 21);
    assert_eq!(num_values, 22);
}

#[test]
fn cooldown_viewer_edit_mode_setting_ids_match_blizzard_order() {
    let env = WowLuaEnv::new().expect("create Lua environment");

    let (
        orientation,
        icon_limit,
        icon_direction,
        icon_size,
        icon_padding,
        bar_width_scale,
        opacity,
        visible_setting,
        bar_content,
        hide_when_inactive,
        show_timer,
        show_tooltips,
    ): (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = env
        .eval(
            r#"
            local setting = Enum.EditModeCooldownViewerSetting
            return setting.Orientation,
                setting.IconLimit,
                setting.IconDirection,
                setting.IconSize,
                setting.IconPadding,
                setting.BarWidthScale,
                setting.Opacity,
                setting.VisibleSetting,
                setting.BarContent,
                setting.HideWhenInactive,
                setting.ShowTimer,
                setting.ShowTooltips
            "#,
        )
        .expect("read cooldown viewer edit mode setting enum");

    assert_eq!(orientation, 0);
    assert_eq!(icon_limit, 1);
    assert_eq!(icon_direction, 2);
    assert_eq!(icon_size, 3);
    assert_eq!(icon_padding, 4);
    assert_eq!(opacity, 5);
    assert_eq!(visible_setting, 6);
    assert_eq!(bar_content, 7);
    assert_eq!(hide_when_inactive, 8);
    assert_eq!(show_timer, 9);
    assert_eq!(show_tooltips, 10);
    assert_eq!(bar_width_scale, 11);
}

#[test]
fn status_tracking_bar_edit_mode_setting_ids_match_live_docs() {
    let env = WowLuaEnv::new().expect("create Lua environment");

    let (height, width, text_size, size, min_value, max_value, num_values): (
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
    ) = env
        .eval(
            r#"
            local setting = Enum.EditModeStatusTrackingBarSetting
            local meta = Enum.EditModeStatusTrackingBarSettingMeta
            return setting.Height,
                setting.Width,
                setting.TextSize,
                setting.Size,
                meta.MinValue,
                meta.MaxValue,
                meta.NumValues
            "#,
        )
        .expect("read status tracking bar edit mode setting enum");

    assert_eq!(height, 0);
    assert_eq!(width, 1);
    assert_eq!(text_size, 2);
    assert_eq!(size, 3);
    assert_eq!(min_value, 0);
    assert_eq!(max_value, 3);
    assert_eq!(num_values, 4);
}

#[test]
fn edit_mode_account_setting_ids_include_totem_action_bar() {
    let env = WowLuaEnv::new().expect("create Lua environment");

    let (totem_action_bar, min_value, max_value, num_values): (i32, i32, i32, i32) = env
        .eval(
            r#"
            local setting = Enum.EditModeAccountSetting
            local meta = Enum.EditModeAccountSettingMeta
            return setting.ShowTotemActionBar,
                meta.MinValue,
                meta.MaxValue,
                meta.NumValues
            "#,
        )
        .expect("read edit mode account setting enum");

    assert_eq!(totem_action_bar, 33);
    assert_eq!(min_value, 0);
    assert_eq!(max_value, 33);
    assert_eq!(num_values, 34);
}

#[test]
fn encounter_events_icon_direction_matches_blizzard_docs() {
    let env = WowLuaEnv::new().expect("create Lua environment");

    let (left, right, top, bottom, min_value, max_value, num_values): (
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
    ) = env
        .eval(
            r#"
            local direction = Enum.EncounterEventsIconDirection
            local meta = Enum.EncounterEventsIconDirectionMeta
            return direction.Left,
                direction.Right,
                direction.Top,
                direction.Bottom,
                meta.MinValue,
                meta.MaxValue,
                meta.NumValues
            "#,
        )
        .expect("read encounter events icon direction enum");

    assert_eq!(left, 0);
    assert_eq!(right, 1);
    assert_eq!(top, 0);
    assert_eq!(bottom, 1);
    assert_eq!(min_value, 0);
    assert_eq!(max_value, 1);
    assert_eq!(num_values, 4);
}

#[test]
fn edit_mode_profile_option_enums_match_blizzard_docs() {
    let env = WowLuaEnv::new().expect("create Lua environment");

    env.exec(
        r#"
        local expected = {
            ActionBarOrientation = { Horizontal = 0, Vertical = 1 },
            ActionBarVisibleSetting = { Always = 0, InCombat = 1, OutOfCombat = 2, Hidden = 3 },
            AuraFrameIconDirection = { Down = 0, Up = 1, Left = 0, Right = 1 },
            AuraFrameIconWrap = { Down = 0, Up = 1, Left = 0, Right = 1 },
            AuraFrameOrientation = { Horizontal = 0, Vertical = 1 },
            AuraFrameVisibleSetting = { Always = 0, InCombat = 1, Hidden = 2 },
            BagsDirection = { Left = 0, Right = 1, Up = 0, Down = 1 },
            BagsOrientation = { Horizontal = 0, Vertical = 1 },
            CooldownViewerBarContent = { IconAndName = 0, IconOnly = 1, NameOnly = 2 },
            CooldownViewerIconDirection = { Left = 0, Right = 1 },
            CooldownViewerOrientation = { Horizontal = 0, Vertical = 1 },
            CooldownViewerVisibleSetting = { Always = 0, InCombat = 1, Hidden = 2 },
            DamageMeterNumbers = { Minimal = 0, Compact = 1, Complete = 2 },
            DamageMeterStyle = { Default = 0, Thin = 1, Bordered = 2, FullBackground = 3 },
            DamageMeterVisibility = { Always = 0, InCombat = 1, Hidden = 2 },
            EditModeActionBarSystemIndices = {
                MainBar = 1, Bar2 = 2, Bar3 = 3, RightBar1 = 4, RightBar2 = 5,
                ExtraBar1 = 6, ExtraBar2 = 7, ExtraBar3 = 8,
                StanceBar = 11, PetActionBar = 12, PossessActionBar = 13,
            },
            EditModeAuraFrameSystemIndices = { BuffFrame = 1, DebuffFrame = 2, ExternalDefensivesFrame = 3 },
            EditModeCooldownViewerSystemIndices = { Essential = 1, Utility = 2, BuffIcon = 3, BuffBar = 4 },
            EditModeEncounterEventsSystemIndices = { Timeline = 1, CriticalWarnings = 2, MediumWarnings = 3, NormalWarnings = 4 },
            EditModeStatusTrackingBarSystemIndices = { StatusTrackingBar1 = 1, StatusTrackingBar2 = 2 },
            EditModeUnitFrameSystemIndices = { Player = 1, Target = 2, Focus = 3, Party = 4, Raid = 5, Boss = 6, Arena = 7, Pet = 8 },
            EncounterEventsIconDirection = { Left = 0, Right = 1, Top = 0, Bottom = 1 },
            EncounterEventsOrientation = { Horizontal = 0, Vertical = 1 },
            EncounterEventsTooltipAnchor = { Hidden = 0, Default = 1, Cursor = 2 },
            EncounterEventsViewType = { Timeline = 0, Bars = 1 },
            EncounterEventsVisibility = { Always = 0, InEncounter = 1, DeprecatedHidden = 2 },
            MicroMenuOrder = { Default = 0, Reverse = 1 },
            MicroMenuOrientation = { Horizontal = 0, Vertical = 1 },
            RaidAuraOrganizationType = { Legacy = 0, BuffsTopDebuffsBottom = 1, BuffsRightDebuffsLeft = 2 },
            ViewArenaSize = { Two = 0, Three = 1 },
            ViewRaidSize = { Ten = 0, TwentyFive = 1, Forty = 2 },
            WidgetOpacityType = {
                OneHundred = 0, Ninety = 1, Eighty = 2, Seventy = 3, Sixty = 4,
                Fifty = 5, Forty = 6, Thirty = 7, Twenty = 8, Ten = 9, Zero = 10,
            },
        }

        local settingEnums = {
            EditModeAccountSetting = {
                "ShowGrid", "GridSpacing", "SettingsExpanded", "ShowTargetAndFocus",
                "ShowStanceBar", "ShowPetActionBar", "ShowPossessActionBar", "ShowCastBar",
                "ShowEncounterBar", "ShowExtraAbilities", "ShowBuffsAndDebuffs",
                "DeprecatedShowDebuffFrame", "ShowPartyFrames", "ShowRaidFrames",
                "ShowTalkingHeadFrame", "ShowVehicleLeaveButton", "ShowBossFrames",
                "ShowArenaFrames", "ShowLootFrame", "ShowHudTooltip", "ShowStatusTrackingBar2",
                "ShowDurabilityFrame", "EnableSnap", "EnableAdvancedOptions", "ShowPetFrame",
                "ShowTimerBars", "ShowVehicleSeatIndicator", "ShowArchaeologyBar",
                "ShowCooldownViewer", "ShowPersonalResourceDisplay", "ShowEncounterEvents",
                "ShowDamageMeter", "ShowExternalDefensives", "ShowTotemActionBar",
            },
            EditModeActionBarSetting = {
                "Orientation", "NumRows", "NumIcons", "IconSize", "IconPadding",
                "VisibleSetting", "HideBarArt", "DeprecatedSnapToSide",
                "HideBarScrolling", "AlwaysShowButtons",
            },
            EditModeArchaeologyBarSetting = { "Size" },
            EditModeAuraFrameSetting = {
                "Orientation", "IconWrap", "IconDirection", "IconLimitBuffFrame",
                "IconLimitDebuffFrame", "IconSize", "IconPadding", "DeprecatedShowFull",
                "VisibleSetting", "Opacity", "ShowDispelType",
            },
            EditModeBagsSetting = { "Orientation", "Direction", "Size", "BagSlotPadding" },
            EditModeCastBarSetting = { "BarSize", "LockToPlayerFrame", "ShowCastTime" },
            EditModeChatFrameSetting = { "WidthHundreds", "WidthTensAndOnes", "HeightHundreds", "HeightTensAndOnes" },
            EditModeCooldownViewerSetting = {
                "Orientation", "IconLimit", "IconDirection", "IconSize", "IconPadding",
                "Opacity", "VisibleSetting", "BarContent", "HideWhenInactive",
                "ShowTimer", "ShowTooltips", "BarWidthScale",
            },
            EditModeDamageMeterSetting = {
                "Visibility", "Style", "Numbers", "FrameWidth", "FrameHeight",
                "Padding", "Transparency", "ObsoleteReuse1", "ShowSpecIcon",
                "ShowClassColor", "BarHeight", "TextSize", "BackgroundTransparency",
            },
            EditModeDurabilityFrameSetting = { "Size" },
            EditModeEncounterEventsSetting = {
                "Orientation", "IconDirection", "ShowSpellName", "IconSize", "OverallSize",
                "BackgroundTransparency", "Transparency", "Visibility", "TooltipAnchor",
                "ShowTimer", "ViewType", "FlipHorizontally", "BarWidth", "Padding",
            },
            EditModeMicroMenuSetting = { "Orientation", "Order", "Size", "EyeSize" },
            EditModeMinimapSetting = { "HeaderUnderneath", "RotateMinimap", "Size" },
            EditModeObjectiveTrackerSetting = { "Height", "Opacity", "TextSize" },
            EditModePersonalResourceDisplaySetting = { "HideHealthAndPower", "OnlyShowInCombat" },
            EditModeStatusTrackingBarSetting = { "Height", "Width", "TextSize", "Size" },
            EditModeTimerBarsSetting = { "Size" },
            EditModeUnitFrameSetting = {
                "HidePortrait", "CastBarUnderneath", "BuffsOnTop", "UseLargerFrame",
                "UseRaidStylePartyFrames", "ShowPartyFrameBackground", "UseHorizontalGroups",
                "CastBarOnSide", "ShowCastTime", "ViewRaidSize", "FrameWidth",
                "FrameHeight", "DisplayBorder", "RaidGroupDisplayType", "SortPlayersBy",
                "RowSize", "FrameSize", "ViewArenaSize", "AuraOrganizationType",
                "IconSize", "Opacity", "BigDefensiveIconSize",
            },
            EditModeVehicleSeatIndicatorSetting = { "Size" },
        }

        for enumName, values in pairs(settingEnums) do
            expected[enumName] = expected[enumName] or {}
            for index, fieldName in ipairs(values) do
                expected[enumName][fieldName] = index - 1
            end
        end

        for enumName, fields in pairs(expected) do
            local actual = Enum[enumName]
            if type(actual) ~= "table" then
                error(enumName .. " is not registered")
            end
            for fieldName, expectedValue in pairs(fields) do
                if actual[fieldName] ~= expectedValue then
                    error(string.format("%s.%s expected %s got %s", enumName, fieldName, tostring(expectedValue), tostring(actual[fieldName])))
                end
            end
        end
        "#,
    )
    .expect("profile option enums should match Blizzard generated docs");
}
