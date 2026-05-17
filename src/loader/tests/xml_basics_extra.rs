use super::*;

#[test]
fn xml_process_time_excludes_script_file_load_time() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = tempfile::tempdir().unwrap();
    let xml_path = temp_dir.path().join("root.xml");
    let lua_path = temp_dir.path().join("large.lua");
    let lua_body = (0..2_000)
        .map(|index| format!("_G.__xml_timing_value = {index}\n"))
        .collect::<String>();

    std::fs::write(&xml_path, r#"<Ui><Script file="large.lua"/></Ui>"#).unwrap();
    std::fs::write(&lua_path, lua_body).unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx = AddonContext::new(
        env.lua(),
        "TestAddon",
        addon_table,
        temp_dir.path(),
        false,
        false,
    )
    .unwrap();
    let mut timing = LoadTiming::default();
    load_xml_file(&env.loader_env(), &xml_path, &ctx, &mut timing).unwrap();

    assert!(
        timing.lua_exec_time > timing.xml_process_time,
        "external <Script> Lua time should not be double-counted as XML process time: {timing:?}"
    );
}

#[test]
fn missing_xml_include_reports_path_to_lua_error_handler() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        __xml_errors = {}
        seterrorhandler(function(message)
            table.insert(__xml_errors, message)
        end)
        "#,
    )
    .unwrap();

    let temp_dir = tempfile::tempdir().unwrap();
    let xml_path = temp_dir.path().join("root.xml");
    let missing_path = temp_dir.path().join("missing.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Include file="missing.xml"/>
    </Ui>"#,
    )
    .unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx = AddonContext::new(
        env.lua(),
        "TestAddon",
        addon_table,
        temp_dir.path(),
        false,
        false,
    )
    .unwrap();
    let result = load_xml_file(
        &env.loader_env(),
        &xml_path,
        &ctx,
        &mut LoadTiming::default(),
    );

    assert!(
        result.is_err(),
        "missing XML include should fail the XML load"
    );

    let captured: String = env
        .eval("return table.concat(__xml_errors, [[\n---\n]])")
        .unwrap();
    assert!(
        captured.contains(&missing_path.display().to_string()),
        "Lua error should include missing include path, got: {captured}"
    );
    assert!(
        !captured
            .lines()
            .any(|line| line == "IO error: No such file or directory (os error 2)"),
        "Lua error should not be the old pathless IO message: {captured}"
    );
}

#[test]
fn fontstring_font_attribute_applies_font_object() {
    let t = load_test_xml(
        "fontstring-font-attribute",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="FontAttributeParent" parent="UIParent">
                <Layers>
                    <Layer level="OVERLAY">
                        <FontString parentKey="Text" font="GameFontNormal"/>
                    </Layer>
                </Layers>
            </Frame>
        </Ui>
        "#,
    );

    t.assert_lua_true(
        "return (function() local font = FontAttributeParent.Text:GetFont(); return font == [[Fonts\\FRIZQT__.TTF]] end)()",
        "FontString font attribute should apply the named font object",
    );
}

#[test]
fn slider_orientation_attribute_applies() {
    let t = load_test_xml(
        "slider-orientation-attribute",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Slider name="VerticalSlider" parent="UIParent" orientation="VERTICAL"/>
        </Ui>
        "#,
    );

    t.assert_lua_true(
        "return VerticalSlider:GetOrientation() == 'VERTICAL'",
        "Slider orientation attribute should seed GetOrientation",
    );
}

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
                        if INTRINSIC_ONUPDATE_ORDER then
                            table.insert(INTRINSIC_ONUPDATE_ORDER, "intrinsic")
                        end
                    </OnUpdate>
                </Scripts>
            </Frame>
        </Ui>"#,
    );

    t.assert_lua_true(
        "return IntrinsicOnUpdateFrame:GetScript('OnUpdate') == nil",
        "inline intrinsic OnUpdate should not occupy the normal script binding",
    );

    t.env.fire_on_update(0.016).unwrap();
    t.assert_lua_str("return tostring(INTRINSIC_ONUPDATE_COUNT)", "1");

    t.env
        .exec(
            r#"
            INTRINSIC_ONUPDATE_ORDER = {}
            IntrinsicOnUpdateFrame:SetScript("OnUpdate", function()
                table.insert(INTRINSIC_ONUPDATE_ORDER, "normal")
            end)
        "#,
        )
        .unwrap();

    t.env.fire_on_update(0.016).unwrap();
    t.assert_lua_str(
        "return table.concat(INTRINSIC_ONUPDATE_ORDER, ',')",
        "normal,intrinsic",
    );
}

#[test]
fn test_intrinsic_onupdate_uses_separate_binding_from_runtime_script() {
    let t = load_test_xml(
        "test-intrinsic-onupdate-bindings",
        r#"<Ui>
            <Frame name="IntrinsicOnUpdateBindingFrame" parent="UIParent">
                <Scripts>
                    <OnUpdate method="OnIntrinsicUpdate" intrinsicOrder="postcall"/>
                </Scripts>
            </Frame>
        </Ui>"#,
    );

    t.env
        .exec(
            r#"
            INTRINSIC_ONUPDATE_BINDING_ORDER = {}
            function IntrinsicOnUpdateBindingFrame:OnIntrinsicUpdate()
                table.insert(INTRINSIC_ONUPDATE_BINDING_ORDER, "intrinsic")
            end
        "#,
        )
        .unwrap();

    t.assert_lua_true(
        "return IntrinsicOnUpdateBindingFrame:GetScript('OnUpdate') == nil",
        "GetScript should only expose the normal script binding",
    );

    t.env.fire_on_update(0.016).unwrap();
    t.assert_lua_str(
        "return table.concat(INTRINSIC_ONUPDATE_BINDING_ORDER, ',')",
        "intrinsic",
    );

    t.env
        .exec(
            r#"
            INTRINSIC_ONUPDATE_BINDING_ORDER = {}
            IntrinsicOnUpdateBindingFrame:SetScript("OnUpdate", function()
                table.insert(INTRINSIC_ONUPDATE_BINDING_ORDER, "normal")
            end)
        "#,
        )
        .unwrap();
    t.env.fire_on_update(0.016).unwrap();
    t.assert_lua_str(
        "return table.concat(INTRINSIC_ONUPDATE_BINDING_ORDER, ',')",
        "normal,intrinsic",
    );

    t.env
        .exec(
            r#"
            INTRINSIC_ONUPDATE_BINDING_ORDER = {}
            IntrinsicOnUpdateBindingFrame:SetScript("OnUpdate", nil)
        "#,
        )
        .unwrap();
    t.env.fire_on_update(0.016).unwrap();
    t.assert_lua_str(
        "return table.concat(INTRINSIC_ONUPDATE_BINDING_ORDER, ',')",
        "intrinsic",
    );

    t.env
        .exec(
            r#"
            INTRINSIC_ONUPDATE_BINDING_ORDER = {}
            IntrinsicOnUpdateBindingFrame:HookScript("OnUpdate", function()
                table.insert(INTRINSIC_ONUPDATE_BINDING_ORDER, "hook")
            end)
        "#,
        )
        .unwrap();
    t.env.fire_on_update(0.016).unwrap();
    t.assert_lua_str(
        "return table.concat(INTRINSIC_ONUPDATE_BINDING_ORDER, ',')",
        "hook,intrinsic",
    );
}

#[test]
fn test_intrinsic_template_default_script_uses_precall_binding() {
    let t = load_test_xml(
        "test-intrinsic-template-default-bindings",
        r#"<Ui>
            <Frame name="IntrinsicDefaultBase" virtual="true" intrinsic="true">
                <Scripts>
                    <OnUpdate>
                        table.insert(INTRINSIC_TEMPLATE_ORDER, "intrinsic")
                    </OnUpdate>
                </Scripts>
            </Frame>
            <Frame name="IntrinsicDefaultDerived" parent="UIParent" inherits="IntrinsicDefaultBase">
                <Scripts>
                    <OnUpdate>
                        table.insert(INTRINSIC_TEMPLATE_ORDER, "normal")
                    </OnUpdate>
                </Scripts>
            </Frame>
        </Ui>"#,
    );

    t.env
        .exec("INTRINSIC_TEMPLATE_ORDER = {}")
        .expect("seed order table");
    t.assert_lua_true(
        "return IntrinsicDefaultDerived:GetScript('OnUpdate') ~= nil",
        "derived template script should occupy the normal binding",
    );
    t.assert_lua_true(
        "return IntrinsicDefaultDerived:GetScript('OnUpdate', 0) ~= nil",
        "intrinsic template default script should occupy the precall binding",
    );

    t.env.fire_on_update(0.016).unwrap();
    t.assert_lua_str(
        "return table.concat(INTRINSIC_TEMPLATE_ORDER, ',')",
        "intrinsic,normal",
    );
}

#[test]
fn test_xml_fontstring_zero_height_uses_text_height() {
    let t = load_test_xml(
        "test-xml-fontstring-zero-height",
        r#"<Ui>
            <Frame name="ZeroHeightFontStringParent" parent="UIParent">
                <Size x="300" y="32"/>
                <Layers>
                    <Layer level="ARTWORK">
                        <FontString name="ZeroHeightFontStringLabel" parentKey="Label" text="Visible label" inherits="GameFontHighlight">
                            <Size x="260" y="0"/>
                            <Anchors>
                                <Anchor point="LEFT"/>
                            </Anchors>
                        </FontString>
                    </Layer>
                </Layers>
            </Frame>
        </Ui>"#,
    );

    t.assert_lua_true(
        "return ZeroHeightFontStringParent.Label:GetHeight() > 0",
        "FontString XML labels with explicit zero height should use their measured text height",
    );
}
