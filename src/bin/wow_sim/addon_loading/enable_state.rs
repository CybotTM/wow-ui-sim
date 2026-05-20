use std::collections::HashMap;
use std::path::PathBuf;

use wow_ui_sim::addon_enable_state::{local_addons_txt_path, read_addon_enable_overrides};
use wow_ui_sim::saved_variables::SavedVariablesManager;
use wow_ui_sim::toc::TocFile;

pub(super) fn addon_enabled(
    name: &str,
    metadata: &super::AddonMetadata,
    enable_overrides: Option<&HashMap<String, bool>>,
) -> bool {
    match enable_overrides {
        Some(overrides) => overrides
            .get(name)
            .copied()
            .unwrap_or(metadata.default_enabled),
        None => metadata.default_enabled,
    }
}

pub(super) fn addon_enable_overrides(
    saved_vars: Option<&SavedVariablesManager>,
) -> Option<HashMap<String, bool>> {
    if let Ok(overrides) = read_addon_enable_overrides(&local_addons_txt_path()) {
        return Some(overrides);
    }

    let config = saved_vars?.wtf_config()?;
    let path = config
        .wtf_path
        .join("Account")
        .join(&config.account)
        .join(&config.realm)
        .join(&config.character)
        .join("AddOns.txt");
    read_addon_enable_overrides(&path).ok()
}

pub(super) fn dependency_aware_enable_overrides(
    addons: &[(String, PathBuf)],
    enable_overrides: Option<&HashMap<String, bool>>,
) -> Option<HashMap<String, bool>> {
    let overrides = enable_overrides?;
    let addon_tocs = addon_toc_map(addons);
    let mut effective = overrides.clone();

    loop {
        let newly_enabled = collect_new_dependency_enables(&addon_tocs, overrides, &effective);
        let newly_disabled = collect_new_dependency_disables(&addon_tocs, &effective);
        if newly_enabled.is_empty() && newly_disabled.is_empty() {
            return Some(effective);
        }
        for name in newly_enabled {
            effective.insert(name, true);
        }
        for name in newly_disabled {
            effective.insert(name, false);
        }
    }
}

fn addon_toc_map(addons: &[(String, PathBuf)]) -> HashMap<String, TocFile> {
    addons
        .iter()
        .filter_map(|(name, path)| TocFile::from_file(path).ok().map(|toc| (name.clone(), toc)))
        .collect()
}

fn collect_new_dependency_enables(
    addon_tocs: &HashMap<String, TocFile>,
    character_overrides: &HashMap<String, bool>,
    effective: &HashMap<String, bool>,
) -> Vec<String> {
    let mut newly_enabled = Vec::new();
    for (name, toc) in addon_tocs {
        if !effective.get(name).copied().unwrap_or(false) {
            continue;
        }
        for dependency in toc.dependencies() {
            if !addon_tocs.contains_key(&dependency) {
                continue;
            }
            if character_overrides.get(&dependency) == Some(&false) {
                continue;
            }
            if !effective.get(&dependency).copied().unwrap_or(false) {
                newly_enabled.push(dependency);
            }
        }
    }
    newly_enabled
}

fn collect_new_dependency_disables(
    addon_tocs: &HashMap<String, TocFile>,
    effective: &HashMap<String, bool>,
) -> Vec<String> {
    let mut newly_disabled = Vec::new();
    for (name, toc) in addon_tocs {
        if !effective.get(name).copied().unwrap_or(false) {
            continue;
        }
        if toc
            .dependencies()
            .iter()
            .any(|dependency| effective.get(dependency) == Some(&false))
        {
            newly_disabled.push(name.clone());
        }
    }
    newly_disabled
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::sync::Mutex;

    static ADDONS_TXT_ENV_LOCK: Mutex<()> = Mutex::new(());

    struct ScopedAddonsTxtEnv {
        previous: Option<std::ffi::OsString>,
    }

    impl ScopedAddonsTxtEnv {
        fn set(path: &Path) -> Self {
            let previous = std::env::var_os("WOW_SIM_ADDONS_TXT");
            unsafe {
                std::env::set_var("WOW_SIM_ADDONS_TXT", path);
            }
            Self { previous }
        }
    }

    impl Drop for ScopedAddonsTxtEnv {
        fn drop(&mut self) {
            unsafe {
                match &self.previous {
                    Some(previous) => std::env::set_var("WOW_SIM_ADDONS_TXT", previous),
                    None => std::env::remove_var("WOW_SIM_ADDONS_TXT"),
                }
            }
        }
    }

    fn metadata(default_enabled: bool) -> super::super::AddonMetadata {
        super::super::AddonMetadata {
            title: "TestAddon".to_string(),
            notes: String::new(),
            metadata: HashMap::new(),
            load_on_demand: false,
            default_enabled,
            dependencies: Vec::new(),
            use_secure_env: false,
        }
    }

    fn write_addon_toc(root: &Path, name: &str, toc: &str) -> (String, PathBuf) {
        let addon_dir = root.join(name);
        std::fs::create_dir_all(&addon_dir).expect("create addon dir");
        let toc_path = addon_dir.join(format!("{name}.toc"));
        std::fs::write(&toc_path, toc).expect("write toc");
        (name.to_string(), toc_path)
    }

    #[test]
    fn addon_enabled_uses_character_addons_txt_before_toc_default() {
        let metadata = metadata(true);
        let overrides = HashMap::from([("DisabledByCharacter".to_string(), false)]);

        assert!(!addon_enabled(
            "DisabledByCharacter",
            &metadata,
            Some(&overrides)
        ));
        assert!(addon_enabled(
            "MissingFromAddOnsTxt",
            &metadata,
            Some(&overrides)
        ));
    }

    #[test]
    fn addon_enabled_preserves_disabled_toc_default_when_missing_from_addons_txt() {
        let metadata = metadata(false);
        let overrides = HashMap::new();

        assert!(!addon_enabled(
            "DefaultDisabledAddon",
            &metadata,
            Some(&overrides)
        ));
    }

    #[test]
    fn addon_enable_overrides_reads_character_addons_txt() {
        let _guard = ADDONS_TXT_ENV_LOCK.lock().expect("env lock");
        let temp = tempfile::tempdir().expect("tempdir");
        let _env = ScopedAddonsTxtEnv::set(&temp.path().join("missing-local-AddOns.txt"));
        let addon_state_dir = temp.path().join("Account/Test/Burning Blade/Palaky");
        std::fs::create_dir_all(&addon_state_dir).expect("create character dir");
        std::fs::write(
            addon_state_dir.join("AddOns.txt"),
            "EnabledAddon: enabled\nDisabledAddon: disabled\n",
        )
        .expect("write AddOns.txt");

        let mut saved_vars = SavedVariablesManager::new();
        saved_vars.set_wtf_config(wow_ui_sim::saved_variables::WtfConfig::new(
            temp.path(),
            "Test",
            "Burning Blade",
            "Palaky",
        ));

        let overrides = addon_enable_overrides(Some(&saved_vars)).expect("read overrides");

        assert_eq!(overrides.get("EnabledAddon"), Some(&true));
        assert_eq!(overrides.get("DisabledAddon"), Some(&false));
    }

    #[test]
    fn addon_enable_overrides_prefers_local_addons_txt() {
        let _guard = ADDONS_TXT_ENV_LOCK.lock().expect("env lock");
        let temp = tempfile::tempdir().expect("tempdir");
        let local_path = temp.path().join("local-AddOns.txt");
        let _env = ScopedAddonsTxtEnv::set(&local_path);
        std::fs::write(&local_path, "LocalDisabled: disabled\n").expect("write local AddOns.txt");

        let wtf_dir = temp.path().join("wtf");
        let addon_state_dir = wtf_dir.join("Account/Test/Burning Blade/Palaky");
        std::fs::create_dir_all(&addon_state_dir).expect("create character dir");
        std::fs::write(
            addon_state_dir.join("AddOns.txt"),
            "LocalDisabled: enabled\n",
        )
        .expect("write WTF AddOns.txt");
        let mut saved_vars = SavedVariablesManager::new();
        saved_vars.set_wtf_config(wow_ui_sim::saved_variables::WtfConfig::new(
            &wtf_dir,
            "Test",
            "Burning Blade",
            "Palaky",
        ));

        let overrides = addon_enable_overrides(Some(&saved_vars)).expect("read overrides");

        assert_eq!(overrides.get("LocalDisabled"), Some(&false));
    }

    #[test]
    fn dependency_aware_overrides_enable_required_deps() {
        let temp = tempfile::tempdir().expect("tempdir");
        let addons = vec![
            write_addon_toc(
                temp.path(),
                "ParentAddon",
                "## Interface: 120005\n## Dependencies: RequiredAddon\n",
            ),
            write_addon_toc(temp.path(), "RequiredAddon", "## Interface: 120005\n"),
        ];
        let overrides = HashMap::from([("ParentAddon".to_string(), true)]);

        let effective =
            dependency_aware_enable_overrides(&addons, Some(&overrides)).expect("effective map");

        assert_eq!(effective.get("ParentAddon"), Some(&true));
        assert_eq!(effective.get("RequiredAddon"), Some(&true));
    }

    #[test]
    fn dependency_aware_overrides_do_not_enable_absent_optional_deps() {
        let temp = tempfile::tempdir().expect("tempdir");
        let addons = vec![
            write_addon_toc(
                temp.path(),
                "ParentAddon",
                "## Interface: 120005\n## OptionalDeps: OptionalAddon\n",
            ),
            write_addon_toc(temp.path(), "OptionalAddon", "## Interface: 120005\n"),
        ];
        let overrides = HashMap::from([("ParentAddon".to_string(), true)]);

        let effective =
            dependency_aware_enable_overrides(&addons, Some(&overrides)).expect("effective map");

        assert_eq!(effective.get("ParentAddon"), Some(&true));
        assert_eq!(effective.get("OptionalAddon"), None);
    }

    #[test]
    fn dependency_aware_overrides_respect_explicit_disabled_optional_deps() {
        let temp = tempfile::tempdir().expect("tempdir");
        let addons = vec![
            write_addon_toc(
                temp.path(),
                "ParentAddon",
                "## Interface: 120005\n## OptionalDeps: OptionalAddon\n",
            ),
            write_addon_toc(temp.path(), "OptionalAddon", "## Interface: 120005\n"),
        ];
        let overrides = HashMap::from([
            ("ParentAddon".to_string(), true),
            ("OptionalAddon".to_string(), false),
        ]);

        let effective =
            dependency_aware_enable_overrides(&addons, Some(&overrides)).expect("effective map");

        assert_eq!(effective.get("ParentAddon"), Some(&true));
        assert_eq!(effective.get("OptionalAddon"), Some(&false));
    }

    #[test]
    fn dependency_aware_overrides_disable_addons_with_disabled_required_deps() {
        let temp = tempfile::tempdir().expect("tempdir");
        let addons = vec![
            write_addon_toc(
                temp.path(),
                "PluginAddon",
                "## Interface: 120005\n## Dependencies: RequiredAddon\n",
            ),
            write_addon_toc(temp.path(), "RequiredAddon", "## Interface: 120005\n"),
        ];
        let overrides = HashMap::from([
            ("PluginAddon".to_string(), true),
            ("RequiredAddon".to_string(), false),
        ]);

        let effective =
            dependency_aware_enable_overrides(&addons, Some(&overrides)).expect("effective map");

        assert_eq!(effective.get("RequiredAddon"), Some(&false));
        assert_eq!(effective.get("PluginAddon"), Some(&false));
    }
}
