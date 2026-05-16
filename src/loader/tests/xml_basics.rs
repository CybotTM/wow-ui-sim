use super::*;
#[path = "xml_basics_extra.rs"]
mod xml_basics_extra;

#[test]
fn test_hidden_xml_parent_does_not_hide_child_shown_flag() {
    let ctx = load_test_xml(
        "hidden-parent-child-shown",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="HiddenParent" parent="UIParent" hidden="true">
                <Frames>
                    <Frame name="HiddenParentChild"/>
                </Frames>
            </Frame>
        </Ui>
        "#,
    );

    ctx.assert_lua_true(
        "return HiddenParent ~= nil and HiddenParentChild ~= nil",
        "frames should exist",
    );
    ctx.assert_lua_true(
        "return HiddenParent:IsShown() == false",
        "parent should start hidden",
    );
    ctx.assert_lua_true(
        "return HiddenParentChild:IsShown() == true",
        "child should keep its own shown flag even when parent starts hidden",
    );
    ctx.assert_lua_true(
        "return HiddenParentChild:IsVisible() == false",
        "child should not be effectively visible while parent is hidden",
    );
}

#[test]
fn test_xml_font_redefinition_keeps_font_object_metatable_methods() {
    let ctx = load_test_xml(
        "font-object-redefinition-methods",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Font name="GameFontNormal" font="Fonts\FRIZQT__.TTF" height="12"/>
        </Ui>
        "#,
    );

    ctx.assert_lua_true(
        r#"
        (function()
            local mt = getmetatable(GameFontNormal)
            local index = mt and mt.__index
            GameFontNormal:SetShadowColor(0.1, 0.2, 0.3, 0.4)
            return type(index) == "table" and type(index.SetShadowColor) == "function"
                and type(GameFontNormal.SetShadowColor) == "function"
        end)()
        "#,
        "XML Font definitions should use the same Font object surface as CreateFont",
    );
}

#[test]
fn test_nested_xml_frame_parent_attribute_overrides_containing_frame() {
    let ctx = load_test_xml(
        "nested-explicit-parent",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="OuterParent" parent="UIParent">
                <Size x="300" y="200"/>
                <Anchors><Anchor point="TOPLEFT" x="10" y="-20"/></Anchors>
                <Frames>
                    <Frame name="ExplicitParent" parent="UIParent">
                        <Size x="100" y="80"/>
                        <Anchors><Anchor point="TOPLEFT" x="200" y="-100"/></Anchors>
                    </Frame>
                    <Frame name="NestedExplicitChild" parent="ExplicitParent">
                        <Size x="40" y="20"/>
                        <Anchors>
                            <Anchor point="BOTTOMRIGHT" relativePoint="TOPRIGHT" x="-6" y="-1"/>
                        </Anchors>
                    </Frame>
                </Frames>
            </Frame>
        </Ui>
        "#,
    );

    ctx.assert_lua_true(
        "return NestedExplicitChild:GetParent() == ExplicitParent",
        "nested frame parent attribute should override containing XML frame",
    );
    ctx.assert_lua_true(
        r#"
        (function()
            local point, relativeTo, relativePoint, x, y = NestedExplicitChild:GetPoint(1)
            return point == "BOTTOMRIGHT"
                and relativeTo == ExplicitParent
                and relativePoint == "TOPRIGHT"
                and x == -6
                and y == -1
        end)()
        "#,
        "implicit anchor target should be the explicit XML parent",
    );
}

#[test]
fn test_xml_frame_with_layers_and_scripts() {
    let t = load_test_xml(
        "test-xml",
        r#"<Ui>
            <Frame name="TestXMLFrame" parent="UIParent">
                <Size x="200" y="150"/>
                <Anchors><Anchor point="CENTER"/></Anchors>
                <Layers>
                    <Layer level="BACKGROUND">
                        <Texture name="TestXMLFrame_BG" parentKey="bg">
                            <Size x="200" y="150"/>
                            <Color r="0.1" g="0.1" b="0.1" a="0.8"/>
                            <Anchors>
                                <Anchor point="TOPLEFT"/>
                                <Anchor point="BOTTOMRIGHT"/>
                            </Anchors>
                        </Texture>
                    </Layer>
                    <Layer level="ARTWORK">
                        <FontString name="TestXMLFrame_Title" parentKey="title" text="Test Title">
                            <Anchors><Anchor point="TOP" y="-10"/></Anchors>
                        </FontString>
                    </Layer>
                </Layers>
                <Scripts><OnLoad>XML_ONLOAD_FIRED = true</OnLoad></Scripts>
                <Frames>
                    <Button name="TestXMLFrame_CloseBtn" parentKey="closeBtn">
                        <Size x="80" y="22"/>
                        <Anchors><Anchor point="BOTTOM" y="10"/></Anchors>
                        <Scripts>
                            <OnClick>XML_ONCLICK_FIRED = true</OnClick>
                            <OnDoubleClick>XML_ONDOUBLECLICK_FIRED = true</OnDoubleClick>
                        </Scripts>
                    </Button>
                </Frames>
            </Frame>
        </Ui>"#,
    );

    assert_layers_and_scripts_frame(&t);
    assert_layers_and_scripts_children(&t);
}

#[test]
fn test_xml_onload_fires_during_load() {
    let t = load_test_xml(
        "test-xml-onload-fired",
        r#"<Ui>
            <Frame name="OnLoadFrame" parent="UIParent">
                <Scripts><OnLoad>XML_ONLOAD_FIRED = true</OnLoad></Scripts>
            </Frame>
        </Ui>"#,
    );

    t.assert_lua_true(
        "return XML_ONLOAD_FIRED == true",
        "OnLoad should fire while the XML frame is finalized",
    );
}

#[test]
fn test_inherited_button_text_is_available_immediately_after_load() {
    let t = load_test_xml(
        "test-inherited-button-text",
        r#"<Ui>
            <Button name="InheritedButtonTextTemplate" virtual="true">
                <ButtonText name="$parentText"/>
            </Button>
            <Button name="InheritedButtonTextButton" parent="UIParent" inherits="InheritedButtonTextTemplate"/>
        </Ui>"#,
    );

    t.assert_lua_true(
        "return InheritedButtonTextButtonText ~= nil",
        "buttons inheriting ButtonText should create the inherited text child",
    );
    t.assert_lua_true(
        "return InheritedButtonTextButton:GetFontString() ~= nil",
        "buttons inheriting ButtonText should expose a font string after XML load",
    );
    t.assert_lua_true(
        "return InheritedButtonTextButton:GetFontString() == InheritedButtonTextButtonText",
        "buttons inheriting ButtonText should expose their own inherited text region immediately after XML load",
    );
}

#[test]
fn test_xml_onshow_only_fires_for_visible_frames() {
    let visible = load_test_xml(
        "test-xml-onshow-visible",
        r#"<Ui>
            <Frame name="VisibleOnShowFrame" parent="UIParent">
                <Scripts><OnShow>VISIBLE_ONSHOW_FIRED = true</OnShow></Scripts>
            </Frame>
        </Ui>"#,
    );
    visible.assert_lua_true(
        "return VISIBLE_ONSHOW_FIRED == true",
        "visible XML frame should fire OnShow during load",
    );

    let hidden = load_test_xml(
        "test-xml-onshow-hidden",
        r#"<Ui>
            <Frame name="HiddenOnShowFrame" parent="UIParent" hidden="true">
                <Scripts><OnShow>HIDDEN_ONSHOW_FIRED = true</OnShow></Scripts>
            </Frame>
        </Ui>"#,
    );
    hidden.assert_lua_true(
        "return HIDDEN_ONSHOW_FIRED == nil",
        "hidden XML frame should not fire OnShow during load",
    );
}

fn assert_layers_and_scripts_frame(t: &TestCtx) {
    t.assert_lua_true("return TestXMLFrame ~= nil", "TestXMLFrame should exist");
    t.assert_lua_true(
        "return TestXMLFrame.bg ~= nil",
        "bg should exist via parentKey",
    );
    t.assert_lua_true(
        "return TestXMLFrame.title ~= nil",
        "title should exist via parentKey",
    );
    t.assert_script_set("TestXMLFrame", "OnLoad");
}

fn assert_layers_and_scripts_children(t: &TestCtx) {
    t.assert_lua_true(
        "return TestXMLFrame_CloseBtn ~= nil",
        "CloseBtn should exist",
    );
    t.assert_lua_true(
        "return TestXMLFrame.closeBtn ~= nil",
        "closeBtn should exist via parentKey",
    );
    t.assert_script_set("TestXMLFrame_CloseBtn", "OnClick");
    t.assert_script_set("TestXMLFrame_CloseBtn", "OnDoubleClick");
}

#[test]
fn test_xml_button_texture_parent_key_assigns_button_field() {
    let t = load_test_xml(
        "test-xml-button-texture-parent-key",
        r#"<Ui>
            <Button name="ParentKeyButton" parent="UIParent">
                <Size x="48" y="48"/>
                <NormalTexture file="Interface\Buttons\UI-PaidCharacterCustomization-Button" parentKey="texture"/>
            </Button>
        </Ui>"#,
    );

    t.assert_lua_true(
        "return ParentKeyButton.texture ~= nil",
        "custom button texture parentKey should assign a Lua field on the button",
    );
    t.assert_lua_true(
        "return ParentKeyButton.texture == ParentKeyButton:GetNormalTexture()",
        "custom parentKey should reference the button's normal texture",
    );
}

#[test]
fn test_xml_scripts_function_attribute() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        SCRIPT_FUNC_CALLED = false
        function MyGlobalOnLoad(self) SCRIPT_FUNC_CALLED = true end
    "#,
    )
    .unwrap();

    let temp_dir = std::env::temp_dir().join("wow-sim-test-scripts");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test_func.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Frame name="FuncTestFrame" parent="UIParent">
            <Scripts><OnLoad function="MyGlobalOnLoad"/></Scripts>
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

    let handler_set: bool = env
        .eval("return FuncTestFrame:GetScript('OnLoad') == MyGlobalOnLoad")
        .unwrap();
    assert!(handler_set, "OnLoad should reference MyGlobalOnLoad");
    std::fs::remove_file(&xml_path).ok();
}

#[test]
fn fontstring_onload_function_fires_after_parent_key_assignment() {
    let t = load_test_xml(
        "fontstring-onload-function",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Script>
                function AliasFontString(fontString)
                    local parent = fontString:GetParent()
                    parent.text = fontString
                end
            </Script>
            <Frame name="FontStringOnLoadParent" parent="UIParent">
                <Layers>
                    <Layer level="ARTWORK">
                        <FontString name="$parentText" parentKey="Text">
                            <Scripts>
                                <OnLoad function="AliasFontString"/>
                            </Scripts>
                        </FontString>
                    </Layer>
                </Layers>
            </Frame>
        </Ui>
        "#,
    );

    t.assert_lua_true(
        "return FontStringOnLoadParent.Text ~= nil",
        "parentKey should wire FontString before OnLoad",
    );
    t.assert_lua_true(
        "return FontStringOnLoadParent.text == FontStringOnLoadParent.Text",
        "FontString OnLoad function should be called with the FontString",
    );
}

#[test]
fn test_xml_scripts_method_attribute() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-method");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test_method.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Frame name="MethodTestFrame" parent="UIParent">
            <Scripts><OnShow method="OnShowHandler"/></Scripts>
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

    env.exec(
        r#"
        METHOD_CALLED = false
        function MethodTestFrame:OnShowHandler() METHOD_CALLED = true end
    "#,
    )
    .unwrap();
    env.exec("MethodTestFrame:GetScript('OnShow')(MethodTestFrame)")
        .unwrap();

    let method_called: bool = env.eval("return METHOD_CALLED").unwrap();
    assert!(
        method_called,
        "OnShow should have called OnShowHandler method"
    );
    std::fs::remove_file(&xml_path).ok();
}

#[test]
fn test_xml_keyvalues() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-kv");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test_kv.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Frame name="KeyValueFrame" parent="UIParent">
            <KeyValues>
                <KeyValue key="myString" value="hello"/>
                <KeyValue key="myNumber" value="42" type="number"/>
                <KeyValue key="myBool" value="true" type="boolean"/>
                <KeyValue key="myFalseBool" value="false" type="boolean"/>
            </KeyValues>
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

    assert_eq!(
        env.eval::<String>("return KeyValueFrame.myString").unwrap(),
        "hello"
    );
    assert_eq!(
        env.eval::<i32>("return KeyValueFrame.myNumber").unwrap(),
        42
    );
    assert!(env.eval::<bool>("return KeyValueFrame.myBool").unwrap());
    assert!(
        !env.eval::<bool>("return KeyValueFrame.myFalseBool")
            .unwrap()
    );
    std::fs::remove_file(&xml_path).ok();
}

#[test]
fn test_xml_keyvalue_global_type_resolves_global_string() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-kv-global");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test_kv_global.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Frame name="KeyValueGlobalFrame" parent="UIParent">
            <KeyValues>
                <KeyValue key="instructionText" value="SEARCH" type="global"/>
            </KeyValues>
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

    let val: String = env
        .eval("return KeyValueGlobalFrame.instructionText")
        .unwrap();
    assert_eq!(
        val, "Search",
        "type='global' should resolve via global string lookup"
    );
    std::fs::remove_file(&xml_path).ok();
}

#[test]
fn test_xml_keyvalues_on_fontstring_and_texture() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-kv-children");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test_kv_children.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Frame name="KVChildFrame" parent="UIParent">
            <Layers>
                <Layer level="OVERLAY">
                    <FontString parentKey="Text" inherits="GameFontNormal">
                        <KeyValues>
                            <KeyValue key="anchorSpacing" value="4" type="number"/>
                            <KeyValue key="myTag" value="hello"/>
                        </KeyValues>
                    </FontString>
                    <Texture parentKey="Icon">
                        <KeyValues>
                            <KeyValue key="iconScale" value="1.5" type="number"/>
                        </KeyValues>
                    </Texture>
                </Layer>
            </Layers>
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

    assert_eq!(
        env.eval::<i32>("return KVChildFrame.Text.anchorSpacing")
            .unwrap(),
        4,
        "FontString KeyValue number"
    );
    assert_eq!(
        env.eval::<String>("return KVChildFrame.Text.myTag")
            .unwrap(),
        "hello",
        "FontString KeyValue string"
    );
    assert_eq!(
        env.eval::<f64>("return KVChildFrame.Icon.iconScale")
            .unwrap(),
        1.5,
        "Texture KeyValue number"
    );
    std::fs::remove_file(&xml_path).ok();
}

#[test]
fn test_layer_child_names_escape_backslashes_in_generated_lua() {
    let ctx = load_test_xml(
        "layer-child-name-escaping",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="Addon\CategoryFrame" parent="UIParent">
                <Layers>
                    <Layer level="ARTWORK">
                        <Texture name="$parentIcon"/>
                        <FontString name="$parentLabel"/>
                    </Layer>
                </Layers>
            </Frame>
        </Ui>
        "#,
    );

    ctx.assert_lua_true(
        r#"return _G["Addon\\CategoryFrameIcon"] ~= nil"#,
        "texture names with backslashes should be valid generated Lua strings",
    );
    ctx.assert_lua_true(
        r#"return _G["Addon\\CategoryFrameLabel"] ~= nil"#,
        "fontstring names with backslashes should be valid generated Lua strings",
    );
}

#[test]
fn test_xml_anchors_with_offset() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-offset");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test_offset.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Frame name="OffsetFrame" parent="UIParent">
            <Size x="100" y="100"/>
            <Anchors>
                <Anchor point="TOPLEFT">
                    <Offset><AbsDimension x="10" y="-20"/></Offset>
                </Anchor>
            </Anchors>
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

    let point_info: String = env
        .eval(
            r#"
        local point, relativeTo, relativePoint, x, y = OffsetFrame:GetPoint(1)
        return string.format("%s,%s,%d,%d", point, relativePoint, x, y)
    "#,
        )
        .unwrap();
    assert_eq!(point_info, "TOPLEFT,TOPLEFT,10,-20");
    std::fs::remove_file(&xml_path).ok();
}

#[test]
fn test_xml_anchors_with_direct_offset_attributes() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-direct-offset");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test_direct_offset.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Frame name="DirectOffsetFrame" parent="UIParent">
            <Size x="100" y="100"/>
            <Anchors>
                <Anchor point="BOTTOMLEFT" relativePoint="BOTTOMLEFT">
                    <Offset x="19" y="-30"/>
                </Anchor>
            </Anchors>
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

    let point_info: String = env
        .eval(
            r#"
        local point, relativeTo, relativePoint, x, y = DirectOffsetFrame:GetPoint(1)
        return string.format("%s,%s,%d,%d", point, relativePoint, x, y)
    "#,
        )
        .unwrap();
    assert_eq!(point_info, "BOTTOMLEFT,BOTTOMLEFT,19,-30");
    std::fs::remove_file(&xml_path).ok();
}

#[test]
fn test_xml_size_with_absdimension() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-abssize");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test_abssize.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
        <Frame name="AbsSizeFrame" parent="UIParent">
            <Size><AbsDimension x="150" y="75"/></Size>
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

    assert_eq!(
        env.eval::<f64>("return AbsSizeFrame:GetWidth()").unwrap(),
        150.0
    );
    assert_eq!(
        env.eval::<f64>("return AbsSizeFrame:GetHeight()").unwrap(),
        75.0
    );
    std::fs::remove_file(&xml_path).ok();
}

#[test]
fn test_xml_partial_size_preserves_single_dimension() {
    let t = load_test_xml(
        "test-partial-size",
        r#"<Ui>
            <Frame name="PartialSizeFrame" parent="UIParent">
                <Size x="8"/>
                <Anchors>
                    <Anchor point="TOP"/>
                    <Anchor point="BOTTOM"/>
                </Anchors>
            </Frame>
            <Frame name="PartialHeightFrame" parent="UIParent">
                <Size y="13"/>
                <Anchors>
                    <Anchor point="LEFT"/>
                    <Anchor point="RIGHT"/>
                </Anchors>
            </Frame>
        </Ui>"#,
    );

    assert_eq!(
        t.env
            .eval::<f64>("return PartialSizeFrame:GetWidth()")
            .unwrap(),
        8.0
    );
    assert_eq!(
        t.env
            .eval::<f64>("return PartialHeightFrame:GetHeight()")
            .unwrap(),
        13.0
    );
}

#[test]
fn test_xml_nested_child_frames() {
    let t = load_test_xml(
        "test-nested",
        r#"<Ui>
            <Frame name="ParentFrame" parent="UIParent">
                <Size x="300" y="200"/>
                <Frames>
                    <Frame name="ChildFrame" parentKey="child">
                        <Size x="100" y="50"/>
                        <Frames>
                            <Button name="GrandchildButton" parentKey="btn">
                                <Size x="80" y="22"/>
                            </Button>
                        </Frames>
                    </Frame>
                </Frames>
            </Frame>
        </Ui>"#,
    );

    assert_nested_frames_exist(&t);
    assert_nested_parent_relationships(&t);
}

fn assert_nested_frames_exist(t: &TestCtx) {
    t.assert_lua_true("return ParentFrame ~= nil", "ParentFrame should exist");
    t.assert_lua_true("return ChildFrame ~= nil", "ChildFrame should exist");
    t.assert_lua_true(
        "return ParentFrame.child == ChildFrame",
        "child should be ChildFrame",
    );
    t.assert_lua_true(
        "return GrandchildButton ~= nil",
        "GrandchildButton should exist",
    );
    t.assert_lua_true(
        "return ChildFrame.btn == GrandchildButton",
        "btn should be GrandchildButton",
    );
}

fn assert_nested_parent_relationships(t: &TestCtx) {
    t.assert_lua_str(
        "return ChildFrame:GetParent():GetName() or 'nil'",
        "ParentFrame",
    );
    t.assert_lua_str(
        "return GrandchildButton:GetParent():GetName() or 'nil'",
        "ChildFrame",
    );
}
