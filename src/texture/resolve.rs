use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::{TextureData, TextureManager, normalize_wow_path};

impl TextureManager {
    /// Number of entries in the texture cache.
    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }

    /// Return all cached texture paths (for diagnostics/tests).
    pub fn cached_paths(&self) -> Vec<&str> {
        self.cache.keys().map(|path| path.as_str()).collect()
    }

    /// Get the dimensions of a cached texture.
    pub fn get_texture_size(&self, wow_path: &str) -> Option<(u32, u32)> {
        let normalized = normalize_wow_path(wow_path);
        self.cache
            .get(&normalized)
            .map(|d| (d.width, d.height))
            .or_else(|| self.size_cache.get(&normalized).copied())
    }

    /// Get dimensions for a texture, using cached metadata before falling back.
    pub fn get_or_load_texture_size(&mut self, wow_path: &str) -> Option<(u32, u32)> {
        if let Some((w, h)) = self.get_texture_size(wow_path) {
            return Some((w, h));
        }
        let normalized = normalize_wow_path(wow_path);
        let file_path = self.resolve_path(&normalized)?;
        let dims = super::read_texture_dimensions(&file_path).ok()?;
        self.size_cache.insert(normalized, dims);
        Some(dims)
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

        if self.sub_cache.contains_key(&key) {
            return self.sub_cache.get(&key);
        }

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

    pub fn get_cached_crop_request(&self, crop_request_path: &str) -> Option<&TextureData> {
        self.sub_cache
            .get(&normalize_crop_request_key(crop_request_path))
    }

    pub fn cache_crop_request_alias(
        &mut self,
        crop_request_path: &str,
        data: &TextureData,
    ) -> Option<&TextureData> {
        let key = normalize_crop_request_key(crop_request_path);
        self.sub_cache.insert(key.clone(), data.clone());
        self.sub_cache.get(&key)
    }

    #[cfg(test)]
    pub fn insert_test_texture(&mut self, wow_path: &str, data: TextureData) {
        let normalized = normalize_wow_path(wow_path);
        self.size_cache
            .insert(normalized.clone(), (data.width, data.height));
        self.cache.insert(normalized, data);
    }

    /// Resolve a WoW texture path to a file system path.
    pub fn resolve_path(&self, normalized_path: &str) -> Option<PathBuf> {
        if let Some(addon_relative) = normalized_path
            .strip_prefix("Interface/AddOns/")
            .or_else(|| normalized_path.strip_prefix("interface/Addons/"))
            .or_else(|| normalized_path.strip_prefix("interface/addons/"))
            && let Some(addons_path) = &self.addons_path
            && let Some(result) = self.try_resolve_in_dir(addons_path, addon_relative)
        {
            return Some(result);
        }

        let path = if normalized_path.len() >= 10
            && normalized_path[..10].eq_ignore_ascii_case("Interface/")
        {
            &normalized_path[10..]
        } else {
            normalized_path
        };

        if let Some(interface_path) = &self.interface_path
            && let Some(result) = self.try_resolve_in_dir(interface_path, path)
        {
            return Some(result);
        }

        if let Some(result) = self.try_resolve_in_dir(&self.textures_path, path) {
            return Some(result);
        }

        None
    }

    /// Try to resolve a path within a given base directory.
    fn try_resolve_in_dir(&self, base: &Path, path: &str) -> Option<PathBuf> {
        for ext in texture_extension_priority() {
            let file_path = base.join(format!("{}.{}", path, ext));
            if file_path.exists() {
                return Some(file_path);
            }
        }

        let file_path = base.join(path);
        if file_path.exists() {
            return Some(file_path);
        }

        if let Some(result) = self.resolve_case_insensitive_in(base, path) {
            return Some(result);
        }

        None
    }

    /// Resolve path with case-insensitive directory matching within a base directory.
    fn resolve_case_insensitive_in(&self, base: &Path, path: &str) -> Option<PathBuf> {
        let components: Vec<&str> = path.split('/').collect();
        let mut current = base.to_path_buf();

        for (index, component) in components.iter().enumerate() {
            let is_last = index == components.len() - 1;

            if is_last {
                for ext in texture_extension_priority() {
                    let with_ext = format!("{}.{}", component, ext);
                    if let Some(entry) = find_case_insensitive(&current, &with_ext) {
                        return Some(entry);
                    }
                }
                if let Some(entry) = find_case_insensitive(&current, component) {
                    return Some(entry);
                }
            } else if let Some(entry) = find_case_insensitive(&current, component) {
                if entry.is_dir() {
                    current = entry;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        None
    }
}

fn normalize_crop_request_key(path: &str) -> String {
    if let Some(index) = path.find("@crop:") {
        let base = normalize_wow_path(&path[..index]);
        return format!("{base}@crop:{}", &path[index + 6..]);
    }
    normalize_wow_path(path)
}

fn texture_extension_priority() -> &'static [&'static str] {
    &[
        "blp", "BLP", "webp", "WEBP", "PNG", "png", "tga", "TGA", "jpg", "JPG",
    ]
}

/// Find a directory entry case-insensitively.
fn find_case_insensitive(dir: &Path, name: &str) -> Option<PathBuf> {
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

/// Extract a sub-region from texture data.
fn extract_sub_region(
    data: &TextureData,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Option<TextureData> {
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
        pixels: Arc::<[u8]>::from(pixels),
    })
}
