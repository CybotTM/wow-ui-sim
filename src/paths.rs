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
///
/// Prefer the repo symlink at `Interface/BlizzardUI`, but fall back to the
/// vendored addon tree when the symlink is missing.
pub fn default_blizzard_ui_addons_path() -> crate::Result<PathBuf> {
    if let Some(path) = env_path("WOW_SIM_BLIZZARD_UI_PATH") {
        if path.is_dir() {
            return Ok(path);
        }
        return Err(crate::Error::Other(format!(
            "WOW_SIM_BLIZZARD_UI_PATH does not exist or is not a directory: {}",
            path.display()
        )));
    }

    let mut first_error = None;
    for root in runtime_roots() {
        match resolve_blizzard_ui_addons_path(&root) {
            Ok(path) => return Ok(path),
            Err(err) => {
                if first_error.is_none() {
                    first_error = Some(err);
                }
            }
        }
    }
    Err(first_error.unwrap_or_else(|| {
        crate::Error::Other(
            "missing Blizzard UI addon tree; run ./scripts/setup-blizzard-ui.sh from the release or repo root.".to_string(),
        )
    }))
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

fn resolve_blizzard_ui_addons_path(root: &Path) -> crate::Result<PathBuf> {
    let symlink_path = root.join("Interface/BlizzardUI");
    if symlink_path.exists() {
        return Ok(symlink_path);
    }

    if symlink_exists(&symlink_path) {
        std::fs::remove_file(&symlink_path).map_err(|e| {
            crate::Error::Other(format!(
                "broken Blizzard UI symlink exists at {} but could not be removed: {e}",
                symlink_path.display()
            ))
        })?;
    }

    if let Some(vendor_path) = find_vendor_blizzard_addons(root) {
        if let Some(parent) = symlink_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                crate::Error::Other(format!(
                    "missing Blizzard UI symlink parent {} and could not create it: {e}",
                    parent.display()
                ))
            })?;
        }
        if let Err(e) = create_blizzard_ui_symlink(&vendor_path, &symlink_path) {
            return Err(crate::Error::Other(format!(
                "missing Blizzard UI symlink at {} and could not link it to {}: {e}",
                symlink_path.display(),
                vendor_path.display()
            )));
        }
        return Ok(symlink_path);
    }

    Err(crate::Error::Other(format!(
        "missing Blizzard UI addon tree; expected {} or an ancestor vendor/wow-ui-source/Interface/AddOns. Run ./scripts/setup-blizzard-ui.sh from the repo root.",
        symlink_path.display()
    )))
}

fn symlink_exists(path: &Path) -> bool {
    std::fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_symlink())
}

fn find_vendor_blizzard_addons(root: &Path) -> Option<PathBuf> {
    root.canonicalize()
        .ok()?
        .ancestors()
        .map(|ancestor| ancestor.join("vendor/wow-ui-source/Interface/AddOns"))
        .find(|candidate| candidate.is_dir())
}

#[cfg(unix)]
fn create_blizzard_ui_symlink(target: &Path, symlink_path: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, symlink_path)
}

#[cfg(windows)]
fn create_blizzard_ui_symlink(target: &Path, symlink_path: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, symlink_path)
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
        paths.push(root.join("_retail_/Interface/AddOns"));
        paths.push(root.join("_beta_/Interface/AddOns"));
        paths.push(root.join("_classic_/Interface/AddOns"));
    }

    paths
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

fn runtime_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd);
    }
    if let Ok(exe) = std::env::current_exe()
        && let Some(parent) = exe.parent()
    {
        roots.push(parent.to_path_buf());
    }
    roots.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")));
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

#[cfg(test)]
mod tests {
    use super::resolve_blizzard_ui_addons_path;
    use std::fs;

    #[test]
    fn blizzard_ui_path_prefers_repo_symlink() {
        let root = tempfile::tempdir().expect("tempdir");
        let ui = root.path().join("Interface/BlizzardUI");
        let vendor = root.path().join("vendor/wow-ui-source/Interface/AddOns");
        fs::create_dir_all(&ui).expect("create ui dir");
        fs::create_dir_all(&vendor).expect("create vendor dir");

        let resolved = resolve_blizzard_ui_addons_path(root.path()).expect("resolve path");
        assert_eq!(resolved, ui);
    }

    #[cfg(unix)]
    #[test]
    fn blizzard_ui_path_links_to_vendor_tree() {
        let root = tempfile::tempdir().expect("tempdir");
        let ui = root.path().join("Interface/BlizzardUI");
        let vendor = root.path().join("vendor/wow-ui-source/Interface/AddOns");
        fs::create_dir_all(root.path().join("Interface")).expect("create interface dir");
        fs::create_dir_all(&vendor).expect("create vendor dir");

        let resolved = resolve_blizzard_ui_addons_path(root.path()).expect("resolve path");
        assert_eq!(resolved, ui);
        assert_eq!(fs::read_link(&resolved).expect("read symlink"), vendor);
    }

    #[cfg(unix)]
    #[test]
    fn blizzard_ui_path_links_to_ancestor_vendor_tree() {
        let root = tempfile::tempdir().expect("tempdir");
        let repo = root.path().join("repo");
        let worktree = repo.join(".claude/worktrees/example");
        let ui = worktree.join("Interface/BlizzardUI");
        let vendor = repo.join("vendor/wow-ui-source/Interface/AddOns");
        fs::create_dir_all(worktree.join("Interface")).expect("create interface dir");
        fs::create_dir_all(&vendor).expect("create vendor dir");

        let resolved = resolve_blizzard_ui_addons_path(&worktree).expect("resolve path");
        assert_eq!(resolved, ui);
        assert_eq!(fs::read_link(&resolved).expect("read symlink"), vendor);
    }
}
