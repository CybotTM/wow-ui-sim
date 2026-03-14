//! Rect query methods: GetRect, GetScaledRect, GetBounds, GetLeft/Right/Top/Bottom/Center.
//!
//! All methods return nothing (0 values) when the frame has no valid rect (no anchors).
//! Calling these methods resolves the dirty flag (layout becomes valid for IsRectValid).
//!
//! After `check_and_resolve()`, the frame's `layout_rect` contains the correct
//! anchor-resolved rectangle in screen space (top-left origin, y-down, scaled by
//! effective_scale). GetRect/edge methods divide by effective_scale to return WoW
//! "UI coordinates"; GetScaledRect returns screen-space values directly.

use super::super::handle::FrameRef;
use crate::LayoutRect;
use crate::lua_api::frame::handle::get_sim_state;
use mlua::Value;

use super::methods_core::screen_dims;

/// Resolved layout data extracted from SimState while borrowed.
struct ResolvedRect {
    rect: LayoutRect,
    eff_scale: f32,
    screen_height: f32,
}

/// Resolve dirty flag, then extract layout_rect + effective_scale + screen_height.
/// Returns None if the frame has no anchors or no layout_rect.
fn resolve_and_extract(lua: &mlua::Lua, id: u64) -> Option<ResolvedRect> {
    let state_rc = get_sim_state(lua);

    let needs_root_rect = {
        let state = state_rc.borrow();
        let frame = state.widgets.get(id)?;
        frame.anchors.is_empty()
            && frame.parent_id.is_none()
            && frame.width > 0.0
            && frame.height > 0.0
            && frame.layout_rect.is_none()
    };

    if needs_root_rect {
        state_rc.borrow_mut().invalidate_layout(id);
    }

    state_rc.borrow_mut().resolve_rect_if_dirty(id);
    let state = state_rc.borrow();
    let (_, sh) = screen_dims(&state);
    let frame = state.widgets.get(id)?;
    let rect = frame.layout_rect?;
    Some(ResolvedRect {
        rect,
        eff_scale: frame.effective_scale,
        screen_height: sh,
    })
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

pub fn add_rect_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_rect(methods);
    add_get_scaled_rect(methods);
    add_get_bounds(methods);
    add_get_left(methods);
    add_get_right(methods);
    add_get_top(methods);
    add_get_bottom(methods);
    add_get_center(methods);
}

fn add_get_rect<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetRect", |lua, this, ()| {
        let r = match resolve_and_extract(lua, this.0) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        let (l, b, w, h) = to_wow_rect(&r);
        Ok(rect_to_multivalue(l, b, w, h))
    });
}

fn add_get_scaled_rect<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetScaledRect", |lua, this, ()| {
        let r = match resolve_and_extract(lua, this.0) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        let (l, b, w, h) = to_wow_scaled_rect(&r);
        Ok(rect_to_multivalue(l, b, w, h))
    });
}

fn add_get_bounds<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetBounds", |lua, this, ()| {
        let r = match resolve_and_extract(lua, this.0) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        let (l, b, w, h) = to_wow_rect(&r);
        Ok(rect_to_multivalue(l, b, w, h))
    });
}

fn add_get_left<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetLeft", |lua, this, ()| {
        let r = match resolve_and_extract(lua, this.0) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        Ok(single_value(r.rect.x / r.eff_scale))
    });
}

fn add_get_right<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetRight", |lua, this, ()| {
        let r = match resolve_and_extract(lua, this.0) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        Ok(single_value((r.rect.x + r.rect.width) / r.eff_scale))
    });
}

fn add_get_top<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetTop", |lua, this, ()| {
        let r = match resolve_and_extract(lua, this.0) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        Ok(single_value((r.screen_height - r.rect.y) / r.eff_scale))
    });
}

fn add_get_bottom<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetBottom", |lua, this, ()| {
        let r = match resolve_and_extract(lua, this.0) {
            Some(r) => r,
            None => return Ok(mlua::MultiValue::new()),
        };
        Ok(single_value(
            (r.screen_height - r.rect.y - r.rect.height) / r.eff_scale,
        ))
    });
}

fn add_get_center<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetCenter", |lua, this, ()| {
        let r = match resolve_and_extract(lua, this.0) {
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
    });
}
