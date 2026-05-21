use std::fs;
use std::path::{Path, PathBuf};

use rilua::LuaApiMut;
use tempfile::{TempDir, tempdir};

use crate::lua_api::WowLuaEnv;
use crate::saved_variables::{SavedVariablesManager, WtfConfig};

#[test]
fn wtf_details_import_clamps_narrow_meter_windows() {
    let fixture = DetailsWtfFixture::new();
    fixture.write_account_saved_variables();
    fixture.write_character_saved_variables();

    let env = WowLuaEnv::new().unwrap();
    let mut manager = fixture.manager();
    let loaded = with_state(&env, |state| manager.load_wtf_for_addon(state, "Details")).unwrap();
    assert_eq!(loaded, 2);
    init_details_saved_variables(&env, &mut manager);

    assert_eq!(details_local_window_width(&env), 300.0);
    assert_eq!(details_profile_window_width(&env), 300.0);
}

struct DetailsWtfFixture {
    _temp_dir: TempDir,
    wtf_root: PathBuf,
    local_root: PathBuf,
}

impl DetailsWtfFixture {
    fn new() -> Self {
        let temp_dir = tempdir().unwrap();
        let wtf_root = temp_dir.path().join("WTF");
        let local_root = temp_dir.path().join("LocalSavedVariables");
        fs::create_dir_all(account_saved_variables_path(&wtf_root)).unwrap();
        fs::create_dir_all(character_saved_variables_path(&wtf_root)).unwrap();
        fs::create_dir_all(&local_root).unwrap();
        Self {
            _temp_dir: temp_dir,
            wtf_root,
            local_root,
        }
    }

    fn manager(&self) -> SavedVariablesManager {
        let mut manager = SavedVariablesManager::with_storage_dir(self.local_root.clone());
        manager.set_wtf_config(WtfConfig::new(
            &self.wtf_root,
            "TestAccount",
            "Realm",
            "Character",
        ));
        manager
    }

    fn write_account_saved_variables(&self) {
        fs::write(
            account_saved_variables_path(&self.wtf_root).join("Details.lua"),
            DETAILS_ACCOUNT_SAVED_VARIABLES,
        )
        .unwrap();
    }

    fn write_character_saved_variables(&self) {
        fs::write(
            character_saved_variables_path(&self.wtf_root).join("Details.lua"),
            DETAILS_CHARACTER_SAVED_VARIABLES,
        )
        .unwrap();
    }
}

const DETAILS_ACCOUNT_SAVED_VARIABLES: &str = r#"
_detalhes_global = {
    ["__profiles"] = {
        ["Haky"] = {
            ["instances"] = {
                {
                    ["__pos"] = {
                        ["normal"] = {
                            ["w"] = 191,
                            ["h"] = 124,
                        },
                    },
                },
            },
        },
    },
}
"#;

const DETAILS_CHARACTER_SAVED_VARIABLES: &str = r#"
_detalhes_database = {
    ["local_instances_config"] = {
        {
            ["is_open"] = true,
            ["pos"] = {
                ["normal"] = {
                    ["x"] = -1060,
                    ["y"] = 406,
                    ["w"] = 191,
                    ["h"] = 124,
                },
            },
        },
    },
}
"#;

fn with_state<T>(env: &WowLuaEnv, f: impl FnOnce(&mut rilua::vm::state::LuaState) -> T) -> T {
    let mut lua = env.rilua_mut();
    f(lua.state_mut())
}

fn init_details_saved_variables(env: &WowLuaEnv, manager: &mut SavedVariablesManager) {
    with_state(env, |state| {
        manager.init_for_addon(state, "Details", &[], &["_detalhes_database".to_string()])
    })
    .unwrap();
}

fn details_local_window_width(env: &WowLuaEnv) -> f64 {
    env.eval("return _detalhes_database.local_instances_config[1].pos.normal.w")
        .unwrap()
}

fn details_profile_window_width(env: &WowLuaEnv) -> f64 {
    env.eval("return _detalhes_global.__profiles.Haky.instances[1].__pos.normal.w")
        .unwrap()
}

fn account_saved_variables_path(wtf_root: &Path) -> PathBuf {
    wtf_root
        .join("Account")
        .join("TestAccount")
        .join("SavedVariables")
}

fn character_saved_variables_path(wtf_root: &Path) -> PathBuf {
    wtf_root
        .join("Account")
        .join("TestAccount")
        .join("Realm")
        .join("Character")
        .join("SavedVariables")
}
