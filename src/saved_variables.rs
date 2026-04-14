//! Saved variables management for addon persistence.
//!
//! WoW addons can declare SavedVariables and SavedVariablesPerCharacter in
//! their `.toc` files. These are global Lua tables that persist between
//! sessions.
//!
//! Storage uses WoW-compatible Lua format (`VarName = { ... }`), so files can
//! be shared between the simulator and a real WoW installation.
//!
//! Loading priority:
//! 1. WTF directory (real WoW installation, if configured)
//! 2. Simulator storage (`~/.local/share/wow-sim/SavedVariables/`)

#[path = "saved_variables_serialize.rs"]
mod saved_variables_serialize;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, Val};

use saved_variables_serialize::serialize_assignment;

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

/// Manages saved variables for all loaded addons.
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

    /// Load WTF saved variables for an addon from a real WoW installation.
    ///
    /// Returns the number of files loaded (0, 1, or 2 for account + character).
    pub fn load_wtf_for_addon(
        &mut self,
        lua: &RefCell<rilua::Lua>,
        addon_name: &str,
    ) -> crate::Result<usize> {
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
            match self.load_lua_file(lua, &path, "@WTF") {
                Ok(()) => loaded += 1,
                Err(error) => tracing::warn!("Failed to load {description}: {error}"),
            }
        }

        self.wtf_loaded.insert(addon_name.to_string(), loaded > 0);
        Ok(loaded)
    }

    /// Initialize saved variables for an addon before it loads.
    pub fn init_for_addon(
        &mut self,
        lua: &RefCell<rilua::Lua>,
        addon_name: &str,
        saved_vars: &[String],
        saved_vars_per_char: &[String],
    ) -> crate::Result<()> {
        self.init_registered_globals(lua, addon_name, saved_vars, false)?;
        self.init_registered_globals(lua, addon_name, saved_vars_per_char, true)?;
        self.remember_registered_vars(addon_name, saved_vars, saved_vars_per_char);
        Ok(())
    }

    /// Save all registered variables for an addon in WoW-compatible Lua format.
    pub fn save_addon(&self, lua: &RefCell<rilua::Lua>, addon_name: &str) -> crate::Result<()> {
        let mut lua = lua.borrow_mut();
        self.write_registered_file(lua.state_mut(), addon_name, false)?;
        self.write_registered_file(lua.state_mut(), addon_name, true)?;
        Ok(())
    }

    /// Save all registered variables for all addons.
    pub fn save_all(&self, lua: &RefCell<rilua::Lua>) -> crate::Result<()> {
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

    pub fn registered_addons(&self) -> Vec<&String> {
        self.registered
            .keys()
            .chain(self.registered_per_char.keys())
            .collect()
    }

    fn init_registered_globals(
        &self,
        lua: &RefCell<rilua::Lua>,
        addon_name: &str,
        variable_names: &[String],
        per_character: bool,
    ) -> crate::Result<()> {
        for variable_name in variable_names {
            let already_present = {
                let mut lua = lua.borrow_mut();
                !matches!(get_global(lua.state_mut(), variable_name), Val::Nil)
            };
            if already_present {
                continue;
            }
            let table = self.load_variable(lua, addon_name, variable_name, per_character)?;
            set_global(lua.borrow_mut().state_mut(), variable_name, table);
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

    fn load_variable(
        &self,
        lua: &RefCell<rilua::Lua>,
        addon_name: &str,
        var_name: &str,
        per_character: bool,
    ) -> crate::Result<Val> {
        let path = self.storage_path(addon_name, per_character);
        if !path.exists() {
            return Ok(create_empty_table(lua.borrow_mut().state_mut()));
        }

        self.load_lua_file(lua, &path, "@SavedVariables")?;
        let value = {
            let mut lua = lua.borrow_mut();
            get_global(lua.state_mut(), var_name)
        };
        if matches!(value, Val::Table(_)) {
            return Ok(value);
        }

        Ok(create_empty_table(lua.borrow_mut().state_mut()))
    }

    fn load_lua_file(
        &self,
        lua: &RefCell<rilua::Lua>,
        path: &Path,
        chunk_prefix: &str,
    ) -> crate::Result<()> {
        let content = fs::read_to_string(path).map_err(|e| crate::Error::Other(e.to_string()))?;
        let content = content.strip_prefix('\u{feff}').unwrap_or(&content);
        let chunk_name = format!(
            "{}/{}",
            chunk_prefix,
            path.file_name().unwrap_or_default().to_string_lossy()
        );
        let mut lua = lua.borrow_mut();
        let func = LuaApiMut::load_bytes(&mut *lua, content.as_bytes(), &chunk_name)?;
        lua.call_function(&func, &[])?;
        Ok(())
    }

    fn write_registered_file(
        &self,
        state: &mut LuaState,
        addon_name: &str,
        per_character: bool,
    ) -> crate::Result<()> {
        let Some(variable_names) = self.registered_vars_for(addon_name, per_character) else {
            return Ok(());
        };
        let path = self.storage_path(addon_name, per_character);
        self.write_vars_file(state, variable_names, &path)
    }

    fn write_vars_file(
        &self,
        state: &mut LuaState,
        vars: &[String],
        path: &Path,
    ) -> crate::Result<()> {
        let mut output = String::from("\n");
        let mut has_data = false;

        for var_name in vars {
            let value = get_global(state, var_name);
            serialize_assignment(&mut output, state, var_name, value);
            has_data = true;
        }

        if !has_data {
            return Ok(());
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| crate::Error::Other(e.to_string()))?;
        }
        fs::write(path, output).map_err(|e| crate::Error::Other(e.to_string()))?;
        Ok(())
    }

    fn registered_vars_for(&self, addon_name: &str, per_character: bool) -> Option<&[String]> {
        if per_character {
            return self.registered_per_char.get(addon_name).map(Vec::as_slice);
        }
        self.registered.get(addon_name).map(Vec::as_slice)
    }

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

fn get_global(state: &mut LuaState, name: &str) -> Val {
    let key = state.gc.intern_string(name.as_bytes());
    state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(key, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

fn set_global(state: &mut LuaState, name: &str, value: Val) {
    let key = state.gc.intern_string(name.as_bytes());
    if let Some(globals) = state.gc.tables.get_mut(state.global) {
        let _ = globals.raw_set(Val::Str(key), value, &state.gc.string_arena);
    }
}

fn create_empty_table(state: &mut LuaState) -> Val {
    Val::Table(state.gc.alloc_table(Table::new()))
}
