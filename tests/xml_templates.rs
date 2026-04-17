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
        XmlElement::Frame(f) | XmlElement::Button(f) => {
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
        _ => panic!("Expected Frame or Button element"),
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
        XmlElement::Frame(f) | XmlElement::Button(f) => {
            register_template(name, widget_type, f.clone());
        }
        _ => panic!("Expected Frame or Button element"),
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
