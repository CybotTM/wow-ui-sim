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

#[test]
fn test_create_frame_from_xml_inline_global_method_with_late_bound_global_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        HelpFrame = { shown = nil }
        function HelpFrame:ShowFrame(page)
            self.shown = page
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui>
        <Button name="XmlInlineLateBoundGlobalMethodButton" parent="UIParent">
            <Scripts><OnClick>HelpFrame:ShowFrame(HELPFRAME_SUBMIT_TICKET)</OnClick></Scripts>
        </Button>
    </Ui>"#,
        "Button",
    );

    env.exec(r#"HELPFRAME_SUBMIT_TICKET = "submit""#).unwrap();
    env.exec(
        r#"
        XmlInlineLateBoundGlobalMethodButton:GetScript("OnClick")(XmlInlineLateBoundGlobalMethodButton)
    "#,
    )
    .unwrap();

    let shown: String = env.eval("return HelpFrame.shown").unwrap();
    assert_eq!(shown, "submit");
}

#[test]
fn test_create_frame_from_xml_inline_global_assignment_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.XmlInlineGlobalAssignTarget = {}
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineGlobalAssignFrame" parent="UIParent">
        <Scripts><OnClick>XmlInlineGlobalAssignTarget.flag = true</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec("XmlInlineGlobalAssignFrame:GetScript('OnClick')(XmlInlineGlobalAssignFrame)")
        .unwrap();

    let flag: bool = env
        .eval("return XmlInlineGlobalAssignTarget.flag == true")
        .unwrap();
    assert!(flag);
}

#[test]
fn test_create_frame_from_xml_inline_global_assignment_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.GuildInviteFrame = {}
        function AcceptGuild()
            GuildInviteFrame.acceptedByFn = true
        end
        function _G.GuildInviteFrame:Hide()
            self.hidden = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineGlobalAssignSequenceFrame" parent="UIParent">
        <Scripts><OnClick>AcceptGuild(); GuildInviteFrame.accepted = true; GuildInviteFrame:Hide()</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec(
        "XmlInlineGlobalAssignSequenceFrame:GetScript('OnClick')(XmlInlineGlobalAssignSequenceFrame)",
    )
    .unwrap();

    let result: (bool, bool, bool) = env
        .eval(
            r#"
            return GuildInviteFrame.acceptedByFn == true,
                   GuildInviteFrame.accepted == true,
                   GuildInviteFrame.hidden == true
        "#,
        )
        .unwrap();
    assert!(result.0);
    assert!(result.1);
    assert!(result.2);
}

#[test]
fn test_create_frame_from_xml_inline_function_with_global_and_self_id_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.XmlInlineGlobalSelfIdTarget = {}
        function XmlInlineSelectTab(frame, tab_id)
            frame.selectedTab = tab_id
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineGlobalSelfIdFrame" parent="UIParent">
        <Scripts><OnClick>XmlInlineSelectTab(XmlInlineGlobalSelfIdTarget, self:GetID())</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    let frame_id: f64 = env
        .eval("return XmlInlineGlobalSelfIdFrame:GetID()")
        .unwrap();
    env.exec("XmlInlineGlobalSelfIdFrame:GetScript('OnClick')(XmlInlineGlobalSelfIdFrame)")
        .unwrap();

    let selected: f64 = env
        .eval("return XmlInlineGlobalSelfIdTarget.selectedTab")
        .unwrap();
    assert_eq!(
        selected, frame_id,
        "inline global+self-id function arg should fire"
    );
}

#[test]
fn test_create_frame_from_xml_inline_lfg_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.SOUNDKIT = { IG_CHARACTER_INFO_TAB = 7 }
        _G.LFGDungeonReadyPopup = {}
        _G.LfgSequence = {}
        function PlaySound(sound_id)
            LfgSequence.soundId = sound_id
        end
        function LFGDebug(message)
            LfgSequence.message = message
        end
        function StaticPopupSpecial_Hide(frame)
            LfgSequence.hidden = frame
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineLfgSequenceFrame" parent="UIParent">
        <Scripts><OnClick>PlaySound(SOUNDKIT.IG_CHARACTER_INFO_TAB); LFGDebug("ready dialog close"); StaticPopupSpecial_Hide(LFGDungeonReadyPopup)</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec("XmlInlineLfgSequenceFrame:GetScript('OnClick')(XmlInlineLfgSequenceFrame)")
        .unwrap();

    let result: (f64, String, bool) = env
        .eval(
            r#"
            return LfgSequence.soundId,
                   LfgSequence.message,
                   LfgSequence.hidden == LFGDungeonReadyPopup
        "#,
        )
        .unwrap();
    assert_eq!(result.0, 7.0);
    assert_eq!(result.1, "ready dialog close");
    assert!(result.2);
}

#[test]
fn test_create_frame_from_xml_inline_commented_invite_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.SOUNDKIT = { IG_MAINMENU_OPTION_CHECKBOX_ON = 9 }
        _G.InviteSequence = {}
        function PlaySound(sound_id)
            InviteSequence.soundId = sound_id
        end
        function BNSendVerifiedBattleTagInvite()
            InviteSequence.sent = true
        end
        function StaticPopupSpecial_Hide(frame)
            InviteSequence.hidden = frame
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineInviteSequenceFrame" parent="UIParent">
        <Scripts><OnClick>PlaySound(SOUNDKIT.IG_MAINMENU_OPTION_CHECKBOX_ON); BNSendVerifiedBattleTagInvite(); -- unit should have been set with BNCheckBattleTagInviteToUnit or BNCheckBattleTagInviteToGuildMember
        StaticPopupSpecial_Hide(self:GetParent())</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec("XmlInlineInviteSequenceFrame:GetScript('OnClick')(XmlInlineInviteSequenceFrame)")
        .unwrap();

    let result: (f64, bool, bool) = env
        .eval(
            r#"
            return InviteSequence.soundId,
                   InviteSequence.sent == true,
                   InviteSequence.hidden == XmlInlineInviteSequenceFrame:GetParent()
        "#,
        )
        .unwrap();
    assert_eq!(result.0, 9.0);
    assert!(result.1);
    assert!(result.2);
}

#[test]
fn test_create_frame_from_xml_inline_report_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.ReportSequence = {}
        C_ReportSystem = {}
        function C_ReportSystem.SendReportPlayer(token, comment)
            ReportSequence.token = token
            ReportSequence.comment = comment
        end
        function StaticPopupSpecial_Hide(frame)
            ReportSequence.hidden = frame
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineReportRoot" parent="UIParent">
        <Button name="XmlInlineReportButton" parent="XmlInlineReportRoot">
            <Scripts><OnClick>C_ReportSystem.SendReportPlayer(self:GetParent().reportToken, self:GetParent().CommentFrame.EditBox:GetText()); StaticPopupSpecial_Hide(self:GetParent())</OnClick></Scripts>
        </Button>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineReportRoot.reportToken = 42
        XmlInlineReportRoot.CommentFrame = {
            EditBox = {
                value = "spam report",
                GetText = function(self)
                    return self.value
                end,
            },
        }
        XmlInlineReportButton:GetScript("OnClick")(XmlInlineReportButton)
    "#,
    )
    .unwrap();

    let result: (i32, String, bool) = env
        .eval(
            r#"
            return ReportSequence.token,
                   ReportSequence.comment,
                   ReportSequence.hidden == XmlInlineReportRoot
        "#,
        )
        .unwrap();
    assert_eq!(result.0, 42);
    assert_eq!(result.1, "spam report");
    assert!(result.2);
}

#[test]
fn test_create_frame_from_xml_inline_merchant_tab_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.MerchantFrame = {}
        _G.EventRegistry = {}
        function PanelTemplates_SetTab(frame, tab_id)
            frame.selectedTab = tab_id
        end
        function MerchantFrame_Update()
            MerchantFrame.updated = true
        end
        function EventRegistry:TriggerEvent(event_name)
            self.lastEvent = event_name
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineMerchantTabFrame" parent="UIParent">
        <Scripts><OnClick>PanelTemplates_SetTab(MerchantFrame, self:GetID()); MerchantFrame_Update(); EventRegistry:TriggerEvent("MerchantFrame.MerchantTabShow")</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    let frame_id: f64 = env
        .eval("return XmlInlineMerchantTabFrame:GetID()")
        .unwrap();
    env.exec("XmlInlineMerchantTabFrame:GetScript('OnClick')(XmlInlineMerchantTabFrame)")
        .unwrap();

    let result: (f64, bool, String) = env
        .eval(
            r#"
            return MerchantFrame.selectedTab,
                   MerchantFrame.updated == true,
                   EventRegistry.lastEvent
        "#,
        )
        .unwrap();
    assert_eq!(result.0, frame_id);
    assert!(result.1);
    assert_eq!(result.2, "MerchantFrame.MerchantTabShow");
}

#[test]
fn test_create_frame_from_xml_inline_sequence3_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function XmlInlineSequence3Mark(self)
            self.sequence3Marked = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineSequence3Frame" parent="UIParent">
        <Scripts>
            <OnLoad>self.layoutIndex = 145; XmlInlineSequence3Mark(self); self:RegisterEvent("UPDATE_INVENTORY_DURABILITY")</OnLoad>
        </Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let marked: bool = env
        .eval("return XmlInlineSequence3Frame.sequence3Marked == true")
        .unwrap();
    let registered: bool = env
        .eval(r#"return XmlInlineSequence3Frame:IsEventRegistered("UPDATE_INVENTORY_DURABILITY")"#)
        .unwrap();
    let layout_index: i32 = env
        .eval("return XmlInlineSequence3Frame.layoutIndex")
        .unwrap();
    assert!(
        marked,
        "inline three-step sequence should run middle function"
    );
    assert!(
        registered,
        "inline three-step sequence should run trailing method"
    );
    assert_eq!(
        layout_index, 145,
        "inline three-step sequence should run leading assignment"
    );
}

#[test]
fn test_create_frame_from_xml_empty_scripts_are_noops() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlEmptyScriptFrame" parent="UIParent">
        <Scripts>
            <OnEvent></OnEvent>
            <OnShow></OnShow>
            <OnUpdate></OnUpdate>
        </Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let exists: bool = env.eval("return XmlEmptyScriptFrame ~= nil").unwrap();
    assert!(
        exists,
        "XML frame with empty script tags should still be created"
    );
}

#[test]
fn test_create_frame_from_xml_function_inherit_append_preserves_order() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlScriptOrder = {}
        function XmlBaseOnLoad(self)
            table.insert(XmlScriptOrder, "base")
        end
        function XmlChildOnLoad(self)
            table.insert(XmlScriptOrder, "child")
        end
    "#,
    )
    .unwrap();

    register_first_template(
        r#"<Ui><Frame name="XmlInheritedScriptTemplate" virtual="true">
        <Scripts><OnLoad function="XmlBaseOnLoad"/></Scripts>
    </Frame></Ui>"#,
        "XmlInheritedScriptTemplate",
        "Frame",
    );
    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInheritedScriptFrame" parent="UIParent" inherits="XmlInheritedScriptTemplate">
        <Scripts><OnLoad function="XmlChildOnLoad" inherit="append"/></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let order: String = env
        .eval("return table.concat(XmlScriptOrder, ',')")
        .unwrap();
    assert_eq!(
        order, "child,base",
        "inherit='append' should run the new handler before the inherited one"
    );
}

#[test]
fn test_create_frame_from_xml_intrinsic_method_onload_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlIntrinsicFastMixin = {}
        function XmlIntrinsicFastMixin:OnPreLoad()
            self.xmlIntrinsicLoaded = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlIntrinsicFastFrame" parent="UIParent" mixin="XmlIntrinsicFastMixin">
        <Scripts><OnLoad method="OnPreLoad" intrinsicOrder="precall"/></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let loaded: bool = env
        .eval("return XmlIntrinsicFastFrame.xmlIntrinsicLoaded == true")
        .unwrap();
    assert!(loaded, "XML intrinsic OnLoad should fire");
}

#[test]
fn test_create_frame_from_xml_key_values_exist_before_template_child_onload() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    register_first_template(
        r#"<Ui><Frame name="XmlKeyValueTemplate" virtual="true">
        <Frames>
            <Frame parentKey="Child">
                <Scripts>
                    <OnLoad>self.parentLayoutIndex = self:GetParent().layoutIndex</OnLoad>
                </Scripts>
            </Frame>
        </Frames>
    </Frame></Ui>"#,
        "XmlKeyValueTemplate",
        "Frame",
    );

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlKeyValueFastFrame" parent="UIParent" inherits="XmlKeyValueTemplate">
        <KeyValues>
            <KeyValue key="layoutIndex" value="7" type="number"/>
        </KeyValues>
    </Frame></Ui>"#,
        "Frame",
    );

    let layout_index: i32 = env
        .eval("return XmlKeyValueFastFrame.Child.parentLayoutIndex")
        .unwrap();
    assert_eq!(
        layout_index, 7,
        "template child OnLoad should see direct frame key values"
    );
}

#[test]
fn test_create_scrollframe_from_xml_registers_scroll_child() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    let ui = parse_xml(
        r#"<Ui><ScrollFrame name="XmlScrollFrame" parent="UIParent">
        <Size x="200" y="100"/>
        <Anchors><Anchor point="CENTER"/></Anchors>
        <ScrollChild>
            <Frame parentKey="Child">
                <Size x="320" y="180"/>
                <Anchors><Anchor point="TOPLEFT"/></Anchors>
            </Frame>
        </ScrollChild>
    </ScrollFrame></Ui>"#,
    )
    .unwrap();
    match &ui.elements[0] {
        XmlElement::ScrollFrame(f) => {
            create_frame_from_xml(
                &env.loader_env(),
                f,
                "ScrollFrame",
                None,
                None,
                &mut LoadTiming::default(),
            )
            .unwrap();
        }
        _ => panic!("Expected ScrollFrame element"),
    }

    let matches_child: bool = env
        .eval("return XmlScrollFrame:GetScrollChild() == XmlScrollFrame.Child")
        .unwrap();
    assert!(
        matches_child,
        "XML ScrollChild should be registered as the ScrollFrame's scroll child"
    );
}

#[test]
fn test_button_text_without_parent_key_registers_as_text_fontstring() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    let ui = parse_xml(
        r#"<Ui><Button name="XmlButtonText" parent="UIParent">
        <ButtonText name="$parentText"/>
    </Button></Ui>"#,
    )
    .unwrap();
    match &ui.elements[0] {
        XmlElement::Button(f) => {
            create_frame_from_xml(
                &env.loader_env(),
                f,
                "Button",
                None,
                None,
                &mut LoadTiming::default(),
            )
            .unwrap();
        }
        _ => panic!("Expected Button element"),
    }

    let same_text_region: bool = env
        .eval("return XmlButtonText:GetFontString() == XmlButtonTextText")
        .unwrap();
    assert!(
        same_text_region,
        "ButtonText without an explicit parentKey should still back GetFontString()"
    );
}

#[test]
fn item_button_xml_uses_item_button_intrinsic_template() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    let dir = create_test_addon(
        r#"<Ui>
    <Button name="ItemButton" intrinsic="true">
        <NormalTexture name="$parentIcon" parentKey="icon"/>
        <Layers>
            <Layer level="OVERLAY">
                <Texture parentKey="IconBorder"/>
            </Layer>
        </Layers>
    </Button>
    <ItemButton name="XmlIntrinsicItemButton" parent="UIParent"/>
</Ui>"#,
        "TestItemButtonIntrinsic",
    );
    let toc_path = dir.path().join("TestItemButtonIntrinsic.toc");

    load_addon(&env.loader_env(), &toc_path).expect("addon load should succeed");

    let (has_icon, has_border): (bool, bool) = env
        .eval(
            r#"
            return XmlIntrinsicItemButton.icon ~= nil,
                   XmlIntrinsicItemButton.IconBorder ~= nil
            "#,
        )
        .unwrap();
    assert!(
        has_icon,
        "top-level <ItemButton> should inherit the intrinsic ItemButton icon child"
    );
    assert!(
        has_border,
        "top-level <ItemButton> should inherit the intrinsic ItemButton border child"
    );
}

#[test]
fn inherited_statusbar_bar_texture_creates_live_bar_child() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    let dir = create_test_addon(
        r#"<Ui>
    <StatusBar name="SharedStatusBarTemplate" virtual="true">
        <BarTexture parentKey="Bar" file="Interface\Buttons\WHITE8X8"/>
    </StatusBar>
    <StatusBar name="XmlInheritedStatusBar" parent="UIParent" inherits="SharedStatusBarTemplate"/>
</Ui>"#,
        "TestInheritedStatusBarBarTexture",
    );
    let toc_path = dir.path().join("TestInheritedStatusBarBarTexture.toc");

    load_addon(&env.loader_env(), &toc_path).expect("addon load should succeed");

    let (has_status_bar_texture, has_bar_field): (bool, bool) = env
        .eval(
            r#"
            return XmlInheritedStatusBar:GetStatusBarTexture() ~= nil,
                   XmlInheritedStatusBar.Bar ~= nil
            "#,
        )
        .unwrap();
    assert!(
        has_status_bar_texture,
        "StatusBar should create a live bar texture from inherited <BarTexture>"
    );
    assert!(
        has_bar_field,
        "StatusBar should expose inherited <BarTexture parentKey='Bar'> as .Bar"
    );
}

#[test]
fn test_create_frame_from_xml_hidden_starts_hidden() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlHiddenFrame" parent="UIParent" hidden="true">
        <Size x="200" y="100"/><Anchors><Anchor point="CENTER"/></Anchors>
    </Frame></Ui>"#,
        "Frame",
    );

    let shown: bool = env.eval("return XmlHiddenFrame:IsShown()").unwrap();
    let visible: bool = env.eval("return XmlHiddenFrame:IsVisible()").unwrap();
    let effective_alpha: f32 = env
        .eval("return XmlHiddenFrame:GetEffectiveAlpha()")
        .unwrap();

    assert!(!shown, "hidden XML frame should start with shown=false");
    assert!(!visible, "hidden XML frame should not be visible");
    assert_eq!(
        effective_alpha, 0.0,
        "hidden XML frame should start with effective alpha 0"
    );
}

#[test]
fn test_create_frame_from_xml_hidden_not_in_render_buckets() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlHiddenBucketFrame" parent="UIParent" hidden="true">
        <Size x="200" y="100"/><Anchors><Anchor point="CENTER"/></Anchors>
    </Frame></Ui>"#,
        "Frame",
    );

    let frame_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("XmlHiddenBucketFrame")
        .expect("hidden XML frame should exist");
    let buckets = build_strata_buckets(&env);
    let in_buckets = buckets.iter().any(|bucket| bucket.contains(&frame_id));

    assert!(
        !in_buckets,
        "hidden XML frame should never enter visible strata buckets"
    );
}

#[test]
fn test_create_frame_from_xml_with_template() {
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui><Frame name="TestPanelTemplateUnique" virtual="true">
        <Size x="300" y="200"/>
        <Layers><Layer level="ARTWORK">
            <FontString parentKey="TitleText"><Size x="280" y="20"/>
                <Anchors><Anchor point="TOP" y="-10"/></Anchors>
            </FontString>
        </Layer></Layers>
        <Frames><Button parentKey="CloseButton"><Size x="24" y="24"/>
            <Anchors><Anchor point="TOPRIGHT" x="-5" y="-5"/></Anchors>
        </Button></Frames>
    </Frame></Ui>"#,
        "Frame",
    );
    assert!(get_template("TestPanelTemplateUnique").is_some());

    create_first_frame(
        &env,
        r#"<Ui><Frame name="TestPanelUnique" parent="UIParent"
        inherits="TestPanelTemplateUnique">
        <Anchors><Anchor point="CENTER"/></Anchors>
    </Frame></Ui>"#,
        "Frame",
    );

    assert_eq!(
        env.eval::<f32>("return TestPanelUnique:GetWidth()")
            .unwrap(),
        300.0
    );
    assert_eq!(
        env.eval::<f32>("return TestPanelUnique:GetHeight()")
            .unwrap(),
        200.0
    );
    assert!(
        env.eval::<bool>("return TestPanelUnique.TitleText ~= nil")
            .unwrap()
    );
    assert!(
        env.eval::<bool>("return TestPanelUnique.CloseButton ~= nil")
            .unwrap()
    );
}

#[test]
fn test_create_frame_from_xml_template_inheritance_chain() {
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui><Frame name="TestBaseTemplateChain" virtual="true">
        <Size x="100" y="100"/>
        <Layers><Layer level="BACKGROUND">
            <Texture parentKey="Bg" setAllPoints="true"/>
        </Layer></Layers>
    </Frame></Ui>"#,
        "Frame",
    );

    create_first_frame(
        &env,
        r#"<Ui><Frame name="TestDerivedTemplateChain" virtual="true"
        inherits="TestBaseTemplateChain"><Size x="200" y="150"/>
        <Layers><Layer level="ARTWORK">
            <FontString parentKey="Title"><Anchors><Anchor point="TOP" y="-5"/></Anchors></FontString>
        </Layer></Layers>
    </Frame></Ui>"#,
        "Frame",
    );

    create_first_frame(
        &env,
        r#"<Ui><Frame name="TestFinalFrameChain" parent="UIParent"
        inherits="TestDerivedTemplateChain">
        <Anchors><Anchor point="CENTER"/></Anchors>
    </Frame></Ui>"#,
        "Frame",
    );

    assert_eq!(
        env.eval::<f32>("return TestFinalFrameChain:GetWidth()")
            .unwrap(),
        200.0
    );
    assert_eq!(
        env.eval::<f32>("return TestFinalFrameChain:GetHeight()")
            .unwrap(),
        150.0
    );
    assert!(
        env.eval::<bool>("return TestFinalFrameChain.Bg ~= nil")
            .unwrap()
    );
    assert!(
        env.eval::<bool>("return TestFinalFrameChain.Title ~= nil")
            .unwrap()
    );
}

#[test]
fn test_create_frame_from_xml_inherited_template_mixin_available() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        TestTemplateMixin = {}
        function TestTemplateMixin:GetProbeValue()
            return 42
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="TestMixinTemplate" virtual="true" mixin="TestTemplateMixin">
        <Size x="100" y="50"/>
    </Frame></Ui>"#,
        "Frame",
    );

    create_first_frame(
        &env,
        r#"<Ui><Frame name="TestMixinFrame" parent="UIParent" inherits="TestMixinTemplate">
        <Anchors><Anchor point="CENTER"/></Anchors>
    </Frame></Ui>"#,
        "Frame",
    );

    let probe_value: i32 = env.eval("return TestMixinFrame:GetProbeValue()").unwrap();
    assert_eq!(probe_value, 42, "template mixin method should be available");
}

#[test]
fn test_create_frame_from_xml_parent_key() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui><Frame name="ParentKeyTestFrame" parent="UIParent">
        <Size x="400" y="300"/>
        <Frames>
            <Frame parentKey="Header"><Size x="400" y="30"/>
                <Anchors><Anchor point="TOP"/></Anchors>
                <Layers><Layer level="ARTWORK">
                    <FontString parentKey="Title"><Anchors><Anchor point="CENTER"/></Anchors></FontString>
                </Layer></Layers>
            </Frame>
            <Frame parentKey="Content"><Size x="380" y="250"/>
                <Anchors><Anchor point="BOTTOM" y="10"/></Anchors>
            </Frame>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    assert!(
        env.eval::<bool>("return ParentKeyTestFrame.Header ~= nil")
            .unwrap()
    );
    assert!(
        env.eval::<bool>("return ParentKeyTestFrame.Content ~= nil")
            .unwrap()
    );
    assert!(
        env.eval::<bool>("return ParentKeyTestFrame.Header.Title ~= nil")
            .unwrap()
    );

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("ParentKeyTestFrame").unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert!(frame.children_keys.contains_key("Header"));
    assert!(frame.children_keys.contains_key("Content"));
}

#[test]
fn test_create_button_from_xml() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlTestButton" parent="UIParent" text="Click Me">
        <Size x="120" y="30"/><Anchors><Anchor point="CENTER"/></Anchors>
    </Button></Ui>"#,
        "Button",
    );

    assert!(env.eval::<bool>("return XmlTestButton ~= nil").unwrap());
    assert_eq!(
        env.eval::<String>("return XmlTestButton:GetObjectType()")
            .unwrap(),
        "Button"
    );
    assert_eq!(
        env.eval::<String>("return XmlTestButton:GetText() or ''")
            .unwrap(),
        "Click Me"
    );
}

#[test]
fn test_create_frame_from_xml_with_scripts() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui><Frame name="ScriptTestFrame" parent="UIParent">
        <Size x="100" y="100"/>
        <Scripts><OnLoad>self.loadedFlag = true</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );
    assert!(
        env.eval::<bool>("return ScriptTestFrame.loadedFlag == true")
            .unwrap()
    );
}

#[test]
fn test_create_frame_from_xml_with_keyvalues() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui><Frame name="KeyValueTestFrame" parent="UIParent">
        <Size x="100" y="100"/>
        <KeyValues>
            <KeyValue key="myString" value="hello" type="string"/>
            <KeyValue key="myNumber" value="42" type="number"/>
            <KeyValue key="myBool" value="true" type="boolean"/>
        </KeyValues>
    </Frame></Ui>"#,
        "Frame",
    );

    assert_eq!(
        env.eval::<String>("return KeyValueTestFrame.myString")
            .unwrap(),
        "hello"
    );
    assert_eq!(
        env.eval::<i32>("return KeyValueTestFrame.myNumber")
            .unwrap(),
        42
    );
    assert!(env.eval::<bool>("return KeyValueTestFrame.myBool").unwrap());
}

/// Count children of a specific widget type under a named frame.
fn count_typed_children(env: &WowLuaEnv, name: &str, wt: wow_ui_sim::widget::WidgetType) -> usize {
    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name(name).unwrap();
    let frame = state.widgets.get(id).unwrap();
    frame
        .children
        .iter()
        .filter(|&&cid| state.widgets.get(cid).is_some_and(|c| c.widget_type == wt))
        .count()
}

#[test]
fn test_template_children_not_duplicated() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui><Button name="TestCloseButtonBase" virtual="true">
        <Size x="24" y="24"/></Button></Ui>"#,
        "Button",
    );
    create_first_frame(
        &env,
        r#"<Ui><Button name="TestCloseButtonAnchored" virtual="true"
        inherits="TestCloseButtonBase">
        <Anchors><Anchor point="TOPRIGHT" x="-2" y="-2"/></Anchors>
    </Button></Ui>"#,
        "Button",
    );
    create_first_frame(
        &env,
        r#"<Ui><Frame name="TestPanelTemplate" virtual="true">
        <Size x="400" y="300"/>
        <Frames><Button name="$parentCloseButton" parentKey="CloseButton"
            inherits="TestCloseButtonAnchored"/></Frames>
    </Frame></Ui>"#,
        "Frame",
    );
    create_first_frame(
        &env,
        r#"<Ui><Frame name="TestPanelInstance" parent="UIParent"
        inherits="TestPanelTemplate">
        <Anchors><Anchor point="CENTER"/></Anchors>
    </Frame></Ui>"#,
        "Frame",
    );

    assert!(
        env.eval::<bool>("return TestPanelInstance.CloseButton ~= nil")
            .unwrap()
    );
    let n = count_typed_children(
        &env,
        "TestPanelInstance",
        wow_ui_sim::widget::WidgetType::Button,
    );
    assert_eq!(
        n, 1,
        "Template child Button should be created exactly once, found {n}"
    );

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestPanelInstance").unwrap();
    let frame = state.widgets.get(id).unwrap();
    let btn_id = *frame.children_keys.get("CloseButton").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();
    assert!(
        !btn.anchors.is_empty(),
        "CloseButton should have anchors from template"
    );
}
