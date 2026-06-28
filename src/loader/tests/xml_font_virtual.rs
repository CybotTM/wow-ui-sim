use super::*;

#[test]
fn test_virtual_xml_font_publishes_global_font_object() {
    let ctx = load_test_xml(
        "virtual-font-publishes-global",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Font name="VirtualXmlFontGlobalProbe" virtual="true" font="Fonts\FRIZQT__.TTF" height="17"/>
            <Frame name="VirtualXmlFontParent">
                <Layers>
                    <Layer level="ARTWORK">
                        <FontString name="VirtualXmlFontStringProbe" inherits="VirtualXmlFontGlobalProbe"/>
                    </Layer>
                </Layers>
            </Frame>
        </Ui>
        "#,
    );

    let probe: (String, String, String, bool, String, f64) = ctx
        .env
        .eval(
            r#"
            local font = VirtualXmlFontGlobalProbe
            local fontObject = VirtualXmlFontStringProbe:GetFontObject()
            local path, height = VirtualXmlFontStringProbe:GetFont()
            return type(font),
                font:GetObjectType(),
                font:GetName(),
                fontObject == font,
                path,
                height
            "#,
        )
        .expect("virtual font probe should execute");

    assert_eq!(
        probe,
        (
            "table".to_string(),
            "Font".to_string(),
            "VirtualXmlFontGlobalProbe".to_string(),
            true,
            "Fonts/FRIZQT__.TTF".to_string(),
            17.0,
        ),
        "retail 12.0.7 publishes virtual XML Font globals and uses them for FontString inheritance",
    );
}
