//! Text-related RustFn methods: SetText, GetText, font, color, justification, wrapping.

mod formatting;
mod style;

pub(super) use formatting::{
    apply_default_text, get_font, get_font_height, get_font_object, get_unbounded_string_width,
    is_truncated, scale_text_to_fit, set_font, set_font_height, set_font_object,
    set_font_objects_to_try, set_formatted_text, set_text_height, set_text_to_fit,
    try_apply_default_text,
};
pub(super) use style::{
    can_non_space_wrap, can_word_wrap, get_hyperlink_format, get_hyperlinks_enabled,
    get_indented_word_wrap, get_justify_h, get_justify_v, get_max_lines, get_text_color,
    get_text_scale, get_word_wrap, set_hyperlink_format, set_hyperlinks_enabled,
    set_indented_word_wrap, set_justify_h, set_justify_v, set_max_lines, set_non_space_wrap,
    set_text_color, set_text_scale, set_word_wrap,
};

use super::helpers::val_to_f32;
use crate::font::WowFontSystem;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, frame_id_from_stack,
    get_or_create_frame_fields, table_set,
};
use crate::lua_api::simple_html::{SimpleHtmlData, TextStyle};
use crate::lua_api::state::SimState;
use crate::lua_bridge::stack_val;
use crate::widget::WidgetType;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::collections::HashMap;

use crate::lua_api::frame::methods::button_anchor_hierarchy::ensure_button_text_child;

#[derive(Copy, Clone)]
struct TooltipLineValues {
    r: Val,
    g: Val,
    b: Val,
    a: Val,
    wrap: Val,
}

#[derive(Clone)]
struct SimpleHtmlTextDataSnapshot {
    hyperlink_format: String,
    hyperlinks_enabled: bool,
    text_styles: HashMap<String, TextStyle>,
}

pub(super) fn set_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = read_text_arg(state, 2);
    let tooltip = read_tooltip_line_values(state);
    let stripped_text = stripped_text_for_frame(state, id, text.clone())?;
    let (is_tooltip, should_update_button_child) =
        update_text_frame(state, id, &text, &stripped_text)?;
    sync_button_text_child(state, id, &text, &stripped_text, should_update_button_child)?;
    refresh_text_measurements(state, id);
    if is_tooltip {
        sync_tooltip_text(state, id, text, tooltip)?;
    }
    Ok(0)
}

fn read_tooltip_line_values(state: &LuaState) -> TooltipLineValues {
    TooltipLineValues {
        r: stack_val(state, 3),
        g: stack_val(state, 4),
        b: stack_val(state, 5),
        a: stack_val(state, 6),
        wrap: stack_val(state, 7),
    }
}

fn is_simple_html_frame(state: &LuaState, id: u64) -> bool {
    borrow_state(state)
        .ok()
        .and_then(|sim| {
            sim.widgets
                .get(id)
                .map(|frame| frame.widget_type == WidgetType::SimpleHTML)
        })
        .unwrap_or(false)
}

fn with_simple_html_data_mut<R>(
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

fn get_simple_html_font(
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

fn set_simple_html_font(
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

fn get_simple_html_text_color(
    state: &mut LuaState,
    id: u64,
    text_type: String,
) -> Option<(f32, f32, f32, f32)> {
    with_simple_html_data_mut(state, id, |data| {
        simple_html_style(data, &text_type).text_color
    })
}

fn set_simple_html_text_color(
    state: &mut LuaState,
    id: u64,
    text_type: String,
    color: (f32, f32, f32, f32),
) {
    let _ = with_simple_html_data_mut(state, id, |data| {
        simple_html_style(data, &text_type).text_color = color;
    });
}

fn resolved_frame_width(state: &LuaState, id: u64) -> f32 {
    let Ok(sim) = borrow_state(state) else {
        return 0.0;
    };
    let mut cache = crate::layout::LayoutCache::default();
    crate::layout::compute_frame_rect_cached(
        &sim.widgets,
        id,
        sim.screen_width,
        sim.screen_height,
        &mut cache,
    )
    .rect
    .width
    .max(0.0)
}

fn build_simple_html_text_data(state: &mut LuaState, id: u64, text: Option<String>) -> Val {
    let Some(snapshot) = capture_simple_html_text_data(state, id) else {
        return Val::Nil;
    };

    let table = create_table(state);
    write_simple_html_text_data_fields(state, table, &snapshot, text);
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

fn strip_html_tags(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_tag = false;
    for ch in text.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(ch),
            _ => {}
        }
    }
    out
}

fn prepare_stripped_text(widget_type: WidgetType, text: Option<String>) -> Option<String> {
    text.map(|value| {
        if widget_type == WidgetType::SimpleHTML {
            strip_html_tags(&value)
        } else {
            crate::render::strip_wow_markup(&value)
        }
    })
}

fn stripped_text_for_frame(
    state: &LuaState,
    id: u64,
    text: Option<String>,
) -> LuaResult<Option<String>> {
    let widget_type = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.widget_type)
        .unwrap_or(WidgetType::FontString);
    Ok(prepare_stripped_text(widget_type, text))
}

fn update_text_frame(
    state: &mut LuaState,
    id: u64,
    text: &Option<String>,
    stripped_text: &Option<String>,
) -> LuaResult<(bool, bool)> {
    let mut sim = borrow_state_mut(state)?;
    let frame = sim.widgets.get(id);
    let current_text = frame.and_then(|frame| frame_text_value(&sim, frame, false));
    let current_stripped_text = frame.and_then(|frame| frame_text_value(&sim, frame, true));
    let is_tooltip = frame
        .map(|frame| frame.widget_type == WidgetType::GameTooltip)
        .unwrap_or(false);
    let is_button = matches!(
        frame.map(|frame| frame.widget_type),
        Some(WidgetType::Button | WidgetType::CheckButton)
    );
    let has_button_text_child = frame
        .and_then(|frame| frame.children_keys.get("Text"))
        .is_some();
    let changed = is_tooltip || current_text != *text || current_stripped_text != *stripped_text;
    if changed && let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.text = text.clone();
        frame.text_stripped = stripped_text.clone();
    }
    let should_update_button_child =
        is_button && (changed || (!has_button_text_child && text.is_some()));
    Ok((is_tooltip, should_update_button_child))
}

fn sync_button_text_child(
    state: &mut LuaState,
    id: u64,
    text: &Option<String>,
    stripped_text: &Option<String>,
    should_update_button_child: bool,
) -> LuaResult<()> {
    if !should_update_button_child {
        return Ok(());
    }
    let Some(text_child_id) = ensure_button_text_child(state, id)? else {
        return Ok(());
    };
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(text_child) = sim.widgets.get_mut_visual(text_child_id) {
            text_child.text = text.clone();
            text_child.text_stripped = stripped_text.clone();
        }
    }
    refresh_text_measurements(state, text_child_id);
    Ok(())
}

fn update_auto_text_height(state: &mut LuaState, id: u64) {
    let Some((is_fontstring, has_text, width, width_is_text_auto, word_wrap)) =
        (match borrow_state(state) {
            Ok(sim) => sim.widgets.get(id).map(|frame| {
                (
                    frame.widget_type == WidgetType::FontString,
                    frame.text.as_ref().is_some_and(|text| !text.is_empty()),
                    frame.width,
                    frame.width_is_text_auto,
                    frame.word_wrap,
                )
            }),
            Err(_) => return,
        })
    else {
        return;
    };
    if !is_fontstring || !has_text {
        return;
    }
    let width_is_explicit = word_wrap && width > 0.0 && !width_is_text_auto;
    let wrap_width = width_is_explicit.then_some(width);
    let height = measure_text_height(state, id, wrap_width) as f32;
    let Ok(mut sim) = borrow_state_mut(state) else {
        return;
    };
    let Some(frame) = sim.widgets.get(id) else {
        return;
    };
    if (frame.height - height).abs() <= 0.5 {
        return;
    }
    let Some(frame) = sim.widgets.get_mut_visual(id) else {
        return;
    };
    frame.height = height;
    sim.widgets.mark_rect_dirty(id);
}

fn update_auto_text_width(state: &mut LuaState, id: u64) {
    let should_update = {
        let Ok(sim) = borrow_state(state) else {
            return;
        };
        sim.widgets
            .get(id)
            .is_some_and(|frame| frame.width <= 0.0 || frame.width_is_text_auto)
    };
    if !should_update {
        return;
    }

    let width = measure_text_width(state, id) as f32;
    let Ok(mut sim) = borrow_state_mut(state) else {
        return;
    };
    let Some(frame) = sim.widgets.get_mut(id) else {
        return;
    };
    if frame.width > 0.0 && !frame.width_is_text_auto {
        return;
    }
    let width_changed = (frame.width - width).abs() > 0.5;
    let auto_flag_changed = !frame.width_is_text_auto;
    if !width_changed && !auto_flag_changed {
        return;
    }
    let Some(frame) = sim.widgets.get_mut_visual(id) else {
        return;
    };
    frame.width = width;
    frame.width_is_text_auto = true;
    sim.widgets.mark_rect_dirty(id);
}

fn refresh_text_measurements(state: &mut LuaState, id: u64) {
    update_auto_text_width(state, id);
    update_auto_text_height(state, id);
}

fn mirror_tooltip_text_fields(
    state: &mut LuaState,
    id: u64,
    text: Option<String>,
    tooltip: TooltipLineValues,
) {
    let fields = get_or_create_frame_fields(state, id);
    let text_val = text.map_or(Val::Nil, |value| create_string(state, &value));
    table_set(state, fields, "text", text_val);
    table_set(state, fields, "r", tooltip.r);
    table_set(state, fields, "g", tooltip.g);
    table_set(state, fields, "b", tooltip.b);
    table_set(state, fields, "a", tooltip.a);
    table_set(state, fields, "wrap", tooltip.wrap);

    let args = create_table(state);
    if let Val::Table(args_ref) = args {
        if let Some(table) = state.gc.tables.get_mut(args_ref) {
            let _ = table.raw_set(Val::Num(1.0), text_val, &state.gc.string_arena);
            let _ = table.raw_set(Val::Num(2.0), tooltip.r, &state.gc.string_arena);
            let _ = table.raw_set(Val::Num(3.0), tooltip.g, &state.gc.string_arena);
            let _ = table.raw_set(Val::Num(4.0), tooltip.b, &state.gc.string_arena);
            let _ = table.raw_set(Val::Num(5.0), tooltip.a, &state.gc.string_arena);
            let _ = table.raw_set(Val::Num(6.0), tooltip.wrap, &state.gc.string_arena);
        }
        state.gc.barrier_back(args_ref);
    }
    table_set(state, fields, "args", args);
}

fn replace_tooltip_lines(
    state: &mut LuaState,
    id: u64,
    text: Option<String>,
    tooltip: TooltipLineValues,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    let td = sim.tooltips.entry(id).or_default();
    td.lines.clear();
    if let Some(text) = text {
        td.lines.push(crate::lua_api::tooltip::TooltipLine {
            left_text: text,
            left_color: (
                val_to_f32(tooltip.r, 1.0),
                val_to_f32(tooltip.g, 1.0),
                val_to_f32(tooltip.b, 1.0),
            ),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap: matches!(tooltip.wrap, Val::Bool(true)),
            texture: None,
        });
    }
    td.spell_id = None;
    Ok(())
}

fn sync_tooltip_text(
    state: &mut LuaState,
    id: u64,
    text: Option<String>,
    tooltip: TooltipLineValues,
) -> LuaResult<()> {
    mirror_tooltip_text_fields(state, id, text.clone(), tooltip);
    replace_tooltip_lines(state, id, text, tooltip)
}

fn read_text_arg(state: &LuaState, index: i32) -> Option<String> {
    match stack_val(state, index) {
        Val::Str(s) => state
            .gc
            .string_arena
            .get(s)
            .map(|ls| String::from_utf8_lossy(ls.data()).to_string()),
        Val::Num(n) => Some(n.to_string()),
        _ => None,
    }
}

pub(super) fn get_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let frame = sim.widgets.get(id);
    let use_stripped = frame
        .map(|f| f.widget_type == WidgetType::SimpleHTML)
        .unwrap_or(false);
    let is_editbox = frame
        .map(|f| f.widget_type == WidgetType::EditBox)
        .unwrap_or(false);
    let text = frame.and_then(|f| frame_text_value(&sim, f, use_stripped));
    drop(sim);
    push_text_result(state, text, is_editbox)
}

fn push_text_result(
    state: &mut LuaState,
    text: Option<String>,
    is_editbox: bool,
) -> LuaResult<u32> {
    match text {
        Some(t) => {
            let s = create_string(state, &t);
            state.push(s);
        }
        None if is_editbox => {
            let s = create_string(state, "");
            state.push(s);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

pub(super) fn clear_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    clear_frame_text(&mut sim, id);
    let text_child_id = sim
        .widgets
        .get(id)
        .and_then(|frame| frame.children_keys.get("Text").copied());
    if let Some(child_id) = text_child_id {
        clear_frame_text(&mut sim, child_id);
    }
    Ok(0)
}

fn clear_frame_text(sim: &mut SimState, id: u64) {
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.text = Some(String::new());
        frame.text_stripped = Some(String::new());
    }
}

pub(super) fn frame_text_value(
    sim: &SimState,
    frame: &crate::widget::Frame,
    stripped: bool,
) -> Option<String> {
    let own_text = || {
        if stripped {
            frame.text_stripped.clone().or_else(|| frame.text.clone())
        } else {
            frame.text.clone()
        }
    };

    if !matches!(
        frame.widget_type,
        WidgetType::Button | WidgetType::CheckButton
    ) {
        return own_text();
    }

    frame
        .children_keys
        .get("Text")
        .and_then(|&cid| sim.widgets.get(cid))
        .and_then(|child| {
            if stripped {
                child.text_stripped.clone().or_else(|| child.text.clone())
            } else {
                child.text.clone()
            }
        })
        .or_else(own_text)
}

fn frame_text_measurement(state: &LuaState, id: u64) -> (String, Option<String>, f32) {
    let sim = borrow_state(state).expect("sim state should exist");
    let frame = sim.widgets.get(id);
    frame
        .map(|f| {
            let text = frame_text_value(&sim, f, true).unwrap_or_default();
            (text, f.font.clone(), f.font_size)
        })
        .unwrap_or_else(|| (String::new(), None, 12.0))
}

fn frame_text_scale_value(state: &LuaState, id: u64) -> f64 {
    borrow_state(state)
        .expect("sim state should exist")
        .widgets
        .get(id)
        .map(|frame| frame.text_scale.max(0.0))
        .unwrap_or(1.0)
}

pub(super) fn measure_text_width(state: &LuaState, id: u64) -> f64 {
    let (text, font, font_size) = frame_text_measurement(state, id);
    if text.is_empty() {
        return 0.0;
    }
    let text_scale = frame_text_scale_value(state, id);
    if let Some(app) = state.app_data::<crate::lua_api::env::WowLuaAppData>()
        && let Some(font_system) = app.font_system.as_ref()
    {
        return font_system
            .borrow_mut()
            .measure_text_width(&text, font.as_deref(), font_size) as f64
            * text_scale;
    }
    let mut fallback = WowFontSystem::new(std::path::Path::new("./fonts"));
    fallback.measure_text_width(&text, font.as_deref(), font_size) as f64 * text_scale
}

fn measure_text_height(state: &LuaState, id: u64, wrap_width: Option<f32>) -> f64 {
    let (text, font, font_size) = frame_text_measurement(state, id);
    if text.is_empty() {
        return 0.0;
    }
    let text_scale = frame_text_scale_value(state, id);
    if let Some(app) = state.app_data::<crate::lua_api::env::WowLuaAppData>()
        && let Some(font_system) = app.font_system.as_ref()
    {
        return font_system.borrow_mut().measure_text_height(
            &text,
            font.as_deref(),
            font_size,
            wrap_width,
        ) as f64
            * text_scale;
    }
    let mut fallback = WowFontSystem::new(std::path::Path::new("./fonts"));
    fallback.measure_text_height(&text, font.as_deref(), font_size, wrap_width) as f64 * text_scale
}

pub(super) fn get_string_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    state.push(Val::Num(measure_text_width(state, id)));
    Ok(1)
}

pub(super) fn get_string_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let wrap_width = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|frame| (frame.word_wrap && frame.width > 0.0).then_some(frame.width))
    };
    state.push(Val::Num(measure_text_height(state, id, wrap_width)));
    Ok(1)
}

pub(super) fn get_text_width(state: &mut LuaState) -> LuaResult<u32> {
    get_string_width(state)
}

pub(super) fn get_content_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let has_text = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|frame| frame_text_value(&sim, frame, true))
            .is_some_and(|text| !text.is_empty())
    };
    let height = if has_text {
        let wrap_width = resolved_frame_width(state, id);
        measure_text_height(state, id, (wrap_width > 0.0).then_some(wrap_width))
    } else {
        0.0
    };
    state.push(Val::Num(height));
    Ok(1)
}

pub(super) fn get_text_data(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|frame| frame.text.clone().or_else(|| frame.text_stripped.clone()))
    };
    let data = if is_simple_html_frame(state, id) {
        build_simple_html_text_data(state, id, text)
    } else {
        create_table(state)
    };
    state.push(data);
    Ok(1)
}

pub(super) fn get_line_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let height = sim
        .widgets
        .get(id)
        .map(|frame| (frame.font_size as f64 * frame.text_scale.max(0.0)) as f32)
        .unwrap_or(0.0);
    drop(sim);
    state.push(Val::Num(height as f64));
    Ok(1)
}

#[cfg(test)]
mod tests {
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
}
