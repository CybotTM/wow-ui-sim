//! GPU-accelerated shader rendering for WoW UI frames.
//!
//! This module provides wgpu-based rendering using iced's `shader::Primitive` trait,
//! replacing the CPU-bound canvas rendering with GPU-accelerated quad batching.

pub mod atlas;
mod pipeline;
pub mod primitive;
mod program;
mod quad;
mod quad_batch_extras;
mod quad_nine_slice;

pub use atlas::{BcFormat, BcTextureEntry, GLYPH_ATLAS_TEX_INDEX, GpuTextureAtlas, TextureEntry};
pub use pipeline::WowUiPipeline;
pub use primitive::{GpuBcTextureData, GpuTextureData, WowUiPrimitive, load_texture_or_crop};
pub use program::WowUiProgram;
pub use quad::FLAG_CIRCLE_CLIP;
pub use quad::FLAG_DESATURATE;
pub use quad::{BlendMode, FrameQuadSnapshot, QuadBatch, QuadVertex, TextureRequest};
pub use quad_nine_slice::NineSliceTextures;
