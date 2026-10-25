use super::{
    LoadedTexture, decode_crop_request, load_texture_prefer_bc,
    load_texture_prefer_bc_with_telemetry,
};
use std::sync::Arc;

#[test]
fn decode_crop_request_rejects_malformed_coords() {
    let mut mgr = crate::texture::TextureManager::new();
    assert!(decode_crop_request(&mut mgr, "foo@crop:0.1,0.2,0.3").is_none());
}

#[test]
fn decode_crop_request_uses_cached_texture_dimensions() {
    let mut mgr = crate::texture::TextureManager::new();
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
    let mut mgr = crate::texture::TextureManager::new();
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
    let mut mgr = crate::texture::TextureManager::new();
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
    let mut mgr = crate::texture::TextureManager::new();
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
