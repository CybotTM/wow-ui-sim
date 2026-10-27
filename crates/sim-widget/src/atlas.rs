/// A single piece of a nine-slice atlas kit.
#[derive(Debug, Clone)]
pub struct NineSlicePiece {
    /// Texture file path (WoW-style).
    pub file: &'static str,
    /// UV coordinates (left, right, top, bottom).
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    /// Piece dimensions in pixels.
    pub width: u32,
    pub height: u32,
}

/// Nine-slice atlas kit: 4 corners + 4 tiling edges + optional center.
#[derive(Debug, Clone)]
pub struct NineSliceAtlasInfo {
    pub corner_tl: NineSlicePiece,
    pub corner_tr: NineSlicePiece,
    pub corner_bl: NineSlicePiece,
    pub corner_br: NineSlicePiece,
    pub edge_top: NineSlicePiece,
    pub edge_bottom: NineSlicePiece,
    pub edge_left: NineSlicePiece,
    pub edge_right: NineSlicePiece,
    pub center: Option<NineSlicePiece>,
}
