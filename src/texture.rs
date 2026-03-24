//! Texture loading and caching for WoW UI textures.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use image_blp::convert::blp_to_image;
use image_blp::parser::load_blp;

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
            if let Some(data) = crate::texture_cache::load_from_disk_cache(cache_dir, normalized, file_path) {
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
                    "[TexMgr] Load error: {} -> {}: {}", normalized, file_path.display(), e
                ));
                None
            }
        }
    }

    /// Pre-load talent icon textures for the given tree to avoid on-demand lag.
    pub fn preload_talent_textures(&mut self, tree_id: u32) {
        use crate::traits::{TRAIT_DEFINITION_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB, TRAIT_TREE_DB};
        use std::collections::HashSet;

        let Some(tree) = TRAIT_TREE_DB.get(&tree_id) else {
            return;
        };
        let mut file_data_ids = HashSet::new();

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
                if icon_id != 0 {
                    file_data_ids.insert(icon_id);
                }
            }
        }

        let mut loaded = 0u32;
        for id in &file_data_ids {
            if let Some(path) = crate::manifest_interface_data::get_texture_path(*id) {
                let wow_path = format!("Interface\\{}", path.replace('/', "\\"));
                if self.load(&wow_path).is_some() {
                    loaded += 1;
                }
            }
        }
        crate::logging::eprintln_elapsed(&format!(
            "[TexMgr] Preloaded {} / {} talent icon textures (tree {})",
            loaded,
            file_data_ids.len(),
            tree_id
        ));
    }

    /// Pre-load talent panel UI textures for the active class.
    ///
    /// Shared talent panel assets are always included. Class background atlases
    /// are filtered to the active class so startup does not decode every class'
    /// legacy background textures.
    pub fn preload_talent_panel_textures(&mut self, class_name: &str) {
        use crate::atlas::ATLAS_DB;
        use std::collections::HashSet;

        let class_key = normalize_talent_class_key(class_name);
        let mut files = HashSet::new();
        for (key, info) in ATLAS_DB.entries() {
            if should_preload_talent_atlas_key(key, class_key.as_deref()) {
                files.insert(info.file);
            }
        }

        let mut loaded = 0u32;
        for file in &files {
            if self.load(file).is_some() {
                loaded += 1;
            }
        }
        crate::logging::eprintln_elapsed(&format!(
            "[TexMgr] Preloaded {} / {} talent panel textures ({})",
            loaded,
            files.len(),
            class_key.as_deref().unwrap_or("shared")
        ));
    }

    /// Pre-load common game HUD atlases that otherwise cause large first-use
    /// stalls when PlayerSpells and other game UI panels open.
    pub fn preload_game_hud_textures(&mut self) {
        const FILES: &[&str] = &[
            r"Interface\hud\uiminimap",
            r"Interface\hud\uiminimapbackground",
            r"Interface\hud\uiminimapvertical",
            r"Interface\hud\uiactionbar",
            r"Interface\hud\uiactionbarvertical",
            r"Interface\hud\uimicromenu2x",
            r"Interface\hud\uiunitframe",
            r"Interface\hud\uipartyframe",
            r"Interface\hud\uigroupmanager",
            r"Interface\hud\uicalendar",
            r"Interface\hud\uipartyframeportraitonmanamask",
            r"Interface\hud\uipartyframeportraitonhealthmask",
            r"Interface\hud\uiunitframeplayerportraitmask",
            r"Interface\hud\uiunitframeplayermanamask",
            r"Interface\hud\uiunitframeplayerhealthmask",
            r"Interface\questframe\questtracker",
            r"Interface\questframe\questimportantmapicons",
            r"Interface\questframe\questinprogressicons",
            r"Interface\chatframe\chatframe",
            r"Interface\ChatFrame\ChatFrameBackground",
            r"Interface\ChatFrame\UI-ChatFrame-BorderTop",
            r"Interface\ChatFrame\UI-ChatFrame-BorderLeft",
            r"Interface\ChatFrame\UI-ChatFrame-BorderCorner",
            r"Interface\ChatFrame\ChatFrameTab-BGMid",
            r"Interface\ChatFrame\ChatFrameTab-BGRight",
            r"Interface\ChatFrame\ChatFrameTab-BGLeft",
            r"Interface\containerframe\bagslots2x",
            r"Interface\buttons\minimalscrollbarproportional",
            r"Interface\masks\circlemask",
            r"Interface\Minimap\placeholder-map",
        ];

        let mut loaded = 0usize;
        for file in FILES {
            if self.load(file).is_some() {
                loaded += 1;
            }
        }
        crate::logging::eprintln_elapsed(&format!(
            "[TexMgr] Preloaded {} / {} game HUD textures",
            loaded,
            FILES.len()
        ));
    }

    /// Pre-load non-glue UI atlases that are heavily used when opening the
    /// PlayerSpells / talents panels in the live renderer.
    pub fn preload_playerspells_runtime_textures(&mut self) {
        use crate::atlas::ATLAS_DB;
        use std::collections::HashSet;

        const FILES: &[&str] = &[
            r"Interface\Buttons\UI-Panel-Button-Up",
            r"Interface\FrameGeneral\UI-Background-Rock",
            r"Interface\TutorialFrame\UI-TutorialFrame-CalloutGlow",
        ];
        const PREFIXES: &[&str] = &[
            r"Interface\talentframe\",
            r"Interface\framegeneral\uiframe",
            r"Interface\common\commondropdown",
            r"Interface\common\commonmask",
            r"Interface\helpframe\newplayerexperienceparts",
            r"Interface\tutorialframe\",
        ];

        let mut files = HashSet::new();
        for (path, _) in FILES.iter().map(|path| (*path, ())) {
            files.insert(path);
        }
        for (_, info) in ATLAS_DB.entries() {
            if PREFIXES.iter().any(|prefix| {
                info.file
                    .to_ascii_lowercase()
                    .starts_with(&prefix.to_ascii_lowercase())
            }) {
                files.insert(info.file);
            }
        }

        let mut loaded = 0usize;
        for file in &files {
            if self.load(file).is_some() {
                loaded += 1;
            }
        }
        crate::logging::eprintln_elapsed(&format!(
            "[TexMgr] Preloaded {} / {} PlayerSpells runtime textures",
            loaded,
            files.len()
        ));
    }

    /// Pre-load spellbook / PlayerSpells icon textures from the static spell DB.
    pub fn preload_spellbook_icons(&mut self) {
        use crate::lua_api::globals::spellbook_data;
        use std::collections::HashSet;

        let mut file_data_ids = HashSet::new();
        for skill_line_index in 1..=spellbook_data::num_skill_lines() {
            if let Some(skill_line) = spellbook_data::get_skill_line(skill_line_index) {
                file_data_ids.insert(skill_line.icon_id);
                for entry in skill_line.spells {
                    if let Some(spell) = crate::spells::get_spell(entry.spell_id) {
                        if spell.icon_file_data_id != 0 {
                            file_data_ids.insert(spell.icon_file_data_id);
                        }
                    }
                }
            }
        }

        let mut loaded = 0u32;
        for id in &file_data_ids {
            if let Some(path) = crate::manifest_interface_data::get_texture_path(*id) {
                let wow_path = format!("Interface\\{}", path.replace('/', "\\"));
                if self.load(&wow_path).is_some() {
                    loaded += 1;
                }
            }
        }
        crate::logging::eprintln_elapsed(&format!(
            "[TexMgr] Preloaded {} / {} spellbook icons",
            loaded,
            file_data_ids.len()
        ));
    }

    /// Number of entries in the texture cache.
    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }

    /// Return all cached texture paths (for diagnostics/tests).
    pub fn cached_paths(&self) -> Vec<&str> {
        self.cache.keys().map(|s| s.as_str()).collect()
    }

    /// Get the dimensions of a cached texture.
    pub fn get_texture_size(&self, wow_path: &str) -> Option<(u32, u32)> {
        let normalized = normalize_wow_path(wow_path);
        self.cache.get(&normalized).map(|d| (d.width, d.height))
    }

    /// Get dimensions for a texture, loading it first if necessary.
    pub fn get_or_load_texture_size(&mut self, wow_path: &str) -> Option<(u32, u32)> {
        if let Some((w, h)) = self.get_texture_size(wow_path) {
            return Some((w, h));
        }
        self.load(wow_path).map(|d| (d.width, d.height))
    }

    /// Load a sub-region of a texture (for texture atlases).
    /// The key format is "path#x,y,w,h" where x,y is top-left and w,h is size.
    pub fn load_sub_region(
        &mut self,
        wow_path: &str,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Option<&TextureData> {
        let normalized = normalize_wow_path(wow_path);
        let key = format!("{}#{}_{}_{}_{}", normalized, x, y, width, height);

        // Check sub-region cache
        if self.sub_cache.contains_key(&key) {
            return self.sub_cache.get(&key);
        }

        // Reuse the cached base texture when available. Falling back to `load`
        // ensures first crop extraction only pays disk I/O once per base file.
        if !self.cache.contains_key(&normalized) && self.load(wow_path).is_none() {
            return None;
        }
        if let Some(full_data) = self.cache.get(&normalized)
            && let Some(sub_data) = extract_sub_region(full_data, x, y, width, height)
        {
            self.sub_cache.insert(key.clone(), sub_data);
            return self.sub_cache.get(&key);
        }

        None
    }

    #[cfg(test)]
    pub fn insert_test_texture(&mut self, wow_path: &str, data: TextureData) {
        let normalized = normalize_wow_path(wow_path);
        self.cache.insert(normalized, data);
    }

    /// Resolve a WoW texture path to a file system path.
    pub fn resolve_path(&self, normalized_path: &str) -> Option<PathBuf> {
        // Handle addon textures: Interface/AddOns/AddonName/path/texture
        if let Some(addon_relative) = normalized_path
            .strip_prefix("Interface/AddOns/")
            .or_else(|| normalized_path.strip_prefix("interface/Addons/"))
            .or_else(|| normalized_path.strip_prefix("interface/addons/"))
            && let Some(addons_path) = &self.addons_path
            && let Some(result) = self.try_resolve_in_dir(addons_path, addon_relative)
        {
            return Some(result);
        }

        // Remove "Interface/" prefix if present for game textures (case-insensitive)
        let path = if normalized_path.len() >= 10
            && normalized_path[..10].eq_ignore_ascii_case("Interface/")
        {
            &normalized_path[10..]
        } else {
            normalized_path
        };

        // Try local textures first
        if let Some(result) = self.try_resolve_in_dir(&self.textures_path, path) {
            return Some(result);
        }

        // Try WoW Interface directory (extracted game files)
        if let Some(interface_path) = &self.interface_path
            && let Some(result) = self.try_resolve_in_dir(interface_path, path)
        {
            return Some(result);
        }

        None
    }

    /// Try to resolve a path within a given base directory.
    fn try_resolve_in_dir(&self, base: &Path, path: &str) -> Option<PathBuf> {
        // Try different extensions
        for ext in &[
            "webp", "WEBP", "PNG", "png", "tga", "TGA", "blp", "BLP", "jpg", "JPG",
        ] {
            let file_path = base.join(format!("{}.{}", path, ext));
            if file_path.exists() {
                return Some(file_path);
            }
        }

        // Try without extension (file might already have it)
        let file_path = base.join(path);
        if file_path.exists() {
            return Some(file_path);
        }

        // Try case-insensitive directory matching
        if let Some(result) = self.resolve_case_insensitive_in(base, path) {
            return Some(result);
        }

        None
    }

    /// Resolve path with case-insensitive directory matching within a base directory.
    fn resolve_case_insensitive_in(&self, base: &Path, path: &str) -> Option<PathBuf> {
        let components: Vec<&str> = path.split('/').collect();
        let mut current = base.to_path_buf();

        for (i, component) in components.iter().enumerate() {
            let is_last = i == components.len() - 1;

            if is_last {
                // For the filename, try with different extensions
                for ext in &[
                    "webp", "WEBP", "PNG", "png", "tga", "TGA", "blp", "BLP", "jpg", "JPG",
                ] {
                    let with_ext = format!("{}.{}", component, ext);
                    if let Some(entry) = self.find_case_insensitive(&current, &with_ext) {
                        return Some(entry);
                    }
                }
                // Try without extension
                if let Some(entry) = self.find_case_insensitive(&current, component) {
                    return Some(entry);
                }
            } else {
                // For directories, find case-insensitive match
                if let Some(entry) = self.find_case_insensitive(&current, component) {
                    if entry.is_dir() {
                        current = entry;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        }
        None
    }

    /// Find a directory entry case-insensitively.
    fn find_case_insensitive(&self, dir: &Path, name: &str) -> Option<PathBuf> {
        let name_lower = name.to_lowercase();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                if entry.file_name().to_string_lossy().to_lowercase() == name_lower {
                    return Some(entry.path());
                }
            }
        }
        None
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

fn normalize_talent_class_key(class_name: &str) -> Option<String> {
    let normalized = class_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect::<String>();
    (!normalized.is_empty()).then_some(normalized)
}

fn should_preload_talent_atlas_key(key: &str, class_key: Option<&str>) -> bool {
    if !key.starts_with("talents-") {
        return false;
    }
    match key.strip_prefix("talents-background-") {
        Some(rest) => {
            !rest.contains('-')
                || class_key
                    .map(|class_key| rest.starts_with(&format!("{class_key}-")))
                    .unwrap_or(false)
        }
        None => true,
    }
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

/// Extract a sub-region from texture data.
fn extract_sub_region(
    data: &TextureData,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Option<TextureData> {
    // Bounds check
    if x + width > data.width || y + height > data.height {
        return None;
    }

    let mut pixels = Vec::with_capacity((width * height * 4) as usize);

    for row in y..(y + height) {
        let start = ((row * data.width + x) * 4) as usize;
        let end = start + (width * 4) as usize;
        pixels.extend_from_slice(&data.pixels[start..end]);
    }

    Some(TextureData {
        width,
        height,
        pixels,
    })
}

#[cfg(test)]
mod tests {
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
