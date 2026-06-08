use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::{SavedVariablesManager, WtfConfig};

#[test]
fn server_snapshot_action_bars_seed_get_action_info() {
    let env = WowLuaEnv::new().expect("Lua env");

    env.exec(
        r#"
        ServerSnapshotDB = {
            lastCharacterKey = "SimRealm/SimPlayer",
            characters = {
                ["SimRealm/SimPlayer"] = {
                    actionBars = {
                        slots = {
                            [1] = { type = "spell", id = 19750, spellID = 19750 },
                            [2] = { type = "macro", id = 1 },
                            [3] = { empty = true },
                            [13] = { type = "spell", id = 4987 },
                        },
                    },
                },
            },
        }
        "#,
    )
    .expect("seed ServerSnapshotDB");

    let imported = wow_ui_sim::server_snapshot_import::apply_loaded_snapshot(&env)
        .expect("import snapshot action bars");
    assert_eq!(imported, 2);

    let (slot1_type, slot1_id, slot13_type, slot13_id, slot2_has, slot3_has): (
        String,
        i64,
        String,
        i64,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local t1, id1 = GetActionInfo(1)
            local t13, id13 = GetActionInfo(13)
            return t1, id1, t13, id13, HasAction(2), HasAction(3)
            "#,
        )
        .expect("read action bars");

    assert_eq!(slot1_type, "spell");
    assert_eq!(slot1_id, 19750);
    assert_eq!(slot13_type, "spell");
    assert_eq!(slot13_id, 4987);
    assert!(!slot2_has, "non-spell snapshot entries are ignored for now");
    assert!(!slot3_has, "empty snapshot entries stay empty");
}

#[test]
fn server_snapshot_uses_latest_character_when_last_key_missing() {
    let env = WowLuaEnv::new().expect("Lua env");

    env.exec(
        r#"
        ServerSnapshotDB = {
            characters = {
                Older = {
                    capturedAt = 10,
                    actionBars = {
                        slots = {
                            [1] = { type = "spell", id = 111 },
                        },
                    },
                },
                Newer = {
                    capturedAt = 20,
                    actionBars = {
                        slots = {
                            [1] = { type = "spell", id = 222 },
                        },
                    },
                },
            },
        }
        "#,
    )
    .expect("seed ServerSnapshotDB");

    let imported = wow_ui_sim::server_snapshot_import::apply_loaded_snapshot(&env)
        .expect("import latest snapshot");
    assert_eq!(imported, 1);

    let spell_id: i64 = env
        .eval(
            r#"
            local _type, id = GetActionInfo(1)
            return id
            "#,
        )
        .expect("read imported spell id");
    assert_eq!(spell_id, 222);
}

#[test]
fn server_snapshot_loads_from_wtf_saved_variables_file() {
    let temp = tempfile::tempdir().expect("temp dir");
    let saved_vars_dir = temp.path().join("Account/AccountName/SavedVariables");
    std::fs::create_dir_all(&saved_vars_dir).expect("create saved vars dir");
    std::fs::write(
        saved_vars_dir.join("ServerSnapshot.lua"),
        r#"
        ServerSnapshotDB = {
            lastCharacterKey = "RealmName/CharacterName",
            characters = {
                ["RealmName/CharacterName"] = {
                    capturedAt = 123,
                    actionBars = {
                        slots = {
                            [1] = { type = "spell", id = 19750, spellID = 19750 },
                        },
                    },
                },
            },
        }
        "#,
    )
    .expect("write ServerSnapshot saved vars");

    let env = WowLuaEnv::new().expect("Lua env");
    let mut saved_vars = SavedVariablesManager::with_storage_dir(temp.path().join("local"));
    saved_vars.set_wtf_config(WtfConfig::new(
        temp.path(),
        "AccountName",
        "RealmName",
        "CharacterName",
    ));

    let imported = wow_ui_sim::server_snapshot_import::load_from_saved_variables(
        &env,
        &mut saved_vars,
    )
    .expect("load ServerSnapshot from WTF");
    assert_eq!(imported, 1);

    let (action_type, spell_id): (String, i64) = env
        .eval(
            r#"
            local actionType, id = GetActionInfo(1)
            return actionType, id
            "#,
        )
        .expect("read imported action");
    assert_eq!(action_type, "spell");
    assert_eq!(spell_id, 19750);
}
