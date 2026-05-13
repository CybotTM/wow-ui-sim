use wow_ui_sim::lua_api::WowLuaEnv;

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
