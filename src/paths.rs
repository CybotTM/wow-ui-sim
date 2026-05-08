//! Default filesystem discovery for local WoW resources.

use std::path::{Path, PathBuf};

use crate::saved_variables::WtfConfig;

const DEFAULT_ACCOUNT: &str = "50868465#2";
const DEFAULT_REALM: &str = "Burning Blade";
const DEFAULT_CHARACTER: &str = "Haky";

#[derive(Debug, Clone)]
pub struct WowResourcePaths {
    pub install_root: Option<PathBuf>,
    pub casc_data_path: Option<PathBuf>,
    pub interface_path: PathBuf,
    pub addons_path: PathBuf,
    pub wtf_path: Option<PathBuf>,
}

pub fn discover_wow_resources() -> WowResourcePaths {
    let install_root = first_existing_path(wow_install_roots());
    WowResourcePaths {
        casc_data_path: default_casc_data_path_for_root(install_root.as_ref()),
        interface_path: default_interface_path_for_root(install_root.as_ref()),
        addons_path: default_addons_path_for_root(install_root.as_ref()),
        wtf_path: default_wtf_path_for_root(install_root.as_ref()),
        install_root,
    }
}

pub fn default_textures_path() -> PathBuf {
    if let Some(path) = env_path("WOW_SIM_TEXTURES_PATH") {
        return path;
    }
    PathBuf::from("./textures")
}

/// Find a directory entry case-insensitively.
pub fn find_case_insensitive(dir: &Path, name: &str) -> Option<PathBuf> {
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

pub fn default_casc_data_path() -> Option<PathBuf> {
    discover_wow_resources().casc_data_path
}

pub fn default_interface_path() -> PathBuf {
    discover_wow_resources().interface_path
}

pub fn default_addons_path() -> PathBuf {
    discover_wow_resources().addons_path
}

/// Blizzard UI addon root used by local benchmarks and dev binaries.
pub fn default_blizzard_ui_addons_path() -> crate::Result<PathBuf> {
    if let Some(cache_path) = crate::blizzard_ui_sync::cached_blizzard_ui_addons_path() {
        return Ok(cache_path);
    }
    Err(crate::Error::Other(format!(
        "missing Blizzard UI cache at {}; run `wow-cli casc sync-blizzard-ui`",
        blizzard_ui_cache_path_label()
    )))
}

pub fn default_wtf_config() -> Option<WtfConfig> {
    let wtf_path = discover_wow_resources().wtf_path?;
    let (account, realm, character) =
        discover_wtf_identity(&wtf_path).unwrap_or_else(|| default_wtf_identity(&wtf_path));
    Some(WtfConfig::new(wtf_path, &account, &realm, &character))
}

pub fn default_wtf_path() -> Option<PathBuf> {
    discover_wow_resources().wtf_path
}

fn default_casc_data_path_for_root(install_root: Option<&PathBuf>) -> Option<PathBuf> {
    if let Some(path) = env_path("WOW_SIM_CASC_PATH") {
        return Some(path);
    }

    install_root
        .map(|root| root.join("Data"))
        .filter(|path| path.exists())
}

fn default_interface_path_for_root(install_root: Option<&PathBuf>) -> PathBuf {
    first_existing_path(interface_path_candidates(install_root)).unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_default()
            .join("Projects/wow/Interface")
    })
}

fn default_addons_path_for_root(install_root: Option<&PathBuf>) -> PathBuf {
    first_existing_path(addons_path_candidates(install_root))
        .unwrap_or_else(|| PathBuf::from("./Interface/AddOns"))
}

fn default_wtf_path_for_root(install_root: Option<&PathBuf>) -> Option<PathBuf> {
    first_existing_path(wtf_path_candidates(install_root))
}

fn blizzard_ui_cache_path_label() -> String {
    crate::blizzard_ui_sync::default_cache_addons_path()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "~/.cache/wow-ui-sim/blizzard-ui".to_string())
}

fn interface_path_candidates(install_root: Option<&PathBuf>) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    push_env_path(&mut paths, "WOW_SIM_INTERFACE_PATH");

    if let Some(home) = dirs::home_dir() {
        paths.push(home.join("Projects/wow/Interface"));
    }

    for root in install_roots_for_candidates(install_root) {
        paths.push(root.join("_retail_/BlizzardInterfaceArt/Interface"));
        paths.push(root.join("_retail_/Interface"));
        paths.push(root.join("_beta_/BlizzardInterfaceArt/Interface"));
        paths.push(root.join("_beta_/Interface"));
    }

    paths
}

fn addons_path_candidates(install_root: Option<&PathBuf>) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    push_env_path(&mut paths, "WOW_SIM_ADDONS_PATH");
    paths.push(PathBuf::from("./Interface/AddOns"));

    for root in install_roots_for_candidates(install_root) {
        paths.extend(addon_paths_for_install_root(&root));
    }

    paths
}

fn addon_paths_for_install_root(root: &Path) -> [PathBuf; 3] {
    [
        root.join("_retail_/Interface/AddOns"),
        root.join("_beta_/Interface/AddOns"),
        root.join("_classic_/Interface/AddOns"),
    ]
}

fn wtf_path_candidates(install_root: Option<&PathBuf>) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    push_env_path(&mut paths, "WOW_SIM_WTF_PATH");

    if cfg!(windows) {
        for root in install_roots_for_candidates(install_root) {
            paths.push(root.join("_retail_/WTF"));
            paths.push(root.join("_beta_/WTF"));
            paths.push(root.join("_classic_/WTF"));
        }
    } else {
        paths.push(PathBuf::from("/syncthing/Sync/Projects/wow/WTF"));
        if let Some(home) = dirs::home_dir() {
            paths.push(home.join("Projects/wow/WTF"));
        }
    }

    paths
}

fn install_roots_for_candidates(install_root: Option<&PathBuf>) -> Vec<PathBuf> {
    install_root
        .cloned()
        .into_iter()
        .chain(wow_install_roots())
        .collect()
}

fn wow_install_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    push_env_path(&mut roots, "WOW_SIM_WOW_PATH");

    if cfg!(windows) {
        roots.push(PathBuf::from(r"C:\World of Warcraft"));
        roots.push(PathBuf::from(r"C:\Program Files (x86)\World of Warcraft"));
        roots.push(PathBuf::from(r"C:\Program Files\World of Warcraft"));
    }

    roots
}

fn discover_wtf_identity(wtf_path: &std::path::Path) -> Option<(String, String, String)> {
    let account_root = wtf_path.join("Account");
    let account = preferred_child_dir(&account_root, DEFAULT_ACCOUNT)?;
    let account_path = account_root.join(&account);
    let realm = preferred_child_dir(&account_path, DEFAULT_REALM)?;
    let character = preferred_child_dir(&account_path.join(&realm), DEFAULT_CHARACTER)?;
    Some((account, realm, character))
}

fn default_wtf_identity(wtf_path: &std::path::Path) -> (String, String, String) {
    let account_root = wtf_path.join("Account");
    let account = preferred_child_dir(&account_root, DEFAULT_ACCOUNT)
        .unwrap_or_else(|| DEFAULT_ACCOUNT.to_string());
    let account_path = account_root.join(&account);
    let realm = preferred_child_dir(&account_path, DEFAULT_REALM)
        .unwrap_or_else(|| DEFAULT_REALM.to_string());
    let character = preferred_child_dir(&account_path.join(&realm), DEFAULT_CHARACTER)
        .unwrap_or_else(|| DEFAULT_CHARACTER.to_string());
    (account, realm, character)
}

fn preferred_child_dir(parent: &std::path::Path, preferred: &str) -> Option<String> {
    let preferred_path = parent.join(preferred);
    if preferred_path.is_dir() {
        return Some(preferred.to_string());
    }

    std::fs::read_dir(parent)
        .ok()?
        .flatten()
        .find(|entry| entry.path().is_dir() && entry.file_name() != "SavedVariables")
        .map(|entry| entry.file_name().to_string_lossy().to_string())
}

fn first_existing_path(paths: Vec<PathBuf>) -> Option<PathBuf> {
    paths.into_iter().find(|path| path.exists())
}

fn push_env_path(paths: &mut Vec<PathBuf>, var: &str) {
    if let Some(path) = env_path(var) {
        paths.push(path);
    }
}

fn env_path(var: &str) -> Option<PathBuf> {
    std::env::var_os(var)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}
