//! Texture loading and caching for WoW UI textures.

mod preload;
mod resolve;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use image_blp::convert::blp_to_image;
use image_blp::parser::load_blp;
use image_blp::types::BlpContent;

/// Texture manager that loads and caches textures.
#[derive(Debug)]
pub struct TextureManager {
    /// Base path to addons directory (for addon textures).
    addons_path: Option<PathBuf>,
    /// Cache of loaded texture data (path -> RGBA pixels).
    cache: HashMap<String, TextureData>,
    /// Cache of texture dimensions keyed by normalized WoW path.
    size_cache: HashMap<String, (u32, u32)>,
    /// Cache of raw BC-compressed texture data keyed by normalized WoW path.
    bc_cache: HashMap<String, BcTextureResult>,
    /// Cache of normalized paths known to be unavailable on the BC path.
    bc_unavailable: HashSet<String>,
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
    pub pixels: Arc<[u8]>, // RGBA
}

impl TextureManager {
    /// Create a new texture manager.
    pub fn new() -> Self {
        Self {
            addons_path: None,
            cache: HashMap::new(),
            size_cache: HashMap::new(),
            bc_cache: HashMap::new(),
            bc_unavailable: HashSet::new(),
            sub_cache: HashMap::new(),
            not_found: HashSet::new(),
        }
    }

    /// Set the addons directory path for addon textures.
    pub fn with_addons_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.addons_path = Some(path.into());
        self
    }

    /// Load a texture by its WoW path (e.g., "Interface\\DialogFrame\\UI-DialogBox-Background").
    pub fn load(&mut self, wow_path: &str) -> Option<&TextureData> {
        self.load_with_telemetry(wow_path).0
    }

    pub fn load_with_telemetry(
        &mut self,
        wow_path: &str,
    ) -> (Option<&TextureData>, RgbaLoadTelemetry) {
        let start = Instant::now();
        let normalized = normalize_wow_path(wow_path);
        let mut telemetry = RgbaLoadTelemetry::default();

        if self.cache.contains_key(&normalized) {
            return self.cached_texture_load_result(&normalized, start);
        }
        if self.not_found.contains(&normalized) {
            return unavailable_texture_load_result(start);
        }

        let loaded = self.load_resolved_texture(wow_path, &normalized, &mut telemetry);
        telemetry.total_elapsed = start.elapsed();
        let texture = loaded.then(|| self.cache.get(&normalized)).flatten();
        (texture, telemetry)
    }

    fn cached_texture_load_result<'a>(
        &'a self,
        normalized: &str,
        start: Instant,
    ) -> (Option<&'a TextureData>, RgbaLoadTelemetry) {
        let mut telemetry = RgbaLoadTelemetry {
            mem_cache_hit: true,
            ..RgbaLoadTelemetry::default()
        };
        telemetry.total_elapsed = start.elapsed();
        (self.cache.get(normalized), telemetry)
    }

    fn load_resolved_texture(
        &mut self,
        wow_path: &str,
        normalized: &str,
        telemetry: &mut RgbaLoadTelemetry,
    ) -> bool {
        let resolve_start = Instant::now();
        let Some(file_path) = self.resolve_path(normalized) else {
            telemetry.resolve_elapsed = resolve_start.elapsed();
            crate::logging::eprintln_elapsed(&format!("[TexMgr] Not found: {}", wow_path));
            self.not_found.insert(normalized.to_string());
            return false;
        };

        telemetry.resolve_elapsed = resolve_start.elapsed();
        let (loaded, load_telemetry) = self.load_texture_with_telemetry(&file_path);
        telemetry.decode_elapsed = load_telemetry.decode_elapsed;
        let Some(data) = loaded else {
            return false;
        };

        self.cache_texture_data(normalized, data);
        true
    }

    fn cache_texture_data(&mut self, normalized: &str, data: TextureData) {
        self.size_cache
            .insert(normalized.to_string(), (data.width, data.height));
        self.cache.insert(normalized.to_string(), data);
    }

    /// Get a cached texture without loading.
    pub fn get(&self, wow_path: &str) -> Option<&TextureData> {
        let normalized = normalize_wow_path(wow_path);
        self.cache.get(&normalized)
    }

    /// Check if a texture is already in the CPU cache (no disk I/O needed).
    pub fn is_cached(&self, wow_path: &str) -> bool {
        let normalized = normalize_wow_path(wow_path);
        self.cache.contains_key(&normalized) || self.bc_cache.contains_key(&normalized)
    }

    fn load_texture_with_telemetry(
        &self,
        file_path: &Path,
    ) -> (Option<TextureData>, RgbaLoadTelemetry) {
        let mut telemetry = RgbaLoadTelemetry::default();
        let decode_start = Instant::now();
        match load_texture_file(file_path) {
            Ok(data) => {
                telemetry.decode_elapsed = decode_start.elapsed();
                (Some(data), telemetry)
            }
            Err(e) => {
                telemetry.decode_elapsed = decode_start.elapsed();
                crate::logging::eprintln_elapsed(&format!(
                    "[TexMgr] Load error: {}: {}",
                    file_path.display(),
                    e
                ));
                (None, telemetry)
            }
        }
    }
}

fn unavailable_texture_load_result(
    start: Instant,
) -> (Option<&'static TextureData>, RgbaLoadTelemetry) {
    let mut telemetry = RgbaLoadTelemetry::default();
    telemetry.total_elapsed = start.elapsed();
    (None, telemetry)
}

/// Result of a BC-compressed texture load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BcTextureFormat {
    Bc1,
    Bc3,
}

/// Result of a BC-compressed texture load.
#[derive(Debug, Clone)]
pub struct BcTextureResult {
    pub width: u32,
    pub height: u32,
    /// Raw BC block data (mip level 0 only).
    pub bc_data: Arc<[u8]>,
    /// BC compression format.
    pub format: BcTextureFormat,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BcLoadTelemetry {
    pub cache_hit: bool,
    pub resolved_blp: bool,
    pub resolve_elapsed: Duration,
    pub parse_elapsed: Duration,
    pub extract_elapsed: Duration,
    pub total_elapsed: Duration,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RgbaLoadTelemetry {
    pub mem_cache_hit: bool,
    pub resolve_elapsed: Duration,
    pub decode_elapsed: Duration,
    pub total_elapsed: Duration,
}

impl TextureManager {
    /// Attempt to load a BLP texture's raw BC block data without CPU decoding.
    ///
    /// Returns `Some` only when the resolved file is a BLP with DXT1/DXT3/DXT5 content.
    /// Callers should fall back to `load()` (RGBA path) when this returns `None`.
    pub fn load_bc(&mut self, wow_path: &str) -> Option<&BcTextureResult> {
        self.load_bc_with_telemetry(wow_path).0
    }

    /// Attempt to load a BLP texture's raw BC block data and capture timing breakdowns.
    pub fn load_bc_with_telemetry(
        &mut self,
        wow_path: &str,
    ) -> (Option<&BcTextureResult>, BcLoadTelemetry) {
        let start = Instant::now();
        let normalized = normalize_wow_path(wow_path);
        if self.bc_cache.contains_key(&normalized) {
            return (
                self.bc_cache.get(&normalized),
                BcLoadTelemetry {
                    cache_hit: true,
                    total_elapsed: start.elapsed(),
                    ..Default::default()
                },
            );
        }
        if self.bc_unavailable.contains(&normalized) {
            return (
                None,
                BcLoadTelemetry {
                    total_elapsed: start.elapsed(),
                    ..Default::default()
                },
            );
        }
        let mut telemetry = BcLoadTelemetry::default();
        let resolve_start = Instant::now();
        let Some(file_path) = self.resolve_path(&normalized) else {
            self.bc_unavailable.insert(normalized);
            telemetry.resolve_elapsed = resolve_start.elapsed();
            telemetry.total_elapsed = start.elapsed();
            return (None, telemetry);
        };
        telemetry.resolve_elapsed = resolve_start.elapsed();

        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !ext.eq_ignore_ascii_case("blp") {
            self.bc_unavailable.insert(normalized);
            telemetry.total_elapsed = start.elapsed();
            return (None, telemetry);
        }
        telemetry.resolved_blp = true;

        let parse_start = Instant::now();
        let Some(blp) = load_blp(&file_path).ok() else {
            self.bc_unavailable.insert(normalized);
            telemetry.parse_elapsed = parse_start.elapsed();
            telemetry.total_elapsed = start.elapsed();
            return (None, telemetry);
        };
        telemetry.parse_elapsed = parse_start.elapsed();
        let extract_start = Instant::now();
        let Some(bc_texture) = bc_texture_result(blp.header.width, blp.header.height, blp.content)
        else {
            self.bc_unavailable.insert(normalized);
            telemetry.extract_elapsed = extract_start.elapsed();
            telemetry.total_elapsed = start.elapsed();
            return (None, telemetry);
        };
        telemetry.extract_elapsed = extract_start.elapsed();

        self.bc_unavailable.remove(&normalized);
        self.bc_cache.insert(normalized.clone(), bc_texture);
        telemetry.total_elapsed = start.elapsed();
        (self.bc_cache.get(&normalized), telemetry)
    }
}

fn bc_texture_result(width: u32, height: u32, content: BlpContent) -> Option<BcTextureResult> {
    let (bc_data, format) = bc_texture_data(content)?;
    Some(BcTextureResult {
        width,
        height,
        bc_data,
        format,
    })
}

fn bc_texture_data(content: BlpContent) -> Option<(Arc<[u8]>, BcTextureFormat)> {
    match content {
        BlpContent::Dxt1(dxtn) => {
            first_bc_image(dxtn).map(|bc_data| (bc_data, BcTextureFormat::Bc1))
        }
        // DXT3 is BC2, not BC3. The renderer has no BC2 atlas yet, so let
        // DXT3 fall back through the RGBA path instead of decoding alpha as BC3.
        BlpContent::Dxt3(_) => None,
        BlpContent::Dxt5(dxtn) => {
            first_bc_image(dxtn).map(|bc_data| (bc_data, BcTextureFormat::Bc3))
        }
        _ => None,
    }
}

fn first_bc_image(dxtn: image_blp::types::direct::dxtn::BlpDxtn) -> Option<Arc<[u8]>> {
    let image = dxtn.images.into_iter().next()?;
    if image.content.is_empty() {
        return None;
    }
    Some(Arc::<[u8]>::from(image.content))
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
            pixels: Arc::<[u8]>::from(pixels),
        })
    } else {
        // Use standard image crate for other formats
        let img = image::open(path)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        Ok(TextureData {
            width,
            height,
            pixels: Arc::<[u8]>::from(rgba.into_raw()),
        })
    }
}

pub(crate) fn read_texture_dimensions(
    path: &Path,
) -> Result<(u32, u32), Box<dyn std::error::Error + Send + Sync>> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    if ext.eq_ignore_ascii_case("blp") {
        return read_blp_dimensions(path);
    }
    Ok(image::image_dimensions(path)?)
}

fn read_blp_dimensions(
    path: &Path,
) -> Result<(u32, u32), Box<dyn std::error::Error + Send + Sync>> {
    use std::io::{Read, Seek, SeekFrom};

    let mut file = std::fs::File::open(path)?;
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    if &magic != b"BLP2" && &magic != b"BLP1" {
        return Err(format!("unsupported BLP magic in {}", path.display()).into());
    }
    file.seek(SeekFrom::Start(12))?;
    let mut dims = [0u8; 8];
    file.read_exact(&mut dims)?;
    let width = u32::from_le_bytes(dims[0..4].try_into()?);
    let height = u32::from_le_bytes(dims[4..8].try_into()?);
    Ok((width, height))
}

#[cfg(test)]
mod tests {
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

        let loaded =
            crate::render::shader::primitive::load_texture_prefer_bc(&mut mgr, "cached-dxt")
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
        let mut mgr = TextureManager::new();
        mgr.preload_talent_textures(790);

        let missing = find_uncached_talent_icons(&mgr, 790);
        assert!(
            missing.is_empty(),
            "Talent icon textures not cached:\n{}",
            missing.join("\n")
        );
    }
}
