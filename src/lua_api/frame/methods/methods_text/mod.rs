//! Text/FontString methods: SetText, SetFont, SetJustifyH, etc.

mod decor;
mod font_object;
mod justify;
mod measure;

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::{get_sim_state, sync_child_to_lua};
use crate::lua_api::simple_html::TextStyle;
use crate::widget::WidgetType;
use mlua::{Lua, Value};

/// Known HTML text types for SimpleHTML per-textType methods.
pub(super) fn is_text_type(s: &str) -> bool {
    matches!(s, "h1" | "h2" | "h3" | "p")
}

/// Check if a frame ID corresponds to a SimpleHTML widget.
pub(super) fn is_simple_html(lua: &Lua, id: u64) -> bool {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .is_some_and(|f| f.widget_type == WidgetType::SimpleHTML)
}

/// Extract f32 from a reference to a Lua Value.
pub(super) fn val_to_f32(val: Option<&Value>, default: f32) -> f32 {
    match val {
        Some(Value::Number(n)) => *n as f32,
        Some(Value::Integer(n)) => *n as f32,
        _ => default,
    }
}

/// Extract f64 from a reference to a Lua Value.
pub(super) fn val_to_f64(val: Option<&Value>, default: f64) -> f64 {
    match val {
        Some(Value::Number(n)) => *n,
        Some(Value::Integer(n)) => *n as f64,
        _ => default,
    }
}

/// Add text/FontString methods to the frame methods table.
pub fn add_text_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_text_get_set_methods(methods);
    decor::add_decor_methods(methods);
    add_set_font_method(methods);
    add_get_font_method(methods);
    add_set_font_height_method(methods);
    add_get_font_height_method(methods);
    font_object::add_font_object_methods(methods);
    font_object::add_font_object_extra_methods(methods);
    add_text_color_methods(methods);
    justify::add_justification_methods(methods);
    measure::add_measure_methods(methods);
}

/// SetText, GetText, SetFormattedText.
fn add_text_get_set_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetText", |lua, this, args: mlua::MultiValue| {
        handle_set_text(lua, this.0, args)
    });
    methods.add_method("GetText", |lua, this, ()| get_text_impl(lua, this.0));
    methods.add_method("SetFormattedText", |lua, this, args: mlua::MultiValue| {
        set_formatted_text_impl(lua, this.0, args)
    });
}

/// GetText implementation: returns text or nil (EditBox returns "").
fn get_text_impl(lua: &Lua, id: u64) -> mlua::Result<Value> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let frame = state.widgets.get(id);
    let is_editbox = frame
        .map(|f| f.widget_type == WidgetType::EditBox)
        .unwrap_or(false);
    let text = frame.and_then(|f| get_text_for_widget(f, &state.widgets));
    match text {
        Some(t) => Ok(Value::String(lua.create_string(&t)?)),
        None if is_editbox => Ok(Value::String(lua.create_string("")?)),
        None => Ok(Value::Nil),
    }
}

/// Get text from widget, delegating to Text child for buttons.
fn get_text_for_widget(
    f: &crate::widget::Frame,
    widgets: &crate::widget::WidgetRegistry,
) -> Option<String> {
    if matches!(f.widget_type, WidgetType::Button | WidgetType::CheckButton) {
        f.children_keys
            .get("Text")
            .and_then(|&cid| widgets.get(cid))
            .and_then(|c| c.text.clone())
    } else {
        f.text.clone()
    }
}

/// SetFormattedText implementation.
fn set_formatted_text_impl(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let string_table: mlua::Table = lua.globals().get("string")?;
    let format_func: mlua::Function = string_table.get("format")?;
    if let Ok(Value::String(result)) = format_func.call::<Value>(args) {
        handle_set_text(
            lua,
            id,
            mlua::MultiValue::from_vec(vec![Value::String(result)]),
        )?;
    }
    Ok(())
}

/// Lazily create a "Text" FontString child for a Button/CheckButton.
///
/// WoW creates this child on first SetText call, not at button creation time.
/// The FontString fills the button (Overlay layer) so text renders above textures.
fn create_button_text_child(
    lua: &Lua,
    state: &mut crate::lua_api::SimState,
    button_id: u64,
) -> u64 {
    use crate::widget::Frame;
    let mut fs = Frame::new(WidgetType::FontString, None, Some(button_id));
    super::methods_helpers::set_all_points_anchors_pub(&mut fs, button_id);
    fs.draw_layer = crate::widget::DrawLayer::Overlay;
    if let Some(parent) = state.widgets.get(button_id) {
        fs.frame_strata = parent.frame_strata;
        fs.frame_level = parent.frame_level + 1;
    }
    let fs_id = fs.id;
    state.widgets.register(fs);
    state.widgets.add_child(button_id, fs_id);
    if let Some(btn) = state.widgets.get_mut_visual(button_id) {
        btn.children_keys.insert("Text".to_string(), fs_id);
    }
    let _ = sync_child_to_lua(lua, button_id, "Text", fs_id);
    fs_id
}

/// SetText(text [, r, g, b, wrap]) - universal handler for all widget types.
fn handle_set_text(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let mut args_iter = args.into_iter();
    let text_str = parse_text_arg(&mut args_iter);

    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();

    if let Some(ref text) = text_str {
        update_tooltip_line(&mut state, id, text, &mut args_iter);
    }

    let (text_child_id, is_html) = resolve_text_child(lua, &mut state, id, &text_str);
    let store_text = apply_html_strip(text_str, is_html);

    set_text_on_frame(&mut state, id, store_text.clone());
    if let Some(cid) = text_child_id {
        set_text_on_frame(&mut state, cid, store_text);
    }

    let ids_to_measure = collect_fontstring_measure_ids(&state, id, text_child_id);
    drop(state);

    measure_and_apply_sizes(lua, &state_rc, &ids_to_measure);
    Ok(())
}

/// Parse the text argument from a SetText arg iterator.
fn parse_text_arg(
    args_iter: &mut std::collections::vec_deque::IntoIter<mlua::Value>,
) -> Option<String> {
    match args_iter.next() {
        Some(mlua::Value::String(s)) => Some(s.to_string_lossy().to_string()),
        Some(mlua::Value::Integer(n)) => Some(n.to_string()),
        Some(mlua::Value::Number(n)) => Some(n.to_string()),
        _ => None,
    }
}

/// Resolve the text child ID and whether the frame is a SimpleHTML widget.
fn resolve_text_child(
    lua: &Lua,
    state: &mut std::cell::RefMut<'_, crate::lua_api::SimState>,
    id: u64,
    text_str: &Option<String>,
) -> (Option<u64>, bool) {
    let f = state.widgets.get(id);
    let child = f.and_then(|f| f.children_keys.get("Text").copied());
    let is_button = f
        .map(|f| {
            matches!(
                f.widget_type,
                crate::widget::WidgetType::Button | crate::widget::WidgetType::CheckButton
            )
        })
        .unwrap_or(false);
    let html = state.simple_htmls.contains_key(&id);
    let child = if child.is_none() && is_button && text_str.is_some() {
        Some(create_button_text_child(lua, state, id))
    } else {
        child
    };
    (child, html)
}

/// Strip HTML if the frame is a SimpleHTML widget.
fn apply_html_strip(text_str: Option<String>, is_html: bool) -> Option<String> {
    text_str.map(|t| {
        if is_html {
            super::widget_tooltip::strip_html_tags(&t)
        } else {
            t
        }
    })
}

/// Info needed to measure a FontString after text changes.
struct FontStringMeasureInfo {
    id: u64,
    text: String,
    font: Option<String>,
    font_size: f32,
    width: f32,
    width_is_text_auto: bool,
    word_wrap: bool,
}

/// Collect FontString IDs that need size measurement after text changes.
fn collect_fontstring_measure_ids(
    state: &std::cell::RefMut<'_, crate::lua_api::SimState>,
    id: u64,
    text_child_id: Option<u64>,
) -> Vec<FontStringMeasureInfo> {
    [Some(id), text_child_id]
        .into_iter()
        .flatten()
        .filter_map(|fid| {
            let f = state.widgets.get(fid)?;
            if f.widget_type != WidgetType::FontString {
                return None;
            }
            let text = f.text.as_ref()?.clone();
            Some(FontStringMeasureInfo {
                id: fid,
                text,
                font: f.font.clone(),
                font_size: f.font_size,
                width: f.width,
                width_is_text_auto: f.width_is_text_auto,
                word_wrap: f.word_wrap,
            })
        })
        .collect()
}

/// Measure text width and height, apply to frames, and invalidate layout.
fn measure_and_apply_sizes(
    lua: &Lua,
    state_rc: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    ids_to_measure: &[FontStringMeasureInfo],
) {
    if ids_to_measure.is_empty() {
        return;
    }
    let changed_ids = measure_with_font_system(lua, state_rc, ids_to_measure);
    if !changed_ids.is_empty() {
        let mut state = state_rc.borrow_mut();
        for id in changed_ids {
            state.widgets.mark_rect_dirty(id);
        }
    }
}

/// Run font measurements and apply size changes, returning IDs that changed.
fn measure_with_font_system(
    lua: &Lua,
    state_rc: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    ids_to_measure: &[FontStringMeasureInfo],
) -> Vec<u64> {
    use crate::render::font::WowFontSystem;
    let Some(fs_rc) = lua.app_data_ref::<std::rc::Rc<std::cell::RefCell<WowFontSystem>>>() else {
        return Vec::new();
    };
    let mut fs = fs_rc.borrow_mut();
    let mut state = state_rc.borrow_mut();
    let mut changed = Vec::new();
    for info in ids_to_measure {
        if apply_font_size_to_frame(&mut fs, &mut state, info) {
            changed.push(info.id);
        }
    }
    changed
}

/// Apply measured width/height to a single FontString frame. Returns true if changed.
fn apply_font_size_to_frame(
    fs: &mut crate::render::font::WowFontSystem,
    state: &mut crate::lua_api::SimState,
    info: &FontStringMeasureInfo,
) -> bool {
    let mut did_change = false;
    let width_is_explicit = info.word_wrap && info.width > 0.0 && !info.width_is_text_auto;
    if !width_is_explicit {
        let width = fs.measure_text_width(&info.text, info.font.as_deref(), info.font_size);
        if state
            .widgets
            .get(info.id)
            .map(|f| f.width != width)
            .unwrap_or(false)
        {
            if let Some(frame) = state.widgets.get_mut_visual(info.id) {
                frame.width = width;
                frame.width_is_text_auto = true;
            }
            did_change = true;
        }
    }
    let wrap_width = if width_is_explicit {
        Some(info.width)
    } else {
        None
    };
    let height =
        fs.measure_text_height(&info.text, info.font.as_deref(), info.font_size, wrap_width);
    if state
        .widgets
        .get(info.id)
        .map(|f| f.height != height)
        .unwrap_or(false)
    {
        if let Some(frame) = state.widgets.get_mut_visual(info.id) {
            frame.height = height;
        }
        did_change = true;
    }
    if did_change {
        state.widgets.mark_rect_dirty(info.id);
    }
    did_change
}

/// Update tooltip line data with optional r, g, b, wrap args.
fn update_tooltip_line(
    state: &mut std::cell::RefMut<'_, crate::lua_api::SimState>,
    id: u64,
    text: &str,
    args_iter: &mut std::collections::vec_deque::IntoIter<mlua::Value>,
) {
    if let Some(td) = state.tooltips.get_mut(&id) {
        let r = val_to_f32(args_iter.next().as_ref(), 1.0);
        let g = val_to_f32(args_iter.next().as_ref(), 1.0);
        let b = val_to_f32(args_iter.next().as_ref(), 1.0);
        let wrap = matches!(args_iter.next(), Some(mlua::Value::Boolean(true)));
        td.lines.clear();
        td.lines.push(crate::lua_api::tooltip::TooltipLine {
            left_text: text.to_string(),
            left_color: (r, g, b),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap,
            texture: None,
        });
    }
}

/// Set text on a frame. Size auto-sizing is handled by `measure_and_apply_sizes`.
fn set_text_on_frame(
    state: &mut std::cell::RefMut<'_, crate::lua_api::SimState>,
    id: u64,
    text: Option<String>,
) {
    if let Some(frame) = state.widgets.get(id) {
        if frame.text == text {
            return;
        }
    }
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.text_stripped = text.as_ref().map(|t| crate::render::strip_wow_markup(t));
        frame.text = text;
    }
}

/// SetFont([textType,] font, size, flags).
fn add_set_font_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFont", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let args_vec: Vec<Value> = args.into_iter().collect();
        let is_html = is_simple_html(lua, id);

        if is_html
            && args_vec.len() >= 2
            && let (Some(Value::String(s1)), Some(Value::String(s2))) =
                (args_vec.first(), args_vec.get(1))
        {
            let type_str = s1.to_string_lossy().to_string();
            if is_text_type(&type_str) {
                return set_font_for_text_type(lua, id, &type_str, s2, &args_vec);
            }
        }

        apply_set_font_standard(lua, id, &args_vec)
    });
}

/// Apply SetFont for a standard (non-SimpleHTML) frame.
fn apply_set_font_standard(lua: &Lua, id: u64, args_vec: &[Value]) -> mlua::Result<bool> {
    let font = match args_vec.first() {
        Some(Value::String(s)) => s.to_string_lossy().to_string(),
        _ => return Ok(true),
    };
    let size = match args_vec.get(1) {
        Some(Value::Number(n)) => Some(*n as f32),
        Some(Value::Integer(n)) => Some(*n as f32),
        _ => None,
    };
    let flags = match args_vec.get(2) {
        Some(Value::String(s)) => Some(s.to_string_lossy().to_string()),
        _ => None,
    };
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.font = Some(font);
        if let Some(s) = size {
            frame.font_size = s;
        }
        if let Some(ref f) = flags {
            frame.font_outline = crate::widget::TextOutline::from_wow_str(f);
        }
    }
    Ok(true)
}

/// Handle SetFont for a SimpleHTML per-textType call.
fn set_font_for_text_type(
    lua: &Lua,
    id: u64,
    type_str: &str,
    font_str: &mlua::String,
    args_vec: &[Value],
) -> mlua::Result<bool> {
    let font_path = font_str.to_string_lossy().to_string();
    let size = match args_vec.get(2) {
        Some(Value::Number(n)) => Some(*n as f32),
        Some(Value::Integer(n)) => Some(*n as f32),
        _ => None,
    };
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(data) = state.simple_htmls.get_mut(&id) {
        let style = data
            .text_styles
            .entry(type_str.to_string())
            .or_insert_with(TextStyle::default);
        style.font = Some(font_path);
        if let Some(s) = size {
            style.font_size = s;
        }
    }
    Ok(true)
}

/// GetFont([textType]).
fn add_get_font_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetFont", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let args_vec: Vec<Value> = args.into_iter().collect();

        if let Some(Value::String(s)) = args_vec.first() {
            let type_str = s.to_string_lossy().to_string();
            if is_text_type(&type_str) {
                return get_font_for_text_type(lua, id, &type_str);
            }
        }

        get_font_standard(lua, id)
    });
}

/// SetFontHeight(height) — sets the font size on a FontString or similar widget.
fn add_set_font_height_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFontHeight", |lua, this, height: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.font_size = height as f32;
        }
        Ok(())
    });
}

/// GetFontHeight() — returns the font size of a FontString or similar widget.
fn add_get_font_height_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetFontHeight", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let size = state
            .widgets
            .get(this.0)
            .map(|f| f.font_size)
            .unwrap_or(12.0);
        Ok(size as f64)
    });
}

/// GetFont for standard frames.
fn get_font_standard(lua: &Lua, id: u64) -> mlua::Result<mlua::MultiValue> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let frame = state.widgets.get(id);
    let font_path = frame
        .and_then(|f| f.font.as_deref())
        .unwrap_or("Fonts\\FRIZQT__.TTF");
    let font_size = frame.map(|f| f.font_size).unwrap_or(12.0);
    let flags = frame
        .map(|f| match f.font_outline {
            crate::widget::TextOutline::None => "",
            crate::widget::TextOutline::Outline => "OUTLINE",
            crate::widget::TextOutline::ThickOutline => "THICKOUTLINE",
        })
        .unwrap_or("");
    Ok(mlua::MultiValue::from_vec(vec![
        Value::String(lua.create_string(font_path)?),
        Value::Number(font_size as f64),
        Value::String(lua.create_string(flags)?),
    ]))
}

/// Handle GetFont for a SimpleHTML per-textType call.
fn get_font_for_text_type(lua: &Lua, id: u64, type_str: &str) -> mlua::Result<mlua::MultiValue> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    if let Some(data) = state.simple_htmls.get(&id)
        && let Some(style) = data.text_styles.get(type_str)
    {
        let font = style.font.as_deref().unwrap_or("Fonts\\FRIZQT__.TTF");
        return Ok(mlua::MultiValue::from_vec(vec![
            Value::String(lua.create_string(font)?),
            Value::Number(style.font_size as f64),
            Value::String(lua.create_string("")?),
        ]));
    }
    Ok(mlua::MultiValue::from_vec(vec![
        Value::String(lua.create_string("Fonts\\FRIZQT__.TTF")?),
        Value::Number(12.0),
        Value::String(lua.create_string("")?),
    ]))
}

/// Apply SetTextColor for SimpleHTML typed text styles.
fn set_text_color_html(lua: &Lua, id: u64, args: &[Value], type_str: String) {
    let r = val_to_f32(args.get(1), 1.0);
    let g = val_to_f32(args.get(2), 1.0);
    let b = val_to_f32(args.get(3), 1.0);
    let a = val_to_f32(args.get(4), 1.0);
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(data) = state.simple_htmls.get_mut(&id) {
        let style = data
            .text_styles
            .entry(type_str)
            .or_insert_with(TextStyle::default);
        style.text_color = (r, g, b, a);
    }
}

/// Apply SetTextColor for standard FontString/Frame widgets.
fn set_text_color_standard(lua: &Lua, id: u64, args: &[Value]) {
    let r = val_to_f32(args.first(), 1.0);
    let g = val_to_f32(args.get(1), 1.0);
    let b = val_to_f32(args.get(2), 1.0);
    let a = val_to_f32(args.get(3), 1.0);
    let new_color = crate::widget::Color::new(r, g, b, a);
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if !state
        .widgets
        .get(id)
        .is_some_and(|f| f.text_color == new_color)
    {
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.text_color = new_color;
        }
    }
}

/// SetTextColor, GetTextColor.
fn add_text_color_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetTextColor", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let args_vec: Vec<Value> = args.into_iter().collect();
        if is_simple_html(lua, id)
            && let Some(Value::String(s)) = args_vec.first()
        {
            let type_str = s.to_string_lossy().to_string();
            if is_text_type(&type_str) {
                set_text_color_html(lua, id, &args_vec, type_str);
                return Ok(());
            }
        }
        set_text_color_standard(lua, id, &args_vec);
        Ok(())
    });
    methods.add_method("GetTextColor", |lua, this, args: mlua::MultiValue| {
        get_text_color_impl(lua, this.0, args)
    });
}

/// GetTextColor implementation.
fn get_text_color_impl(
    lua: &Lua,
    id: u64,
    args: mlua::MultiValue,
) -> mlua::Result<(f32, f32, f32, f32)> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    if let Some(Value::String(s)) = args_vec.first() {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            return get_text_color_html(lua, id, &type_str);
        }
    }
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    if let Some(frame) = state.widgets.get(id) {
        Ok((
            frame.text_color.r,
            frame.text_color.g,
            frame.text_color.b,
            frame.text_color.a,
        ))
    } else {
        Ok((1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32))
    }
}

/// GetTextColor for a SimpleHTML text type.
fn get_text_color_html(lua: &Lua, id: u64, type_str: &str) -> mlua::Result<(f32, f32, f32, f32)> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    if let Some(data) = state.simple_htmls.get(&id)
        && let Some(style) = data.text_styles.get(type_str)
    {
        return Ok((
            style.text_color.0,
            style.text_color.1,
            style.text_color.2,
            style.text_color.3,
        ));
    }
    Ok((1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32))
}
