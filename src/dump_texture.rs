//! Dump textures used by frames to disk for debugging atlas crops.

use std::path::Path;

use crate::render::QuadBatch;
use crate::render::shader::load_texture_or_crop;
use crate::texture::TextureManager;

/// Save all unique textures from a QuadBatch to disk as PNGs.
///
/// Filters texture paths by optional substring match (case-insensitive).
/// Saves both regular and mask textures.
pub fn dump_batch_textures(
    batch: &QuadBatch,
    tex_mgr: &mut TextureManager,
    output_dir: &Path,
    filter: Option<&str>,
) {
    std::fs::create_dir_all(output_dir).ok();
    let mut seen = std::collections::HashSet::new();
    let mut saved = 0;
    let filter = filter.map(str::to_lowercase);

    for request in iter_texture_requests(batch) {
        if !should_dump_texture(&request.path, &mut seen, filter.as_deref()) {
            continue;
        }
        saved += dump_texture_request(tex_mgr, output_dir, &request.path);
    }
    eprintln!("Saved {saved} textures to {}", output_dir.display());
}

fn iter_texture_requests(
    batch: &QuadBatch,
) -> impl Iterator<Item = &crate::render::TextureRequest> {
    batch
        .texture_requests
        .iter()
        .chain(&batch.mask_texture_requests)
}

fn should_dump_texture(
    path: &str,
    seen: &mut std::collections::HashSet<String>,
    filter: Option<&str>,
) -> bool {
    if !seen.insert(path.to_string()) {
        return false;
    }
    filter.is_none_or(|needle| path.to_lowercase().contains(needle))
}

fn dump_texture_request(tex_mgr: &mut TextureManager, output_dir: &Path, path: &str) -> usize {
    let Some(gpu_data) = load_texture_or_crop(tex_mgr, path) else {
        eprintln!("  FAILED: {path}");
        return 0;
    };

    let filename = sanitize_texture_filename(path);
    let out_path = output_dir.join(&filename);
    match image::RgbaImage::from_raw(gpu_data.width, gpu_data.height, gpu_data.rgba.to_vec()) {
        Some(img) => save_texture_image(img, &out_path, &filename, gpu_data.width, gpu_data.height),
        None => {
            eprintln!(
                "  BAD DATA: {path} ({}x{}, expected {} bytes)",
                gpu_data.width,
                gpu_data.height,
                gpu_data.width * gpu_data.height * 4
            );
            0
        }
    }
}

fn save_texture_image(
    img: image::RgbaImage,
    out_path: &Path,
    filename: &str,
    width: u32,
    height: u32,
) -> usize {
    if let Err(e) = img.save(out_path) {
        eprintln!("  ERROR saving {filename}: {e}");
        return 0;
    }
    eprintln!("  {width}x{height} → {}", out_path.display());
    1
}

/// Convert a texture path (with @crop: suffix) to a safe filename.
fn sanitize_texture_filename(path: &str) -> String {
    let name = path
        .replace('\\', "_")
        .replace('/', "_")
        .replace('@', "_at_")
        .replace(':', "_")
        .replace(',', "_");
    // Strip leading underscores from Interface_ prefix
    let name = name.trim_start_matches('_');
    format!("{name}.png")
}
