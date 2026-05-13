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
        point,
        offset_x,
        setting_value,
    ): (i32, i32, i32, i32, i32, i32, String, i32, String, f64, i32) = env
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
    assert_eq!(point, "BOTTOM");
    assert_eq!(offset_x, 12.5);
    assert_eq!(setting_value, 1);
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
