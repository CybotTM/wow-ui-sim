use super::*;

#[test]
fn test_xml_button_normal_texture_keeps_tex_coords() {
    let t = load_test_xml(
        "test-xml-button-normal-texture-texcoords",
        r#"<Ui>
            <Button name="ButtonTextureTexCoordsButton" parent="UIParent">
                <Size x="140" y="20"/>
                <NormalTexture setAllPoints="true" file="Interface\GuildFrame\GuildFrame">
                    <TexCoords left="0.36230469" right="0.38183594" top="0.95898438" bottom="0.99804688"/>
                </NormalTexture>
            </Button>
        </Ui>"#,
    );

    t.assert_lua_true(
        r#"
        return (function()
            local tex = ButtonTextureTexCoordsButton:GetNormalTexture()
            if not tex then return false end
            local left, right, top, bottom = tex:GetTexCoord()
            return math.abs(left - 0.36230469) < 0.00001
                and math.abs(right - 0.38183594) < 0.00001
                and math.abs(top - 0.95898438) < 0.00001
                and math.abs(bottom - 0.99804688) < 0.00001
        end)()
        "#,
        "XML NormalTexture TexCoords should be applied to the generated button texture child",
    );
}
