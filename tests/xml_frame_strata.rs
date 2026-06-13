//! Tests for XML frameStrata and frameLevel attribute parsing.

use wow_ui_sim::loader::{LoadTiming, create_frame_from_xml};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::{XmlElement, clear_templates, parse_xml};

#[test]
fn test_create_frame_from_xml_frame_strata() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    let xml = r#"
        <Ui>
            <Frame name="DialogStrataFrame" parent="UIParent" frameStrata="DIALOG">
                <Size x="200" y="100"/>
            </Frame>
        </Ui>
    "#;

    let ui = parse_xml(xml).unwrap();
    if let XmlElement::Frame(frame) = &ui.elements[0] {
        create_frame_from_xml(
            &env.loader_env(),
            frame,
            "Frame",
            None,
            None,
            None,
            &mut LoadTiming::default(),
        )
        .unwrap();
    }

    let strata: String = env
        .eval("return DialogStrataFrame:GetFrameStrata()")
        .unwrap();
    assert_eq!(strata, "DIALOG");

    // Children should inherit the parent's strata
    let child_strata: String = env
        .eval(
            r#"
            local child = CreateFrame("Frame", "DialogChild", DialogStrataFrame)
            return child:GetFrameStrata()
            "#,
        )
        .unwrap();
    assert_eq!(child_strata, "DIALOG");
}

#[test]
fn test_frame_strata_inherited_from_template() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    let template_xml = r#"
        <Ui>
            <Frame name="HighStrataTemplate" virtual="true" frameStrata="HIGH">
                <Size x="100" y="100"/>
            </Frame>
        </Ui>
    "#;
    let ui = parse_xml(template_xml).unwrap();
    if let XmlElement::Frame(frame) = &ui.elements[0] {
        create_frame_from_xml(
            &env.loader_env(),
            frame,
            "Frame",
            None,
            None,
            None,
            &mut LoadTiming::default(),
        )
        .unwrap();
    }

    let frame_xml = r#"
        <Ui>
            <Frame name="InheritsHighStrata" parent="UIParent" inherits="HighStrataTemplate">
                <Anchors><Anchor point="CENTER"/></Anchors>
            </Frame>
        </Ui>
    "#;
    let ui2 = parse_xml(frame_xml).unwrap();
    if let XmlElement::Frame(frame) = &ui2.elements[0] {
        create_frame_from_xml(
            &env.loader_env(),
            frame,
            "Frame",
            None,
            None,
            None,
            &mut LoadTiming::default(),
        )
        .unwrap();
    }

    let strata: String = env
        .eval("return InheritsHighStrata:GetFrameStrata()")
        .unwrap();
    assert_eq!(strata, "HIGH");
}

#[test]
fn test_xml_frame_level_uses_parent_relative_offset() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    let xml = r#"
        <Ui>
            <Frame name="XmlLevelParent" parent="UIParent" frameLevel="50">
                <Size x="10" y="10"/>
            </Frame>
            <Frame name="XmlLevelChild" parent="XmlLevelParent" frameLevel="10">
                <Size x="10" y="10"/>
            </Frame>
        </Ui>
    "#;

    let ui = parse_xml(xml).unwrap();
    for element in &ui.elements {
        if let XmlElement::Frame(frame) = element {
            create_frame_from_xml(
                &env.loader_env(),
                frame,
                "Frame",
                None,
                None,
                None,
                &mut LoadTiming::default(),
            )
            .unwrap();
        }
    }

    // Ground truth (XmlFrameLevelProbe, retail 12.0.5): a bare XML
    // `frameLevel` is the ABSOLUTE level (10), not a parent-relative offset,
    // and the frame does NOT report IsUsingParentLevel.
    let (parent_level, child_level, child_uses_parent): (i32, i32, bool) = env
        .eval(
            r#"
            return XmlLevelParent:GetFrameLevel(), XmlLevelChild:GetFrameLevel(), XmlLevelChild:IsUsingParentLevel()
            "#,
        )
        .unwrap();
    assert_eq!(parent_level, 50);
    assert_eq!(
        child_level, 10,
        "bare XML frameLevel is absolute, not parent + offset"
    );
    assert!(!child_uses_parent);

    // It is still non-fixed: a later parent level change shifts the child by
    // the parent's delta (probe: parent 50->60 moved childPlain 10->20). Here
    // parent 50->300 is +250, so the child's absolute 10 becomes 260.
    let updated_child_level: i32 = env
        .eval(
            r#"
            XmlLevelParent:SetFrameLevel(300)
            return XmlLevelChild:GetFrameLevel()
            "#,
        )
        .unwrap();
    assert_eq!(updated_child_level, 260);
}

#[test]
fn test_xml_fixed_frame_level_stops_parent_propagation_after_initial_offset() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    let xml = r#"
        <Ui>
            <Frame name="XmlFixedParent" parent="UIParent" frameLevel="50">
                <Size x="10" y="10"/>
            </Frame>
            <Frame name="XmlFixedChild" parent="XmlFixedParent" frameLevel="10" fixedFrameLevel="true">
                <Size x="10" y="10"/>
            </Frame>
        </Ui>
    "#;

    let ui = parse_xml(xml).unwrap();
    for element in &ui.elements {
        if let XmlElement::Frame(frame) = element {
            create_frame_from_xml(
                &env.loader_env(),
                frame,
                "Frame",
                None,
                None,
                None,
                &mut LoadTiming::default(),
            )
            .unwrap();
        }
    }

    // Ground truth (XmlFrameLevelProbe childFixed): absolute level 10, fixed,
    // not using parent level. Parent is 50, so the gap is -40 — the prior
    // `== 10` offset assertion was the disproven April model.
    let (parent_level, child_level, child_uses_parent): (i32, i32, bool) = env
        .eval(
            r#"
            return XmlFixedParent:GetFrameLevel(), XmlFixedChild:GetFrameLevel(), XmlFixedChild:IsUsingParentLevel()
            "#,
        )
        .unwrap();
    assert_eq!(parent_level, 50);
    assert_eq!(child_level, 10, "fixed XML frameLevel is absolute");
    assert!(!child_uses_parent);

    let (child_before, child_after): (i32, i32) = env
        .eval(
            r#"
            local before = XmlFixedChild:GetFrameLevel()
            XmlFixedParent:SetFrameLevel(400)
            return before, XmlFixedChild:GetFrameLevel()
            "#,
        )
        .unwrap();
    assert_eq!(child_after, child_before);
}

#[test]
fn test_xml_frame_level_inherited_from_template_is_parent_relative_offset() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    let template_xml = r#"
        <Ui>
            <Frame name="XmlLevelOffsetTemplate" virtual="true" frameLevel="10">
                <Size x="10" y="10"/>
            </Frame>
        </Ui>
    "#;
    let template_ui = parse_xml(template_xml).unwrap();
    if let XmlElement::Frame(frame) = &template_ui.elements[0] {
        create_frame_from_xml(
            &env.loader_env(),
            frame,
            "Frame",
            None,
            None,
            None,
            &mut LoadTiming::default(),
        )
        .unwrap();
    }

    let instance_xml = r#"
        <Ui>
            <Frame name="XmlTemplateLevelParent" parent="UIParent" frameLevel="80">
                <Size x="10" y="10"/>
            </Frame>
            <Frame name="XmlTemplateLevelChild" parent="XmlTemplateLevelParent" inherits="XmlLevelOffsetTemplate">
                <Size x="10" y="10"/>
            </Frame>
        </Ui>
    "#;
    let instance_ui = parse_xml(instance_xml).unwrap();
    for element in &instance_ui.elements {
        if let XmlElement::Frame(frame) = element {
            create_frame_from_xml(
                &env.loader_env(),
                frame,
                "Frame",
                None,
                None,
                None,
                &mut LoadTiming::default(),
            )
            .unwrap();
        }
    }

    // Ground truth (XmlFrameLevelProbe childTemplated): a frameLevel inherited
    // from a template behaves like an instance bare frameLevel — absolute 10,
    // non-fixed, not using parent level. Parent is 80, so the gap is -70.
    let (parent_level, child_level, child_uses_parent): (i32, i32, bool) = env
        .eval(
            r#"
            return XmlTemplateLevelParent:GetFrameLevel(), XmlTemplateLevelChild:GetFrameLevel(), XmlTemplateLevelChild:IsUsingParentLevel()
            "#,
        )
        .unwrap();
    assert_eq!(parent_level, 80);
    assert_eq!(
        child_level, 10,
        "template-inherited XML frameLevel is absolute, not parent + offset"
    );
    assert!(!child_uses_parent);
}

#[test]
fn test_xml_use_parent_level_overrides_inherited_frame_level() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();

    // Mirrors the DialogBorderTemplate / NineSlicePanelTemplate combo:
    // a high explicit frameLevel on a chain entry that should be overridden
    // by `useParentLevel="true"` on a sibling chain entry.
    let template_xml = r#"
        <Ui>
            <Frame name="HighLevelTemplate" virtual="true" frameLevel="500">
                <Size x="10" y="10"/>
            </Frame>
            <Frame name="ParentLevelBorderTemplate" virtual="true" inherits="HighLevelTemplate" useParentLevel="true">
                <Size x="10" y="10"/>
            </Frame>
        </Ui>
    "#;
    let template_ui = parse_xml(template_xml).unwrap();
    for element in &template_ui.elements {
        if let XmlElement::Frame(frame) = element {
            create_frame_from_xml(
                &env.loader_env(),
                frame,
                "Frame",
                None,
                None,
                None,
                &mut LoadTiming::default(),
            )
            .unwrap();
        }
    }

    let instance_xml = r#"
        <Ui>
            <Frame name="UseParentLevelHost" parent="UIParent" frameLevel="40">
                <Size x="100" y="100"/>
                <Frames>
                    <Frame parentKey="Border" inherits="ParentLevelBorderTemplate"/>
                    <Frame parentKey="Header">
                        <Size x="50" y="20"/>
                    </Frame>
                </Frames>
            </Frame>
        </Ui>
    "#;
    let instance_ui = parse_xml(instance_xml).unwrap();
    if let XmlElement::Frame(frame) = &instance_ui.elements[0] {
        create_frame_from_xml(
            &env.loader_env(),
            frame,
            "Frame",
            None,
            None,
            None,
            &mut LoadTiming::default(),
        )
        .unwrap();
    }

    let (host_level, border_level, header_level): (i32, i32, i32) = env
        .eval(
            r#"
            return UseParentLevelHost:GetFrameLevel(),
                   UseParentLevelHost.Border:GetFrameLevel(),
                   UseParentLevelHost.Header:GetFrameLevel()
            "#,
        )
        .unwrap();
    assert_eq!(
        border_level, host_level,
        "useParentLevel=true should pin the border to the host's level (not host+500)"
    );
    assert!(
        header_level > border_level,
        "default-level sibling should render above the useParentLevel border (header={header_level}, border={border_level})"
    );

    // Border must follow parent level changes (offset 0, not fixed).
    let (host_after, border_after): (i32, i32) = env
        .eval(
            r#"
            UseParentLevelHost:SetFrameLevel(120)
            return UseParentLevelHost:GetFrameLevel(), UseParentLevelHost.Border:GetFrameLevel()
            "#,
        )
        .unwrap();
    assert_eq!(host_after, 120);
    assert_eq!(border_after, host_after);
}
