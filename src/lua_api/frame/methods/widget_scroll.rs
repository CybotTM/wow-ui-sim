//! Pure scroll-child helpers shared by loader and template code.

use crate::lua_api::frame::methods::methods_hierarchy::reparent_widget;
use crate::widget::AnchorPoint;

pub(crate) fn assign_scroll_child(
    state: &mut crate::lua_api::SimState,
    parent_id: u64,
    child_id: u64,
    should_reparent: bool,
) {
    if let Some(frame) = state.widgets.get_mut_visual(parent_id) {
        frame.scroll_child_id = Some(child_id);
        frame.scroll_child_rect_size = None;
    }
    if should_reparent {
        reparent_widget(&mut state.widgets, child_id, Some(parent_id));
    }
    anchor_scroll_child_to_parent_if_needed(state, parent_id, child_id);
    state.visible_on_update_cache = None;
    state.invalidate_layout(child_id);
}

fn anchor_scroll_child_to_parent_if_needed(
    state: &mut crate::lua_api::SimState,
    parent_id: u64,
    child_id: u64,
) {
    let needs_anchor = state
        .widgets
        .get(child_id)
        .is_some_and(|child| child.anchors.is_empty());
    if !needs_anchor {
        return;
    }

    if let Some(child) = state.widgets.get_mut_visual(child_id) {
        child.set_point(
            AnchorPoint::TopLeft,
            Some(parent_id as usize),
            AnchorPoint::TopLeft,
            0.0,
            0.0,
        );
    }
    state.widgets.add_anchor_dependent(parent_id, child_id);
}
