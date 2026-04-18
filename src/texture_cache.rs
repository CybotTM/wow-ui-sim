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
use std::time::{Duration, Instant};

use crate::texture::TextureData;

#[derive(Debug, Default, Clone, Copy)]
pub struct DiskCacheLoadTelemetry {
    pub cache_hit: bool,
    pub probe_elapsed: Duration,
    pub read_elapsed: Duration,
    pub decompress_elapsed: Duration,
}

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

pub fn load_from_disk_cache_with_telemetry(
    cache_dir: &Path,
    normalized_path: &str,
    source_path: &Path,
) -> (Option<TextureData>, DiskCacheLoadTelemetry) {
    let probe_start = Instant::now();
    let cache_path = cache_file_path(cache_dir, normalized_path);
    let mut telemetry = DiskCacheLoadTelemetry::default();
    let Some(cache_meta) = std::fs::metadata(&cache_path).ok() else {
        telemetry.probe_elapsed = probe_start.elapsed();
        return (None, telemetry);
    };
    let Some(source_meta) = std::fs::metadata(source_path).ok() else {
        telemetry.probe_elapsed = probe_start.elapsed();
        return (None, telemetry);
    };

    let Some(cache_mtime) = cache_meta.modified().ok() else {
        telemetry.probe_elapsed = probe_start.elapsed();
        return (None, telemetry);
    };
    let Some(source_mtime) = source_meta.modified().ok() else {
        telemetry.probe_elapsed = probe_start.elapsed();
        return (None, telemetry);
    };
    if cache_mtime < source_mtime {
        telemetry.probe_elapsed = probe_start.elapsed();
        return (None, telemetry); // Cache is stale
    }
    telemetry.probe_elapsed = probe_start.elapsed();

    let (data, read_elapsed, decompress_elapsed) = read_cache_file_with_telemetry(&cache_path);
    telemetry.cache_hit = data.is_some();
    telemetry.read_elapsed = read_elapsed;
    telemetry.decompress_elapsed = decompress_elapsed;
    (data, telemetry)
}

pub fn write_to_disk_cache_with_telemetry(
    cache_dir: &Path,
    normalized_path: &str,
    data: &TextureData,
) -> Duration {
    let cache_path = cache_file_path(cache_dir, normalized_path);
    let start = Instant::now();
    if let Err(e) = write_cache_file(&cache_path, data) {
        eprintln!("[TexCache] Write error {}: {}", cache_path.display(), e);
    }
    start.elapsed()
}

fn read_cache_file_with_telemetry(path: &Path) -> (Option<TextureData>, Duration, Duration) {
    let read_start = Instant::now();
    let Some(bytes) = std::fs::read(path).ok() else {
        return (None, read_start.elapsed(), Duration::ZERO);
    };
    let read_elapsed = read_start.elapsed();
    if bytes.len() < 8 {
        return (None, read_elapsed, Duration::ZERO);
    }
    let Some(width) = bytes[0..4].try_into().ok().map(u32::from_le_bytes) else {
        return (None, read_elapsed, Duration::ZERO);
    };
    let Some(height) = bytes[4..8].try_into().ok().map(u32::from_le_bytes) else {
        return (None, read_elapsed, Duration::ZERO);
    };
    let expected_size = (width as usize) * (height as usize) * 4;
    let decompress_start = Instant::now();
    let Some(pixels) = lz4_flex::decompress_size_prepended(&bytes[8..]).ok() else {
        return (None, read_elapsed, decompress_start.elapsed());
    };
    let decompress_elapsed = decompress_start.elapsed();
    if pixels.len() != expected_size {
        return (None, read_elapsed, decompress_elapsed);
    }
    (
        Some(TextureData {
            width,
            height,
            pixels: Arc::<[u8]>::from(pixels),
        }),
        read_elapsed,
        decompress_elapsed,
    )
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
