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
