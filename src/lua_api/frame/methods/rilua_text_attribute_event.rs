//! rilua RustFn equivalents for methods_text/, methods_attribute.rs, and methods_event.rs.
//!
//! Each function is a `RustFn` (`fn(&mut LuaState) -> LuaResult<u32>`) that mirrors
//! the corresponding mlua method. Complex operations are stubbed with TODO.

use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, frame_id_from_stack,
    frame_ref, registry_table_or_create, table_get, table_set, val_to_string,
};
use crate::lua_api::rilua_script_helpers::{
    call_error_handler_state, get_script as get_rilua_script, remove_script as remove_rilua_script,
    set_script as set_rilua_script,
};
use crate::lua_api::state::SimState;
use crate::lua_bridge::{stack_val, table_set_rust_fn};
use crate::render::font::WowFontSystem;
use crate::widget::WidgetType;
use rilua::LuaApiMut;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val, runtime_error};

// ── Text methods ────────────────────────────────────────────────────────────

fn set_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text_val = stack_val(state, 2);
    let text = match text_val {
        Val::Str(s) => {
            let lua_str = state.gc.string_arena.get(s);
            lua_str.map(|ls| String::from_utf8_lossy(ls.data()).to_string())
        }
        Val::Num(n) => Some(n.to_string()),
        _ => None,
    };
    // TODO: button Text child creation, HTML stripping, font measurement, tooltip lines
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.text = text.clone();
        frame.text_stripped = text.as_ref().map(|t| crate::render::strip_wow_markup(t));
    }
    Ok(0)
}

fn get_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let frame = sim.widgets.get(id);
    let is_editbox = frame
        .map(|f| f.widget_type == crate::widget::WidgetType::EditBox)
        .unwrap_or(false);
    let text = frame.and_then(|f| frame_text_value(&sim, f, false));
    drop(sim);
    match text {
        Some(t) => {
            let s = create_string(state, &t);
            state.push(s);
            Ok(1)
        }
        None if is_editbox => {
            let s = create_string(state, "");
            state.push(s);
            Ok(1)
        }
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

fn clear_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.text = Some(String::new());
        frame.text_stripped = Some(String::new());
    }
    let text_child_id = sim
        .widgets
        .get(id)
        .and_then(|frame| frame.children_keys.get("Text").copied());
    if let Some(text_child_id) = text_child_id
        && let Some(child) = sim.widgets.get_mut_visual(text_child_id)
    {
        child.text = Some(String::new());
        child.text_stripped = Some(String::new());
    }
    Ok(0)
}

fn frame_text_value(
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
    let result = frame
        .map(|f| {
            let text = frame_text_value(&sim, f, true).unwrap_or_default();
            (text, f.font.clone(), f.font_size)
        })
        .unwrap_or_else(|| (String::new(), None, 12.0));
    drop(sim);
    result
}

fn frame_text_scale_value(state: &LuaState, id: u64) -> f64 {
    borrow_state(state)
        .expect("sim state should exist")
        .widgets
        .get(id)
        .map(|frame| frame.text_scale.max(0.0))
        .unwrap_or(1.0)
}

fn measure_text_width(state: &LuaState, id: u64) -> f64 {
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

    let mut fallback_font_system = WowFontSystem::new(std::path::Path::new("./fonts"));
    fallback_font_system.measure_text_width(&text, font.as_deref(), font_size) as f64 * text_scale
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

    let mut fallback_font_system = WowFontSystem::new(std::path::Path::new("./fonts"));
    fallback_font_system.measure_text_height(&text, font.as_deref(), font_size, wrap_width) as f64
        * text_scale
}

fn get_string_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    state.push(Val::Num(measure_text_width(state, id)));
    Ok(1)
}

fn get_string_height(state: &mut LuaState) -> LuaResult<u32> {
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

fn get_text_width(state: &mut LuaState) -> LuaResult<u32> {
    get_string_width(state)
}

fn get_line_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let line_height = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| (frame.font_size as f64 * frame.text_scale.max(0.0)) as f32)
            .unwrap_or(0.0)
    };
    state.push(Val::Num(line_height as f64));
    Ok(1)
}

fn is_truncated(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let (width, height, word_wrap, max_lines, line_height) = {
        let sim = borrow_state(state)?;
        let frame = sim.widgets.get(id);
        let width = frame.map(|f| f.width as f64).unwrap_or(0.0);
        let height = frame.map(|f| f.height as f64).unwrap_or(0.0);
        let word_wrap = frame.map(|f| f.word_wrap).unwrap_or(false);
        let max_lines = frame.map(|f| f.max_lines).unwrap_or(0);
        let line_height = frame
            .map(|frame| frame.font_size as f64 * frame.text_scale.max(0.0))
            .unwrap_or(0.0);
        (width, height, word_wrap, max_lines, line_height)
    };

    let width_overflow = width > 0.0 && measure_text_width(state, id) > width + 0.5;
    let vertical_overflow = if !word_wrap || width <= 0.0 {
        false
    } else {
        let wrapped_height = measure_text_height(state, id, Some(width as f32));
        let max_lines_height = if max_lines > 0 {
            Some(line_height * max_lines as f64)
        } else {
            None
        };
        let available_height = match (height > 0.0, max_lines_height) {
            (true, Some(lines_height)) => height.min(lines_height),
            (true, None) => height,
            (false, Some(lines_height)) => lines_height,
            (false, None) => 0.0,
        };
        available_height > 0.0 && wrapped_height > available_height + 0.5
    };
    let truncated = width_overflow || vertical_overflow;

    state.push(Val::Bool(truncated));
    Ok(1)
}

fn set_formatted_text(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: format the text with string.format and delegate to set_text logic
    let id = frame_id_from_stack(state, 1)?;
    let _ = id;
    Ok(0)
}

fn set_font(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: SimpleHTML per-textType dispatch
    let font_val = stack_val(state, 2);
    let size_val = stack_val(state, 3);
    let flags_val = stack_val(state, 4);
    let font = val_to_string(state, font_val);
    let size = match size_val {
        Val::Num(n) => Some(n as f32),
        _ => None,
    };
    let flags = val_to_string(state, flags_val);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
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
    drop(sim);
    state.push(Val::Bool(true));
    Ok(1)
}

fn get_font(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: SimpleHTML per-textType dispatch
    let sim = borrow_state(state)?;
    let frame = sim.widgets.get(id);
    let font_path = frame
        .and_then(|f| f.font.as_deref())
        .unwrap_or("Fonts\\FRIZQT__.TTF")
        .to_string();
    let font_size = frame.map(|f| f.font_size).unwrap_or(12.0);
    let flags = frame
        .map(|f| match f.font_outline {
            crate::widget::TextOutline::None => "",
            crate::widget::TextOutline::Outline => "OUTLINE",
            crate::widget::TextOutline::ThickOutline => "THICKOUTLINE",
        })
        .unwrap_or("")
        .to_string();
    drop(sim);
    let font_val = create_string(state, &font_path);
    let flags_val = create_string(state, &flags);
    state.push(font_val);
    state.push(Val::Num(font_size as f64));
    state.push(flags_val);
    Ok(3)
}

fn set_font_height(state: &mut LuaState) -> LuaResult<u32> {
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

fn set_text_height(state: &mut LuaState) -> LuaResult<u32> {
    set_font_height(state)
}

fn get_font_height(state: &mut LuaState) -> LuaResult<u32> {
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

fn set_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_object = stack_val(state, 2);
    let store = get_or_create_font_object_store(state);
    table_set(state, store, &id.to_string(), font_object);
    Ok(0)
}

fn get_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_font_object_store(state);
    let font_object = table_get(state, store, &id.to_string());
    state.push(font_object);
    Ok(1)
}

fn set_font_objects_to_try(state: &mut LuaState) -> LuaResult<u32> {
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

fn get_unbounded_string_width(state: &mut LuaState) -> LuaResult<u32> {
    get_string_width(state)
}

fn set_text_to_fit(state: &mut LuaState) -> LuaResult<u32> {
    set_text(state)
}

fn scale_text_to_fit(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn apply_default_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let default_text = val_to_string(state, stack_val(state, 2)).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.attributes.insert(
            "__default_text".to_string(),
            crate::widget::AttributeValue::String(default_text.clone()),
        );
        frame.attributes.insert(
            "__default_text_enabled".to_string(),
            crate::widget::AttributeValue::Boolean(true),
        );
        if frame.text.as_deref().unwrap_or_default().is_empty() {
            frame.text = Some(default_text);
            frame.attributes.insert(
                "__defaulted".to_string(),
                crate::widget::AttributeValue::Boolean(true),
            );
        }
    }
    Ok(0)
}

fn try_apply_default_text(state: &mut LuaState) -> LuaResult<u32> {
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
        frame.text = Some(default_text);
        frame.attributes.insert(
            "__defaulted".to_string(),
            crate::widget::AttributeValue::Boolean(true),
        );
    }
    Ok(0)
}

fn set_hyperlinks_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = matches!(stack_val(state, 2), Val::Bool(true));
    store_simple_attribute(state, id, "__hyperlinks_enabled", Val::Bool(enabled))?;
    Ok(0)
}

fn get_hyperlinks_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.attributes.get("__hyperlinks_enabled"))
        .is_some_and(|value| matches!(value, crate::widget::AttributeValue::Boolean(true)));
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn set_text_color(state: &mut LuaState) -> LuaResult<u32> {
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

fn get_text_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: SimpleHTML per-textType dispatch
    let sim = borrow_state(state)?;
    let (r, g, b, a) = if let Some(frame) = sim.widgets.get(id) {
        (
            frame.text_color.r,
            frame.text_color.g,
            frame.text_color.b,
            frame.text_color.a,
        )
    } else {
        (1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32)
    };
    drop(sim);
    state.push(Val::Num(r as f64));
    state.push(Val::Num(g as f64));
    state.push(Val::Num(b as f64));
    state.push(Val::Num(a as f64));
    Ok(4)
}

fn set_justify_h(state: &mut LuaState) -> LuaResult<u32> {
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

fn get_justify_h(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let justify = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| frame.justify_h.as_h_str())
            .unwrap_or("LEFT")
    };
    let justify = create_string(state, justify);
    state.push(justify);
    Ok(1)
}

fn set_justify_v(state: &mut LuaState) -> LuaResult<u32> {
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

fn get_justify_v(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let justify = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| frame.justify_v.as_v_str())
            .unwrap_or("TOP")
    };
    let justify = create_string(state, justify);
    state.push(justify);
    Ok(1)
}

fn set_word_wrap(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let word_wrap = matches!(stack_val(state, 2), Val::Bool(true));
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.word_wrap = word_wrap;
    }
    Ok(0)
}

fn set_max_lines(state: &mut LuaState) -> LuaResult<u32> {
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

fn get_max_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let max_lines = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.max_lines)
        .unwrap_or(0);
    state.push(Val::Num(max_lines as f64));
    Ok(1)
}

fn get_word_wrap(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let word_wrap = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.word_wrap)
        .unwrap_or(true);
    state.push(Val::Bool(word_wrap));
    Ok(1)
}

fn can_word_wrap(state: &mut LuaState) -> LuaResult<u32> {
    get_word_wrap(state)
}

fn set_non_space_wrap(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = matches!(stack_val(state, 2), Val::Bool(true));
    store_simple_attribute(state, id, "__non_space_wrap", Val::Bool(enabled))?;
    Ok(0)
}

fn can_non_space_wrap(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.attributes.get("__non_space_wrap"))
        .is_some_and(|value| matches!(value, crate::widget::AttributeValue::Boolean(true)));
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn get_text_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    state.push(Val::Num(frame_text_scale_value(state, id)));
    Ok(1)
}

fn set_text_scale(state: &mut LuaState) -> LuaResult<u32> {
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

// ── Attribute methods ───────────────────────────────────────────────────────

fn get_attribute(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let first = val_to_string(state, stack_val(state, 2));
    let second = val_to_string(state, stack_val(state, 3));
    let third = val_to_string(state, stack_val(state, 4));
    let keys = match (first, second, third) {
        (Some(name), None, None) => vec![name],
        (Some(prefix), Some(name), suffix) => {
            attribute_lookup_keys(&prefix, &name, suffix.as_deref().unwrap_or_default())
        }
        _ => {
            state.push(Val::Nil);
            return Ok(1);
        }
    };

    let attr = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|frame| {
            keys.iter()
                .find_map(|key| frame.attributes.get(key.as_str()).cloned())
        })
    };
    let val = attribute_to_val(state, attr.as_ref());
    state.push(val);
    Ok(1)
}

fn attribute_lookup_keys(prefix: &str, name: &str, suffix: &str) -> Vec<String> {
    let mut keys = Vec::with_capacity(5);
    keys.push(format!("{prefix}{name}{suffix}"));
    keys.push(format!("*{name}{suffix}"));
    keys.push(format!("{prefix}{name}*"));
    keys.push(format!("*{name}*"));
    keys.push(name.to_string());
    keys
}

fn set_attribute(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let name_val = stack_val(state, 2);
    let value = stack_val(state, 3);
    let Some(name) = val_to_string(state, name_val) else {
        return Ok(0);
    };
    if protected_write_blocked(state, id) {
        return Ok(0);
    }
    let name_arg = create_string(state, &name);
    store_simple_attribute(state, id, &name, value)?;
    if let Some(handler) = get_rilua_script(state, id, "OnAttributeChanged") {
        let frame = frame_ref(state, id)?;
        dispatch_attribute_changed(state, handler, frame, name_arg, value);
    }
    Ok(0)
}

/// True when the target frame is protected and the current call is
/// running under addon taint, matching Blizzard's "protected frames
/// reject insecure attribute writes" rule. When blocked, `SetAttribute`
/// / `SetAttributeNoHandler` silently skip the mutation — matching real
/// WoW, which drops the write and surfaces it through
/// `ADDON_ACTION_FORBIDDEN` instead of a Lua error. The event dispatch
/// is a follow-up; silent skip already gives insecure addons the
/// "nothing happened" outcome they'd see in-game.
fn protected_write_blocked(state: &mut LuaState, id: u64) -> bool {
    if rilua::api::state_is_secure(state) {
        return false;
    }
    let Ok(sim) = borrow_state(state) else {
        return false;
    };
    sim.widgets.get(id).is_some_and(|f| f.is_protected)
}

fn dispatch_attribute_changed(
    state: &mut LuaState,
    handler: Val,
    frame: Val,
    name: Val,
    value: Val,
) {
    let Ok(dispatcher) = state.load(
        r#"
        local handler, frame, name, value = ...
        handler(frame, name, value)
        "#,
    ) else {
        return;
    };

    let call_base = state.top;
    state.ensure_stack(call_base + 5);
    state.stack_set(call_base, Val::Function(dispatcher.gc_ref()));
    state.stack_set(call_base + 1, handler);
    state.stack_set(call_base + 2, frame);
    state.stack_set(call_base + 3, name);
    state.stack_set(call_base + 4, value);
    state.top = call_base + 5;

    if let Err(error) = state.call_function(call_base, 0) {
        call_error_handler_state(state, &error.to_string());
    }
    state.top = call_base;
}

fn set_attribute_no_handler(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let name_val = stack_val(state, 2);
    let value = stack_val(state, 3);
    let Some(name) = val_to_string(state, name_val) else {
        return Ok(0);
    };
    if protected_write_blocked(state, id) {
        return Ok(0);
    }
    store_simple_attribute(state, id, &name, value)?;
    Ok(0)
}

fn clear_attributes(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.attributes.clear();
    }
    Ok(0)
}

fn execute_attribute(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: full ExecuteAttribute semantics (function callback and snippet execution)
    let _id = frame_id_from_stack(state, 1)?;
    let reason = create_string(state, "attribute-missing");
    state.push(Val::Bool(false));
    state.push(reason);
    Ok(2)
}

fn set_frame_ref(state: &mut LuaState) -> LuaResult<u32> {
    let _id = frame_id_from_stack(state, 1)?;
    let label_val = stack_val(state, 2);
    let frame_val = stack_val(state, 3);
    let Some(label) = val_to_string(state, label_val) else {
        return Ok(0);
    };
    let key = format!("frameref-{}", label);
    // TODO: store frame refs in Lua-side attribute table to preserve Val identity
    // For now, only handle frame-backed table refs as Nil (ref itself is stored by caller)
    let _ = (frame_val, key);
    Ok(0)
}

fn get_frame_ref(state: &mut LuaState) -> LuaResult<u32> {
    let _id = frame_id_from_stack(state, 1)?;
    let _label_val = stack_val(state, 2);
    // TODO: retrieve from Lua-side attribute table
    state.push(Val::Nil);
    Ok(1)
}

fn set_forbidden(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: combat lockdown check
    let forbidden = match stack_val(state, 2) {
        Val::Nil => true,
        Val::Bool(b) => b,
        _ => true,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.forbidden = forbidden;
    }
    Ok(0)
}

fn is_forbidden(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim.widgets.get(id).map(|f| f.forbidden).unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(val));
    Ok(1)
}

fn can_change_protected_state(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: combat lockdown check
    state.push(Val::Bool(true));
    Ok(1)
}

fn set_pass_through_buttons(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: parse variadic button names from stack
    let _ = id;
    Ok(0)
}

fn set_flattens_render_layers(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let flatten = match stack_val(state, 2) {
        Val::Nil => false,
        Val::Bool(b) => b,
        _ => false,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.flattens_render_layers = flatten;
    }
    Ok(0)
}

fn set_motion_scripts_while_disabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = match stack_val(state, 2) {
        Val::Nil => false,
        Val::Bool(b) => b,
        _ => false,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.motion_scripts_while_disabled = enabled;
    }
    Ok(0)
}

fn get_motion_scripts_while_disabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.motion_scripts_while_disabled)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(val));
    Ok(1)
}

fn set_script(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let handler_name = val_to_string(state, stack_val(state, 2))
        .ok_or_else(|| runtime_error("SetScript: handler name required"))?;
    let handler = stack_val(state, 3);
    ensure_script_supported(state, frame_id, &handler_name)?;

    if matches!(handler, Val::Nil) {
        remove_rilua_script(state, frame_id, &handler_name);
    } else {
        if !matches!(handler, Val::Function(_)) {
            return Err(runtime_error(format!(
                "SetScript: handler for '{handler_name}' must be a function or nil"
            )));
        }
        set_rilua_script(state, frame_id, &handler_name, handler);
    }

    Ok(0)
}

fn get_script(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let handler_name = val_to_string(state, stack_val(state, 2))
        .ok_or_else(|| runtime_error("GetScript: handler name required"))?;
    let handler = get_rilua_script(state, frame_id, &handler_name).unwrap_or(Val::Nil);
    state.push(handler);
    Ok(1)
}

fn has_script(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let handler_name = val_to_string(state, stack_val(state, 2))
        .ok_or_else(|| runtime_error("HasScript: handler name required"))?;
    state.push(Val::Bool(script_supported(state, frame_id, &handler_name)));
    Ok(1)
}

fn hook_script(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let handler_name = val_to_string(state, stack_val(state, 2))
        .ok_or_else(|| runtime_error("HookScript: handler name required"))?;
    ensure_script_supported(state, frame_id, &handler_name)?;
    let hook = stack_val(state, 3);
    if !matches!(hook, Val::Function(_)) {
        return Err(runtime_error(format!(
            "HookScript: hook for '{handler_name}' must be a function"
        )));
    }
    let old = get_rilua_script(state, frame_id, &handler_name).unwrap_or(Val::Nil);
    let chained = build_hooked_script(state, old, hook)?;
    set_rilua_script(state, frame_id, &handler_name, chained);
    state.push(Val::Bool(true));
    Ok(1)
}

fn build_hooked_script(state: &mut LuaState, old: Val, hook: Val) -> LuaResult<Val> {
    let func = state.load(
        r#"
        local old, hook = ...
        if old == nil then
            return hook
        end
        return function(...)
            old(...)
            hook(...)
        end
    "#,
    )?;
    let call_base = state.top;
    state.ensure_stack(call_base + 4);
    state.stack_set(call_base, Val::Function(func.gc_ref()));
    state.stack_set(call_base + 1, old);
    state.stack_set(call_base + 2, hook);
    state.top = call_base + 3;
    state.call_function(call_base, 1)?;
    let result = state.stack_get(call_base);
    state.top = call_base;
    Ok(result)
}

fn ensure_script_supported(state: &LuaState, frame_id: u64, handler_name: &str) -> LuaResult<()> {
    if script_supported(state, frame_id, handler_name) {
        return Ok(());
    }
    Err(runtime_error(format!(
        "invalid script handler '{handler_name}'"
    )))
}

fn script_supported(state: &LuaState, frame_id: u64, handler_name: &str) -> bool {
    let Ok(sim) = borrow_state(state) else {
        return false;
    };
    let Some(widget_type) = sim.widgets.get(frame_id).map(|frame| frame.widget_type) else {
        return false;
    };
    script_supported_for_widget(widget_type, handler_name)
}

fn script_supported_for_widget(widget_type: WidgetType, handler_name: &str) -> bool {
    match handler_name {
        "OnLoad" | "OnEvent" | "OnUpdate" | "OnShow" | "OnHide" | "OnEnter" | "OnLeave"
        | "OnMouseDown" | "OnMouseUp" | "OnMouseWheel" | "OnDragStart" | "OnDragStop"
        | "OnReceiveDrag" | "OnSizeChanged" | "OnAttributeChanged" | "OnPlay" | "OnFinished"
        | "OnStop" | "OnLoop" | "OnPause" => true,
        "OnClick" | "PreClick" | "PostClick" => {
            matches!(widget_type, WidgetType::Button | WidgetType::CheckButton)
        }
        "OnEnable" | "OnDisable" => matches!(
            widget_type,
            WidgetType::Button
                | WidgetType::CheckButton
                | WidgetType::EditBox
                | WidgetType::Slider
                | WidgetType::ScrollFrame
        ),
        "OnEnterPressed" | "OnEscapePressed" | "OnTabPressed" | "OnSpacePressed" | "OnChar"
        | "OnKeyDown" | "OnKeyUp" => true,
        "OnTextChanged"
        | "OnTextSet"
        | "OnEditFocusGained"
        | "OnEditFocusLost"
        | "OnInputLanguageChanged" => matches!(widget_type, WidgetType::EditBox),
        "OnValueChanged" => matches!(widget_type, WidgetType::Slider | WidgetType::StatusBar),
        "OnVerticalScroll" | "OnScrollRangeChanged" => {
            matches!(widget_type, WidgetType::ScrollFrame | WidgetType::EditBox)
        }
        "OnColorSelect" => matches!(widget_type, WidgetType::ColorSelect),
        "OnHyperlinkClick" | "OnHyperlinkEnter" | "OnHyperlinkLeave" => true,
        _ => false,
    }
}

fn set_clips_children(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let clips = match stack_val(state, 2) {
        Val::Nil => false,
        Val::Bool(b) => b,
        _ => false,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.clips_children = clips;
    }
    Ok(0)
}

fn does_clip_children(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.clips_children)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(val));
    Ok(1)
}

fn set_hit_rect_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: combat lockdown check
    let l = val_to_f32(stack_val(state, 2), 0.0);
    let r = val_to_f32(stack_val(state, 3), 0.0);
    let t = val_to_f32(stack_val(state, 4), 0.0);
    let b = val_to_f32(stack_val(state, 5), 0.0);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.hit_rect_insets = (l, r, t, b);
    }
    Ok(0)
}

fn get_hit_rect_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (l, r, t, b) = sim
        .widgets
        .get(id)
        .map(|f| f.hit_rect_insets)
        .unwrap_or((0.0, 0.0, 0.0, 0.0));
    drop(sim);
    state.push(Val::Num(l as f64));
    state.push(Val::Num(r as f64));
    state.push(Val::Num(t as f64));
    state.push(Val::Num(b as f64));
    Ok(4)
}

// ── Event methods ───────────────────────────────────────────────────────────

fn register_event(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let event_val = stack_val(state, 2);
    let Some(event) = val_to_string(state, event_val) else {
        return Err(runtime_error("RegisterEvent: event name required"));
    };
    let newly_registered = {
        let mut sim = borrow_state_mut(state)?;
        if !crate::event::is_registerable_event(&event) {
            let frame_name = sim
                .widgets
                .get(id)
                .and_then(|f| f.name.clone())
                .unwrap_or_else(|| "Frame".to_string());
            return Err(runtime_error(format!(
                "{}:RegisterEvent(): {}:RegisterEvent(): Attempt to register unknown event \"{}\"",
                frame_name, frame_name, event
            )));
        }
        sim.widgets
            .get_mut(id)
            .map(|f| f.registered_events.insert(event.clone()))
            .unwrap_or(false)
    };
    if newly_registered {
        rilua_hlist_register_individual(state, id, &event)?;
    }
    let restricted = crate::event::is_restricted_event(&event);
    state.push(Val::Bool(newly_registered && !restricted));
    Ok(1)
}

fn register_unit_event(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let event_val = stack_val(state, 2);
    let Some(event) = val_to_string(state, event_val) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    // Unit args at 3+ are intentionally ignored (unit event filtering not implemented)
    let newly_registered = {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets
            .get_mut(id)
            .map(|f| f.registered_events.insert(event.clone()))
            .unwrap_or(false)
    };
    if newly_registered {
        rilua_hlist_register_individual(state, id, &event)?;
    }
    state.push(Val::Bool(newly_registered));
    Ok(1)
}

fn unregister_event(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let event_val = stack_val(state, 2);
    let Some(event) = val_to_string(state, event_val) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let was_registered = {
        let mut sim = borrow_state_mut(state)?;
        if !crate::event::is_registerable_event(&event) {
            let frame_name = sim
                .widgets
                .get(id)
                .and_then(|f| f.name.clone())
                .unwrap_or_else(|| "Frame".to_string());
            return Err(runtime_error(format!(
                "{}:RegisterEvent(): {}:RegisterEvent(): Attempt to register unknown event \"{}\"",
                frame_name, frame_name, event
            )));
        }
        sim.widgets
            .get_mut(id)
            .map(|f| f.registered_events.remove(&event))
            .unwrap_or(false)
    };
    if was_registered {
        rilua_hlist_unregister_individual(state, id, &event)?;
    }
    state.push(Val::Bool(was_registered));
    Ok(1)
}

fn unregister_all_events(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut(id) {
            frame.registered_events.clear();
            frame.register_all_events = false;
        }
    }
    rilua_hlist_unregister_all(state, id)?;
    Ok(0)
}

fn register_all_events(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut(id) {
            frame.register_all_events = true;
        }
    }
    rilua_hlist_register_all(state, id)?;
    Ok(0)
}

fn is_event_registered(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let event_val = stack_val(state, 2);
    let event = val_to_string(state, event_val).unwrap_or_default();
    let sim = borrow_state(state)?;
    let registered = sim
        .widgets
        .get(id)
        .map(|f| f.registered_events.contains(&event))
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(registered));
    state.push(Val::Nil);
    Ok(2)
}

fn register_event_callback(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let event_val = stack_val(state, 2);
    let Some(event) = val_to_string(state, event_val) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    if !crate::event::is_callback_event(&event) {
        return Err(runtime_error(format!(
            "Frame:RegisterEventCallback(): Attempt to register unknown event \"{}\"",
            event
        )));
    }
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut(id) {
        f.registered_events.insert(event.clone());
    }
    let restricted = crate::event::is_restricted_event(&event);
    drop(sim);
    state.push(Val::Bool(!restricted));
    Ok(1)
}

const FRAME_CALLBACKS_KEY: &str = "__callbacks";

fn callback_event_table(
    state: &mut LuaState,
    frame_id: u64,
    event: &str,
    create: bool,
) -> LuaResult<Option<GcRef<Table>>> {
    let frame = frame_ref(state, frame_id)?;
    let Val::Table(frame_ref) = frame else {
        return Ok(None);
    };

    let callbacks_key = state.gc.intern_string(FRAME_CALLBACKS_KEY.as_bytes());
    let callbacks = match state
        .gc
        .tables
        .get(frame_ref)
        .map(|t| t.get_str(callbacks_key, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
    {
        Val::Table(table_ref) => table_ref,
        _ if create => {
            let table_ref = state.gc.alloc_table(Table::new());
            if let Some(frame_table) = state.gc.tables.get_mut(frame_ref) {
                let _ = frame_table.raw_set(
                    Val::Str(callbacks_key),
                    Val::Table(table_ref),
                    &state.gc.string_arena,
                );
            }
            table_ref
        }
        _ => return Ok(None),
    };

    let event_key = state.gc.intern_string(event.as_bytes());
    let event_table = match state
        .gc
        .tables
        .get(callbacks)
        .map(|t| t.get_str(event_key, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
    {
        Val::Table(table_ref) => table_ref,
        _ if create => {
            let table_ref = state.gc.alloc_table(Table::new());
            if let Some(callbacks_table) = state.gc.tables.get_mut(callbacks) {
                let _ = callbacks_table.raw_set(
                    Val::Str(event_key),
                    Val::Table(table_ref),
                    &state.gc.string_arena,
                );
            }
            table_ref
        }
        _ => return Ok(None),
    };

    Ok(Some(event_table))
}

fn callback_entries(state: &LuaState, event_table: GcRef<Table>) -> Vec<Val> {
    state
        .gc
        .tables
        .get(event_table)
        .map(|table| table.array_slice().to_vec())
        .unwrap_or_default()
}

fn callback_entry_fields(state: &mut LuaState, entry: Val) -> Option<(Val, Val)> {
    let Val::Table(entry_ref) = entry else {
        return None;
    };
    let owner_key = state.gc.intern_string(b"owner");
    let func_key = state.gc.intern_string(b"func");
    let table = state.gc.tables.get(entry_ref)?;
    Some((
        table.get_str(owner_key, &state.gc.string_arena),
        table.get_str(func_key, &state.gc.string_arena),
    ))
}

fn rewrite_callback_entries(state: &mut LuaState, event_table: GcRef<Table>, entries: &[Val]) {
    let old_len = state
        .gc
        .tables
        .get(event_table)
        .map(|table| table.array_slice().len())
        .unwrap_or(0);
    let new_len = entries.len();
    let clear_to = old_len.max(new_len);

    if let Some(table) = state.gc.tables.get_mut(event_table) {
        for (index, entry) in entries.iter().copied().enumerate() {
            let _ = table.raw_set(Val::Num((index + 1) as f64), entry, &state.gc.string_arena);
        }
        for index in new_len..clear_to {
            let _ = table.raw_set(
                Val::Num((index + 1) as f64),
                Val::Nil,
                &state.gc.string_arena,
            );
        }
    }
}

fn register_callback(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let event = val_to_string(state, stack_val(state, 2)).ok_or_else(|| {
        runtime_error("CallbackRegistryMixin:RegisterCallback 'event' requires string type.")
    })?;
    let func = stack_val(state, 3);
    if !matches!(func, Val::Function(_)) {
        return Err(runtime_error(
            "CallbackRegistryMixin:RegisterCallback 'func' requires function type.",
        ));
    }

    let owner = match stack_val(state, 4) {
        Val::Nil => func,
        owner => owner,
    };

    if let Some(event_table) = callback_event_table(state, frame_id, &event, true)? {
        let mut entries = callback_entries(state, event_table)
            .into_iter()
            .filter(|entry| {
                callback_entry_fields(state, *entry)
                    .map(|(entry_owner, _)| entry_owner != owner)
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();

        let entry_ref = state.gc.alloc_table(Table::new());
        let owner_key = state.gc.intern_string(b"owner");
        let func_key = state.gc.intern_string(b"func");
        if let Some(entry_table) = state.gc.tables.get_mut(entry_ref) {
            let _ = entry_table.raw_set(Val::Str(owner_key), owner, &state.gc.string_arena);
            let _ = entry_table.raw_set(Val::Str(func_key), func, &state.gc.string_arena);
        }
        entries.push(Val::Table(entry_ref));
        rewrite_callback_entries(state, event_table, &entries);
    }

    state.push(owner);
    Ok(1)
}

fn unregister_callback(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let event = val_to_string(state, stack_val(state, 2)).ok_or_else(|| {
        runtime_error("CallbackRegistryMixin:UnregisterCallback 'event' requires string type.")
    })?;
    let owner = stack_val(state, 3);
    if matches!(owner, Val::Nil) {
        return Err(runtime_error(
            "CallbackRegistryMixin:UnregisterCallback 'owner' is required.",
        ));
    }

    if let Some(event_table) = callback_event_table(state, frame_id, &event, false)? {
        let entries = callback_entries(state, event_table)
            .into_iter()
            .filter(|entry| {
                callback_entry_fields(state, *entry)
                    .map(|(entry_owner, _)| entry_owner != owner)
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();
        rewrite_callback_entries(state, event_table, &entries);
    }

    Ok(0)
}

fn trigger_callback_event(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let event = val_to_string(state, stack_val(state, 2)).ok_or_else(|| {
        runtime_error("CallbackRegistryMixin:TriggerEvent 'event' requires string type.")
    })?;

    let arg_count = state.top.saturating_sub(state.base) as i32;
    let args: Vec<Val> = if arg_count >= 3 {
        (3..=arg_count).map(|idx| stack_val(state, idx)).collect()
    } else {
        Vec::new()
    };
    let callbacks = callback_event_table(state, frame_id, &event, false)?
        .map(|event_table| callback_entries(state, event_table))
        .unwrap_or_default();

    for entry in callbacks {
        let Some((owner, func)) = callback_entry_fields(state, entry) else {
            continue;
        };
        if matches!(func, Val::Nil) {
            continue;
        }
        let mut call_args = Vec::with_capacity(args.len() + 1);
        call_args.push(owner);
        call_args.extend(args.iter().copied());
        if let Err(error) = call_function_state(state, func, &call_args) {
            call_error_handler_state(state, &error.to_string());
        }
    }

    Ok(0)
}

fn register_unit_event_callback(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: full unit-event callback (unit filter + Lua callback storage)
    let id = frame_id_from_stack(state, 1)?;
    let event_val = stack_val(state, 2);
    let Some(event) = val_to_string(state, event_val) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut(id) {
        f.registered_events.insert(event.clone());
    }
    let restricted = crate::event::is_restricted_event(&event);
    drop(sim);
    rilua_hlist_register_individual(state, id, &event)?;
    state.push(Val::Bool(!restricted));
    Ok(1)
}

fn set_propagate_keyboard_input(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: combat lockdown check
    let propagate = match stack_val(state, 2) {
        Val::Bool(b) => b,
        _ => false,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut(id) {
        f.propagate_keyboard_input = propagate;
    }
    Ok(0)
}

fn get_propagate_keyboard_input(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.propagate_keyboard_input)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(val));
    Ok(1)
}

// ── hlist helpers (mirrors methods_event.rs) ────────────────────────────────

/// Insert `id` into the per-event hlist stored in registry["__event_individual"][event].
fn rilua_hlist_register_individual(state: &mut LuaState, id: u64, event: &str) -> LuaResult<()> {
    let individual_val = crate::lua_api::rilua_methods::registry_get(state, "__event_individual");
    let Val::Table(individual) = individual_val else {
        return Ok(());
    };
    let event_key = state.gc.intern_string(event.as_bytes());
    // Look up existing sub-table or create a new one
    let existing = state
        .gc
        .tables
        .get(individual)
        .map(|t| t.get_str(event_key, &state.gc.string_arena));
    let event_tbl = match existing {
        Some(Val::Table(t)) => t,
        _ => {
            let new_tbl = state.gc.alloc_table(Table::new());
            if let Some(t) = state.gc.tables.get_mut(individual) {
                let _ = t.raw_set(
                    Val::Str(event_key),
                    Val::Table(new_tbl),
                    &state.gc.string_arena,
                );
            }
            new_tbl
        }
    };
    rilua_hlist_insert(state, event_tbl, id)
}

/// Remove `id` from the per-event hlist.
fn rilua_hlist_unregister_individual(state: &mut LuaState, id: u64, event: &str) -> LuaResult<()> {
    let individual_val = crate::lua_api::rilua_methods::registry_get(state, "__event_individual");
    let Val::Table(individual) = individual_val else {
        return Ok(());
    };
    let event_key = state.gc.intern_string(event.as_bytes());
    let existing = state
        .gc
        .tables
        .get(individual)
        .map(|t| t.get_str(event_key, &state.gc.string_arena));
    if let Some(Val::Table(event_tbl)) = existing {
        rilua_hlist_remove(state, event_tbl, id)?;
    }
    Ok(())
}

/// Insert `id` into the all-events hlist stored in registry["__event_all"].
fn rilua_hlist_register_all(state: &mut LuaState, id: u64) -> LuaResult<()> {
    let all_val = crate::lua_api::rilua_methods::registry_get(state, "__event_all");
    if let Val::Table(all_tbl) = all_val {
        rilua_hlist_insert(state, all_tbl, id)?;
    }
    Ok(())
}

/// Remove `id` from all individual event hlists and the all-events hlist.
fn rilua_hlist_unregister_all(state: &mut LuaState, id: u64) -> LuaResult<()> {
    // Remove from all individual event tables
    let individual_val = crate::lua_api::rilua_methods::registry_get(state, "__event_individual");
    if let Val::Table(individual) = individual_val {
        // Collect all event sub-tables first to avoid borrow conflicts
        let sub_tables: Vec<GcRef<Table>> = state
            .gc
            .tables
            .get(individual)
            .map(|t| {
                t.hash_entries()
                    .into_iter()
                    .filter_map(|(_, v)| if let Val::Table(t) = v { Some(t) } else { None })
                    .collect()
            })
            .unwrap_or_default();
        for event_tbl in sub_tables {
            rilua_hlist_remove(state, event_tbl, id)?;
        }
    }
    // Remove from all-events hlist
    let all_val = crate::lua_api::rilua_methods::registry_get(state, "__event_all");
    if let Val::Table(all_tbl) = all_val {
        rilua_hlist_remove(state, all_tbl, id)?;
    }
    Ok(())
}

/// hlist insert: append id to array, record index in "_s" sub-table.
fn rilua_hlist_insert(state: &mut LuaState, tbl: GcRef<Table>, id: u64) -> LuaResult<()> {
    let set = rilua_hlist_set(state, tbl);
    // Check if already present (use integer key lookup to avoid string_arena borrow)
    let already = state
        .gc
        .tables
        .get(set)
        .map(|t| t.get_int(id as i64) != Val::Nil)
        .unwrap_or(false);
    if already {
        return Ok(());
    }
    let n = state.gc.tables.get(tbl).map(|t| t.array_len()).unwrap_or(0) + 1;
    if let Some(t) = state.gc.tables.get_mut(tbl) {
        let _ = t.raw_set(
            Val::Num(n as f64),
            Val::Num(id as f64),
            &state.gc.string_arena,
        );
    }
    if let Some(s) = state.gc.tables.get_mut(set) {
        let _ = s.raw_set(
            Val::Num(id as f64),
            Val::Num(n as f64),
            &state.gc.string_arena,
        );
    }
    Ok(())
}

/// hlist remove: swap-remove to keep array dense.
fn rilua_hlist_remove(state: &mut LuaState, tbl: GcRef<Table>, id: u64) -> LuaResult<()> {
    let set = rilua_hlist_set(state, tbl);
    // Use get_int to avoid string_arena borrow conflict
    let idx = state
        .gc
        .tables
        .get(set)
        .and_then(|t| match t.get_int(id as i64) {
            Val::Num(n) if n > 0.0 => Some(n as usize),
            _ => None,
        });
    let Some(idx) = idx else { return Ok(()) };
    let n = state.gc.tables.get(tbl).map(|t| t.array_len()).unwrap_or(0);
    if idx != n {
        let last_id = state
            .gc
            .tables
            .get(tbl)
            .and_then(|t| match t.get_int(n as i64) {
                Val::Num(lid) => Some(lid as u64),
                _ => None,
            });
        if let Some(lid) = last_id {
            if let Some(t) = state.gc.tables.get_mut(tbl) {
                let _ = t.raw_set(
                    Val::Num(idx as f64),
                    Val::Num(lid as f64),
                    &state.gc.string_arena,
                );
            }
            if let Some(s) = state.gc.tables.get_mut(set) {
                let _ = s.raw_set(
                    Val::Num(lid as f64),
                    Val::Num(idx as f64),
                    &state.gc.string_arena,
                );
            }
        }
    }
    if let Some(t) = state.gc.tables.get_mut(tbl) {
        let _ = t.raw_set(Val::Num(n as f64), Val::Nil, &state.gc.string_arena);
    }
    if let Some(s) = state.gc.tables.get_mut(set) {
        let _ = s.raw_set(Val::Num(id as f64), Val::Nil, &state.gc.string_arena);
    }
    Ok(())
}

/// Get or create the "_s" set sub-table of a hlist table.
fn rilua_hlist_set(state: &mut LuaState, tbl: GcRef<Table>) -> GcRef<Table> {
    let key_ref = state.gc.intern_string(b"_s");
    let existing = state
        .gc
        .tables
        .get(tbl)
        .map(|t| t.get_str(key_ref, &state.gc.string_arena));
    if let Some(Val::Table(s)) = existing {
        return s;
    }
    let new_set = state.gc.alloc_table(Table::new());
    if let Some(t) = state.gc.tables.get_mut(tbl) {
        let _ = t.raw_set(
            Val::Str(key_ref),
            Val::Table(new_set),
            &state.gc.string_arena,
        );
    }
    new_set
}

// ── Local helpers ───────────────────────────────────────────────────────────

fn val_to_f32(val: Val, default: f32) -> f32 {
    match val {
        Val::Num(n) => n as f32,
        _ => default,
    }
}

fn attribute_to_val(state: &mut LuaState, attr: Option<&crate::widget::AttributeValue>) -> Val {
    match attr {
        None => Val::Nil,
        Some(crate::widget::AttributeValue::Nil) => Val::Nil,
        Some(crate::widget::AttributeValue::Boolean(b)) => Val::Bool(*b),
        Some(crate::widget::AttributeValue::Number(n)) => Val::Num(*n),
        Some(crate::widget::AttributeValue::String(s)) => create_string(state, s),
    }
}

fn val_to_attribute(val: Val, state: &LuaState) -> crate::widget::AttributeValue {
    match val {
        Val::Nil => crate::widget::AttributeValue::Nil,
        Val::Bool(b) => crate::widget::AttributeValue::Boolean(b),
        Val::Num(n) => crate::widget::AttributeValue::Number(n),
        Val::Str(s) => {
            let text = state
                .gc
                .string_arena
                .get(s)
                .and_then(|ls| String::from_utf8(ls.data().to_vec()).ok())
                .unwrap_or_default();
            crate::widget::AttributeValue::String(text)
        }
        _ => crate::widget::AttributeValue::Nil,
    }
}

fn store_simple_attribute(state: &mut LuaState, id: u64, name: &str, value: Val) -> LuaResult<()> {
    let attr = val_to_attribute(value, state);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        if matches!(attr, crate::widget::AttributeValue::Nil) {
            frame.attributes.remove(name);
        } else {
            frame.attributes.insert(name.to_string(), attr);
        }
    }
    Ok(())
}

// ── Registration ────────────────────────────────────────────────────────────

/// Register all text, attribute, and event RustFn methods on the given table.
pub fn register_all(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    // Text methods
    table_set_rust_fn(state, table, "SetText", set_text)?;
    table_set_rust_fn(state, table, "GetText", get_text)?;
    table_set_rust_fn(state, table, "ClearText", clear_text)?;
    table_set_rust_fn(state, table, "SetFormattedText", set_formatted_text)?;
    table_set_rust_fn(state, table, "SetFont", set_font)?;
    table_set_rust_fn(state, table, "GetFont", get_font)?;
    table_set_rust_fn(state, table, "SetFontObject", set_font_object)?;
    table_set_rust_fn(state, table, "SetFontObjectsToTry", set_font_objects_to_try)?;
    table_set_rust_fn(state, table, "GetFontObject", get_font_object)?;
    table_set_rust_fn(state, table, "SetFontHeight", set_font_height)?;
    table_set_rust_fn(state, table, "SetTextHeight", set_text_height)?;
    table_set_rust_fn(state, table, "GetFontHeight", get_font_height)?;
    table_set_rust_fn(state, table, "GetStringWidth", get_string_width)?;
    table_set_rust_fn(state, table, "GetStringHeight", get_string_height)?;
    table_set_rust_fn(state, table, "GetTextWidth", get_text_width)?;
    table_set_rust_fn(state, table, "GetLineHeight", get_line_height)?;
    table_set_rust_fn(state, table, "IsTruncated", is_truncated)?;
    table_set_rust_fn(
        state,
        table,
        "GetUnboundedStringWidth",
        get_unbounded_string_width,
    )?;
    table_set_rust_fn(state, table, "SetJustifyH", set_justify_h)?;
    table_set_rust_fn(state, table, "GetJustifyH", get_justify_h)?;
    table_set_rust_fn(state, table, "SetJustifyV", set_justify_v)?;
    table_set_rust_fn(state, table, "GetJustifyV", get_justify_v)?;
    table_set_rust_fn(state, table, "SetWordWrap", set_word_wrap)?;
    table_set_rust_fn(state, table, "GetWordWrap", get_word_wrap)?;
    table_set_rust_fn(state, table, "CanWordWrap", can_word_wrap)?;
    table_set_rust_fn(state, table, "SetMaxLines", set_max_lines)?;
    table_set_rust_fn(state, table, "GetMaxLines", get_max_lines)?;
    table_set_rust_fn(state, table, "SetNonSpaceWrap", set_non_space_wrap)?;
    table_set_rust_fn(state, table, "CanNonSpaceWrap", can_non_space_wrap)?;
    table_set_rust_fn(state, table, "GetTextScale", get_text_scale)?;
    table_set_rust_fn(state, table, "SetTextScale", set_text_scale)?;
    table_set_rust_fn(state, table, "SetTextToFit", set_text_to_fit)?;
    table_set_rust_fn(state, table, "ScaleTextToFit", scale_text_to_fit)?;
    table_set_rust_fn(state, table, "ApplyDefaultText", apply_default_text)?;
    table_set_rust_fn(state, table, "TryApplyDefaultText", try_apply_default_text)?;
    table_set_rust_fn(state, table, "SetTextColor", set_text_color)?;
    table_set_rust_fn(state, table, "GetTextColor", get_text_color)?;
    table_set_rust_fn(state, table, "SetHyperlinksEnabled", set_hyperlinks_enabled)?;
    table_set_rust_fn(state, table, "GetHyperlinksEnabled", get_hyperlinks_enabled)?;
    // Attribute methods
    table_set_rust_fn(state, table, "GetAttribute", get_attribute)?;
    table_set_rust_fn(state, table, "SetAttribute", set_attribute)?;
    table_set_rust_fn(
        state,
        table,
        "SetAttributeNoHandler",
        set_attribute_no_handler,
    )?;
    table_set_rust_fn(state, table, "ClearAttributes", clear_attributes)?;
    table_set_rust_fn(state, table, "ExecuteAttribute", execute_attribute)?;
    table_set_rust_fn(state, table, "SetFrameRef", set_frame_ref)?;
    table_set_rust_fn(state, table, "GetFrameRef", get_frame_ref)?;
    table_set_rust_fn(state, table, "SetForbidden", set_forbidden)?;
    table_set_rust_fn(state, table, "IsForbidden", is_forbidden)?;
    table_set_rust_fn(
        state,
        table,
        "CanChangeProtectedState",
        can_change_protected_state,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetPassThroughButtons",
        set_pass_through_buttons,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetFlattensRenderLayers",
        set_flattens_render_layers,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetMotionScriptsWhileDisabled",
        set_motion_scripts_while_disabled,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetMotionScriptsWhileDisabled",
        get_motion_scripts_while_disabled,
    )?;
    table_set_rust_fn(state, table, "SetClipsChildren", set_clips_children)?;
    table_set_rust_fn(state, table, "DoesClipChildren", does_clip_children)?;
    table_set_rust_fn(state, table, "SetHitRectInsets", set_hit_rect_insets)?;
    table_set_rust_fn(state, table, "GetHitRectInsets", get_hit_rect_insets)?;
    // Event methods
    table_set_rust_fn(state, table, "RegisterEvent", register_event)?;
    table_set_rust_fn(state, table, "RegisterUnitEvent", register_unit_event)?;
    table_set_rust_fn(state, table, "UnregisterEvent", unregister_event)?;
    table_set_rust_fn(state, table, "UnregisterAllEvents", unregister_all_events)?;
    table_set_rust_fn(state, table, "RegisterAllEvents", register_all_events)?;
    table_set_rust_fn(state, table, "IsEventRegistered", is_event_registered)?;
    table_set_rust_fn(
        state,
        table,
        "RegisterEventCallback",
        register_event_callback,
    )?;
    table_set_rust_fn(state, table, "RegisterCallback", register_callback)?;
    table_set_rust_fn(state, table, "UnregisterCallback", unregister_callback)?;
    table_set_rust_fn(state, table, "TriggerEvent", trigger_callback_event)?;
    table_set_rust_fn(
        state,
        table,
        "RegisterUnitEventCallback",
        register_unit_event_callback,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetPropagateKeyboardInput",
        set_propagate_keyboard_input,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetPropagateKeyboardInput",
        get_propagate_keyboard_input,
    )?;
    table_set_rust_fn(state, table, "SetScript", set_script)?;
    table_set_rust_fn(state, table, "GetScript", get_script)?;
    table_set_rust_fn(state, table, "HasScript", has_script)?;
    table_set_rust_fn(state, table, "HookScript", hook_script)?;
    Ok(())
}
