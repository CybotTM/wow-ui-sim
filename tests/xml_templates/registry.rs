use super::*;
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

#[test]
fn test_env_reinstalls_intrinsic_templates_after_clear() {
    clear_templates();
    let _env = WowLuaEnv::new().unwrap();
    assert!(
        get_template("WoWScrollBox").is_some(),
        "WowLuaEnv::new should restore intrinsic XML templates after a clear"
    );
}

#[test]
fn intrinsic_dropdown_scripts_chain_with_style_template_scripts() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        XmlIntrinsicDropdownCalls = {}

        XmlIntrinsicDropdownMixin = {}
        function XmlIntrinsicDropdownMixin:OnMouseDown_Intrinsic()
            table.insert(XmlIntrinsicDropdownCalls, "intrinsic")
        end

        XmlStyleDropdownMixin = {}
        function XmlStyleDropdownMixin:OnMouseDown()
            table.insert(XmlIntrinsicDropdownCalls, "style")
        end
    "#,
    )
    .unwrap();
    let dir = create_test_addon(
        r#"<Ui>
            <DropdownButton name="DropdownButton" intrinsic="true" mixin="XmlIntrinsicDropdownMixin">
                <Scripts>
                    <OnMouseDown method="OnMouseDown_Intrinsic"/>
                </Scripts>
            </DropdownButton>
            <DropdownButton name="XmlStyleDropdownTemplate" virtual="true" mixin="XmlStyleDropdownMixin">
                <Scripts>
                    <OnMouseDown method="OnMouseDown"/>
                </Scripts>
            </DropdownButton>
            <DropdownButton name="XmlConcreteDropdown" parent="UIParent" inherits="XmlStyleDropdownTemplate"/>
        </Ui>"#,
        "TestIntrinsicDropdownScripts",
    );
    let toc_path = dir.path().join("TestIntrinsicDropdownScripts.toc");

    load_addon(&env.loader_env(), &toc_path).expect("addon load should succeed");
    env.exec(
        r#"
        local handler = XmlConcreteDropdown:GetScript("OnMouseDown")
        handler(XmlConcreteDropdown)
    "#,
    )
    .unwrap();

    let calls: String = env
        .eval("return table.concat(XmlIntrinsicDropdownCalls, ',')")
        .unwrap();
    assert_eq!(
        calls, "style,intrinsic",
        "derived style handlers should not replace intrinsic dropdown handlers"
    );
}

#[test]
fn sibling_virtual_button_templates_do_not_share_onclick_scripts() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec("SiblingTemplateCalls = {}").unwrap();
    let dir = create_test_addon(
        r#"<Ui>
            <Button name="FirstSiblingButtonTemplate" virtual="true">
                <Scripts>
                    <OnClick>table.insert(SiblingTemplateCalls, "first")</OnClick>
                </Scripts>
            </Button>
            <Button name="SecondSiblingButtonTemplate" virtual="true">
                <Scripts>
                    <OnClick>table.insert(SiblingTemplateCalls, "second")</OnClick>
                </Scripts>
            </Button>
            <Button name="ConcreteSiblingButton" parent="UIParent" inherits="SecondSiblingButtonTemplate"/>
        </Ui>"#,
        "TestSiblingButtonTemplateScripts",
    );
    let toc_path = dir.path().join("TestSiblingButtonTemplateScripts.toc");

    load_addon(&env.loader_env(), &toc_path).expect("addon load should succeed");
    env.exec("ConcreteSiblingButton:Click()").unwrap();

    let calls: String = env
        .eval("return table.concat(SiblingTemplateCalls, ',')")
        .unwrap();
    assert_eq!(
        calls, "second",
        "a concrete frame should inherit only its named template script"
    );
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
