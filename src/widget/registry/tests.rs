use super::*;
use crate::widget::{Frame, FrameStrata, WidgetType};

fn frame(id: u64, widget_type: WidgetType, parent_id: Option<u64>, strata: FrameStrata) -> Frame {
    Frame {
        id,
        widget_type,
        parent_id,
        frame_strata: strata,
        ..Default::default()
    }
}

#[test]
fn take_render_dirty_batch_drains_ids_and_sources() {
    let mut registry = WidgetRegistry::default();
    registry.register(frame(1, WidgetType::Frame, None, FrameStrata::High));
    registry.register(frame(2, WidgetType::Texture, Some(1), FrameStrata::Low));
    registry.add_child(1, 2);

    registry.set_render_dirty_source(Some(RenderDirtySource {
        frame_id: 42,
        method: "TestMethod",
    }));
    registry.mark_visual_dirty(2);
    registry.set_render_dirty_source(None);

    let batch = registry.take_render_dirty_batch();

    assert_eq!(batch.strata_mask, 1u16 << FrameStrata::High.as_index());
    assert_eq!(batch.frame_ids, Some(FxHashSet::from_iter([2])));
    assert_eq!(
        batch.sources.get(&2),
        Some(&FxHashSet::from_iter([RenderDirtySource {
            frame_id: 42,
            method: "TestMethod",
        }]))
    );
    assert!(!registry.has_dirty_frames());
    assert!(registry.take_render_dirty_batch().sources.is_empty());
}

#[test]
fn take_render_dirty_batch_returns_none_ids_for_full_rebuild_sentinel() {
    let registry = WidgetRegistry::default();
    registry.set_render_dirty_source(Some(RenderDirtySource {
        frame_id: 99,
        method: "MarkAll",
    }));
    registry.mark_all_visual_dirty();
    registry.set_render_dirty_source(None);

    let batch = registry.take_render_dirty_batch();
    let all_mask = (1u16 << FrameStrata::COUNT) - 1;

    assert_eq!(batch.strata_mask, all_mask);
    assert_eq!(batch.frame_ids, None);
    assert!(!registry.has_dirty_frames());
}

#[test]
fn visible_texture_paths_ignores_children_under_hidden_parents() {
    let mut registry = WidgetRegistry::default();
    let mut parent = frame(1, WidgetType::Frame, None, FrameStrata::High);
    parent.visible = false;
    let mut child = frame(2, WidgetType::Texture, Some(1), FrameStrata::High);
    child.visible = true;
    child.texture = Some("Interface/WorldMap/HiddenTile".to_string());

    registry.register(parent);
    registry.register(child);
    registry.add_child(1, 2);

    assert!(
        registry.visible_texture_paths().is_empty(),
        "hidden parent subtrees must not enter visible texture warmup"
    );
}

#[test]
fn add_child_reparents_child_and_updates_effective_alpha() {
    let mut registry = WidgetRegistry::default();

    let mut parent_a = frame(1, WidgetType::Frame, None, FrameStrata::High);
    parent_a.alpha = 1.0;
    let mut parent_b = frame(2, WidgetType::Frame, None, FrameStrata::High);
    parent_b.alpha = 0.0;
    let mut child = frame(3, WidgetType::Texture, Some(1), FrameStrata::High);
    child.alpha = 1.0;

    registry.register(parent_a);
    registry.register(parent_b);
    registry.register(child);

    registry.add_child(1, 3);
    registry.propagate_all_effective_alpha();
    assert_eq!(
        registry
            .get(3)
            .expect("child frame should exist after first parenting")
            .effective_alpha,
        1.0
    );

    registry.add_child(2, 3);
    assert_eq!(
        registry
            .get(3)
            .expect("child frame should remain registered after reparent")
            .parent_id,
        Some(2)
    );
    assert_eq!(
        registry
            .get(3)
            .expect("child frame should keep updated effective alpha")
            .effective_alpha,
        0.0
    );
    assert!(
        !registry
            .get(1)
            .expect("old parent should remain registered")
            .children
            .contains(&3),
        "child should be detached from old parent"
    );

    registry.add_child(2, 3);
    let duplicates = registry
        .get(2)
        .expect("new parent should remain registered")
        .children
        .iter()
        .filter(|&&id| id == 3)
        .count();
    assert_eq!(duplicates, 1, "add_child should not duplicate child IDs");
}

#[test]
fn propagate_all_updates_rooted_effective_scale() {
    let mut registry = WidgetRegistry::default();

    let mut parent = frame(1, WidgetType::Frame, None, FrameStrata::High);
    parent.scale = 2.0;
    let mut child = frame(2, WidgetType::Texture, Some(1), FrameStrata::High);
    child.scale = 0.5;

    registry.register(parent);
    registry.register(child);
    registry.add_child(1, 2);
    registry.propagate_all_effective_scale();

    assert_eq!(
        registry
            .get(2)
            .expect("child frame should exist")
            .effective_scale,
        1.0
    );
}

#[test]
fn add_child_repairs_missing_parent_child_entry() {
    let mut registry = WidgetRegistry::default();

    registry.register(frame(1, WidgetType::Frame, None, FrameStrata::High));
    registry.register(frame(2, WidgetType::Texture, Some(1), FrameStrata::High));

    registry.add_child(1, 2);

    assert_eq!(
        registry
            .get(1)
            .expect("parent should remain registered")
            .children,
        vec![2],
        "add_child should repair parent child lists when parent_id is already set"
    );
}

#[test]
fn late_resolve_anchor_with_parent_prefix_suffix() {
    // Reproduces the Communities Guild Info panel bug: a child frame is
    // anchored to `$parentHeader2` but Header2 (a layer Texture) is
    // created AFTER the child frame, so initial resolution stores the
    // unresolved expression. The late-bind retry must substitute the
    // $parent prefix and look the resulting name up in the names index.
    let mut registry = WidgetRegistry::default();

    let mut info = frame(1, WidgetType::Frame, None, FrameStrata::Medium);
    info.name = Some("InfoPanel".to_string());
    registry.register(info);

    let mut child = frame(2, WidgetType::ScrollFrame, Some(1), FrameStrata::Medium);
    child.set_point_with_name(
        crate::widget::AnchorPoint::TopLeft,
        Some("$parentHeader2".to_string()),
        crate::widget::AnchorPoint::BottomLeft,
        14.0,
        -1.0,
    );
    registry.register(child);
    registry.add_child(1, 2);

    // At this point Header2 doesn't exist yet — late-bind would fail.
    registry.resolve_named_anchor_targets_for_frame(2);
    assert!(
        registry
            .get(2)
            .unwrap()
            .anchors
            .first()
            .unwrap()
            .relative_to_id
            .is_none(),
        "anchor must remain unresolved before Header2 is registered"
    );

    // Now create Header2 with the substituted name.
    let mut header2 = frame(3, WidgetType::Texture, Some(1), FrameStrata::Medium);
    header2.name = Some("InfoPanelHeader2".to_string());
    registry.register(header2);
    registry.add_child(1, 3);

    // Late-bind retry must now find Header2 via $parent substitution.
    registry.resolve_named_anchor_targets_for_frame(2);
    assert_eq!(
        registry
            .get(2)
            .unwrap()
            .anchors
            .first()
            .unwrap()
            .relative_to_id,
        Some(3),
        "late-bind must substitute $parentHeader2 to InfoPanelHeader2"
    );
}
