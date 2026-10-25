use crate::render::shader::atlas::BcFormat;
use std::sync::Arc;
use std::time::{Duration, Instant};

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
    pub bc: crate::texture::BcLoadTelemetry,
    pub rgba: crate::texture::RgbaLoadTelemetry,
    pub total_elapsed: Duration,
}

/// Result of loading a texture: either RGBA (CPU-decoded) or BC (raw GPU-native).
pub enum LoadedTexture {
    Rgba(GpuTextureData),
    Bc(GpuBcTextureData),
}

/// Load a texture by path, handling `@crop:` paths by extracting a sub-region.
///
/// Atlas sub-region paths have format `"base_path@crop:left,right,top,bottom"` where
/// left/right/top/bottom are UV coordinates in the source texture. The sub-region is
/// extracted at native resolution so small atlas entries aren't degraded by downscaling
/// the full oversized texture into a 512px GPU atlas cell.
pub fn load_texture_or_crop(
    tex_mgr: &mut crate::texture::TextureManager,
    path: &str,
) -> Option<GpuTextureData> {
    match load_texture_prefer_bc(tex_mgr, path)? {
        LoadedTexture::Rgba(data) => Some(data),
        LoadedTexture::Bc(_) => {
            // Caller expects RGBA - fall back to standard decode.
            let tex_data = tex_mgr.load(path)?;
            Some(GpuTextureData {
                path: path.to_string(),
                width: tex_data.width,
                height: tex_data.height,
                rgba: Arc::clone(&tex_data.pixels),
            })
        }
    }
}

/// Load a texture, preferring raw BC when the source is a BLP with DXT compression.
///
/// Returns `Bc` for BLP files with DXT content (dimensions must be multiples of 4),
/// falls back to `Rgba` for everything else (webp, png, non-DXT BLP, crop requests).
pub fn load_texture_prefer_bc(
    tex_mgr: &mut crate::texture::TextureManager,
    path: &str,
) -> Option<LoadedTexture> {
    load_texture_prefer_bc_with_telemetry(tex_mgr, path).0
}

pub fn load_texture_prefer_bc_with_telemetry(
    tex_mgr: &mut crate::texture::TextureManager,
    path: &str,
) -> (Option<LoadedTexture>, TextureLoadTelemetry) {
    let start = Instant::now();
    if path.contains("@crop:") {
        let (loaded, mut telemetry) = load_cropped_texture_with_telemetry(tex_mgr, path);
        telemetry.total_elapsed = start.elapsed();
        return (loaded, telemetry);
    }
    let (bc_texture, mut telemetry) = try_load_bc_texture_with_telemetry(tex_mgr, path);
    if bc_texture.is_some() {
        telemetry.total_elapsed = start.elapsed();
        return (bc_texture, telemetry);
    }
    let rgba_start = Instant::now();
    let (loaded, rgba_telemetry) = load_rgba_texture_with_telemetry(tex_mgr, path);
    telemetry.rgba = rgba_telemetry;
    if telemetry.rgba.total_elapsed.is_zero() {
        telemetry.rgba.total_elapsed = rgba_start.elapsed();
    }
    telemetry.total_elapsed = start.elapsed();
    (loaded, telemetry)
}

fn load_cropped_texture_with_telemetry(
    tex_mgr: &mut crate::texture::TextureManager,
    path: &str,
) -> (Option<LoadedTexture>, TextureLoadTelemetry) {
    let mut telemetry = TextureLoadTelemetry::default();
    if let Some(tex_data) = tex_mgr.get_cached_crop_request(path) {
        return (Some(rgba_upload_from_texture(path, tex_data)), telemetry);
    }
    let decode_start = Instant::now();
    let Some((base_path, x, y, crop_w, crop_h)) = decode_crop_request(tex_mgr, path) else {
        telemetry.crop_decode_elapsed = decode_start.elapsed();
        return (None, telemetry);
    };
    telemetry.crop_decode_elapsed = decode_start.elapsed();
    let crop_start = Instant::now();
    let Some(tex_data) = tex_mgr.load_sub_region(base_path, x, y, crop_w, crop_h) else {
        telemetry.crop_extract_elapsed = crop_start.elapsed();
        return (None, telemetry);
    };
    telemetry.crop_extract_elapsed = crop_start.elapsed();
    let cropped_texture = tex_data.clone();
    if tex_mgr
        .cache_crop_request_alias(path, &cropped_texture)
        .is_none()
    {
        return (None, telemetry);
    }
    (
        Some(rgba_upload_from_texture(path, &cropped_texture)),
        telemetry,
    )
}

fn rgba_upload_from_texture(path: &str, tex_data: &crate::texture::TextureData) -> LoadedTexture {
    LoadedTexture::Rgba(GpuTextureData {
        path: path.to_string(),
        width: tex_data.width,
        height: tex_data.height,
        rgba: Arc::clone(&tex_data.pixels),
    })
}

fn try_load_bc_texture_with_telemetry(
    tex_mgr: &mut crate::texture::TextureManager,
    path: &str,
) -> (Option<LoadedTexture>, TextureLoadTelemetry) {
    let mut telemetry = TextureLoadTelemetry::default();
    if !crate::render::shader::atlas::is_bc_supported() {
        return (None, telemetry);
    }
    let (bc_result, bc_telemetry) = tex_mgr.load_bc_with_telemetry(path);
    telemetry.bc = bc_telemetry;
    let Some(bc_result) = bc_result else {
        return (None, telemetry);
    };
    if !bc_texture_dimensions_fit_gpu_atlas(bc_result.width, bc_result.height) {
        return (None, telemetry);
    }
    (
        Some(LoadedTexture::Bc(GpuBcTextureData {
            path: path.to_string(),
            width: bc_result.width,
            height: bc_result.height,
            bc_data: Arc::clone(&bc_result.bc_data),
            bc_format: bc_result.format.into(),
        })),
        telemetry,
    )
}

pub(crate) fn bc_texture_dimensions_fit_gpu_atlas(width: u32, height: u32) -> bool {
    const BC_BLOCK_DIMENSION: u32 = 4;
    width.is_multiple_of(BC_BLOCK_DIMENSION)
        && height.is_multiple_of(BC_BLOCK_DIMENSION)
        && width <= crate::render::shader::atlas::BC_CELL_SIZE
        && height <= crate::render::shader::atlas::BC_CELL_SIZE
}

fn load_rgba_texture_with_telemetry(
    tex_mgr: &mut crate::texture::TextureManager,
    path: &str,
) -> (Option<LoadedTexture>, crate::texture::RgbaLoadTelemetry) {
    let (tex_data, telemetry) = tex_mgr.load_with_telemetry(path);
    let Some(tex_data) = tex_data else {
        return (None, telemetry);
    };
    (
        Some(LoadedTexture::Rgba(GpuTextureData {
            path: path.to_string(),
            width: tex_data.width,
            height: tex_data.height,
            rgba: Arc::clone(&tex_data.pixels),
        })),
        telemetry,
    )
}

pub(crate) fn decode_crop_request<'a>(
    tex_mgr: &mut crate::texture::TextureManager,
    path: &'a str,
) -> Option<(&'a str, u32, u32, u32, u32)> {
    let crop_start = path.find("@crop:")?;
    let base_path = &path[..crop_start];
    let crop_str = &path[crop_start + 6..];
    let coords: Vec<f32> = crop_str.split(',').filter_map(|s| s.parse().ok()).collect();
    if coords.len() != 4 {
        return None;
    }
    let (cl, cr, ct, cb) = (coords[0], coords[1], coords[2], coords[3]);
    let (w, h) = tex_mgr.get_or_load_texture_size(base_path)?;
    let x0 = (cl * w as f32).round() as u32;
    let x1 = (cr * w as f32).round() as u32;
    let y0 = (ct * h as f32).round() as u32;
    let y1 = (cb * h as f32).round() as u32;
    let crop_w = x1.saturating_sub(x0).max(1).min(w);
    let crop_h = y1.saturating_sub(y0).max(1).min(h);
    Some((
        base_path,
        x0.min(w.saturating_sub(1)),
        y0.min(h.saturating_sub(1)),
        crop_w,
        crop_h,
    ))
}
