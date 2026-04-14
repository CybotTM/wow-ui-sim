//! Pure scroll-child helpers shared by loader and template code.

use crate::lua_api::frame::methods::methods_hierarchy::reparent_widget;

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
    state.visible_on_update_cache = None;
    state.invalidate_layout(child_id);
}
