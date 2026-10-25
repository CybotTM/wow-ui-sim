//! GPU primitive facade plus composition-owned texture loading.
#[cfg(test)]
use super::primitive_textures::decode_crop_request;
pub use super::primitive_textures::{
    load_texture_or_crop, load_texture_prefer_bc, load_texture_prefer_bc_with_telemetry,
};
pub use sim_render::shader::primitive::*;
#[cfg(test)]
#[path = "primitive_tests.rs"]
mod tests;
