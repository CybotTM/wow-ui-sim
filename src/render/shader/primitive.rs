//! WoW UI shader primitive implementation.

use super::{QuadBatch, WowUiPipeline};
use iced::Rectangle;
use iced::widget::shader::{self, Viewport};
use std::sync::Arc;

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
    // Crop requests always go through RGBA (sub-region extraction needs decoded pixels)
    if path.contains("@crop:") {
        if let Some((base_path, x, y, crop_w, crop_h)) = decode_crop_request(tex_mgr, path) {
            let tex_data = tex_mgr.load_sub_region(base_path, x, y, crop_w, crop_h)?;
            return Some(LoadedTexture::Rgba(GpuTextureData {
                path: path.to_string(),
                width: tex_data.width,
                height: tex_data.height,
                rgba: Arc::clone(&tex_data.pixels),
            }));
        }
        return None;
    }

    // Try BC path first (only succeeds for BLP files with DXT content AND GPU BC support)
    if crate::render::shader::atlas::is_bc_supported() {
        if let Some(bc_result) = tex_mgr.load_bc(path) {
            // BC textures require dimensions that are multiples of 4
            if bc_result.width % 4 == 0
                && bc_result.height % 4 == 0
                && bc_result.width <= crate::render::shader::atlas::BC_CELL_SIZE
                && bc_result.height <= crate::render::shader::atlas::BC_CELL_SIZE
            {
                return Some(LoadedTexture::Bc(GpuBcTextureData {
                    path: path.to_string(),
                    width: bc_result.width,
                    height: bc_result.height,
                    bc_data: Arc::clone(&bc_result.bc_data),
                    bc_format: bc_result.format,
                }));
            }
        }
    }

    // Fall back to RGBA
    let tex_data = tex_mgr.load(path)?;
    Some(LoadedTexture::Rgba(GpuTextureData {
        path: path.to_string(),
        width: tex_data.width,
        height: tex_data.height,
        rgba: Arc::clone(&tex_data.pixels),
    }))
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
mod tests {
    use super::{
        LoadedTexture, WowUiPipeline, decode_crop_request, load_texture_prefer_bc, remap_entry_uv,
        resolve_and_scale_quads,
    };
    use crate::render::BlendMode;
    use crate::render::shader::QuadBatch;
    use iced::widget::shader::Pipeline;
    use iced::{Point, Rectangle, Size};
    use std::sync::Arc;

    #[test]
    fn decode_crop_request_rejects_malformed_coords() {
        let mut mgr = crate::texture::TextureManager::new(".");
        assert!(decode_crop_request(&mut mgr, "foo@crop:0.1,0.2,0.3").is_none());
    }

    #[test]
    fn decode_crop_request_uses_cached_texture_dimensions() {
        let mut mgr = crate::texture::TextureManager::new(".");
        mgr.insert_test_texture(
            r"Interface\Foo\Bar",
            crate::texture::TextureData {
                width: 200,
                height: 100,
                pixels: Arc::<[u8]>::from(vec![0; 200 * 100 * 4]),
            },
        );
        let decoded = decode_crop_request(
            &mut mgr,
            r"Interface\Foo\Bar@crop:0.100000,0.600000,0.200000,0.700000",
        )
        .expect("crop request should decode");
        assert_eq!(decoded.0, r"Interface\Foo\Bar");
        assert_eq!(decoded.1, 20);
        assert_eq!(decoded.2, 20);
        assert_eq!(decoded.3, 100);
        assert_eq!(decoded.4, 50);
    }

    #[test]
    fn load_texture_prefer_bc_reuses_cached_rgba_buffer() {
        let mut mgr = crate::texture::TextureManager::new(".");
        let cached_pixels = Arc::<[u8]>::from(vec![0xaa; 4 * 4 * 4]);
        mgr.insert_test_texture(
            r"Interface\Foo\Cached",
            crate::texture::TextureData {
                width: 4,
                height: 4,
                pixels: Arc::clone(&cached_pixels),
            },
        );

        let loaded = load_texture_prefer_bc(&mut mgr, r"Interface\Foo\Cached")
            .expect("cached RGBA texture should load");
        let LoadedTexture::Rgba(upload) = loaded else {
            panic!("plain cached texture should stay on the RGBA upload path");
        };

        assert_eq!(
            upload.rgba.as_ptr(),
            cached_pixels.as_ptr(),
            "RGBA upload path should reuse cached pixels instead of cloning them"
        );
    }

    #[test]
    fn load_texture_prefer_bc_reuses_cached_crop_buffer() {
        let mut mgr = crate::texture::TextureManager::new(".");
        mgr.insert_test_texture(
            r"Interface\Foo\CropSource",
            crate::texture::TextureData {
                width: 8,
                height: 8,
                pixels: Arc::<[u8]>::from(vec![0xbb; 8 * 8 * 4]),
            },
        );

        let crop_path = r"Interface\Foo\CropSource@crop:0.250000,0.750000,0.250000,0.750000";
        let _ = mgr
            .load_sub_region(r"Interface\Foo\CropSource", 2, 2, 4, 4)
            .expect("crop should populate the sub-region cache");
        let cached_crop_ptr = mgr
            .load_sub_region(r"Interface\Foo\CropSource", 2, 2, 4, 4)
            .expect("crop should stay cached");
        let cached_crop_ptr = cached_crop_ptr.pixels.as_ptr();

        let loaded =
            load_texture_prefer_bc(&mut mgr, crop_path).expect("cached crop texture should load");
        let LoadedTexture::Rgba(upload) = loaded else {
            panic!("crop requests should stay on the RGBA upload path");
        };

        assert_eq!(
            upload.rgba.as_ptr(),
            cached_crop_ptr,
            "crop upload path should reuse cached crop pixels instead of cloning them"
        );
    }

    #[test]
    fn remap_entry_uv_insets_slot_edges_by_half_texel() {
        let left = remap_entry_uv(0.0, 0.25, 32.0 / 4096.0, 32, 0);
        let right = remap_entry_uv(1.0, 0.25, 32.0 / 4096.0, 32, 0);

        assert!((left - (0.25 + 0.5 / 4096.0)).abs() < 1e-6);
        assert!((right - (0.25 + 31.5 / 4096.0)).abs() < 1e-6);
    }

    #[test]
    fn unresolved_pending_textures_become_transparent() {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let (device, queue) = pollster::block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions::default())
                .await
                .expect("adapter");
            adapter
                .request_device(&wgpu::DeviceDescriptor::default())
                .await
                .expect("device")
        });

        let mut pipeline = WowUiPipeline::new(&device, &queue, wgpu::TextureFormat::Rgba8UnormSrgb);
        let mut batch = QuadBatch::default();
        batch.push_textured_path(
            Rectangle::new(Point::ORIGIN, Size::new(16.0, 16.0)),
            r"Interface\Missing\Pending",
            [1.0, 1.0, 1.0, 1.0],
            BlendMode::Alpha,
        );

        let resolved = resolve_and_scale_quads(&mut pipeline, &batch, 1.0);
        assert!(resolved.vertices.iter().all(|v| v.tex_index == -1));
        assert!(resolved.vertices.iter().all(|v| v.color[3] == 0.0));
    }
}

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
        }
    }
}

/// Upload pending textures and glyph atlas data to the GPU atlas.
fn upload_pending_textures(
    pipeline: &mut WowUiPipeline,
    queue: &wgpu::Queue,
    textures: &[GpuTextureData],
    bc_textures: &[GpuBcTextureData],
    glyph_atlas_data: &Option<Vec<u8>>,
    glyph_atlas_size: u32,
) {
    let atlas = pipeline.texture_atlas_mut();
    for tex_data in textures {
        if atlas.get(&tex_data.path).is_none() {
            atlas.upload(
                queue,
                &tex_data.path,
                tex_data.width,
                tex_data.height,
                tex_data.rgba.as_ref(),
            );
        }
    }

    for bc_data in bc_textures {
        if atlas.get_bc(&bc_data.path).is_none() && atlas.get(&bc_data.path).is_none() {
            atlas.upload_bc(
                queue,
                &bc_data.path,
                bc_data.width,
                bc_data.height,
                bc_data.bc_data.as_ref(),
                bc_data.bc_format,
            );
        }
    }

    if let Some(glyph_data) = glyph_atlas_data {
        atlas.upload_glyph_atlas(queue, glyph_data, glyph_atlas_size);
    }

    log_gpu_memory_once(atlas);
}

/// Log GPU atlas memory usage once after the first batch of textures.
fn log_gpu_memory_once(atlas: &crate::render::shader::atlas::GpuTextureAtlas) {
    use std::sync::atomic::{AtomicBool, Ordering};
    static LOGGED: AtomicBool = AtomicBool::new(false);
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
    atlas: &mut crate::render::shader::atlas::GpuTextureAtlas,
    requests: &[crate::render::TextureRequest],
    vertices: &mut [crate::render::QuadVertex],
) {
    for request in requests {
        // Try RGBA atlas first, then BC atlas
        if let Some(entry) = atlas.get(&request.path) {
            let start = request.vertex_start as usize;
            let end = start + request.vertex_count as usize;
            let tex_idx = entry.tex_index();
            for vertex in vertices[start..end].iter_mut() {
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
        } else if let Some(bc_entry) = atlas.get_bc(&request.path).copied() {
            let start = request.vertex_start as usize;
            let end = start + request.vertex_count as usize;
            let tex_idx = bc_entry.tex_index();
            for vertex in vertices[start..end].iter_mut() {
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

        upload_pending_textures(
            pipeline,
            queue,
            &self.textures,
            &self.bc_textures,
            &self.glyph_atlas_data,
            self.glyph_atlas_size,
        );
        pipeline.update_projection(queue, &physical_bounds);

        // Upload only dirty strata (Some = dirty, None = keep previous GPU buffer).
        for (i, batch_opt) in self.strata_batches.iter().enumerate() {
            if let Some(batch) = batch_opt {
                let resolved = resolve_and_scale_quads(pipeline, batch, scale);
                pipeline.upload_strata(device, queue, i, &resolved);
            }
        }

        // Overlay slot (index = COUNT) — always re-uploaded since cursor moves every frame.
        let overlay_idx = FrameStrata::COUNT;
        if !self.overlay.vertices.is_empty() {
            let resolved = resolve_and_scale_quads(pipeline, &self.overlay, scale);
            pipeline.upload_strata(device, queue, overlay_idx, &resolved);
        } else {
            pipeline.clear_strata(overlay_idx);
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
