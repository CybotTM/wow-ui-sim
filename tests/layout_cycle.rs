use wow_ui_sim::layout::{LayoutCache, compute_frame_rect, compute_frame_rect_cached};
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

#[test]
fn mutual_anchor_cycle_preserves_entry_order_and_cached_results() {
    let mut registry = WidgetRegistry::new();
    registry.register(make_frame(1, None, 1024.0, 768.0, vec![2, 3], vec![]));
    let mut a = Anchor::from_relative_id(AnchorPoint::TopLeft, Some(3), AnchorPoint::TopLeft);
    a.x_offset = 3.0;
    let mut b = Anchor::from_relative_id(AnchorPoint::TopLeft, Some(2), AnchorPoint::TopLeft);
    b.x_offset = 5.0;
    registry.register(make_frame(2, Some(1), 20.0, 10.0, vec![], vec![a]));
    registry.register(make_frame(3, Some(1), 30.0, 15.0, vec![], vec![b]));

    // Each recursive edge observes a zero rectangle, not a partially resolved frame.
    for (first, expected_a, expected_b) in [(2, 8.0, 5.0), (3, 3.0, 8.0)] {
        let mut cache = LayoutCache::default();
        compute_frame_rect_cached(&registry, first, 1024.0, 768.0, &mut cache);
        for _ in 0..2 {
            let a = compute_frame_rect_cached(&registry, 2, 1024.0, 768.0, &mut cache);
            let b = compute_frame_rect_cached(&registry, 3, 1024.0, 768.0, &mut cache);
            assert_eq!(
                (a.rect.x, a.rect.y, a.rect.width, a.rect.height),
                (expected_a, 0.0, 20.0, 10.0)
            );
            assert_eq!(
                (b.rect.x, b.rect.y, b.rect.width, b.rect.height),
                (expected_b, 0.0, 30.0, 15.0)
            );
            assert_eq!((a.eff_scale, b.eff_scale), (1.0, 1.0));
        }
    }
}

#[test]
fn self_anchor_recomputes_after_cache_removal_and_missing_frames_stay_zero() {
    let mut registry = WidgetRegistry::new();
    registry.register(make_frame(1, None, 1024.0, 768.0, vec![2], vec![]));
    let mut anchor = Anchor::from_relative_id(AnchorPoint::TopLeft, Some(2), AnchorPoint::TopLeft);
    anchor.x_offset = 10.0;
    registry.register(make_frame(2, Some(1), 100.0, 40.0, vec![], vec![anchor]));
    let mut cache = LayoutCache::default();
    let before = compute_frame_rect_cached(&registry, 2, 1024.0, 768.0, &mut cache);
    assert_eq!((before.rect.x, before.rect.width), (10.0, 100.0));

    let frame = registry.get_mut(2).unwrap();
    frame.anchors[0].x_offset = 17.0;
    frame.width = 120.0;
    cache.remove(&2);
    let after = compute_frame_rect_cached(&registry, 2, 1024.0, 768.0, &mut cache);
    assert_eq!(
        (
            after.rect.x,
            after.rect.y,
            after.rect.width,
            after.rect.height
        ),
        (17.0, 0.0, 120.0, 40.0)
    );
    assert_eq!(after.eff_scale, 1.0);

    for _ in 0..2 {
        let missing = compute_frame_rect_cached(&registry, 999, 1024.0, 768.0, &mut cache);
        assert_eq!(
            (
                missing.rect.x,
                missing.rect.y,
                missing.rect.width,
                missing.rect.height
            ),
            (0.0, 0.0, 0.0, 0.0)
        );
        assert_eq!(missing.eff_scale, 1.0);
    }
}
