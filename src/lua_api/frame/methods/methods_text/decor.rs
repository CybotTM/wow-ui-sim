//! Frame decoration text methods: title, border, portrait, shadow.

use super::super::super::handle::{FrameRef, get_sim_state};
use super::{is_simple_html, is_text_type, val_to_f32, val_to_f64};
use crate::loader::helpers::lua_global_ref;
use crate::lua_api::simple_html::TextStyle;
use mlua::{Lua, Value};

/// Add title, border, portrait, and shadow methods.
pub fn add_decor_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_title_methods(methods);
    add_border_methods(methods);
    add_shadow_offset_methods(methods);
    add_shadow_color_methods(methods);
}

/// SetTitle, GetTitle, SetTitleOffsets.
fn add_title_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetTitle", |lua, this, title: Option<String>| {
        set_title_impl(lua, this.0, title)
    });
    methods.add_method("GetTitle", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let title = state
            .widgets
            .get(this.0)
            .and_then(|f| f.title.clone())
            .unwrap_or_default();
        Ok(title)
    });
    methods.add_method(
        "SetTitleOffsets",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
}

/// SetTitle implementation: updates TitleText child and frame title field.
fn set_title_impl(lua: &Lua, id: u64, title: Option<String>) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let title_text_id = find_title_text_id(&state, id);
    if let Some(tt_id) = title_text_id
        && let Some(title_text) = state.widgets.get_mut_visual(tt_id)
    {
        title_text.text = title.clone();
        if title_text.height == 0.0 {
            title_text.height = title_text.font_size.max(12.0);
        }
    }
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.title = title;
    }
    Ok(())
}

/// Find the TitleText FontString ID for a frame.
fn find_title_text_id(state: &crate::lua_api::SimState, id: u64) -> Option<u64> {
    state
        .widgets
        .get(id)
        .and_then(|f| f.children_keys.get("TitleContainer").copied())
        .and_then(|tc_id| state.widgets.get(tc_id))
        .and_then(|tc| tc.children_keys.get("TitleText").copied())
}

/// SetBorder, SetBorderColor, SetBorderInsets.
fn add_border_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetBorder", |lua, this, layout_name: Option<String>| {
        set_border_impl(lua, this.0, layout_name)
    });
    methods.add_method("SetBorderColor", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method(
        "SetBorderInsets",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
}

/// SetBorder implementation.
fn set_border_impl(lua: &Lua, id: u64, layout_name: Option<String>) -> mlua::Result<()> {
    if let Some(layout) = layout_name {
        let frame_name = {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            state
                .widgets
                .get(id)
                .and_then(|f| f.name.clone())
                .unwrap_or_else(|| format!("__frame_{}", id))
        };
        exec_set_border_lua(lua, &frame_name, &layout);
    }
    Ok(())
}

/// Execute the SetBorder NineSlice Lua code.
fn exec_set_border_lua(lua: &Lua, frame_name: &str, layout: &str) {
    let frame_ref = lua_global_ref(frame_name);
    let code = format!(
        r#"
        local frame = {frame_ref}
        local layoutName = "{layout}"
        if frame and frame.NineSlice then
            if NineSliceUtil and NineSliceUtil.ApplyLayout and NineSliceUtil.GetLayout then
                local layoutTable = NineSliceUtil.GetLayout(layoutName)
                if layoutTable then
                    NineSliceUtil.ApplyLayout(frame.NineSlice, layoutTable)
                end
            end
        end
        "#
    );
    if let Err(e) = lua.load(&code).exec() {
        eprintln!("SetBorder Lua error: {}", e);
    }
}

/// Extract numeric RGBA values from a mixed argument list, skipping non-numbers.
fn extract_rgba(args: &[Value]) -> (f32, f32, f32, f32) {
    let values: Vec<f32> = args
        .iter()
        .filter_map(|v| match v {
            Value::Number(n) => Some(*n as f32),
            Value::Integer(n) => Some(*n as f32),
            _ => None,
        })
        .collect();
    (
        values.first().copied().unwrap_or(0.0),
        values.get(1).copied().unwrap_or(0.0),
        values.get(2).copied().unwrap_or(0.0),
        values.get(3).copied().unwrap_or(1.0),
    )
}

/// SetShadowOffset, GetShadowOffset.
fn add_shadow_offset_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetShadowOffset", |lua, this, args: mlua::MultiValue| {
        set_shadow_offset_impl(lua, this.0, args)
    });
    methods.add_method("GetShadowOffset", |lua, this, args: mlua::MultiValue| {
        get_shadow_offset_impl(lua, this.0, args)
    });
}

/// SetShadowOffset implementation.
fn set_shadow_offset_impl(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    let is_html = is_simple_html(lua, id);
    if is_html && let Some(Value::String(s)) = args_vec.first() {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            return set_shadow_offset_html(lua, id, &type_str, &args_vec);
        }
    }
    let x = val_to_f64(args_vec.first(), 0.0);
    let y = val_to_f64(args_vec.get(1), 0.0);
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.shadow_offset = (x as f32, y as f32);
    }
    Ok(())
}

/// GetShadowOffset implementation.
fn get_shadow_offset_impl(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<(f64, f64)> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    if let Some(Value::String(s)) = args_vec.first() {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            return get_shadow_offset_html(lua, id, &type_str);
        }
    }
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let (x, y) = state
        .widgets
        .get(id)
        .map(|f| f.shadow_offset)
        .unwrap_or((0.0, 0.0));
    Ok((x as f64, y as f64))
}

/// Set shadow offset for a SimpleHTML text type.
fn set_shadow_offset_html(
    lua: &Lua,
    id: u64,
    type_str: &str,
    args_vec: &[Value],
) -> mlua::Result<()> {
    let x = val_to_f64(args_vec.get(1), 0.0);
    let y = val_to_f64(args_vec.get(2), 0.0);
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(data) = state.simple_htmls.get_mut(&id) {
        let style = data
            .text_styles
            .entry(type_str.to_string())
            .or_insert_with(TextStyle::default);
        style.shadow_offset = (x as f32, y as f32);
    }
    Ok(())
}

/// Get shadow offset for a SimpleHTML text type.
fn get_shadow_offset_html(lua: &Lua, id: u64, type_str: &str) -> mlua::Result<(f64, f64)> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    if let Some(data) = state.simple_htmls.get(&id)
        && let Some(style) = data.text_styles.get(type_str)
    {
        return Ok((style.shadow_offset.0 as f64, style.shadow_offset.1 as f64));
    }
    Ok((0.0_f64, 0.0_f64))
}

/// SetShadowColor, GetShadowColor.
fn add_shadow_color_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetShadowColor", |lua, this, args: mlua::MultiValue| {
        set_shadow_color_impl(lua, this.0, args)
    });
    methods.add_method("GetShadowColor", |lua, this, args: mlua::MultiValue| {
        get_shadow_color_impl(lua, this.0, args)
    });
}

/// SetShadowColor implementation.
fn set_shadow_color_impl(lua: &Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    if is_simple_html(lua, id)
        && let Some(Value::String(s)) = args_vec.first()
    {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            return set_shadow_color_html(lua, id, &type_str, &args_vec);
        }
    }
    let (r, g, b, a) = extract_rgba(&args_vec);
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.shadow_color = crate::widget::Color::new(r, g, b, a);
    }
    Ok(())
}

/// GetShadowColor implementation.
fn get_shadow_color_impl(
    lua: &Lua,
    id: u64,
    args: mlua::MultiValue,
) -> mlua::Result<(f64, f64, f64, f64)> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    if let Some(Value::String(s)) = args_vec.first() {
        let type_str = s.to_string_lossy().to_string();
        if is_text_type(&type_str) {
            return get_shadow_color_html(lua, id, &type_str);
        }
    }
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let color = state
        .widgets
        .get(id)
        .map(|f| f.shadow_color)
        .unwrap_or(crate::widget::Color::new(0.0, 0.0, 0.0, 0.0));
    Ok((
        color.r as f64,
        color.g as f64,
        color.b as f64,
        color.a as f64,
    ))
}

/// Set shadow color for a SimpleHTML text type.
fn set_shadow_color_html(
    lua: &Lua,
    id: u64,
    type_str: &str,
    args_vec: &[Value],
) -> mlua::Result<()> {
    let r = val_to_f32(args_vec.get(1), 0.0);
    let g = val_to_f32(args_vec.get(2), 0.0);
    let b = val_to_f32(args_vec.get(3), 0.0);
    let a = val_to_f32(args_vec.get(4), 1.0);
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(data) = state.simple_htmls.get_mut(&id) {
        let style = data
            .text_styles
            .entry(type_str.to_string())
            .or_insert_with(TextStyle::default);
        style.shadow_color = (r, g, b, a);
    }
    Ok(())
}

/// Get shadow color for a SimpleHTML text type.
fn get_shadow_color_html(lua: &Lua, id: u64, type_str: &str) -> mlua::Result<(f64, f64, f64, f64)> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    if let Some(data) = state.simple_htmls.get(&id)
        && let Some(style) = data.text_styles.get(type_str)
    {
        return Ok((
            style.shadow_color.0 as f64,
            style.shadow_color.1 as f64,
            style.shadow_color.2 as f64,
            style.shadow_color.3 as f64,
        ));
    }
    Ok((0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64))
}
