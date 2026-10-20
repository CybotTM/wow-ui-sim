use super::*;

#[test]
fn synthetic_xml_inheritance_needs_no_blizzard_templates() {
    let t = load_test_xml(
        "fixture-independent-xml",
        r#"<Ui>
            <Frame name="FixtureOnlyTemplate" virtual="true">
                <Size x="137" y="59"/>
                <Frames><Button parentKey="Action"/></Frames>
            </Frame>
            <Frame name="FixtureOnlyInstance" inherits="FixtureOnlyTemplate"/>
        </Ui>"#,
    );

    let dimensions: (f64, f64) = t.env.eval("return FixtureOnlyInstance:GetSize()").unwrap();
    assert_eq!(dimensions, (137.0, 59.0));
    t.assert_lua_true(
        "FixtureOnlyInstance.Action:GetParent() == FixtureOnlyInstance",
        "synthetic inherited child should belong to the instance",
    );
}

#[test]
fn synthetic_lua_addon_needs_no_blizzard_templates() {
    let (t, addon) = load_test_lua(
        "fixture-independent-lua",
        r#"
        local _, addon = ...
        local frame = CreateFrame("Frame", "FixtureLuaFrame", UIParent)
        frame:SetSize(137, 59)
        addon.width = frame:GetWidth()
        "#,
    );

    assert_eq!(val_to_i32(table_get(&t.env, addon, "width")), 137);
    t.assert_lua_true(
        "FixtureLuaFrame:GetParent() == UIParent",
        "synthetic Lua frame should keep its parent",
    );
}
