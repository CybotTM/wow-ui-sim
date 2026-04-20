//! Layout computation helpers for WoW frame positioning.

use rustc_hash::FxHashMap;

#[path = "iced_app/layout_line.rs"]
mod layout_line;

use crate::LayoutRect;
use crate::widget::{AnchorPoint, WidgetRegistry, WidgetType};

use layout_line::resolve_line_frame_rect;

/// Cached layout result: computed rect + effective scale.
#[derive(Clone, Copy)]
pub struct CachedFrameLayout {
    pub rect: LayoutRect,
    pub eff_scale: f32,
}

/// Memoization cache for frame layout computation.
///
/// Each frame is computed at most once per cache lifetime; siblings share
/// the cached parent result instead of redundantly walking the parent chain.
/// `FxHashMap` keeps the lookup cost low — `u64` frame IDs don't need a
/// DOS-resistant hash, and the default siphash dominated the layout
/// profile (~1% of total wall time).
pub type LayoutCache = FxHashMap<u64, CachedFrameLayout>;

/// Resolved edge constraints from multiple anchors.
struct AnchorEdges {
    left_x: Option<f32>,
    right_x: Option<f32>,
    top_y: Option<f32>,
    bottom_y: Option<f32>,
    center_x: Option<f32>,
    center_y: Option<f32>,
}

/// Resolve each anchor in a multi-anchor frame to edge constraints.
fn resolve_multi_anchor_edges(
    registry: &WidgetRegistry,
    frame: &crate::widget::Frame,
    parent_rect: LayoutRect,
    eff_scale: f32,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> AnchorEdges {
    let mut edges = empty_anchor_edges();

    for anchor in &frame.anchors {
        apply_multi_anchor_edge(
            &mut edges,
            registry,
            anchor,
            parent_rect,
            eff_scale,
            screen_width,
            screen_height,
            cache,
        );
    }

    edges
}

fn empty_anchor_edges() -> AnchorEdges {
    AnchorEdges {
        left_x: None,
        right_x: None,
        top_y: None,
        bottom_y: None,
        center_x: None,
        center_y: None,
    }
}

fn resolve_multi_anchor_relative_rect(
    registry: &WidgetRegistry,
    anchor: &crate::widget::Anchor,
    parent_rect: LayoutRect,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> LayoutRect {
    if let Some(rel_id) = anchor.relative_to_id {
        return compute_frame_rect_cached(
            registry,
            rel_id as u64,
            screen_width,
            screen_height,
            cache,
        )
        .rect;
    }
    parent_rect
}

fn apply_multi_anchor_edge(
    edges: &mut AnchorEdges,
    registry: &WidgetRegistry,
    anchor: &crate::widget::Anchor,
    parent_rect: LayoutRect,
    eff_scale: f32,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) {
    let relative_rect = resolve_multi_anchor_relative_rect(
        registry,
        anchor,
        parent_rect,
        screen_width,
        screen_height,
        cache,
    );
    let target = resolve_anchor_target(anchor, relative_rect, eff_scale);
    apply_anchor_target(edges, anchor.point, target);
}

fn resolve_anchor_target(
    anchor: &crate::widget::Anchor,
    relative_rect: LayoutRect,
    eff_scale: f32,
) -> (f32, f32) {
    let (anchor_x, anchor_y) = anchor_position(
        anchor.relative_point,
        relative_rect.x,
        relative_rect.y,
        relative_rect.width,
        relative_rect.height,
    );
    (
        anchor_x + anchor.x_offset * eff_scale,
        anchor_y - anchor.y_offset * eff_scale,
    )
}

fn apply_anchor_target(edges: &mut AnchorEdges, point: AnchorPoint, target: (f32, f32)) {
    let (target_x, target_y) = target;
    match point {
        AnchorPoint::TopLeft => {
            edges.left_x = Some(target_x);
            edges.top_y = Some(target_y);
        }
        AnchorPoint::TopRight => {
            edges.right_x = Some(target_x);
            edges.top_y = Some(target_y);
        }
        AnchorPoint::BottomLeft => {
            edges.left_x = Some(target_x);
            edges.bottom_y = Some(target_y);
        }
        AnchorPoint::BottomRight => {
            edges.right_x = Some(target_x);
            edges.bottom_y = Some(target_y);
        }
        AnchorPoint::Top => {
            edges.top_y = Some(target_y);
            edges.center_x = Some(target_x);
        }
        AnchorPoint::Bottom => {
            edges.bottom_y = Some(target_y);
            edges.center_x = Some(target_x);
        }
        AnchorPoint::Left => {
            edges.left_x = Some(target_x);
            edges.center_y = Some(target_y);
        }
        AnchorPoint::Right => {
            edges.right_x = Some(target_x);
            edges.center_y = Some(target_y);
        }
        AnchorPoint::Center => {
            edges.center_x = Some(target_x);
            edges.center_y = Some(target_y);
        }
    }
}

fn compute_rect_from_edges(
    edges: AnchorEdges,
    frame: &crate::widget::Frame,
    parent_rect: LayoutRect,
    scale: f32,
) -> LayoutRect {
    let horizontal = resolve_axis_layout(
        edges.left_x,
        edges.right_x,
        edges.center_x,
        frame.width,
        scale,
        parent_rect.x,
        parent_rect.width,
    );
    let vertical = resolve_axis_layout(
        edges.top_y,
        edges.bottom_y,
        edges.center_y,
        frame.height,
        scale,
        parent_rect.y,
        parent_rect.height,
    );

    LayoutRect {
        x: horizontal.position,
        y: vertical.position,
        width: horizontal.size,
        height: vertical.size,
    }
}

struct AxisLayout {
    position: f32,
    size: f32,
}

fn resolve_axis_layout(
    start: Option<f32>,
    end: Option<f32>,
    center: Option<f32>,
    explicit_size: f32,
    scale: f32,
    parent_start: f32,
    parent_size: f32,
) -> AxisLayout {
    let (start, end) = normalize_bounds(start, end);
    let size = resolve_axis_size(start, end, explicit_size, scale);
    let position = resolve_axis_position(start, end, center, size, parent_start, parent_size);
    AxisLayout { position, size }
}

fn normalize_bounds(start: Option<f32>, end: Option<f32>) -> (Option<f32>, Option<f32>) {
    match (start, end) {
        (Some(a), Some(b)) if a > b => (Some(b), Some(a)),
        _ => (start, end),
    }
}

fn resolve_axis_size(start: Option<f32>, end: Option<f32>, explicit_size: f32, scale: f32) -> f32 {
    match (start, end) {
        (Some(start), Some(end)) => end - start,
        _ if explicit_size > 0.0 => explicit_size * scale,
        _ => 0.0,
    }
}

fn resolve_axis_position(
    start: Option<f32>,
    end: Option<f32>,
    center: Option<f32>,
    size: f32,
    parent_start: f32,
    parent_size: f32,
) -> f32 {
    start
        .or_else(|| end.map(|end| end - size))
        .or_else(|| center.map(|center| center - size / 2.0))
        .unwrap_or_else(|| parent_start + (parent_size - size) / 2.0)
}

fn resolve_single_anchor(
    registry: &WidgetRegistry,
    frame: &crate::widget::Frame,
    parent_rect: LayoutRect,
    eff_scale: f32,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> LayoutRect {
    let anchor = &frame.anchors[0];
    let width = frame.width * eff_scale;
    let height = frame.height * eff_scale;

    let relative_rect = if let Some(rel_id) = anchor.relative_to_id {
        compute_frame_rect_cached(registry, rel_id as u64, screen_width, screen_height, cache).rect
    } else {
        parent_rect
    };

    let (anchor_x, anchor_y) = anchor_position(
        anchor.relative_point,
        relative_rect.x,
        relative_rect.y,
        relative_rect.width,
        relative_rect.height,
    );

    let target_x = anchor_x + anchor.x_offset * eff_scale;
    let target_y = anchor_y - anchor.y_offset * eff_scale;

    let (frame_x, frame_y) =
        frame_position_from_anchor(anchor.point, target_x, target_y, width, height);

    LayoutRect {
        x: frame_x,
        y: frame_y,
        width,
        height,
    }
}

pub fn compute_frame_rect_cached(
    registry: &WidgetRegistry,
    id: u64,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> CachedFrameLayout {
    if let Some(&cached) = cache.get(&id) {
        return cached;
    }

    let result = registry
        .get(id)
        .map(|frame| {
            resolve_uncached_frame_layout(registry, id, frame, screen_width, screen_height, cache)
        })
        .unwrap_or_else(missing_frame_layout);

    cache_layout_result(cache, id, result)
}

fn resolve_uncached_frame_layout(
    registry: &WidgetRegistry,
    id: u64,
    frame: &crate::widget::Frame,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> CachedFrameLayout {
    if is_root_screen_frame(frame, id) {
        return root_frame_layout(frame, screen_width, screen_height);
    }

    let parent_rect = resolve_parent_rect(registry, frame, screen_width, screen_height, cache);
    let scale = frame.effective_scale;
    let base_rect = resolve_frame_layout_rect(
        registry,
        frame,
        parent_rect,
        scale,
        screen_width,
        screen_height,
        cache,
    );
    let mut rect = apply_frame_layout_adjustments(
        base_rect,
        registry,
        frame,
        scale,
        screen_width,
        screen_height,
        cache,
    );
    maybe_clamp_frame_rect(frame, &mut rect, screen_width, screen_height);

    CachedFrameLayout {
        rect,
        eff_scale: scale,
    }
}

fn missing_frame_layout() -> CachedFrameLayout {
    CachedFrameLayout {
        rect: LayoutRect::default(),
        eff_scale: 1.0,
    }
}

fn root_frame_layout(
    frame: &crate::widget::Frame,
    screen_width: f32,
    screen_height: f32,
) -> CachedFrameLayout {
    CachedFrameLayout {
        rect: full_screen_rect(screen_width, screen_height),
        eff_scale: frame.effective_scale,
    }
}

fn cache_layout_result(
    cache: &mut LayoutCache,
    id: u64,
    result: CachedFrameLayout,
) -> CachedFrameLayout {
    cache.insert(id, result);
    result
}

fn is_root_screen_frame(frame: &crate::widget::Frame, id: u64) -> bool {
    frame.name.as_deref() == Some("UIParent") || (frame.parent_id.is_none() && id == 1)
}

fn full_screen_rect(screen_width: f32, screen_height: f32) -> LayoutRect {
    LayoutRect {
        x: 0.0,
        y: 0.0,
        width: screen_width,
        height: screen_height,
    }
}

fn resolve_parent_rect(
    registry: &WidgetRegistry,
    frame: &crate::widget::Frame,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> LayoutRect {
    frame
        .parent_id
        .map(|parent_id| {
            compute_frame_rect_cached(registry, parent_id, screen_width, screen_height, cache).rect
        })
        .unwrap_or_else(|| full_screen_rect(screen_width, screen_height))
}

fn resolve_frame_layout_rect(
    registry: &WidgetRegistry,
    frame: &crate::widget::Frame,
    parent_rect: LayoutRect,
    scale: f32,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> LayoutRect {
    if frame.anchors.is_empty() {
        return anchorless_rect(registry, frame, parent_rect, scale);
    }
    if frame.anchors.len() >= 2 {
        let edges = resolve_multi_anchor_edges(
            registry,
            frame,
            parent_rect,
            scale,
            screen_width,
            screen_height,
            cache,
        );
        return compute_rect_from_edges(edges, frame, parent_rect, scale);
    }
    resolve_single_anchor(
        registry,
        frame,
        parent_rect,
        scale,
        screen_width,
        screen_height,
        cache,
    )
}

fn anchorless_rect(
    registry: &WidgetRegistry,
    frame: &crate::widget::Frame,
    parent_rect: LayoutRect,
    scale: f32,
) -> LayoutRect {
    let width = frame.width * scale;
    let height = frame.height * scale;
    if width == 0.0 && height == 0.0 && is_statusbar_bar_child(registry, frame) {
        return parent_rect;
    }
    LayoutRect {
        x: parent_rect.x,
        y: parent_rect.y,
        width,
        height,
    }
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

fn apply_frame_layout_adjustments(
    mut rect: LayoutRect,
    registry: &WidgetRegistry,
    frame: &crate::widget::Frame,
    scale: f32,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> LayoutRect {
    rect.x += frame.anim_offset_x;
    rect.y += frame.anim_offset_y;
    if let Some(line_rect) =
        resolve_line_frame_rect(frame, registry, scale, screen_width, screen_height, cache)
    {
        return line_rect;
    }
    rect
}

fn maybe_clamp_frame_rect(
    frame: &crate::widget::Frame,
    rect: &mut LayoutRect,
    screen_width: f32,
    screen_height: f32,
) {
    if should_clamp_frame_to_screen(frame) && rect.width > 0.0 && rect.height > 0.0 {
        clamp_rect_to_screen(rect, screen_width, screen_height);
    }
}

fn should_clamp_frame_to_screen(frame: &crate::widget::Frame) -> bool {
    frame.clamped_to_screen || frame.widget_type == WidgetType::GameTooltip
}

pub fn compute_frame_rect(
    registry: &WidgetRegistry,
    id: u64,
    screen_width: f32,
    screen_height: f32,
) -> LayoutRect {
    let mut cache = LayoutCache::default();
    compute_frame_rect_cached(registry, id, screen_width, screen_height, &mut cache).rect
}

fn clamp_rect_to_screen(rect: &mut LayoutRect, screen_w: f32, screen_h: f32) {
    rect.x = clamp_axis_to_viewport(rect.x, rect.width, screen_w);
    rect.y = clamp_axis_to_viewport(rect.y, rect.height, screen_h);
}

fn clamp_axis_to_viewport(position: f32, size: f32, viewport_size: f32) -> f32 {
    if size >= viewport_size {
        0.0
    } else {
        position.clamp(0.0, viewport_size - size)
    }
}

pub fn anchor_position(point: AnchorPoint, x: f32, y: f32, w: f32, h: f32) -> (f32, f32) {
    match point {
        AnchorPoint::TopLeft => (x, y),
        AnchorPoint::Top => (x + w / 2.0, y),
        AnchorPoint::TopRight => (x + w, y),
        AnchorPoint::Left => (x, y + h / 2.0),
        AnchorPoint::Center => (x + w / 2.0, y + h / 2.0),
        AnchorPoint::Right => (x + w, y + h / 2.0),
        AnchorPoint::BottomLeft => (x, y + h),
        AnchorPoint::Bottom => (x + w / 2.0, y + h),
        AnchorPoint::BottomRight => (x + w, y + h),
    }
}

pub fn frame_position_from_anchor(
    point: AnchorPoint,
    anchor_x: f32,
    anchor_y: f32,
    w: f32,
    h: f32,
) -> (f32, f32) {
    match point {
        AnchorPoint::TopLeft => (anchor_x, anchor_y),
        AnchorPoint::Top => (anchor_x - w / 2.0, anchor_y),
        AnchorPoint::TopRight => (anchor_x - w, anchor_y),
        AnchorPoint::Left => (anchor_x, anchor_y - h / 2.0),
        AnchorPoint::Center => (anchor_x - w / 2.0, anchor_y - h / 2.0),
        AnchorPoint::Right => (anchor_x - w, anchor_y - h / 2.0),
        AnchorPoint::BottomLeft => (anchor_x, anchor_y - h),
        AnchorPoint::Bottom => (anchor_x - w / 2.0, anchor_y - h),
        AnchorPoint::BottomRight => (anchor_x - w, anchor_y - h),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::{Anchor, Frame};

    fn anchor(point: AnchorPoint, rel_id: Option<usize>, rel_point: AnchorPoint) -> Anchor {
        Anchor {
            point,
            relative_to_id: rel_id,
            relative_to: None,
            relative_point: rel_point,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }

    fn make_frame(
        id: u64,
        parent: Option<u64>,
        w: f32,
        h: f32,
        children: Vec<u64>,
        anchors: Vec<Anchor>,
    ) -> Frame {
        let mut f = Frame::default();
        f.id = id;
        f.parent_id = parent;
        f.width = w;
        f.height = h;
        f.children = children;
        f.anchors = anchors;
        f
    }

    fn build_three_slice_registry() -> WidgetRegistry {
        let mut reg = WidgetRegistry::new();
        register_ui_parent(&mut reg);
        register_button_frame(&mut reg);
        register_edge_frame(&mut reg, 20, AnchorPoint::Left);
        register_edge_frame(&mut reg, 21, AnchorPoint::Right);
        register_center_frame(&mut reg);
        reg
    }

    fn register_ui_parent(reg: &mut WidgetRegistry) {
        let mut uip = make_frame(1, None, 1024.0, 768.0, vec![10], vec![]);
        uip.name = Some("UIParent".to_string());
        reg.register(uip);
    }

    fn register_button_frame(reg: &mut WidgetRegistry) {
        reg.register(make_frame(
            10,
            Some(1),
            200.0,
            36.0,
            vec![20, 21, 22],
            vec![anchor(AnchorPoint::Center, Some(1), AnchorPoint::Center)],
        ));
    }

    fn register_edge_frame(reg: &mut WidgetRegistry, id: u64, point: AnchorPoint) {
        reg.register(make_frame(
            id,
            Some(10),
            32.0,
            39.0,
            vec![],
            vec![anchor(point, Some(10), point)],
        ));
    }

    fn register_center_frame(reg: &mut WidgetRegistry) {
        reg.register(make_frame(
            22,
            Some(10),
            0.0,
            0.0,
            vec![],
            vec![
                anchor(AnchorPoint::TopLeft, Some(20), AnchorPoint::TopRight),
                anchor(AnchorPoint::BottomRight, Some(21), AnchorPoint::BottomLeft),
            ],
        ));
    }

    #[test]
    fn test_cross_frame_anchor_center_texture() {
        let registry = build_three_slice_registry();
        let rect = compute_frame_rect(&registry, 22, 1024.0, 768.0);
        assert!(
            rect.width > 100.0,
            "Center width should be ~136, got {}",
            rect.width
        );
        assert!(
            rect.height > 30.0,
            "Center height should be ~39, got {}",
            rect.height
        );
    }

    #[test]
    fn test_nil_relative_to_anchors_multi_anchor_frame_to_screen() {
        let mut registry = WidgetRegistry::new();
        let mut root = make_frame(1, None, 1024.0, 768.0, vec![2], vec![]);
        root.name = Some("UIParent".to_string());
        registry.register(root);
        let mut frame = make_frame(
            2,
            Some(1),
            100.0,
            40.0,
            vec![],
            vec![
                anchor(AnchorPoint::TopLeft, None, AnchorPoint::TopLeft),
                anchor(AnchorPoint::BottomRight, None, AnchorPoint::BottomRight),
            ],
        );
        frame.anchors[0].x_offset = 10.0;
        frame.anchors[0].y_offset = -20.0;
        frame.anchors[1].x_offset = -30.0;
        frame.anchors[1].y_offset = 40.0;
        registry.register(frame);

        let rect = compute_frame_rect(&registry, 2, 1024.0, 768.0);
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 984.0);
        assert_eq!(rect.height, 708.0);
    }
}
