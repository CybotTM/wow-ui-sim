//! Standalone drag helper functions for mouse event handling.

use crate::widget::{AnchorPoint, WidgetType};

pub(super) fn frame_motion_scripts_allowed(frame: &crate::widget::Frame) -> bool {
    frame_enabled(frame) || frame.motion_scripts_while_disabled
}

pub(super) fn frame_enabled(frame: &crate::widget::Frame) -> bool {
    frame
        .attributes
        .get("__enabled")
        .and_then(|value| match value {
            crate::widget::AttributeValue::Boolean(enabled) => Some(*enabled),
            _ => None,
        })
        .unwrap_or(true)
}

pub(super) fn moving_drag_anchor_update(
    state: &crate::lua_api::SimState,
    drag_id: u64,
    dx: f32,
    dy: f32,
    screen_width: f32,
    screen_height: f32,
) -> Option<(Option<u64>, f32, f32)> {
    let frame = state.widgets.get(drag_id)?;
    if !frame.is_moving {
        return None;
    }

    let rect = frame.layout_rect?;
    let parent_id = frame.parent_id;
    let (parent_x, parent_y) = parent_id
        .and_then(|id| state.widgets.get(id).and_then(|parent| parent.layout_rect))
        .map(|rect| (rect.x, rect.y))
        .unwrap_or((0.0, 0.0));

    let mut new_left = rect.x + dx;
    let mut new_top = rect.y + dy;
    if frame.clamped_to_screen {
        new_left = clamp_axis_to_viewport(new_left, rect.width, screen_width);
        new_top = clamp_axis_to_viewport(new_top, rect.height, screen_height);
    }
    Some((parent_id, new_left - parent_x, -(new_top - parent_y)))
}

pub(super) fn clamp_axis_to_viewport(position: f32, size: f32, viewport_size: f32) -> f32 {
    if size >= viewport_size {
        0.0
    } else {
        position.clamp(0.0, viewport_size - size)
    }
}

pub(super) fn reanchor_moving_drag_frame(
    state: &mut crate::lua_api::SimState,
    drag_id: u64,
    parent_id: Option<u64>,
    x_offset: f32,
    y_offset: f32,
) {
    state.widgets.remove_all_anchor_dependents_for(drag_id);
    if let Some(parent_id) = parent_id {
        state.widgets.add_anchor_dependent(parent_id, drag_id);
    }

    if let Some(frame) = state.widgets.get_mut_visual(drag_id) {
        frame.clear_all_points();
        frame.set_point(
            AnchorPoint::TopLeft,
            parent_id.map(|id| id as usize),
            AnchorPoint::TopLeft,
            x_offset,
            y_offset,
        );
    }
    state.widgets.mark_rect_dirty(drag_id);
}

pub(super) fn find_drag_script_target(
    env: &crate::lua_api::WowLuaEnv,
    frame_id: u64,
    script_name: &str,
) -> Option<u64> {
    let mut current = Some(frame_id);
    while let Some(id) = current {
        if env.has_script_handler(id, script_name) {
            return Some(id);
        }
        current = env
            .state()
            .borrow()
            .widgets
            .get(id)
            .and_then(|f| f.parent_id);
    }
    None
}

pub(super) fn find_slider_drag_target(
    state: &crate::lua_api::state::SimState,
    frame_id: u64,
) -> Option<u64> {
    let mut current = Some(frame_id);
    while let Some(id) = current {
        let frame = state.widgets.get(id)?;
        if frame.widget_type == WidgetType::Slider {
            return Some(id);
        }
        current = frame.parent_id;
    }
    None
}
