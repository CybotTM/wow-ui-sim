//! Profile-neutral quad batching and nine-slice geometry.
pub mod quad;
pub mod quad_batch_extras;
pub mod quad_nine_slice;
pub use quad::{FrameQuadSnapshot, QuadBatch, QuadVertex, TextureRequest};
