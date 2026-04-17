//! BC (block-compressed) texture atlas for DXT1/DXT5 textures.
//!
//! Extracted from atlas.rs. Provides `BcFormat`, `BcTextureEntry`, and
//! `BcAtlasTier` plus helper functions for GPU upload and slot management.

use super::atlas::{BC1_ATLAS_TEX_INDEX, BC3_ATLAS_TEX_INDEX};

/// Cell size for BC atlas slots (256x256, matching map tile size).
pub const BC_CELL_SIZE: u32 = 256;

/// Size of the BC atlas texture (4096x4096).
const BC_ATLAS_SIZE: u32 = 4096;

/// Whether the GPU supports BC texture compression (set once at atlas init).
static BC_SUPPORTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Check if the GPU supports BC texture compression.
/// Only valid after `GpuTextureAtlas::new()` has been called.
pub fn is_bc_supported() -> bool {
    BC_SUPPORTED.load(std::sync::atomic::Ordering::Relaxed)
}

/// Override BC support flag for tests.
#[cfg(test)]
pub(crate) fn set_bc_supported_for_tests(supported: bool) -> bool {
    BC_SUPPORTED.swap(supported, std::sync::atomic::Ordering::Relaxed)
}

/// BC compression format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BcFormat {
    /// DXT1 — no alpha channel.
    Bc1,
    /// DXT3/DXT5 — with alpha channel.
    Bc3,
}

impl BcFormat {
    pub(super) fn bytes_per_block(self) -> u32 {
        match self {
            BcFormat::Bc1 => 8,
            BcFormat::Bc3 => 16,
        }
    }

    pub(super) fn wgpu_format(self) -> wgpu::TextureFormat {
        match self {
            BcFormat::Bc1 => wgpu::TextureFormat::Bc1RgbaUnormSrgb,
            BcFormat::Bc3 => wgpu::TextureFormat::Bc3RgbaUnormSrgb,
        }
    }

    fn tex_index(self) -> i32 {
        match self {
            BcFormat::Bc1 => BC1_ATLAS_TEX_INDEX,
            BcFormat::Bc3 => BC3_ATLAS_TEX_INDEX,
        }
    }
}

impl From<crate::texture::BcTextureFormat> for BcFormat {
    fn from(value: crate::texture::BcTextureFormat) -> Self {
        match value {
            crate::texture::BcTextureFormat::Bc1 => Self::Bc1,
            crate::texture::BcTextureFormat::Bc3 => Self::Bc3,
        }
    }
}

/// Entry for a BC-compressed texture in the BC atlas.
#[derive(Debug, Clone, Copy)]
pub struct BcTextureEntry {
    /// Which atlas (BC1 or BC3).
    pub format: BcFormat,
    /// Grid position X within the atlas.
    pub grid_x: u32,
    /// Grid position Y within the atlas.
    pub grid_y: u32,
    /// Original texture dimensions.
    pub original_width: u32,
    pub original_height: u32,
    /// UV rectangle within the atlas.
    pub uv_x: f32,
    pub uv_y: f32,
    pub uv_width: f32,
    pub uv_height: f32,
}

impl BcTextureEntry {
    /// Get the shader tex_index for this entry.
    pub fn tex_index(&self) -> i32 {
        self.format.tex_index()
    }

    /// Get UV rectangle for the shader.
    pub fn uv_rect(&self) -> iced::Rectangle {
        iced::Rectangle::new(
            iced::Point::new(self.uv_x, self.uv_y),
            iced::Size::new(self.uv_width, self.uv_height),
        )
    }
}

/// A single BC-format atlas (BC1 or BC3).
pub(super) struct BcAtlasTier {
    pub(super) texture: wgpu::Texture,
    pub(super) view: wgpu::TextureView,
    grid_size: u32,
    next_slot: u32,
}

impl BcAtlasTier {
    /// Create a real BC-compressed atlas (requires `TEXTURE_COMPRESSION_BC` feature).
    fn new(device: &wgpu::Device, format: BcFormat) -> Self {
        let grid_size = BC_ATLAS_SIZE / BC_CELL_SIZE;
        let label = match format {
            BcFormat::Bc1 => "WoW UI BC1 Atlas",
            BcFormat::Bc3 => "WoW UI BC3 Atlas",
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: BC_ATLAS_SIZE,
                height: BC_ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: format.wgpu_format(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("{} View", label)),
            dimension: Some(wgpu::TextureViewDimension::D2),
            ..Default::default()
        });
        Self {
            texture,
            view,
            grid_size,
            next_slot: 0,
        }
    }

    /// Create a 1x1 RGBA8 placeholder (used when BC compression is unavailable).
    fn new_placeholder(device: &wgpu::Device, label: &str) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("{} View", label)),
            dimension: Some(wgpu::TextureViewDimension::D2),
            ..Default::default()
        });
        Self {
            texture,
            view,
            grid_size: 0,
            next_slot: 0,
        }
    }

    pub(super) fn reset(&mut self) {
        self.next_slot = 0;
    }

    pub(super) fn is_full(&self) -> bool {
        self.next_slot >= self.grid_size * self.grid_size
    }

    pub(super) fn allocate_slot(&mut self) -> Option<(u32, u32)> {
        if self.is_full() {
            return None;
        }
        let slot = self.next_slot;
        self.next_slot += 1;
        Some((slot % self.grid_size, slot / self.grid_size))
    }

    fn pixel_offset(&self, grid_x: u32, grid_y: u32) -> (u32, u32) {
        (grid_x * BC_CELL_SIZE, grid_y * BC_CELL_SIZE)
    }

    pub(super) fn uv_offset(&self, grid_x: u32, grid_y: u32) -> (f32, f32) {
        let cell_uv = BC_CELL_SIZE as f32 / BC_ATLAS_SIZE as f32;
        (grid_x as f32 * cell_uv, grid_y as f32 * cell_uv)
    }
}

/// Detect BC texture compression support and create BC1/BC3 atlas tiers.
pub(super) fn init_bc_atlases(device: &wgpu::Device) -> (BcAtlasTier, BcAtlasTier, bool) {
    let has_bc_support = device
        .features()
        .contains(wgpu::Features::TEXTURE_COMPRESSION_BC);
    BC_SUPPORTED.store(has_bc_support, std::sync::atomic::Ordering::Relaxed);
    eprintln!(
        "{} [GPU] BC texture compression: {}",
        crate::logging::global_elapsed_prefix(),
        if has_bc_support {
            "supported"
        } else {
            "NOT supported (using placeholders)"
        }
    );
    let (bc1, bc3) = if has_bc_support {
        (
            BcAtlasTier::new(device, BcFormat::Bc1),
            BcAtlasTier::new(device, BcFormat::Bc3),
        )
    } else {
        (
            BcAtlasTier::new_placeholder(device, "WoW UI BC1 Atlas (placeholder)"),
            BcAtlasTier::new_placeholder(device, "WoW UI BC3 Atlas (placeholder)"),
        )
    };
    (bc1, bc3, has_bc_support)
}

/// Write BC-compressed data into an allocated atlas slot.
pub(super) fn write_bc_slot(
    queue: &wgpu::Queue,
    atlas: &BcAtlasTier,
    grid_x: u32,
    grid_y: u32,
    width: u32,
    height: u32,
    bc_data: &[u8],
    format: BcFormat,
) {
    let (pixel_x, pixel_y) = atlas.pixel_offset(grid_x, grid_y);
    let upload_width = width.max(4);
    let upload_height = height.max(4);
    let bytes_per_row = (upload_width / 4) * format.bytes_per_block();

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &atlas.texture,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: pixel_x,
                y: pixel_y,
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        },
        bc_data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(bytes_per_row),
            rows_per_image: Some(upload_height / 4),
        },
        wgpu::Extent3d {
            width: upload_width,
            height: upload_height,
            depth_or_array_layers: 1,
        },
    );
}

/// Build a BcTextureEntry from an allocated slot's grid coordinates.
pub(super) fn build_bc_entry(
    atlas: &BcAtlasTier,
    format: BcFormat,
    grid_x: u32,
    grid_y: u32,
    width: u32,
    height: u32,
) -> BcTextureEntry {
    let (uv_base_x, uv_base_y) = atlas.uv_offset(grid_x, grid_y);
    let cell_uv = BC_CELL_SIZE as f32 / BC_ATLAS_SIZE as f32;
    BcTextureEntry {
        format,
        grid_x,
        grid_y,
        original_width: width,
        original_height: height,
        uv_x: uv_base_x,
        uv_y: uv_base_y,
        uv_width: (width as f32 / BC_ATLAS_SIZE as f32).min(cell_uv),
        uv_height: (height as f32 / BC_ATLAS_SIZE as f32).min(cell_uv),
    }
}
