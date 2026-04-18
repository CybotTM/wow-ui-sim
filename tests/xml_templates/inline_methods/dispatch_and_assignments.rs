use super::*;
#[test]
fn test_create_frame_from_xml_inline_global_method_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineSequenceTarget = {}
        function XmlInlineSequenceTarget:SetOwner(frame, anchor)
            self.owner = frame
            self.anchor = anchor
        end
        function XmlInlineSequenceTarget:SetThing(value)
            self.value = value
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineGlobalMethodSequenceFrame" parent="UIParent">
        <Scripts><OnLoad>XmlInlineSequenceTarget:SetOwner(self, "ANCHOR_RIGHT"); XmlInlineSequenceTarget:SetThing(self.currencyID)</OnLoad></Scripts>
        <KeyValues>
            <KeyValue key="currencyID" value="42" type="number"/>
        </KeyValues>
    </Frame></Ui>"#,
        "Frame",
    );

    let owner_matches: bool = env
        .eval("return XmlInlineSequenceTarget.owner == XmlInlineGlobalMethodSequenceFrame")
        .unwrap();
    let anchor_matches: bool = env
        .eval("return XmlInlineSequenceTarget.anchor == 'ANCHOR_RIGHT'")
        .unwrap();
    let value: f64 = env.eval("return XmlInlineSequenceTarget.value").unwrap();
    assert!(owner_matches);
    assert!(anchor_matches);
    assert_eq!(value, 42.0);
}

#[test]
fn test_create_frame_from_xml_inline_global_method_then_assign_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineGlobalMethodAssignTarget = {}
        function XmlInlineGlobalMethodAssignTarget:Hide()
            self.hidden = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineGlobalMethodAssignFrame" parent="UIParent">
        <Scripts><OnLoad>XmlInlineGlobalMethodAssignTarget:Hide(); self.showingTooltip = false</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let target_hidden: bool = env
        .eval("return XmlInlineGlobalMethodAssignTarget.hidden == true")
        .unwrap();
    let flag: bool = env
        .eval("return XmlInlineGlobalMethodAssignFrame.showingTooltip == false")
        .unwrap();
    assert!(target_hidden);
    assert!(flag);
}

#[test]
fn test_create_frame_from_xml_inline_function_with_self_id_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineSelfIdValue = nil
        function XmlInlineSelfIdFunction(id)
            XmlInlineSelfIdValue = id
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineSelfIdButton" parent="UIParent">
        <Scripts><OnLoad>XmlInlineSelfIdFunction(self:GetID())</OnLoad></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    let button_id: f64 = env.eval("return XmlInlineSelfIdButton:GetID()").unwrap();
    let captured_id: f64 = env.eval("return XmlInlineSelfIdValue").unwrap();
    assert_eq!(captured_id, button_id);
}

#[test]
fn test_create_frame_from_xml_inline_function_with_self_string_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineSelfStringArg = nil
        function XmlInlineSelfStringFunction(frame, mode)
            XmlInlineSelfStringArg = mode
            frame.selfStringApplied = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineSelfStringFrame" parent="UIParent">
        <Scripts><OnLoad>XmlInlineSelfStringFunction(self, "STATIC")</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let applied: bool = env
        .eval("return XmlInlineSelfStringFrame.selfStringApplied == true")
        .unwrap();
    let arg: String = env.eval("return XmlInlineSelfStringArg").unwrap();
    assert!(applied);
    assert_eq!(arg, "STATIC");
}

#[test]
fn test_create_frame_from_xml_inline_function_with_global_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        SOUNDKIT = { XML_INLINE_SOUND = 77 }
        XmlInlineGlobalArgValue = nil
        function XmlInlineGlobalArgFunction(value)
            XmlInlineGlobalArgValue = value
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineGlobalArgFrame" parent="UIParent">
        <Scripts><OnLoad>XmlInlineGlobalArgFunction(SOUNDKIT.XML_INLINE_SOUND)</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let value: f64 = env.eval("return XmlInlineGlobalArgValue").unwrap();
    assert_eq!(value, 77.0);
}

#[test]
fn test_create_frame_from_xml_inline_self_method_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineMethodMixin = {}
        function XmlInlineMethodMixin:Prime()
            self.xmlInlineMethodLoaded = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineMethodFrame" parent="UIParent" mixin="XmlInlineMethodMixin">
        <Scripts><OnLoad>self:Prime();</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let loaded: bool = env
        .eval("return XmlInlineMethodFrame.xmlInlineMethodLoaded == true")
        .unwrap();
    assert!(loaded, "self-method inline OnLoad should fire");
}

#[test]
fn test_create_frame_from_xml_inline_parent_method_with_args_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineParentArgsMixin = {}
        function XmlInlineParentArgsMixin:Prime(button)
            self.parentButton = button
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineParentArgsFrame" parent="UIParent" mixin="XmlInlineParentArgsMixin">
        <Frames>
            <Button parentKey="Child">
                <Scripts><OnClick>self:GetParent():Prime(button)</OnClick></Scripts>
            </Button>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineParentArgsFrame.Child:GetScript('OnClick')(XmlInlineParentArgsFrame.Child, 'LeftButton')")
        .unwrap();

    let button: String = env
        .eval("return XmlInlineParentArgsFrame.parentButton")
        .unwrap();
    assert_eq!(button, "LeftButton");
}

#[test]
fn test_create_frame_from_xml_inline_assign_parent_ref_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineAssignParentRefFrame" parent="UIParent">
        <Frames>
            <Button parentKey="Child">
                <Scripts><OnClick>self.parentRef = self:GetParent()</OnClick></Scripts>
            </Button>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineAssignParentRefFrame.Child:GetScript('OnClick')(XmlInlineAssignParentRefFrame.Child)")
        .unwrap();

    let same_ref: bool = env
        .eval(
            "return XmlInlineAssignParentRefFrame.Child.parentRef == XmlInlineAssignParentRefFrame",
        )
        .unwrap();
    assert!(
        same_ref,
        "inline parent-ref assignment should store parent on self"
    );
}

#[test]
fn test_create_frame_from_xml_inline_assign_grandparent_ref_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineAssignGrandparentRefFrame" parent="UIParent">
        <Frames>
            <Frame parentKey="Middle">
                <Frames>
                    <Button parentKey="Child">
                        <Scripts><OnClick>self.ownerRef = self:GetParent():GetParent()</OnClick></Scripts>
                    </Button>
                </Frames>
            </Frame>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineAssignGrandparentRefFrame.Middle.Child:GetScript('OnClick')(XmlInlineAssignGrandparentRefFrame.Middle.Child)")
        .unwrap();

    let same_ref: bool = env
        .eval("return XmlInlineAssignGrandparentRefFrame.Middle.Child.ownerRef == XmlInlineAssignGrandparentRefFrame")
        .unwrap();
    assert!(
        same_ref,
        "inline grandparent-ref assignment should store grandparent on self"
    );
}

#[test]
fn test_create_frame_from_xml_inline_set_frame_level_from_parent_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineFrameLevelFrame" parent="UIParent">
        <Frame parentKey="Child">
            <Scripts><OnLoad>self:SetFrameLevel(self:GetParent():GetFrameLevel() + 7)</OnLoad></Scripts>
        </Frame>
    </Frame></Ui>"#,
        "Frame",
    );

    let frame_level: f64 = env
        .eval(
            "return XmlInlineFrameLevelFrame.Child:GetFrameLevel() - XmlInlineFrameLevelFrame:GetFrameLevel()",
        )
        .unwrap();
    assert_eq!(
        frame_level, 7.0,
        "inline frame-level adjustment should use parent frame level"
    );
}

#[test]
fn test_create_frame_from_xml_inline_register_for_clicks_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineRegisterClicksButton" parent="UIParent">
        <Scripts><OnLoad>self:RegisterForClicks("LeftButtonUp", "RightButtonUp", "MiddleButtonUp")</OnLoad></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    let exists: bool = env
        .eval("return XmlInlineRegisterClicksButton ~= nil")
        .unwrap();
    assert!(
        exists,
        "inline RegisterForClicks should not break frame creation"
    );
}

#[test]
fn test_create_frame_from_xml_inline_register_for_drag_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineRegisterDragButton" parent="UIParent">
        <Scripts><OnLoad>self:RegisterForDrag("LeftButton")</OnLoad></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    let state = env.state().borrow();
    let frame = state
        .widgets
        .iter_ids()
        .find_map(|id| {
            state
                .widgets
                .get(id)
                .filter(|frame| frame.name.as_deref() == Some("XmlInlineRegisterDragButton"))
        })
        .expect("button should exist");
    assert!(
        frame.registered_drag_buttons.contains("LeftButton"),
        "inline RegisterForDrag should populate drag buttons"
    );
}

#[test]
fn test_create_frame_from_xml_inline_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineSequenceButton" parent="UIParent">
        <Scripts><OnLoad>self:RegisterForClicks("LeftButtonUp", "RightButtonUp"); self:RegisterForDrag("LeftButton")</OnLoad></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    let state = env.state().borrow();
    let frame = state
        .widgets
        .iter_ids()
        .find_map(|id| {
            state
                .widgets
                .get(id)
                .filter(|frame| frame.name.as_deref() == Some("XmlInlineSequenceButton"))
        })
        .expect("button should exist");
    assert!(
        frame.registered_drag_buttons.contains("LeftButton"),
        "inline handler sequence should run both statements"
    );
}

#[test]
fn test_create_frame_from_xml_inline_set_alpha_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineSetAlphaFrame" parent="UIParent">
        <Scripts><OnLoad>self:SetAlpha(0)</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let alpha: f64 = env
        .eval("return XmlInlineSetAlphaFrame:GetAlpha()")
        .unwrap();
    assert_eq!(alpha, 0.0);
}

#[test]
fn test_create_frame_from_xml_inline_parent_method_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineParentMixin = {}
        function XmlInlineParentMixin:Prime()
            self.parentPrimed = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineParentFrame" parent="UIParent" mixin="XmlInlineParentMixin">
        <Frames>
            <Button parentKey="Child">
                <Scripts><OnClick>self:GetParent():Prime()</OnClick></Scripts>
            </Button>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineParentFrame.Child:GetScript('OnClick')(XmlInlineParentFrame.Child)")
        .unwrap();

    let loaded: bool = env
        .eval("return XmlInlineParentFrame.parentPrimed == true")
        .unwrap();
    assert!(loaded, "parent-method inline OnClick should fire");
}

#[test]
fn test_create_frame_from_xml_inline_parent_method_with_empty_string_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineParentStringFrame" parent="UIParent">
        <Frames>
            <Button parentKey="Child">
                <Scripts><OnClick>self:GetParent():SetText("")</OnClick></Scripts>
            </Button>
        </Frames>
    </Button></Ui>"#,
        "Button",
    );

    env.exec(r#"XmlInlineParentStringFrame:SetText("Seed")"#)
        .unwrap();
    env.exec(
        "XmlInlineParentStringFrame.Child:GetScript('OnClick')(XmlInlineParentStringFrame.Child)",
    )
    .unwrap();

    let value: String = env
        .eval("return XmlInlineParentStringFrame:GetText()")
        .unwrap();
    assert_eq!(
        value, "",
        "parent-method inline empty-string arg should fire"
    );
}

#[test]
fn test_create_frame_from_xml_inline_grandparent_method_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineGrandparentMixin = {}
        function XmlInlineGrandparentMixin:Prime()
            self.grandparentPrimed = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineGrandparentFrame" parent="UIParent" mixin="XmlInlineGrandparentMixin">
        <Frames>
            <Frame parentKey="Middle">
                <Frames>
                    <Button parentKey="Child">
                        <Scripts><OnClick>self:GetParent():GetParent():Prime()</OnClick></Scripts>
                    </Button>
                </Frames>
            </Frame>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineGrandparentFrame.Middle.Child:GetScript('OnClick')(XmlInlineGrandparentFrame.Middle.Child)").unwrap();

    let loaded: bool = env
        .eval("return XmlInlineGrandparentFrame.grandparentPrimed == true")
        .unwrap();
    assert!(loaded, "grandparent-method inline OnClick should fire");
}

#[test]
fn test_create_frame_from_xml_inline_method_with_bool_arg_after_comment_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineBoolFrame" parent="UIParent">
        <Scripts><OnLoad>-- disabled by request
            self:EnableMouse(false);
        </OnLoad></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    let enabled: bool = env
        .eval("return XmlInlineBoolFrame:IsMouseEnabled()")
        .unwrap();
    assert!(
        !enabled,
        "inline bool-arg method should run after leading comment"
    );
}

#[test]
fn test_create_frame_from_xml_inline_method_with_string_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineStringArgFrame" parent="UIParent">
        <Scripts><OnLoad>self:RegisterEvent("UPDATE_INVENTORY_DURABILITY")</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let registered: bool = env
        .eval(r#"return XmlInlineStringArgFrame:IsEventRegistered("UPDATE_INVENTORY_DURABILITY")"#)
        .unwrap();
    assert!(registered, "inline string-arg self method should fire");
}

#[test]
fn test_create_frame_from_xml_inline_parent_assignment_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec("XmlInlineParentAssignmentValue = 13").unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineParentAssignmentFrame" parent="UIParent">
        <Frames>
            <Button parentKey="Child">
                <Scripts><OnClick>self:GetParent().layoutIndex = XmlInlineParentAssignmentValue</OnClick></Scripts>
            </Button>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineParentAssignmentFrame.Child:GetScript('OnClick')(XmlInlineParentAssignmentFrame.Child)").unwrap();

    let value: f64 = env
        .eval("return XmlInlineParentAssignmentFrame.layoutIndex")
        .unwrap();
    assert_eq!(value, 13.0, "parent assignment inline OnClick should fire");
}

#[test]
fn test_create_frame_from_xml_inline_assignment_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec("XmlInlineAssignmentValue = 7").unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineAssignmentFrame" parent="UIParent">
        <Scripts><OnLoad>self.layoutIndex = XmlInlineAssignmentValue;</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let value: f64 = env
        .eval("return XmlInlineAssignmentFrame.layoutIndex")
        .unwrap();
    assert_eq!(value, 7.0, "inline assignment OnLoad should fire");
}

#[test]
fn test_create_frame_from_xml_inline_function_with_global_and_self_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineGlobalContainer = {}
        function XmlInlineRemoveFrame(container, frame)
            container.last = frame
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineGlobalSelfFrame" parent="UIParent">
        <Frames>
            <Button parentKey="Child">
                <Scripts><OnClick>XmlInlineRemoveFrame(XmlInlineGlobalContainer, self)</OnClick></Scripts>
            </Button>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineGlobalSelfFrame.Child:GetScript('OnClick')(XmlInlineGlobalSelfFrame.Child)")
        .unwrap();

    let matched: bool = env
        .eval("return XmlInlineGlobalContainer.last == XmlInlineGlobalSelfFrame.Child")
        .unwrap();
    assert!(matched, "inline global+self function call should fire");
}

#[test]
fn test_create_frame_from_xml_inline_nested_assignment_sequence_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function XmlInlineAssignedClick(self)
            self.assignedClickRan = true
        end
        function XmlInlineRunAssignedClick(self)
            self.checkButton.onClick(self.checkButton)
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineNestedAssignFrame" parent="UIParent">
        <Frames>
            <Button parentKey="checkButton"/>
        </Frames>
        <Scripts>
            <OnLoad>self.checkButton.onClick = XmlInlineAssignedClick; XmlInlineRunAssignedClick(self)</OnLoad>
        </Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let assigned: bool = env
        .eval("return XmlInlineNestedAssignFrame.checkButton.assignedClickRan == true")
        .unwrap();
    assert!(assigned, "inline nested assignment sequence should fire");
}

#[test]
fn test_create_frame_from_xml_inherit_prepend_nested_table_assignment_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.ShoppingTooltip1 = { name = "ShoppingTooltip1" }
        _G.ShoppingTooltip2 = { name = "ShoppingTooltip2" }
    "#,
    )
    .unwrap();

    register_first_template(
        r#"<Ui>
        <Frame name="XmlInlineShoppingTooltipTemplate" virtual="true">
            <Frames>
                <Frame parentKey="Tooltip"/>
            </Frames>
            <Scripts>
                <OnLoad>self.inheritedRan = true</OnLoad>
            </Scripts>
        </Frame>
    </Ui>"#,
        "XmlInlineShoppingTooltipTemplate",
        "Frame",
    );

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineShoppingTooltipFrame" parent="UIParent" inherits="XmlInlineShoppingTooltipTemplate">
        <Scripts>
            <OnLoad inherit="prepend">self.Tooltip.shoppingTooltips = { ShoppingTooltip1, ShoppingTooltip2 }</OnLoad>
        </Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let result: (bool, String, String) = env
        .eval(
            r#"
            return XmlInlineShoppingTooltipFrame.inheritedRan == true,
                   XmlInlineShoppingTooltipFrame.Tooltip.shoppingTooltips[1].name,
                   XmlInlineShoppingTooltipFrame.Tooltip.shoppingTooltips[2].name
        "#,
        )
        .unwrap();
    assert!(result.0);
    assert_eq!(result.1, "ShoppingTooltip1");
    assert_eq!(result.2, "ShoppingTooltip2");
}
