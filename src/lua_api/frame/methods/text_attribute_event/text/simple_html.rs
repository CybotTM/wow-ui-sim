use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_set,
};
use crate::lua_api::simple_html::{SimpleHtmlData, TextStyle};
use crate::widget::WidgetType;
use rilua::Val;
use rilua::vm::state::LuaState;
use std::collections::HashMap;

#[derive(Clone)]
struct SimpleHtmlTextDataSnapshot {
    hyperlink_format: String,
    hyperlinks_enabled: bool,
    text_styles: HashMap<String, TextStyle>,
}

pub(super) fn is_simple_html_frame(state: &LuaState, id: u64) -> bool {
    borrow_state(state)
        .ok()
        .and_then(|sim| {
            sim.widgets
                .get(id)
                .map(|frame| frame.widget_type == WidgetType::SimpleHTML)
        })
        .unwrap_or(false)
}

pub(super) fn with_simple_html_data_mut<R>(
    state: &mut LuaState,
    id: u64,
    f: impl FnOnce(&mut SimpleHtmlData) -> R,
) -> Option<R> {
    if !is_simple_html_frame(state, id) {
        return None;
    }
    let mut sim = borrow_state_mut(state).ok()?;
    Some(f(sim.simple_htmls.entry(id).or_default()))
}

fn simple_html_style<'a>(data: &'a mut SimpleHtmlData, text_type: &str) -> &'a mut TextStyle {
    data.text_styles.entry(text_type.to_string()).or_default()
}

pub(super) fn get_simple_html_font(
    state: &mut LuaState,
    id: u64,
    text_type: String,
) -> Option<(String, f32, String)> {
    with_simple_html_data_mut(state, id, |data| {
        let style = simple_html_style(data, &text_type);
        let font = style
            .font
            .clone()
            .unwrap_or_else(|| "Fonts\\FRIZQT__.TTF".to_string());
        let flags = style.font_object.clone().unwrap_or_default();
        (font, style.font_size, flags)
    })
}

pub(super) fn set_simple_html_font(
    state: &mut LuaState,
    id: u64,
    text_type: String,
    font: Option<String>,
    size: Option<f32>,
    flags: Option<String>,
) {
    let _ = with_simple_html_data_mut(state, id, |data| {
        let style = simple_html_style(data, &text_type);
        if let Some(font) = font {
            style.font = Some(font);
        }
        if let Some(size) = size {
            style.font_size = size;
        }
        if let Some(flags) = flags {
            style.font_object = Some(flags);
        }
    });
}

pub(super) fn get_simple_html_text_color(
    state: &mut LuaState,
    id: u64,
    text_type: String,
) -> Option<(f32, f32, f32, f32)> {
    with_simple_html_data_mut(state, id, |data| {
        simple_html_style(data, &text_type).text_color
    })
}

pub(super) fn set_simple_html_text_color(
    state: &mut LuaState,
    id: u64,
    text_type: String,
    color: (f32, f32, f32, f32),
) {
    let _ = with_simple_html_data_mut(state, id, |data| {
        simple_html_style(data, &text_type).text_color = color;
    });
}

pub(super) fn build_simple_html_text_data(
    state: &mut LuaState,
    id: u64,
    text: Option<String>,
) -> Val {
    let Some(snapshot) = capture_simple_html_text_data(state, id) else {
        return Val::Nil;
    };

    build_simple_html_text_data_table(state, &snapshot, text)
}

fn build_simple_html_text_data_table(
    state: &mut LuaState,
    snapshot: &SimpleHtmlTextDataSnapshot,
    text: Option<String>,
) -> Val {
    let table = create_table(state);
    write_simple_html_text_data_fields(state, table, snapshot, text);
    let styles = build_simple_html_text_styles_table(state, &snapshot.text_styles);
    table_set(state, table, "textStyles", styles);
    table
}

fn capture_simple_html_text_data(
    state: &mut LuaState,
    id: u64,
) -> Option<SimpleHtmlTextDataSnapshot> {
    with_simple_html_data_mut(state, id, |data| SimpleHtmlTextDataSnapshot {
        hyperlink_format: data.hyperlink_format.clone(),
        hyperlinks_enabled: data.hyperlinks_enabled,
        text_styles: data.text_styles.clone(),
    })
}

fn write_simple_html_text_data_fields(
    state: &mut LuaState,
    table: Val,
    snapshot: &SimpleHtmlTextDataSnapshot,
    text: Option<String>,
) {
    let hyperlink_format = create_string(state, &snapshot.hyperlink_format);
    table_set(state, table, "hyperlinkFormat", hyperlink_format);
    table_set(
        state,
        table,
        "hyperlinksEnabled",
        Val::Bool(snapshot.hyperlinks_enabled),
    );
    if let Some(text) = text {
        let text_value = create_string(state, &text);
        table_set(state, table, "text", text_value);
    }
}

fn build_simple_html_text_styles_table(
    state: &mut LuaState,
    text_styles: &HashMap<String, TextStyle>,
) -> Val {
    let styles = create_table(state);
    for (text_type, style) in text_styles {
        let style_table = build_simple_html_style_table(state, style);
        table_set(state, styles, text_type.as_str(), style_table);
    }
    styles
}

fn build_simple_html_style_table(state: &mut LuaState, style: &TextStyle) -> Val {
    let style_table = create_table(state);
    let font_value = style
        .font
        .as_ref()
        .map(|font| create_string(state, font))
        .unwrap_or(Val::Nil);
    table_set(state, style_table, "font", font_value);
    table_set(
        state,
        style_table,
        "fontSize",
        Val::Num(style.font_size as f64),
    );
    style_table
}
