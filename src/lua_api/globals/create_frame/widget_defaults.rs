use super::super::super::SimState;
use super::super::super::frame::sync_child_to_lua;
use crate::widget::{Frame, WidgetType};
use mlua::Lua;

/// Create default children for widget types that fundamentally need them.
/// This is separate from templates - these are intrinsic to the widget type.
pub(super) fn create_widget_type_defaults(
    lua: &Lua,
    state: &mut SimState,
    frame_id: u64,
    widget_type: WidgetType,
) {
    match widget_type {
        WidgetType::Button | WidgetType::CheckButton => {
            create_button_defaults(state, frame_id);
        }
        WidgetType::GameTooltip => {
            create_tooltip_defaults(state, frame_id);
        }
        WidgetType::SimpleHTML => {
            state.simple_htmls.insert(
                frame_id,
                crate::lua_api::simple_html::SimpleHtmlData::default(),
            );
        }
        WidgetType::MessageFrame => {
            state.message_frames.insert(
                frame_id,
                crate::lua_api::message_frame::MessageFrameData::default(),
            );
        }
        WidgetType::Slider => {
            create_slider_defaults(lua, state, frame_id);
        }
        WidgetType::EditBox => {
            if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
                frame.mouse_enabled = true;
                frame.visible = false;
            }
        }
        _ => {}
    }
}

/// Create intrinsic children for ItemButton (from WoW's intrinsic="true" template).
/// ItemButton defines: icon (Texture), Count (FontString), Stock (FontString),
/// searchOverlay, ItemContextOverlay, IconBorder, IconOverlay, IconOverlay2 (Textures).
pub(super) fn create_item_button_intrinsics(lua: &Lua, state: &mut SimState, frame_id: u64) {
    let children = ItemButtonChildren {
        icon_id: create_item_button_icon(state, frame_id),
        count_id: create_item_button_count(state, frame_id),
        stock_id: create_hidden_artwork_fontstring(state, frame_id),
        icon_border_id: create_hidden_overlay(state, frame_id),
        icon_overlay_id: create_hidden_overlay(state, frame_id),
        icon_overlay2_id: create_hidden_overlay(state, frame_id),
        search_overlay_id: create_fill_parent_overlay(state, frame_id),
        context_overlay_id: create_hidden_overlay(state, frame_id),
    };
    register_item_button_children(lua, state, frame_id, &children);
}

/// Initialize Button/CheckButton defaults (no child widgets created).
///
/// WoW creates button texture/text children lazily via SetNormalTexture,
/// SetText, etc. — not at button creation time. See `get_or_create_button_texture`
/// and `apply_set_button_texture` for lazy creation.
fn create_button_defaults(state: &mut SimState, frame_id: u64) {
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.mouse_enabled = true;
    }
}

/// Create default tooltip state and set TOOLTIP strata.
fn create_tooltip_defaults(state: &mut SimState, frame_id: u64) {
    state
        .tooltips
        .insert(frame_id, crate::lua_api::tooltip::TooltipData::default());
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.frame_strata = crate::widget::FrameStrata::Tooltip;
        frame.has_fixed_frame_strata = true;
    }
}

/// Create default fontstrings and thumb texture for Slider.
fn create_slider_defaults(lua: &Lua, state: &mut SimState, frame_id: u64) {
    let low_id = create_child_widget(state, WidgetType::FontString, frame_id);
    let high_id = create_child_widget(state, WidgetType::FontString, frame_id);
    let text_id = create_child_widget(state, WidgetType::FontString, frame_id);
    let thumb_id = create_child_widget(state, WidgetType::Texture, frame_id);

    if let Some(slider) = state.widgets.get_mut_visual(frame_id) {
        slider.children_keys.insert("Low".to_string(), low_id);
        slider.children_keys.insert("High".to_string(), high_id);
        slider.children_keys.insert("Text".to_string(), text_id);
        slider
            .children_keys
            .insert("ThumbTexture".to_string(), thumb_id);
    }
    let _ = sync_child_to_lua(lua, frame_id, "Low", low_id);
    let _ = sync_child_to_lua(lua, frame_id, "High", high_id);
    let _ = sync_child_to_lua(lua, frame_id, "Text", text_id);
    let _ = sync_child_to_lua(lua, frame_id, "ThumbTexture", thumb_id);
}

/// Create a child widget of the given type, register it, and add it as a child. Returns the ID.
fn create_child_widget(state: &mut SimState, widget_type: WidgetType, parent_id: u64) -> u64 {
    let child = Frame::new(widget_type, None, Some(parent_id));
    let child_id = child.id;
    state.widgets.register(child);
    state.widgets.add_child(parent_id, child_id);
    let parent_props = state
        .widgets
        .get(parent_id)
        .map(|parent| (parent.frame_strata, parent.frame_level));
    if let Some((parent_strata, parent_level)) = parent_props
        && let Some(frame) = state.widgets.get_mut_visual(child_id)
    {
        frame.frame_strata = parent_strata;
        frame.frame_level = parent_level + 1;
    }
    child_id
}

/// Add TOPLEFT+BOTTOMRIGHT anchors to fill the parent (equivalent to SetAllPoints).
fn add_fill_parent_anchors(frame: &mut Frame, parent_id: u64) {
    use crate::widget::{Anchor, AnchorPoint};

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

fn create_item_button_icon(state: &mut SimState, frame_id: u64) -> u64 {
    let id = create_child_widget(state, WidgetType::Texture, frame_id);
    if let Some(texture) = state.widgets.get_mut_visual(id) {
        texture.draw_layer = crate::widget::DrawLayer::Border;
        add_fill_parent_anchors(texture, frame_id);
    }
    id
}

fn create_item_button_count(state: &mut SimState, frame_id: u64) -> u64 {
    let id = create_child_widget(state, WidgetType::FontString, frame_id);
    if let Some(font_string) = state.widgets.get_mut_visual(id) {
        font_string.draw_layer = crate::widget::DrawLayer::Artwork;
        font_string.visible = false;
        font_string.justify_h = crate::widget::TextJustify::Right;
        font_string.anchors.push(crate::widget::Anchor {
            point: crate::widget::AnchorPoint::BottomRight,
            relative_to: None,
            relative_to_id: Some(frame_id as usize),
            relative_point: crate::widget::AnchorPoint::BottomRight,
            x_offset: -5.0,
            y_offset: -2.0,
        });
    }
    id
}

fn create_hidden_artwork_fontstring(state: &mut SimState, frame_id: u64) -> u64 {
    let id = create_child_widget(state, WidgetType::FontString, frame_id);
    if let Some(font_string) = state.widgets.get_mut_visual(id) {
        font_string.draw_layer = crate::widget::DrawLayer::Artwork;
        font_string.visible = false;
    }
    id
}

fn create_fill_parent_overlay(state: &mut SimState, frame_id: u64) -> u64 {
    let id = create_child_widget(state, WidgetType::Texture, frame_id);
    if let Some(texture) = state.widgets.get_mut_visual(id) {
        texture.draw_layer = crate::widget::DrawLayer::Overlay;
        texture.visible = false;
        add_fill_parent_anchors(texture, frame_id);
    }
    id
}

fn register_item_button_children(
    lua: &Lua,
    state: &mut SimState,
    frame_id: u64,
    children: &ItemButtonChildren,
) {
    let child_pairs = children.as_pairs();
    if let Some(button) = state.widgets.get_mut_visual(frame_id) {
        for (key, child_id) in child_pairs {
            button.children_keys.insert(key.to_string(), child_id);
        }
    }
    for (key, child_id) in child_pairs {
        let _ = sync_child_to_lua(lua, frame_id, key, child_id);
    }
}

struct ItemButtonChildren {
    icon_id: u64,
    count_id: u64,
    stock_id: u64,
    icon_border_id: u64,
    icon_overlay_id: u64,
    icon_overlay2_id: u64,
    search_overlay_id: u64,
    context_overlay_id: u64,
}

impl ItemButtonChildren {
    fn as_pairs(&self) -> [(&'static str, u64); 8] {
        [
            ("icon", self.icon_id),
            ("Count", self.count_id),
            ("Stock", self.stock_id),
            ("IconBorder", self.icon_border_id),
            ("IconOverlay", self.icon_overlay_id),
            ("IconOverlay2", self.icon_overlay2_id),
            ("searchOverlay", self.search_overlay_id),
            ("ItemContextOverlay", self.context_overlay_id),
        ]
    }
}

/// Create a hidden overlay texture child (OVERLAY layer, hidden, centered on parent).
fn create_hidden_overlay(state: &mut SimState, parent_id: u64) -> u64 {
    let id = create_child_widget(state, WidgetType::Texture, parent_id);
    if let Some(texture) = state.widgets.get_mut_visual(id) {
        texture.draw_layer = crate::widget::DrawLayer::Overlay;
        texture.visible = false;
        texture.anchors.push(crate::widget::Anchor {
            point: crate::widget::AnchorPoint::Center,
            relative_to: None,
            relative_to_id: Some(parent_id as usize),
            relative_point: crate::widget::AnchorPoint::Center,
            x_offset: 0.0,
            y_offset: 0.0,
        });
    }
    id
}
