use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn color_globals_register_format_and_equality_methods() {
    let env = WowLuaEnv::new().unwrap();

    let methods_ok: bool = env
        .eval(
            r#"
            local color = NORMAL_FONT_COLOR
            return color:GenerateHexColor() == "ffffd100"
                and color:WrapTextInColorCode("Ready") == "|cffffd100Ready|r"
                and color:IsRGBEqualTo(NORMAL_FONT_COLOR)
                and color:IsEqualTo(NORMAL_FONT_COLOR)
            "#,
        )
        .unwrap();

    assert!(methods_ok);
}

#[test]
fn create_color_exposes_byte_channel_methods() {
    let env = WowLuaEnv::new().unwrap();

    let channels: (i64, i64, i64, i64, String) = env
        .eval(
            r#"
            local color = CreateColor(0.5, 0.25, 1.0, 0.1)
            local r, g, b = color:GetRGBAsBytes()
            local _, _, _, a = color:GetRGBAAsBytes()
            return r, g, b, a, color:GenerateHexColorNoAlpha()
            "#,
        )
        .unwrap();

    assert_eq!(channels, (128, 64, 255, 26, "8040FF".to_string()));
}

#[test]
fn rust_created_color_tables_expose_byte_channel_methods() {
    let env = WowLuaEnv::new().unwrap();

    let channels: (i64, i64, i64, i64, String) = env
        .eval(
            r#"
            local r, g, b = NORMAL_FONT_COLOR:GetRGBAsBytes()
            local _, _, _, a = NORMAL_FONT_COLOR:GetRGBAAsBytes()
            return r, g, b, a, NORMAL_FONT_COLOR:GenerateHexColorNoAlpha()
            "#,
        )
        .unwrap();

    assert_eq!(channels, (255, 209, 0, 255, "FFD100".to_string()));
}
