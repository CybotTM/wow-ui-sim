//! Concrete timing benchmark: BLP load from disk cache vs CASC extract.
//!
//! Picks a small set of textures, then for each one:
//!   1. Force cache miss (delete extracted BLP) and time full pipeline.
//!   2. Time again with the BLP present (warm disk cache, fresh TextureManager).
//!   3. Time once more on the same TextureManager (warm in-memory cache).

use std::path::PathBuf;
use std::time::{Duration, Instant};
use wow_ui_sim::texture::TextureManager;

const PROBES: &[&str] = &[
    "Interface\\Buttons\\UI-Panel-Button-Up",
    "Interface\\DialogFrame\\UI-DialogBox-Background",
    "Interface\\Icons\\INV_Misc_QuestionMark",
    "Interface\\Buttons\\UI-Panel-MinimizeButton-Up",
    "Interface\\WorldMap\\GEAR_64GREY",
    "Interface\\Tooltips\\UI-Tooltip-Background",
    "Interface\\PaperDoll\\UI-Backpack-EmptySlot",
    "Interface\\TargetingFrame\\UI-StatusBar",
];

fn cache_extract_dir() -> PathBuf {
    dirs::cache_dir()
        .expect("cache_dir")
        .join("wow-ui-sim/casc-extract")
}

/// Best-effort delete of every cached path that could host this probe.
fn purge_cached_blp(probe: &str) -> usize {
    let normalized = probe.replace('\\', "/");
    let stripped = normalized
        .strip_prefix("Interface/")
        .or_else(|| normalized.strip_prefix("interface/"))
        .or_else(|| normalized.strip_prefix("INTERFACE/"))
        .unwrap_or(&normalized);

    let cache_root = cache_extract_dir();
    let mut deleted = 0;
    for case in ["Interface", "interface", "INTERFACE"] {
        for ext in ["blp", "BLP"] {
            let path = cache_root.join(case).join(format!("{stripped}.{ext}"));
            if path.exists() {
                let _ = std::fs::remove_file(&path);
                deleted += 1;
            }
        }
    }
    deleted
}

fn time_load(mgr: &mut TextureManager, probe: &str) -> (Option<(u32, u32)>, Duration) {
    let start = Instant::now();
    let dims = mgr.load(probe).map(|d| (d.width, d.height));
    (dims, start.elapsed())
}

fn human(d: Duration) -> String {
    let micros = d.as_secs_f64() * 1_000_000.0;
    if micros < 10.0 {
        format!("{:>9.3} µs", micros)
    } else if micros < 1_000.0 {
        format!("{:>9.1} µs", micros)
    } else {
        format!("{:>9.2} ms", micros / 1_000.0)
    }
}

fn main() {
    unsafe {
        std::env::set_var("WOW_SIM_CASC", "1");
    }

    println!(
        "{:55}  {:>12}  {:>12}  {:>12}",
        "probe", "CASC extract", "disk cache", "mem cache"
    );
    println!("{}", "-".repeat(55 + 14 * 3));

    let mut totals = [Duration::ZERO; 3];

    for probe in PROBES {
        // 1) Cache miss: force re-extract.
        let removed = purge_cached_blp(probe);
        let mut mgr = TextureManager::new();
        let (dims, t_extract) = time_load(&mut mgr, probe);
        let dims_str = match dims {
            Some((w, h)) => format!("{w}x{h}"),
            None => "MISS".into(),
        };

        // 2) Disk cache hit: same probe, fresh TextureManager (no in-memory cache).
        let mut mgr = TextureManager::new();
        let (_, t_disk) = time_load(&mut mgr, probe);

        // 3) In-memory hit on the same TextureManager.
        let (_, t_mem) = time_load(&mut mgr, probe);

        totals[0] += t_extract;
        totals[1] += t_disk;
        totals[2] += t_mem;

        println!(
            "{:55}  {}  {}  {}  ({}, purged {} stale)",
            probe,
            human(t_extract),
            human(t_disk),
            human(t_mem),
            dims_str,
            removed
        );
    }

    let n = PROBES.len() as u32;
    println!("{}", "-".repeat(55 + 14 * 3));
    println!(
        "{:55}  {}  {}  {}",
        "average",
        human(totals[0] / n),
        human(totals[1] / n),
        human(totals[2] / n)
    );
}
