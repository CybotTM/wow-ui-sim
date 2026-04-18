use super::*;
#[test]
fn test_create_frame_from_xml_inline_function_with_two_global_args_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        Settings = { INTERFACE_CATEGORY_ID = 7, OpenedCategory = nil }
        RAID_FRAMES_LABEL = "Raid Frames"
        SOUNDKIT = { IG_MAINMENU_OPTION = 5 }
        XmlInlinePlayedSound = nil
        function Settings.OpenToCategory(category_id, label)
            Settings.OpenedCategory = { category_id, label }
        end
        function PlaySound(sound_id)
            XmlInlinePlayedSound = sound_id
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineTwoGlobalArgsFrame" parent="UIParent">
        <Scripts><OnClick>Settings.OpenToCategory(Settings.INTERFACE_CATEGORY_ID, RAID_FRAMES_LABEL); PlaySound(SOUNDKIT.IG_MAINMENU_OPTION)</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec("XmlInlineTwoGlobalArgsFrame:GetScript('OnClick')(XmlInlineTwoGlobalArgsFrame)")
        .unwrap();

    let result: (i32, String, i32) = env
        .eval(
            r#"
            return Settings.OpenedCategory[1],
                   Settings.OpenedCategory[2],
                   XmlInlinePlayedSound
        "#,
        )
        .unwrap();
    assert_eq!(result.0, 7);
    assert_eq!(result.1, "Raid Frames");
    assert_eq!(result.2, 5);
}

#[test]
fn test_create_frame_from_xml_inline_toggle_global_visibility_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineToggleRoot" parent="UIParent">
            <Frame name="XmlInlineToggleTarget" parent="XmlInlineToggleRoot"/>
            <Button name="XmlInlineToggleButton" parent="XmlInlineToggleRoot">
                <Scripts><OnClick>if ( XmlInlineToggleTarget:IsShown() ) then XmlInlineToggleTarget:Hide(); else XmlInlineToggleTarget:Show(); end</OnClick></Scripts>
            </Button>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineToggleTarget:Show()").unwrap();
    env.exec("XmlInlineToggleButton:GetScript('OnClick')(XmlInlineToggleButton)")
        .unwrap();
    let hidden: bool = env
        .eval("return not XmlInlineToggleTarget:IsShown()")
        .unwrap();
    assert!(hidden);

    env.exec("XmlInlineToggleButton:GetScript('OnClick')(XmlInlineToggleButton)")
        .unwrap();
    let shown: bool = env.eval("return XmlInlineToggleTarget:IsShown()").unwrap();
    assert!(shown);
}

#[test]
fn test_create_frame_from_xml_inline_conditional_global_noarg_then_else_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineRepairMode = true
        XmlInlineRepairCursorVisible = nil
        function InRepairMode()
            return XmlInlineRepairMode
        end
        function ShowRepairCursor()
            XmlInlineRepairCursorVisible = true
        end
        function HideRepairCursor()
            XmlInlineRepairCursorVisible = false
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineRepairRoot" parent="UIParent">
            <Frame name="MerchantFrame" parent="XmlInlineRepairRoot"/>
            <Button name="XmlInlineRepairButton" parent="XmlInlineRepairRoot">
                <Scripts><OnClick>if ( InRepairMode() ) then MerchantFrame:UnregisterEvent("PLAYER_MONEY"); HideRepairCursor(); else MerchantFrame:RegisterEvent("PLAYER_MONEY"); ShowRepairCursor(); end</OnClick></Scripts>
            </Button>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(r#"MerchantFrame:RegisterEvent("PLAYER_MONEY")"#)
        .unwrap();
    env.exec(r#"XmlInlineRepairButton:GetScript("OnClick")(XmlInlineRepairButton)"#)
        .unwrap();
    let repair_mode_result: (bool, bool) = env
        .eval(
            r#"return MerchantFrame:IsEventRegistered("PLAYER_MONEY"), XmlInlineRepairCursorVisible"#,
        )
        .unwrap();
    assert!(!repair_mode_result.0);
    assert!(!repair_mode_result.1);

    env.exec(
        r#"
        XmlInlineRepairMode = false
        XmlInlineRepairButton:GetScript("OnClick")(XmlInlineRepairButton)
    "#,
    )
    .unwrap();
    let normal_mode_result: (bool, bool) = env
        .eval(
            r#"return MerchantFrame:IsEventRegistered("PLAYER_MONEY"), XmlInlineRepairCursorVisible"#,
        )
        .unwrap();
    assert!(normal_mode_result.0);
    assert!(normal_mode_result.1);
}

#[test]
fn test_create_frame_from_xml_inline_conditional_self_method_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        SOUNDKIT = {
            IG_MAINMENU_OPTION_CHECKBOX_ON = 11,
            IG_MAINMENU_OPTION_CHECKBOX_OFF = 12,
        }
        XmlInlinePlayedSound = nil
        XmlInlineAutoAccept = nil
        function PlaySound(sound_id)
            XmlInlinePlayedSound = sound_id
        end
        function LFGListUtil_SetAutoAccept(value)
            XmlInlineAutoAccept = value
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineCheckedRoot" parent="UIParent">
            <CheckButton name="XmlInlineCheckedButton" parent="XmlInlineCheckedRoot">
                <Scripts><OnClick>if ( self:GetChecked() ) then PlaySound(SOUNDKIT.IG_MAINMENU_OPTION_CHECKBOX_ON); else PlaySound(SOUNDKIT.IG_MAINMENU_OPTION_CHECKBOX_OFF); end LFGListUtil_SetAutoAccept(self:GetChecked())</OnClick></Scripts>
            </CheckButton>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineCheckedButton:SetChecked(true)
        XmlInlineCheckedButton:GetScript("OnClick")(XmlInlineCheckedButton)
    "#,
    )
    .unwrap();
    let checked_result: (i32, bool) = env
        .eval("return XmlInlinePlayedSound, XmlInlineAutoAccept")
        .unwrap();
    assert_eq!(checked_result.0, 11);
    assert!(checked_result.1);

    env.exec(
        r#"
        XmlInlineCheckedButton:SetChecked(false)
        XmlInlineCheckedButton:GetScript("OnClick")(XmlInlineCheckedButton)
    "#,
    )
    .unwrap();
    let unchecked_result: (i32, bool) = env
        .eval("return XmlInlinePlayedSound, XmlInlineAutoAccept")
        .unwrap();
    assert_eq!(unchecked_result.0, 12);
    assert!(!unchecked_result.1);
}

#[test]
fn test_create_frame_from_xml_inline_checked_assignment_then_callbacks_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        CHATCONFIG_SELECTED_FILTER = { settings = { lineHighlighting = false } }
        XmlInlineCheckedAssignmentUpdateCount = 0
        XmlInlineCheckedAssignmentSound = nil
        function CombatConfig_Colorize_Update()
            XmlInlineCheckedAssignmentUpdateCount = XmlInlineCheckedAssignmentUpdateCount + 1
        end
        function ChatConfigFrame_PlayCheckboxSound(checked)
            XmlInlineCheckedAssignmentSound = checked
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineCheckedAssignmentRoot" parent="UIParent">
            <CheckButton name="XmlInlineCheckedAssignmentButton" parent="XmlInlineCheckedAssignmentRoot">
                <Scripts><OnClick>
                    local checked = self:GetChecked()
                    if ( checked ) then
                        CHATCONFIG_SELECTED_FILTER.settings.lineHighlighting = true;
                    else
                        CHATCONFIG_SELECTED_FILTER.settings.lineHighlighting = false;
                    end
                    CombatConfig_Colorize_Update();
                    ChatConfigFrame_PlayCheckboxSound(checked);
                </OnClick></Scripts>
            </CheckButton>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineCheckedAssignmentButton:SetChecked(true)
        XmlInlineCheckedAssignmentButton:GetScript("OnClick")(XmlInlineCheckedAssignmentButton)
    "#,
    )
    .unwrap();
    let checked_result: (bool, i32, bool) = env
        .eval(
            r#"
            return CHATCONFIG_SELECTED_FILTER.settings.lineHighlighting,
                   XmlInlineCheckedAssignmentUpdateCount,
                   XmlInlineCheckedAssignmentSound
        "#,
        )
        .unwrap();
    assert!(checked_result.0);
    assert_eq!(checked_result.1, 1);
    assert!(checked_result.2);

    env.exec(
        r#"
        XmlInlineCheckedAssignmentButton:SetChecked(false)
        XmlInlineCheckedAssignmentButton:GetScript("OnClick")(XmlInlineCheckedAssignmentButton)
    "#,
    )
    .unwrap();
    let unchecked_result: (bool, i32, bool) = env
        .eval(
            r#"
            return CHATCONFIG_SELECTED_FILTER.settings.lineHighlighting,
                   XmlInlineCheckedAssignmentUpdateCount,
                   XmlInlineCheckedAssignmentSound
        "#,
        )
        .unwrap();
    assert!(!unchecked_result.0);
    assert_eq!(unchecked_result.1, 2);
    assert!(!unchecked_result.2);
}

#[test]
fn test_create_frame_from_xml_inline_checked_assignments3_then_callbacks_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        CHATCONFIG_SELECTED_FILTER = {
            settings = {
                unitColoring = false,
                sourceColoring = false,
                destColoring = false,
            },
        }
        XmlInlineCheckedTripleUpdateCount = 0
        XmlInlineCheckedTripleSound = nil
        function CombatConfig_Colorize_Update()
            XmlInlineCheckedTripleUpdateCount = XmlInlineCheckedTripleUpdateCount + 1
        end
        function ChatConfigFrame_PlayCheckboxSound(checked)
            XmlInlineCheckedTripleSound = checked
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineCheckedTripleRoot" parent="UIParent">
            <CheckButton name="XmlInlineCheckedTripleButton" parent="XmlInlineCheckedTripleRoot">
                <Scripts><OnClick>
                    local checked = self:GetChecked()
                    if ( checked ) then
                        CHATCONFIG_SELECTED_FILTER.settings.unitColoring = true;
                        CHATCONFIG_SELECTED_FILTER.settings.sourceColoring = true;
                        CHATCONFIG_SELECTED_FILTER.settings.destColoring = true;
                    else
                        CHATCONFIG_SELECTED_FILTER.settings.unitColoring = false;
                        CHATCONFIG_SELECTED_FILTER.settings.sourceColoring = false;
                        CHATCONFIG_SELECTED_FILTER.settings.destColoring = false;
                    end
                    CombatConfig_Colorize_Update();
                    ChatConfigFrame_PlayCheckboxSound(checked);
                </OnClick></Scripts>
            </CheckButton>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineCheckedTripleButton:SetChecked(true)
        XmlInlineCheckedTripleButton:GetScript("OnClick")(XmlInlineCheckedTripleButton)
    "#,
    )
    .unwrap();
    let checked_result: (bool, bool, bool, i32, bool) = env
        .eval(
            r#"
            return CHATCONFIG_SELECTED_FILTER.settings.unitColoring,
                   CHATCONFIG_SELECTED_FILTER.settings.sourceColoring,
                   CHATCONFIG_SELECTED_FILTER.settings.destColoring,
                   XmlInlineCheckedTripleUpdateCount,
                   XmlInlineCheckedTripleSound
        "#,
        )
        .unwrap();
    assert!(checked_result.0);
    assert!(checked_result.1);
    assert!(checked_result.2);
    assert_eq!(checked_result.3, 1);
    assert!(checked_result.4);

    env.exec(
        r#"
        XmlInlineCheckedTripleButton:SetChecked(false)
        XmlInlineCheckedTripleButton:GetScript("OnClick")(XmlInlineCheckedTripleButton)
    "#,
    )
    .unwrap();
    let unchecked_result: (bool, bool, bool, i32, bool) = env
        .eval(
            r#"
            return CHATCONFIG_SELECTED_FILTER.settings.unitColoring,
                   CHATCONFIG_SELECTED_FILTER.settings.sourceColoring,
                   CHATCONFIG_SELECTED_FILTER.settings.destColoring,
                   XmlInlineCheckedTripleUpdateCount,
                   XmlInlineCheckedTripleSound
        "#,
        )
        .unwrap();
    assert!(!unchecked_result.0);
    assert!(!unchecked_result.1);
    assert!(!unchecked_result.2);
    assert_eq!(unchecked_result.3, 2);
    assert!(!unchecked_result.4);
}

#[test]
fn test_create_frame_from_xml_inline_checked_assignment_then_two_callbacks_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        CHATCONFIG_SELECTED_FILTER = { hasQuickButton = false }
        XmlInlineCheckedDualUpdateQuick = 0
        XmlInlineCheckedDualUpdateSettings = 0
        XmlInlineCheckedDualSound = nil
        function Blizzard_CombatLog_Update_QuickButtons()
            XmlInlineCheckedDualUpdateQuick = XmlInlineCheckedDualUpdateQuick + 1
        end
        function CombatConfig_Settings_Update()
            XmlInlineCheckedDualUpdateSettings = XmlInlineCheckedDualUpdateSettings + 1
        end
        function ChatConfigFrame_PlayCheckboxSound(checked)
            XmlInlineCheckedDualSound = checked
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineCheckedDualRoot" parent="UIParent">
            <CheckButton name="XmlInlineCheckedDualButton" parent="XmlInlineCheckedDualRoot">
                <Scripts><OnClick>
                    local checked = self:GetChecked()
                    if ( checked ) then
                        CHATCONFIG_SELECTED_FILTER.hasQuickButton = true;
                    else
                        CHATCONFIG_SELECTED_FILTER.hasQuickButton = false;
                    end
                    Blizzard_CombatLog_Update_QuickButtons();
                    CombatConfig_Settings_Update();
                    ChatConfigFrame_PlayCheckboxSound(checked);
                </OnClick></Scripts>
            </CheckButton>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineCheckedDualButton:SetChecked(true)
        XmlInlineCheckedDualButton:GetScript("OnClick")(XmlInlineCheckedDualButton)
    "#,
    )
    .unwrap();
    let checked_result: (bool, i32, i32, bool) = env
        .eval(
            r#"
            return CHATCONFIG_SELECTED_FILTER.hasQuickButton,
                   XmlInlineCheckedDualUpdateQuick,
                   XmlInlineCheckedDualUpdateSettings,
                   XmlInlineCheckedDualSound
        "#,
        )
        .unwrap();
    assert!(checked_result.0);
    assert_eq!(checked_result.1, 1);
    assert_eq!(checked_result.2, 1);
    assert!(checked_result.3);

    env.exec(
        r#"
        XmlInlineCheckedDualButton:SetChecked(false)
        XmlInlineCheckedDualButton:GetScript("OnClick")(XmlInlineCheckedDualButton)
    "#,
    )
    .unwrap();
    let unchecked_result: (bool, i32, i32, bool) = env
        .eval(
            r#"
            return CHATCONFIG_SELECTED_FILTER.hasQuickButton,
                   XmlInlineCheckedDualUpdateQuick,
                   XmlInlineCheckedDualUpdateSettings,
                   XmlInlineCheckedDualSound
        "#,
        )
        .unwrap();
    assert!(!unchecked_result.0);
    assert_eq!(unchecked_result.1, 2);
    assert_eq!(unchecked_result.2, 2);
    assert!(!unchecked_result.3);
}

#[test]
fn test_create_frame_from_xml_inline_checked_number_assignment_then_callbacks_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        CHATCONFIG_SELECTED_FILTER = { settings = { lineColorPriority = 0 } }
        XmlInlineCheckedNumberUpdateCount = 0
        XmlInlineCheckedNumberSound = nil
        function CombatConfig_Colorize_Update()
            XmlInlineCheckedNumberUpdateCount = XmlInlineCheckedNumberUpdateCount + 1
        end
        function ChatConfigFrame_PlayCheckboxSound(checked)
            XmlInlineCheckedNumberSound = checked
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineCheckedNumberRoot" parent="UIParent">
            <CheckButton name="XmlInlineCheckedNumberButton" parent="XmlInlineCheckedNumberRoot">
                <Scripts><OnClick>
                    local checked = self:GetChecked();
                    CHATCONFIG_SELECTED_FILTER.settings.lineColorPriority = 2;
                    CombatConfig_Colorize_Update();
                    ChatConfigFrame_PlayCheckboxSound(checked);
                </OnClick></Scripts>
            </CheckButton>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineCheckedNumberButton:SetChecked(true)
        XmlInlineCheckedNumberButton:GetScript("OnClick")(XmlInlineCheckedNumberButton)
    "#,
    )
    .unwrap();
    let result: (i32, i32, bool) = env
        .eval(
            r#"
            return CHATCONFIG_SELECTED_FILTER.settings.lineColorPriority,
                   XmlInlineCheckedNumberUpdateCount,
                   XmlInlineCheckedNumberSound
        "#,
        )
        .unwrap();
    assert_eq!(result.0, 2);
    assert_eq!(result.1, 1);
    assert!(result.2);
}

#[test]
fn test_create_frame_from_xml_inline_parent_field_local_toggle_shown_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        FriendsUnavailableInfoMixin = {}
        function FriendsUnavailableInfoMixin:IsShown()
            return self.shown == true
        end
        function FriendsUnavailableInfoMixin:SetShown(value)
            self.shown = value
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineToggleRoot" parent="UIParent">
            <Button name="XmlInlineToggleButton" parent="XmlInlineToggleRoot">
                <Scripts><OnClick>
                    local infoFrame = self:GetParent().UnavailableInfoFrame;
                    infoFrame:SetShown(not infoFrame:IsShown());
                </OnClick></Scripts>
            </Button>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineToggleRoot.UnavailableInfoFrame = setmetatable({ shown = false }, { __index = FriendsUnavailableInfoMixin })
        XmlInlineToggleButton:GetScript("OnClick")(XmlInlineToggleButton)
    "#,
    )
    .unwrap();
    let first: bool = env
        .eval("return XmlInlineToggleRoot.UnavailableInfoFrame.shown")
        .unwrap();
    assert!(first);

    env.exec(
        r#"
        XmlInlineToggleButton:GetScript("OnClick")(XmlInlineToggleButton)
    "#,
    )
    .unwrap();
    let second: bool = env
        .eval("return XmlInlineToggleRoot.UnavailableInfoFrame.shown")
        .unwrap();
    assert!(!second);
}
