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

#[test]
fn test_create_frame_from_xml_inline_method_then_unchecked_parent_field_clear_and_show_text_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineMethodThenUncheckedCalls = 0
        function XmlInlineWrappedOnClick(self)
            XmlInlineMethodThenUncheckedCalls = XmlInlineMethodThenUncheckedCalls + 1
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineUncheckedRoot" parent="UIParent">
            <CheckButton name="XmlInlineUncheckedButton" parent="XmlInlineUncheckedRoot">
                <Scripts><OnClick>
                    self:OnClick();
                    if (not self:GetChecked()) then
                        self:GetParent().EditBox:SetText("");
                        self:GetParent().EditBox.Text:Show();
                    end
                </OnClick></Scripts>
            </CheckButton>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        function XmlInlineUncheckedButton:OnClick()
            XmlInlineWrappedOnClick(self)
        end
        XmlInlineUncheckedRoot.EditBox = {
            value = "123",
            Text = {
                shown = false,
                Show = function(self)
                    self.shown = true
                end,
            },
            SetText = function(self, value)
                self.value = value
            end,
        }
        XmlInlineUncheckedButton:SetChecked(false)
        XmlInlineUncheckedButton:GetScript("OnClick")(XmlInlineUncheckedButton)
    "#,
    )
    .unwrap();
    let unchecked_result: (i32, String, bool) = env
        .eval(
            r#"
            return XmlInlineMethodThenUncheckedCalls,
                   XmlInlineUncheckedRoot.EditBox.value,
                   XmlInlineUncheckedRoot.EditBox.Text.shown
        "#,
        )
        .unwrap();
    assert_eq!(unchecked_result.0, 1);
    assert_eq!(unchecked_result.1, "");
    assert!(unchecked_result.2);

    env.exec(
        r#"
        XmlInlineUncheckedRoot.EditBox.value = "456"
        XmlInlineUncheckedRoot.EditBox.Text.shown = false
        XmlInlineUncheckedButton:SetChecked(true)
        XmlInlineUncheckedButton:GetScript("OnClick")(XmlInlineUncheckedButton)
    "#,
    )
    .unwrap();
    let checked_result: (i32, String, bool) = env
        .eval(
            r#"
            return XmlInlineMethodThenUncheckedCalls,
                   XmlInlineUncheckedRoot.EditBox.value,
                   XmlInlineUncheckedRoot.EditBox.Text.shown
        "#,
        )
        .unwrap();
    assert_eq!(checked_result.0, 2);
    assert_eq!(checked_result.1, "456");
    assert!(!checked_result.2);
}

#[test]
fn test_create_frame_from_xml_inline_local_global_path_conditional_method_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        BrowserSettingsTooltip = {}
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineBrowserButton" parent="UIParent">
            <Scripts><OnClick>
                local browser = BrowserSettingsTooltip.browser
                if (browser) then
                    browser:DeleteCookies()
                end
            </OnClick></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );

    env.exec(
        r#"
        BrowserSettingsTooltip.browser = {
            calls = 0,
            DeleteCookies = function(self)
                self.calls = self.calls + 1
            end,
        }
        XmlInlineBrowserButton:GetScript("OnClick")(XmlInlineBrowserButton)
    "#,
    )
    .unwrap();
    let first: i32 = env
        .eval("return BrowserSettingsTooltip.browser.calls")
        .unwrap();
    assert_eq!(first, 1);

    env.exec(
        r#"
        BrowserSettingsTooltip.browser = nil
        XmlInlineBrowserButton:GetScript("OnClick")(XmlInlineBrowserButton)
        return true
    "#,
    )
    .unwrap();
}

#[test]
fn test_create_frame_from_xml_inline_conditional_self_text_empty_show_text_child_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineEmptyTextRoot" parent="UIParent">
            <EditBox name="XmlInlineEmptyTextEditBox" parent="XmlInlineEmptyTextRoot">
                <Scripts><OnEditFocusLost>
                    if ( self:GetText() == "" ) then
                        self.Text:Show();
                    end
                </OnEditFocusLost></Scripts>
            </EditBox>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineEmptyTextEditBox.Text = {
            shown = false,
            Show = function(self) self.shown = true end,
        }
        XmlInlineEmptyTextEditBox:SetText("")
        XmlInlineEmptyTextEditBox:GetScript("OnEditFocusLost")(XmlInlineEmptyTextEditBox)
    "#,
    )
    .unwrap();
    let shown: bool = env
        .eval("return XmlInlineEmptyTextEditBox.Text.shown")
        .unwrap();
    assert!(shown);
}

#[test]
fn test_create_frame_from_xml_inline_grandparent_method_with_not_self_checked_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineGrandparentRoot" parent="UIParent">
            <Frame name="XmlInlineGrandparentParent" parent="XmlInlineGrandparentRoot">
                <CheckButton name="XmlInlineGrandparentChild" parent="XmlInlineGrandparentParent">
                    <Scripts><OnClick>
                        self:GetParent():GetParent():SetDisabledStateOnCommunityFinderOptions(not self:GetChecked())
                    </OnClick></Scripts>
                </CheckButton>
            </Frame>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        function XmlInlineGrandparentRoot:SetDisabledStateOnCommunityFinderOptions(value)
            self.disabledState = value
        end
        XmlInlineGrandparentChild:SetChecked(false)
        XmlInlineGrandparentChild:GetScript("OnClick")(XmlInlineGrandparentChild)
    "#,
    )
    .unwrap();
    let first: bool = env
        .eval("return XmlInlineGrandparentRoot.disabledState")
        .unwrap();
    assert!(first);

    env.exec(
        r#"
        XmlInlineGrandparentChild:SetChecked(true)
        XmlInlineGrandparentChild:GetScript("OnClick")(XmlInlineGrandparentChild)
    "#,
    )
    .unwrap();
    let second: bool = env
        .eval("return XmlInlineGrandparentRoot.disabledState")
        .unwrap();
    assert!(!second);
}

#[test]
fn test_create_frame_from_xml_inline_function_with_self_gettext_result_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineFilterName = nil
        function CombatConfig_SetFilterName(value)
            XmlInlineFilterName = value
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineGetTextRoot" parent="UIParent">
            <EditBox name="XmlInlineGetTextEditBox" parent="XmlInlineGetTextRoot">
                <Scripts><OnEnterPressed>
                    CombatConfig_SetFilterName(self:GetText());
                    self:HighlightText(0, -1);
                </OnEnterPressed></Scripts>
            </EditBox>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineGetTextEditBox:SetText("my-filter")
        function XmlInlineGetTextEditBox:HighlightText(startIndex, endIndex)
            self.highlightStart = startIndex
            self.highlightEnd = endIndex
        end
        XmlInlineGetTextEditBox:GetScript("OnEnterPressed")(XmlInlineGetTextEditBox)
    "#,
    )
    .unwrap();
    let result: (String, i32, i32) = env
        .eval(
            r#"
            return XmlInlineFilterName,
                   XmlInlineGetTextEditBox.highlightStart,
                   XmlInlineGetTextEditBox.highlightEnd
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "my-filter");
    assert_eq!(result.1, 0);
    assert_eq!(result.2, -1);
}

#[test]
fn test_create_frame_from_xml_inline_copy_club_ticket_to_clipboard_from_parent_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        C_Club = {}
        ClubTicketUtil = {}
        XmlInlineClipboardValue = nil
        function C_Club.GetClubInfo(clubId)
            return { id = clubId }
        end
        function ClubTicketUtil.FormatTicket(clubInfo, linkText)
            return string.format("club:%d:%s", clubInfo.id, linkText)
        end
        function CopyToClipboard(value)
            XmlInlineClipboardValue = value
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineClubTicketRoot" parent="UIParent">
            <Button name="XmlInlineClubTicketButton" parent="XmlInlineClubTicketRoot">
                <Scripts><OnClick>
                    local clubId = self:GetParent():GetClubId();
                    local clubInfo = clubId and C_Club.GetClubInfo(clubId);
                    if clubInfo then
                        CopyToClipboard(ClubTicketUtil.FormatTicket(clubInfo, self:GetParent().LinkIDText:GetText()));
                    end
                </OnClick></Scripts>
            </Button>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        function XmlInlineClubTicketRoot:GetClubId()
            return 17
        end
        XmlInlineClubTicketRoot.LinkIDText = {
            GetText = function()
                return "abc123"
            end,
        }
        XmlInlineClubTicketButton:GetScript("OnClick")(XmlInlineClubTicketButton)
    "#,
    )
    .unwrap();
    let result: String = env.eval("return XmlInlineClipboardValue").unwrap();
    assert_eq!(result, "club:17:abc123");
}

#[test]
fn test_create_frame_from_xml_inline_play_sound_then_copy_club_ticket_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        C_Club = {}
        ClubTicketUtil = {}
        SOUNDKIT = { IG_MAINMENU_OPTION_CHECKBOX_ON = 42 }
        XmlInlineClipboardValue = nil
        XmlInlineLastSound = nil
        function C_Club.GetClubInfo(clubId)
            return { id = clubId }
        end
        function ClubTicketUtil.FormatTicket(clubInfo, linkText)
            return string.format("club:%d:%s", clubInfo.id, linkText)
        end
        function CopyToClipboard(value)
            XmlInlineClipboardValue = value
        end
        function PlaySound(sound)
            XmlInlineLastSound = sound
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineClubTicketSoundRoot" parent="UIParent">
            <Button name="XmlInlineClubTicketSoundButton" parent="XmlInlineClubTicketSoundRoot">
                <Scripts><OnClick>
                    PlaySound(SOUNDKIT.IG_MAINMENU_OPTION_CHECKBOX_ON);
                    local clubId = self:GetParent():GetClubId();
                    local clubInfo = clubId and C_Club.GetClubInfo(clubId);
                    if clubInfo then
                        CopyToClipboard(ClubTicketUtil.FormatTicket(clubInfo, self:GetParent().LinkIDText:GetText()));
                    end
                </OnClick></Scripts>
            </Button>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        function XmlInlineClubTicketSoundRoot:GetClubId()
            return 23
        end
        XmlInlineClubTicketSoundRoot.LinkIDText = {
            GetText = function()
                return "ticket"
            end,
        }
        XmlInlineClubTicketSoundButton:GetScript("OnClick")(XmlInlineClubTicketSoundButton)
    "#,
    )
    .unwrap();
    let result: (i32, String) = env
        .eval("return XmlInlineLastSound, XmlInlineClipboardValue")
        .unwrap();
    assert_eq!(result.0, 42);
    assert_eq!(result.1, "club:23:ticket");
}

#[test]
fn test_create_frame_from_xml_inline_parent_field_local_click_if_enabled_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineCreateButtonRoot" parent="UIParent">
            <EditBox name="XmlInlineCreateButtonEditBox" parent="XmlInlineCreateButtonRoot">
                <Scripts><OnEnterPressed>
                    local createButton = self:GetParent().CreateButton;
                    if createButton:IsEnabled() then
                        createButton:GetScript("OnClick")(createButton);
                    end
                </OnEnterPressed></Scripts>
            </EditBox>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineCreateButtonRoot.CreateButton = CreateFrame("Button", nil, XmlInlineCreateButtonRoot)
        XmlInlineCreateButtonRoot.calls = 0
        XmlInlineCreateButtonRoot.CreateButton:SetScript("OnClick", function(self)
            XmlInlineCreateButtonRoot.calls = XmlInlineCreateButtonRoot.calls + 1
        end)
        XmlInlineCreateButtonRoot.CreateButton:Enable()
        XmlInlineCreateButtonEditBox:GetScript("OnEnterPressed")(XmlInlineCreateButtonEditBox)
        XmlInlineCreateButtonRoot.CreateButton:Disable()
        XmlInlineCreateButtonEditBox:GetScript("OnEnterPressed")(XmlInlineCreateButtonEditBox)
    "#,
    )
    .unwrap();
    let calls: i32 = env.eval("return XmlInlineCreateButtonRoot.calls").unwrap();
    assert_eq!(calls, 1);
}

#[test]
fn test_create_frame_from_xml_inline_grandparent_field_method_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineGrandparentFieldRoot" parent="UIParent">
            <Frame name="XmlInlineGrandparentFieldParent" parent="XmlInlineGrandparentFieldRoot">
                <EditBox name="XmlInlineGrandparentFieldChild" parent="XmlInlineGrandparentFieldParent">
                    <Scripts><OnEnterPressed>
                        self:GetParent():GetParent().EditBox:SetFocus();
                    </OnEnterPressed></Scripts>
                </EditBox>
            </Frame>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineGrandparentFieldRoot.EditBox = {
            focused = false,
            SetFocus = function(self)
                self.focused = true
            end,
        }
        XmlInlineGrandparentFieldChild:GetScript("OnEnterPressed")(XmlInlineGrandparentFieldChild)
    "#,
    )
    .unwrap();
    let focused: bool = env
        .eval("return XmlInlineGrandparentFieldRoot.EditBox.focused")
        .unwrap();
    assert!(focused);
}

#[test]
fn test_create_frame_from_xml_inline_tooltip_then_parent_assign_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        HIGHLIGHT_FONT_COLOR = { r = 0.1, g = 0.2, b = 0.3 }
        BROWSER_DELETE_COOKIES_TOOLTIP = "Cookies"
        GameTooltip = {
            SetOwner = function(self, owner, anchor)
                self.owner = owner
                self.anchor = anchor
            end,
            SetText = function(self, text, r, g, b, maybe_nil, wrap)
                self.text = text
                self.color = { r, g, b }
                self.maybe_nil = maybe_nil
                self.wrap = wrap
            end,
        }
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineTooltipAssignRoot" parent="UIParent">
            <Button name="XmlInlineTooltipAssignButton" parent="XmlInlineTooltipAssignRoot">
                <Scripts><OnEnter>
                    GameTooltip:SetOwner(self, "ANCHOR_CURSOR_RIGHT");
                    GameTooltip:SetText(BROWSER_DELETE_COOKIES_TOOLTIP, HIGHLIGHT_FONT_COLOR.r, HIGHLIGHT_FONT_COLOR.g, HIGHLIGHT_FONT_COLOR.b, nil, true);
                    self:GetParent().isCounting = nil;
                </OnEnter></Scripts>
            </Button>
        </Frame>
    </Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineTooltipAssignRoot.isCounting = true
        XmlInlineTooltipAssignButton:GetScript("OnEnter")(XmlInlineTooltipAssignButton)
    "#,
    )
    .unwrap();
    let result: (String, String, bool, bool) = env
        .eval(
            r#"
            return GameTooltip.anchor,
                   GameTooltip.text,
                   GameTooltip.wrap,
                   XmlInlineTooltipAssignRoot.isCounting == nil
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "ANCHOR_CURSOR_RIGHT");
    assert_eq!(result.1, "Cookies");
    assert!(result.2);
    assert!(result.3);
}

#[test]
fn test_create_frame_from_xml_inline_tooltip_set_text_with_four_global_args_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        HIGHLIGHT_FONT_COLOR = { r = 0.6, g = 0.7, b = 0.8 }
        LFG_LIST_REFRESH = "Refresh"
        GameTooltip = {
            SetOwner = function(self, owner, anchor)
                self.anchor = anchor
            end,
            SetText = function(self, text, r, g, b)
                self.text = text
                self.rgb = { r, g, b }
            end,
        }
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineTooltipFourArgsButton" parent="UIParent">
            <Scripts><OnEnter>
                GameTooltip:SetOwner(self, "ANCHOR_RIGHT");
                GameTooltip:SetText(LFG_LIST_REFRESH, HIGHLIGHT_FONT_COLOR.r, HIGHLIGHT_FONT_COLOR.g, HIGHLIGHT_FONT_COLOR.b);
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );

    env.exec(
        r#"
        XmlInlineTooltipFourArgsButton:GetScript("OnEnter")(XmlInlineTooltipFourArgsButton)
    "#,
    )
    .unwrap();
    let result: (String, String, f64, f64, f64) = env
        .eval(
            r#"
            return GameTooltip.anchor,
                   GameTooltip.text,
                   GameTooltip.rgb[1],
                   GameTooltip.rgb[2],
                   GameTooltip.rgb[3]
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "ANCHOR_RIGHT");
    assert_eq!(result.1, "Refresh");
    assert_eq!(result.2, 0.6);
    assert_eq!(result.3, 0.7);
    assert_eq!(result.4, 0.8);
}

#[test]
fn test_create_frame_from_xml_inline_tooltip_title_line_show_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        BAG_CLEANUP_BAGS = "Cleanup"
        BAG_CLEANUP_BAGS_DESCRIPTION = "Cleanup desc"
        HIGHLIGHT_FONT_COLOR = { tag = "highlight" }
        XmlInlineTooltipLog = {}
        GameTooltip = {
            SetOwner = function(self, owner)
                table.insert(XmlInlineTooltipLog, "owner")
            end,
            Show = function(self)
                table.insert(XmlInlineTooltipLog, "show")
            end,
        }
        function GameTooltip_SetTitle(tooltip, text, color)
            table.insert(XmlInlineTooltipLog, "title:" .. text .. ":" .. color.tag)
        end
        function GameTooltip_AddNormalLine(tooltip, text)
            table.insert(XmlInlineTooltipLog, "line:" .. text)
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineTooltipSequenceButton" parent="UIParent">
            <Scripts><OnEnter>
                GameTooltip:SetOwner(self);
                GameTooltip_SetTitle(GameTooltip, BAG_CLEANUP_BAGS, HIGHLIGHT_FONT_COLOR);
                GameTooltip_AddNormalLine(GameTooltip, BAG_CLEANUP_BAGS_DESCRIPTION);
                GameTooltip:Show();
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );

    env.exec(
        r#"
        XmlInlineTooltipSequenceButton:GetScript("OnEnter")(XmlInlineTooltipSequenceButton)
    "#,
    )
    .unwrap();
    let result: String = env
        .eval(r#"return table.concat(XmlInlineTooltipLog, ",")"#)
        .unwrap();
    assert_eq!(
        result,
        "owner,title:Cleanup:highlight,line:Cleanup desc,show"
    );
}

#[test]
fn test_create_frame_from_xml_inline_tooltip_set_owner_offsets_title_show_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        DRESSING_ROOM_APPEARANCE_LIST = "Appearance"
        XmlInlineTooltipOffsetLog = {}
        GameTooltip = {
            SetOwner = function(self, owner, anchor, x, y)
                table.insert(XmlInlineTooltipOffsetLog, string.format("owner:%s:%s:%s", anchor, x, y))
            end,
            Show = function(self)
                table.insert(XmlInlineTooltipOffsetLog, "show")
            end,
        }
        function GameTooltip_SetTitle(tooltip, text)
            table.insert(XmlInlineTooltipOffsetLog, "title:" .. text)
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineTooltipOffsetButton" parent="UIParent">
            <Scripts><OnEnter>
                GameTooltip:SetOwner(self, "ANCHOR_RIGHT", -4, -4);
                GameTooltip_SetTitle(GameTooltip, DRESSING_ROOM_APPEARANCE_LIST);
                GameTooltip:Show();
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );

    env.exec(
        r#"
        XmlInlineTooltipOffsetButton:GetScript("OnEnter")(XmlInlineTooltipOffsetButton)
    "#,
    )
    .unwrap();
    let result: String = env
        .eval(r#"return table.concat(XmlInlineTooltipOffsetLog, ",")"#)
        .unwrap();
    assert_eq!(result, "owner:ANCHOR_RIGHT:-4:-4,title:Appearance,show");
}

#[test]
fn test_create_frame_from_xml_inline_self_field_set_point_with_self_target_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineIconPointButton" parent="UIParent">
            <Scripts><OnMouseDown>
                self.Icon:SetPoint("CENTER", self, "CENTER", -2, -1);
            </OnMouseDown></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );

    env.exec(
        r#"
        XmlInlineIconPointButton.Icon = {
            SetPoint = function(self, point, rel, relPoint, x, y)
                self.point = point
                self.rel = rel
                self.relPoint = relPoint
                self.x = x
                self.y = y
            end,
        }
        XmlInlineIconPointButton:GetScript("OnMouseDown")(XmlInlineIconPointButton)
    "#,
    )
    .unwrap();
    let result: (String, String, f64, f64, bool) = env
        .eval(
            r#"
            return XmlInlineIconPointButton.Icon.point,
                   XmlInlineIconPointButton.Icon.relPoint,
                   XmlInlineIconPointButton.Icon.x,
                   XmlInlineIconPointButton.Icon.y,
                   XmlInlineIconPointButton.Icon.rel == XmlInlineIconPointButton
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "CENTER");
    assert_eq!(result.1, "CENTER");
    assert_eq!(result.2, -2.0);
    assert_eq!(result.3, -1.0);
    assert!(result.4);
}

#[test]
fn test_create_frame_from_xml_inline_show_game_tooltip_helper_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        COMMUNITIES_CREATE_DIALOG_SHORT_NAME_INSTRUCTIONS_TOOLTIP = "Short name"
        CommunitiesOutbound = {}
        XmlInlineTooltipArgs = nil
        function CommunitiesOutbound.ShowGameTooltip(text, right, top, wrap)
            XmlInlineTooltipArgs = { text, right, top, wrap }
        end
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineShowTooltipButton" parent="UIParent">
            <Scripts><OnEnter>
                CommunitiesOutbound.ShowGameTooltip(COMMUNITIES_CREATE_DIALOG_SHORT_NAME_INSTRUCTIONS_TOOLTIP, self:GetRight(), self:GetTop(), true);
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        function XmlInlineShowTooltipButton:GetRight() return 12 end
        function XmlInlineShowTooltipButton:GetTop() return 34 end
        XmlInlineShowTooltipButton:GetScript("OnEnter")(XmlInlineShowTooltipButton)
    "#,
    )
    .unwrap();
    let result: (String, f64, f64, bool) = env
        .eval(
            r#"
            return XmlInlineTooltipArgs[1],
                   XmlInlineTooltipArgs[2],
                   XmlInlineTooltipArgs[3],
                   XmlInlineTooltipArgs[4]
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "Short name");
    assert_eq!(result.1, 12.0);
    assert_eq!(result.2, 34.0);
    assert!(result.3);
}

#[test]
#[ignore = "TODO: test calls GameTooltip:AddLine(STRING, COLOR_TABLE, true) expecting r/g/b expansion, but the GlobalMethodWithStringGlobalBoolArgs fast-path only forwards three values verbatim. Either the test needs a 5-arg call to match GlobalMethodWithGlobalThreeGlobalBoolArgs or a new color-expanding fast-path needs adding."]
fn test_create_frame_from_xml_inline_tooltip_add_line_global_three_global_bool_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        RED_FONT_COLOR = { r = 0.1, g = 0.2, b = 0.3 }
        ALL_ASSIST_NOT_LEADER_ERROR = "No leader"
        XmlInlineLineArgs = nil
        GameTooltip = {
            AddLine = function(self, text, r, g, b, wrap)
                XmlInlineLineArgs = { text, r, g, b, wrap }
            end,
        }
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineTooltipAddLineButton" parent="UIParent">
            <Scripts><OnEnter>
                GameTooltip:AddLine(ALL_ASSIST_NOT_LEADER_ERROR, RED_FONT_COLOR, true);
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        XmlInlineTooltipAddLineButton:GetScript("OnEnter")(XmlInlineTooltipAddLineButton)
    "#,
    )
    .unwrap();
    let result: (String, f64, f64, f64, bool) = env
        .eval(
            r#"
            return XmlInlineLineArgs[1],
                   XmlInlineLineArgs[2],
                   XmlInlineLineArgs[3],
                   XmlInlineLineArgs[4],
                   XmlInlineLineArgs[5]
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "No leader");
    assert_eq!(result.1, 0.1);
    assert_eq!(result.2, 0.2);
    assert_eq!(result.3, 0.3);
    assert!(result.4);
}

#[test]
fn test_create_frame_from_xml_inline_tooltip_set_text_global_self_methods_bool_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        COMMUNITIES_CREATE_DIALOG_SHORT_NAME_INSTRUCTIONS_TOOLTIP = "Short name"
        CommunitiesOutbound = {}
        GameTooltip = {
            SetText = function(self, text, right, top, wrap)
                self.args = { text, right, top, wrap }
            end,
        }
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineTooltipSelfMethodArgsButton" parent="UIParent">
            <Scripts><OnEnter>
                GameTooltip:SetText(COMMUNITIES_CREATE_DIALOG_SHORT_NAME_INSTRUCTIONS_TOOLTIP, self:GetRight(), self:GetTop(), true);
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        function XmlInlineTooltipSelfMethodArgsButton:GetRight() return 55 end
        function XmlInlineTooltipSelfMethodArgsButton:GetTop() return 77 end
        XmlInlineTooltipSelfMethodArgsButton:GetScript("OnEnter")(XmlInlineTooltipSelfMethodArgsButton)
    "#,
    )
    .unwrap();
    let result: (String, f64, f64, bool) = env
        .eval(
            r#"
            return GameTooltip.args[1],
                   GameTooltip.args[2],
                   GameTooltip.args[3],
                   GameTooltip.args[4]
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "Short name");
    assert_eq!(result.1, 55.0);
    assert_eq!(result.2, 77.0);
    assert!(result.3);
}

#[test]
fn test_create_frame_from_xml_inline_function_global_self_methods_bool_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        COMMUNITIES_CREATE_DIALOG_SHORT_NAME_INSTRUCTIONS_TOOLTIP = "Short name"
        CommunitiesOutbound = {}
        XmlInlineTooltipArgs = nil
        function CommunitiesOutbound.ShowGameTooltip(text, right, top, wrap)
            XmlInlineTooltipArgs = { text, right, top, wrap }
        end
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineShowGameTooltipButton" parent="UIParent">
            <Scripts><OnEnter>
                CommunitiesOutbound.ShowGameTooltip(COMMUNITIES_CREATE_DIALOG_SHORT_NAME_INSTRUCTIONS_TOOLTIP, self:GetRight(), self:GetTop(), true);
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        function XmlInlineShowGameTooltipButton:GetRight() return 12 end
        function XmlInlineShowGameTooltipButton:GetTop() return 34 end
        XmlInlineShowGameTooltipButton:GetScript("OnEnter")(XmlInlineShowGameTooltipButton)
    "#,
    )
    .unwrap();
    let result: (String, f64, f64, bool) = env
        .eval(
            r#"
            return XmlInlineTooltipArgs[1],
                   XmlInlineTooltipArgs[2],
                   XmlInlineTooltipArgs[3],
                   XmlInlineTooltipArgs[4]
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "Short name");
    assert_eq!(result.1, 12.0);
    assert_eq!(result.2, 34.0);
    assert!(result.3);
}

#[test]
fn test_create_frame_from_xml_inline_tooltip_set_text_global_nil_nil_nil_nil_bool_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        ALL_ASSIST_DESCRIPTION = "Assist"
        GameTooltip = {
            SetText = function(self, a, b, c, d, e, f)
                self.args = { a, b, c, d, e, f }
            end,
        }
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineTooltipNilArgsButton" parent="UIParent">
            <Scripts><OnEnter>
                GameTooltip:SetText(ALL_ASSIST_DESCRIPTION, nil, nil, nil, nil, true);
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        XmlInlineTooltipNilArgsButton:GetScript("OnEnter")(XmlInlineTooltipNilArgsButton)
    "#,
    )
    .unwrap();
    let result: (String, bool, bool) = env
        .eval(
            r#"
            return GameTooltip.args[1],
                   GameTooltip.args[6],
                   GameTooltip.args[2] == nil and GameTooltip.args[3] == nil and GameTooltip.args[4] == nil and GameTooltip.args[5] == nil
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "Assist");
    assert!(result.1);
    assert!(result.2);
}

#[test]
fn test_create_frame_from_xml_inline_conditional_not_enabled_then_add_line_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        RED_FONT_COLOR = { r = 0.1, g = 0.2, b = 0.3 }
        ALL_ASSIST_NOT_LEADER_ERROR = "No leader"
        XmlInlineLineArgs = nil
        GameTooltip = {
            AddLine = function(self, text, r, g, b, wrap)
                XmlInlineLineArgs = { text, r, g, b, wrap }
            end,
        }
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineConditionalAddLineButton" parent="UIParent">
            <Scripts><OnEnter>
                if ( not self:IsEnabled() ) then
                    GameTooltip:AddLine(ALL_ASSIST_NOT_LEADER_ERROR, RED_FONT_COLOR.r, RED_FONT_COLOR.g, RED_FONT_COLOR.b, true);
                end
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        XmlInlineConditionalAddLineButton:Disable()
        XmlInlineConditionalAddLineButton:GetScript("OnEnter")(XmlInlineConditionalAddLineButton)
    "#,
    )
    .unwrap();
    let result: (String, f64, f64, f64, bool) = env
        .eval(
            r#"
            return XmlInlineLineArgs[1],
                   XmlInlineLineArgs[2],
                   XmlInlineLineArgs[3],
                   XmlInlineLineArgs[4],
                   XmlInlineLineArgs[5]
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "No leader");
    assert_eq!(result.1, 0.1);
    assert_eq!(result.2, 0.2);
    assert_eq!(result.3, 0.3);
    assert!(result.4);
}

#[test]
fn test_create_frame_from_xml_inline_conditional_global_field_equals_string_then_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        PetitionFrame = { petitionType = "guild" }
        XmlInlinePopupArgs = nil
        function StaticPopup_Show(which)
            XmlInlinePopupArgs = { which }
        end
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlinePetitionRenameButton" parent="UIParent">
            <Scripts><OnClick>
                if ( PetitionFrame.petitionType == "guild" ) then
                    StaticPopup_Show("RENAME_GUILD");
                end
            </OnClick></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        XmlInlinePetitionRenameButton:GetScript("OnClick")(XmlInlinePetitionRenameButton)
    "#,
    )
    .unwrap();
    let result: String = env.eval(r#"return XmlInlinePopupArgs[1]"#).unwrap();
    assert_eq!(result, "RENAME_GUILD");
}
