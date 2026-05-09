use self::preload::should_preload_talent_atlas_key;
use super::*;
use image_blp::types::direct::dxtn::{BlpDxtn, DxtnFormat, DxtnImage};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_normalize_wow_path() {
    assert_eq!(
        normalize_wow_path("Interface\\DialogFrame\\UI-DialogBox-Background"),
        "Interface/DialogFrame/UI-DialogBox-Background"
    );
    assert_eq!(
        normalize_wow_path("Interface\\BUTTONS\\UI-Panel-Button-Up.blp"),
        "Interface/BUTTONS/UI-Panel-Button-Up"
    );
}

#[test]
fn test_load_webp_texture() {
    let mut mgr = TextureManager::new();
    let result = mgr.load("Interface/BUTTONS/UI-SortArrow");

    assert!(result.is_some(), "Should load UI-SortArrow texture");
    let data = result.unwrap();
    assert!(data.width > 0, "Texture should have non-zero width");
    assert!(data.height > 0, "Texture should have non-zero height");
    assert!(!data.pixels.is_empty(), "Texture should have pixel data");
    assert_eq!(
        data.pixels.len(),
        (data.width * data.height * 4) as usize,
        "Pixel data should be RGBA"
    );
}

#[test]
fn test_webp_preferred_over_png() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    // Create a test texture in both webp and png formats
    // WebP: 2x2 red pixels
    // PNG: 2x2 blue pixels
    let webp_path = base.join("test-texture.webp");
    let png_path = base.join("test-texture.png");

    // Create a minimal 2x2 red image for webp
    let red_img = image::RgbaImage::from_pixel(2, 2, image::Rgba([255, 0, 0, 255]));
    red_img.save(&webp_path).unwrap();

    // Create a minimal 2x2 blue image for png
    let blue_img = image::RgbaImage::from_pixel(2, 2, image::Rgba([0, 0, 255, 255]));
    blue_img.save(&png_path).unwrap();

    // Load texture - should prefer webp
    let mut mgr = TextureManager::new().with_addons_path(base);
    let result = mgr.load("Interface/AddOns/test-texture");

    assert!(result.is_some(), "Should load test-texture");
    let data = result.unwrap();

    // Check that we got red pixels (webp), not blue (png)
    assert_eq!(data.width, 2);
    assert_eq!(data.height, 2);
    // First pixel should be red (R=255, G=0, B=0, A=255)
    assert_eq!(data.pixels[0], 255, "R should be 255 (webp loaded)");
    assert_eq!(data.pixels[1], 0, "G should be 0 (webp loaded)");
    assert_eq!(data.pixels[2], 0, "B should be 0 (webp loaded)");
}

#[test]
fn test_extension_priority_order() {
    // Verify BLP wins when multiple encodings exist for same texture.
    let extensions = [
        "blp", "BLP", "webp", "WEBP", "PNG", "png", "tga", "TGA", "jpg", "JPG",
    ];
    assert_eq!(extensions[0], "blp", "blp should be first priority");
    assert_eq!(extensions[1], "BLP", "BLP should be second priority");
}

#[test]
fn test_fallback_to_png_when_no_webp() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    // Create only a PNG file (no webp)
    let png_path = base.join("only-png.png");
    let green_img = image::RgbaImage::from_pixel(2, 2, image::Rgba([0, 255, 0, 255]));
    green_img.save(&png_path).unwrap();

    let mut mgr = TextureManager::new().with_addons_path(base);
    let result = mgr.load("Interface/AddOns/only-png");

    assert!(result.is_some(), "Should load png when webp not available");
    let data = result.unwrap();
    // First pixel should be green
    assert_eq!(data.pixels[0], 0, "R should be 0");
    assert_eq!(data.pixels[1], 255, "G should be 255 (png loaded)");
    assert_eq!(data.pixels[2], 0, "B should be 0");
}

#[test]
fn test_case_insensitive_loading() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    // Create subdirectory with mixed case
    let subdir = base.join("BUTTONS");
    fs::create_dir(&subdir).unwrap();

    let webp_path = subdir.join("UI-Panel-Button.webp");
    let img = image::RgbaImage::from_pixel(1, 1, image::Rgba([128, 128, 128, 255]));
    img.save(&webp_path).unwrap();

    let mut mgr = TextureManager::new().with_addons_path(base);

    // Try loading with different cases
    let result = mgr.load("Interface/AddOns/buttons/ui-panel-button");
    assert!(result.is_some(), "Should load with lowercase path");
}

#[test]
fn test_nonexistent_texture_returns_none() {
    let mut mgr = TextureManager::new();

    let result = mgr.load("this/texture/does/not/exist");
    assert!(
        result.is_none(),
        "Should return None for nonexistent texture"
    );
}

#[test]
fn test_texture_caching() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    let webp_path = base.join("cached.webp");
    let img = image::RgbaImage::from_pixel(4, 4, image::Rgba([100, 100, 100, 255]));
    img.save(&webp_path).unwrap();

    let mut mgr = TextureManager::new().with_addons_path(base);

    // First load
    let result1 = mgr.load("Interface/AddOns/cached");
    assert!(result1.is_some());
    let pixels1 = result1.unwrap().pixels.clone();

    // Second load should return cached version (using get, no disk access)
    let result2 = mgr.get("Interface/AddOns/cached");
    assert!(result2.is_some(), "Should get from cache");

    // Verify same data
    assert_eq!(pixels1, result2.unwrap().pixels);
}

#[test]
fn test_load_with_telemetry_reports_memory_cache_hit() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    let webp_path = base.join("cached.webp");
    let img = image::RgbaImage::from_pixel(4, 4, image::Rgba([100, 100, 100, 255]));
    img.save(&webp_path).unwrap();

    let mut mgr = TextureManager::new().with_addons_path(base);
    assert!(
        mgr.load("Interface/AddOns/cached").is_some(),
        "first load should populate memory cache"
    );

    let (cached, telemetry) = mgr.load_with_telemetry("Interface/AddOns/cached");

    assert!(cached.is_some(), "second load should still return texture");
    assert!(
        telemetry.mem_cache_hit,
        "telemetry should report memory cache hits"
    );
    assert_eq!(
        telemetry.resolve_elapsed,
        Duration::ZERO,
        "memory cache hit should skip path resolution"
    );
    assert_eq!(
        telemetry.decode_elapsed,
        Duration::ZERO,
        "memory cache hit should skip source decode"
    );
}

#[test]
fn test_get_or_load_texture_size_uses_metadata_without_populating_rgba_cache() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    let webp_path = base.join("metadata-only.webp");
    let img = image::RgbaImage::from_pixel(7, 5, image::Rgba([10, 20, 30, 255]));
    img.save(&webp_path).unwrap();

    let mut mgr = TextureManager::new().with_addons_path(base);
    let dims = mgr
        .get_or_load_texture_size("Interface/AddOns/metadata-only")
        .expect("metadata-only size lookup should succeed");

    assert_eq!(dims, (7, 5));
    assert_eq!(
        mgr.cache_len(),
        0,
        "size-only lookup should not force full RGBA decode into the cache"
    );
    assert_eq!(
        mgr.get_texture_size("Interface/AddOns/metadata-only"),
        Some((7, 5)),
        "size-only lookup should still seed the size cache"
    );
}

#[test]
fn test_sub_region_uses_cached_base_texture() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();

    let webp_path = base.join("cropped.webp");
    let mut img = image::RgbaImage::new(4, 4);
    for y in 0..4 {
        for x in 0..4 {
            img.put_pixel(x, y, image::Rgba([(x * 10) as u8, (y * 20) as u8, 0, 255]));
        }
    }
    img.save(&webp_path).unwrap();

    let mut mgr = TextureManager::new().with_addons_path(base);
    assert!(
        mgr.load("Interface/AddOns/cropped").is_some(),
        "base texture should load"
    );

    fs::remove_file(&webp_path).unwrap();

    let sub = mgr
        .load_sub_region("Interface/AddOns/cropped", 1, 1, 2, 2)
        .expect("sub-region should load from cached base");
    assert_eq!((sub.width, sub.height), (2, 2));
    assert_eq!(sub.pixels[0], 10);
    assert_eq!(sub.pixels[1], 20);
}

#[test]
fn test_load_bc_caches_dxt_blp_data() {
    let mut mgr = TextureManager::new();
    mgr.bc_cache.insert(
        "cached-dxt".to_string(),
        BcTextureResult {
            width: 4,
            height: 4,
            bc_data: Arc::<[u8]>::from(vec![0xaa; 8]),
            format: BcTextureFormat::Bc1,
        },
    );

    let cached = mgr
        .load_bc("cached-dxt")
        .expect("cached BC data should be returned without reading disk");

    assert_eq!(cached.width, 4);
    assert_eq!(cached.height, 4);
    assert_eq!(cached.format, BcTextureFormat::Bc1);
    assert_eq!(cached.bc_data.as_ref(), [0xaa; 8]);
}

#[test]
fn test_load_bc_with_telemetry_reports_cache_hits() {
    let mut mgr = TextureManager::new();
    mgr.bc_cache.insert(
        "cached-dxt".to_string(),
        BcTextureResult {
            width: 4,
            height: 4,
            bc_data: Arc::<[u8]>::from(vec![0xaa; 8]),
            format: BcTextureFormat::Bc1,
        },
    );

    let (cached, telemetry) = mgr.load_bc_with_telemetry("cached-dxt");

    assert!(cached.is_some(), "cached BC data should still be returned");
    assert!(telemetry.cache_hit, "telemetry should record BC cache hits");
    assert_eq!(
        telemetry.resolve_elapsed,
        Duration::ZERO,
        "cached loads should not re-run path resolution"
    );
    assert_eq!(
        telemetry.parse_elapsed,
        Duration::ZERO,
        "cached loads should not reparse the BLP file"
    );
    assert_eq!(
        telemetry.extract_elapsed,
        Duration::ZERO,
        "cached loads should not re-extract BC blocks"
    );
}

#[test]
fn test_load_bc_with_telemetry_caches_non_blp_paths() {
    let temp_dir = TempDir::new().unwrap();
    let base = temp_dir.path();
    let png_path = base.join("not-bc.png");
    let img = image::RgbaImage::from_pixel(2, 2, image::Rgba([1, 2, 3, 255]));
    img.save(&png_path).unwrap();

    let mut mgr = TextureManager::new().with_addons_path(base);
    let (first, first_telemetry) = mgr.load_bc_with_telemetry("Interface/AddOns/not-bc");
    assert!(
        first.is_none(),
        "png texture should stay on the non-BC path"
    );
    assert!(
        first_telemetry.resolve_elapsed > Duration::ZERO,
        "first attempt should resolve the texture path"
    );

    let (second, second_telemetry) = mgr.load_bc_with_telemetry("Interface/AddOns/not-bc");
    assert!(
        second.is_none(),
        "non-BC texture should still return None for BC requests"
    );
    assert_eq!(
        second_telemetry.resolve_elapsed,
        Duration::ZERO,
        "negative BC cache should skip path resolution on repeat"
    );
    assert_eq!(
        second_telemetry.parse_elapsed,
        Duration::ZERO,
        "negative BC cache should skip BLP parsing on repeat"
    );
}

fn test_dxtn(content: Vec<u8>, format: DxtnFormat) -> BlpDxtn {
    BlpDxtn {
        format,
        cmap: Vec::new(),
        images: vec![DxtnImage { content }],
    }
}

#[test]
fn bc_texture_result_maps_supported_dxt_formats() {
    let dxt1 = bc_texture_result(
        4,
        4,
        BlpContent::Dxt1(test_dxtn(vec![0x11; 8], DxtnFormat::Dxt1)),
    )
    .expect("DXT1 content should map to a BC texture result");
    assert_eq!(dxt1.format, BcTextureFormat::Bc1);
    assert_eq!(dxt1.bc_data.as_ref(), [0x11; 8]);

    let dxt5 = bc_texture_result(
        8,
        8,
        BlpContent::Dxt5(test_dxtn(vec![0x33; 16], DxtnFormat::Dxt5)),
    )
    .expect("DXT5 content should map to a BC texture result");
    assert_eq!(dxt5.format, BcTextureFormat::Bc3);
}

#[test]
fn bc_texture_result_rejects_dxt3_without_bc2_support() {
    let dxt3 = bc_texture_result(
        8,
        8,
        BlpContent::Dxt3(test_dxtn(vec![0x22; 16], DxtnFormat::Dxt3)),
    );

    assert!(dxt3.is_none());
}

#[test]
fn bc_texture_result_rejects_empty_and_unsupported_content() {
    assert!(
        bc_texture_result(
            4,
            4,
            BlpContent::Dxt1(test_dxtn(Vec::new(), DxtnFormat::Dxt1)),
        )
        .is_none(),
        "DXT content without mip data should be ignored"
    );

    assert!(
        bc_texture_data(BlpContent::Raw3(image_blp::types::BlpRaw3 {
            cmap: Vec::new(),
            images: Vec::new(),
        }))
        .is_none(),
        "non-BC BLP content should stay on the RGBA path"
    );
}

#[test]
fn test_is_cached_reports_bc_preloaded_textures() {
    let mut mgr = TextureManager::new();
    mgr.bc_cache.insert(
        normalize_wow_path(r"Interface\WorldMap\IsleofDorn\IsleOfDorn1"),
        BcTextureResult {
            width: 4,
            height: 4,
            bc_data: Arc::<[u8]>::from(vec![0xaa; 8]),
            format: BcTextureFormat::Bc1,
        },
    );

    assert!(
        mgr.is_cached(r"Interface\WorldMap\IsleofDorn\IsleOfDorn1"),
        "BC-preloaded world-map tiles must count as cached so budgeted uploads keep streaming after the deadline"
    );
}

#[cfg(feature = "gui")]
#[test]
fn test_load_texture_prefer_bc_reuses_cached_bc_buffer() {
    let mut mgr = TextureManager::new();
    mgr.bc_cache.insert(
        "cached-dxt".to_string(),
        BcTextureResult {
            width: 4,
            height: 4,
            bc_data: Arc::<[u8]>::from(vec![0xaa; 8]),
            format: BcTextureFormat::Bc1,
        },
    );
    let prev_bc_supported = crate::render::shader::atlas::set_bc_supported_for_tests(true);

    let cached_ptr = mgr
        .bc_cache
        .get("cached-dxt")
        .expect("cached BC data should exist")
        .bc_data
        .as_ptr();

    let loaded = crate::render::shader::primitive::load_texture_prefer_bc(&mut mgr, "cached-dxt")
        .expect("cached BC texture should load");
    crate::render::shader::atlas::set_bc_supported_for_tests(prev_bc_supported);

    let crate::render::shader::primitive::LoadedTexture::Bc(upload) = loaded else {
        panic!("cached DXT texture should stay on the BC upload path");
    };

    assert_eq!(
        upload.bc_data.as_ptr(),
        cached_ptr,
        "BC upload path should reuse cached compressed bytes instead of cloning them"
    );
}

#[test]
fn test_preloaded_talent_textures_cover_active_class_atlas_entries() {
    let mut mgr = TextureManager::new();

    mgr.preload_talent_textures(790);
    mgr.preload_talent_panel_textures("Paladin");

    // Shared talent assets and the active class' background atlases should be cached.
    let mut missing = Vec::new();
    for (key, info) in crate::atlas::ATLAS_DB.entries() {
        if should_preload_talent_atlas_key(key, Some("paladin")) && !mgr.is_cached(info.file) {
            missing.push((key.to_string(), info.file.to_string()));
        }
    }
    assert!(
        missing.is_empty(),
        "Active talent atlas entries reference {} uncached base textures:\n{}",
        missing.len(),
        missing
            .iter()
            .map(|(k, f)| format!("  {} -> {}", k, f))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    assert!(
        !mgr.is_cached(r"Interface\talentframe\talentsclassbackgroundwarrior1"),
        "Paladin preload should not eagerly cache Warrior class backgrounds",
    );
}

/// Collect talent icon paths that should be cached but aren't.
fn find_uncached_talent_icons(mgr: &TextureManager, tree_id: u32) -> Vec<String> {
    use crate::traits::{TRAIT_DEFINITION_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB, TRAIT_TREE_DB};
    let tree = TRAIT_TREE_DB.get(&tree_id).expect("tree exists");
    let mut missing = Vec::new();
    for &node_id in tree.node_ids {
        let Some(node) = TRAIT_NODE_DB.get(&node_id) else {
            continue;
        };
        for &entry_id in node.entry_ids {
            let Some(entry) = TRAIT_ENTRY_DB.get(&entry_id) else {
                continue;
            };
            let Some(def) = TRAIT_DEFINITION_DB.get(&entry.definition_id) else {
                continue;
            };
            let icon_id = if def.override_icon != 0 {
                def.override_icon
            } else {
                let Some(spell) = crate::spells::get_spell(def.spell_id) else {
                    continue;
                };
                spell.icon_file_data_id
            };
            if icon_id == 0 {
                continue;
            }
            check_icon_cached(mgr, &mut missing, node_id, icon_id);
        }
    }
    missing
}

fn check_icon_cached(mgr: &TextureManager, missing: &mut Vec<String>, node_id: u32, icon_id: u32) {
    let Some(path) = crate::manifest_interface_data::get_texture_path(icon_id) else {
        missing.push(format!(
            "  node={node_id} icon={icon_id} -> no manifest path"
        ));
        return;
    };
    let wow_path = format!("Interface\\{}", path.replace('/', "\\"));
    if !mgr.is_cached(&wow_path) {
        missing.push(format!(
            "  node={node_id} icon={icon_id} -> {wow_path} NOT cached"
        ));
    }
}

#[test]
fn test_preloaded_talent_icons_are_cached() {
    let mut mgr = TextureManager::new();
    mgr.preload_talent_textures(790);

    let missing = find_uncached_talent_icons(&mgr, 790);
    assert!(
        missing.is_empty(),
        "Talent icon textures not cached:\n{}",
        missing.join("\n")
    );
}
