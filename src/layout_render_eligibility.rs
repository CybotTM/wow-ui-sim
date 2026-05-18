use crate::widget::WidgetRegistry;

pub fn frame_has_layout_anchor(
    registry: &WidgetRegistry,
    frame: &crate::widget::Frame,
    id: u64,
) -> bool {
    is_root_screen_frame(frame, id)
        || !frame.anchors.is_empty()
        || is_statusbar_bar_child(registry, frame)
        || is_scroll_frame_child(registry, frame)
}

pub fn frame_has_render_layout(registry: &WidgetRegistry, id: u64) -> bool {
    let mut current_id = id;
    let mut is_target = true;
    for _ in 0..64 {
        let Some(frame) = registry.get(current_id) else {
            return false;
        };
        if !frame_has_layout_anchor(registry, frame, current_id)
            && (is_target || frame.layout_rect.is_none())
        {
            return false;
        }
        let Some(parent_id) = frame.parent_id else {
            return true;
        };
        current_id = parent_id;
        is_target = false;
    }
    false
}

fn is_root_screen_frame(frame: &crate::widget::Frame, id: u64) -> bool {
    frame.name.as_deref() == Some("UIParent") || (frame.parent_id.is_none() && id == 1)
}

fn is_statusbar_bar_child(registry: &WidgetRegistry, frame: &crate::widget::Frame) -> bool {
    let Some(parent_id) = frame.parent_id else {
        return false;
    };
    let Some(parent) = registry.get(parent_id) else {
        return false;
    };
    parent.statusbar_bar_id == Some(frame.id)
}

fn is_scroll_frame_child(registry: &WidgetRegistry, frame: &crate::widget::Frame) -> bool {
    let Some(parent_id) = frame.parent_id else {
        return false;
    };
    let Some(parent) = registry.get(parent_id) else {
        return false;
    };
    parent.scroll_child_id == Some(frame.id)
}
