//! Default filesystem discovery for local WoW resources.

use std::path::{Path, PathBuf};

use crate::saved_variables::WtfConfig;

const DEFAULT_ACCOUNT: &str = "50868465#2";
const DEFAULT_REALM: &str = "Burning Blade";
const DEFAULT_CHARACTER: &str = "Haky";
const ENV_WTF_ACCOUNT: &str = "WOW_SIM_WTF_ACCOUNT";
const ENV_WTF_REALM: &str = "WOW_SIM_WTF_REALM";
const ENV_WTF_CHARACTER: &str = "WOW_SIM_WTF_CHARACTER";
const ENV_INCLUDE_BETA_ADDONS: &str = "WOW_SIM_INCLUDE_BETA_ADDONS";

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

pub fn default_addons_paths() -> Vec<PathBuf> {
    let install_root = first_existing_path(wow_install_roots());
    existing_unique_paths(addons_path_candidates(install_root.as_ref()))
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

fn resolve_blizzard_ui_addons_path(root: &Path) -> crate::Result<PathBuf> {
    let profile = crate::client_profile::ACTIVE;
    let profile_addons = root
        .join("Interface/BlizzardUI")
        .join(profile.subdir())
        .join("AddOns");
    if profile_addons.exists() {
        return Ok(profile_addons);
    }

    let vendor_path = root
        .join("vendor")
        .join(format!("wow-ui-source-{}", profile.subdir().to_lowercase()))
        .join("Interface/AddOns");
    if vendor_path.exists() {
        return Ok(vendor_path);
    }

    Err(crate::Error::Other(format!(
        "missing Blizzard UI addon tree for {:?} profile; expected {} or {}",
        profile,
        profile_addons.display(),
        vendor_path.display()
    )))
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
    paths.extend(bundled_addons_path_candidates());

    for root in install_roots_for_candidates(install_root) {
        paths.extend(addon_paths_for_install_root(&root));
    }

    paths
}

fn bundled_addons_path_candidates() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(exe_path) = std::env::current_exe() {
        paths.extend(bundled_addons_path_candidates_for_exe(&exe_path));
    }
    paths.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/AddOns"));
    paths
}

fn bundled_addons_path_candidates_for_exe(exe_path: &Path) -> Vec<PathBuf> {
    let Some(exe_dir) = exe_path.parent() else {
        return Vec::new();
    };

    let mut paths = vec![exe_dir.join("Interface/AddOns")];
    if let Some(resources_dir) = macos_app_resources_dir(exe_dir) {
        paths.push(resources_dir.join("Interface/AddOns"));
    }

    paths
}

fn macos_app_resources_dir(exe_dir: &Path) -> Option<PathBuf> {
    let contents_dir = exe_dir.parent()?;
    if exe_dir.file_name()? != "MacOS" {
        return None;
    }
    if contents_dir.file_name()? != "Contents" {
        return None;
    }

    Some(contents_dir.join("Resources"))
}

fn addon_paths_for_install_root(root: &Path) -> Vec<PathBuf> {
    addon_paths_for_install_root_with_beta(root, include_beta_addons())
}

fn addon_paths_for_install_root_with_beta(root: &Path, include_beta: bool) -> Vec<PathBuf> {
    let mut paths = vec![root.join("_retail_/Interface/AddOns")];
    if include_beta {
        paths.push(root.join("_beta_/Interface/AddOns"));
    }
    paths
}

fn include_beta_addons() -> bool {
    std::env::var_os(ENV_INCLUDE_BETA_ADDONS).is_some_and(|value| {
        let value = value.to_string_lossy();
        value != "0" && !value.eq_ignore_ascii_case("false")
    })
}

fn wtf_path_candidates(install_root: Option<&PathBuf>) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    push_env_path(&mut paths, "WOW_SIM_WTF_PATH");

    for root in install_roots_for_candidates(install_root) {
        paths.push(root.join("_retail_/WTF"));
        paths.push(root.join("_beta_/WTF"));
        paths.push(root.join("_classic_/WTF"));
    }

    if !cfg!(windows) {
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
    } else {
        roots.push(PathBuf::from("/syncthing/World of Warcraft"));
    }

    roots
}

fn discover_wtf_identity(wtf_path: &std::path::Path) -> Option<(String, String, String)> {
    let account_root = wtf_path.join("Account");
    let account = preferred_child_dir(&account_root, &preferred_wtf_account())?;
    let account_path = account_root.join(&account);
    let realm = preferred_child_dir(&account_path, &preferred_wtf_realm())?;
    let character = preferred_child_dir(&account_path.join(&realm), &preferred_wtf_character())?;
    Some((account, realm, character))
}

fn default_wtf_identity(wtf_path: &std::path::Path) -> (String, String, String) {
    let account_root = wtf_path.join("Account");
    let account = preferred_wtf_account();
    let account = preferred_child_dir(&account_root, &account).unwrap_or(account);
    let account_path = account_root.join(&account);
    let realm = preferred_wtf_realm();
    let realm = preferred_child_dir(&account_path, &realm).unwrap_or(realm);
    let character = preferred_wtf_character();
    let character =
        preferred_child_dir(&account_path.join(&realm), &character).unwrap_or(character);
    (account, realm, character)
}

fn preferred_wtf_account() -> String {
    env_string(ENV_WTF_ACCOUNT).unwrap_or_else(|| DEFAULT_ACCOUNT.to_string())
}

fn preferred_wtf_realm() -> String {
    env_string(ENV_WTF_REALM).unwrap_or_else(|| DEFAULT_REALM.to_string())
}

fn preferred_wtf_character() -> String {
    env_string(ENV_WTF_CHARACTER).unwrap_or_else(|| DEFAULT_CHARACTER.to_string())
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
    paths
        .into_iter()
        .find_map(|path| path.exists().then(|| path.canonicalize().unwrap_or(path)))
}

fn existing_unique_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut unique = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for path in paths.into_iter().filter(|path| path.exists()) {
        let key = path.canonicalize().unwrap_or_else(|_| path.clone());
        if seen.insert(key) {
            unique.push(path);
        }
    }

    unique
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

fn env_string(var: &str) -> Option<String> {
    std::env::var_os(var)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use super::*;

    #[test]
    fn bundled_addons_candidates_include_macos_app_resources() {
        let exe_path = Path::new("/Applications/WoW Sim.app/Contents/MacOS/wow-sim");

        let candidates = bundled_addons_path_candidates_for_exe(exe_path);

        assert!(
            candidates.contains(&PathBuf::from(
                "/Applications/WoW Sim.app/Contents/Resources/Interface/AddOns"
            )),
            "{candidates:#?}"
        );
    }

    #[test]
    fn wtf_candidates_prefer_install_root_before_project_mirror_on_linux() {
        let install_root = PathBuf::from("/tmp/wow-install");

        let candidates = wtf_path_candidates(Some(&install_root));

        let retail_wtf = install_root.join("_retail_/WTF");
        let project_mirror = PathBuf::from("/syncthing/Sync/Projects/wow/WTF");
        let retail_index = candidates
            .iter()
            .position(|path| path == &retail_wtf)
            .expect("retail WTF should be a candidate");
        let project_index = candidates
            .iter()
            .position(|path| path == &project_mirror)
            .expect("project mirror should remain a fallback");
        assert!(
            retail_index < project_index,
            "live install WTF should win over stale project mirrors"
        );
    }

    #[test]
    fn addon_candidates_do_not_mix_beta_addons_into_retail_loads() {
        let install_root = PathBuf::from("/tmp/wow-install");

        let candidates = addon_paths_for_install_root_with_beta(&install_root, false);

        assert_eq!(
            candidates.as_slice(),
            &[install_root.join("_retail_/Interface/AddOns")]
        );
    }

    #[test]
    fn addon_candidates_can_include_beta_addons_when_requested() {
        let install_root = PathBuf::from("/tmp/wow-install");

        let candidates = addon_paths_for_install_root_with_beta(&install_root, true);

        assert_eq!(
            candidates.as_slice(),
            &[
                install_root.join("_retail_/Interface/AddOns"),
                install_root.join("_beta_/Interface/AddOns"),
            ]
        );
    }

    #[test]
    fn wtf_identity_can_target_palaky_from_environment() {
        let temp = tempfile::tempdir().expect("create temp dir");
        let palaky_path = temp.path().join("Account/50868465#2/Burning Blade/Palaky");
        std::fs::create_dir_all(&palaky_path).expect("create Palaky WTF path");

        let _guard = EnvVarGuard::set("WOW_SIM_WTF_CHARACTER", "Palaky");
        let (_account, _realm, character) = default_wtf_identity(temp.path());

        assert_eq!(character, "Palaky");
    }

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<std::ffi::OsString>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = std::env::var_os(key);
            unsafe {
                std::env::set_var(key, value);
            }
            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.previous {
                    Some(value) => std::env::set_var(self.key, value),
                    None => std::env::remove_var(self.key),
                }
            }
        }
    }

    #[test]
    fn blizzard_ui_path_prefers_repo_symlink() {
        let root = tempfile::tempdir().expect("tempdir");
        let profile_subdir = crate::client_profile::ACTIVE.subdir();
        let ui = root
            .path()
            .join("Interface/BlizzardUI")
            .join(profile_subdir)
            .join("AddOns");
        let vendor = root
            .path()
            .join(format!(
                "vendor/wow-ui-source-{}",
                profile_subdir.to_lowercase()
            ))
            .join("Interface/AddOns");
        fs::create_dir_all(&ui).expect("create ui dir");
        fs::create_dir_all(&vendor).expect("create vendor dir");

        let resolved = resolve_blizzard_ui_addons_path(root.path()).expect("resolve path");
        assert_eq!(resolved, ui);
    }

    #[test]
    fn blizzard_ui_path_falls_back_to_vendor_tree() {
        let root = tempfile::tempdir().expect("tempdir");
        let profile_subdir = crate::client_profile::ACTIVE.subdir();
        let vendor = root
            .path()
            .join(format!(
                "vendor/wow-ui-source-{}",
                profile_subdir.to_lowercase()
            ))
            .join("Interface/AddOns");
        fs::create_dir_all(&vendor).expect("create vendor dir");

        let resolved = resolve_blizzard_ui_addons_path(root.path()).expect("resolve path");
        assert_eq!(resolved, vendor);
    }
}
