//! CASC-backed Blizzard UI source synchronization.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const BLIZZARD_UI_MANIFEST: &str = include_str!("../data/blizzard-ui-files.txt");
const COMPLETE_MARKER: &str = ".wow-ui-sim-blizzard-ui-complete";
static CASC_CONFIGURED: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncSummary {
    pub root: PathBuf,
    pub total: usize,
    pub extracted: usize,
    pub present: usize,
    pub missing: usize,
}

pub fn default_cache_addons_path() -> crate::Result<PathBuf> {
    dirs::cache_dir()
        .map(|dir| dir.join("wow-ui-sim/blizzard-ui"))
        .ok_or_else(|| crate::Error::Other("could not determine user cache directory".to_string()))
}

pub fn cached_blizzard_ui_addons_path() -> Option<PathBuf> {
    let path = default_cache_addons_path().ok()?;
    path.join(COMPLETE_MARKER).is_file().then_some(path)
}

pub fn sync_blizzard_ui() -> crate::Result<SyncSummary> {
    let root = default_cache_addons_path()?;
    sync_blizzard_ui_to(&root)
}

pub fn sync_blizzard_ui_to(root: &Path) -> crate::Result<SyncSummary> {
    sync_blizzard_ui_entries(root, manifest_entries())
}

pub fn manifest_entries() -> impl Iterator<Item = &'static str> {
    BLIZZARD_UI_MANIFEST
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
}

fn sync_blizzard_ui_entries<'a>(
    root: &Path,
    entries: impl Iterator<Item = &'a str>,
) -> crate::Result<SyncSummary> {
    let mut summary = SyncSummary {
        root: root.to_path_buf(),
        total: 0,
        extracted: 0,
        present: 0,
        missing: 0,
    };

    for entry in entries {
        summary.total += 1;
        let Some(fdid) = manifest_entry_fdid(entry) else {
            summary.missing += 1;
            continue;
        };
        let out_path = root.join(entry);
        if out_path.is_file() {
            summary.present += 1;
            continue;
        }
        match extract_fdid(entry, fdid, &out_path)? {
            true => summary.extracted += 1,
            false => summary.missing += 1,
        }
    }

    if summary.missing > 0 {
        return Err(crate::Error::Other(format!(
            "failed to sync {} of {} Blizzard UI files from CASC into {}",
            summary.missing,
            summary.total,
            root.display()
        )));
    }

    write_complete_marker(root)?;
    Ok(summary)
}

fn manifest_entry_fdid(entry: &str) -> Option<u32> {
    let asset_path = format!("interface/addons/{}", entry.replace('\\', "/"));
    crate::limited_listfile::lookup_path(&asset_path)
}

#[cfg(feature = "casc")]
fn extract_fdid(entry: &str, fdid: u32, out_path: &Path) -> crate::Result<bool> {
    if !casc_available() {
        return Err(crate::Error::Other(
            "local WoW CASC data is not available; set WOW_INSTALL_PATH or WOW_DATA_PATH, and make sure WOW_SIM_CASC is not 0".to_string(),
        ));
    }
    remove_missing_marker(out_path);
    let resolver = asset_resolver::CascListfileResolver;
    if resolver.ensure_cached(fdid, out_path).is_some() {
        return Ok(true);
    }
    copy_from_installed_interface_code(entry, out_path)
}

#[cfg(not(feature = "casc"))]
fn extract_fdid(_entry: &str, _fdid: u32, _out_path: &Path) -> crate::Result<bool> {
    Err(crate::Error::Other(
        "Blizzard UI CASC sync requires the `casc` feature".to_string(),
    ))
}

fn write_complete_marker(root: &Path) -> crate::Result<()> {
    std::fs::create_dir_all(root).map_err(|e| {
        crate::Error::Other(format!(
            "could not create Blizzard UI cache directory {}: {e}",
            root.display()
        ))
    })?;
    std::fs::write(root.join(COMPLETE_MARKER), b"ok\n").map_err(|e| {
        crate::Error::Other(format!(
            "could not write Blizzard UI cache marker in {}: {e}",
            root.display()
        ))
    })
}

#[cfg(feature = "casc")]
fn casc_available() -> bool {
    *CASC_CONFIGURED.get_or_init(|| {
        if std::env::var("WOW_SIM_CASC").ok().as_deref() == Some("0") {
            return false;
        }
        configure_default_asset_resolver_root();
        asset_resolver::wow_install_path().is_some()
    })
}

#[cfg(feature = "casc")]
fn configure_default_asset_resolver_root() {
    if std::env::var_os("GAME_ENGINE_SHARED_ROOT").is_some() {
        return;
    }
    let Some(home) = dirs::home_dir() else {
        return;
    };
    let default_root = home.join("Projects/world-of-osso/game-engine");
    if default_root.exists() {
        // SAFETY: this runs behind a OnceLock before asset-resolver is initialized here.
        unsafe {
            std::env::set_var("GAME_ENGINE_SHARED_ROOT", default_root);
        }
    }
}

fn remove_missing_marker(path: &Path) {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let missing_marker = path.with_extension(format!("{extension}.missing"));
    if missing_marker.is_file() {
        let _ = std::fs::remove_file(missing_marker);
    }
}

fn copy_from_installed_interface_code(entry: &str, out_path: &Path) -> crate::Result<bool> {
    let Some(source_path) = installed_interface_code_path(entry) else {
        return Ok(false);
    };
    let Some(parent) = out_path.parent() else {
        return Ok(false);
    };
    std::fs::create_dir_all(parent).map_err(|e| {
        crate::Error::Other(format!(
            "could not create Blizzard UI cache directory {}: {e}",
            parent.display()
        ))
    })?;
    std::fs::copy(&source_path, out_path).map_err(|e| {
        crate::Error::Other(format!(
            "could not copy Blizzard UI fallback {} to {}: {e}",
            source_path.display(),
            out_path.display()
        ))
    })?;
    remove_missing_marker(out_path);
    Ok(true)
}

fn installed_interface_code_path(entry: &str) -> Option<PathBuf> {
    installed_interface_code_roots()
        .into_iter()
        .map(|root| {
            root.join("_retail_/BlizzardInterfaceCode/Interface/AddOns")
                .join(entry)
        })
        .find(|path| path.is_file())
}

fn installed_interface_code_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(root) = crate::paths::discover_wow_resources().install_root {
        roots.push(root);
    }
    push_env_path(&mut roots, "WOW_INSTALL_PATH");
    push_data_parent_env_path(&mut roots, "WOW_DATA_PATH");
    push_data_parent_env_path(&mut roots, "WOW_SIM_CASC_PATH");
    push_asset_resolver_install_root(&mut roots);
    roots
}

fn push_env_path(roots: &mut Vec<PathBuf>, name: &str) {
    if let Some(path) = std::env::var_os(name).map(PathBuf::from) {
        roots.push(path);
    }
}

fn push_data_parent_env_path(roots: &mut Vec<PathBuf>, name: &str) {
    let Some(data_path) = std::env::var_os(name).map(PathBuf::from) else {
        return;
    };
    if is_data_dir(&data_path) {
        if let Some(parent) = data_path.parent() {
            roots.push(parent.to_path_buf());
            return;
        }
    }
    roots.push(data_path);
}

#[cfg(feature = "casc")]
fn push_asset_resolver_install_root(roots: &mut Vec<PathBuf>) {
    if let Some(data_path) = asset_resolver::wow_install_path() {
        if is_data_dir(&data_path) {
            if let Some(parent) = data_path.parent() {
                roots.push(parent.to_path_buf());
                return;
            }
        }
        roots.push(data_path.to_path_buf());
    }
}

#[cfg(not(feature = "casc"))]
fn push_asset_resolver_install_root(_roots: &mut Vec<PathBuf>) {}

fn is_data_dir(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|name| name == OsStr::new("Data"))
}

#[cfg(test)]
mod tests {
    use super::{manifest_entries, manifest_entry_fdid};

    #[test]
    fn manifest_preserves_blizzard_addon_case() {
        let first = manifest_entries()
            .next()
            .expect("manifest should not be empty");
        assert!(first.starts_with("Blizzard_"));
    }

    #[test]
    fn manifest_entries_resolve_through_limited_listfile() {
        let missing: Vec<_> = manifest_entries()
            .filter(|entry| manifest_entry_fdid(entry).is_none())
            .take(10)
            .collect();
        assert!(
            missing.is_empty(),
            "unmapped Blizzard UI files: {missing:?}"
        );
    }
}
