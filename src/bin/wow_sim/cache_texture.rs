use std::path::PathBuf;
use std::time::Instant;
use wow_ui_sim::texture::TextureManager;

pub fn run_cache_texture(path: &str, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let normalized = wow_ui_sim::texture::normalize_wow_path(path);
    println!("path:       {path}");
    println!("normalized: {normalized}");

    if force {
        print_removed_missing_sentinels(&normalized);
    }

    let mut mgr =
        TextureManager::new().with_addons_paths(wow_ui_sim::paths::default_addons_paths());
    let load_start = Instant::now();
    let result = mgr.load(path);
    let elapsed = load_start.elapsed();

    match result {
        Some(td) => {
            let (w, h) = (td.width, td.height);
            println!("status:     OK");
            println!("dimensions: {w}x{h}");
            println!("elapsed:    {elapsed:.2?}");
            Ok(())
        }
        None => {
            println!("status:     MISS");
            println!("elapsed:    {elapsed:.2?}");
            print_missing_sentinels(&normalized);
            std::process::exit(1);
        }
    }
}

fn print_removed_missing_sentinels(normalized: &str) {
    for marker in remove_missing_sentinels(normalized) {
        println!("removed:    {}", marker.display());
    }
}

fn print_missing_sentinels(normalized: &str) {
    for marker in list_missing_sentinels(normalized) {
        println!("sentinel:   {}", marker.display());
    }
}

fn casc_extract_root() -> Option<PathBuf> {
    dirs::cache_dir().map(|d| d.join("wow-ui-sim/casc-extract"))
}

fn candidate_extract_paths(normalized: &str) -> Vec<PathBuf> {
    let Some(root) = casc_extract_root() else {
        return Vec::new();
    };
    let extensions = ["blp", "BLP", "tga", "TGA", "ttf", "TTF", "otf", "OTF"];
    let mut paths = Vec::with_capacity(extensions.len() + 1);
    paths.push(root.join(normalized));
    for ext in extensions {
        paths.push(root.join(format!("{normalized}.{ext}")));
    }
    paths
}

fn remove_missing_sentinels(normalized: &str) -> Vec<PathBuf> {
    matching_missing_sentinels(normalized)
        .into_iter()
        .filter(|marker| std::fs::remove_file(marker).is_ok())
        .collect()
}

fn list_missing_sentinels(normalized: &str) -> Vec<PathBuf> {
    matching_missing_sentinels(normalized)
}

fn matching_missing_sentinels(normalized: &str) -> Vec<PathBuf> {
    candidate_extract_paths(normalized)
        .into_iter()
        .filter_map(missing_sentinel_for_path)
        .filter(|marker| marker.exists())
        .collect()
}

fn missing_sentinel_for_path(path: PathBuf) -> Option<PathBuf> {
    let file_name = path.file_name()?.to_str()?;
    Some(path.with_file_name(format!("{file_name}.missing")))
}
