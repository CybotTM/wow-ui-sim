use super::*;
use crate::widget::{Frame, FrameStrata, WidgetType};

fn test_frame(id: u64, widget_type: WidgetType, parent_id: Option<u64>, visible: bool) -> Frame {
    let mut frame = Frame {
        id,
        widget_type,
        parent_id,
        visible,
        width: 10.0,
        height: 10.0,
        layout_rect: Some(crate::LayoutRect {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        }),
        ..Default::default()
    };
    frame.effective_alpha = if visible { 1.0 } else { 0.0 };
    frame
}

fn register_child(
    state: &mut SimState,
    id: u64,
    widget_type: WidgetType,
    parent_id: u64,
    visible: bool,
) {
    state
        .widgets
        .register(test_frame(id, widget_type, Some(parent_id), visible));
    state.widgets.add_child(parent_id, id);
}

fn medium_bucket(state: &mut SimState) -> Vec<u64> {
    state
        .get_strata_buckets()
        .unwrap()
        .get(FrameStrata::Medium.as_index())
        .cloned()
        .unwrap_or_default()
}

#[test]
fn show_visible_region_repairs_parent_subtree_without_invalidating_buckets() {
    let mut state = SimState::default();
    state
        .widgets
        .register(test_frame(1, WidgetType::Frame, None, true));
    register_child(&mut state, 2, WidgetType::Texture, 1, true);
    register_child(&mut state, 3, WidgetType::Texture, 1, false);
    register_child(&mut state, 4, WidgetType::Frame, 1, true);
    register_child(&mut state, 5, WidgetType::Texture, 4, true);
    register_child(&mut state, 6, WidgetType::FontString, 1, true);

    assert_eq!(medium_bucket(&mut state), vec![1, 2, 4, 5, 6]);
    assert!(state.strata_buckets.is_some());

    state.set_frame_visible(3, true);

    assert!(state.strata_buckets.is_some());
    assert_eq!(medium_bucket(&mut state), vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn show_visible_child_frame_repairs_parent_subtree_without_invalidating_buckets() {
    let mut state = SimState::default();
    state
        .widgets
        .register(test_frame(10, WidgetType::Frame, None, true));
    register_child(&mut state, 11, WidgetType::Texture, 10, true);
    register_child(&mut state, 12, WidgetType::Frame, 10, false);
    register_child(&mut state, 13, WidgetType::Texture, 12, true);
    register_child(&mut state, 14, WidgetType::FontString, 10, true);

    assert_eq!(medium_bucket(&mut state), vec![10, 11, 14]);
    assert!(state.strata_buckets.is_some());

    state.set_frame_visible(12, true);

    assert!(state.strata_buckets.is_some());
    assert_eq!(medium_bucket(&mut state), vec![10, 11, 12, 13, 14]);
}

#[test]
fn show_root_frame_still_falls_back_to_full_invalidation() {
    let mut state = SimState::default();
    state
        .widgets
        .register(test_frame(20, WidgetType::Frame, None, false));
    let _ = medium_bucket(&mut state);

    state.set_frame_visible(20, true);

    assert!(state.strata_buckets.is_none());
}
