//! Layout computation helpers for WoW frame positioning.

use std::collections::HashMap;

use crate::LayoutRect;
use crate::widget::{AnchorPoint, LineAnchor, WidgetRegistry, WidgetType};

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
pub type LayoutCache = HashMap<u64, CachedFrameLayout>;

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
    _parent_rect: LayoutRect,
    eff_scale: f32,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> AnchorEdges {
    let mut edges = empty_anchor_edges();

    for anchor in &frame.anchors {
        let relative_rect = resolve_multi_anchor_relative_rect(
            registry,
            anchor,
            screen_width,
            screen_height,
            cache,
        );
        let target = resolve_anchor_target(anchor, relative_rect, eff_scale);
        apply_anchor_target(&mut edges, anchor.point, target);
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
    // nil relativeTo in SetPoint anchors to the screen (UIParent), not the parent frame.
    // XML anchors with no relativeTo are resolved to an explicit parent_id before storage,
    // so they never reach this branch with None.
    LayoutRect {
        x: 0.0,
        y: 0.0,
        width: screen_width,
        height: screen_height,
    }
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

/// Compute final rect from resolved edge constraints and frame size.
///
/// WoW behavior: anchors defining opposite edges override explicit size.
/// When anchors create inverted bounds, WoW swaps them to get positive dimensions.
fn compute_rect_from_edges(
    edges: AnchorEdges,
    frame: &crate::widget::Frame,
    parent_rect: LayoutRect,
    scale: f32,
) -> LayoutRect {
    let (left_x, right_x) = normalize_bounds(edges.left_x, edges.right_x);
    let (top_y, bottom_y) = normalize_bounds(edges.top_y, edges.bottom_y);
    let width = resolve_axis_size(left_x, right_x, frame.width, scale);
    let height = resolve_axis_size(top_y, bottom_y, frame.height, scale);
    let x = resolve_axis_position(
        left_x,
        right_x,
        edges.center_x,
        width,
        parent_rect.x,
        parent_rect.width,
    );
    let y = resolve_axis_position(
        top_y,
        bottom_y,
        edges.center_y,
        height,
        parent_rect.y,
        parent_rect.height,
    );

    LayoutRect {
        x,
        y,
        width,
        height,
    }
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

/// Resolve a single-anchor frame's position.
fn resolve_single_anchor(
    registry: &WidgetRegistry,
    frame: &crate::widget::Frame,
    _parent_rect: LayoutRect,
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
        // nil relativeTo in SetPoint anchors to the screen (UIParent), not the parent frame.
        LayoutRect {
            x: 0.0,
            y: 0.0,
            width: screen_width,
            height: screen_height,
        }
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

/// Compute frame rect with memoization. Each frame is computed at most once
/// per cache lifetime; parent results are reused by siblings.
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

    let frame = match registry.get(id) {
        Some(f) => f,
        None => {
            let result = CachedFrameLayout {
                rect: LayoutRect::default(),
                eff_scale: 1.0,
            };
            cache.insert(id, result);
            return result;
        }
    };

    // Special case: UIParent (id=1) fills the entire screen
    if frame.name.as_deref() == Some("UIParent") || (frame.parent_id.is_none() && id == 1) {
        let result = CachedFrameLayout {
            rect: LayoutRect {
                x: 0.0,
                y: 0.0,
                width: screen_width,
                height: screen_height,
            },
            eff_scale: frame.effective_scale,
        };
        cache.insert(id, result);
        return result;
    }

    // Compute parent layout (cache hit for siblings)
    let parent_rect = if let Some(parent_id) = frame.parent_id {
        compute_frame_rect_cached(registry, parent_id, screen_width, screen_height, cache).rect
    } else {
        LayoutRect {
            x: 0.0,
            y: 0.0,
            width: screen_width,
            height: screen_height,
        }
    };

    // Effective scale is eagerly propagated on frames.
    let scale = frame.effective_scale;

    let mut rect = if frame.anchors.is_empty() {
        let w = frame.width * scale;
        let h = frame.height * scale;
        LayoutRect {
            x: parent_rect.x,
            y: parent_rect.y,
            width: w,
            height: h,
        }
    } else if frame.anchors.len() >= 2 {
        let edges = resolve_multi_anchor_edges(
            registry,
            frame,
            parent_rect,
            scale,
            screen_width,
            screen_height,
            cache,
        );
        compute_rect_from_edges(edges, frame, parent_rect, scale)
    } else {
        resolve_single_anchor(
            registry,
            frame,
            parent_rect,
            scale,
            screen_width,
            screen_height,
            cache,
        )
    };

    rect.x += frame.anim_offset_x;
    rect.y += frame.anim_offset_y;

    if frame.widget_type == WidgetType::Line {
        if let (Some(start), Some(end)) = (&frame.line_start, &frame.line_end) {
            if let (Some(sp), Some(ep)) = (
                resolve_line_anchor(start, registry, screen_width, screen_height, cache),
                resolve_line_anchor(end, registry, screen_width, screen_height, cache),
            ) {
                rect = line_bounding_box(sp, ep, frame.line_thickness * scale);
            }
        }
    }

    if frame.clamped_to_screen && rect.width > 0.0 && rect.height > 0.0 {
        clamp_rect_to_screen(&mut rect, screen_width, screen_height);
    }

    let result = CachedFrameLayout {
        rect,
        eff_scale: scale,
    };
    cache.insert(id, result);
    result
}

/// Compute frame rect with anchor resolution (uncached).
///
/// Thin wrapper that creates a temporary cache. Used by callers that compute
/// a single frame rect (inspector panel, tree dump, one-off test assertions).
pub fn compute_frame_rect(
    registry: &WidgetRegistry,
    id: u64,
    screen_width: f32,
    screen_height: f32,
) -> LayoutRect {
    let mut cache = LayoutCache::new();
    compute_frame_rect_cached(registry, id, screen_width, screen_height, &mut cache).rect
}

/// Shift a rect so it stays within screen bounds (WoW `clampedToScreen` behavior).
///
/// WoW's clamping prevents frames from being positioned entirely off-screen.
/// It keeps the top-left corner on-screen (x >= 0, y >= 0) and allows frames
/// to extend past the bottom/right edges — tall frames anchored partway down
/// the screen simply clip at the bottom rather than being pulled upward.
fn clamp_rect_to_screen(rect: &mut LayoutRect, _screen_w: f32, _screen_h: f32) {
    if rect.x < 0.0 {
        rect.x = 0.0;
    }
    if rect.y < 0.0 {
        rect.y = 0.0;
    }
}

/// Get the position of an anchor point on a rectangle.
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

/// Calculate frame position given its anchor point and target position.
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

/// Resolve a line anchor to absolute screen coordinates.
fn resolve_line_anchor(
    anchor: &LineAnchor,
    registry: &WidgetRegistry,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> Option<(f32, f32)> {
    let target_id = anchor.target_id?;
    let target_layout =
        compute_frame_rect_cached(registry, target_id, screen_width, screen_height, cache);
    let r = target_layout.rect;
    let (ax, ay) = anchor_position(anchor.point, r.x, r.y, r.width, r.height);
    // WoW y-offset is inverted (positive = up in WoW, but down in screen coords)
    Some((ax + anchor.x_offset, ay - anchor.y_offset))
}

/// Compute axis-aligned bounding box for a line between two points with thickness.
fn line_bounding_box(start: (f32, f32), end: (f32, f32), thickness: f32) -> LayoutRect {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.001 {
        return LayoutRect {
            x: start.0,
            y: start.1,
            width: 0.0,
            height: 0.0,
        };
    }
    let half_t = thickness / 2.0;
    let px = -dy / len * half_t;
    let py = dx / len * half_t;
    let corners = [
        (start.0 + px, start.1 + py),
        (start.0 - px, start.1 - py),
        (end.0 + px, end.1 + py),
        (end.0 - px, end.1 - py),
    ];
    let min_x = corners.iter().map(|c| c.0).fold(f32::INFINITY, f32::min);
    let max_x = corners
        .iter()
        .map(|c| c.0)
        .fold(f32::NEG_INFINITY, f32::max);
    let min_y = corners.iter().map(|c| c.1).fold(f32::INFINITY, f32::min);
    let max_y = corners
        .iter()
        .map(|c| c.1)
        .fold(f32::NEG_INFINITY, f32::max);
    LayoutRect {
        x: min_x,
        y: min_y,
        width: max_x - min_x,
        height: max_y - min_y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::{Anchor, AnchorPoint, Frame};

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

    /// Build a three-slice registry: UIParent → button → Left, Right, Center.
    ///
    /// Anchors use explicit parent IDs (matching XML-loaded frames, which resolve
    /// nil relativeTo to the parent ID before storage).
    fn build_three_slice_registry() -> WidgetRegistry {
        let mut reg = WidgetRegistry::new();
        let mut uip = make_frame(1, None, 1024.0, 768.0, vec![10], vec![]);
        uip.name = Some("UIParent".to_string());
        reg.register(uip);
        // Button (10): CENTER anchor to UIParent (id=1) — explicit parent ID as XML loading stores
        reg.register(make_frame(
            10,
            Some(1),
            200.0,
            36.0,
            vec![20, 21, 22],
            vec![anchor(AnchorPoint::Center, Some(1), AnchorPoint::Center)],
        ));
        // Left (20): LEFT anchor to button (id=10)
        reg.register(make_frame(
            20,
            Some(10),
            32.0,
            39.0,
            vec![],
            vec![anchor(AnchorPoint::Left, Some(10), AnchorPoint::Left)],
        ));
        // Right (21): RIGHT anchor to button (id=10)
        reg.register(make_frame(
            21,
            Some(10),
            32.0,
            39.0,
            vec![],
            vec![anchor(AnchorPoint::Right, Some(10), AnchorPoint::Right)],
        ));
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
        reg
    }

    /// Three-slice Center texture with cross-frame anchors must have non-zero size.
    #[test]
    fn test_cross_frame_anchor_center_texture() {
        let registry = build_three_slice_registry();
        let rect = compute_frame_rect(&registry, 22, 1024.0, 768.0);
        // Width = btn.width - left.width - right.width = 200 - 32 - 32 = 136
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
