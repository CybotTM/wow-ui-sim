//! Text-related RustFn methods: SetText, GetText, font, color, justification, wrapping.

use super::helpers::{store_simple_attribute, val_to_f32};
use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, create_string, create_string_static, frame_id_from_stack,
    registry_table_or_create, table_get, table_set, val_to_string,
};
use crate::lua_api::state::SimState;
use crate::lua_bridge::stack_val;
use crate::render::font::WowFontSystem;
use crate::widget::WidgetType;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn set_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = read_text_arg(state, 2);
    // TODO: button Text child creation, HTML stripping, font measurement, tooltip lines
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.text = text.clone();
        frame.text_stripped = text.as_ref().map(|t| crate::render::strip_wow_markup(t));
    }
    Ok(0)
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
    let is_editbox = frame
        .map(|f| f.widget_type == WidgetType::EditBox)
        .unwrap_or(false);
    let text = frame.and_then(|f| frame_text_value(&sim, f, false));
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

pub(super) fn is_truncated(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let props = read_truncation_props(state, id);
    let truncated = check_truncated(state, id, props);
    state.push(Val::Bool(truncated));
    Ok(1)
}

struct TruncationProps {
    width: f64,
    height: f64,
    word_wrap: bool,
    max_lines: u32,
    line_height: f64,
}

fn read_truncation_props(state: &LuaState, id: u64) -> TruncationProps {
    let sim = borrow_state(state).expect("sim state should exist");
    let frame = sim.widgets.get(id);
    TruncationProps {
        width: frame.map(|f| f.width as f64).unwrap_or(0.0),
        height: frame.map(|f| f.height as f64).unwrap_or(0.0),
        word_wrap: frame.map(|f| f.word_wrap).unwrap_or(false),
        max_lines: frame.map(|f| f.max_lines).unwrap_or(0),
        line_height: frame
            .map(|f| f.font_size as f64 * f.text_scale.max(0.0))
            .unwrap_or(0.0),
    }
}

fn check_truncated(state: &LuaState, id: u64, p: TruncationProps) -> bool {
    let width_overflow = p.width > 0.0 && measure_text_width(state, id) > p.width + 0.5;
    let vertical_overflow = check_vertical_overflow(state, id, &p);
    width_overflow || vertical_overflow
}

fn check_vertical_overflow(state: &LuaState, id: u64, p: &TruncationProps) -> bool {
    if !p.word_wrap || p.width <= 0.0 {
        return false;
    }
    let wrapped_height = measure_text_height(state, id, Some(p.width as f32));
    let max_lines_height = (p.max_lines > 0).then_some(p.line_height * p.max_lines as f64);
    let available_height = match (p.height > 0.0, max_lines_height) {
        (true, Some(lh)) => p.height.min(lh),
        (true, None) => p.height,
        (false, Some(lh)) => lh,
        (false, None) => 0.0,
    };
    available_height > 0.0 && wrapped_height > available_height + 0.5
}

pub(super) fn set_formatted_text(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: format the text with string.format and delegate to set_text logic
    let id = frame_id_from_stack(state, 1)?;
    let _ = id;
    Ok(0)
}

pub(super) fn set_font(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: SimpleHTML per-textType dispatch
    let font = val_to_string(state, stack_val(state, 2));
    let size = match stack_val(state, 3) {
        Val::Num(n) => Some(n as f32),
        _ => None,
    };
    let flags = val_to_string(state, stack_val(state, 4));
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        apply_font_args(frame, font, size, flags);
    }
    drop(sim);
    state.push(Val::Bool(true));
    Ok(1)
}

fn apply_font_args(
    frame: &mut crate::widget::Frame,
    font: Option<String>,
    size: Option<f32>,
    flags: Option<String>,
) {
    if let Some(f) = font {
        frame.font = Some(f);
    }
    if let Some(s) = size {
        frame.font_size = s;
    }
    if let Some(ref f) = flags {
        frame.font_outline = crate::widget::TextOutline::from_wow_str(f);
    }
}

pub(super) fn get_font(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: SimpleHTML per-textType dispatch
    let sim = borrow_state(state)?;
    let frame = sim.widgets.get(id);
    let font_path = frame
        .and_then(|f| f.font.as_deref())
        .unwrap_or("Fonts\\FRIZQT__.TTF")
        .to_string();
    let font_size = frame.map(|f| f.font_size).unwrap_or(12.0);
    let flags = outline_to_str(frame).to_string();
    drop(sim);
    let font_path_val = create_string(state, &font_path);
    state.push(font_path_val);
    state.push(Val::Num(font_size as f64));
    let flags_val = create_string(state, &flags);
    state.push(flags_val);
    Ok(3)
}

fn outline_to_str(frame: Option<&crate::widget::Frame>) -> &'static str {
    frame
        .map(|f| match f.font_outline {
            crate::widget::TextOutline::None => "",
            crate::widget::TextOutline::Outline => "OUTLINE",
            crate::widget::TextOutline::ThickOutline => "THICKOUTLINE",
        })
        .unwrap_or("")
}

pub(super) fn set_font_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let height = match stack_val(state, 2) {
        Val::Num(n) => n as f32,
        _ => return Ok(0),
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.font_size = height;
    }
    Ok(0)
}

pub(super) fn set_text_height(state: &mut LuaState) -> LuaResult<u32> {
    set_font_height(state)
}

pub(super) fn get_font_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let size = sim.widgets.get(id).map(|f| f.font_size).unwrap_or(12.0);
    drop(sim);
    state.push(Val::Num(size as f64));
    Ok(1)
}

fn get_or_create_font_object_store(state: &mut LuaState) -> Val {
    registry_table_or_create(state, "__font_objects")
}

pub(super) fn set_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_object = stack_val(state, 2);
    let store = get_or_create_font_object_store(state);
    table_set(state, store, &id.to_string(), font_object);
    Ok(0)
}

pub(super) fn get_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_font_object_store(state);
    let font_object = table_get(state, store, &id.to_string());
    state.push(font_object);
    Ok(1)
}

pub(super) fn set_font_objects_to_try(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    for index in 2..=8 {
        let font_object = stack_val(state, index);
        if !matches!(font_object, Val::Nil) {
            let store = get_or_create_font_object_store(state);
            table_set(state, store, &id.to_string(), font_object);
            break;
        }
    }
    Ok(0)
}

pub(super) fn get_unbounded_string_width(state: &mut LuaState) -> LuaResult<u32> {
    get_string_width(state)
}

pub(super) fn set_text_to_fit(state: &mut LuaState) -> LuaResult<u32> {
    set_text(state)
}

pub(super) fn scale_text_to_fit(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub(super) fn apply_default_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let default_text = val_to_string(state, stack_val(state, 2)).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        store_default_text_attrs(frame, &default_text, true);
    }
    Ok(0)
}

fn store_default_text_attrs(
    frame: &mut crate::widget::Frame,
    default_text: &str,
    mark_enabled: bool,
) {
    use crate::widget::AttributeValue;
    frame.attributes.insert(
        "__default_text".to_string(),
        AttributeValue::String(default_text.to_string()),
    );
    if mark_enabled {
        frame.attributes.insert(
            "__default_text_enabled".to_string(),
            AttributeValue::Boolean(true),
        );
    }
    if frame.text.as_deref().unwrap_or_default().is_empty() {
        frame.text = Some(default_text.to_string());
        frame
            .attributes
            .insert("__defaulted".to_string(), AttributeValue::Boolean(true));
    }
}

pub(super) fn try_apply_default_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let default_text = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|frame| match frame.attributes.get("__default_text") {
                Some(crate::widget::AttributeValue::String(text)) => Some(text.clone()),
                _ => None,
            })
    };
    let Some(default_text) = default_text else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id)
        && frame.text.as_deref().unwrap_or_default().is_empty()
    {
        frame.text = Some(default_text.clone());
        frame.attributes.insert(
            "__defaulted".to_string(),
            crate::widget::AttributeValue::Boolean(true),
        );
    }
    Ok(0)
}

pub(super) fn set_hyperlinks_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = matches!(stack_val(state, 2), Val::Bool(true));
    store_simple_attribute(state, id, "__hyperlinks_enabled", Val::Bool(enabled))?;
    Ok(0)
}

pub(super) fn get_hyperlinks_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.attributes.get("__hyperlinks_enabled"))
        .is_some_and(|value| matches!(value, crate::widget::AttributeValue::Boolean(true)));
    state.push(Val::Bool(enabled));
    Ok(1)
}

pub(super) fn set_text_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: SimpleHTML per-textType dispatch
    let r = val_to_f32(stack_val(state, 2), 1.0);
    let g = val_to_f32(stack_val(state, 3), 1.0);
    let b = val_to_f32(stack_val(state, 4), 1.0);
    let a = val_to_f32(stack_val(state, 5), 1.0);
    let new_color = crate::widget::Color::new(r, g, b, a);
    let mut sim = borrow_state_mut(state)?;
    if !sim
        .widgets
        .get(id)
        .is_some_and(|f| f.text_color == new_color)
    {
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.text_color = new_color;
        }
    }
    Ok(0)
}

pub(super) fn get_text_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: SimpleHTML per-textType dispatch
    let sim = borrow_state(state)?;
    let (r, g, b, a) = sim
        .widgets
        .get(id)
        .map(|f| {
            (
                f.text_color.r,
                f.text_color.g,
                f.text_color.b,
                f.text_color.a,
            )
        })
        .unwrap_or((1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32));
    drop(sim);
    state.push(Val::Num(r as f64));
    state.push(Val::Num(g as f64));
    state.push(Val::Num(b as f64));
    state.push(Val::Num(a as f64));
    Ok(4)
}

pub(super) fn set_justify_h(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(justification) = val_to_string(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.justify_h = crate::widget::TextJustify::from_wow_str(&justification);
    }
    Ok(0)
}

pub(super) fn get_justify_h(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let justify = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| frame.justify_h.as_h_str())
            .unwrap_or("LEFT")
    };
    let justify_val = create_string_static(state, justify);
    state.push(justify_val);
    Ok(1)
}

pub(super) fn set_justify_v(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(justification) = val_to_string(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.justify_v = crate::widget::TextJustify::from_wow_str(&justification);
    }
    Ok(0)
}

pub(super) fn get_justify_v(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let justify = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| frame.justify_v.as_v_str())
            .unwrap_or("TOP")
    };
    let justify_val = create_string_static(state, justify);
    state.push(justify_val);
    Ok(1)
}

pub(super) fn set_word_wrap(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let word_wrap = matches!(stack_val(state, 2), Val::Bool(true));
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.word_wrap = word_wrap;
    }
    Ok(0)
}

pub(super) fn set_max_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let max_lines = match stack_val(state, 2) {
        Val::Num(value) if value >= 0.0 => value as u32,
        _ => 0,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.max_lines = max_lines;
    }
    Ok(0)
}

pub(super) fn get_max_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let max_lines = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.max_lines)
        .unwrap_or(0);
    state.push(Val::Num(max_lines as f64));
    Ok(1)
}

pub(super) fn get_word_wrap(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let word_wrap = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.word_wrap)
        .unwrap_or(true);
    state.push(Val::Bool(word_wrap));
    Ok(1)
}

pub(super) fn can_word_wrap(state: &mut LuaState) -> LuaResult<u32> {
    get_word_wrap(state)
}

pub(super) fn set_non_space_wrap(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = matches!(stack_val(state, 2), Val::Bool(true));
    store_simple_attribute(state, id, "__non_space_wrap", Val::Bool(enabled))?;
    Ok(0)
}

pub(super) fn can_non_space_wrap(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.attributes.get("__non_space_wrap"))
        .is_some_and(|value| matches!(value, crate::widget::AttributeValue::Boolean(true)));
    state.push(Val::Bool(enabled));
    Ok(1)
}

pub(super) fn get_text_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    state.push(Val::Num(frame_text_scale_value(state, id)));
    Ok(1)
}

pub(super) fn set_text_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text_scale = match stack_val(state, 2) {
        Val::Num(value) => value.max(0.0),
        _ => return Ok(0),
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.text_scale = text_scale;
    }
    Ok(0)
}
