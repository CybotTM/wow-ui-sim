use wow_ui_sim::layout::compute_frame_rect;
use wow_ui_sim::widget::{Anchor, AnchorPoint, Frame, WidgetRegistry};

fn make_frame(
    id: u64,
    parent: Option<u64>,
    width: f32,
    height: f32,
    children: Vec<u64>,
    anchors: Vec<Anchor>,
) -> Frame {
    let mut frame = Frame {
        id,
        parent_id: parent,
        width,
        height,
        children,
        anchors,
        ..Frame::default()
    };
    if id == 1 {
        frame.name = Some("UIParent".to_string());
    }
    frame
}

#[test]
fn self_relative_anchor_uses_cycle_fallback() {
    let mut registry = WidgetRegistry::new();
    registry.register(make_frame(1, None, 1024.0, 768.0, vec![2], vec![]));
    let mut anchor = Anchor::from_relative_id(AnchorPoint::TopLeft, Some(2), AnchorPoint::TopLeft);
    anchor.x_offset = 10.0;
    anchor.y_offset = -20.0;
    registry.register(make_frame(2, Some(1), 100.0, 40.0, vec![], vec![anchor]));

    let rect = compute_frame_rect(&registry, 2, 1024.0, 768.0);

    assert_eq!(rect.x, 10.0);
    assert_eq!(rect.y, 20.0);
    assert_eq!(rect.width, 100.0);
    assert_eq!(rect.height, 40.0);
}

#[test]
fn parent_cycle_uses_cycle_fallback() {
    let mut registry = WidgetRegistry::new();
    registry.register(make_frame(2, Some(3), 100.0, 40.0, vec![3], vec![]));
    registry.register(make_frame(3, Some(2), 80.0, 20.0, vec![2], vec![]));

    let rect = compute_frame_rect(&registry, 2, 1024.0, 768.0);

    assert_eq!(rect.x, 0.0);
    assert_eq!(rect.y, 0.0);
    assert_eq!(rect.width, 100.0);
    assert_eq!(rect.height, 40.0);
}
