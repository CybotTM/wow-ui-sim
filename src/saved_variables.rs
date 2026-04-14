//! Saved variables manager placeholder during the rilua migration.

use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration for loading WTF saved variables from a real WoW installation.
#[derive(Debug, Clone)]
pub struct WtfConfig {
    pub wtf_path: PathBuf,
    pub account: String,
    pub realm: String,
    pub character: String,
}

impl WtfConfig {
    pub fn new(wtf_path: impl Into<PathBuf>, account: &str, realm: &str, character: &str) -> Self {
        Self {
            wtf_path: wtf_path.into(),
            account: account.to_string(),
            realm: realm.to_string(),
            character: character.to_string(),
        }
    }

    pub fn account_saved_vars_path(&self) -> PathBuf {
        self.wtf_path
            .join("Account")
            .join(&self.account)
            .join("SavedVariables")
    }

    pub fn character_saved_vars_path(&self) -> PathBuf {
        self.wtf_path
            .join("Account")
            .join(&self.account)
            .join(&self.realm)
            .join(&self.character)
            .join("SavedVariables")
    }

    pub fn account_saved_vars_file(&self, addon_name: &str) -> PathBuf {
        self.account_saved_vars_path()
            .join(format!("{}.lua", addon_name))
    }

    pub fn character_saved_vars_file(&self, addon_name: &str) -> PathBuf {
        self.character_saved_vars_path()
            .join(format!("{}.lua", addon_name))
    }
}

/// Temporary no-op manager that preserves the public API while saved-variable
/// loading is still mlua-era code.
#[derive(Debug)]
pub struct SavedVariablesManager {
    storage_dir: PathBuf,
    character_name: String,
    realm_name: String,
    registered: HashMap<String, Vec<String>>,
    registered_per_char: HashMap<String, Vec<String>>,
    wtf_config: Option<WtfConfig>,
    wtf_loaded: HashMap<String, bool>,
}

impl SavedVariablesManager {
    pub fn new() -> Self {
        let storage_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wow-sim")
            .join("SavedVariables");
        Self::with_storage_dir(storage_dir)
    }

    pub fn with_storage_dir(storage_dir: PathBuf) -> Self {
        Self {
            storage_dir,
            character_name: "SimPlayer".to_string(),
            realm_name: "SimRealm".to_string(),
            registered: HashMap::new(),
            registered_per_char: HashMap::new(),
            wtf_config: None,
            wtf_loaded: HashMap::new(),
        }
    }

    pub fn set_character(&mut self, name: &str, realm: &str) {
        self.character_name = name.to_string();
        self.realm_name = realm.to_string();
    }

    pub fn set_wtf_config(&mut self, config: WtfConfig) {
        self.character_name = config.character.clone();
        self.realm_name = config.realm.clone();
        self.wtf_config = Some(config);
    }

    pub fn wtf_config(&self) -> Option<&WtfConfig> {
        self.wtf_config.as_ref()
    }

    pub fn load_wtf_for_addon<T>(&mut self, _lua: &T, addon_name: &str) -> crate::Result<usize> {
        self.wtf_loaded.insert(addon_name.to_string(), false);
        Ok(0)
    }

    pub fn init_for_addon<T>(
        &mut self,
        _lua: &T,
        addon_name: &str,
        saved_vars: &[String],
        saved_vars_per_char: &[String],
    ) -> crate::Result<()> {
        if !saved_vars.is_empty() {
            self.registered
                .insert(addon_name.to_string(), saved_vars.to_vec());
        }
        if !saved_vars_per_char.is_empty() {
            self.registered_per_char
                .insert(addon_name.to_string(), saved_vars_per_char.to_vec());
        }
        let _ = &self.storage_dir;
        Ok(())
    }

    pub fn save_addon<T>(&self, _lua: &T, _addon_name: &str) -> crate::Result<()> {
        Ok(())
    }

    pub fn save_all<T>(&self, _lua: &T) -> crate::Result<()> {
        Ok(())
    }

    pub fn registered_addons(&self) -> Vec<&String> {
        self.registered
            .keys()
            .chain(self.registered_per_char.keys())
            .collect()
    }
}
