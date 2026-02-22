//! Rect query methods: GetRect, GetScaledRect, GetBounds, GetLeft/Right/Top/Bottom/Center.
//!
//! All methods return nothing (0 values) when the frame has no valid rect (no anchors).

use crate::lua_api::frame::handle::{get_sim_state, lud_to_id};
use crate::lua_api::layout::compute_frame_rect;
use crate::lua_api::SimState;
use mlua::{LightUserData, Lua, Value};

use super::methods_core::screen_dims;

/// Check if a frame has a valid rect (has anchors that can be resolved).
fn frame_has_valid_rect(state: &SimState, id: u64) -> bool {
    state.widgets.get(id).map(|f| !f.anchors.is_empty()).unwrap_or(false)
}

/// Compute effective scale by walking the parent chain.
fn effective_scale(widgets: &crate::widget::WidgetRegistry, id: u64) -> f32 {
    let mut scale = 1.0f32;
    let mut cur = Some(id);
    while let Some(cid) = cur {
        if let Some(f) = widgets.get(cid) {
            scale *= f.scale;
            cur = f.parent_id;
        } else {
            break;
        }
    }
    scale
}

/// Convert rect values to a 4-value MultiValue.
fn rect_to_multivalue(left: f32, bottom: f32, width: f32, height: f32) -> mlua::MultiValue {
    mlua::MultiValue::from_vec(vec![
        Value::Number(left as f64),
        Value::Number(bottom as f64),
        Value::Number(width as f64),
        Value::Number(height as f64),
    ])
}

pub fn add_rect_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    add_rect_full_methods(lua, methods)?;
    add_rect_edge_methods(lua, methods)?;
    Ok(())
}

/// GetRect, GetScaledRect, GetBounds — return nothing when no valid rect.
fn add_rect_full_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetRect", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if !frame_has_valid_rect(&state, id) { return Ok(mlua::MultiValue::new()); }
        let (sw, sh) = screen_dims(&state);
        let rect = compute_frame_rect(&state.widgets, id, sw, sh);
        let bottom = sh - rect.y - rect.height;
        Ok(rect_to_multivalue(rect.x, bottom, rect.width, rect.height))
    })?)?;

    methods.set("GetScaledRect", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if !frame_has_valid_rect(&state, id) { return Ok(mlua::MultiValue::new()); }
        let (sw, sh) = screen_dims(&state);
        let rect = compute_frame_rect(&state.widgets, id, sw, sh);
        let scale = effective_scale(&state.widgets, id);
        let left = rect.x * scale;
        let bottom = (sh - rect.y - rect.height) * scale;
        Ok(rect_to_multivalue(left, bottom, rect.width * scale, rect.height * scale))
    })?)?;

    methods.set("GetBounds", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if !frame_has_valid_rect(&state, id) { return Ok(mlua::MultiValue::new()); }
        let (sw, sh) = screen_dims(&state);
        let rect = compute_frame_rect(&state.widgets, id, sw, sh);
        let bottom = sh - rect.y - rect.height;
        Ok(rect_to_multivalue(rect.x, bottom, rect.width, rect.height))
    })?)?;

    Ok(())
}

/// GetLeft, GetRight, GetTop, GetBottom, GetCenter — return nothing when no valid rect.
fn add_rect_edge_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetLeft", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if !frame_has_valid_rect(&state, id) { return Ok(mlua::MultiValue::new()); }
        let (sw, sh) = screen_dims(&state);
        let rect = compute_frame_rect(&state.widgets, id, sw, sh);
        Ok(mlua::MultiValue::from_vec(vec![Value::Number(rect.x as f64)]))
    })?)?;

    methods.set("GetRight", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if !frame_has_valid_rect(&state, id) { return Ok(mlua::MultiValue::new()); }
        let (sw, sh) = screen_dims(&state);
        let rect = compute_frame_rect(&state.widgets, id, sw, sh);
        Ok(mlua::MultiValue::from_vec(vec![Value::Number((rect.x + rect.width) as f64)]))
    })?)?;

    methods.set("GetTop", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if !frame_has_valid_rect(&state, id) { return Ok(mlua::MultiValue::new()); }
        let (sw, sh) = screen_dims(&state);
        let rect = compute_frame_rect(&state.widgets, id, sw, sh);
        Ok(mlua::MultiValue::from_vec(vec![Value::Number((sh - rect.y) as f64)]))
    })?)?;

    methods.set("GetBottom", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if !frame_has_valid_rect(&state, id) { return Ok(mlua::MultiValue::new()); }
        let (sw, sh) = screen_dims(&state);
        let rect = compute_frame_rect(&state.widgets, id, sw, sh);
        Ok(mlua::MultiValue::from_vec(vec![Value::Number((sh - rect.y - rect.height) as f64)]))
    })?)?;

    methods.set("GetCenter", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if !frame_has_valid_rect(&state, id) { return Ok(mlua::MultiValue::new()); }
        let (sw, sh) = screen_dims(&state);
        let rect = compute_frame_rect(&state.widgets, id, sw, sh);
        let cx = Value::Number((rect.x + rect.width / 2.0) as f64);
        let cy = Value::Number((sh - rect.y - rect.height / 2.0) as f64);
        Ok(mlua::MultiValue::from_vec(vec![cx, cy]))
    })?)?;

    Ok(())
}
