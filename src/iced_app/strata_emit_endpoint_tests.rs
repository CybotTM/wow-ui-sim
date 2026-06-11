use super::{SingleStrataEmit, build_render_list, emit_single_strata};
use crate::render::QuadBatch;
use crate::widget::{AnchorPoint, Frame, LineAnchor, WidgetRegistry, WidgetType};

#[test]
fn render_list_keeps_line_with_endpoint_geometry_under_unanchored_edge_frame() {
    let (mut registry, edge_id, start_id, end_id) = edge_fixture();

    let line_id = register_endpoint_line(&mut registry, edge_id, start_id, end_id);
    let render_list = build_render_list(&[line_id], &registry, (1024.0, 768.0));

    assert_eq!(render_list.len(), 1);
    assert_eq!(render_list[0].0, line_id);
    assert_line_emits_quad(&registry, line_id);
}

#[test]
fn render_list_keeps_child_anchored_to_non_parent_under_unanchored_edge_frame() {
    let (mut registry, edge_id, _, end_id) = edge_fixture();

    let arrowhead_id = register_arrowhead(&mut registry, edge_id, end_id);
    let render_list = build_render_list(&[arrowhead_id], &registry, (1024.0, 768.0));

    assert_eq!(render_list.len(), 1);
    assert_eq!(render_list[0].0, arrowhead_id);
}

fn edge_fixture() -> (WidgetRegistry, u64, u64, u64) {
    let mut registry = WidgetRegistry::new();
    let root_id = register_root(&mut registry);
    let edge_id = register_unanchored_edge_parent(&mut registry, root_id);
    let start_id = register_endpoint_button(&mut registry, root_id, "Start", 100.0, 100.0);
    let end_id = register_endpoint_button(&mut registry, root_id, "End", 200.0, 140.0);
    (registry, edge_id, start_id, end_id)
}

fn register_endpoint_line(
    registry: &mut WidgetRegistry,
    edge_id: u64,
    start_id: u64,
    end_id: u64,
) -> u64 {
    let mut line = Frame::new(WidgetType::Line, None, Some(edge_id));
    line.line_start = Some(line_anchor(start_id));
    line.line_end = Some(line_anchor(end_id));
    let line_id = line.id;
    registry.register(line);
    registry.add_child(edge_id, line_id);
    line_id
}

fn line_anchor(target_id: u64) -> LineAnchor {
    LineAnchor {
        point: AnchorPoint::Center,
        target_id: Some(target_id),
        x_offset: 0.0,
        y_offset: 0.0,
    }
}

fn register_arrowhead(registry: &mut WidgetRegistry, edge_id: u64, end_id: u64) -> u64 {
    let mut arrowhead = Frame::new(WidgetType::Texture, None, Some(edge_id));
    arrowhead.set_size(16.0, 16.0);
    arrowhead.set_point(
        AnchorPoint::Center,
        Some(end_id as usize),
        AnchorPoint::Center,
        0.0,
        0.0,
    );
    let arrowhead_id = arrowhead.id;
    registry.register(arrowhead);
    registry.add_child(edge_id, arrowhead_id);
    arrowhead_id
}

fn assert_line_emits_quad(registry: &WidgetRegistry, line_id: u64) {
    let mut batch = QuadBatch::new();
    let mut text_ctx = None;
    let visible_ids = None;
    emit_single_strata(
        &mut batch,
        &mut text_ctx,
        SingleStrataEmit {
            bucket: &[line_id],
            registry,
            visible_ids: &visible_ids,
            screen_size: (1024.0, 768.0),
            pressed_frame: None,
            hovered_frame: None,
            message_frames: None,
            tooltip_data: None,
            quest_blobs: None,
            elapsed_secs: 0.0,
        },
    );

    assert_eq!(batch.vertices.len(), 4);
    assert_eq!(batch.indices.len(), 6);
}

fn register_root(registry: &mut WidgetRegistry) -> u64 {
    let mut root = Frame::new(WidgetType::Frame, Some("UIParent".to_string()), None);
    root.layout_rect = Some(crate::LayoutRect {
        x: 0.0,
        y: 0.0,
        width: 1024.0,
        height: 768.0,
    });
    let root_id = root.id;
    registry.register(root);
    root_id
}

fn register_unanchored_edge_parent(registry: &mut WidgetRegistry, root_id: u64) -> u64 {
    let edge = Frame::new(WidgetType::Frame, None, Some(root_id));
    let edge_id = edge.id;
    registry.register(edge);
    registry.add_child(root_id, edge_id);
    edge_id
}

fn register_endpoint_button(
    registry: &mut WidgetRegistry,
    root_id: u64,
    name: &str,
    x: f32,
    y: f32,
) -> u64 {
    let mut button = Frame::new(WidgetType::Button, Some(name.to_string()), Some(root_id));
    button.layout_rect = Some(crate::LayoutRect {
        x,
        y,
        width: 40.0,
        height: 40.0,
    });
    button.set_point(
        AnchorPoint::TopLeft,
        Some(root_id as usize),
        AnchorPoint::TopLeft,
        x,
        -y,
    );
    let button_id = button.id;
    registry.register(button);
    registry.add_child(root_id, button_id);
    button_id
}
