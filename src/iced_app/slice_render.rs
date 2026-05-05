//! Slice rendering: three-slice, stretch (nine-slice), and tile-slice emission.
//!
//! Extracted from quad_builders_textures.rs to keep file sizes manageable.

use iced::{Point, Rectangle, Size};

use crate::render::{BlendMode, QuadBatch};

use super::tiling::{
    GridTileStrip, HorizTileStrip, VertTileStrip, emit_grid_tiles, emit_horiz_tiles,
    emit_vert_tiles,
};

pub(super) type TextureUvs = (f32, f32, f32, f32);

#[derive(Clone, Copy)]
pub(super) struct TexturedSlice<'a> {
    pub path: &'a str,
    pub uvs: TextureUvs,
    pub tint: [f32; 4],
    pub blend: BlendMode,
}

#[derive(Clone, Copy)]
pub(super) struct ThreeSliceRender<'a> {
    pub bounds: Rectangle,
    pub texture: TexturedSlice<'a>,
    pub left_cap_px: f32,
    pub right_cap_px: f32,
    pub atlas_width_px: f32,
}

#[derive(Clone, Copy)]
pub(super) struct StretchSliceRender<'a> {
    pub bounds: Rectangle,
    pub texture: TexturedSlice<'a>,
    pub left_px: f32,
    pub top_px: f32,
    pub right_px: f32,
    pub bottom_px: f32,
    pub atlas_width_px: f32,
    pub atlas_height_px: f32,
}

#[derive(Clone, Copy)]
pub(super) struct TileSliceRender<'a> {
    pub bounds: Rectangle,
    pub texture: TexturedSlice<'a>,
    pub left_px: f32,
    pub top_px: f32,
    pub right_px: f32,
    pub bottom_px: f32,
    pub atlas_width_px: f32,
    pub atlas_height_px: f32,
}

/// Render an atlas texture as 3 horizontal slices (left cap, stretched middle, right cap).
pub(super) fn emit_three_slice_h_atlas(batch: &mut QuadBatch, render: ThreeSliceRender<'_>) {
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

pub(super) fn emit_stretch_slice_atlas(batch: &mut QuadBatch, render: StretchSliceRender<'_>) {
    push_textured_rects(batch, render.texture, stretch_slice_rects(render));
}

pub(super) fn emit_tile_slice_atlas(batch: &mut QuadBatch, render: TileSliceRender<'_>) {
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

pub(super) fn tile_slice_center_width(render: TileSliceRender<'_>) -> Option<(f32, f32)> {
    let dst_w = render.bounds.width - render.left_px - render.right_px;
    let src_w = render.atlas_width_px - render.left_px - render.right_px;
    (dst_w > 0.0 && src_w > 0.0).then_some((dst_w, src_w))
}

pub(super) fn tile_slice_center_height(render: TileSliceRender<'_>) -> Option<(f32, f32)> {
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

    if emit_unit_repeat_quad(batch, texture, bounds, source_uvs, tile_w) {
        return;
    }

    let (path, full_uvs) = crop_flattened_subregion(texture.path, source_uvs);
    emit_horiz_tiles(
        batch,
        HorizTileStrip {
            bounds,
            uvs: &full_uvs,
            tex_path: &path,
            tile_w,
            tint: texture.tint,
            blend: texture.blend,
        },
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

    if emit_unit_repeat_quad(batch, texture, bounds, source_uvs, tile_h) {
        return;
    }

    let (path, full_uvs) = crop_flattened_subregion(texture.path, source_uvs);
    emit_vert_tiles(
        batch,
        VertTileStrip {
            bounds,
            uvs: &full_uvs,
            tex_path: &path,
            tile_h,
            tint: texture.tint,
            blend: texture.blend,
        },
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

    if try_emit_collapsed_grid_repeat(batch, texture, bounds, source_uvs, tile_w, tile_h) {
        return;
    }

    let (path, full_uvs) = crop_flattened_subregion(texture.path, source_uvs);
    emit_grid_tiles(
        batch,
        GridTileStrip {
            bounds,
            uvs: &full_uvs,
            tex_path: &path,
            tile_w,
            tile_h,
            tint: texture.tint,
            blend: texture.blend,
        },
    );
}

fn emit_unit_repeat_quad(
    batch: &mut QuadBatch,
    texture: TexturedSlice<'_>,
    bounds: Rectangle,
    source_uvs: Rectangle,
    tile_extent: f32,
) -> bool {
    if tile_extent > 1.0 {
        return false;
    }
    push_cropped_slice_quad(batch, texture, bounds, source_uvs);
    true
}

fn try_emit_collapsed_grid_repeat(
    batch: &mut QuadBatch,
    texture: TexturedSlice<'_>,
    bounds: Rectangle,
    source_uvs: Rectangle,
    tile_w: f32,
    tile_h: f32,
) -> bool {
    if tile_w <= 1.0 && tile_h <= 1.0 {
        push_cropped_slice_quad(batch, texture, bounds, source_uvs);
        return true;
    }
    if tile_w <= 1.0 {
        emit_vert_tiled_slice(batch, texture, bounds, source_uvs, tile_h);
        return true;
    }
    if tile_h <= 1.0 {
        emit_horiz_tiled_slice(batch, texture, bounds, source_uvs, tile_w);
        return true;
    }
    false
}

pub(super) fn atlas_subregion_uvs(
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

pub(super) fn crop_flattened_subregion(tex_path: &str, sub_uvs: Rectangle) -> (String, Rectangle) {
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

pub(super) fn decode_crop_path(path: &str) -> Option<(&str, Rectangle)> {
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

pub(super) fn full_uv_rect() -> Rectangle {
    Rectangle::new(Point::ORIGIN, Size::new(1.0, 1.0))
}

pub(super) fn is_full_uv_rect(rect: Rectangle) -> bool {
    rect.x.abs() < 0.001
        && rect.y.abs() < 0.001
        && (rect.width - 1.0).abs() < 0.001
        && (rect.height - 1.0).abs() < 0.001
}
