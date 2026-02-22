//! Rect query methods: GetRect, GetScaledRect, GetBounds, GetLeft/Right/Top/Bottom/Center.
//!
//! All methods return nothing (0 values) when the frame has no valid rect (no anchors).
//! Calling these methods resolves the dirty flag (layout becomes valid for IsRectValid).
//!
//! After `check_and_resolve()`, the frame's `layout_rect` contains the correct
//! anchor-resolved rectangle in screen space (top-left origin, y-down, scaled by
//! effective_scale). GetRect/edge methods divide by effective_scale to return WoW
//! "UI coordinates"; GetScaledRect returns screen-space values directly.

use crate::lua_api::frame::handle::{get_sim_state, lud_to_id};
use crate::lua_api::SimState;
use crate::LayoutRect;
use mlua::{LightUserData, Lua, Value};

use super::methods_core::screen_dims;

/// Resolved layout data extracted from SimState while borrowed.
struct ResolvedRect {
    rect: LayoutRect,
    eff_scale: f32,
    screen_height: f32,
}

/// Check if a frame has anchors. Returns false if no frame or no anchors.
fn has_anchors(state: &SimState, id: u64) -> bool {
    state.widgets.get(id).map(|f| !f.anchors.is_empty()).unwrap_or(false)
}

/// Resolve dirty flag, then extract layout_rect + effective_scale + screen_height.
/// Returns None if the frame has no anchors or no layout_rect.
fn resolve_and_extract(lua: &Lua, id: u64) -> Option<ResolvedRect> {
    let state_rc = get_sim_state(lua);
    if !has_anchors(&state_rc.borrow(), id) { return None; }
    state_rc.borrow_mut().resolve_rect_if_dirty(id);
    let state = state_rc.borrow();
    let (_, sh) = screen_dims(&state);
    let frame = state.widgets.get(id)?;
    let rect = frame.layout_rect?;
    Some(ResolvedRect { rect, eff_scale: frame.effective_scale, screen_height: sh })
}

/// Convert layout_rect to WoW UI coordinates (bottom-left origin, divided by effective_scale).
fn to_wow_rect(r: &ResolvedRect) -> (f32, f32, f32, f32) {
    let e = r.eff_scale;
    let left = r.rect.x / e;
    let bottom = (r.screen_height - r.rect.y - r.rect.height) / e;
    (left, bottom, r.rect.width / e, r.rect.height / e)
}

/// Convert layout_rect to WoW screen-space coordinates (no scale division).
fn to_wow_scaled_rect(r: &ResolvedRect) -> (f32, f32, f32, f32) {
    let left = r.rect.x;
    let bottom = r.screen_height - r.rect.y - r.rect.height;
    (left, bottom, r.rect.width, r.rect.height)
}

fn single_value(v: f32) -> mlua::MultiValue {
    mlua::MultiValue::from_vec(vec![Value::Number(v as f64)])
}

fn rect_to_multivalue(left: f32, bottom: f32, width: f32, height: f32) -> mlua::MultiValue {
    mlua::MultiValue::from_vec(vec![
        Value::Number(left as f64),
        Value::Number(bottom as f64),
        Value::Number(width as f64),
        Value::Number(height as f64),
    ])
}

pub fn add_rect_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    add_get_rect(lua, methods)?;
    add_get_scaled_rect(lua, methods)?;
    add_get_bounds(lua, methods)?;
    add_get_left(lua, methods)?;
    add_get_right(lua, methods)?;
    add_get_top(lua, methods)?;
    add_get_bottom(lua, methods)?;
    add_get_center(lua, methods)?;
    Ok(())
}

fn add_get_rect(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetRect", lua.create_function(|lua, ud: LightUserData| {
        let r = match resolve_and_extract(lua, lud_to_id(ud)) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        let (l, b, w, h) = to_wow_rect(&r);
        Ok(rect_to_multivalue(l, b, w, h))
    })?)
}

fn add_get_scaled_rect(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetScaledRect", lua.create_function(|lua, ud: LightUserData| {
        let r = match resolve_and_extract(lua, lud_to_id(ud)) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        let (l, b, w, h) = to_wow_scaled_rect(&r);
        Ok(rect_to_multivalue(l, b, w, h))
    })?)
}

fn add_get_bounds(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetBounds", lua.create_function(|lua, ud: LightUserData| {
        let r = match resolve_and_extract(lua, lud_to_id(ud)) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        let (l, b, w, h) = to_wow_rect(&r);
        Ok(rect_to_multivalue(l, b, w, h))
    })?)
}

fn add_get_left(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetLeft", lua.create_function(|lua, ud: LightUserData| {
        let r = match resolve_and_extract(lua, lud_to_id(ud)) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        Ok(single_value(r.rect.x / r.eff_scale))
    })?)
}

fn add_get_right(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetRight", lua.create_function(|lua, ud: LightUserData| {
        let r = match resolve_and_extract(lua, lud_to_id(ud)) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        Ok(single_value((r.rect.x + r.rect.width) / r.eff_scale))
    })?)
}

fn add_get_top(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetTop", lua.create_function(|lua, ud: LightUserData| {
        let r = match resolve_and_extract(lua, lud_to_id(ud)) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        Ok(single_value((r.screen_height - r.rect.y) / r.eff_scale))
    })?)
}

fn add_get_bottom(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetBottom", lua.create_function(|lua, ud: LightUserData| {
        let r = match resolve_and_extract(lua, lud_to_id(ud)) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        Ok(single_value((r.screen_height - r.rect.y - r.rect.height) / r.eff_scale))
    })?)
}

fn add_get_center(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetCenter", lua.create_function(|lua, ud: LightUserData| {
        let r = match resolve_and_extract(lua, lud_to_id(ud)) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        let e = r.eff_scale;
        let cx = (r.rect.x + r.rect.width / 2.0) / e;
        let cy = (r.screen_height - r.rect.y - r.rect.height / 2.0) / e;
        Ok(mlua::MultiValue::from_vec(vec![
            Value::Number(cx as f64),
            Value::Number(cy as f64),
        ]))
    })?)
}
