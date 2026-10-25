use crate::LayoutRect;
use crate::widget::{LineAnchor, WidgetRegistry, WidgetType};

use super::{LayoutCache, anchor_position, compute_frame_rect_cached};

pub(super) fn resolve_line_frame_rect(
    frame: &crate::widget::Frame,
    registry: &WidgetRegistry,
    scale: f32,
    screen_width: f32,
    screen_height: f32,
    cache: &mut LayoutCache,
) -> Option<LayoutRect> {
    if frame.widget_type != WidgetType::Line {
        return None;
    }

    let (Some(start), Some(end)) = (&frame.line_start, &frame.line_end) else {
        return None;
    };

    let (Some(start_point), Some(end_point)) = (
        resolve_line_anchor(start, registry, screen_width, screen_height, cache),
        resolve_line_anchor(end, registry, screen_width, screen_height, cache),
    ) else {
        return None;
    };

    Some(line_bounding_box(
        start_point,
        end_point,
        frame.line_thickness * scale,
    ))
}

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
    let rect = target_layout.rect;
    let (anchor_x, anchor_y) =
        anchor_position(anchor.point, rect.x, rect.y, rect.width, rect.height);

    // WoW y-offset is inverted (positive = up in WoW, but down in screen coords)
    Some((anchor_x + anchor.x_offset, anchor_y - anchor.y_offset))
}

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

    let corners = line_box_corners(start, end, dx, dy, len, thickness);
    let ((min_x, max_x), (min_y, max_y)) = corner_bounds(&corners);
    LayoutRect {
        x: min_x,
        y: min_y,
        width: max_x - min_x,
        height: max_y - min_y,
    }
}

fn line_box_corners(
    start: (f32, f32),
    end: (f32, f32),
    dx: f32,
    dy: f32,
    len: f32,
    thickness: f32,
) -> [(f32, f32); 4] {
    let half_thickness = thickness / 2.0;
    let perp_x = -dy / len * half_thickness;
    let perp_y = dx / len * half_thickness;
    [
        (start.0 + perp_x, start.1 + perp_y),
        (start.0 - perp_x, start.1 - perp_y),
        (end.0 + perp_x, end.1 + perp_y),
        (end.0 - perp_x, end.1 - perp_y),
    ]
}

fn corner_bounds(corners: &[(f32, f32); 4]) -> ((f32, f32), (f32, f32)) {
    let min_x = corners
        .iter()
        .map(|corner| corner.0)
        .fold(f32::INFINITY, f32::min);
    let max_x = corners
        .iter()
        .map(|corner| corner.0)
        .fold(f32::NEG_INFINITY, f32::max);
    let min_y = corners
        .iter()
        .map(|corner| corner.1)
        .fold(f32::INFINITY, f32::min);
    let max_y = corners
        .iter()
        .map(|corner| corner.1)
        .fold(f32::NEG_INFINITY, f32::max);
    ((min_x, max_x), (min_y, max_y))
}

#[cfg(test)]
mod tests {
    use super::line_bounding_box;

    #[test]
    fn test_line_bounding_box_for_horizontal_line() {
        let rect = line_bounding_box((10.0, 20.0), (30.0, 20.0), 4.0);
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 18.0);
        assert_eq!(rect.width, 20.0);
        assert_eq!(rect.height, 4.0);
    }
}
