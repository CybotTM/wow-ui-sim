use super::*;
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
