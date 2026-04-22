//! WoW UI shader primitive implementation.

use super::{QuadBatch, WowUiPipeline};
use iced::Rectangle;
use iced::widget::shader::{self, Viewport};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
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

use crate::render::shader::atlas::BcFormat;

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

/// Load a texture by path, handling `@crop:` paths by extracting a sub-region.
///
/// Atlas sub-region paths have format `"base_path@crop:left,right,top,bottom"` where
/// left/right/top/bottom are UV coordinates in the source texture. The sub-region is
/// extracted at native resolution so small atlas entries aren't degraded by downscaling
/// the full oversized texture into a 512px GPU atlas cell.
/// Result of loading a texture — either RGBA (CPU-decoded) or BC (raw GPU-native).
pub enum LoadedTexture {
    Rgba(GpuTextureData),
    Bc(GpuBcTextureData),
}

pub fn load_texture_or_crop(
    tex_mgr: &mut crate::texture::TextureManager,
    path: &str,
) -> Option<GpuTextureData> {
    match load_texture_prefer_bc(tex_mgr, path)? {
        LoadedTexture::Rgba(data) => Some(data),
        LoadedTexture::Bc(_) => {
            // Caller expects RGBA — fall back to standard decode
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
        return (
            Some(LoadedTexture::Rgba(GpuTextureData {
                path: path.to_string(),
                width: tex_data.width,
                height: tex_data.height,
                rgba: Arc::clone(&tex_data.pixels),
            })),
            telemetry,
        );
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
        Some(LoadedTexture::Rgba(GpuTextureData {
            path: path.to_string(),
            width: cropped_texture.width,
            height: cropped_texture.height,
            rgba: Arc::clone(&cropped_texture.pixels),
        })),
        telemetry,
    )
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

fn bc_texture_dimensions_fit_gpu_atlas(width: u32, height: u32) -> bool {
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

fn decode_crop_request<'a>(
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

#[cfg(test)]
#[path = "primitive_tests.rs"]
mod tests;

use crate::widget::FrameStrata;

/// Primitive data for rendering WoW UI frames.
///
/// Per-strata batches: each `FrameStrata` gets its own vertex/index data on
/// the GPU.  Only dirty strata carry `Some(batch)` — clean strata are `None`
/// and the pipeline keeps their GPU buffers from the previous frame.
#[derive(Debug)]
pub struct WowUiPrimitive {
    /// Per-strata quad batches. Index = `FrameStrata::as_index()`.
    /// `Some` = dirty (re-upload), `None` = clean (pipeline keeps old buffer).
    pub strata_batches: [Option<Arc<QuadBatch>>; FrameStrata::COUNT],
    /// Small overlay batch (cursor, hover highlight) appended after world quads.
    pub overlay: QuadBatch,
    /// Background clear color.
    pub clear_color: [f32; 4],
    /// Texture data to upload (path -> image data).
    pub textures: Vec<GpuTextureData>,
    /// BC-compressed texture data to upload directly to the GPU BC atlas.
    pub bc_textures: Vec<GpuBcTextureData>,
    /// Glyph atlas RGBA data for text rendering (2048x2048).
    pub glyph_atlas_data: Option<Vec<u8>>,
    /// Size of the glyph atlas (width = height).
    pub glyph_atlas_size: u32,
    /// Shared app-side tracker for staged texture paths awaiting successful atlas upload.
    pub gpu_uploaded_textures: Option<Arc<Mutex<HashSet<String>>>>,
    /// Shared app-side tracker for texture paths confirmed ready in the atlas.
    pub gpu_ready_textures: Option<Arc<Mutex<HashSet<String>>>>,
    /// Shared app-side tracker for texture paths that must retry on the RGBA atlas.
    pub gpu_force_rgba_textures: Option<Arc<Mutex<HashSet<String>>>>,
}

impl WowUiPrimitive {
    /// Create a primitive with a single merged batch placed in strata 0 (World).
    ///
    /// Used by the headless renderer and tests where per-strata separation
    /// isn't needed — all quads are already in draw order.
    pub fn new_merged(quads: Arc<QuadBatch>) -> Self {
        let mut strata_batches: [Option<Arc<QuadBatch>>; FrameStrata::COUNT] =
            std::array::from_fn(|_| None);
        strata_batches[0] = Some(quads);
        Self {
            strata_batches,
            overlay: QuadBatch::new(),
            clear_color: [0.10, 0.11, 0.14, 1.0],
            textures: Vec::new(),
            bc_textures: Vec::new(),
            glyph_atlas_data: None,
            glyph_atlas_size: 0,
            gpu_uploaded_textures: None,
            gpu_ready_textures: None,
            gpu_force_rgba_textures: None,
        }
    }

    /// Create a merged primitive with texture data (headless path).
    pub fn new_merged_with_textures(
        quads: Arc<QuadBatch>,
        textures: Vec<GpuTextureData>,
        bc_textures: Vec<GpuBcTextureData>,
    ) -> Self {
        let mut p = Self::new_merged(quads);
        p.textures = textures;
        p.bc_textures = bc_textures;
        p
    }

    /// Create an empty primitive.
    pub fn empty() -> Self {
        Self {
            strata_batches: std::array::from_fn(|_| None),
            overlay: QuadBatch::new(),
            clear_color: [0.10, 0.11, 0.14, 1.0],
            textures: Vec::new(),
            bc_textures: Vec::new(),
            glyph_atlas_data: None,
            glyph_atlas_size: 0,
            gpu_uploaded_textures: None,
            gpu_ready_textures: None,
            gpu_force_rgba_textures: None,
        }
    }
}

#[derive(Debug, Default)]
struct TextureUploadOutcome {
    ready_paths: HashSet<String>,
    retry_paths: HashSet<String>,
    force_rgba_retry_paths: HashSet<String>,
}

/// Upload pending textures and glyph atlas data to the GPU atlas.
fn upload_pending_textures(
    pipeline: &mut WowUiPipeline,
    queue: &wgpu::Queue,
    textures: &[GpuTextureData],
    bc_textures: &[GpuBcTextureData],
    glyph_atlas_data: &Option<Vec<u8>>,
    glyph_atlas_size: u32,
) -> TextureUploadOutcome {
    let atlas = pipeline.texture_atlas_mut();
    let mut outcome = TextureUploadOutcome::default();
    upload_rgba_textures(atlas, queue, textures, &mut outcome);
    upload_bc_textures(atlas, queue, bc_textures, &mut outcome);
    upload_glyph_atlas_if_present(atlas, queue, glyph_atlas_data, glyph_atlas_size);
    log_gpu_memory_once(atlas);
    outcome
}

fn record_texture_upload_outcome(
    outcome: TextureUploadOutcome,
    uploaded: Option<&Arc<Mutex<HashSet<String>>>>,
    ready: Option<&Arc<Mutex<HashSet<String>>>>,
    force_rgba: Option<&Arc<Mutex<HashSet<String>>>>,
) {
    if let Some(uploaded) = uploaded
        && let Ok(mut uploaded) = uploaded.lock()
    {
        for path in &outcome.retry_paths {
            uploaded.remove(path);
        }
    }
    let Some(ready) = ready else {
        if let Some(force_rgba) = force_rgba
            && let Ok(mut force_rgba) = force_rgba.lock()
        {
            force_rgba.extend(outcome.force_rgba_retry_paths);
        }
        return;
    };
    let Ok(mut ready) = ready.lock() else {
        if let Some(force_rgba) = force_rgba
            && let Ok(mut force_rgba) = force_rgba.lock()
        {
            force_rgba.extend(outcome.force_rgba_retry_paths);
        }
        return;
    };
    ready.extend(outcome.ready_paths);
    if let Some(force_rgba) = force_rgba
        && let Ok(mut force_rgba) = force_rgba.lock()
    {
        force_rgba.extend(outcome.force_rgba_retry_paths);
    }
}

fn upload_rgba_textures(
    atlas: &mut crate::render::shader::atlas::GpuTextureAtlas,
    queue: &wgpu::Queue,
    textures: &[GpuTextureData],
    outcome: &mut TextureUploadOutcome,
) {
    for tex_data in textures {
        if atlas.get(&tex_data.path).is_some() {
            outcome.ready_paths.insert(tex_data.path.clone());
            continue;
        }
        if atlas
            .upload(
                queue,
                &tex_data.path,
                tex_data.width,
                tex_data.height,
                tex_data.rgba.as_ref(),
            )
            .is_some()
        {
            outcome.ready_paths.insert(tex_data.path.clone());
        } else {
            outcome.retry_paths.insert(tex_data.path.clone());
        }
    }
}

fn upload_bc_textures(
    atlas: &mut crate::render::shader::atlas::GpuTextureAtlas,
    queue: &wgpu::Queue,
    bc_textures: &[GpuBcTextureData],
    outcome: &mut TextureUploadOutcome,
) {
    for bc_data in bc_textures {
        let already_uploaded =
            atlas.get_bc(&bc_data.path).is_some() || atlas.get(&bc_data.path).is_some();
        if already_uploaded {
            outcome.ready_paths.insert(bc_data.path.clone());
            continue;
        }
        if atlas
            .upload_bc(
                queue,
                &bc_data.path,
                bc_data.width,
                bc_data.height,
                bc_data.bc_data.as_ref(),
                bc_data.bc_format,
            )
            .is_some()
        {
            outcome.ready_paths.insert(bc_data.path.clone());
        } else {
            outcome.retry_paths.insert(bc_data.path.clone());
            outcome.force_rgba_retry_paths.insert(bc_data.path.clone());
        }
    }
}

fn upload_glyph_atlas_if_present(
    atlas: &mut crate::render::shader::atlas::GpuTextureAtlas,
    queue: &wgpu::Queue,
    glyph_atlas_data: &Option<Vec<u8>>,
    glyph_atlas_size: u32,
) {
    let Some(glyph_data) = glyph_atlas_data else {
        return;
    };
    atlas.upload_glyph_atlas(queue, glyph_data, glyph_atlas_size);
}

/// Log GPU atlas memory usage once after the first batch of textures.
fn log_gpu_memory_once(atlas: &crate::render::shader::atlas::GpuTextureAtlas) {
    use std::sync::atomic::{AtomicBool, Ordering};
    static LOGGED: AtomicBool = AtomicBool::new(false);
    if !crate::logging::texture_load_debug_enabled() {
        return;
    }
    if LOGGED.swap(true, Ordering::Relaxed) {
        return;
    }
    let stats = atlas.memory_stats();
    eprintln!(
        "{} [GPU] Atlas memory: {:.0} MB allocated, {:.1} MB used | slots: 64px={} 128px={} 256px={} 512px={} 2048px={}",
        crate::logging::global_elapsed_prefix(),
        stats.allocated_bytes as f64 / (1024.0 * 1024.0),
        stats.used_bytes as f64 / (1024.0 * 1024.0),
        stats.used_slots[0],
        stats.used_slots[1],
        stats.used_slots[2],
        stats.used_slots[3],
        stats.used_slots[4],
    );
}

/// Resolve pending texture indices (-2) and scale vertex positions to physical pixels.
fn resolve_and_scale_quads(
    pipeline: &mut WowUiPipeline,
    quads: &QuadBatch,
    scale: f32,
) -> QuadBatch {
    let mut resolved = quads.clone();
    let atlas = pipeline.texture_atlas_mut();
    resolve_texture_requests(atlas, &quads.texture_requests, &mut resolved.vertices);
    resolve_mask_requests(atlas, &quads.mask_texture_requests, &mut resolved.vertices);
    clear_pending_and_scale(&mut resolved.vertices, scale);
    resolved
}

/// Remap primary texture UVs for resolved atlas entries (RGBA or BC).
fn resolve_texture_requests(
    atlas: &crate::render::shader::atlas::GpuTextureAtlas,
    requests: &[crate::render::TextureRequest],
    vertices: &mut [crate::render::QuadVertex],
) {
    for request in requests {
        if let Some(entry) = resolved_texture_entry(atlas, &request.path) {
            apply_resolved_texture_entry(request_vertices(request, vertices), entry);
        }
    }
}

fn request_vertices<'a>(
    request: &crate::render::TextureRequest,
    vertices: &'a mut [crate::render::QuadVertex],
) -> &'a mut [crate::render::QuadVertex] {
    let start = request.vertex_start as usize;
    let end = start + request.vertex_count as usize;
    &mut vertices[start..end]
}

#[derive(Debug, Clone, Copy)]
enum ResolvedTextureEntry {
    Rgba(crate::render::shader::atlas::TextureEntry),
    Bc(crate::render::shader::atlas::BcTextureEntry),
}

fn resolved_texture_entry(
    atlas: &crate::render::shader::atlas::GpuTextureAtlas,
    path: &str,
) -> Option<ResolvedTextureEntry> {
    atlas
        .get(path)
        .copied()
        .map(ResolvedTextureEntry::Rgba)
        .or_else(|| atlas.get_bc(path).copied().map(ResolvedTextureEntry::Bc))
}

fn apply_resolved_texture_entry(
    vertices: &mut [crate::render::QuadVertex],
    entry: ResolvedTextureEntry,
) {
    match entry {
        ResolvedTextureEntry::Rgba(entry) => apply_rgba_entry(vertices, &entry),
        ResolvedTextureEntry::Bc(entry) => apply_bc_entry(vertices, &entry),
    }
}

fn apply_rgba_entry(
    vertices: &mut [crate::render::QuadVertex],
    entry: &crate::render::shader::atlas::TextureEntry,
) {
    let tex_idx = entry.tex_index();
    for vertex in vertices.iter_mut() {
        if vertex.tex_index == -2 {
            vertex.tex_index = tex_idx;
            vertex.tex_coords[0] = remap_entry_uv(
                vertex.tex_coords[0],
                entry.uv_x,
                entry.uv_width,
                entry.original_width,
                entry.tier,
            );
            vertex.tex_coords[1] = remap_entry_uv(
                vertex.tex_coords[1],
                entry.uv_y,
                entry.uv_height,
                entry.original_height,
                entry.tier,
            );
        }
    }
}

fn apply_bc_entry(
    vertices: &mut [crate::render::QuadVertex],
    bc_entry: &crate::render::shader::atlas::BcTextureEntry,
) {
    let tex_idx = bc_entry.tex_index();
    for vertex in vertices.iter_mut() {
        if vertex.tex_index == -2 {
            vertex.tex_index = tex_idx;
            vertex.tex_coords[0] = remap_bc_entry_uv(
                vertex.tex_coords[0],
                bc_entry.uv_x,
                bc_entry.uv_width,
                bc_entry.original_width,
            );
            vertex.tex_coords[1] = remap_bc_entry_uv(
                vertex.tex_coords[1],
                bc_entry.uv_y,
                bc_entry.uv_height,
                bc_entry.original_height,
            );
        }
    }
}

/// Remap mask texture UVs for resolved atlas entries.
fn resolve_mask_requests(
    atlas: &mut crate::render::shader::atlas::GpuTextureAtlas,
    requests: &[crate::render::TextureRequest],
    vertices: &mut [crate::render::QuadVertex],
) {
    for request in requests {
        if let Some(entry) = atlas.get(&request.path) {
            let start = request.vertex_start as usize;
            let end = start + request.vertex_count as usize;
            let tex_idx = entry.tex_index();
            for vertex in vertices[start..end].iter_mut() {
                if vertex.mask_tex_index == -2 {
                    vertex.mask_tex_index = tex_idx;
                    vertex.mask_tex_coords[0] = remap_entry_uv(
                        vertex.mask_tex_coords[0],
                        entry.uv_x,
                        entry.uv_width,
                        entry.original_width,
                        entry.tier,
                    );
                    vertex.mask_tex_coords[1] = remap_entry_uv(
                        vertex.mask_tex_coords[1],
                        entry.uv_y,
                        entry.uv_height,
                        entry.original_height,
                        entry.tier,
                    );
                }
            }
        }
    }
}

fn remap_entry_uv(local_uv: f32, base_uv: f32, span_uv: f32, original_size: u32, tier: u32) -> f32 {
    let cell_size = crate::render::shader::atlas::TIER_SIZES[tier as usize];
    let uploaded_size = original_size.min(cell_size).max(1) as f32;
    let inset = if uploaded_size > 1.0 {
        (span_uv * 0.5 / uploaded_size).min(span_uv * 0.5)
    } else {
        0.0
    };
    base_uv + inset + local_uv * (span_uv - inset * 2.0).max(0.0)
}

/// Remap UV for BC atlas entries (fixed 256x256 cell size).
fn remap_bc_entry_uv(local_uv: f32, base_uv: f32, span_uv: f32, original_size: u32) -> f32 {
    let cell_size = crate::render::shader::atlas::BC_CELL_SIZE;
    let uploaded_size = original_size.min(cell_size).max(1) as f32;
    let inset = if uploaded_size > 1.0 {
        (span_uv * 0.5 / uploaded_size).min(span_uv * 0.5)
    } else {
        0.0
    };
    base_uv + inset + local_uv * (span_uv - inset * 2.0).max(0.0)
}

/// Hide unresolved textures and scale positions to physical pixels.
fn clear_pending_and_scale(vertices: &mut [crate::render::QuadVertex], scale: f32) {
    for vertex in vertices.iter_mut() {
        if vertex.tex_index == -2 {
            vertex.color[3] = 0.0;
            vertex.tex_index = -1;
        }
        if vertex.mask_tex_index == -2 {
            vertex.mask_tex_index = -1;
        }
        vertex.position[0] *= scale;
        vertex.position[1] *= scale;
    }
}

impl shader::Primitive for WowUiPrimitive {
    type Pipeline = WowUiPipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: &Rectangle,
        viewport: &Viewport,
    ) {
        let scale = viewport.scale_factor();
        let physical_bounds = Rectangle::new(
            iced::Point::new(bounds.x * scale, bounds.y * scale),
            iced::Size::new(bounds.width * scale, bounds.height * scale),
        );

        let prepare_started = Instant::now();
        crate::logging::set_blocking_phase("prepare_textures");
        let textures_started = Instant::now();
        let ready_before = self
            .gpu_ready_textures
            .as_ref()
            .and_then(|ready| ready.lock().ok().map(|ready| ready.len()))
            .unwrap_or_default();
        let staged_before = self
            .gpu_uploaded_textures
            .as_ref()
            .and_then(|uploaded| uploaded.lock().ok().map(|uploaded| uploaded.len()))
            .unwrap_or_default();
        let upload_outcome = upload_pending_textures(
            pipeline,
            queue,
            &self.textures,
            &self.bc_textures,
            &self.glyph_atlas_data,
            self.glyph_atlas_size,
        );
        let retry_count = upload_outcome.retry_paths.len();
        let force_rgba_retry_count = upload_outcome.force_rgba_retry_paths.len();
        record_texture_upload_outcome(
            upload_outcome,
            self.gpu_uploaded_textures.as_ref(),
            self.gpu_ready_textures.as_ref(),
            self.gpu_force_rgba_textures.as_ref(),
        );
        if crate::logging::gui_trace_enabled() {
            let ready_after = self
                .gpu_ready_textures
                .as_ref()
                .and_then(|ready| ready.lock().ok().map(|ready| ready.len()))
                .unwrap_or_default();
            let staged_after = self
                .gpu_uploaded_textures
                .as_ref()
                .and_then(|uploaded| uploaded.lock().ok().map(|uploaded| uploaded.len()))
                .unwrap_or_default();
            crate::logging::eprintln_gui_trace(&format!(
                "prepare ready_before={ready_before} ready_after={ready_after} staged_before={staged_before} staged_after={staged_after} retry={retry_count} force_rgba_retry={force_rgba_retry_count} dirty_strata={} new_rgba={} new_bc={}",
                self.strata_batches
                    .iter()
                    .filter(|batch| batch.is_some())
                    .count(),
                self.textures.len(),
                self.bc_textures.len()
            ));
        }
        let textures_elapsed = textures_started.elapsed();
        crate::logging::set_blocking_phase("prepare_projection");
        pipeline.update_projection(queue, &physical_bounds);

        // Upload only dirty strata (Some = dirty, None = keep previous GPU buffer).
        crate::logging::set_blocking_phase("prepare_strata");
        let strata_started = Instant::now();
        for (i, batch_opt) in self.strata_batches.iter().enumerate() {
            if let Some(batch) = batch_opt {
                let resolved = resolve_and_scale_quads(pipeline, batch, scale);
                pipeline.upload_strata(device, queue, i, &resolved);
            }
        }
        let strata_elapsed = strata_started.elapsed();

        // Overlay slot (index = COUNT) — always re-uploaded since cursor moves every frame.
        let overlay_idx = FrameStrata::COUNT;
        crate::logging::set_blocking_phase("prepare_overlay");
        let overlay_started = Instant::now();
        if !self.overlay.vertices.is_empty() {
            let resolved = resolve_and_scale_quads(pipeline, &self.overlay, scale);
            pipeline.upload_strata(device, queue, overlay_idx, &resolved);
        } else {
            pipeline.clear_strata(overlay_idx);
        }
        let overlay_elapsed = overlay_started.elapsed();
        let prepare_elapsed = prepare_started.elapsed();
        if prepare_elapsed >= Duration::from_millis(50) {
            crate::logging::eprintln_elapsed(&format!(
                "[prepare] total={prepare_elapsed:.1?} textures={textures_elapsed:.1?} strata={strata_elapsed:.1?} overlay={overlay_elapsed:.1?} dirty_strata={} new_rgba={} new_bc={}",
                self.strata_batches
                    .iter()
                    .filter(|batch| batch.is_some())
                    .count(),
                self.textures.len(),
                self.bc_textures.len()
            ));
        }
    }

    fn render(
        &self,
        pipeline: &Self::Pipeline,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        pipeline.render(encoder, target, clip_bounds);
    }
}
