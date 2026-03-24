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
    pub rgba: Vec<u8>,
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
    if let Some((base_path, x, y, crop_w, crop_h)) = decode_crop_request(tex_mgr, path) {
        let tex_data = tex_mgr.load_sub_region(base_path, x, y, crop_w, crop_h)?;
        Some(GpuTextureData {
            path: path.to_string(),
            width: tex_data.width,
            height: tex_data.height,
            rgba: tex_data.pixels.clone(),
        })
    } else {
        let tex_data = tex_mgr.load(path)?;
        Some(GpuTextureData {
            path: path.to_string(),
            width: tex_data.width,
            height: tex_data.height,
            rgba: tex_data.pixels.clone(),
        })
    }
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
    use super::{WowUiPipeline, decode_crop_request, resolve_and_scale_quads};
    use crate::render::BlendMode;
    use crate::render::shader::QuadBatch;
    use iced::widget::shader::Pipeline;
    use iced::{Point, Rectangle, Size};

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
                pixels: vec![0; 200 * 100 * 4],
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
            glyph_atlas_data: None,
            glyph_atlas_size: 0,
        }
    }

    /// Create a merged primitive with texture data (headless path).
    pub fn new_merged_with_textures(quads: Arc<QuadBatch>, textures: Vec<GpuTextureData>) -> Self {
        let mut p = Self::new_merged(quads);
        p.textures = textures;
        p
    }

    /// Create an empty primitive.
    pub fn empty() -> Self {
        Self {
            strata_batches: std::array::from_fn(|_| None),
            overlay: QuadBatch::new(),
            clear_color: [0.10, 0.11, 0.14, 1.0],
            textures: Vec::new(),
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
                &tex_data.rgba,
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

/// Remap primary texture UVs for resolved atlas entries.
fn resolve_texture_requests(
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
                if vertex.tex_index == -2 {
                    vertex.tex_index = tex_idx;
                    vertex.tex_coords[0] = entry.uv_x + vertex.tex_coords[0] * entry.uv_width;
                    vertex.tex_coords[1] = entry.uv_y + vertex.tex_coords[1] * entry.uv_height;
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
                    vertex.mask_tex_coords[0] =
                        entry.uv_x + vertex.mask_tex_coords[0] * entry.uv_width;
                    vertex.mask_tex_coords[1] =
                        entry.uv_y + vertex.mask_tex_coords[1] * entry.uv_height;
                }
            }
        }
    }
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
