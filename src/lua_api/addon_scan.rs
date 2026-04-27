//! Scan addon directories and build AddonInfo entries from TOC files.

use super::state::AddonInfo;
use crate::toc::TocFile;

fn addon_info_from_toc(name: &str, toc: Option<&TocFile>) -> AddonInfo {
    let (title, notes, load_on_demand, use_secure_env, dependencies) = toc
        .map(|t| {
            let title = t
                .metadata
                .get("Title")
                .cloned()
                .unwrap_or_else(|| name.to_string());
            let notes = t.metadata.get("Notes").cloned().unwrap_or_default();
            let lod = t
                .metadata
                .get("LoadOnDemand")
                .map(|v| v == "1")
                .unwrap_or(false);
            (title, notes, lod, t.is_secure_env(), t.dependencies())
        })
        .unwrap_or_else(|| (name.to_string(), String::new(), false, false, Vec::new()));
    AddonInfo {
        folder_name: name.to_string(),
        title,
        notes,
        enabled: true,
        load_on_demand,
        use_secure_env,
        dependencies,
        ..Default::default()
    }
}

/// Scan an addons directory and return AddonInfo for each valid addon folder.
pub(crate) fn scan_addon_entries(addons_path: &std::path::Path) -> Vec<AddonInfo> {
    let Ok(entries) = std::fs::read_dir(addons_path) else {
        return Vec::new();
    };
    let mut addons = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        if name.starts_with('.') || name == "BlizzardUI" {
            continue;
        }
        let Some(toc_path) = crate::loader::find_toc_file(&path) else {
            continue;
        };
        let toc = TocFile::from_file(&toc_path).ok();
        addons.push(addon_info_from_toc(&name, toc.as_ref()));
    }
    addons
}
