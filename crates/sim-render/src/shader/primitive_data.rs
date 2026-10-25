use super::atlas::BcFormat;
use std::sync::Arc;
use std::time::Duration;

/// Loaded texture data ready for GPU upload.
#[derive(Debug, Clone)]
pub struct GpuTextureData {
    /// Texture path (normalized).
    pub path: String,
    /// Image width.
    pub width: u32,
    /// Image height.
    pub height: u32,
    /// RGBA pixel data.
    pub rgba: Arc<[u8]>,
}

/// BC-compressed texture data ready for direct GPU upload (no CPU decode).
#[derive(Debug, Clone)]
pub struct GpuBcTextureData {
    /// Texture path (normalized).
    pub path: String,
    /// Image width.
    pub width: u32,
    /// Image height.
    pub height: u32,
    /// Raw BC block data (mip level 0).
    pub bc_data: Arc<[u8]>,
    /// BC compression format (BC1 = DXT1, BC3 = DXT3/DXT5).
    pub bc_format: BcFormat,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TextureLoadTelemetry {
    pub crop_decode_elapsed: Duration,
    pub crop_extract_elapsed: Duration,
    pub bc: sim_texture::BcLoadTelemetry,
    pub rgba: sim_texture::RgbaLoadTelemetry,
    pub total_elapsed: Duration,
}

/// Result of loading a texture: either RGBA (CPU-decoded) or BC (raw GPU-native).
pub enum LoadedTexture {
    Rgba(GpuTextureData),
    Bc(GpuBcTextureData),
}

pub fn bc_texture_dimensions_fit_gpu_atlas(width: u32, height: u32) -> bool {
    const BC_BLOCK_DIMENSION: u32 = 4;
    width.is_multiple_of(BC_BLOCK_DIMENSION)
        && height.is_multiple_of(BC_BLOCK_DIMENSION)
        && width <= super::atlas::BC_CELL_SIZE
        && height <= super::atlas::BC_CELL_SIZE
}
