//! Profile-neutral GPU atlas and quad data.
pub mod atlas;
pub mod atlas_bc;
mod atlas_bind_groups;
pub mod pipeline;
pub mod primitive;
pub mod primitive_data;
pub mod program;
pub mod quad;
pub mod quad_batch_extras;
pub mod quad_nine_slice;
pub use atlas::{GpuTextureAtlas, TextureEntry};
pub use pipeline::WowUiPipeline;
pub use primitive::{GpuBcTextureData, GpuTextureData, WowUiPrimitive};
pub use program::WowUiProgram;
pub use quad::{FrameQuadSnapshot, QuadBatch, QuadVertex, TextureRequest};
