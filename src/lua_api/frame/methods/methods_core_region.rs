use super::super::handle::{FrameRef, extract_frame_id};
use crate::lua_api::frame::handle::get_sim_state;
use crate::widget::DrawLayer;
use mlua::Value;

pub(super) fn add_region_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_rect_query_methods(methods);
    add_region_stub_methods(methods);
}

fn add_rect_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsRectValid", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let has_anchors = state
            .widgets
            .get(id)
            .map(|f| !f.anchors.is_empty())
            .unwrap_or(false);
        if !has_anchors {
            return Ok(false);
        }
        Ok(!state.widgets.is_rect_dirty(id))
    });

    methods.add_method("IsMouseMotionFocus", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.hovered_frame == Some(this.0))
    });
}

fn add_region_stub_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsObjectLoaded", |_lua, _this, ()| Ok(true));
    add_is_mouse_over_method(methods);
    methods.add_method("StopAnimating", |_lua, _this, ()| Ok(()));
    add_get_source_location_method(methods);
    add_intersects_method(methods);
    methods.add_method("IsDrawLayerEnabled", |lua, this, layer: String| {
        let Some(layer) = draw_layer_from_name(&layer) else {
            return Ok(false);
        };
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.is_draw_layer_enabled(layer))
            .unwrap_or(false))
    });
    methods.add_method(
        "SetDrawLayerEnabled",
        |lua, this, (layer, enabled): (String, bool)| {
            let Some(layer) = draw_layer_from_name(&layer) else {
                return Ok(());
            };
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                frame.set_draw_layer_enabled(layer, enabled);
            }
            Ok(())
        },
    );
}

fn parse_mouse_over_offsets(args: mlua::MultiValue) -> (f32, f32, f32, f32) {
    let mut values = args.into_iter();
    let left = next_mouse_over_offset(&mut values);
    let right = next_mouse_over_offset(&mut values);
    let top = next_mouse_over_offset(&mut values);
    let bottom = next_mouse_over_offset(&mut values);
    (left, right, top, bottom)
}

fn next_mouse_over_offset(values: &mut impl Iterator<Item = Value>) -> f32 {
    match values.next() {
        Some(Value::Number(n)) => n as f32,
        Some(Value::Integer(i)) => i as f32,
        _ => 0.0,
    }
}

fn draw_layer_from_name(layer: &str) -> Option<DrawLayer> {
    DrawLayer::from_str(layer)
}

fn source_location_for_owner(
    state: &crate::lua_api::state::SimState,
    owner_addon: Option<u16>,
) -> Option<String> {
    let addon = owner_addon.and_then(|idx| state.addons.get(idx as usize))?;
    let folder = addon.folder_name.as_str();
    if folder == "__BuiltIn" {
        return Some("Interface/FrameXML".to_string());
    }
    Some(format!("Interface/AddOns/{folder}"))
}

fn layout_rects_intersect(a: crate::LayoutRect, b: crate::LayoutRect) -> bool {
    let left = a.x.max(b.x);
    let top = a.y.max(b.y);
    let right = (a.x + a.width).min(b.x + b.width);
    let bottom = (a.y + a.height).min(b.y + b.height);
    right > left && bottom > top
}

fn add_is_mouse_over_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsMouseOver", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let (left, right, top, bottom) = parse_mouse_over_offsets(args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state.resolve_rect_if_dirty(id);
        let Some((mouse_x, mouse_y)) = state.mouse_position else {
            return Ok(false);
        };
        let Some(frame) = state.widgets.get(id) else {
            return Ok(false);
        };
        if !frame.visible || frame.effective_alpha <= 0.0 || !frame.mouse_enabled {
            return Ok(false);
        }
        let Some(rect) = frame.layout_rect else {
            return Ok(false);
        };
        let in_bounds = mouse_x >= rect.x - left
            && mouse_x <= rect.x + rect.width + right
            && mouse_y >= rect.y - top
            && mouse_y <= rect.y + rect.height + bottom;
        Ok(in_bounds)
    });
}

fn add_get_source_location_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetSourceLocation", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let Some(frame) = state.widgets.get(this.0) else {
            return Ok(Value::Nil);
        };
        let Some(location) = source_location_for_owner(&state, frame.owner_addon) else {
            return Ok(Value::Nil);
        };
        Ok(Value::String(lua.create_string(&location)?))
    });
}

fn add_intersects_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("Intersects", |lua, this, region: Value| {
        let Some(other_id) = extract_frame_id(&region) else {
            return Ok(false);
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let this_id = this.0;
        state.resolve_rect_if_dirty(this_id);
        state.resolve_rect_if_dirty(other_id);

        let Some(this_rect) = state.widgets.get(this_id).and_then(|f| f.layout_rect) else {
            return Ok(false);
        };
        let Some(other_rect) = state.widgets.get(other_id).and_then(|f| f.layout_rect) else {
            return Ok(false);
        };
        Ok(layout_rects_intersect(this_rect, other_rect))
    });
}
