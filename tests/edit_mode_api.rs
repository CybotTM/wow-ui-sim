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

    let (active, layout_count, account_setting, name, system, point, offset_x, setting_value): (
        i32,
        i32,
        i32,
        String,
        i32,
        String,
        f64,
        i32,
    ) = env
        .eval(
            r#"
            local info = C_EditMode.GetLayouts()
            local settings = C_EditMode.GetAccountSettings()
            local system = info.layouts[1].systems[1]
            return info.activeLayout,
                #info.layouts,
                settings[2].value,
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
    assert_eq!(account_setting, 100);
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
