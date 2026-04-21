//! Line widget quad rendering — rotated quads between two anchor points.

use crate::render::{BlendMode, QuadBatch};

fn is_talent_arrow_line(f: &crate::widget::Frame) -> bool {
    f.atlas
        .as_deref()
        .is_some_and(|atlas| atlas.starts_with("talents-arrow-line"))
}

/// Resolve a line anchor to screen-space pixel coordinates.
fn resolve_line_endpoint(
    anchor: &crate::widget::LineAnchor,
    registry: &crate::widget::WidgetRegistry,
) -> Option<(f32, f32)> {
    use super::layout::anchor_position;

    let target_id = anchor.target_id?;
    let r = registry.get(target_id)?.layout_rect?;
    let (ax, ay) = anchor_position(anchor.point, r.x, r.y, r.width, r.height);
    let ui_scale = crate::render::texture::UI_SCALE;
    Some((
        (ax + anchor.x_offset) * ui_scale,
        (ay - anchor.y_offset) * ui_scale,
    ))
}

/// Talent arrow lines are anchored from start-button center to end-button edge in Lua.
/// Nudge the rendered start point toward the endpoint so the visible line begins
/// near the start node border instead of crossing the icon center.
fn adjust_talent_line_start_endpoint(
    f: &crate::widget::Frame,
    start_anchor: &crate::widget::LineAnchor,
    end_anchor: Option<&crate::widget::LineAnchor>,
    registry: &crate::widget::WidgetRegistry,
    start: (f32, f32),
    end: (f32, f32),
) -> (f32, f32) {
    if !is_talent_arrow_line(f) {
        return start;
    }
    let Some(start_target_id) = start_anchor.target_id else {
        return start;
    };
    let Some(start_target) = registry.get(start_target_id) else {
        return start;
    };
    let Some(start_rect) = start_target.layout_rect else {
        return start;
    };

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.001 {
        return start;
    }

    let mut diameter_scale = 1.2_f32;
    if let Some(end_anchor) = end_anchor
        && let Some(end_target_id) = end_anchor.target_id
        && let Some(end_target) = registry.get(end_target_id)
        && let Some(end_rect) = end_target.layout_rect
    {
        let ui_scale = crate::render::texture::UI_SCALE;
        let rx = (end_rect.width * ui_scale * 0.5).abs();
        let ry = (end_rect.height * ui_scale * 0.5).abs();
        if rx > 0.0 && ry > 0.0 {
            let nx = (end_anchor.x_offset * ui_scale / rx).abs();
            let ny = (end_anchor.y_offset * ui_scale / ry).abs();
            let measured = (nx * nx + ny * ny).sqrt();
            if measured.is_finite() && measured > 0.1 {
                diameter_scale = measured.clamp(1.0, 1.6);
            }
        }
    }

    let trim = ((start_rect.width.min(start_rect.height) * crate::render::texture::UI_SCALE * 0.5)
        * diameter_scale)
        .min((len - 1.0).max(0.0));
    (start.0 + (dx / len) * trim, start.1 + (dy / len) * trim)
}

/// Compute the 4 corner positions of a rotated line quad from endpoints and thickness.
fn line_quad_positions(
    start: (f32, f32),
    end: (f32, f32),
    thickness: f32,
) -> Option<[[f32; 2]; 4]> {
    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.001 {
        return None;
    }
    let half_t = thickness / 2.0;
    let px = -dy / len * half_t;
    let py = dx / len * half_t;
    Some([
        [start.0 + px, start.1 + py],
        [start.0 - px, start.1 - py],
        [end.0 - px, end.1 - py],
        [end.0 + px, end.1 + py],
    ])
}

/// Build quads for a Line widget — a rotated quad between two anchor points.
pub(super) fn build_line_quads(
    batch: &mut QuadBatch,
    f: &crate::widget::Frame,
    registry: &crate::widget::WidgetRegistry,
    alpha: f32,
) {
    let Some((sp, ep, positions, uvs, tint)) = resolve_line_quad_inputs(f, registry, alpha) else {
        return;
    };
    if emit_horiz_tiled_line_quads(batch, f, sp, ep, tint, uvs) {
        return;
    }
    emit_resolved_line_quad(batch, f, alpha, &positions, &uvs, tint);
}

fn resolve_line_quad_inputs(
    f: &crate::widget::Frame,
    registry: &crate::widget::WidgetRegistry,
    alpha: f32,
) -> Option<(
    (f32, f32),
    (f32, f32),
    [[f32; 2]; 4],
    [[f32; 2]; 4],
    [f32; 4],
)> {
    let (Some(start_anchor), Some(end_anchor)) = (&f.line_start, &f.line_end) else {
        return None;
    };
    let sp = resolve_line_endpoint(start_anchor, registry)?;
    let ep = resolve_line_endpoint(end_anchor, registry)?;
    let sp = adjust_talent_line_start_endpoint(f, start_anchor, Some(end_anchor), registry, sp, ep);
    let thickness = f.line_thickness * crate::render::texture::UI_SCALE;
    let positions = line_quad_positions(sp, ep, thickness)?;
    Some((sp, ep, positions, line_uvs(f), line_tint(f, alpha)))
}

fn line_uvs(f: &crate::widget::Frame) -> [[f32; 2]; 4] {
    if let Some((left, right, top, bottom)) = f.tex_coords {
        [[left, top], [left, bottom], [right, bottom], [right, top]]
    } else {
        [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]]
    }
}

fn line_tint(f: &crate::widget::Frame, alpha: f32) -> [f32; 4] {
    let vc = f.vertex_color.as_ref();
    [
        vc.map_or(1.0, |c| c.r),
        vc.map_or(1.0, |c| c.g),
        vc.map_or(1.0, |c| c.b),
        vc.map_or(1.0, |c| c.a) * alpha,
    ]
}

fn emit_resolved_line_quad(
    batch: &mut QuadBatch,
    f: &crate::widget::Frame,
    alpha: f32,
    positions: &[[f32; 2]; 4],
    uvs: &[[f32; 2]; 4],
    tint: [f32; 4],
) {
    if let Some(color) = f.color_texture {
        let color_tint = [
            color.r * tint[0],
            color.g * tint[1],
            color.b * tint[2],
            color.a * alpha,
        ];
        emit_line_vertices(batch, positions, uvs, color_tint, -1, f.blend_mode);
        return;
    }
    if let Some(ref tex_path) = f.texture {
        let vertex_start = batch.vertices.len() as u32;
        emit_line_vertices(batch, positions, uvs, tint, -2, f.blend_mode);
        batch
            .texture_requests
            .push(crate::render::shader::TextureRequest {
                path: tex_path.clone(),
                vertex_start,
                vertex_count: 4,
            });
        return;
    }
    emit_line_vertices(batch, positions, uvs, tint, -1, f.blend_mode);
}

fn emit_horiz_tiled_line_quads(
    batch: &mut QuadBatch,
    f: &crate::widget::Frame,
    start: (f32, f32),
    end: (f32, f32),
    tint: [f32; 4],
    base_uvs: [[f32; 2]; 4],
) -> bool {
    if !f.horiz_tile {
        return false;
    }
    let Some(tex_path) = f.texture.as_ref() else {
        return false;
    };

    let dx = end.0 - start.0;
    let dy = end.1 - start.1;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 0.001 {
        return false;
    }
    let tile_len = line_tile_length_px(f).max(1.0);
    let ux = dx / len;
    let uy = dy / len;
    let half_t = f.line_thickness * crate::render::texture::UI_SCALE * 0.5;
    let px = -uy * half_t;
    let py = ux * half_t;

    let left = base_uvs[0][0];
    let right = base_uvs[2][0];
    let top = base_uvs[0][1];
    let bottom = base_uvs[1][1];

    let mut offset = 0.0_f32;
    // Adjacent rotated quads can show tiny raster cracks at tile joins due to
    // floating-point edge coverage. A small overlap hides those seams.
    let join_overlap = 0.5_f32;
    // Arrow connectors can also show a tiny gap where the line meets the
    // arrow head. Extend the first/last segment slightly to hide that seam.
    let start_cap_overlap = if is_talent_arrow_line(f) { 0.0 } else { 1.0 };
    let end_cap_overlap = line_end_cap_overlap_px(f);
    while offset < len - 0.001 {
        let seg_len = (len - offset).min(tile_len);
        let u_ratio = (seg_len / tile_len).clamp(0.0, 1.0);
        let seg_right = left + (right - left) * u_ratio;
        let mut seg_start_offset = if offset > 0.0 {
            (offset - join_overlap).max(0.0)
        } else {
            0.0
        };
        let mut seg_end_offset = if offset + seg_len < len {
            (offset + seg_len + join_overlap).min(len)
        } else {
            len
        };
        if offset <= f32::EPSILON {
            seg_start_offset -= start_cap_overlap;
        }
        if offset + seg_len >= len - 0.001 {
            seg_end_offset += end_cap_overlap;
        }
        let seg_start = (
            start.0 + ux * seg_start_offset,
            start.1 + uy * seg_start_offset,
        );
        let seg_end = (start.0 + ux * seg_end_offset, start.1 + uy * seg_end_offset);
        let positions = [
            [seg_start.0 + px, seg_start.1 + py],
            [seg_start.0 - px, seg_start.1 - py],
            [seg_end.0 - px, seg_end.1 - py],
            [seg_end.0 + px, seg_end.1 + py],
        ];
        let uvs = [
            [left, top],
            [left, bottom],
            [seg_right, bottom],
            [seg_right, top],
        ];
        let vertex_start = batch.vertices.len() as u32;
        emit_line_vertices(batch, &positions, &uvs, tint, -2, f.blend_mode);
        batch
            .texture_requests
            .push(crate::render::shader::TextureRequest {
                path: tex_path.clone(),
                vertex_start,
                vertex_count: 4,
            });
        offset += tile_len;
    }

    true
}

fn line_tile_length_px(f: &crate::widget::Frame) -> f32 {
    if let Some(atlas_name) = f.atlas.as_deref()
        && let Some(lookup) = crate::atlas::get_render_atlas_info(atlas_name)
    {
        return lookup.width() as f32 * crate::render::texture::UI_SCALE;
    }
    if f.width > 1.0 {
        return f.width * crate::render::texture::UI_SCALE;
    }
    (f.line_thickness * crate::render::texture::UI_SCALE).max(1.0)
}

fn line_end_cap_overlap_px(f: &crate::widget::Frame) -> f32 {
    if is_talent_arrow_line(f) {
        return (f.line_thickness * crate::render::texture::UI_SCALE).clamp(3.0, 8.0);
    }
    3.0
}

/// Push 4 vertices and 6 indices for a line quad with arbitrary positions.
fn emit_line_vertices(
    batch: &mut QuadBatch,
    positions: &[[f32; 2]; 4],
    uvs: &[[f32; 2]; 4],
    color: [f32; 4],
    tex_index: i32,
    blend_mode: BlendMode,
) {
    use crate::render::shader::QuadVertex;

    let base = batch.vertices.len() as u32;
    let flags = blend_mode as u32;
    for i in 0..4 {
        batch.vertices.push(QuadVertex {
            position: positions[i],
            tex_coords: uvs[i],
            color,
            tex_index,
            flags,
            local_uv: uvs[i],
            mask_tex_index: -1,
            mask_tex_coords: [0.0, 0.0],
        });
    }
    // TL(0)-BL(1)-BR(2) and TL(0)-BR(2)-TR(3)
    batch
        .indices
        .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::{Color, Frame, LineAnchor, WidgetRegistry, WidgetType};

    #[test]
    fn line_uvs_uses_tex_coords_when_present() {
        let mut frame = Frame::default();
        frame.tex_coords = Some((0.1, 0.9, 0.2, 0.8));

        assert_eq!(
            line_uvs(&frame),
            [[0.1, 0.2], [0.1, 0.8], [0.9, 0.8], [0.9, 0.2]]
        );
    }

    #[test]
    fn line_tint_uses_vertex_color_and_alpha() {
        let mut frame = Frame::default();
        frame.vertex_color = Some(Color::new(0.2, 0.4, 0.6, 0.5));

        assert_eq!(line_tint(&frame, 0.8), [0.2, 0.4, 0.6, 0.4]);
    }

    #[test]
    fn talent_arrow_line_start_is_trimmed_toward_endpoint() {
        let mut registry = WidgetRegistry::new();

        let mut start_target = Frame::new(WidgetType::Button, Some("Start".to_string()), None);
        start_target.layout_rect = Some(crate::LayoutRect {
            x: 0.0,
            y: 0.0,
            width: 40.0,
            height: 40.0,
        });
        let start_id = start_target.id;
        registry.register(start_target);

        let mut end_target = Frame::new(WidgetType::Button, Some("End".to_string()), None);
        end_target.layout_rect = Some(crate::LayoutRect {
            x: 100.0,
            y: 0.0,
            width: 40.0,
            height: 40.0,
        });
        let end_id = end_target.id;
        registry.register(end_target);

        let mut line = Frame::new(WidgetType::Line, Some("EdgeLine".to_string()), None);
        line.atlas = Some("talents-arrow-line-yellow".to_string());
        line.line_start = Some(LineAnchor {
            point: crate::widget::AnchorPoint::Center,
            target_id: Some(start_id),
            x_offset: 0.0,
            y_offset: 0.0,
        });
        line.line_end = Some(LineAnchor {
            point: crate::widget::AnchorPoint::Center,
            target_id: Some(end_id),
            x_offset: 24.0,
            y_offset: 0.0,
        });

        let start = (20.0, 20.0);
        let end = (120.0, 20.0);
        let adjusted = adjust_talent_line_start_endpoint(
            &line,
            line.line_start.as_ref().unwrap(),
            line.line_end.as_ref(),
            &registry,
            start,
            end,
        );
        assert!(
            adjusted.0 > start.0,
            "talent line start should move toward end"
        );
        assert!(adjusted.0 < end.0, "trimmed start must remain before end");
        assert_eq!(
            adjusted.1, start.1,
            "horizontal test should keep y unchanged"
        );
    }
}
