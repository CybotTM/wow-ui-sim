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

#[cfg(feature = "gui")]
pub use shader::{
    GpuTextureAtlas, GpuTextureData, NineSliceTextures, QuadBatch, QuadVertex,
    TextureEntry, TextureRequest, WowUiPipeline, WowUiPrimitive, WowUiProgram,
    load_texture_or_crop,
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
