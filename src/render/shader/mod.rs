//! GPU-accelerated shader rendering for WoW UI frames.
//!
//! This module provides wgpu-based rendering using iced's `shader::Primitive` trait,
//! replacing the CPU-bound canvas rendering with GPU-accelerated quad batching.

pub use sim_render::shader::atlas;
pub use sim_render::shader::atlas_bc;
pub(crate) use sim_render::shader::pipeline;
pub mod primitive;
mod primitive_textures;
pub(crate) use sim_render::shader::program;
pub(crate) use sim_render::shader::quad;
pub(crate) use sim_render::shader::quad_batch_extras;
pub(crate) use sim_render::shader::quad_nine_slice;

pub use atlas::{GLYPH_ATLAS_TEX_INDEX, GpuTextureAtlas, TextureEntry};
pub use atlas_bc::{BcFormat, BcTextureEntry};
pub use pipeline::WowUiPipeline;
pub use primitive::{GpuBcTextureData, GpuTextureData, WowUiPrimitive, load_texture_or_crop};
pub use program::WowUiProgram;
pub use quad::FLAG_CIRCLE_CLIP;
pub use quad::FLAG_DESATURATE;
pub use quad::{BlendMode, FrameQuadSnapshot, QuadBatch, QuadVertex, TextureRequest};
pub(crate) use quad_batch_extras::ThreeSlicePathParams;
pub use quad_nine_slice::NineSliceTextures;
