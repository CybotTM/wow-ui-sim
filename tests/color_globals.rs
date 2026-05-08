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
