use wow_ui_sim::xml::parse_xml;

#[test]
fn parse_xml_accepts_attribute_equals_spacing() {
    let xml = r#"<Ui xmlns="http://www.blizzard.com/wow/ui/">
<Frame name="SpacedEqualsFrame">
    <KeyValues>
        <KeyValue key="spellSlot" value ="0" type="number"/>
    </KeyValues>
</Frame>
</Ui>"#;

    let parsed = parse_xml(xml).expect("spaced attribute equals should parse");
    let frame = parsed.elements[0].as_frame_data().unwrap().0;
    let key_value = &frame.key_values().as_ref().unwrap().values[0];
    assert_eq!(key_value.value, "0");
}

#[test]
fn parse_xml_ignores_commented_key_values() {
    let xml = r#"<Ui xmlns="http://www.blizzard.com/wow/ui/">
<Frame name="CommentedKeyValuesFrame">
    <!--
    <KeyValues>
        <KeyValue key="layoutType" value="UniqueCornersLayout" type="string"/>
    </KeyValues>
    -->
</Frame>
</Ui>"#;

    parse_xml(xml).expect("commented KeyValues should not deserialize");
}

#[test]
fn parse_xml_allows_blank_numeric_attrs() {
    let xml = r#"<Ui xmlns="http://www.blizzard.com/wow/ui/">
<Frame name="BlankNumericAttrFrame">
    <Anchors>
        <Anchor point="TOPLEFT">
            <Offset>
                <AbsDimension x="-25" y=""/>
            </Offset>
        </Anchor>
    </Anchors>
</Frame>
</Ui>"#;

    let parsed = parse_xml(xml).expect("blank numeric attrs should parse as absent");
    let frame = parsed.elements[0].as_frame_data().unwrap().0;
    let anchor = &frame.anchors().as_ref().unwrap().anchors[0];
    let offset = anchor.offset.as_ref().unwrap();
    let abs = offset.abs_dimension.as_ref().unwrap();
    assert_eq!(abs.x, Some(-25.0));
    assert_eq!(abs.y, None);
}

#[test]
fn parse_xml_allows_key_value_without_value_attr() {
    let xml = r#"<Ui xmlns="http://www.blizzard.com/wow/ui/">
<Frame name="MissingKeyValueValueFrame">
    <KeyValues>
        <KeyValue key="editBoxHeaderText" type="global"/>
    </KeyValues>
</Frame>
</Ui>"#;

    let parsed = parse_xml(xml).expect("missing KeyValue value should default to empty");
    let frame = parsed.elements[0].as_frame_data().unwrap().0;
    let key_value = &frame.key_values().as_ref().unwrap().values[0];
    assert_eq!(key_value.key, "editBoxHeaderText");
    assert_eq!(key_value.value, "");
}
