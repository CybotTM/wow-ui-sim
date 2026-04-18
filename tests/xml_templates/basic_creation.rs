use super::*;
// ============================================================================
// Frame Creation from XML Tests
// ============================================================================

#[test]
fn test_create_frame_from_xml_basic() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlTestFrame" parent="UIParent">
        <Size x="200" y="100"/><Anchors><Anchor point="CENTER"/></Anchors>
    </Frame></Ui>"#,
        "Frame",
    );

    assert!(env.eval::<bool>("return XmlTestFrame ~= nil").unwrap());
    assert_eq!(
        env.eval::<f32>("return XmlTestFrame:GetWidth()").unwrap(),
        200.0
    );
    assert_eq!(
        env.eval::<f32>("return XmlTestFrame:GetHeight()").unwrap(),
        100.0
    );
}

#[test]
fn test_create_frame_from_xml_method_only_onload_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlMethodOnlyMixin = {}
        function XmlMethodOnlyMixin:OnLoad()
            self.xmlMethodLoaded = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlMethodOnlyFrame" parent="UIParent" mixin="XmlMethodOnlyMixin">
        <Scripts><OnLoad method="OnLoad"/></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let loaded: bool = env
        .eval("return XmlMethodOnlyFrame.xmlMethodLoaded == true")
        .unwrap();
    assert!(loaded, "XML method-only OnLoad should fire");
}

#[test]
fn test_create_frame_from_xml_function_only_onload_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function XmlFunctionOnlyOnLoad(self)
            self.xmlFunctionLoaded = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlFunctionOnlyFrame" parent="UIParent">
        <Scripts><OnLoad function="XmlFunctionOnlyOnLoad"/></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let loaded: bool = env
        .eval("return XmlFunctionOnlyFrame.xmlFunctionLoaded == true")
        .unwrap();
    assert!(loaded, "XML function-only OnLoad should fire");
}

#[test]
fn test_create_frame_from_xml_inline_function_call_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function XmlInlineBodyOnLoad(self)
            self.xmlInlineLoaded = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineFunctionFrame" parent="UIParent">
        <Scripts><OnLoad>XmlInlineBodyOnLoad(self);</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let loaded: bool = env
        .eval("return XmlInlineFunctionFrame.xmlInlineLoaded == true")
        .unwrap();
    assert!(loaded, "single-call inline OnLoad should fire");
}

#[test]
fn test_create_frame_from_xml_inline_noarg_function_call_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineNoArgCount = 0
        function XmlInlineNoArgOnLoad()
            XmlInlineNoArgCount = XmlInlineNoArgCount + 1
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineNoArgFrame" parent="UIParent">
        <Scripts><OnLoad>XmlInlineNoArgOnLoad();</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let calls: f64 = env.eval("return XmlInlineNoArgCount").unwrap();
    assert_eq!(calls, 1.0, "single-call inline no-arg OnLoad should fire");
}

#[test]
fn test_create_frame_from_xml_inline_parent_function_call_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function XmlInlineParentFunction(target)
            target.parentFunctionHit = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineParentFunctionFrame" parent="UIParent">
        <Frames>
            <Button parentKey="Child">
                <Scripts><OnClick>XmlInlineParentFunction(self:GetParent())</OnClick></Scripts>
            </Button>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineParentFunctionFrame.Child:GetScript('OnClick')(XmlInlineParentFunctionFrame.Child)").unwrap();

    let loaded: bool = env
        .eval("return XmlInlineParentFunctionFrame.parentFunctionHit == true")
        .unwrap();
    assert!(loaded, "parent-arg inline function OnClick should fire");
}

#[test]
fn test_create_frame_from_xml_inline_grandparent_function_call_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function XmlInlineGrandparentFunction(target)
            target.grandparentFunctionHit = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineGrandparentFunctionFrame" parent="UIParent">
        <Frames>
            <Frame parentKey="Middle">
                <Frames>
                    <Button parentKey="Child">
                        <Scripts><OnClick>XmlInlineGrandparentFunction(self:GetParent():GetParent())</OnClick></Scripts>
                    </Button>
                </Frames>
            </Frame>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineGrandparentFunctionFrame.Middle.Child:GetScript('OnClick')(XmlInlineGrandparentFunctionFrame.Middle.Child)").unwrap();

    let loaded: bool = env
        .eval("return XmlInlineGrandparentFunctionFrame.grandparentFunctionHit == true")
        .unwrap();
    assert!(
        loaded,
        "grandparent-arg inline function OnClick should fire"
    );
}

#[test]
fn test_create_frame_from_xml_inline_parent_id_function_call_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineParentIdValue = 0
        function XmlInlineParentIdFunction(id)
            XmlInlineParentIdValue = id
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineParentIdFrame" parent="UIParent">
        <Frames>
            <Button parentKey="Child">
                <Scripts><OnClick>XmlInlineParentIdFunction(self:GetParent():GetID())</OnClick></Scripts>
            </Button>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineParentIdFrame.Child:GetScript('OnClick')(XmlInlineParentIdFrame.Child)")
        .unwrap();

    let parent_id: f64 = env.eval("return XmlInlineParentIdFrame:GetID()").unwrap();
    let captured_id: f64 = env.eval("return XmlInlineParentIdValue").unwrap();
    assert_eq!(
        captured_id, parent_id,
        "parent-id inline function should see parent id"
    );
}

#[test]
fn test_create_frame_from_xml_inline_global_method_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineGlobalMethodTarget = {}
        function XmlInlineGlobalMethodTarget:Hide()
            self.hidden = true
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineGlobalMethodFrame" parent="UIParent">
        <Scripts><OnLoad>XmlInlineGlobalMethodTarget:Hide()</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let hidden: bool = env
        .eval("return XmlInlineGlobalMethodTarget.hidden == true")
        .unwrap();
    assert!(hidden, "inline global-target method call should fire");
}

#[test]
fn test_create_frame_from_xml_inline_global_method_with_self_string_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineGlobalMethodStringTarget = {}
        function XmlInlineGlobalMethodStringTarget:SetOwner(frame, anchor)
            self.owner = frame
            self.anchor = anchor
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineGlobalMethodStringFrame" parent="UIParent">
        <Scripts><OnLoad>XmlInlineGlobalMethodStringTarget:SetOwner(self, "ANCHOR_RIGHT")</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let anchor_matches: bool = env
        .eval("return XmlInlineGlobalMethodStringTarget.anchor == 'ANCHOR_RIGHT'")
        .unwrap();
    assert!(anchor_matches);
}

#[test]
fn test_create_frame_from_xml_inline_named_global_method_with_global_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.TEST_FAST_NAMED_GLOBAL_COLOR = "Color by school"
        _G.XmlInlineNamedGlobalFrameText = {}
        function _G.XmlInlineNamedGlobalFrameText:SetText(text)
            self.text = text
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineNamedGlobalFrame" parent="UIParent">
        <Scripts><OnLoad>_G[self:GetName().."Text"]:SetText(TEST_FAST_NAMED_GLOBAL_COLOR)</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let text: String = env
        .eval("return XmlInlineNamedGlobalFrameText.text")
        .unwrap();
    assert_eq!(text, "Color by school");
}

#[test]
fn test_create_frame_from_xml_inline_named_global_method_sequence_with_assignment_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.TEST_FAST_NAMED_GLOBAL_COLOR = "Color by school"
        _G.TEST_FAST_NAMED_GLOBAL_TOOLTIP = "school tooltip"
        _G.XmlInlineNamedGlobalSequenceFrameText = {}
        function _G.XmlInlineNamedGlobalSequenceFrameText:SetText(text)
            self.text = text
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineNamedGlobalSequenceFrame" parent="UIParent">
        <Scripts><OnLoad>_G[self:GetName().."Text"]:SetText(TEST_FAST_NAMED_GLOBAL_COLOR); self.tooltip = TEST_FAST_NAMED_GLOBAL_TOOLTIP</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let result: (String, String) = env
        .eval(
            r#"
            return XmlInlineNamedGlobalSequenceFrameText.text,
                   XmlInlineNamedGlobalSequenceFrame.tooltip
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "Color by school");
    assert_eq!(result.1, "school tooltip");
}

#[test]
fn test_create_frame_from_xml_inline_conditional_tooltip_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.TEST_FAST_TOOLTIP_COLOR = { r = 0.1, g = 0.2, b = 0.3 }
        _G.GameTooltip = {}
        function _G.GameTooltip:SetOwner(frame, anchor)
            self.owner = frame
            self.anchor = anchor
        end
        function _G.GameTooltip:SetText(text, r, g, b)
            self.text = text
            self.r = r
            self.g = g
            self.b = b
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineTooltipConditionalFrame" parent="UIParent">
        <Scripts>
            <OnLoad>self.tooltip = "tooltip text"</OnLoad>
            <OnEnter>if (self.tooltip) then GameTooltip:SetOwner(self, "ANCHOR_RIGHT"); GameTooltip:SetText(self.tooltip, TEST_FAST_TOOLTIP_COLOR.r, TEST_FAST_TOOLTIP_COLOR.g, TEST_FAST_TOOLTIP_COLOR.b); end</OnEnter>
        </Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec(
        "XmlInlineTooltipConditionalFrame:GetScript('OnEnter')(XmlInlineTooltipConditionalFrame)",
    )
    .unwrap();

    let result: (String, String, f64, f64, f64) = env
        .eval(
            r#"
            return GameTooltip.anchor,
                   GameTooltip.text,
                   GameTooltip.r,
                   GameTooltip.g,
                   GameTooltip.b
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "ANCHOR_RIGHT");
    assert_eq!(result.1, "tooltip text");
    assert_eq!(result.2, 0.1);
    assert_eq!(result.3, 0.2);
    assert_eq!(result.4, 0.3);
}

#[test]
fn test_create_frame_from_xml_inline_global_tooltip_settext_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.ARTIFACT_XP_REWARD = "artifact xp"
        _G.HIGHLIGHT_FONT_COLOR = { r = 0.1, g = 0.2, b = 0.3 }
        _G.GameTooltip = {}
        function _G.GameTooltip:SetOwner(frame, anchor)
            self.owner = frame
            self.anchor = anchor
        end
        function _G.GameTooltip:SetText(text, r, g, b, a, wrap)
            self.text = text
            self.r = r
            self.g = g
            self.b = b
            self.a = a
            self.wrap = wrap
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineTooltipGlobalFrame" parent="UIParent">
        <Scripts><OnEnter>GameTooltip:SetOwner(self, "ANCHOR_RIGHT"); GameTooltip:SetText(ARTIFACT_XP_REWARD, HIGHLIGHT_FONT_COLOR.r, HIGHLIGHT_FONT_COLOR.g, HIGHLIGHT_FONT_COLOR.b, nil, true)</OnEnter></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineTooltipGlobalFrame:GetScript('OnEnter')(XmlInlineTooltipGlobalFrame)")
        .unwrap();

    let result: (String, String, f64, f64, f64, bool) = env
        .eval(
            r#"
            return GameTooltip.anchor,
                   GameTooltip.text,
                   GameTooltip.r,
                   GameTooltip.g,
                   GameTooltip.b,
                   GameTooltip.wrap
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "ANCHOR_RIGHT");
    assert_eq!(result.1, "artifact xp");
    assert_eq!(result.2, 0.1);
    assert_eq!(result.3, 0.2);
    assert_eq!(result.4, 0.3);
    assert!(result.5);
}

#[test]
fn test_create_frame_from_xml_inline_global_tooltip_literal_settext_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.GameTooltip = {}
        function _G.GameTooltip:SetOwner(frame, anchor)
            self.owner = frame
            self.anchor = anchor
        end
        function _G.GameTooltip:SetText(text, r, g, b)
            self.text = text
            self.r = r
            self.g = g
            self.b = b
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineTooltipLiteralFrame" parent="UIParent">
        <Scripts><OnEnter>GameTooltip:SetOwner(self, "ANCHOR_RIGHT"); GameTooltip:SetText("", 1.0, 1.0, 1.0)</OnEnter></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineTooltipLiteralFrame:GetScript('OnEnter')(XmlInlineTooltipLiteralFrame)")
        .unwrap();

    let result: (String, String, f64, f64, f64) = env
        .eval(
            r#"
            return GameTooltip.anchor,
                   GameTooltip.text,
                   GameTooltip.r,
                   GameTooltip.g,
                   GameTooltip.b
        "#,
        )
        .unwrap();
    assert_eq!(result.0, "ANCHOR_RIGHT");
    assert_eq!(result.1, "");
    assert_eq!(result.2, 1.0);
    assert_eq!(result.3, 1.0);
    assert_eq!(result.4, 1.0);
}

#[test]
fn test_create_frame_from_xml_inline_self_field_method_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineSelfFieldMethodFrame" parent="UIParent">
        <Layers>
            <Layer level="OVERLAY">
                <Texture parentKey="Highlight" hidden="true"/>
            </Layer>
        </Layers>
        <Scripts><OnLoad>self.Highlight:Show()</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let shown: bool = env
        .eval("return XmlInlineSelfFieldMethodFrame.Highlight:IsShown()")
        .unwrap();
    assert!(shown);
}

#[test]
fn test_create_frame_from_xml_inline_self_field_method_with_string_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineSelfFieldStringFrame" parent="UIParent">
        <Layers>
            <Layer level="OVERLAY">
                <FontString parentKey="Name"/>
            </Layer>
        </Layers>
        <Scripts><OnLoad>self.Name:SetText("Hello")</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let text: String = env
        .eval("return XmlInlineSelfFieldStringFrame.Name:GetText()")
        .unwrap();
    assert_eq!(text, "Hello");
}

#[test]
fn test_create_frame_from_xml_inline_self_field_method_with_number_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineSelfFieldNumberFrame" parent="UIParent">
        <Layers>
            <Layer level="OVERLAY">
                <Texture parentKey="texture"/>
            </Layer>
        </Layers>
        <Scripts><OnLoad>self.texture:SetAlpha(0.5)</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let alpha: f64 = env
        .eval("return XmlInlineSelfFieldNumberFrame.texture:GetAlpha()")
        .unwrap();
    assert_eq!(alpha, 0.5);
}

#[test]
fn test_create_frame_from_xml_inline_self_field_method_with_string_number_number_args_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineSelfFieldPointFrame" parent="UIParent">
        <Layers>
            <Layer level="OVERLAY">
                <Texture parentKey="texture"/>
            </Layer>
        </Layers>
        <Scripts><OnLoad>self.texture:SetPoint("TOPLEFT", 1, -1)</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let (point, relative_point, x, y): (String, String, f64, f64) = env
        .eval(
            "local point, _, relativePoint, x, y = XmlInlineSelfFieldPointFrame.texture:GetPoint(1); return point, relativePoint, x, y",
        )
        .unwrap();
    assert_eq!(point, "TOPLEFT");
    assert_eq!(relative_point, "TOPLEFT");
    assert_eq!(x, 1.0);
    assert_eq!(y, -1.0);
}

#[test]
fn test_create_frame_from_xml_inline_self_field_method_with_global_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"XML_INLINE_LABEL = "GlobalHello""#).unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineSelfFieldGlobalFrame" parent="UIParent">
        <Layers>
            <Layer level="OVERLAY">
                <FontString parentKey="Name"/>
            </Layer>
        </Layers>
        <Scripts><OnLoad>self.Name:SetText(XML_INLINE_LABEL)</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let text: String = env
        .eval("return XmlInlineSelfFieldGlobalFrame.Name:GetText()")
        .unwrap();
    assert_eq!(text, "GlobalHello");
}

#[test]
fn test_create_frame_from_xml_inline_self_field_method_with_self_field_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineSelfFieldArgFrame" parent="UIParent">
        <Layers>
            <Layer level="OVERLAY">
                <FontString parentKey="Name"/>
            </Layer>
        </Layers>
        <KeyValues>
            <KeyValue key="tooltipText" value="TooltipValue" type="string"/>
        </KeyValues>
        <Scripts><OnLoad>self.Name:SetText(self.tooltipText)</OnLoad></Scripts>
    </Frame></Ui>"#,
        "Frame",
    );

    let text: String = env
        .eval("return XmlInlineSelfFieldArgFrame.Name:GetText()")
        .unwrap();
    assert_eq!(text, "TooltipValue");
}

#[test]
fn test_create_frame_from_xml_inline_global_method_with_self_id_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineGlobalMethodSelfIdTarget = {}
        function XmlInlineGlobalMethodSelfIdTarget:SetThing(id)
            self.id = id
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineGlobalMethodSelfIdButton" parent="UIParent">
        <Scripts><OnLoad>XmlInlineGlobalMethodSelfIdTarget:SetThing(self:GetID())</OnLoad></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    let button_id: f64 = env
        .eval("return XmlInlineGlobalMethodSelfIdButton:GetID()")
        .unwrap();
    let captured_id: f64 = env
        .eval("return XmlInlineGlobalMethodSelfIdTarget.id")
        .unwrap();
    assert_eq!(captured_id, button_id);
}

#[test]
fn test_create_frame_from_xml_inline_global_method_with_self_field_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineGlobalMethodFieldTarget = {}
        function XmlInlineGlobalMethodFieldTarget:SetThing(value)
            self.value = value
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineGlobalMethodFieldFrame" parent="UIParent">
        <Scripts><OnLoad>XmlInlineGlobalMethodFieldTarget:SetThing(self.tooltipText)</OnLoad></Scripts>
        <KeyValues>
            <KeyValue key="tooltipText" value="TooltipValue" type="string"/>
        </KeyValues>
    </Frame></Ui>"#,
        "Frame",
    );

    let value: String = env
        .eval("return XmlInlineGlobalMethodFieldTarget.value")
        .unwrap();
    assert_eq!(value, "TooltipValue");
}
