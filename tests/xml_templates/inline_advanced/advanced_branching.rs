use super::*;
#[test]
fn test_create_frame_from_xml_inline_petbattle_tooltip_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        Enum = { BattlePetOwner = { Weather = "weather" } }
        PET_BATTLE_PAD_INDEX = "pad"
        XmlInlinePetBattleTooltipLog = {}
        function PetBattleAbilityTooltip_SetAura(owner, index, aura)
            table.insert(XmlInlinePetBattleTooltipLog, string.format("aura:%s:%s:%s", owner, index, aura))
        end
        function PetBattleAbilityTooltip_Show(anchor, owner, relative, x, y)
            table.insert(XmlInlinePetBattleTooltipLog, string.format("show:%s:%s:%s:%s", anchor, relative, x, y))
        end
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlinePetBattleTooltipButton" parent="UIParent">
            <Scripts><OnEnter>
                PetBattleAbilityTooltip_SetAura(Enum.BattlePetOwner.Weather, PET_BATTLE_PAD_INDEX, 1);
                PetBattleAbilityTooltip_Show("TOP", self, "BOTTOM", 0, 0);
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        XmlInlinePetBattleTooltipButton:GetScript("OnEnter")(XmlInlinePetBattleTooltipButton)
    "#,
    )
    .unwrap();
    let result: String = env
        .eval(r#"return table.concat(XmlInlinePetBattleTooltipLog, ",")"#)
        .unwrap();
    assert_eq!(result, "aura:weather:pad:1,show:TOP:BOTTOM:0:0");
}

#[test]
fn test_create_frame_from_xml_inline_hide_then_conditional_then_sound_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        ChatConfigFrame = "chat-config"
        Blizzard_CombatLog_CurrentSettings = "settings"
        XmlInlineChatConfigLog = {}
        function HideUIPanel(frame)
            table.insert(XmlInlineChatConfigLog, "hide:" .. tostring(frame))
        end
        function FCF_GetCurrentChatFrame()
            table.insert(XmlInlineChatConfigLog, "frame")
            return "frame-1"
        end
        function IsCombatLog(frame)
            table.insert(XmlInlineChatConfigLog, "iscombat:" .. tostring(frame))
            return true
        end
        function Blizzard_CombatLog_RefreshGlobalLinks()
            table.insert(XmlInlineChatConfigLog, "refresh")
        end
        C_CombatLog = {
            ApplyFilterSettings = function(settings)
                table.insert(XmlInlineChatConfigLog, "apply:" .. tostring(settings))
            end,
            RefilterEntries = function()
                table.insert(XmlInlineChatConfigLog, "refilter")
            end,
        }
        SOUNDKIT = { GS_TITLE_OPTION_OK = "ok" }
        function PlaySound(sound)
            table.insert(XmlInlineChatConfigLog, "sound:" .. tostring(sound))
        end
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineChatConfigButton" parent="UIParent">
            <Scripts><OnClick>
                HideUIPanel(ChatConfigFrame);
                if ( IsCombatLog(FCF_GetCurrentChatFrame()) ) then
                    Blizzard_CombatLog_RefreshGlobalLinks();
                    C_CombatLog.ApplyFilterSettings(Blizzard_CombatLog_CurrentSettings);
                    C_CombatLog.RefilterEntries();
                end
                PlaySound(SOUNDKIT.GS_TITLE_OPTION_OK);
            </OnClick></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        XmlInlineChatConfigButton:GetScript("OnClick")(XmlInlineChatConfigButton)
    "#,
    )
    .unwrap();
    let result: String = env
        .eval(r#"return table.concat(XmlInlineChatConfigLog, ",")"#)
        .unwrap();
    assert_eq!(
        result,
        "hide:chat-config,frame,iscombat:frame-1,refresh,apply:settings,refilter,sound:ok"
    );
}

#[test]
fn test_create_frame_from_xml_inline_shift_insert_else_parent_execute_and_clear_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineConsoleLog = {}
        function IsShiftKeyDown()
            return false
        end
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Frame name="XmlInlineConsoleRoot" parent="UIParent">
            <Frames>
                <EditBox name="XmlInlineConsoleEditBox">
                    <Scripts><OnEnterPressed>
                        if IsShiftKeyDown() then
                            self:Insert("\n");
                        else
                            local text = self:GetText();
                            if text and #text > 0 then
                                self:GetParent():ExecuteCommand(self:GetText());
                                self:SetText("");
                            end
                        end
                    </OnEnterPressed></Scripts>
                </EditBox>
            </Frames>
        </Frame>
    </Ui>"#,
        "Frame",
    );
    env.exec(
        r#"
        function XmlInlineConsoleRoot:ExecuteCommand(text)
            table.insert(XmlInlineConsoleLog, text)
        end
        XmlInlineConsoleEditBox:SetText("run this")
        XmlInlineConsoleEditBox:GetScript("OnEnterPressed")(XmlInlineConsoleEditBox)
    "#,
    )
    .unwrap();
    let result: (String, String) = env
        .eval(
            r#"
            return XmlInlineConsoleLog[1], XmlInlineConsoleEditBox:GetText()
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "run this");
    assert_eq!(result.1, "");
}

#[test]
fn test_create_frame_from_xml_inline_tooltip_set_text_function_result_and_three_numbers_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        CHARACTER_INFO = "Character"
        function MicroButtonTooltipText(label, binding)
            return label .. ":" .. binding
        end
        GameTooltip = {
            SetText = function(self, text, r, g, b)
                self.args = { text, r, g, b }
            end,
        }
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineTooltipFunctionResultButton" parent="UIParent">
            <Scripts><OnEnter>
                GameTooltip:SetText(MicroButtonTooltipText(CHARACTER_INFO, "TOGGLECHARACTER0"), 1.0, 1.0, 1.0);
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        XmlInlineTooltipFunctionResultButton:GetScript("OnEnter")(XmlInlineTooltipFunctionResultButton)
    "#,
    )
    .unwrap();
    let result: (String, f64, f64, f64) = env
        .eval(
            r#"
            return GameTooltip.args[1],
                   GameTooltip.args[2],
                   GameTooltip.args[3],
                   GameTooltip.args[4]
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "Character:TOGGLECHARACTER0");
    assert_eq!(result.1, 1.0);
    assert_eq!(result.2, 1.0);
    assert_eq!(result.3, 1.0);
}

#[test]
fn test_create_frame_from_xml_inline_tooltip_set_owner_then_function_result_text_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        CHARACTER_INFO = "Character"
        function MicroButtonTooltipText(label, binding)
            return label .. ":" .. binding
        end
        GameTooltip = {
            SetOwner = function(self, owner, anchor)
                self.owner = { owner, anchor }
            end,
            SetText = function(self, text, r, g, b)
                self.args = { text, r, g, b }
            end,
        }
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineTooltipOwnerFunctionResultButton" parent="UIParent">
            <Scripts><OnEnter>
                GameTooltip:SetOwner(self, "ANCHOR_RIGHT");
                GameTooltip:SetText(MicroButtonTooltipText(CHARACTER_INFO, "TOGGLECHARACTER0"), 1.0,1.0,1.0 );
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        XmlInlineTooltipOwnerFunctionResultButton:GetScript("OnEnter")(XmlInlineTooltipOwnerFunctionResultButton)
    "#,
    )
    .unwrap();
    let result: (String, String, f64, f64, f64) = env
        .eval(
            r#"
            return GameTooltip.owner[2],
                   GameTooltip.args[1],
                   GameTooltip.args[2],
                   GameTooltip.args[3],
                   GameTooltip.args[4]
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "ANCHOR_RIGHT");
    assert_eq!(result.1, "Character:TOGGLECHARACTER0");
    assert_eq!(result.2, 1.0);
    assert_eq!(result.3, 1.0);
    assert_eq!(result.4, 1.0);
}

#[test]
fn test_create_frame_from_xml_inline_tooltip_set_text_global_string_function_result_and_three_numbers_runs()
 {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        CHARACTER_INFO = "Character"
        function MicroButtonTooltipText(label, binding)
            return label .. ":" .. binding
        end
        GameTooltip = {
            SetText = function(self, text, r, g, b)
                self.args = { text, r, g, b }
            end,
        }
    "#,
    )
    .unwrap();
    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineTooltipGlobalStringFunctionResultButton" parent="UIParent">
            <Scripts><OnEnter>
                GameTooltip:SetText(MicroButtonTooltipText(CHARACTER_INFO, "TOGGLECHARACTER0"), 1.0, 1.0, 1.0);
            </OnEnter></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );
    env.exec(
        r#"
        XmlInlineTooltipGlobalStringFunctionResultButton:GetScript("OnEnter")(XmlInlineTooltipGlobalStringFunctionResultButton)
    "#,
    )
    .unwrap();
    let result: (String, f64, f64, f64) = env
        .eval(
            r#"
            return GameTooltip.args[1],
                   GameTooltip.args[2],
                   GameTooltip.args[3],
                   GameTooltip.args[4]
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "Character:TOGGLECHARACTER0");
    assert_eq!(result.1, 1.0);
    assert_eq!(result.2, 1.0);
    assert_eq!(result.3, 1.0);
}

#[test]
fn test_create_frame_from_xml_inherited_append_number_method_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    register_first_template(
        r#"<Ui>
        <EditBox name="XmlInlineBaseAppendTemplate" virtual="true">
            <Scripts><OnEnable method="BaseOnEnable"/></Scripts>
        </EditBox>
    </Ui>"#,
        "XmlInlineBaseAppendTemplate",
        "EditBox",
    );
    register_first_template(
        r#"<Ui>
        <EditBox name="XmlInlineDerivedAppendTemplate" inherits="XmlInlineBaseAppendTemplate" virtual="true">
            <Scripts><OnEnable inherit="append">
                self:SetMaxLetters(31);
            </OnEnable></Scripts>
        </EditBox>
    </Ui>"#,
        "XmlInlineDerivedAppendTemplate",
        "EditBox",
    );

    env.exec(
        r#"
        XmlInlineAppendEditBox = CreateFrame("EditBox", "XmlInlineAppendEditBox", UIParent, "XmlInlineDerivedAppendTemplate")
        function XmlInlineAppendEditBox:BaseOnEnable()
            self.baseOnEnableCalls = (self.baseOnEnableCalls or 0) + 1
        end
        XmlInlineAppendEditBox:GetScript("OnEnable")(XmlInlineAppendEditBox)
    "#,
    )
    .unwrap();
    let result: (i32, i32) = env
        .eval("return XmlInlineAppendEditBox.baseOnEnableCalls, XmlInlineAppendEditBox:GetMaxLetters()")
        .unwrap();
    assert_eq!(result.0, 1);
    assert_eq!(result.1, 31);
}

#[test]
fn test_create_frame_from_xml_inline_get_lfg_mode_branch_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        LE_LFG_CATEGORY_LFD = 1
        XmlInlineLfgBranch = {}
        function GetLFGMode(category)
            XmlInlineLfgBranch.modeCategory = category
            return XmlInlineLfgBranch.mode, nil
        end
        function LeaveLFG(category)
            XmlInlineLfgBranch.leaveCategory = category
        end
        function LFDQueueFrame_Join()
            XmlInlineLfgBranch.joined = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineLfgBranchButton" parent="UIParent">
            <Scripts><OnClick>
                local mode, subMode = GetLFGMode(LE_LFG_CATEGORY_LFD);
                if ( mode == "queued" or mode == "listed" or mode == "rolecheck" or mode == "suspended" ) then
                    LeaveLFG(LE_LFG_CATEGORY_LFD);
                else
                    LFDQueueFrame_Join();
                end
            </OnClick></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );

    env.exec(
        r#"
        XmlInlineLfgBranch.mode = "queued"
        XmlInlineLfgBranch.leaveCategory = nil
        XmlInlineLfgBranch.joined = false
        XmlInlineLfgBranchButton:GetScript("OnClick")(XmlInlineLfgBranchButton)
    "#,
    )
    .unwrap();
    let queued: (i32, i32, bool) = env
        .eval(
            r#"
            return XmlInlineLfgBranch.modeCategory,
                   XmlInlineLfgBranch.leaveCategory,
                   XmlInlineLfgBranch.joined
        "#,
        )
        .unwrap();
    assert_eq!(queued.0, 1);
    assert_eq!(queued.1, 1);
    assert!(!queued.2);

    env.exec(
        r#"
        XmlInlineLfgBranch.mode = "none"
        XmlInlineLfgBranch.leaveCategory = nil
        XmlInlineLfgBranch.joined = false
        XmlInlineLfgBranchButton:GetScript("OnClick")(XmlInlineLfgBranchButton)
    "#,
    )
    .unwrap();
    let joined: bool = env.eval("return XmlInlineLfgBranch.joined").unwrap();
    assert!(joined);
}

#[test]
fn test_create_frame_from_xml_inline_conditional_self_field_then_else_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        HelpBrowser = { ticket = nil, home = nil }
        function HelpBrowser:OpenTicket(index)
            self.ticket = index
        end
        function HelpBrowser:NavigateHome(page)
            self.home = page
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineCaseIndexButton" parent="UIParent">
            <Scripts><OnClick>if (self.caseIndex) then HelpBrowser:OpenTicket(self.caseIndex) else HelpBrowser:NavigateHome("GMTicketStatus") end</OnClick></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );

    env.exec(
        r#"
        XmlInlineCaseIndexButton.caseIndex = 42
        XmlInlineCaseIndexButton:GetScript("OnClick")(XmlInlineCaseIndexButton)
    "#,
    )
    .unwrap();
    let ticket: i32 = env.eval("return HelpBrowser.ticket").unwrap();
    assert_eq!(ticket, 42);

    env.exec(
        r#"
        XmlInlineCaseIndexButton.caseIndex = nil
        HelpBrowser.home = nil
        XmlInlineCaseIndexButton:GetScript("OnClick")(XmlInlineCaseIndexButton)
    "#,
    )
    .unwrap();
    let home: String = env.eval("return HelpBrowser.home").unwrap();
    assert_eq!(home, "GMTicketStatus");
}

#[test]
fn test_create_frame_from_xml_inline_newline_then_conditional_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        HelpFrame = { shown = nil }
        HelpBrowser = { ticket = nil, home = nil }
        function HelpFrame:ShowFrame(page)
            self.shown = page
        end
        function HelpBrowser:OpenTicket(index)
            self.ticket = index
        end
        function HelpBrowser:NavigateHome(page)
            self.home = page
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineNewlineConditionalButton" parent="UIParent">
            <Scripts><OnClick>HelpFrame:ShowFrame(HELPFRAME_SUBMIT_TICKET)
if (self.caseIndex) then
    HelpBrowser:OpenTicket(self.caseIndex)
else
    HelpBrowser:NavigateHome("GMTicketStatus")
end</OnClick></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );

    env.exec(r#"HELPFRAME_SUBMIT_TICKET = "submit""#).unwrap();
    env.exec(
        r#"
        XmlInlineNewlineConditionalButton.caseIndex = 7
        XmlInlineNewlineConditionalButton:GetScript("OnClick")(XmlInlineNewlineConditionalButton)
    "#,
    )
    .unwrap();
    let with_case: (String, i32) = env
        .eval("return HelpFrame.shown, HelpBrowser.ticket")
        .unwrap();
    assert_eq!(with_case.0, "submit");
    assert_eq!(with_case.1, 7);

    env.exec(
        r#"
        XmlInlineNewlineConditionalButton.caseIndex = nil
        HelpBrowser.home = nil
        XmlInlineNewlineConditionalButton:GetScript("OnClick")(XmlInlineNewlineConditionalButton)
    "#,
    )
    .unwrap();
    let without_case: String = env.eval("return HelpBrowser.home").unwrap();
    assert_eq!(without_case, "GMTicketStatus");
}
