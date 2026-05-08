use super::*;

#[test]
fn test_xml_texture_color() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-texcolor");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test_texcolor.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Frame name="ColorTexFrame" parent="UIParent">
            <Size x="100" y="100"/>
            <Layers><Layer level="BACKGROUND">
                <Texture name="ColorTexFrame_BG" parentKey="bg">
                    <Size x="100" y="100"/>
                    <Color r="1.0" g="0.5" b="0.25" a="0.8"/>
                </Texture>
            </Layer></Layers>
        </Frame>
    </Ui>"#,
    )
    .unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx =
        AddonContext::new(env.lua(), "TestAddon", addon_table, &temp_dir, false, false).unwrap();
    load_xml_file(
        &env.loader_env(),
        &xml_path,
        &ctx,
        &mut LoadTiming::default(),
    )
    .unwrap();

    assert!(
        env.eval::<bool>("return ColorTexFrame.bg ~= nil").unwrap(),
        "bg should exist"
    );
    assert!(
        env.eval::<bool>("return ColorTexFrame_BG ~= nil").unwrap(),
        "BG should exist as global"
    );
    std::fs::remove_file(&xml_path).ok();
}

#[test]
fn test_xml_virtual_frames_skipped() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-virtual");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test_virtual.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Frame name="VirtualTemplate" virtual="true"><Size x="200" y="100"/></Frame>
        <Frame name="ConcreteFrame" parent="UIParent" inherits="VirtualTemplate">
            <Anchors><Anchor point="CENTER"/></Anchors>
        </Frame>
    </Ui>"#,
    )
    .unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx =
        AddonContext::new(env.lua(), "TestAddon", addon_table, &temp_dir, false, false).unwrap();
    load_xml_file(
        &env.loader_env(),
        &xml_path,
        &ctx,
        &mut LoadTiming::default(),
    )
    .unwrap();

    assert!(
        !env.eval::<bool>("return VirtualTemplate ~= nil").unwrap(),
        "VirtualTemplate should NOT exist"
    );
    assert!(
        env.eval::<bool>("return ConcreteFrame ~= nil").unwrap(),
        "ConcreteFrame should exist"
    );
    std::fs::remove_file(&xml_path).ok();
}

#[test]
fn test_xml_multiple_anchors() {
    let t = load_test_xml(
        "test-multianchor",
        r#"<Ui>
            <Frame name="MultiAnchorFrame" parent="UIParent">
                <Anchors>
                    <Anchor point="TOPLEFT" x="10" y="-10"/>
                    <Anchor point="BOTTOMRIGHT" x="-10" y="10"/>
                </Anchors>
            </Frame>
        </Ui>"#,
    );

    assert_eq!(
        t.env
            .eval::<i32>("return MultiAnchorFrame:GetNumPoints()")
            .unwrap(),
        2
    );
    t.assert_lua_str(
        r#"
        local point, _, relPoint, x, y = MultiAnchorFrame:GetPoint(1)
        return string.format("%s,%s,%d,%d", point, relPoint, x, y)
    "#,
        "TOPLEFT,TOPLEFT,10,-10",
    );
    t.assert_lua_str(
        r#"
        local point, _, relPoint, x, y = MultiAnchorFrame:GetPoint(2)
        return string.format("%s,%s,%d,%d", point, relPoint, x, y)
    "#,
        "BOTTOMRIGHT,BOTTOMRIGHT,-10,10",
    );
}

#[test]
fn test_xml_set_all_points_keeps_explicit_anchors_authoritative() {
    let t = load_test_xml(
        "test-set-all-points-explicit-anchors",
        r#"<Ui>
            <Frame name="ExplicitAnchorParent" parent="UIParent">
                <Size x="400" y="300"/>
                <Anchors>
                    <Anchor point="CENTER"/>
                </Anchors>
                <Frames>
                    <Frame name="$parentInset" parentKey="Inset">
                        <Anchors>
                            <Anchor point="TOPLEFT" x="20" y="-30"/>
                            <Anchor point="BOTTOMRIGHT" x="-40" y="50"/>
                        </Anchors>
                    </Frame>
                    <Frame name="$parentChild" parentKey="Child" setAllPoints="true">
                        <Anchors>
                            <Anchor point="TOPLEFT" relativeTo="$parentInset" x="0" y="0"/>
                            <Anchor point="BOTTOMRIGHT" relativeTo="$parentInset" x="0" y="0"/>
                        </Anchors>
                    </Frame>
                </Frames>
            </Frame>
        </Ui>"#,
    );

    t.assert_lua_str(
        r#"
        local _, topLeftRelativeTo = ExplicitAnchorParent.Child:GetPoint(1)
        local _, bottomRightRelativeTo = ExplicitAnchorParent.Child:GetPoint(2)
        return tostring(topLeftRelativeTo == ExplicitAnchorParent.Inset)
            .. "|"
            .. tostring(bottomRightRelativeTo == ExplicitAnchorParent.Inset)
    "#,
        "true|true",
    );
}

#[test]
fn test_xml_all_script_handlers() {
    let t = load_test_xml(
        "test-allscripts",
        r#"<Ui>
            <Frame name="AllScriptsFrame" parent="UIParent">
                <Scripts>
                    <OnLoad>ONLOAD = true</OnLoad>
                    <OnEvent>ONEVENT = true</OnEvent>
                    <OnUpdate>ONUPDATE = true</OnUpdate>
                    <OnShow>ONSHOW = true</OnShow>
                    <OnHide>ONHIDE = true</OnHide>
                </Scripts>
            </Frame>
            <Button name="AllScriptsButton" parent="UIParent">
                <Scripts><OnClick>ONCLICK = true</OnClick></Scripts>
            </Button>
        </Ui>"#,
    );

    for handler in &["OnLoad", "OnEvent", "OnUpdate", "OnShow", "OnHide"] {
        t.assert_script_set("AllScriptsFrame", handler);
    }
    t.assert_script_set("AllScriptsButton", "OnClick");
}

#[test]
fn test_xml_intrinsic_onupdate_runs_during_update_tick() {
    let t = load_test_xml(
        "test-intrinsic-onupdate",
        r#"<Ui>
            <Frame name="IntrinsicOnUpdateFrame" parent="UIParent">
                <Scripts>
                    <OnUpdate intrinsicOrder="postcall">
                        INTRINSIC_ONUPDATE_COUNT = (INTRINSIC_ONUPDATE_COUNT or 0) + 1
                    </OnUpdate>
                </Scripts>
            </Frame>
        </Ui>"#,
    );

    t.env.fire_on_update(0.016).unwrap();
    t.assert_lua_str("return tostring(INTRINSIC_ONUPDATE_COUNT)", "1");
}
