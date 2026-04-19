use super::{
    LoadedTexture, ResolvedTextureEntry, WowUiPipeline, apply_resolved_texture_entry,
    bc_texture_dimensions_fit_gpu_atlas, decode_crop_request, load_texture_prefer_bc,
    load_texture_prefer_bc_with_telemetry, remap_bc_entry_uv, remap_entry_uv,
    resolve_and_scale_quads,
};
use crate::render::BlendMode;
use crate::render::shader::QuadBatch;
use crate::render::shader::atlas::{BcFormat, BcTextureEntry};
use crate::render::shader::quad::QuadVertex;
use bytemuck::Zeroable;
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
fn load_texture_prefer_bc_cached_crop_request_skips_crop_decode_work() {
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
    let _ = load_texture_prefer_bc_with_telemetry(&mut mgr, crop_path)
        .0
        .expect("first crop request should populate the crop-request cache");

    let (loaded, telemetry) = load_texture_prefer_bc_with_telemetry(&mut mgr, crop_path);
    let LoadedTexture::Rgba(_) = loaded.expect("cached crop request should still load") else {
        panic!("crop requests should stay on the RGBA upload path");
    };

    assert_eq!(
        telemetry.crop_decode_elapsed,
        std::time::Duration::ZERO,
        "cached crop requests should bypass crop decoding entirely"
    );
    assert_eq!(
        telemetry.crop_extract_elapsed,
        std::time::Duration::ZERO,
        "cached crop requests should bypass sub-region extraction entirely"
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
fn bc_texture_dimensions_must_fit_bc_gpu_cell() {
    assert!(bc_texture_dimensions_fit_gpu_atlas(4, 4));
    assert!(bc_texture_dimensions_fit_gpu_atlas(
        crate::render::shader::atlas::BC_CELL_SIZE,
        crate::render::shader::atlas::BC_CELL_SIZE,
    ));
    assert!(!bc_texture_dimensions_fit_gpu_atlas(2, 4));
    assert!(!bc_texture_dimensions_fit_gpu_atlas(
        crate::render::shader::atlas::BC_CELL_SIZE + 4,
        4,
    ));
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
    assert!(
        resolved
            .vertices
            .iter()
            .all(|vertex| vertex.tex_index == -1)
    );
    assert!(
        resolved
            .vertices
            .iter()
            .all(|vertex| vertex.color[3] == 0.0)
    );
}

#[test]
fn resolved_textures_remap_quad_uvs_into_atlas_slot() {
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
    let path = r"Interface\Foo\Resolved";
    let entry = pipeline
        .texture_atlas_mut()
        .upload(&queue, path, 16, 16, &[0xff; 16 * 16 * 4])
        .expect("texture should upload into the atlas");

    let mut batch = QuadBatch::default();
    batch.push_textured_path(
        Rectangle::new(Point::ORIGIN, Size::new(16.0, 16.0)),
        path,
        [1.0, 1.0, 1.0, 1.0],
        BlendMode::Alpha,
    );

    let resolved = resolve_and_scale_quads(&mut pipeline, &batch, 1.0);
    for vertex in &resolved.vertices {
        let expected_u = remap_entry_uv(
            vertex.local_uv[0],
            entry.uv_x,
            entry.uv_width,
            entry.original_width,
            entry.tier,
        );
        let expected_v = remap_entry_uv(
            vertex.local_uv[1],
            entry.uv_y,
            entry.uv_height,
            entry.original_height,
            entry.tier,
        );
        assert_eq!(vertex.tex_index, entry.tex_index());
        assert!((vertex.tex_coords[0] - expected_u).abs() < 1e-6);
        assert!((vertex.tex_coords[1] - expected_v).abs() < 1e-6);
    }
}

#[test]
fn resolved_bc_entries_remap_quad_uvs_into_bc_slot() {
    let bc_entry = BcTextureEntry {
        format: BcFormat::Bc3,
        grid_x: 0,
        grid_y: 0,
        original_width: 128,
        original_height: 64,
        uv_x: 0.25,
        uv_y: 0.5,
        uv_width: 0.125,
        uv_height: 0.25,
    };
    let mut vertices = [
        QuadVertex {
            tex_coords: [0.0, 0.0],
            local_uv: [0.0, 0.0],
            tex_index: -2,
            ..QuadVertex::zeroed()
        },
        QuadVertex {
            tex_coords: [1.0, 1.0],
            local_uv: [1.0, 1.0],
            tex_index: -2,
            ..QuadVertex::zeroed()
        },
    ];

    apply_resolved_texture_entry(&mut vertices, ResolvedTextureEntry::Bc(bc_entry));

    assert_eq!(vertices[0].tex_index, bc_entry.tex_index());
    assert_eq!(vertices[1].tex_index, bc_entry.tex_index());
    assert!((vertices[0].tex_coords[0] - remap_bc_entry_uv(0.0, 0.25, 0.125, 128)).abs() < 1e-6);
    assert!((vertices[0].tex_coords[1] - remap_bc_entry_uv(0.0, 0.5, 0.25, 64)).abs() < 1e-6);
    assert!((vertices[1].tex_coords[0] - remap_bc_entry_uv(1.0, 0.25, 0.125, 128)).abs() < 1e-6);
    assert!((vertices[1].tex_coords[1] - remap_bc_entry_uv(1.0, 0.5, 0.25, 64)).abs() < 1e-6);
}
