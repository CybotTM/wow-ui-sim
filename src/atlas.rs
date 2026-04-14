//! Atlas lookup with fallback resolution for WoW-style atlas names.
//!
//! Wraps the auto-generated atlas data and adds resolution logic for
//! size-suffixed entries (e.g. "coin-copper" → "coin-copper-20x20").

pub use crate::atlas_data::{
    ATLAS_DB, AtlasInfo, AtlasLookup, AtlasSliceInfo, AtlasSliceMode, get_atlas_slice_info,
};
pub use crate::atlas_elements::get_atlas_name_by_element_id;

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

/// Check if an atlas name is a nine-slice kit prefix and return all pieces.
///
/// Detection: if `{lowercase(name)}-nineslice-cornertopleft` exists in ATLAS_DB,
/// this is a nine-slice kit. Returns `None` if any required piece is missing.
pub fn get_nine_slice_atlas_info(name: &str) -> Option<NineSliceAtlasInfo> {
    let kit = name.to_lowercase();
    let probe = format!("{kit}-nineslice-cornertopleft");
    ATLAS_DB.get(&probe as &str)?;

    let piece = |key: &str| -> Option<NineSlicePiece> {
        let lookup = paired_2x_variant(key).or_else(|| ATLAS_DB.get(key))?;
        let from_2x = ATLAS_DB.get(key).is_none();
        let (width, height) = if from_2x {
            (
                (lookup.width as f32 / 2.0).round() as u32,
                (lookup.height as f32 / 2.0).round() as u32,
            )
        } else {
            (lookup.width, lookup.height)
        };
        Some(NineSlicePiece {
            file: lookup.file,
            left: lookup.left_tex_coord,
            right: lookup.right_tex_coord,
            top: lookup.top_tex_coord,
            bottom: lookup.bottom_tex_coord,
            width,
            height,
        })
    };

    Some(NineSliceAtlasInfo {
        corner_tl: piece(&format!("{kit}-nineslice-cornertopleft"))?,
        corner_tr: piece(&format!("{kit}-nineslice-cornertopright"))?,
        corner_bl: piece(&format!("{kit}-nineslice-cornerbottomleft"))?,
        corner_br: piece(&format!("{kit}-nineslice-cornerbottomright"))?,
        edge_top: piece(&format!("_{kit}-nineslice-edgetop"))?,
        edge_bottom: piece(&format!("_{kit}-nineslice-edgebottom"))?,
        edge_left: piece(&format!("!{kit}-nineslice-edgeleft"))?,
        edge_right: piece(&format!("!{kit}-nineslice-edgeright"))?,
        center: piece(&format!("{kit}-nineslice-center")),
    })
}

/// Common square sizes used in WoW's size-suffixed atlas entries.
const SIZE_SUFFIXES: &[u32] = &[16, 20, 32, 48, 64];
const RENDER_PREFERRED_2X_ATLASES: &[&str] = &["questlog-icon-ticksquare"];

fn exact_atlas_info(name: &str) -> Option<AtlasLookup> {
    crate::atlas_data::get_atlas_info(name)
}

fn paired_2x_variant(lower: &str) -> Option<&'static AtlasInfo> {
    if lower.ends_with("_1x")
        || lower.ends_with("-1x")
        || lower.ends_with("_2x")
        || lower.ends_with("-2x")
    {
        return None;
    }

    for sep in ["_", "-"] {
        let with_2x = format!("{lower}{sep}2x");
        if let Some(info) = ATLAS_DB.get(&with_2x as &str) {
            return Some(info);
        }
    }

    None
}

fn render_preferred_2x_variant(lower: &str) -> Option<&'static AtlasInfo> {
    if !RENDER_PREFERRED_2X_ATLASES.contains(&lower) {
        return None;
    }
    paired_2x_variant(lower)
}

/// Get atlas info by name (case-insensitive).
///
/// Resolution order:
/// 1. Exact match
/// 2. With `-NxN` size suffix (e.g. `coin-copper` → `coin-copper-20x20`)
/// 3. With `_2x` / `-2x` / `_1x` / `-1x` suffixes (e.g. `bags-item-slot64` → `-2x`)
pub fn get_atlas_info(name: &str) -> Option<AtlasLookup> {
    let lower = name.to_lowercase();

    if let Some(lookup) = exact_atlas_info(name) {
        return Some(lookup);
    }

    // Try with -NxN size suffixes
    for &size in SIZE_SUFFIXES {
        let suffixed = format!("{lower}-{size}x{size}");
        if let Some(info) = ATLAS_DB.get(&suffixed as &str) {
            return Some(AtlasLookup {
                info,
                is_2x_fallback: false,
            });
        }
    }

    // Try with _2x/_1x underscore and -2x/-1x hyphen suffixes
    for sep in ["_", "-"] {
        let with_2x = format!("{lower}{sep}2x");
        if let Some(info) = ATLAS_DB.get(&with_2x as &str) {
            return Some(AtlasLookup {
                info,
                is_2x_fallback: true,
            });
        }
        let with_1x = format!("{lower}{sep}1x");
        if let Some(info) = ATLAS_DB.get(&with_1x as &str) {
            return Some(AtlasLookup {
                info,
                is_2x_fallback: false,
            });
        }
    }

    // Blizzard typo corrections (divider→devider in atlas DB)
    try_spelling_corrections(&lower)
}

/// Get atlas info for rendering, preferring a paired 2x entry when one exists.
///
/// This keeps logical atlas dimensions unchanged while sourcing texels from
/// the higher-resolution atlas file.
pub fn get_render_atlas_info(name: &str) -> Option<AtlasLookup> {
    let lower = name.to_lowercase();

    if exact_atlas_info(name).is_some() {
        if let Some(info) = render_preferred_2x_variant(&lower) {
            return Some(AtlasLookup {
                info,
                is_2x_fallback: true,
            });
        }
        return exact_atlas_info(name);
    }

    get_atlas_info(name)
}

/// Atlas DB has some Blizzard typos. Try known corrections.
fn try_spelling_corrections(lower: &str) -> Option<AtlasLookup> {
    let corrected = lower.replace("divider", "devider");
    if corrected != *lower {
        return crate::atlas_data::get_atlas_info(&corrected);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{get_atlas_info, get_render_atlas_info};

    #[test]
    fn nine_slice_uses_2x_fallback_with_logical_sizes() {
        let ns_info = super::get_nine_slice_atlas_info("ui-frame-metal")
            .expect("metal nineslice should exist from 2x atlas fallback");
        let corner = super::ATLAS_DB
            .get("ui-frame-metal-cornertopleft-2x")
            .expect("metal corner +2x entry should exist");
        let edge_top = super::ATLAS_DB
            .get("_ui-frame-metal-edgetop-2x")
            .expect("metal edge top +2x entry should exist");

        assert_eq!(
            ns_info.corner_tl.width,
            (corner.width as f32 / 2.0).round() as u32
        );
        assert_eq!(
            ns_info.edge_top.width,
            (edge_top.width as f32 / 2.0).round() as u32
        );
        assert_eq!(
            ns_info.edge_top.height,
            (edge_top.height as f32 / 2.0).round() as u32
        );
    }

    #[test]
    fn exact_unsuffixed_atlas_beats_2x_fallback() {
        let lookup = get_atlas_info("glues-characterselect-card-singles")
            .expect("character select singles atlas should exist");
        assert!(!lookup.is_2x_fallback);
        assert_eq!(
            lookup.info.file,
            r"Interface\glues\characterselect\uicharacterselectglues"
        );
        assert_eq!(lookup.width(), 310);
        assert_eq!(lookup.height(), 89);
    }

    #[test]
    fn render_lookup_prefers_paired_2x_atlas_without_changing_logical_size() {
        let lookup = get_render_atlas_info("questlog-icon-ticksquare")
            .expect("quest log checkbox atlas should exist");
        assert!(lookup.is_2x_fallback);
        assert_eq!(lookup.info.file, r"Interface\questframe\questlogframe2x");
        assert_eq!(lookup.width(), 14);
        assert_eq!(lookup.height(), 14);
    }

    #[test]
    fn render_lookup_keeps_other_exact_atlases_on_their_base_texture() {
        let lookup =
            get_render_atlas_info("questlog-tab-side").expect("quest log tab atlas should exist");
        assert!(!lookup.is_2x_fallback);
        assert_eq!(lookup.info.file, r"Interface\questframe\questlogframe");
        assert_eq!(lookup.width(), 51);
        assert_eq!(lookup.height(), 60);
    }
}
