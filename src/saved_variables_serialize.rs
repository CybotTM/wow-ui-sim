use std::fmt::Write;

use rilua::Val;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;

/// Serialize a top-level `VarName = value` assignment in WoW SavedVariables format.
pub(super) fn serialize_assignment(out: &mut String, state: &LuaState, name: &str, value: Val) {
    let _ = write!(out, "{name} = ");
    let mut seen = Vec::new();
    serialize_value(out, state, value, 0, &mut seen);
    out.push('\n');
}

fn serialize_value(
    out: &mut String,
    state: &LuaState,
    value: Val,
    depth: usize,
    seen: &mut Vec<GcRef<Table>>,
) {
    match value {
        Val::Nil => out.push_str("nil"),
        Val::Bool(value) => out.push_str(if value { "true" } else { "false" }),
        Val::Num(value) => write_number(out, value),
        Val::Str(value) => write_string(out, state, value),
        Val::Table(table) => serialize_table(out, state, table, depth, seen),
        _ => out.push_str("nil"),
    }
}

fn write_number(out: &mut String, value: f64) {
    if value.fract() == 0.0 && value.abs() < i64::MAX as f64 {
        let _ = write!(out, "{}", value as i64);
        return;
    }
    let _ = write!(out, "{value}");
}

fn write_string(
    out: &mut String,
    state: &LuaState,
    value: rilua::vm::gc::arena::GcRef<rilua::vm::string::LuaString>,
) {
    let Some(value) = state.gc.string_arena.get(value) else {
        out.push_str("\"\"");
        return;
    };
    let value = String::from_utf8_lossy(value.data());

    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\0' => out.push_str("\\0"),
            other => out.push(other),
        }
    }
    out.push('"');
}

fn serialize_table(
    out: &mut String,
    state: &LuaState,
    table_ref: GcRef<Table>,
    depth: usize,
    seen: &mut Vec<GcRef<Table>>,
) {
    if seen.contains(&table_ref) {
        out.push_str("{}");
        return;
    }
    seen.push(table_ref);

    out.push_str("{\n");
    let indent = "\t".repeat(depth + 1);
    let Some(table) = state.gc.tables.get(table_ref) else {
        let _ = write!(out, "{}}}", "\t".repeat(depth));
        seen.pop();
        return;
    };
    let array_values = table.array_slice();
    let array_len = array_values
        .iter()
        .take_while(|value| !matches!(value, Val::Nil))
        .count();

    for (index, value) in array_values.iter().take(array_len).copied().enumerate() {
        let _ = write!(out, "{indent}");
        serialize_value(out, state, value, depth + 1, seen);
        let _ = writeln!(out, ", -- [{}]", index + 1);
    }

    for (key, value) in collect_hash_entries(state, table_ref, array_len) {
        let _ = write!(out, "{indent}[");
        write_key(out, &key);
        out.push_str("] = ");
        serialize_value(out, state, value, depth + 1, seen);
        out.push_str(",\n");
    }

    let _ = write!(out, "{}}}", "\t".repeat(depth));
    seen.pop();
}

fn collect_hash_entries(
    state: &LuaState,
    table_ref: GcRef<Table>,
    array_len: usize,
) -> Vec<(SavedVarKey, Val)> {
    let Some(table) = state.gc.tables.get(table_ref) else {
        return Vec::new();
    };

    let mut entries: Vec<(SavedVarKey, Val)> = table
        .hash_entries()
        .into_iter()
        .filter_map(|(key, value)| match classify_key(state, key) {
            Some(SavedVarKey::ArrayIndex(index)) if index >= 1 && index <= array_len as i64 => None,
            Some(kind) => Some((kind, value)),
            None => None,
        })
        .collect();
    entries.sort_by(|left, right| left.0.sort_key().cmp(&right.0.sort_key()));
    entries
}

fn classify_key(state: &LuaState, key: Val) -> Option<SavedVarKey> {
    match key {
        Val::Str(key) => {
            let string = state.gc.string_arena.get(key)?;
            Some(SavedVarKey::String(
                String::from_utf8_lossy(string.data()).into_owned(),
            ))
        }
        Val::Num(number) if is_integral(number) => Some(SavedVarKey::ArrayIndex(number as i64)),
        Val::Num(number) => Some(SavedVarKey::Number(number)),
        _ => None,
    }
}

fn is_integral(number: f64) -> bool {
    number.fract() == 0.0 && number.is_finite() && number.abs() < i64::MAX as f64
}

fn write_key(out: &mut String, key: &SavedVarKey) {
    match key {
        SavedVarKey::String(key) => {
            out.push('"');
            write_escaped_key(out, key);
            out.push('"');
        }
        SavedVarKey::ArrayIndex(index) => {
            let _ = write!(out, "{index}");
        }
        SavedVarKey::Number(number) => write_number(out, *number),
    }
}

fn write_escaped_key(out: &mut String, key: &str) {
    for ch in key.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            other => out.push(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SavedVarKey {
    String(String),
    ArrayIndex(i64),
    Number(f64),
}

impl SavedVarKey {
    fn sort_key(&self) -> String {
        match self {
            Self::String(key) => format!("s:{key}"),
            Self::ArrayIndex(index) => format!("i:{index:020}"),
            Self::Number(number) => format!("n:{number}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use rilua::LuaApi;
    use rilua::LuaApiMut;
    use tempfile::tempdir;

    use crate::lua_api::WowLuaEnv;
    use crate::saved_variables::{SavedVariablesManager, WtfConfig};

    fn new_env() -> WowLuaEnv {
        WowLuaEnv::new().unwrap()
    }

    fn with_state<T>(env: &WowLuaEnv, f: impl FnOnce(&mut LuaState) -> T) -> T {
        let mut lua = env.rilua_mut();
        f(lua.state_mut())
    }

    fn ensure_saved_var_table(env: &WowLuaEnv, var_name: &str) {
        env.exec(&format!("{var_name} = {var_name} or {{}}"))
            .unwrap();
    }

    #[test]
    fn test_init_missing_variables_stay_nil_until_initialized_by_addon() {
        let env = new_env();
        let dir = tempdir().unwrap();
        let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

        with_state(&env, |state| {
            mgr.init_for_addon(state, "TestAddon", &["TestDB".to_string()], &[])
        })
        .unwrap();

        let value_type: String = env.eval("return type(TestDB)").unwrap();
        assert_eq!(value_type, "nil");
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();

        {
            let env = new_env();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

            with_state(&env, |state| {
                mgr.init_for_addon(state, "TestAddon", &["TestDB".to_string()], &[])
            })
            .unwrap();

            ensure_saved_var_table(&env, "TestDB");
            env.exec(r#"TestDB.setting1 = "hello"; TestDB.setting2 = 42"#)
                .unwrap();

            with_state(&env, |state| mgr.save_addon(state, "TestAddon")).unwrap();
        }

        {
            let env = new_env();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

            with_state(&env, |state| {
                mgr.init_for_addon(state, "TestAddon", &["TestDB".to_string()], &[])
            })
            .unwrap();

            let val1: String = env.eval("return TestDB.setting1").unwrap();
            let val2: i64 = env.eval("return TestDB.setting2").unwrap();

            assert_eq!(val1, "hello");
            assert_eq!(val2, 42);
        }
    }

    #[test]
    fn test_save_produces_lua_format() {
        let dir = tempdir().unwrap();
        let env = new_env();
        let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

        with_state(&env, |state| {
            mgr.init_for_addon(state, "TestAddon", &["TestDB".to_string()], &[])
        })
        .unwrap();

        ensure_saved_var_table(&env, "TestDB");
        env.exec(r#"TestDB.name = "Haky"; TestDB.level = 70; TestDB.active = true"#)
            .unwrap();

        with_state(&env, |state| mgr.save_addon(state, "TestAddon")).unwrap();

        let content = fs::read_to_string(dir.path().join("TestAddon.lua")).unwrap();
        assert!(content.contains("TestDB = {"), "should have Lua assignment");
        assert!(
            content.contains("[\"name\"] = \"Haky\""),
            "should have string value"
        );
        assert!(
            content.contains("[\"level\"] = 70"),
            "should have integer value"
        );
        assert!(
            content.contains("[\"active\"] = true"),
            "should have boolean value"
        );
    }

    #[test]
    fn test_save_nested_tables() {
        let dir = tempdir().unwrap();
        let env = new_env();
        let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

        with_state(&env, |state| {
            mgr.init_for_addon(state, "TestAddon", &["TestDB".to_string()], &[])
        })
        .unwrap();

        ensure_saved_var_table(&env, "TestDB");
        env.exec(
            r#"
            TestDB.nested = { a = 1, b = { c = "deep" } }
            TestDB.list = { 10, 20, 30 }
        "#,
        )
        .unwrap();

        with_state(&env, |state| mgr.save_addon(state, "TestAddon")).unwrap();

        let env2 = new_env();
        let mut mgr2 = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
        with_state(&env2, |state| {
            mgr2.init_for_addon(state, "TestAddon", &["TestDB".to_string()], &[])
        })
        .unwrap();

        let deep: String = env2.eval("return TestDB.nested.b.c").unwrap();
        assert_eq!(deep, "deep");

        let second: i64 = env2.eval("return TestDB.list[2]").unwrap();
        assert_eq!(second, 20);

        let len: i64 = env2.eval("return #TestDB.list").unwrap();
        assert_eq!(len, 3);
    }

    #[test]
    fn test_save_string_escaping() {
        let dir = tempdir().unwrap();
        let env = new_env();
        let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

        with_state(&env, |state| {
            mgr.init_for_addon(state, "TestAddon", &["TestDB".to_string()], &[])
        })
        .unwrap();

        ensure_saved_var_table(&env, "TestDB");
        env.exec(r#"TestDB.msg = "line1\nline2"; TestDB.path = "C:\\Users\\test""#)
            .unwrap();

        with_state(&env, |state| mgr.save_addon(state, "TestAddon")).unwrap();

        let env2 = new_env();
        let mut mgr2 = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
        with_state(&env2, |state| {
            mgr2.init_for_addon(state, "TestAddon", &["TestDB".to_string()], &[])
        })
        .unwrap();

        let msg: String = env2.eval("return TestDB.msg").unwrap();
        assert_eq!(msg, "line1\nline2");

        let path: String = env2.eval("return TestDB.path").unwrap();
        assert_eq!(path, "C:\\Users\\test");
    }

    #[test]
    fn test_per_character_variables() {
        let dir = tempdir().unwrap();

        {
            let env = new_env();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
            mgr.set_character("Thrall", "Hyjal");

            with_state(&env, |state| {
                mgr.init_for_addon(state, "TestAddon", &[], &["CharDB".to_string()])
            })
            .unwrap();

            ensure_saved_var_table(&env, "CharDB");
            env.exec("CharDB.level = 70").unwrap();
            with_state(&env, |state| mgr.save_addon(state, "TestAddon")).unwrap();
        }

        {
            let env = new_env();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
            mgr.set_character("Thrall", "Hyjal");

            with_state(&env, |state| {
                mgr.init_for_addon(state, "TestAddon", &[], &["CharDB".to_string()])
            })
            .unwrap();

            let level: i64 = env.eval("return CharDB.level").unwrap();
            assert_eq!(level, 70);
        }

        {
            let env = new_env();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
            mgr.set_character("Jaina", "Hyjal");

            with_state(&env, |state| {
                mgr.init_for_addon(state, "TestAddon", &[], &["CharDB".to_string()])
            })
            .unwrap();

            let value_type: String = env.eval("return type(CharDB)").unwrap();
            assert_eq!(value_type, "nil");
        }
    }

    #[test]
    fn test_multiple_variables_per_addon() {
        let dir = tempdir().unwrap();

        {
            let env = new_env();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

            with_state(&env, |state| {
                mgr.init_for_addon(
                    state,
                    "Angleur",
                    &[
                        "AngleurConfig".to_string(),
                        "AngleurMinimapButton".to_string(),
                    ],
                    &["AngleurCharacter".to_string()],
                )
            })
            .unwrap();

            ensure_saved_var_table(&env, "AngleurConfig");
            ensure_saved_var_table(&env, "AngleurMinimapButton");
            ensure_saved_var_table(&env, "AngleurCharacter");
            env.exec(
                r#"
                AngleurConfig.method = "oneKey"
                AngleurMinimapButton.hide = true
                AngleurCharacter.sleeping = false
            "#,
            )
            .unwrap();

            with_state(&env, |state| mgr.save_addon(state, "Angleur")).unwrap();
        }

        {
            let env = new_env();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

            with_state(&env, |state| {
                mgr.init_for_addon(
                    state,
                    "Angleur",
                    &[
                        "AngleurConfig".to_string(),
                        "AngleurMinimapButton".to_string(),
                    ],
                    &["AngleurCharacter".to_string()],
                )
            })
            .unwrap();

            let method: String = env.eval("return AngleurConfig.method").unwrap();
            assert_eq!(method, "oneKey");

            let hide: bool = env.eval("return AngleurMinimapButton.hide").unwrap();
            assert!(hide);

            let sleeping: bool = env.eval("return AngleurCharacter.sleeping").unwrap();
            assert!(!sleeping);
        }
    }

    #[test]
    fn test_wtf_import_source_takes_precedence_over_local_storage() {
        let dir = tempdir().unwrap();
        let wtf_root = dir.path().join("WTF");
        let local_root = dir.path().join("LocalSavedVariables");
        let wtf_path = wtf_root
            .join("Account")
            .join("TestAccount")
            .join("SavedVariables");
        fs::create_dir_all(&wtf_path).unwrap();
        fs::create_dir_all(&local_root).unwrap();
        fs::write(
            wtf_path.join("TestAddon.lua"),
            "\nTestDB = { [\"source\"] = \"wtf\" }\n",
        )
        .unwrap();
        fs::write(
            local_root.join("TestAddon.lua"),
            "\nTestDB = { [\"source\"] = \"local\" }\n",
        )
        .unwrap();

        let env = new_env();
        let mut mgr = SavedVariablesManager::with_storage_dir(local_root);
        mgr.set_wtf_config(WtfConfig::new(
            &wtf_root,
            "TestAccount",
            "Realm",
            "Character",
        ));

        let loaded = with_state(&env, |state| mgr.load_wtf_for_addon(state, "TestAddon")).unwrap();
        assert_eq!(loaded, 1);
        with_state(&env, |state| {
            mgr.init_for_addon(state, "TestAddon", &["TestDB".to_string()], &[])
        })
        .unwrap();

        let source: String = env.eval("return TestDB.source").unwrap();
        assert_eq!(source, "wtf");
    }

    #[test]
    fn test_wtf_source_is_read_only_when_saving() {
        let dir = tempdir().unwrap();
        let wtf_root = dir.path().join("WTF");
        let local_root = dir.path().join("LocalSavedVariables");
        let wtf_path = wtf_root
            .join("Account")
            .join("TestAccount")
            .join("SavedVariables");
        fs::create_dir_all(&wtf_path).unwrap();
        fs::create_dir_all(&local_root).unwrap();
        let wtf_file = wtf_path.join("TestAddon.lua");
        fs::write(
            &wtf_file,
            "\nTestDB = { [\"source\"] = \"wtf\", [\"unchanged\"] = true }\n",
        )
        .unwrap();

        let env = new_env();
        let mut mgr = SavedVariablesManager::with_storage_dir(local_root.clone());
        mgr.set_wtf_config(WtfConfig::new(
            &wtf_root,
            "TestAccount",
            "Realm",
            "Character",
        ));

        let loaded = with_state(&env, |state| mgr.load_wtf_for_addon(state, "TestAddon")).unwrap();
        assert_eq!(loaded, 1);
        with_state(&env, |state| {
            mgr.init_for_addon(state, "TestAddon", &["TestDB".to_string()], &[])
        })
        .unwrap();

        env.exec(r#"TestDB.source = "sim"; TestDB.newValue = 42"#)
            .unwrap();
        with_state(&env, |state| mgr.save_addon(state, "TestAddon")).unwrap();

        let wtf_content = fs::read_to_string(&wtf_file).unwrap();
        assert!(
            wtf_content.contains("[\"source\"] = \"wtf\""),
            "live WTF file should remain unchanged"
        );
        assert!(
            !wtf_content.contains("newValue"),
            "live WTF file should not receive simulator writes"
        );

        let local_content = fs::read_to_string(local_root.join("TestAddon.lua")).unwrap();
        assert!(local_content.contains("[\"source\"] = \"sim\""));
        assert!(local_content.contains("[\"newValue\"] = 42"));
    }

    #[test]
    fn test_invalid_local_saved_variables_do_not_abort_initialization() {
        let dir = tempdir().unwrap();
        let local_root = dir.path().join("LocalSavedVariables");
        fs::create_dir_all(&local_root).unwrap();
        fs::write(
            local_root.join("TestAddon.lua"),
            "\nTestDB TestMinimapDB = nil\n",
        )
        .unwrap();

        let env = new_env();
        let mut mgr = SavedVariablesManager::with_storage_dir(local_root);
        with_state(&env, |state| {
            mgr.init_for_addon(
                state,
                "TestAddon",
                &["TestDB".to_string(), "TestMinimapDB".to_string()],
                &[],
            )
        })
        .unwrap();

        let types: (String, String) = env
            .eval("return type(TestDB), type(TestMinimapDB)")
            .unwrap();
        assert_eq!(types, ("nil".to_string(), "nil".to_string()));
    }

    #[test]
    fn test_invalid_local_saved_variables_fall_back_to_wtf_source() {
        let dir = tempdir().unwrap();
        let wtf_root = dir.path().join("WTF");
        let local_root = dir.path().join("LocalSavedVariables");
        let wtf_path = wtf_root
            .join("Account")
            .join("TestAccount")
            .join("SavedVariables");
        fs::create_dir_all(&wtf_path).unwrap();
        fs::create_dir_all(&local_root).unwrap();
        fs::write(
            local_root.join("TestAddon.lua"),
            "\nTestDB TestMinimapDB = nil\n",
        )
        .unwrap();
        fs::write(
            wtf_path.join("TestAddon.lua"),
            "\nTestDB = { [\"source\"] = \"wtf\" }\nTestMinimapDB = { [\"hide\"] = true }\n",
        )
        .unwrap();

        let env = new_env();
        let mut mgr = SavedVariablesManager::with_storage_dir(local_root);
        mgr.set_wtf_config(WtfConfig::new(
            &wtf_root,
            "TestAccount",
            "Realm",
            "Character",
        ));
        with_state(&env, |state| {
            mgr.init_for_addon(
                state,
                "TestAddon",
                &["TestDB".to_string(), "TestMinimapDB".to_string()],
                &[],
            )
        })
        .unwrap();

        let values: (String, bool) = env
            .eval("return TestDB.source, TestMinimapDB.hide")
            .unwrap();
        assert_eq!(values, ("wtf".to_string(), true));
    }

    #[test]
    fn test_serialize_format_matches_wow() {
        let env = new_env();
        env.exec(
            r#"
            TestVar = {
                ["setting"] = "hello",
                ["items"] = { 10, 20, 30 },
            }
        "#,
        )
        .unwrap();

        let value: Val = env.eval("return TestVar").unwrap();
        let lua = env.rilua();
        let mut output = String::new();
        serialize_assignment(&mut output, lua.state(), "TestVar", value);

        assert!(output.starts_with("TestVar = {"));
        assert!(output.contains("-- [1]"));
        assert!(output.contains("-- [2]"));
        assert!(output.contains("[\"setting\"] = \"hello\""));
    }
}
