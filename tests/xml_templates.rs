//! Tests for XML template registration and frame creation from XML.

use std::io::Write;

use wow_ui_sim::loader::load_addon;
use wow_ui_sim::loader::{LoadTiming, create_frame_from_xml};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::{XmlElement, clear_templates, get_template, parse_xml, register_template};

/// Parse XML and create the first frame element via the loader.
fn create_first_frame(env: &WowLuaEnv, xml: &str, widget_type: &str) {
    let ui = parse_xml(xml).unwrap();
    match &ui.elements[0] {
        XmlElement::Frame(f) | XmlElement::Button(f) | XmlElement::EditBox(f) => {
            create_frame_from_xml(
                &env.loader_env(),
                f,
                widget_type,
                None,
                None,
                &mut LoadTiming::default(),
            )
            .unwrap();
        }
        _ => panic!("Expected frame-like element"),
    }
}

fn build_strata_buckets(env: &WowLuaEnv) -> Vec<Vec<u64>> {
    let mut state = env.state().borrow_mut();
    let _ = state.get_strata_buckets();
    state.strata_buckets.as_ref().unwrap().clone()
}

fn create_test_addon(xml: &str, addon_name: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let toc_path = dir.path().join(format!("{addon_name}.toc"));
    let xml_path = dir.path().join(format!("{addon_name}.xml"));
    let mut toc = std::fs::File::create(&toc_path).unwrap();
    writeln!(toc, "## Title: {addon_name}").unwrap();
    writeln!(toc, "{addon_name}.xml").unwrap();
    let mut xml_file = std::fs::File::create(&xml_path).unwrap();
    write!(xml_file, "{xml}").unwrap();
    dir
}

/// Parse XML and register the first element as a template.
fn register_first_template(xml: &str, name: &str, widget_type: &str) {
    let ui = parse_xml(xml).unwrap();
    match &ui.elements[0] {
        XmlElement::Frame(f) | XmlElement::Button(f) | XmlElement::EditBox(f) => {
            register_template(name, widget_type, f.clone());
        }
        _ => panic!("Expected frame-like element"),
    }
}

// ============================================================================
// XML Template Registry Tests
// ============================================================================

#[test]
fn test_register_xml_template() {
    clear_templates();
    let xml = r#"<Ui><Frame name="MyCustomTemplate" virtual="true">
        <Size x="100" y="50"/>
        <Layers><Layer level="ARTWORK">
            <FontString parentKey="Title" inherits="GameFontNormal">
                <Anchors><Anchor point="TOP" y="-5"/></Anchors>
            </FontString>
        </Layer></Layers>
    </Frame></Ui>"#;

    register_first_template(xml, "MyCustomTemplate", "Frame");
    let entry = get_template("MyCustomTemplate").expect("Template should be registered");
    assert_eq!(entry.name, "MyCustomTemplate");
    assert_eq!(entry.widget_type, "Frame");
}

#[test]
fn test_xml_template_with_children() {
    clear_templates();
    let xml = r#"<Ui><Frame name="PanelTemplate" virtual="true">
        <Size x="300" y="200"/>
        <Frames>
            <Frame parentKey="TitleContainer"><Size x="280" y="24"/>
                <Anchors><Anchor point="TOP" y="-10"/></Anchors>
                <Layers><Layer level="ARTWORK">
                    <FontString parentKey="TitleText" inherits="GameFontNormal"/>
                </Layer></Layers>
            </Frame>
            <Button parentKey="CloseButton"><Size x="24" y="24"/>
                <Anchors><Anchor point="TOPRIGHT" x="-5" y="-5"/></Anchors>
            </Button>
        </Frames>
    </Frame></Ui>"#;

    register_first_template(xml, "PanelTemplate", "Frame");
    let template = get_template("PanelTemplate").unwrap();
    assert!(!template.frame.all_frame_elements().is_empty());
}

#[test]
fn test_xml_template_inheritance() {
    clear_templates();
    register_first_template(
        r#"<Ui><Frame name="BaseTemplate" virtual="true"><Size x="100" y="100"/></Frame></Ui>"#,
        "BaseTemplate",
        "Frame",
    );
    register_first_template(
        r#"<Ui><Frame name="DerivedTemplate" virtual="true" inherits="BaseTemplate">
            <Size x="200" y="200"/></Frame></Ui>"#,
        "DerivedTemplate",
        "Frame",
    );
    assert!(get_template("BaseTemplate").is_some());
    let derived = get_template("DerivedTemplate").unwrap();
    assert_eq!(derived.frame.inherits, Some("BaseTemplate".to_string()));
}

// ============================================================================
// CreateFrame with XML Template Tests
// ============================================================================

#[test]
fn test_create_frame_finds_xml_template() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    register_first_template(
        r#"<Ui><Frame name="TestSizeTemplate" virtual="true"><Size x="150" y="75"/></Frame></Ui>"#,
        "TestSizeTemplate",
        "Frame",
    );
    env.exec(r#"local f = CreateFrame("Frame", "TestWithTemplate", UIParent, "TestSizeTemplate")"#)
        .unwrap();
    assert!(env.eval::<bool>("return TestWithTemplate ~= nil").unwrap());
}

#[test]
fn test_create_frame_method_only_template_script_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        TestMethodOnlyTemplateMixin = {}
        function TestMethodOnlyTemplateMixin:OnLoad()
            self.methodOnlyLoaded = true
        end
    "#,
    )
    .unwrap();

    register_first_template(
        r#"<Ui><Frame name="TestMethodOnlyTemplate" virtual="true" mixin="TestMethodOnlyTemplateMixin">
            <Scripts><OnLoad method="OnLoad"/></Scripts>
        </Frame></Ui>"#,
        "TestMethodOnlyTemplate",
        "Frame",
    );

    env.exec(
        r#"local f = CreateFrame("Frame", "TestMethodOnlyFrame", UIParent, "TestMethodOnlyTemplate")"#,
    )
    .unwrap();

    let loaded: bool = env
        .eval("return TestMethodOnlyFrame.methodOnlyLoaded == true")
        .unwrap();
    assert!(loaded, "method-only template OnLoad should fire");
}

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

#[test]
fn test_create_frame_from_xml_inline_function_with_self_and_parent_field_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function XmlInlineParentFieldSetup(self, value)
            self.capturedPartyBackfill = value
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineParentFieldFrame" parent="UIParent">
        <KeyValues>
            <KeyValue key="PartyBackfill" value="true" type="boolean"/>
        </KeyValues>
        <Frames>
            <Button parentKey="Child">
                <Scripts><OnLoad>XmlInlineParentFieldSetup(self, self:GetParent().PartyBackfill)</OnLoad></Scripts>
            </Button>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    let captured: bool = env
        .eval("return XmlInlineParentFieldFrame.Child.capturedPartyBackfill == true")
        .unwrap();
    assert!(
        captured,
        "inline self+parent-field function call should fire"
    );
}

#[test]
fn test_create_frame_from_xml_inline_function_with_number_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlinePageChange = 0
        function XmlInlineSetPage(delta)
            XmlInlinePageChange = delta
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineNumberArgFrame" parent="UIParent">
        <Scripts><OnClick>XmlInlineSetPage(-1)</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec("XmlInlineNumberArgFrame:GetScript('OnClick')(XmlInlineNumberArgFrame)")
        .unwrap();

    let delta: i32 = env.eval("return XmlInlinePageChange").unwrap();
    assert_eq!(delta, -1, "inline numeric function arg should fire");
}

#[test]
fn test_create_frame_from_xml_inline_function_with_self_number_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        function XmlInlineSelectMailTab(self, tab_index)
            self.selectedTab = tab_index
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineSelfNumberArgFrame" parent="UIParent">
        <Scripts><OnClick>XmlInlineSelectMailTab(self, 2)</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec("XmlInlineSelfNumberArgFrame:GetScript('OnClick')(XmlInlineSelfNumberArgFrame)")
        .unwrap();

    let selected: i32 = env
        .eval("return XmlInlineSelfNumberArgFrame.selectedTab")
        .unwrap();
    assert_eq!(selected, 2, "inline self+number function arg should fire");
}

#[test]
fn test_create_frame_from_xml_inline_function_with_string_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineDebugMessage = nil
        function XmlInlineDebug(message)
            XmlInlineDebugMessage = message
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineStringArgFrame" parent="UIParent">
        <Scripts><OnClick>XmlInlineDebug("debug line")</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec("XmlInlineStringArgFrame:GetScript('OnClick')(XmlInlineStringArgFrame)")
        .unwrap();

    let message: String = env.eval("return XmlInlineDebugMessage").unwrap();
    assert_eq!(message, "debug line");
}

#[test]
fn test_create_frame_from_xml_inline_function_with_noarg_function_result_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineTrackedQuestId = nil
        function XmlInlineGetQuestId()
            return 42
        end
        function XmlInlineTrackQuest(quest_id)
            XmlInlineTrackedQuestId = quest_id
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineNoArgFunctionResultFrame" parent="UIParent">
        <Scripts><OnClick>XmlInlineTrackQuest(XmlInlineGetQuestId())</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec(
        "XmlInlineNoArgFunctionResultFrame:GetScript('OnClick')(XmlInlineNoArgFunctionResultFrame)",
    )
    .unwrap();

    let tracked: i32 = env.eval("return XmlInlineTrackedQuestId").unwrap();
    assert_eq!(tracked, 42);
}

#[test]
fn test_create_frame_from_xml_inline_function_with_parent_field_arg_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineTrackedQuestId = nil
        function XmlInlineTrackQuest(quest_id)
            XmlInlineTrackedQuestId = quest_id
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineParentFieldOnlyFrame" parent="UIParent">
        <KeyValues>
            <KeyValue key="questID" value="42" type="number"/>
        </KeyValues>
        <Frames>
            <Button parentKey="Child">
                <Scripts><OnClick>XmlInlineTrackQuest(self:GetParent().questID)</OnClick></Scripts>
            </Button>
        </Frames>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec("XmlInlineParentFieldOnlyFrame.Child:GetScript('OnClick')(XmlInlineParentFieldOnlyFrame.Child)")
        .unwrap();

    let tracked: i32 = env.eval("return XmlInlineTrackedQuestId").unwrap();
    assert_eq!(tracked, 42);
}

#[test]
fn test_create_frame_from_xml_inline_function_with_global_method_result_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        _G.XmlInlineEditBox = {}
        function _G.XmlInlineEditBox:GetText()
            return "filter name"
        end
        XmlInlineCapturedFilterName = nil
        function XmlInlineSetFilterName(name)
            XmlInlineCapturedFilterName = name
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineGlobalMethodResultFrame" parent="UIParent">
        <Scripts><OnClick>XmlInlineSetFilterName(XmlInlineEditBox:GetText())</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec(
        "XmlInlineGlobalMethodResultFrame:GetScript('OnClick')(XmlInlineGlobalMethodResultFrame)",
    )
    .unwrap();

    let captured: String = env.eval("return XmlInlineCapturedFilterName").unwrap();
    assert_eq!(captured, "filter name");
}

#[test]
fn test_create_frame_from_xml_inline_function_with_string_number_args_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlInlineCVarKey = nil
        XmlInlineCVarValue = nil
        function XmlInlineSetCVar(key, value)
            XmlInlineCVarKey = key
            XmlInlineCVarValue = value
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlineStringNumberArgsFrame" parent="UIParent">
        <Scripts><OnClick>XmlInlineSetCVar("addFriendInfoShown", 1)</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec("XmlInlineStringNumberArgsFrame:GetScript('OnClick')(XmlInlineStringNumberArgsFrame)")
        .unwrap();

    let result: (String, i32) = env
        .eval("return XmlInlineCVarKey, XmlInlineCVarValue")
        .unwrap();
    assert_eq!(result.0, "addFriendInfoShown");
    assert_eq!(result.1, 1);
}

#[test]
fn test_create_frame_from_xml_inline_function_with_string_nil_nil_global_args_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        CHATCONFIG_SELECTED_FILTER = "SelectedFilter"
        XmlInlinePopupName = nil
        XmlInlinePopupData = nil
        function StaticPopup_Show(name, _, _, data)
            XmlInlinePopupName = name
            XmlInlinePopupData = data
        end
    "#,
    )
    .unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Button name="XmlInlinePopupShowFrame" parent="UIParent">
        <Scripts><OnClick>StaticPopup_Show("COPY_COMBAT_FILTER", nil, nil, CHATCONFIG_SELECTED_FILTER)</OnClick></Scripts>
    </Button></Ui>"#,
        "Button",
    );

    env.exec("XmlInlinePopupShowFrame:GetScript('OnClick')(XmlInlinePopupShowFrame)")
        .unwrap();

    let result: (String, String) = env
        .eval("return XmlInlinePopupName, XmlInlinePopupData")
        .unwrap();
    assert_eq!(result.0, "COPY_COMBAT_FILTER");
    assert_eq!(result.1, "SelectedFilter");
}

#[test]
fn test_create_frame_from_xml_inline_parent_field_method_with_self_result_runs() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    create_first_frame(
        &env,
        r#"<Ui><Frame name="XmlInlineParentFieldMethodFrame" parent="UIParent">
        <Button parentKey="BuyButton"/>
        <CheckButton name="XmlInlineParentFieldMethodToggle" parent="XmlInlineParentFieldMethodFrame">
            <Scripts><OnClick>self:GetParent().BuyButton:SetEnabled(self:GetChecked())</OnClick></Scripts>
        </CheckButton>
    </Frame></Ui>"#,
        "Frame",
    );

    env.exec(
        r#"
        XmlInlineParentFieldMethodToggle:SetChecked(false)
        XmlInlineParentFieldMethodToggle:GetScript("OnClick")(XmlInlineParentFieldMethodToggle)
    "#,
    )
    .unwrap();
    let disabled: bool = env
        .eval("return XmlInlineParentFieldMethodFrame.BuyButton:IsEnabled()")
        .unwrap();
    assert!(!disabled);

    env.exec(
        r#"
        XmlInlineParentFieldMethodToggle:SetChecked(true)
        XmlInlineParentFieldMethodToggle:GetScript("OnClick")(XmlInlineParentFieldMethodToggle)
    "#,
    )
    .unwrap();
    let enabled: bool = env
        .eval("return XmlInlineParentFieldMethodFrame.BuyButton:IsEnabled()")
        .unwrap();
    assert!(enabled);
}

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

// ============================================================================
// Three-Slice Button Tests
// ============================================================================

const THREE_SLICE_TEMPLATE_XML: &str = r#"<Ui>
    <Button name="ThreeSliceButtonTemplate" mixin="ThreeSliceButtonMixin" virtual="true">
        <Size x="20" y="20"/>
        <Layers><Layer level="BACKGROUND">
            <Texture parentKey="Left"><Anchors><Anchor point="TOPLEFT"/></Anchors></Texture>
            <Texture parentKey="Right"><Anchors><Anchor point="TOPRIGHT"/></Anchors></Texture>
            <Texture parentKey="Center">
                <Anchors>
                    <Anchor point="TOPLEFT" relativeKey="$parent.Left" relativePoint="TOPRIGHT"/>
                    <Anchor point="BOTTOMRIGHT" relativeKey="$parent.Right" relativePoint="BOTTOMLEFT"/>
                </Anchors>
            </Texture>
        </Layer></Layers>
        <Frames><Frame parentKey="Controller" mixin="ButtonControllerMixin">
            <Scripts><OnLoad method="OnLoad"/></Scripts>
        </Frame></Frames>
    </Button>
    <Button name="BigRedThreeSliceButtonTemplate" inherits="ThreeSliceButtonTemplate" virtual="true">
        <Size x="441" y="128"/>
        <KeyValues><KeyValue key="atlasName" value="128-RedButton" type="string"/></KeyValues>
    </Button>
    <Button name="SharedButtonSmallTemplate" inherits="BigRedThreeSliceButtonTemplate" virtual="true">
        <Size x="138" y="28"/>
    </Button>
</Ui>"#;

/// Set up env with three-slice templates and mixins registered.
fn setup_three_slice_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    register_three_slice_templates();
    install_three_slice_mixins(&env);
    env
}

fn register_three_slice_templates() {
    let ui = parse_xml(THREE_SLICE_TEMPLATE_XML).unwrap();
    for element in &ui.elements {
        register_three_slice_button_template(element);
    }
}

fn register_three_slice_button_template(element: &XmlElement) {
    let XmlElement::Button(frame) = element else {
        return;
    };
    let Some(name) = frame.name.as_deref() else {
        return;
    };
    register_template(name, "Button", frame.clone());
}

fn install_three_slice_mixins(env: &WowLuaEnv) {
    env.exec(THREE_SLICE_MIXIN_LUA).unwrap();
}

const THREE_SLICE_MIXIN_LUA: &str = r#"
    ThreeSliceButtonMixin = {}
    function ThreeSliceButtonMixin:InitButton()
        self.leftAtlasInfo = C_Texture.GetAtlasInfo(self.atlasName .. "-Left")
        self.rightAtlasInfo = C_Texture.GetAtlasInfo(self.atlasName .. "-Right")
        self:SetHighlightAtlas(self.atlasName .. "-Highlight")
    end
    function ThreeSliceButtonMixin:UpdateButton(buttonState)
        buttonState = buttonState or "NORMAL"
        self.Left:SetAtlas(self.atlasName .. "-Left", true)
        self.Center:SetAtlas("_" .. self.atlasName .. "-Center")
        self.Right:SetAtlas(self.atlasName .. "-Right", true)
        self:UpdateScale()
    end
    function ThreeSliceButtonMixin:UpdateScale()
        local scale = self:GetHeight() / self.leftAtlasInfo.height
        self.Left:SetScale(scale)
        self.Right:SetScale(scale)
        self.Left:SetTexCoord(0, 1, 0, 1)
        self.Left:SetWidth(self.leftAtlasInfo.width)
        self.Right:SetTexCoord(0, 1, 0, 1)
        self.Right:SetWidth(self.rightAtlasInfo.width)
    end
    ButtonControllerMixin = {}
    function ButtonControllerMixin:OnLoad()
        self:GetParent():InitButton()
    end
"#;

/// Three-slice InitButton runs via Controller:OnLoad after all templates applied.
#[test]
fn test_three_slice_button_texture_scaling() {
    let env = setup_three_slice_env();
    assert!(
        env.eval::<bool>("return C_Texture.GetAtlasInfo('128-RedButton-Left') ~= nil")
            .unwrap()
    );

    let result: String = env.eval(r#"
        local btn = CreateFrame("Button", "TestThreeSliceBtn", UIParent, "SharedButtonSmallTemplate")
        btn:SetSize(120, 22)
        if not btn.leftAtlasInfo then return "leftAtlasInfo nil" end
        if not btn.rightAtlasInfo then return "rightAtlasInfo nil" end
        return "ok"
    "#).unwrap();
    assert!(
        result.starts_with("ok"),
        "InitButton should have run: {result}"
    );
}

/// The three-slice template should end up with Left/Right/Center atlases set
/// after the real InitButton + UpdateButton lifecycle runs.
#[test]
fn test_three_slice_button_children_get_expected_atlases() {
    let env = setup_three_slice_env();
    let result: String = env
        .eval(
            r#"
        local btn = CreateFrame("Button", "TestThreeSliceAtlases", UIParent, "SharedButtonSmallTemplate")
        btn:SetSize(120, 22)
        btn:Show()
        btn:UpdateButton("NORMAL")

        local leftAtlas = btn.Left and btn.Left:GetAtlas() or ""
        local centerAtlas = btn.Center and btn.Center:GetAtlas() or ""
        local rightAtlas = btn.Right and btn.Right:GetAtlas() or ""
        return table.concat({ leftAtlas, centerAtlas, rightAtlas }, "|")
    "#,
        )
        .unwrap();

    assert_eq!(
        result, "128-RedButton-Left|_128-RedButton-Center|128-RedButton-Right",
        "Three-slice button should assign the expected Left/Center/Right atlases"
    );
}

/// Center texture gets non-zero width via cross-frame anchors to Left/Right siblings.
#[test]
fn test_three_slice_center_texture_layout() {
    let env = setup_three_slice_env();
    let result: String = env
        .eval(
            r#"
        local btn = CreateFrame("Button", "TestThreeSlice2", UIParent, "SharedButtonSmallTemplate")
        btn:SetSize(120, 22)
        if not btn.Center then return "Center child missing" end
        if btn.Center:GetNumPoints() ~= 2 then
            return "Center has " .. btn.Center:GetNumPoints() .. " anchors, expected 2"
        end
        btn:UpdateButton()
        local leftW = btn.Left:GetWidth()
        local rightW = btn.Right:GetWidth()
        if leftW == 0 then return "Left width 0" end
        if rightW == 0 then return "Right width 0" end
        local centerW = btn.Center:GetWidth()
        if centerW == 0 then return "Center width 0 (cross-frame anchors not resolving)" end
        return "ok:" .. string.format("L=%.1f R=%.1f C=%.1f", leftW, rightW, centerW)
    "#,
        )
        .unwrap();
    assert!(
        result.starts_with("ok"),
        "Center texture should have non-zero width: {result}"
    );
}
