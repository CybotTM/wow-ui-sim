//! Nine-slice atlas rendering: 4 corners + 4 tiled edges + optional center.

use iced::{Point, Rectangle, Size};

use crate::atlas::{NineSliceAtlasInfo, NineSlicePiece};
use crate::render::{BlendMode, QuadBatch};

use super::tiling::{crop_path_for_subregion, emit_horiz_tiles, emit_vert_tiles};

/// Build a `@crop:` path for a nine-slice piece so it gets its own GPU atlas slot,
/// preventing bilinear bleed into adjacent content in the source texture.
fn crop_piece(piece: &NineSlicePiece) -> (String, Rectangle) {
    let uvs = piece_uvs(piece);
    crop_path_for_subregion(piece.file, &uvs)
}

/// Emit a single nine-slice piece as a textured quad.
fn emit_piece(batch: &mut QuadBatch, bounds: Rectangle, piece: &NineSlicePiece, alpha: f32) {
    let (path, uvs) = crop_piece(piece);
    batch.push_textured_path_uv(bounds, uvs, &path, [1.0, 1.0, 1.0, alpha], BlendMode::Alpha);
}

/// UV rectangle for a nine-slice piece.
fn piece_uvs(p: &NineSlicePiece) -> Rectangle {
    Rectangle::new(
        Point::new(p.left, p.top),
        Size::new(p.right - p.left, p.bottom - p.top),
    )
}

/// Emit all four corners of a nine-slice kit.
fn emit_corners(batch: &mut QuadBatch, bounds: Rectangle, ns: &NineSliceAtlasInfo, alpha: f32) {
    let (left_w, right_w) = (ns.corner_tl.width as f32, ns.corner_tr.width as f32);
    let (top_h, bottom_h) = (ns.corner_tl.height as f32, ns.corner_bl.height as f32);

    let tl = Rectangle::new(bounds.position(), Size::new(left_w, top_h));
    let tr = Rectangle::new(
        Point::new(bounds.x + bounds.width - right_w, bounds.y),
        Size::new(right_w, top_h),
    );
    let bl = Rectangle::new(
        Point::new(bounds.x, bounds.y + bounds.height - bottom_h),
        Size::new(left_w, bottom_h),
    );
    let br = Rectangle::new(
        Point::new(bounds.x + bounds.width - right_w, bounds.y + bounds.height - bottom_h),
        Size::new(right_w, bottom_h),
    );

    emit_piece(batch, tl, &ns.corner_tl, alpha);
    emit_piece(batch, tr, &ns.corner_tr, alpha);
    emit_piece(batch, bl, &ns.corner_bl, alpha);
    emit_piece(batch, br, &ns.corner_br, alpha);
}

/// Emit tiled horizontal edges (top and bottom) between corners.
fn emit_horiz_edges(batch: &mut QuadBatch, bounds: Rectangle, ns: &NineSliceAtlasInfo, alpha: f32) {
    let edge_x = bounds.x + ns.corner_tl.width as f32;
    let edge_w = bounds.width - ns.corner_tl.width as f32 - ns.corner_tr.width as f32;
    if edge_w <= 0.0 {
        return;
    }

    let top_bounds = Rectangle::new(
        Point::new(edge_x, bounds.y),
        Size::new(edge_w, ns.edge_top.height as f32),
    );
    let (top_path, top_uvs) = crop_piece(&ns.edge_top);
    emit_horiz_tiles(batch, top_bounds, &top_uvs, &top_path, ns.edge_top.width as f32, [1.0, 1.0, 1.0, alpha], BlendMode::Alpha);

    let bot_bounds = Rectangle::new(
        Point::new(edge_x, bounds.y + bounds.height - ns.edge_bottom.height as f32),
        Size::new(edge_w, ns.edge_bottom.height as f32),
    );
    let (bot_path, bot_uvs) = crop_piece(&ns.edge_bottom);
    emit_horiz_tiles(batch, bot_bounds, &bot_uvs, &bot_path, ns.edge_bottom.width as f32, [1.0, 1.0, 1.0, alpha], BlendMode::Alpha);
}

/// Emit tiled vertical edges (left and right) between corners.
fn emit_vert_edges(batch: &mut QuadBatch, bounds: Rectangle, ns: &NineSliceAtlasInfo, alpha: f32) {
    let edge_y = bounds.y + ns.corner_tl.height as f32;
    let edge_h = bounds.height - ns.corner_tl.height as f32 - ns.corner_bl.height as f32;
    if edge_h <= 0.0 {
        return;
    }

    let left_bounds = Rectangle::new(
        Point::new(bounds.x, edge_y),
        Size::new(ns.edge_left.width as f32, edge_h),
    );
    let (left_path, left_uvs) = crop_piece(&ns.edge_left);
    emit_vert_tiles(batch, left_bounds, &left_uvs, &left_path, ns.edge_left.height as f32, [1.0, 1.0, 1.0, alpha], BlendMode::Alpha);

    let right_bounds = Rectangle::new(
        Point::new(bounds.x + bounds.width - ns.edge_right.width as f32, edge_y),
        Size::new(ns.edge_right.width as f32, edge_h),
    );
    let (right_path, right_uvs) = crop_piece(&ns.edge_right);
    emit_vert_tiles(batch, right_bounds, &right_uvs, &right_path, ns.edge_right.height as f32, [1.0, 1.0, 1.0, alpha], BlendMode::Alpha);
}

/// Emit a nine-slice atlas kit: 4 corners, 4 tiled edges, optional stretched center.
pub fn emit_nine_slice_atlas(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    ns: &NineSliceAtlasInfo,
    alpha: f32,
) {
    emit_corners(batch, bounds, ns, alpha);
    emit_horiz_edges(batch, bounds, ns, alpha);
    emit_vert_edges(batch, bounds, ns, alpha);

    if let Some(ref center) = ns.center {
        let cx = bounds.x + ns.corner_tl.width as f32;
        let cy = bounds.y + ns.corner_tl.height as f32;
        let cw = bounds.width - ns.corner_tl.width as f32 - ns.corner_tr.width as f32;
        let ch = bounds.height - ns.corner_tl.height as f32 - ns.corner_bl.height as f32;
        if cw > 0.0 && ch > 0.0 {
            emit_piece(batch, Rectangle::new(Point::new(cx, cy), Size::new(cw, ch)), center, alpha);
        }
    }
}

/// Emit a nine-slice border (corners + edges) with a solid-color center fill.
///
/// Used for tooltips where `SetCenterColor` tints the center texture to a solid color.
/// `center_overlap` extends the center fill into the corners by the given number of pixels
/// (WoW's TooltipDefaultLayout uses 4px via anchor offsets `x=-4, y=4, x1=4, y1=-4`).
pub fn emit_nine_slice_with_center_color(
    batch: &mut QuadBatch,
    bounds: Rectangle,
    ns: &NineSliceAtlasInfo,
    alpha: f32,
    center_color: [f32; 4],
    center_overlap: f32,
) {
    // Solid center fill (drawn first, behind the border pieces).
    // WoW anchors the center from TopLeftCorner.BOTTOMRIGHT to BottomRightCorner.TOPLEFT
    // with negative insets, so the fill extends under the corners to plug transparent areas.
    let cx = bounds.x + ns.corner_tl.width as f32 - center_overlap;
    let cy = bounds.y + ns.corner_tl.height as f32 - center_overlap;
    let cw = bounds.width - ns.corner_tl.width as f32 - ns.corner_tr.width as f32 + center_overlap * 2.0;
    let ch = bounds.height - ns.corner_tl.height as f32 - ns.corner_bl.height as f32 + center_overlap * 2.0;
    if cw > 0.0 && ch > 0.0 {
        batch.push_solid(Rectangle::new(Point::new(cx, cy), Size::new(cw, ch)), center_color);
    }

    emit_corners(batch, bounds, ns, alpha);
    emit_horiz_edges(batch, bounds, ns, alpha);
    emit_vert_edges(batch, bounds, ns, alpha);
}
