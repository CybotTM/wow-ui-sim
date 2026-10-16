use super::prepare_stripped_text;
use crate::widget::WidgetType;

#[test]
fn prepare_stripped_text_uses_html_stripping_for_simple_html() {
    let stripped = prepare_stripped_text(
        WidgetType::SimpleHTML,
        Some("<p>Hello <b>World</b></p>".to_string()),
    );
    assert_eq!(stripped.as_deref(), Some("Hello World"));
}

#[test]
fn prepare_stripped_text_uses_wow_markup_stripping_for_font_strings() {
    let stripped = prepare_stripped_text(
        WidgetType::FontString,
        Some("|cff00ff00Hello|r".to_string()),
    );
    assert_eq!(stripped.as_deref(), Some("Hello"));
}

#[test]
fn stripped_text_preserves_nil_empty_unicode_and_embedded_nul() {
    for widget_type in [WidgetType::FontString, WidgetType::SimpleHTML] {
        assert_eq!(prepare_stripped_text(widget_type, None), None);
        assert_eq!(
            prepare_stripped_text(widget_type, Some(String::new())),
            Some(String::new())
        );
        let original = "|cff00ff00é猫|r\0tail".to_string();
        let stripped = prepare_stripped_text(widget_type, Some(original.clone()));
        assert_eq!(stripped.as_deref(), Some("é猫\0tail"));
        assert_eq!(original, "|cff00ff00é猫|r\0tail");
    }
}

#[test]
fn repeated_set_text_keeps_raw_and_stripped_text_after_style_changes() {
    let env = crate::lua_api::WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        PerfStageText = UIParent:CreateFontString('PerfStageText')
        local fs = PerfStageText
        local text = '|cff00ff00é猫|r'
        fs:SetText(text)
        fs:SetText(text)
        fs:SetWidth(40)
        fs:SetWordWrap(true)
        fs:SetFont('Fonts\\FRIZQT__.TTF', 18)
        fs:SetText(text)
        assert(fs:GetText() == text)
        assert(fs:GetWidth() == 40)
        local height = fs:GetStringHeight()
        fs:SetText(text)
        assert(fs:GetStringHeight() == height)
    "#,
    )
    .unwrap();
    let state = env.state();
    let sim = state.borrow();
    let frame = sim
        .widgets
        .get(sim.widgets.get_id_by_name("PerfStageText").unwrap())
        .unwrap();
    assert_eq!(frame.text.as_deref(), Some("|cff00ff00é猫|r"));
    assert_eq!(frame.text_stripped.as_deref(), Some("é猫"));
}
