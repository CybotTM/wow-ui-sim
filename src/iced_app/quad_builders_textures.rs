use iced::{Point, Rectangle, Size};

use crate::atlas::{AtlasSliceMode, get_atlas_info, get_atlas_slice_info};
use crate::render::{BlendMode, QuadBatch};

use super::super::statusbar::StatusBarFill;
use super::super::tiling::emit_tiled_texture;

/// Build quads for a Texture widget, optionally clipped by a StatusBar fill.
pub(crate) fn build_texture_quads(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    bar_fill: Option<&StatusBarFill>,
    alpha: f32,
) {
    if let Some(ref ns) = f.nine_slice_atlas {
        super::super::nine_slice::emit_nine_slice_atlas(batch, bounds, ns, alpha);
        return;
    }

    let tint = resolve_tint(f, bar_fill, alpha);

    if let Some(color) = f.color_texture {
        let fill_bounds = apply_bar_fill(bounds, bar_fill);
        if let Some(ref grad) = f.gradient {
            push_gradient_quad(batch, fill_bounds, grad, alpha);
        } else {
            batch.push_solid(
                fill_bounds,
                [
                    color.r * tint[0],
                    color.g * tint[1],
                    color.b * tint[2],
                    color.a * alpha,
                ],
            );
        }
        return;
    }

    let Some(tex_path) = &f.texture else {
        emit_bar_fill_fallback(batch, bar_fill, bounds, alpha);
        return;
    };
    emit_textured_quad(batch, bounds, f, bar_fill, tex_path, tint, alpha);
}

/// Compute the vertex color tint from vertex_color and bar fill override.
fn resolve_tint(
    f: &crate::widget::Frame,
    bar_fill: Option<&StatusBarFill>,
    alpha: f32,
) -> [f32; 4] {
    if let Some(fill) = bar_fill
        && let Some(c) = &fill.color
    {
        return [c.r, c.g, c.b, c.a * alpha];
    }
    let vc = f.vertex_color.as_ref();
    [
        vc.map_or(1.0, |c| c.r),
        vc.map_or(1.0, |c| c.g),
        vc.map_or(1.0, |c| c.b),
        vc.map_or(1.0, |c| c.a) * alpha,
    ]
}

/// Emit a solid color quad when no texture path exists but a bar fill has a color.
fn emit_bar_fill_fallback(
    batch: &mut QuadBatch,
    bar_fill: Option<&StatusBarFill>,
    bounds: Rectangle,
    alpha: f32,
) {
    if let Some(fill) = bar_fill
        && let Some(c) = &fill.color
    {
        let fill_bounds = apply_bar_fill(bounds, bar_fill);
        batch.push_solid(fill_bounds, [c.r, c.g, c.b, c.a * alpha]);
    }
}

/// Emit a gradient quad with per-vertex colors (VERTICAL or HORIZONTAL).
fn push_gradient_quad(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    grad: &crate::widget::Gradient,
    alpha: f32,
) {
    let min = &grad.min_color;
    let max = &grad.max_color;
    let (top_color, bottom_color) = if grad.vertical {
        (
            [max.r, max.g, max.b, max.a * alpha],
            [min.r, min.g, min.b, min.a * alpha],
        )
    } else {
        (
            [min.r, min.g, min.b, min.a * alpha],
            [min.r, min.g, min.b, min.a * alpha],
        )
    };
    let colors = if grad.vertical {
        [top_color, top_color, bottom_color, bottom_color]
    } else {
        let right = [max.r, max.g, max.b, max.a * alpha];
        let left = [min.r, min.g, min.b, min.a * alpha];
        [left, right, right, left]
    };
    batch.push_gradient(bounds, colors);
}

/// Emit a textured quad with atlas cropping, three-slice, tiling, rotation, desaturation.
fn emit_textured_quad(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    f: &crate::widget::Frame,
    bar_fill: Option<&StatusBarFill>,
    tex_path: &str,
    tint: [f32; 4],
    alpha: f32,
) {
    let (fill_bounds, fill_uvs) = apply_bar_fill_with_uvs(bounds, f.tex_coords, bar_fill);
    let (effective_path, effective_uvs) = remap_atlas_crop(tex_path, fill_uvs, f.atlas_tex_coords);
    let vert_before = batch.vertices.len();

    if let Some((slice, atlas_info)) = f
        .atlas
        .as_deref()
        .and_then(|name| get_atlas_slice_info(name).zip(get_atlas_info(name)))
        && slice.mode == AtlasSliceMode::Stretch
        && let Some((left, right, top, bottom)) = effective_uvs
        && fill_bounds.width > (slice.left + slice.right) as f32
        && fill_bounds.height > (slice.top + slice.bottom) as f32
    {
        emit_stretch_slice_atlas(
            batch,
            fill_bounds,
            slice.left as f32,
            slice.top as f32,
            slice.right as f32,
            slice.bottom as f32,
            atlas_info.info.width as f32,
            atlas_info.info.height as f32,
            (left, right, top, bottom),
            &effective_path,
            tint,
            f.blend_mode,
        );
    } else if let Some((left_cap, right_cap, atlas_w)) = f.three_slice_h
        && let Some((left, right, top, bottom)) = effective_uvs
        && fill_bounds.width > left_cap + right_cap
    {
        emit_three_slice_h_atlas(
            batch,
            fill_bounds,
            left_cap,
            right_cap,
            atlas_w,
            (left, right, top, bottom),
            &effective_path,
            tint,
            f.blend_mode,
        );
    } else if let Some((left, right, top, bottom)) = effective_uvs {
        let uvs = Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top));
        if f.horiz_tile || f.vert_tile {
            emit_tiled_texture(batch, fill_bounds, &uvs, &effective_path, f, alpha);
        } else {
            batch.push_textured_path_uv(fill_bounds, uvs, &effective_path, tint, f.blend_mode);
        }
    } else {
        batch.push_textured_path(fill_bounds, &effective_path, tint, f.blend_mode);
    }

    if f.rotation != 0.0 {
        apply_uv_rotation(batch, vert_before, f.rotation);
    }
    if f.desaturated {
        apply_desaturate_flag(batch, vert_before);
    }
}

/// Render an atlas texture as 3 horizontal slices (left cap, stretched middle, right cap).
#[allow(clippy::too_many_arguments)]
fn emit_three_slice_h_atlas(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    left_cap_px: f32,
    right_cap_px: f32,
    atlas_width_px: f32,
    (left_uv, right_uv, top_uv, bottom_uv): (f32, f32, f32, f32),
    tex_path: &str,
    tint: [f32; 4],
    blend: crate::render::BlendMode,
) {
    let uv_w = right_uv - left_uv;
    let uv_h = bottom_uv - top_uv;
    let left_frac = left_cap_px / atlas_width_px;
    let right_frac = right_cap_px / atlas_width_px;

    let left_cap_uv_end = left_uv + left_frac * uv_w;
    let right_cap_uv_start = right_uv - right_frac * uv_w;

    let mid_x = bounds.x + left_cap_px;
    let mid_w = bounds.width - left_cap_px - right_cap_px;
    let right_x = bounds.x + bounds.width - right_cap_px;

    batch.push_textured_path_uv(
        Rectangle::new(
            Point::new(bounds.x, bounds.y),
            Size::new(left_cap_px, bounds.height),
        ),
        Rectangle::new(
            Point::new(left_uv, top_uv),
            Size::new(left_cap_uv_end - left_uv, uv_h),
        ),
        tex_path,
        tint,
        blend,
    );
    batch.push_textured_path_uv(
        Rectangle::new(Point::new(mid_x, bounds.y), Size::new(mid_w, bounds.height)),
        Rectangle::new(
            Point::new(left_cap_uv_end, top_uv),
            Size::new(right_cap_uv_start - left_cap_uv_end, uv_h),
        ),
        tex_path,
        tint,
        blend,
    );
    batch.push_textured_path_uv(
        Rectangle::new(
            Point::new(right_x, bounds.y),
            Size::new(right_cap_px, bounds.height),
        ),
        Rectangle::new(
            Point::new(right_cap_uv_start, top_uv),
            Size::new(right_uv - right_cap_uv_start, uv_h),
        ),
        tex_path,
        tint,
        blend,
    );
}

#[allow(clippy::too_many_arguments)]
fn emit_stretch_slice_atlas(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    left_px: f32,
    top_px: f32,
    right_px: f32,
    bottom_px: f32,
    atlas_width_px: f32,
    atlas_height_px: f32,
    (left_uv, right_uv, top_uv, bottom_uv): (f32, f32, f32, f32),
    tex_path: &str,
    tint: [f32; 4],
    blend: crate::render::BlendMode,
) {
    let uv_w = right_uv - left_uv;
    let uv_h = bottom_uv - top_uv;
    let left_u = (left_px / atlas_width_px) * uv_w;
    let right_u = (right_px / atlas_width_px) * uv_w;
    let top_v = (top_px / atlas_height_px) * uv_h;
    let bottom_v = (bottom_px / atlas_height_px) * uv_h;

    let u1 = left_uv + left_u;
    let u2 = right_uv - right_u;
    let v1 = top_uv + top_v;
    let v2 = bottom_uv - bottom_v;

    let left_w = left_px;
    let right_w = right_px;
    let top_h = top_px;
    let bottom_h = bottom_px;
    let center_w = bounds.width - left_w - right_w;
    let center_h = bounds.height - top_h - bottom_h;

    let x0 = bounds.x;
    let x1 = bounds.x + left_w;
    let x2 = bounds.x + bounds.width - right_w;
    let y0 = bounds.y;
    let y1 = bounds.y + top_h;
    let y2 = bounds.y + bounds.height - bottom_h;

    let rects = [
        (
            Rectangle::new(Point::new(x0, y0), Size::new(left_w, top_h)),
            Rectangle::new(
                Point::new(left_uv, top_uv),
                Size::new(u1 - left_uv, v1 - top_uv),
            ),
        ),
        (
            Rectangle::new(Point::new(x1, y0), Size::new(center_w, top_h)),
            Rectangle::new(Point::new(u1, top_uv), Size::new(u2 - u1, v1 - top_uv)),
        ),
        (
            Rectangle::new(Point::new(x2, y0), Size::new(right_w, top_h)),
            Rectangle::new(
                Point::new(u2, top_uv),
                Size::new(right_uv - u2, v1 - top_uv),
            ),
        ),
        (
            Rectangle::new(Point::new(x0, y1), Size::new(left_w, center_h)),
            Rectangle::new(Point::new(left_uv, v1), Size::new(u1 - left_uv, v2 - v1)),
        ),
        (
            Rectangle::new(Point::new(x1, y1), Size::new(center_w, center_h)),
            Rectangle::new(Point::new(u1, v1), Size::new(u2 - u1, v2 - v1)),
        ),
        (
            Rectangle::new(Point::new(x2, y1), Size::new(right_w, center_h)),
            Rectangle::new(Point::new(u2, v1), Size::new(right_uv - u2, v2 - v1)),
        ),
        (
            Rectangle::new(Point::new(x0, y2), Size::new(left_w, bottom_h)),
            Rectangle::new(
                Point::new(left_uv, v2),
                Size::new(u1 - left_uv, bottom_uv - v2),
            ),
        ),
        (
            Rectangle::new(Point::new(x1, y2), Size::new(center_w, bottom_h)),
            Rectangle::new(Point::new(u1, v2), Size::new(u2 - u1, bottom_uv - v2)),
        ),
        (
            Rectangle::new(Point::new(x2, y2), Size::new(right_w, bottom_h)),
            Rectangle::new(Point::new(u2, v2), Size::new(right_uv - u2, bottom_uv - v2)),
        ),
    ];

    for (dst, src) in rects {
        batch.push_textured_path_uv(dst, src, tex_path, tint, blend);
    }
}

/// Apply StatusBar fill clipping to bounds.
fn apply_bar_fill(bounds: Rectangle, bar_fill: Option<&StatusBarFill>) -> Rectangle {
    let Some(fill) = bar_fill else { return bounds };
    let fill_width = bounds.width * fill.fraction;
    if fill.reverse {
        Rectangle::new(
            Point::new(bounds.x + bounds.width - fill_width, bounds.y),
            Size::new(fill_width, bounds.height),
        )
    } else {
        Rectangle::new(bounds.position(), Size::new(fill_width, bounds.height))
    }
}

/// Remap atlas sub-region textures: encode crop coords in path, remap UVs to [0,1].
pub(super) fn remap_atlas_crop(
    tex_path: &str,
    fill_uvs: Option<(f32, f32, f32, f32)>,
    atlas_tex_coords: Option<(f32, f32, f32, f32)>,
) -> (String, Option<(f32, f32, f32, f32)>) {
    let Some((cl, cr, ct, cb)) = atlas_tex_coords else {
        return (tex_path.to_string(), fill_uvs);
    };
    let is_full = (cl - 0.0).abs() < 0.001
        && (cr - 1.0).abs() < 0.001
        && (ct - 0.0).abs() < 0.001
        && (cb - 1.0).abs() < 0.001;
    if is_full {
        return (tex_path.to_string(), fill_uvs);
    }

    let crop_key = format!("{tex_path}@crop:{cl:.6},{cr:.6},{ct:.6},{cb:.6}");
    let remapped_uvs = fill_uvs.map(|(fl, fr, ft, fb)| {
        let cw = cr - cl;
        let ch = cb - ct;
        if cw <= 0.0 || ch <= 0.0 {
            return (0.0, 1.0, 0.0, 1.0);
        }
        (
            (fl - cl) / cw,
            (fr - cl) / cw,
            (ft - ct) / ch,
            (fb - ct) / ch,
        )
    });

    (crop_key, remapped_uvs)
}

/// Apply StatusBar fill clipping to bounds and UV coordinates.
fn apply_bar_fill_with_uvs(
    bounds: Rectangle,
    tex_coords: Option<(f32, f32, f32, f32)>,
    bar_fill: Option<&StatusBarFill>,
) -> (Rectangle, Option<(f32, f32, f32, f32)>) {
    let Some(fill) = bar_fill else {
        return (bounds, tex_coords);
    };
    let fill_bounds = apply_bar_fill(bounds, bar_fill);
    let (uv_left, uv_right, uv_top, uv_bottom) = tex_coords.unwrap_or((0.0, 1.0, 0.0, 1.0));
    let uv_range = uv_right - uv_left;
    let fill_uvs = if fill.reverse {
        (
            uv_left + uv_range * (1.0 - fill.fraction),
            uv_right,
            uv_top,
            uv_bottom,
        )
    } else {
        (
            uv_left,
            uv_left + uv_range * fill.fraction,
            uv_top,
            uv_bottom,
        )
    };
    (fill_bounds, Some(fill_uvs))
}

/// Rotate texture UV coordinates around their center for vertices added after `vert_before`.
fn apply_uv_rotation(batch: &mut QuadBatch, vert_before: usize, radians: f32) {
    let verts = &mut batch.vertices[vert_before..];
    if verts.len() < 4 {
        return;
    }
    let (sin_r, cos_r) = radians.sin_cos();
    for chunk in verts.chunks_exact_mut(4) {
        let cx = (chunk[0].tex_coords[0]
            + chunk[1].tex_coords[0]
            + chunk[2].tex_coords[0]
            + chunk[3].tex_coords[0])
            * 0.25;
        let cy = (chunk[0].tex_coords[1]
            + chunk[1].tex_coords[1]
            + chunk[2].tex_coords[1]
            + chunk[3].tex_coords[1])
            * 0.25;
        for v in chunk.iter_mut() {
            let du = v.tex_coords[0] - cx;
            let dv = v.tex_coords[1] - cy;
            v.tex_coords[0] = cx + du * cos_r - dv * sin_r;
            v.tex_coords[1] = cy + du * sin_r + dv * cos_r;
        }
    }
}

/// Apply the desaturation flag to vertices added after `vert_before`.
fn apply_desaturate_flag(batch: &mut QuadBatch, vert_before: usize) {
    use crate::render::shader::FLAG_DESATURATE;
    for v in &mut batch.vertices[vert_before..] {
        v.flags |= FLAG_DESATURATE;
    }
}

/// Build quads for a Minimap widget - map texture clipped to a circle.
pub(crate) fn build_minimap_quads(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    _f: &crate::widget::Frame,
    alpha: f32,
) {
    use crate::render::shader::FLAG_CIRCLE_CLIP;
    batch.push_textured_path(
        bounds,
        r"Interface\Minimap\placeholder-map",
        [1.0, 1.0, 1.0, alpha],
        BlendMode::Alpha,
    );
    batch.set_extra_flags(4, FLAG_CIRCLE_CLIP);
}

#[cfg(test)]
mod tests {
    use super::remap_atlas_crop;

    #[test]
    fn remap_atlas_crop_rewrites_subregion_to_crop_key() {
        let (path, uvs) = remap_atlas_crop(
            r"Interface\Glues\CharacterSelect\Glues-AddOn-Icons",
            Some((0.25, 0.5, 0.125, 0.625)),
            Some((0.25, 0.5, 0.125, 0.625)),
        );

        assert_eq!(
            path,
            r"Interface\Glues\CharacterSelect\Glues-AddOn-Icons@crop:0.250000,0.500000,0.125000,0.625000"
        );
        assert_eq!(uvs, Some((0.0, 1.0, 0.0, 1.0)));
    }
}
