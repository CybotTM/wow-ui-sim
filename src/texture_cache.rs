//! Disk cache for decoded RGBA texture data (lz4 compressed).
//!
//! Cache files: `{cache_dir}/{hash}.rgba.lz4`
//! Format: [u32 width][u32 height][lz4-compressed RGBA pixels]
//!
//! Cache validity: source file mtime is checked against cache file mtime.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::texture::TextureData;

/// Compute a stable hash for the normalized texture path.
fn cache_hash(normalized_path: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    normalized_path.hash(&mut hasher);
    hasher.finish()
}

/// Build the cache file path for a given normalized texture path.
fn cache_file_path(cache_dir: &Path, normalized_path: &str) -> PathBuf {
    cache_dir.join(format!("{:016x}.rgba.lz4", cache_hash(normalized_path)))
}

/// Try to load texture data from the disk cache.
///
/// Returns `None` if the cache file doesn't exist or is stale (older than source).
pub fn load_from_disk_cache(
    cache_dir: &Path,
    normalized_path: &str,
    source_path: &Path,
) -> Option<TextureData> {
    let cache_path = cache_file_path(cache_dir, normalized_path);
    let cache_meta = std::fs::metadata(&cache_path).ok()?;
    let source_meta = std::fs::metadata(source_path).ok()?;

    let cache_mtime = cache_meta.modified().ok()?;
    let source_mtime = source_meta.modified().ok()?;
    if cache_mtime < source_mtime {
        return None; // Cache is stale
    }

    read_cache_file(&cache_path)
}

/// Write texture data to the disk cache.
pub fn write_to_disk_cache(cache_dir: &Path, normalized_path: &str, data: &TextureData) {
    let cache_path = cache_file_path(cache_dir, normalized_path);
    if let Err(e) = write_cache_file(&cache_path, data) {
        eprintln!("[TexCache] Write error {}: {}", cache_path.display(), e);
    }
}

fn read_cache_file(path: &Path) -> Option<TextureData> {
    let bytes = std::fs::read(path).ok()?;
    if bytes.len() < 8 {
        return None;
    }
    let width = u32::from_le_bytes(bytes[0..4].try_into().ok()?);
    let height = u32::from_le_bytes(bytes[4..8].try_into().ok()?);
    let expected_size = (width as usize) * (height as usize) * 4;
    let pixels = lz4_flex::decompress_size_prepended(&bytes[8..]).ok()?;
    if pixels.len() != expected_size {
        return None;
    }
    Some(TextureData {
        width,
        height,
        pixels: Arc::<[u8]>::from(pixels),
    })
}

fn write_cache_file(path: &Path, data: &TextureData) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Vec::with_capacity(8 + data.pixels.len() / 2);
    buf.extend_from_slice(&data.width.to_le_bytes());
    buf.extend_from_slice(&data.height.to_le_bytes());
    let compressed = lz4_flex::compress_prepend_size(&data.pixels);
    buf.extend_from_slice(&compressed);
    std::fs::write(path, &buf)?;
    Ok(())
}
