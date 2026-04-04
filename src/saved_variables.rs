//! Saved variables management for addon persistence.
//!
//! WoW addons can declare SavedVariables and SavedVariablesPerCharacter in
//! their .toc files. These are global Lua tables that persist between sessions.
//!
//! Storage uses WoW-compatible Lua format (`VarName = { ... }`), so files can
//! be shared between the simulator and a real WoW installation.
//!
//! Loading priority:
//! 1. WTF directory (real WoW installation, if configured)
//! 2. Simulator storage (~/.local/share/wow-sim/SavedVariables/)

#[path = "saved_variables_serialize.rs"]
mod saved_variables_serialize;

use mlua::{Lua, Result, Table, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use saved_variables_serialize::serialize_assignment;

/// Configuration for loading WTF saved variables from a real WoW installation.
#[derive(Debug, Clone)]
pub struct WtfConfig {
    /// Base WTF directory path (e.g., /path/to/WoW/WTF)
    pub wtf_path: PathBuf,
    /// Account ID/name (e.g., "50868465#2")
    pub account: String,
    /// Realm name (e.g., "Burning Blade")
    pub realm: String,
    /// Character name (e.g., "Haky")
    pub character: String,
}

impl WtfConfig {
    /// Create a new WTF configuration.
    pub fn new(wtf_path: impl Into<PathBuf>, account: &str, realm: &str, character: &str) -> Self {
        Self {
            wtf_path: wtf_path.into(),
            account: account.to_string(),
            realm: realm.to_string(),
            character: character.to_string(),
        }
    }

    /// Get the path to account-level SavedVariables directory.
    pub fn account_saved_vars_path(&self) -> PathBuf {
        self.wtf_path
            .join("Account")
            .join(&self.account)
            .join("SavedVariables")
    }

    /// Get the path to character-level SavedVariables directory.
    pub fn character_saved_vars_path(&self) -> PathBuf {
        self.wtf_path
            .join("Account")
            .join(&self.account)
            .join(&self.realm)
            .join(&self.character)
            .join("SavedVariables")
    }

    /// Get the path to account-level SavedVariables file for an addon.
    pub fn account_saved_vars_file(&self, addon_name: &str) -> PathBuf {
        self.account_saved_vars_path()
            .join(format!("{}.lua", addon_name))
    }

    /// Get the path to character-level SavedVariables file for an addon.
    pub fn character_saved_vars_file(&self, addon_name: &str) -> PathBuf {
        self.character_saved_vars_path()
            .join(format!("{}.lua", addon_name))
    }
}

/// Manages saved variables for all loaded addons.
#[derive(Debug)]
pub struct SavedVariablesManager {
    /// Base directory for saved variables storage.
    storage_dir: PathBuf,
    /// Character name for per-character variables.
    character_name: String,
    /// Realm name for per-character variables.
    realm_name: String,
    /// Track which variables have been registered (addon_name -> var_names).
    registered: HashMap<String, Vec<String>>,
    /// Track per-character variables.
    registered_per_char: HashMap<String, Vec<String>>,
    /// Optional WTF configuration for loading real WoW saved variables.
    wtf_config: Option<WtfConfig>,
    /// Track which addons have had WTF variables loaded.
    wtf_loaded: HashMap<String, bool>,
}

impl SavedVariablesManager {
    /// Create a new manager with default storage location.
    pub fn new() -> Self {
        let storage_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wow-sim")
            .join("SavedVariables");
        Self::with_storage_dir(storage_dir)
    }

    /// Create with custom storage directory.
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

    /// Set character info for per-character variables.
    pub fn set_character(&mut self, name: &str, realm: &str) {
        self.character_name = name.to_string();
        self.realm_name = realm.to_string();
    }

    /// Set WTF configuration for loading real WoW saved variables.
    pub fn set_wtf_config(&mut self, config: WtfConfig) {
        self.character_name = config.character.clone();
        self.realm_name = config.realm.clone();
        self.wtf_config = Some(config);
    }

    /// Get a reference to the WTF configuration.
    pub fn wtf_config(&self) -> Option<&WtfConfig> {
        self.wtf_config.as_ref()
    }

    /// Load WTF saved variables for an addon from the real WoW installation.
    /// This executes the Lua files to set global variables.
    /// Returns the number of files loaded (0, 1, or 2 for account + character).
    pub fn load_wtf_for_addon(&mut self, lua: &Lua, addon_name: &str) -> Result<usize> {
        let Some(config) = self.wtf_config.clone() else {
            return Ok(0);
        };
        if self.wtf_loaded.contains_key(addon_name) {
            return Ok(0);
        }

        let files = [
            (
                config.account_saved_vars_file(addon_name),
                format!("account SavedVariables for {addon_name}"),
            ),
            (
                config.character_saved_vars_file(addon_name),
                format!("character SavedVariables for {addon_name}"),
            ),
        ];

        let mut loaded = 0;
        for (path, description) in files {
            if !path.exists() {
                continue;
            }
            match self.load_wtf_lua_file(lua, &path) {
                Ok(()) => loaded += 1,
                Err(error) => tracing::warn!("Failed to load {description}: {error}"),
            }
        }

        self.wtf_loaded.insert(addon_name.to_string(), loaded > 0);
        Ok(loaded)
    }

    /// Initialize saved variables for an addon before it loads.
    /// This creates empty tables in Lua globals for each declared variable,
    /// then loads any existing saved data into them.
    pub fn init_for_addon(
        &mut self,
        lua: &Lua,
        addon_name: &str,
        saved_vars: &[String],
        saved_vars_per_char: &[String],
    ) -> Result<()> {
        let globals = lua.globals();
        self.init_registered_globals(&globals, lua, addon_name, saved_vars, false)?;
        self.init_registered_globals(&globals, lua, addon_name, saved_vars_per_char, true)?;
        self.remember_registered_vars(addon_name, saved_vars, saved_vars_per_char);
        Ok(())
    }

    /// Save all registered variables for an addon in WoW-compatible Lua format.
    pub fn save_addon(&self, lua: &Lua, addon_name: &str) -> Result<()> {
        let globals = lua.globals();
        self.write_registered_file(&globals, addon_name, false);
        self.write_registered_file(&globals, addon_name, true);
        let _ = lua;
        Ok(())
    }

    /// Save all registered variables for all addons.
    pub fn save_all(&self, lua: &Lua) -> Result<()> {
        let addon_names: Vec<String> = self
            .registered
            .keys()
            .chain(self.registered_per_char.keys())
            .cloned()
            .collect();
        for addon_name in addon_names {
            self.save_addon(lua, &addon_name)?;
        }
        Ok(())
    }

    /// Get list of registered addons.
    pub fn registered_addons(&self) -> Vec<&String> {
        self.registered
            .keys()
            .chain(self.registered_per_char.keys())
            .collect()
    }

    fn init_registered_globals(
        &self,
        globals: &Table,
        lua: &Lua,
        addon_name: &str,
        variable_names: &[String],
        per_character: bool,
    ) -> Result<()> {
        for variable_name in variable_names {
            if !globals.get::<Value>(variable_name.as_str())?.is_nil() {
                continue;
            }
            let table = self.load_variable(lua, addon_name, variable_name, per_character)?;
            globals.set(variable_name.as_str(), table)?;
        }
        Ok(())
    }

    fn remember_registered_vars(
        &mut self,
        addon_name: &str,
        saved_vars: &[String],
        saved_vars_per_char: &[String],
    ) {
        if !saved_vars.is_empty() {
            self.registered
                .insert(addon_name.to_string(), saved_vars.to_vec());
        }
        if !saved_vars_per_char.is_empty() {
            self.registered_per_char
                .insert(addon_name.to_string(), saved_vars_per_char.to_vec());
        }
    }

    /// Load a WTF Lua file, executing it to set global variables.
    fn load_wtf_lua_file(&self, lua: &Lua, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path).map_err(mlua::Error::external)?;
        let content = content.strip_prefix('\u{feff}').unwrap_or(&content);
        let chunk_name = format!(
            "@WTF/{}",
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        lua.load(content).set_name(&chunk_name).exec()?;
        Ok(())
    }

    /// Load a single variable from storage by executing the saved .lua file.
    fn load_variable(
        &self,
        lua: &Lua,
        addon_name: &str,
        var_name: &str,
        per_character: bool,
    ) -> Result<Table> {
        let path = self.storage_path(addon_name, per_character);
        if !path.exists() {
            return lua.create_table();
        }

        self.load_wtf_lua_file(lua, &path)?;
        match lua.globals().get(var_name)? {
            Value::Table(table) => Ok(table),
            _ => lua.create_table(),
        }
    }

    /// Write variable values to a .lua file in WoW SavedVariables format.
    fn write_registered_file(&self, globals: &Table, addon_name: &str, per_character: bool) {
        let Some(variable_names) = self.registered_vars_for(addon_name, per_character) else {
            return;
        };
        let path = self.storage_path(addon_name, per_character);
        self.write_vars_file(globals, variable_names, &path);
    }

    fn write_vars_file(&self, globals: &Table, vars: &[String], path: &Path) {
        let mut output = String::from("\n");
        let mut has_data = false;

        for var_name in vars {
            let value: Value = match globals.get(var_name.as_str()) {
                Ok(value) => value,
                Err(_) => continue,
            };
            serialize_assignment(&mut output, var_name, &value);
            has_data = true;
        }

        if !has_data {
            return;
        }
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(path, output);
    }

    fn registered_vars_for(&self, addon_name: &str, per_character: bool) -> Option<&[String]> {
        if per_character {
            return self.registered_per_char.get(addon_name).map(Vec::as_slice);
        }
        self.registered.get(addon_name).map(Vec::as_slice)
    }

    /// Get the storage path for account-wide or per-character saved variables.
    fn storage_path(&self, addon_name: &str, per_character: bool) -> PathBuf {
        if per_character {
            return self
                .storage_dir
                .join(&self.realm_name)
                .join(&self.character_name)
                .join(format!("{}.lua", addon_name));
        }
        self.storage_dir.join(format!("{}.lua", addon_name))
    }
}

impl Default for SavedVariablesManager {
    fn default() -> Self {
        Self::new()
    }
}
