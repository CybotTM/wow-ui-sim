//! Rendering module for WoW UI frames.
//!
//! Provides both canvas-based (CPU) and shader-based (GPU) rendering.

pub mod font;
#[cfg(feature = "gui")]
pub mod glyph;
#[cfg(feature = "gui")]
pub mod shader;
#[cfg(feature = "gui")]
pub mod headless;
#[cfg(feature = "gui")]
pub mod text;
#[cfg(feature = "gui")]
pub mod texture;

pub use crate::BlendMode;
pub use font::WowFontSystem;

/// Strip WoW markup from text: textures (`|T...|t`), atlases (`|A...|a`),
/// colors (`|cXXXXXXXX`/`|r`), and hyperlinks (`|H...|h`/`|h`).
/// Preserves plain text content visible to the player.
pub fn strip_wow_markup(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '|' {
            if let Some(&next) = chars.peek() {
                if next == 'T' || next == 'A' {
                    let end = if next == 'T' { 't' } else { 'a' };
                    chars.next();
                    skip_until_marker(&mut chars, end);
                    continue;
                }
                if next == 'H' {
                    chars.next();
                    skip_until_marker(&mut chars, 'h');
                    continue;
                }
                if next == 'h' || next == 'r' {
                    chars.next();
                    continue;
                }
                if next == 'c' {
                    chars.next();
                    for _ in 0..8 { chars.next(); }
                    continue;
                }
            }
        }
        result.push(c);
    }

    result
}

fn skip_until_marker(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    marker: char,
) {
    while let Some(ch) = chars.next() {
        if ch == '|' && chars.peek() == Some(&marker) {
            chars.next();
            break;
        }
    }
}

#[cfg(feature = "gui")]
pub use shader::{
    FrameQuadSnapshot, GpuTextureAtlas, GpuTextureData, NineSliceTextures, QuadBatch,
    QuadVertex, TextureEntry, TextureRequest, WowUiPipeline, WowUiPrimitive,
    WowUiProgram, load_texture_or_crop,
};
#[cfg(feature = "gui")]
pub use glyph::{emit_text_quads, GlyphAtlas};
#[cfg(feature = "gui")]
pub use text::TextRenderer;
#[cfg(feature = "gui")]
pub use texture::{
    draw_horizontal_slice_texture, draw_nine_slice_texture, draw_scaled_texture,
    draw_texture_with_texcoords, draw_tiled_texture,
};
