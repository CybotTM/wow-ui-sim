//! Layout computation helpers for frame positioning.

pub use crate::LayoutRect;
pub use crate::layout::{
    CachedFrameLayout, LayoutCache, anchor_position, frame_position_from_anchor,
};
use crate::widget::WidgetRegistry;

/// Get depth in parent hierarchy (for indentation).
pub fn get_parent_depth(registry: &WidgetRegistry, id: u64) -> usize {
    let mut depth = 0;
    let mut current = id;
    while let Some(frame) = registry.get(current) {
        if let Some(parent_id) = frame.parent_id {
            depth += 1;
            current = parent_id;
        } else {
            break;
        }
    }
    depth
}

/// Compute frame rect using the same layout path as the renderer.
pub fn compute_frame_rect(
    registry: &WidgetRegistry,
    id: u64,
    screen_width: f32,
    screen_height: f32,
) -> LayoutRect {
    crate::layout::compute_frame_rect(registry, id, screen_width, screen_height)
}
