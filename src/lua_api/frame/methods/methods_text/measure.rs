//! Text measurement, word wrap, text scale, and spacing methods.

use super::super::super::handle::{FrameRef, get_sim_state};
use super::{is_simple_html, is_text_type, val_to_f64};
use crate::lua_api::simple_html::TextStyle;
use crate::render::font::WowFontSystem;
use mlua::{Lua, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Add measurement, word wrap, text scale, and spacing methods.
pub fn add_measure_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_text_measurement_methods(methods);
    add_text_height_methods(methods);
    add_word_wrap_methods(methods);
    add_text_scale_methods(methods);
    add_spacing_methods(methods);
}

/// GetStringWidth, GetTextWidth, GetUnboundedStringWidth, GetStringHeight, GetLineHeight.
fn add_text_measurement_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetStringWidth", |lua, this, ()| {
        measure_text_width(lua, this.0)
    });
    methods.add_method("GetTextWidth", |lua, this, ()| {
        measure_text_width(lua, this.0)
    });
    methods.add_method("GetUnboundedStringWidth", |lua, this, ()| {
        measure_text_width(lua, this.0)
    });
    methods.add_method("GetStringHeight", |lua, this, ()| {
        measure_string_height(lua, this.0)
    });
    methods.add_method("GetLineHeight", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let font_size = state.widgets.get(this.0).map_or(12.0_f32, |f| f.font_size);
        Ok((font_size * 1.2).ceil() as f64)
    });
}

/// Shared implementation for GetStringWidth / GetTextWidth / GetUnboundedStringWidth.
fn measure_text_width(lua: &Lua, id: u64) -> mlua::Result<f64> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let (text, font_path, font_size) = match state.widgets.get(id) {
        Some(f) => (f.text.clone(), f.font.clone(), f.font_size),
        None => return Ok(0.0),
    };
    drop(state);

    let text = match text {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(0.0),
    };

    if let Some(fs_rc) = lua.app_data_ref::<Rc<RefCell<WowFontSystem>>>() {
        let mut fs = fs_rc.borrow_mut();
        Ok(fs.measure_text_width(&text, font_path.as_deref(), font_size) as f64)
    } else {
        Ok(text.len() as f64 * 7.0)
    }
}

/// Measure string height, accounting for word wrap.
fn measure_string_height(lua: &Lua, id: u64) -> mlua::Result<f64> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let (text, font_path, font_size, word_wrap, width) = match state.widgets.get(id) {
        Some(f) => (
            f.text.clone(),
            f.font.clone(),
            f.font_size,
            f.word_wrap,
            f.width,
        ),
        None => return Ok(12.0_f64),
    };
    drop(state);
    let text = match text {
        Some(t) if !t.is_empty() => t,
        _ => return Ok((font_size * 1.2).ceil() as f64),
    };
    let wrap_width = if word_wrap && width > 0.0 {
        Some(width)
    } else {
        None
    };
    if let Some(fs_rc) = lua.app_data_ref::<Rc<RefCell<WowFontSystem>>>() {
        let mut fs = fs_rc.borrow_mut();
        Ok(fs.measure_text_height(&text, font_path.as_deref(), font_size, wrap_width) as f64)
    } else {
        Ok((font_size * 1.2).ceil() as f64)
    }
}

/// SetWordWrap, GetWordWrap, IsTruncated, CanWordWrap, GetWrappedWidth,
/// SetNonSpaceWrap, CanNonSpaceWrap, SetMaxLines, GetMaxLines.
fn add_word_wrap_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetWordWrap", |lua, this, wrap: bool| {
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(this.0)
        {
            frame.word_wrap = wrap;
        }
        Ok(())
    });
    methods.add_method("GetWordWrap", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(s) = state_rc.try_borrow()
            && let Some(frame) = s.widgets.get(this.0)
        {
            return Ok(frame.word_wrap);
        }
        Ok(false)
    });
    methods.add_method("IsTruncated", |_, _this, ()| Ok(false));
    methods.add_method("CanWordWrap", |_, _this, ()| Ok(true));
    methods.add_method("GetWrappedWidth", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.width).unwrap_or(0.0))
    });
    methods.add_method("SetNonSpaceWrap", |_, _this, _wrap: bool| Ok(()));
    methods.add_method("CanNonSpaceWrap", |_, _this, ()| Ok(true));
    add_max_lines_methods(methods);
}

/// SetMaxLines, GetMaxLines.
fn add_max_lines_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetMaxLines", |lua, this, max_lines: i32| {
        let state_rc = get_sim_state(lua);
        if let Ok(mut s) = state_rc.try_borrow_mut()
            && let Some(frame) = s.widgets.get_mut_visual(this.0)
        {
            frame.max_lines = max_lines.max(0) as u32;
        }
        Ok(())
    });
    methods.add_method("GetMaxLines", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        if let Ok(s) = state_rc.try_borrow()
            && let Some(frame) = s.widgets.get(this.0)
        {
            return Ok(frame.max_lines as i32);
        }
        Ok(0i32)
    });
}

/// SetTextHeight, GetTextHeight.
fn add_text_height_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetTextHeight", |lua, this, height: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.font_size = height as f32;
        }
        Ok(())
    });
    methods.add_method("GetTextHeight", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            return Ok(frame.font_size as f64);
        }
        Ok(12.0_f64)
    });
}

/// SetTextScale, GetTextScale.
fn add_text_scale_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetTextScale", |lua, this, scale: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.text_scale = scale;
        }
        Ok(())
    });
    methods.add_method("GetTextScale", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0) {
            return Ok(frame.text_scale);
        }
        Ok(1.0_f64)
    });
}

/// SetIndentedWordWrap, SetSpacing, GetSpacing.
fn add_spacing_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetIndentedWordWrap",
        |lua, this, args: mlua::MultiValue| set_indented_word_wrap_impl(lua, this.0, args),
    );
    methods.add_method("SetSpacing", |lua, this, args: mlua::MultiValue| {
        set_spacing_impl(lua, this.0, args)
    });
    methods.add_method("GetSpacing", |lua, this, args: mlua::MultiValue| {
        get_spacing_impl(lua, this.0, args)
    });
}

/// SetIndentedWordWrap implementation.
fn set_indented_word_wrap_impl(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    let is_html = is_simple_html(lua, id);
    if is_html
        && args_vec.len() >= 2
        && let Some(Value::String(s)) = args_vec.first()
    {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            return set_indented_wrap_html(lua, id, &type_str, &args_vec);
        }
    }
    Ok(())
}

/// SetSpacing implementation.
fn set_spacing_impl(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    let is_html = is_simple_html(lua, id);
    if is_html
        && args_vec.len() >= 2
        && let Some(Value::String(s)) = args_vec.first()
    {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            return set_spacing_html(lua, id, &type_str, &args_vec);
        }
    }
    Ok(())
}

/// GetSpacing implementation.
fn get_spacing_impl(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<f64> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    if let Some(Value::String(s)) = args_vec.first() {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            return get_spacing_html(lua, id, &type_str);
        }
    }
    Ok(0.0_f64)
}

/// Set indented word wrap for a SimpleHTML text type.
fn set_indented_wrap_html(
    lua: &Lua,
    id: u64,
    type_str: &str,
    args_vec: &[Value],
) -> mlua::Result<()> {
    let indent = matches!(args_vec.get(1), Some(Value::Boolean(true)));
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(data) = state.simple_htmls.get_mut(&id) {
        let style = data
            .text_styles
            .entry(type_str.to_string())
            .or_insert_with(TextStyle::default);
        style.indented_word_wrap = indent;
    }
    Ok(())
}

/// Set spacing for a SimpleHTML text type.
fn set_spacing_html(lua: &Lua, id: u64, type_str: &str, args_vec: &[Value]) -> mlua::Result<()> {
    let spacing = val_to_f64(args_vec.get(1), 0.0);
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(data) = state.simple_htmls.get_mut(&id) {
        let style = data
            .text_styles
            .entry(type_str.to_string())
            .or_insert_with(TextStyle::default);
        style.spacing = spacing as f32;
    }
    Ok(())
}

/// Get spacing for a SimpleHTML text type.
fn get_spacing_html(lua: &Lua, id: u64, type_str: &str) -> mlua::Result<f64> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    if let Some(data) = state.simple_htmls.get(&id)
        && let Some(style) = data.text_styles.get(type_str)
    {
        return Ok(style.spacing as f64);
    }
    Ok(0.0_f64)
}
