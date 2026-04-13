use iced::{Point, Rectangle, Size};

use crate::atlas::{AtlasSliceMode, get_atlas_info, get_atlas_slice_info};
use crate::render::{BlendMode, QuadBatch};

use super::super::statusbar::StatusBarFill;
use super::super::tiling::{
    emit_grid_tiles, emit_horiz_tiles, emit_tiled_texture, emit_vert_tiles,
};

type TextureUvs = (f32, f32, f32, f32);

#[derive(Clone, Copy)]
struct TexturedSlice<'a> {
    path: &'a str,
    uvs: TextureUvs,
    tint: [f32; 4],
    blend: BlendMode,
}

#[derive(Clone, Copy)]
struct ThreeSliceRender<'a> {
    bounds: Rectangle,
    texture: TexturedSlice<'a>,
    left_cap_px: f32,
    right_cap_px: f32,
    atlas_width_px: f32,
}

#[derive(Clone, Copy)]
struct StretchSliceRender<'a> {
    bounds: Rectangle,
    texture: TexturedSlice<'a>,
    left_px: f32,
    top_px: f32,
    right_px: f32,
    bottom_px: f32,
    atlas_width_px: f32,
    atlas_height_px: f32,
}

#[derive(Clone, Copy)]
struct TileSliceRender<'a> {
    bounds: Rectangle,
    texture: TexturedSlice<'a>,
    left_px: f32,
    top_px: f32,
    right_px: f32,
    bottom_px: f32,
    atlas_width_px: f32,
    atlas_height_px: f32,
}

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
    emit_texture_fill(
        batch,
        fill_bounds,
        effective_uvs,
        &effective_path,
        f,
        tint,
        alpha,
    );
    finalize_textured_quad(batch, vert_before, f);
}

fn emit_texture_fill(
    batch: &mut QuadBatch,
    fill_bounds: Rectangle,
    effective_uvs: Option<TextureUvs>,
    effective_path: &str,
    f: &crate::widget::Frame,
    tint: [f32; 4],
    alpha: f32,
) {
    let Some(uvs) = effective_uvs else {
        batch.push_textured_path(fill_bounds, &effective_path, tint, f.blend_mode);
        return;
    };

    let texture = TexturedSlice {
        path: effective_path,
        uvs,
        tint,
        blend: f.blend_mode,
    };

    if emit_specialized_textured_fill(batch, fill_bounds, f, texture) {
        return;
    }

    emit_basic_textured_fill(batch, fill_bounds, texture, f, alpha);
}

fn emit_specialized_textured_fill(
    batch: &mut QuadBatch,
    fill_bounds: Rectangle,
    f: &crate::widget::Frame,
    texture: TexturedSlice<'_>,
) -> bool {
    if let Some(render) = stretch_slice_render(f, fill_bounds, texture) {
        emit_stretch_slice_atlas(batch, render);
        return true;
    }

    if let Some(render) = tile_slice_render(f, fill_bounds, texture) {
        emit_tile_slice_atlas(batch, render);
        return true;
    }

    if let Some(render) = three_slice_render(f, fill_bounds, texture) {
        emit_three_slice_h_atlas(batch, render);
        return true;
    }

    false
}

fn stretch_slice_render<'a>(
    f: &crate::widget::Frame,
    bounds: Rectangle,
    texture: TexturedSlice<'a>,
) -> Option<StretchSliceRender<'a>> {
    let atlas_name = f.atlas.as_deref()?;
    let slice = get_atlas_slice_info(atlas_name)?;
    if slice.mode != AtlasSliceMode::Stretch {
        return None;
    }

    if bounds.width <= (slice.left + slice.right) as f32
        || bounds.height <= (slice.top + slice.bottom) as f32
    {
        return None;
    }

    let atlas_info = get_atlas_info(atlas_name)?;
    Some(StretchSliceRender {
        bounds,
        texture,
        left_px: slice.left as f32,
        top_px: slice.top as f32,
        right_px: slice.right as f32,
        bottom_px: slice.bottom as f32,
        atlas_width_px: atlas_info.info.width as f32,
        atlas_height_px: atlas_info.info.height as f32,
    })
}

fn tile_slice_render<'a>(
    f: &crate::widget::Frame,
    bounds: Rectangle,
    texture: TexturedSlice<'a>,
) -> Option<TileSliceRender<'a>> {
    let atlas_name = f.atlas.as_deref()?;
    let slice = get_atlas_slice_info(atlas_name)?;
    if slice.mode != AtlasSliceMode::Tile {
        return None;
    }

    if bounds.width < (slice.left + slice.right) as f32
        || bounds.height < (slice.top + slice.bottom) as f32
    {
        return None;
    }

    let atlas_info = get_atlas_info(atlas_name)?;
    Some(TileSliceRender {
        bounds,
        texture,
        left_px: slice.left as f32,
        top_px: slice.top as f32,
        right_px: slice.right as f32,
        bottom_px: slice.bottom as f32,
        atlas_width_px: atlas_info.info.width as f32,
        atlas_height_px: atlas_info.info.height as f32,
    })
}

fn three_slice_render<'a>(
    f: &crate::widget::Frame,
    bounds: Rectangle,
    texture: TexturedSlice<'a>,
) -> Option<ThreeSliceRender<'a>> {
    let (left_cap_px, right_cap_px, atlas_width_px) = f.three_slice_h?;
    if bounds.width <= left_cap_px + right_cap_px {
        return None;
    }

    Some(ThreeSliceRender {
        bounds,
        texture,
        left_cap_px,
        right_cap_px,
        atlas_width_px,
    })
}

fn emit_basic_textured_fill(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    texture: TexturedSlice<'_>,
    f: &crate::widget::Frame,
    alpha: f32,
) {
    let uvs = uv_rect(texture.uvs);
    if f.horiz_tile || f.vert_tile {
        emit_tiled_texture(batch, bounds, &uvs, texture.path, f, alpha);
        return;
    }

    batch.push_textured_path_uv(bounds, uvs, texture.path, texture.tint, texture.blend);
}

fn uv_rect((left, right, top, bottom): TextureUvs) -> Rectangle {
    Rectangle::new(Point::new(left, top), Size::new(right - left, bottom - top))
}

fn finalize_textured_quad(batch: &mut QuadBatch, vert_before: usize, f: &crate::widget::Frame) {
    if f.rotation != 0.0 {
        apply_uv_rotation(batch, vert_before, f.rotation);
    }
    if f.desaturated {
        apply_desaturate_flag(batch, vert_before);
    }
}

/// Render an atlas texture as 3 horizontal slices (left cap, stretched middle, right cap).
fn emit_three_slice_h_atlas(batch: &mut QuadBatch, render: ThreeSliceRender<'_>) {
    push_textured_rects(batch, render.texture, three_slice_rects(render));
}

fn three_slice_rects(render: ThreeSliceRender<'_>) -> [(Rectangle, Rectangle); 3] {
    let bounds = render.bounds;
    let (left_uv, right_uv, top_uv, bottom_uv) = render.texture.uvs;
    let uv_w = right_uv - left_uv;
    let uv_h = bottom_uv - top_uv;
    let left_frac = render.left_cap_px / render.atlas_width_px;
    let right_frac = render.right_cap_px / render.atlas_width_px;

    let left_cap_uv_end = left_uv + left_frac * uv_w;
    let right_cap_uv_start = right_uv - right_frac * uv_w;

    let mid_x = bounds.x + render.left_cap_px;
    let mid_w = bounds.width - render.left_cap_px - render.right_cap_px;
    let right_x = bounds.x + bounds.width - render.right_cap_px;

    [
        (
            Rectangle::new(
                Point::new(bounds.x, bounds.y),
                Size::new(render.left_cap_px, bounds.height),
            ),
            Rectangle::new(
                Point::new(left_uv, top_uv),
                Size::new(left_cap_uv_end - left_uv, uv_h),
            ),
        ),
        (
            Rectangle::new(Point::new(mid_x, bounds.y), Size::new(mid_w, bounds.height)),
            Rectangle::new(
                Point::new(left_cap_uv_end, top_uv),
                Size::new(right_cap_uv_start - left_cap_uv_end, uv_h),
            ),
        ),
        (
            Rectangle::new(
                Point::new(right_x, bounds.y),
                Size::new(render.right_cap_px, bounds.height),
            ),
            Rectangle::new(
                Point::new(right_cap_uv_start, top_uv),
                Size::new(right_uv - right_cap_uv_start, uv_h),
            ),
        ),
    ]
}

fn emit_stretch_slice_atlas(batch: &mut QuadBatch, render: StretchSliceRender<'_>) {
    push_textured_rects(batch, render.texture, stretch_slice_rects(render));
}

fn emit_tile_slice_atlas(batch: &mut QuadBatch, render: TileSliceRender<'_>) {
    emit_tile_slice_corners(batch, render);
    emit_tile_slice_edges(batch, render);
    emit_tile_slice_center(batch, render);
}

fn emit_tile_slice_corners(batch: &mut QuadBatch, render: TileSliceRender<'_>) {
    push_cropped_slice_rects(batch, render.texture, tile_slice_corner_rects(render));
}

fn emit_tile_slice_edges(batch: &mut QuadBatch, render: TileSliceRender<'_>) {
    emit_tile_slice_horiz_edges(batch, render);
    emit_tile_slice_vert_edges(batch, render);
}

fn emit_tile_slice_center(batch: &mut QuadBatch, render: TileSliceRender<'_>) {
    let Some((center_dst_w, center_src_w)) = tile_slice_center_width(render) else {
        return;
    };
    let Some((center_dst_h, center_src_h)) = tile_slice_center_height(render) else {
        return;
    };

    let center_uvs = atlas_subregion_uvs(
        render.texture,
        render.atlas_width_px,
        render.atlas_height_px,
        render.left_px,
        render.top_px,
        center_src_w,
        center_src_h,
    );
    let center_bounds = Rectangle::new(
        Point::new(
            render.bounds.x + render.left_px,
            render.bounds.y + render.top_px,
        ),
        Size::new(center_dst_w, center_dst_h),
    );
    emit_grid_tiled_slice(
        batch,
        render.texture,
        center_bounds,
        center_uvs,
        center_src_w,
        center_src_h,
    );
}

fn tile_slice_corner_rects(render: TileSliceRender<'_>) -> [(Rectangle, Rectangle); 4] {
    [
        (
            Rectangle::new(
                Point::new(render.bounds.x, render.bounds.y),
                Size::new(render.left_px, render.top_px),
            ),
            atlas_subregion_uvs(
                render.texture,
                render.atlas_width_px,
                render.atlas_height_px,
                0.0,
                0.0,
                render.left_px,
                render.top_px,
            ),
        ),
        (
            Rectangle::new(
                Point::new(
                    render.bounds.x + render.bounds.width - render.right_px,
                    render.bounds.y,
                ),
                Size::new(render.right_px, render.top_px),
            ),
            atlas_subregion_uvs(
                render.texture,
                render.atlas_width_px,
                render.atlas_height_px,
                render.atlas_width_px - render.right_px,
                0.0,
                render.right_px,
                render.top_px,
            ),
        ),
        (
            Rectangle::new(
                Point::new(
                    render.bounds.x,
                    render.bounds.y + render.bounds.height - render.bottom_px,
                ),
                Size::new(render.left_px, render.bottom_px),
            ),
            atlas_subregion_uvs(
                render.texture,
                render.atlas_width_px,
                render.atlas_height_px,
                0.0,
                render.atlas_height_px - render.bottom_px,
                render.left_px,
                render.bottom_px,
            ),
        ),
        (
            Rectangle::new(
                Point::new(
                    render.bounds.x + render.bounds.width - render.right_px,
                    render.bounds.y + render.bounds.height - render.bottom_px,
                ),
                Size::new(render.right_px, render.bottom_px),
            ),
            atlas_subregion_uvs(
                render.texture,
                render.atlas_width_px,
                render.atlas_height_px,
                render.atlas_width_px - render.right_px,
                render.atlas_height_px - render.bottom_px,
                render.right_px,
                render.bottom_px,
            ),
        ),
    ]
}

fn emit_tile_slice_horiz_edges(batch: &mut QuadBatch, render: TileSliceRender<'_>) {
    let Some((center_dst_w, center_src_w)) = tile_slice_center_width(render) else {
        return;
    };

    let top_uvs = atlas_subregion_uvs(
        render.texture,
        render.atlas_width_px,
        render.atlas_height_px,
        render.left_px,
        0.0,
        center_src_w,
        render.top_px,
    );
    let top_bounds = Rectangle::new(
        Point::new(render.bounds.x + render.left_px, render.bounds.y),
        Size::new(center_dst_w, render.top_px),
    );
    emit_horiz_tiled_slice(batch, render.texture, top_bounds, top_uvs, center_src_w);

    let bottom_uvs = atlas_subregion_uvs(
        render.texture,
        render.atlas_width_px,
        render.atlas_height_px,
        render.left_px,
        render.atlas_height_px - render.bottom_px,
        center_src_w,
        render.bottom_px,
    );
    let bottom_bounds = Rectangle::new(
        Point::new(
            render.bounds.x + render.left_px,
            render.bounds.y + render.bounds.height - render.bottom_px,
        ),
        Size::new(center_dst_w, render.bottom_px),
    );
    emit_horiz_tiled_slice(
        batch,
        render.texture,
        bottom_bounds,
        bottom_uvs,
        center_src_w,
    );
}

fn emit_tile_slice_vert_edges(batch: &mut QuadBatch, render: TileSliceRender<'_>) {
    let Some((center_dst_h, center_src_h)) = tile_slice_center_height(render) else {
        return;
    };

    let left_uvs = atlas_subregion_uvs(
        render.texture,
        render.atlas_width_px,
        render.atlas_height_px,
        0.0,
        render.top_px,
        render.left_px,
        center_src_h,
    );
    let left_bounds = Rectangle::new(
        Point::new(render.bounds.x, render.bounds.y + render.top_px),
        Size::new(render.left_px, center_dst_h),
    );
    emit_vert_tiled_slice(batch, render.texture, left_bounds, left_uvs, center_src_h);

    let right_uvs = atlas_subregion_uvs(
        render.texture,
        render.atlas_width_px,
        render.atlas_height_px,
        render.atlas_width_px - render.right_px,
        render.top_px,
        render.right_px,
        center_src_h,
    );
    let right_bounds = Rectangle::new(
        Point::new(
            render.bounds.x + render.bounds.width - render.right_px,
            render.bounds.y + render.top_px,
        ),
        Size::new(render.right_px, center_dst_h),
    );
    emit_vert_tiled_slice(batch, render.texture, right_bounds, right_uvs, center_src_h);
}

fn tile_slice_center_width(render: TileSliceRender<'_>) -> Option<(f32, f32)> {
    let dst_w = render.bounds.width - render.left_px - render.right_px;
    let src_w = render.atlas_width_px - render.left_px - render.right_px;
    (dst_w > 0.0 && src_w > 0.0).then_some((dst_w, src_w))
}

fn tile_slice_center_height(render: TileSliceRender<'_>) -> Option<(f32, f32)> {
    let dst_h = render.bounds.height - render.top_px - render.bottom_px;
    let src_h = render.atlas_height_px - render.top_px - render.bottom_px;
    (dst_h > 0.0 && src_h > 0.0).then_some((dst_h, src_h))
}

fn stretch_slice_rects(render: StretchSliceRender<'_>) -> [(Rectangle, Rectangle); 9] {
    let bounds = render.bounds;
    let (left_uv, right_uv, top_uv, bottom_uv) = render.texture.uvs;
    let uv_w = right_uv - left_uv;
    let uv_h = bottom_uv - top_uv;
    let left_u = (render.left_px / render.atlas_width_px) * uv_w;
    let right_u = (render.right_px / render.atlas_width_px) * uv_w;
    let top_v = (render.top_px / render.atlas_height_px) * uv_h;
    let bottom_v = (render.bottom_px / render.atlas_height_px) * uv_h;

    let u1 = left_uv + left_u;
    let u2 = right_uv - right_u;
    let v1 = top_uv + top_v;
    let v2 = bottom_uv - bottom_v;

    let left_w = render.left_px;
    let right_w = render.right_px;
    let top_h = render.top_px;
    let bottom_h = render.bottom_px;
    let center_w = bounds.width - left_w - right_w;
    let center_h = bounds.height - top_h - bottom_h;

    let x0 = bounds.x;
    let x1 = bounds.x + left_w;
    let x2 = bounds.x + bounds.width - right_w;
    let y0 = bounds.y;
    let y1 = bounds.y + top_h;
    let y2 = bounds.y + bounds.height - bottom_h;

    [
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
    ]
}

fn push_textured_rects<const N: usize>(
    batch: &mut QuadBatch,
    texture: TexturedSlice<'_>,
    rects: [(Rectangle, Rectangle); N],
) {
    for (dst, src) in rects {
        batch.push_textured_path_uv(dst, src, texture.path, texture.tint, texture.blend);
    }
}

fn push_cropped_slice_quad(
    batch: &mut QuadBatch,
    texture: TexturedSlice<'_>,
    bounds: Rectangle,
    source_uvs: Rectangle,
) {
    if bounds.width <= 0.0
        || bounds.height <= 0.0
        || source_uvs.width <= 0.0
        || source_uvs.height <= 0.0
    {
        return;
    }

    let (path, full_uvs) = crop_flattened_subregion(texture.path, source_uvs);
    batch.push_textured_path_uv(bounds, full_uvs, &path, texture.tint, texture.blend);
}

fn push_cropped_slice_rects<const N: usize>(
    batch: &mut QuadBatch,
    texture: TexturedSlice<'_>,
    rects: [(Rectangle, Rectangle); N],
) {
    for (bounds, source_uvs) in rects {
        push_cropped_slice_quad(batch, texture, bounds, source_uvs);
    }
}

fn emit_horiz_tiled_slice(
    batch: &mut QuadBatch,
    texture: TexturedSlice<'_>,
    bounds: Rectangle,
    source_uvs: Rectangle,
    tile_w: f32,
) {
    if bounds.width <= 0.0
        || bounds.height <= 0.0
        || tile_w <= 0.0
        || source_uvs.width <= 0.0
        || source_uvs.height <= 0.0
    {
        return;
    }

    let (path, full_uvs) = crop_flattened_subregion(texture.path, source_uvs);
    emit_horiz_tiles(
        batch,
        bounds,
        &full_uvs,
        &path,
        tile_w,
        texture.tint,
        texture.blend,
    );
}

fn emit_vert_tiled_slice(
    batch: &mut QuadBatch,
    texture: TexturedSlice<'_>,
    bounds: Rectangle,
    source_uvs: Rectangle,
    tile_h: f32,
) {
    if bounds.width <= 0.0
        || bounds.height <= 0.0
        || tile_h <= 0.0
        || source_uvs.width <= 0.0
        || source_uvs.height <= 0.0
    {
        return;
    }

    let (path, full_uvs) = crop_flattened_subregion(texture.path, source_uvs);
    emit_vert_tiles(
        batch,
        bounds,
        &full_uvs,
        &path,
        tile_h,
        texture.tint,
        texture.blend,
    );
}

fn emit_grid_tiled_slice(
    batch: &mut QuadBatch,
    texture: TexturedSlice<'_>,
    bounds: Rectangle,
    source_uvs: Rectangle,
    tile_w: f32,
    tile_h: f32,
) {
    if bounds.width <= 0.0
        || bounds.height <= 0.0
        || tile_w <= 0.0
        || tile_h <= 0.0
        || source_uvs.width <= 0.0
        || source_uvs.height <= 0.0
    {
        return;
    }

    let (path, full_uvs) = crop_flattened_subregion(texture.path, source_uvs);
    emit_grid_tiles(
        batch,
        bounds,
        &full_uvs,
        &path,
        tile_w,
        tile_h,
        texture.tint,
        texture.blend,
    );
}

fn atlas_subregion_uvs(
    texture: TexturedSlice<'_>,
    atlas_width_px: f32,
    atlas_height_px: f32,
    left_px: f32,
    top_px: f32,
    width_px: f32,
    height_px: f32,
) -> Rectangle {
    let (uv_left, uv_right, uv_top, uv_bottom) = texture.uvs;
    let uv_w = uv_right - uv_left;
    let uv_h = uv_bottom - uv_top;
    Rectangle::new(
        Point::new(
            uv_left + (left_px / atlas_width_px) * uv_w,
            uv_top + (top_px / atlas_height_px) * uv_h,
        ),
        Size::new(
            (width_px / atlas_width_px) * uv_w,
            (height_px / atlas_height_px) * uv_h,
        ),
    )
}

fn crop_flattened_subregion(tex_path: &str, sub_uvs: Rectangle) -> (String, Rectangle) {
    if is_full_uv_rect(sub_uvs) {
        return (tex_path.to_string(), full_uv_rect());
    }

    let (base_path, parent_uvs) = decode_crop_path(tex_path).unwrap_or((tex_path, full_uv_rect()));
    let left = parent_uvs.x + sub_uvs.x * parent_uvs.width;
    let right = parent_uvs.x + (sub_uvs.x + sub_uvs.width) * parent_uvs.width;
    let top = parent_uvs.y + sub_uvs.y * parent_uvs.height;
    let bottom = parent_uvs.y + (sub_uvs.y + sub_uvs.height) * parent_uvs.height;
    (
        format!("{base_path}@crop:{left:.6},{right:.6},{top:.6},{bottom:.6}"),
        full_uv_rect(),
    )
}

fn decode_crop_path(path: &str) -> Option<(&str, Rectangle)> {
    let crop_start = path.find("@crop:")?;
    let base_path = &path[..crop_start];
    let coords: Vec<f32> = path[crop_start + 6..]
        .split(',')
        .filter_map(|part| part.parse().ok())
        .collect();
    if coords.len() != 4 {
        return None;
    }

    Some((
        base_path,
        Rectangle::new(
            Point::new(coords[0], coords[2]),
            Size::new(coords[1] - coords[0], coords[3] - coords[2]),
        ),
    ))
}

fn full_uv_rect() -> Rectangle {
    Rectangle::new(Point::ORIGIN, Size::new(1.0, 1.0))
}

fn is_full_uv_rect(rect: Rectangle) -> bool {
    rect.x.abs() < 0.001
        && rect.y.abs() < 0.001
        && (rect.width - 1.0).abs() < 0.001
        && (rect.height - 1.0).abs() < 0.001
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
    fill_uvs: Option<TextureUvs>,
    atlas_tex_coords: Option<TextureUvs>,
) -> (String, Option<TextureUvs>) {
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
    tex_coords: Option<TextureUvs>,
    bar_fill: Option<&StatusBarFill>,
) -> (Rectangle, Option<TextureUvs>) {
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
    use super::{emit_texture_fill, remap_atlas_crop};
    use crate::render::QuadBatch;
    use crate::widget::{Frame, WidgetType};
    use iced::{Point, Rectangle, Size};

    fn texture_frame_with_atlas(name: &str) -> Frame {
        let mut frame = Frame::new(WidgetType::Texture, None, None);
        frame.atlas = Some(name.to_string());
        frame
    }

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

    #[test]
    fn stretch_atlas_slices_emit_nine_quads() {
        let mut batch = QuadBatch::new();
        let frame = texture_frame_with_atlas("common-button-tertiary-normal");

        emit_texture_fill(
            &mut batch,
            Rectangle::new(Point::ORIGIN, Size::new(160.0, 32.0)),
            Some((0.0, 1.0, 0.0, 1.0)),
            "stretch-path",
            &frame,
            [1.0, 1.0, 1.0, 1.0],
            1.0,
        );

        assert_eq!(batch.vertices.len(), 36);
        assert_eq!(batch.texture_requests.len(), 9);
    }

    #[test]
    fn tile_atlas_slices_emit_cropped_tiled_quads() {
        let mut batch = QuadBatch::new();
        let frame = texture_frame_with_atlas("questlog-frame");

        emit_texture_fill(
            &mut batch,
            Rectangle::new(Point::ORIGIN, Size::new(120.0, 120.0)),
            Some((0.0, 1.0, 0.0, 1.0)),
            "tile-path@crop:0.001953,0.210938,0.076172,0.285156",
            &frame,
            [1.0, 1.0, 1.0, 1.0],
            1.0,
        );

        assert_eq!(batch.vertices.len(), 1024);
        assert_eq!(batch.texture_requests.len(), 256);
        assert!(
            batch
                .texture_requests
                .iter()
                .all(|request| request.path.matches("@crop:").count() == 1),
            "tile atlas slices should flatten crop paths, got: {:?}",
            batch.texture_requests
        );
    }
}
