//! Pure frame helper functions shared by loader code and rilua method modules.

use crate::widget::{Anchor, AnchorPoint, Frame, WidgetType};

pub fn set_all_points_anchors_pub(frame: &mut Frame, parent_id: u64) {
    frame.anchors.push(Anchor {
        point: AnchorPoint::TopLeft,
        relative_to: None,
        relative_to_id: Some(parent_id as usize),
        relative_point: AnchorPoint::TopLeft,
        x_offset: 0.0,
        y_offset: 0.0,
    });
    frame.anchors.push(Anchor {
        point: AnchorPoint::BottomRight,
        relative_to: None,
        relative_to_id: Some(parent_id as usize),
        relative_point: AnchorPoint::BottomRight,
        x_offset: 0.0,
        y_offset: 0.0,
    });
}

pub fn get_or_create_button_texture(
    state: &mut crate::lua_api::SimState,
    button_id: u64,
    key: &str,
) -> u64 {
    if let Some(tex_id) = button_texture_child_id(state, button_id, key) {
        return refresh_button_texture_child(state, button_id, key, tex_id);
    }
    create_button_texture_child(state, button_id, key)
}

fn button_texture_child_id(
    state: &crate::lua_api::SimState,
    button_id: u64,
    key: &str,
) -> Option<u64> {
    state
        .widgets
        .get(button_id)
        .and_then(|frame| frame.children_keys.get(key).copied())
}

fn refresh_button_texture_child(
    state: &mut crate::lua_api::SimState,
    button_id: u64,
    key: &str,
    tex_id: u64,
) -> u64 {
    let needs_anchors = state
        .widgets
        .get(tex_id)
        .map(|texture| texture.anchors.is_empty())
        .unwrap_or(false);
    let needs_parent_key = state
        .widgets
        .get(tex_id)
        .map(|texture| texture.parent_key.as_deref() != Some(key))
        .unwrap_or(false);
    if needs_anchors || needs_parent_key {
        if let Some(texture) = state.widgets.get_mut_visual(tex_id) {
            if needs_anchors {
                set_all_points_anchors_pub(texture, button_id);
            }
            texture.parent_key = Some(key.to_string());
        }
        state.widgets.mark_rect_dirty(button_id);
    }
    tex_id
}

fn create_button_texture_child(
    state: &mut crate::lua_api::SimState,
    button_id: u64,
    key: &str,
) -> u64 {
    let texture = new_button_texture_child(state, button_id, key);
    let texture_id = texture.id;
    state.widgets.register(texture);
    state.widgets.add_child(button_id, texture_id);

    if let Some(frame) = state.widgets.get_mut_visual(button_id) {
        frame.children_keys.insert(key.to_string(), texture_id);
    }
    state.widgets.mark_rect_dirty(button_id);
    texture_id
}

fn new_button_texture_child(state: &crate::lua_api::SimState, button_id: u64, key: &str) -> Frame {
    let mut texture = Frame::new(WidgetType::Texture, None, Some(button_id));
    set_all_points_anchors_pub(&mut texture, button_id);
    texture.parent_key = Some(key.to_string());
    apply_button_texture_defaults(&mut texture, state, button_id, key);
    texture
}

fn apply_button_texture_defaults(
    texture: &mut Frame,
    state: &crate::lua_api::SimState,
    button_id: u64,
    key: &str,
) {
    if key == "HighlightTexture" {
        texture.draw_layer = crate::widget::DrawLayer::Highlight;
        texture.alpha_mode = Some("ADD".to_string());
        texture.blend_mode = crate::render::BlendMode::Additive;
    }
    if let Some(parent) = state.widgets.get(button_id) {
        texture.frame_strata = parent.frame_strata;
        texture.frame_level = parent.frame_level + 1;
        texture.layout_rect = parent.layout_rect;
    }
}
