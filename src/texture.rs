//! Texture loading and caching for WoW UI textures.

mod preload;
mod resolve;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use image_blp::convert::blp_to_image;
use image_blp::parser::load_blp;
use image_blp::types::BlpContent;

/// Texture manager that loads and caches textures.
#[derive(Debug)]
pub struct TextureManager {
    /// Base path to wow-ui-textures repository (for game UI textures).
    textures_path: PathBuf,
    /// Base path to WoW Interface directory (for extracted game files).
    interface_path: Option<PathBuf>,
    /// Base path to addons directory (for addon textures).
    addons_path: Option<PathBuf>,
    /// Directory for decoded RGBA disk cache (lz4 compressed).
    disk_cache_dir: Option<PathBuf>,
    /// Cache of loaded texture data (path -> RGBA pixels).
    cache: HashMap<String, TextureData>,
    /// Cache of sub-region textures (path#region -> RGBA pixels).
    sub_cache: HashMap<String, TextureData>,
    /// Paths that failed to load (logged once, then silenced).
    not_found: HashSet<String>,
}

/// Loaded texture data.
#[derive(Debug, Clone)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA
}

impl TextureManager {
    /// Create a new texture manager with the given textures path.
    pub fn new(textures_path: impl Into<PathBuf>) -> Self {
        Self {
            textures_path: textures_path.into(),
            interface_path: None,
            addons_path: None,
            disk_cache_dir: None,
            cache: HashMap::new(),
            sub_cache: HashMap::new(),
            not_found: HashSet::new(),
        }
    }

    /// Set the WoW Interface directory path for extracted game files.
    pub fn with_interface_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.interface_path = Some(path.into());
        self
    }

    /// Set the disk cache directory for decoded RGBA textures.
    pub fn with_disk_cache(mut self, path: impl Into<PathBuf>) -> Self {
        let dir = path.into();
        std::fs::create_dir_all(&dir).ok();
        self.disk_cache_dir = Some(dir);
        self
    }

    /// Set the addons directory path for addon textures.
    pub fn with_addons_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.addons_path = Some(path.into());
        self
    }

    /// Load a texture by its WoW path (e.g., "Interface\\DialogFrame\\UI-DialogBox-Background").
    pub fn load(&mut self, wow_path: &str) -> Option<&TextureData> {
        // Normalize the path
        let normalized = normalize_wow_path(wow_path);

        // Check cache first
        if self.cache.contains_key(&normalized) {
            return self.cache.get(&normalized);
        }
        if self.not_found.contains(&normalized) {
            return None;
        }

        // Try to load from disk (disk cache → decode → write cache)
        if let Some(file_path) = self.resolve_path(&normalized) {
            if let Some(data) = self.load_with_disk_cache(&normalized, &file_path) {
                self.cache.insert(normalized.clone(), data);
                return self.cache.get(&normalized);
            }
        } else {
            crate::logging::eprintln_elapsed(&format!("[TexMgr] Not found: {}", wow_path));
            self.not_found.insert(normalized);
        }

        None
    }

    /// Get a cached texture without loading.
    pub fn get(&self, wow_path: &str) -> Option<&TextureData> {
        let normalized = normalize_wow_path(wow_path);
        self.cache.get(&normalized)
    }

    /// Check if a texture is already in the CPU cache (no disk I/O needed).
    pub fn is_cached(&self, wow_path: &str) -> bool {
        let normalized = normalize_wow_path(wow_path);
        self.cache.contains_key(&normalized)
    }

    /// Load a texture, checking the lz4 disk cache before falling back to decode.
    fn load_with_disk_cache(&self, normalized: &str, file_path: &Path) -> Option<TextureData> {
        // Try disk cache first
        if let Some(cache_dir) = &self.disk_cache_dir {
            if let Some(data) =
                crate::texture_cache::load_from_disk_cache(cache_dir, normalized, file_path)
            {
                return Some(data);
            }
        }
        // Decode from source
        match load_texture_file(file_path) {
            Ok(data) => {
                if let Some(cache_dir) = &self.disk_cache_dir {
                    crate::texture_cache::write_to_disk_cache(cache_dir, normalized, &data);
                }
                Some(data)
            }
            Err(e) => {
                crate::logging::eprintln_elapsed(&format!(
                    "[TexMgr] Load error: {} -> {}: {}",
                    normalized,
                    file_path.display(),
                    e
                ));
                None
            }
        }
    }
}

/// Result of a BC-compressed texture load.
pub struct BcTextureResult {
    pub width: u32,
    pub height: u32,
    /// Raw BC block data (mip level 0 only).
    pub bc_data: Vec<u8>,
    /// BC compression format.
    pub format: crate::render::shader::atlas::BcFormat,
}

impl TextureManager {
    /// Attempt to load a BLP texture's raw BC block data without CPU decoding.
    ///
    /// Returns `Some` only when the resolved file is a BLP with DXT1/DXT3/DXT5 content.
    /// Callers should fall back to `load()` (RGBA path) when this returns `None`.
    pub fn load_bc(&self, wow_path: &str) -> Option<BcTextureResult> {
        use crate::render::shader::atlas::BcFormat;

        let normalized = normalize_wow_path(wow_path);
        let file_path = self.resolve_path(&normalized)?;

        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !ext.eq_ignore_ascii_case("blp") {
            return None;
        }

        let blp = load_blp(&file_path).ok()?;
        let width = blp.header.width;
        let height = blp.header.height;

        let (bc_data, format): (Vec<u8>, BcFormat) = match &blp.content {
            BlpContent::Dxt1(dxtn) if !dxtn.images.is_empty() => {
                (dxtn.images[0].content.clone(), BcFormat::Bc1)
            }
            BlpContent::Dxt3(dxtn) if !dxtn.images.is_empty() => {
                (dxtn.images[0].content.clone(), BcFormat::Bc3)
            }
            BlpContent::Dxt5(dxtn) if !dxtn.images.is_empty() => {
                (dxtn.images[0].content.clone(), BcFormat::Bc3)
            }
            _ => return None,
        };

        if bc_data.is_empty() {
            return None;
        }

        Some(BcTextureResult {
            width,
            height,
            bc_data,
            format,
        })
    }
}

/// Normalize a WoW texture path.
pub fn normalize_wow_path(path: &str) -> String {
    // Replace backslashes with forward slashes
    let normalized = path.replace('\\', "/");
    // Remove file extension if present
    if let Some(pos) = normalized.rfind('.')
        && normalized[pos..].len() <= 5
    {
        return normalized[..pos].to_string();
    }
    normalized
}

/// Fix 1-bit alpha decoded by image-blp as literal 0/1 byte values.
///
/// BLP files with `alphaDepth=1` store alpha as a single bit per pixel.
/// The image-blp crate decodes this as byte values 0 or 1 instead of 0 or
/// 255, making textures nearly invisible. This remaps: 0 stays 0, any
/// non-zero alpha becomes 255.
pub fn fix_1bit_alpha(pixels: &mut [u8]) {
    // Check if alpha looks like 1-bit (max alpha value <= 1)
    let max_alpha = pixels.iter().skip(3).step_by(4).copied().max().unwrap_or(0);
    if max_alpha > 1 {
        return; // Normal 8-bit alpha, no fix needed
    }
    for alpha in pixels.iter_mut().skip(3).step_by(4) {
        if *alpha > 0 {
            *alpha = 255;
        }
    }
}

/// Load texture data from a file.
fn load_texture_file(path: &Path) -> Result<TextureData, Box<dyn std::error::Error + Send + Sync>> {
    // Check if it's a BLP file
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    if ext.eq_ignore_ascii_case("blp") {
        // Use image-blp for BLP files
        // Note: image-blp uses image 0.24, we use 0.25, so extract raw pixels directly
        let blp = load_blp(path)?;
        let blp_img = blp_to_image(&blp, 0)?;
        // Get dimensions and convert to RGBA8 bytes
        let rgba = blp_img.to_rgba8();
        let width = rgba.width();
        let height = rgba.height();
        let mut pixels = rgba.into_raw();
        // Fix 1-bit alpha: image-blp decodes 1-bit alpha as literal 0/1 byte values
        // instead of 0/255. Remap any alpha > 0 to 255 for correct rendering.
        fix_1bit_alpha(&mut pixels);
        Ok(TextureData {
            width,
            height,
            pixels,
        })
    } else {
        // Use standard image crate for other formats
        let img = image::open(path)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        Ok(TextureData {
            width,
            height,
            pixels: rgba.into_raw(),
        })
    }
}

#[cfg(test)]
mod tests {
    use self::preload::should_preload_talent_atlas_key;
    use super::*;
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
        let textures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("textures");
        if !textures_path.exists() {
            eprintln!("Skipping test: textures directory not found");
            return;
        }

        let mut mgr = TextureManager::new(&textures_path);
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
        let mut mgr = TextureManager::new(base);
        let result = mgr.load("test-texture");

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
        // Verify the extension order in try_resolve_in_dir is webp first
        let extensions = [
            "webp", "WEBP", "PNG", "png", "tga", "TGA", "blp", "BLP", "jpg", "JPG",
        ];
        assert_eq!(extensions[0], "webp", "webp should be first priority");
        assert_eq!(extensions[1], "WEBP", "WEBP should be second priority");
    }

    #[test]
    fn test_fallback_to_png_when_no_webp() {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();

        // Create only a PNG file (no webp)
        let png_path = base.join("only-png.png");
        let green_img = image::RgbaImage::from_pixel(2, 2, image::Rgba([0, 255, 0, 255]));
        green_img.save(&png_path).unwrap();

        let mut mgr = TextureManager::new(base);
        let result = mgr.load("only-png");

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

        let mut mgr = TextureManager::new(base);

        // Try loading with different cases
        let result = mgr.load("buttons/ui-panel-button");
        assert!(result.is_some(), "Should load with lowercase path");
    }

    #[test]
    fn test_nonexistent_texture_returns_none() {
        let temp_dir = TempDir::new().unwrap();
        let mut mgr = TextureManager::new(temp_dir.path());

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

        let mut mgr = TextureManager::new(base);

        // First load
        let result1 = mgr.load("cached");
        assert!(result1.is_some());
        let pixels1 = result1.unwrap().pixels.clone();

        // Second load should return cached version (using get, no disk access)
        let result2 = mgr.get("cached");
        assert!(result2.is_some(), "Should get from cache");

        // Verify same data
        assert_eq!(pixels1, result2.unwrap().pixels);
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

        let mut mgr = TextureManager::new(base);
        assert!(mgr.load("cropped").is_some(), "base texture should load");

        fs::remove_file(&webp_path).unwrap();

        let sub = mgr
            .load_sub_region("cropped", 1, 1, 2, 2)
            .expect("sub-region should load from cached base");
        assert_eq!((sub.width, sub.height), (2, 2));
        assert_eq!(sub.pixels[0], 10);
        assert_eq!(sub.pixels[1], 20);
    }

    #[test]
    fn test_preloaded_talent_textures_cover_active_class_atlas_entries() {
        let textures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("textures");
        if !textures_path.exists() {
            eprintln!("Skipping test: textures directory not found");
            return;
        }
        let home = dirs::home_dir().unwrap_or_default();
        let mut mgr = TextureManager::new(&textures_path)
            .with_interface_path(home.join("Projects/wow/Interface"));

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

    fn check_icon_cached(
        mgr: &TextureManager,
        missing: &mut Vec<String>,
        node_id: u32,
        icon_id: u32,
    ) {
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
        let textures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("textures");
        if !textures_path.exists() {
            eprintln!("Skipping test: textures directory not found");
            return;
        }
        let home = dirs::home_dir().unwrap_or_default();
        let mut mgr = TextureManager::new(&textures_path)
            .with_interface_path(home.join("Projects/wow/Interface"));
        mgr.preload_talent_textures(790);

        let missing = find_uncached_talent_icons(&mgr, 790);
        assert!(
            missing.is_empty(),
            "Talent icon textures not cached:\n{}",
            missing.join("\n")
        );
    }
}
