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
    /// Base paths to addon directories (for addon textures).
    addons_paths: Vec<PathBuf>,
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
            addons_paths: Vec::new(),
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
        self.addons_paths = vec![path.into()];
        self
    }

    /// Set all addon directory paths for addon textures.
    pub fn with_addons_paths(mut self, paths: impl IntoIterator<Item = PathBuf>) -> Self {
        self.addons_paths = paths.into_iter().collect();
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

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}

fn unavailable_texture_load_result(
    start: Instant,
) -> (Option<&'static TextureData>, RgbaLoadTelemetry) {
    let telemetry = RgbaLoadTelemetry {
        total_elapsed: start.elapsed(),
        ..Default::default()
    };
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
            return self.cached_bc_load_result(&normalized, start);
        }
        if self.bc_unavailable.contains(&normalized) {
            return unavailable_bc_load_result(start);
        }

        let mut telemetry = BcLoadTelemetry::default();
        let loaded = self.load_resolved_bc_texture(&normalized, &mut telemetry);
        telemetry.total_elapsed = start.elapsed();
        let texture = loaded.then(|| self.bc_cache.get(&normalized)).flatten();
        (texture, telemetry)
    }

    fn cached_bc_load_result<'a>(
        &'a self,
        normalized: &str,
        start: Instant,
    ) -> (Option<&'a BcTextureResult>, BcLoadTelemetry) {
        let mut telemetry = BcLoadTelemetry {
            cache_hit: true,
            ..BcLoadTelemetry::default()
        };
        telemetry.total_elapsed = start.elapsed();
        (self.bc_cache.get(normalized), telemetry)
    }

    fn load_resolved_bc_texture(
        &mut self,
        normalized: &str,
        telemetry: &mut BcLoadTelemetry,
    ) -> bool {
        let resolve_start = Instant::now();
        let Some(file_path) = self.resolve_path(normalized) else {
            telemetry.resolve_elapsed = resolve_start.elapsed();
            self.mark_bc_unavailable(normalized);
            return false;
        };
        telemetry.resolve_elapsed = resolve_start.elapsed();

        if !is_blp_file(&file_path) {
            self.mark_bc_unavailable(normalized);
            return false;
        }
        telemetry.resolved_blp = true;

        let Some(bc_texture) = load_bc_texture_from_blp(&file_path, telemetry) else {
            self.mark_bc_unavailable(normalized);
            return false;
        };

        self.cache_bc_texture_data(normalized, bc_texture);
        true
    }

    fn mark_bc_unavailable(&mut self, normalized: &str) {
        self.bc_unavailable.insert(normalized.to_string());
    }

    fn cache_bc_texture_data(&mut self, normalized: &str, bc_texture: BcTextureResult) {
        self.bc_unavailable.remove(normalized);
        self.bc_cache.insert(normalized.to_string(), bc_texture);
    }
}

fn unavailable_bc_load_result(
    start: Instant,
) -> (Option<&'static BcTextureResult>, BcLoadTelemetry) {
    let telemetry = BcLoadTelemetry {
        total_elapsed: start.elapsed(),
        ..Default::default()
    };
    (None, telemetry)
}

fn is_blp_file(file_path: &Path) -> bool {
    let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    ext.eq_ignore_ascii_case("blp")
}

fn load_bc_texture_from_blp(
    file_path: &Path,
    telemetry: &mut BcLoadTelemetry,
) -> Option<BcTextureResult> {
    let parse_start = Instant::now();
    let blp = load_blp(file_path).ok();
    telemetry.parse_elapsed = parse_start.elapsed();

    let blp = blp?;
    let extract_start = Instant::now();
    let bc_texture = bc_texture_result(blp.header.width, blp.header.height, blp.content);
    telemetry.extract_elapsed = extract_start.elapsed();
    bc_texture
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
    let normalized = collapse_path_separators(&path.replace('\\', "/"));
    // Remove file extension if present
    if let Some(pos) = normalized.rfind('.')
        && normalized[pos..].len() <= 5
    {
        return normalized[..pos].to_string();
    }
    normalized
}

fn collapse_path_separators(path: &str) -> String {
    let mut collapsed = String::with_capacity(path.len());
    let mut last_was_separator = false;
    for ch in path.chars() {
        if ch == '/' {
            if !last_was_separator {
                collapsed.push(ch);
            }
            last_was_separator = true;
        } else {
            collapsed.push(ch);
            last_was_separator = false;
        }
    }
    collapsed
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
mod tests;
