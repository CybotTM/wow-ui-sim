use std::fmt::Write;

use mlua::{Table, Value};

/// Serialize a top-level `VarName = value` assignment in WoW SavedVariables format.
pub(super) fn serialize_assignment(out: &mut String, name: &str, value: &Value) {
    let _ = write!(out, "{} = ", name);
    serialize_value(out, value, 0);
    out.push('\n');
}

/// Serialize a Lua value to WoW SavedVariables format.
fn serialize_value(out: &mut String, value: &Value, depth: usize) {
    match value {
        Value::Nil => out.push_str("nil"),
        Value::Boolean(value) => out.push_str(if *value { "true" } else { "false" }),
        Value::Integer(value) => {
            let _ = write!(out, "{}", value);
        }
        Value::Number(value) => write_number(out, *value),
        Value::String(value) => write_string(out, value),
        Value::Table(table) => serialize_table(out, table, depth),
        _ => out.push_str("nil"),
    }
}

fn write_number(out: &mut String, value: f64) {
    if value.fract() == 0.0 && value.abs() < i64::MAX as f64 {
        let _ = write!(out, "{}", value as i64);
        return;
    }
    let _ = write!(out, "{}", value);
}

fn write_string(out: &mut String, value: &mlua::String) {
    let Ok(value) = value.to_str() else {
        out.push_str("\"\"");
        return;
    };

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

/// Collect non-array entries from a table, sorted by key for deterministic output.
fn collect_hash_entries(table: &Table, array_len: usize) -> Vec<(String, Value)> {
    let Ok(pairs) = table
        .clone()
        .pairs::<Value, Value>()
        .collect::<std::result::Result<Vec<_>, _>>()
    else {
        return Vec::new();
    };
    let mut entries: Vec<(String, Value)> = pairs
        .into_iter()
        .filter_map(|(key, value)| match key {
            Value::Integer(index) if index >= 1 && index <= array_len as i64 => None,
            Value::String(string_key) => {
                string_key.to_str().ok().map(|key| (key.to_string(), value))
            }
            Value::Integer(index) => Some((index.to_string(), value)),
            Value::Number(number) => Some((number.to_string(), value)),
            _ => None,
        })
        .collect();
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
}

/// Write a Lua-escaped key string (for `["key"]` syntax).
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

/// Serialize a Lua table in WoW SavedVariables format.
///
/// WoW uses a specific format:
/// - Array entries (sequential integer keys 1..N) are written without explicit keys
/// - String/other keys use `["key"] = value` syntax
/// - Tables are indented with tabs
fn serialize_table(out: &mut String, table: &Table, depth: usize) {
    out.push_str("{\n");
    let indent = "\t".repeat(depth + 1);
    let array_len = table.raw_len();

    for index in 1..=array_len {
        let value: Value = match table.get(index as i64) {
            Ok(value) => value,
            Err(_) => break,
        };
        if value.is_nil() {
            break;
        }
        let _ = write!(out, "{}", indent);
        serialize_value(out, &value, depth + 1);
        let _ = writeln!(out, ", -- [{}]", index);
    }

    for (key, value) in &collect_hash_entries(table, array_len) {
        let _ = write!(out, "{}[\"", indent);
        write_escaped_key(out, key);
        out.push_str("\"] = ");
        serialize_value(out, value, depth + 1);
        out.push_str(",\n");
    }

    let _ = write!(out, "{}}}", "\t".repeat(depth));
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use mlua::Lua;
    use tempfile::tempdir;

    use crate::saved_variables::SavedVariablesManager;

    #[test]
    fn test_init_empty_variables() {
        let lua = Lua::new();
        let dir = tempdir().unwrap();
        let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

        mgr.init_for_addon(&lua, "TestAddon", &["TestDB".to_string()], &[])
            .unwrap();

        let globals = lua.globals();
        let db: Table = globals.get("TestDB").unwrap();
        assert!(db.is_empty());
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();

        {
            let lua = Lua::new();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

            mgr.init_for_addon(&lua, "TestAddon", &["TestDB".to_string()], &[])
                .unwrap();

            lua.load(r#"TestDB.setting1 = "hello"; TestDB.setting2 = 42"#)
                .exec()
                .unwrap();

            mgr.save_addon(&lua, "TestAddon").unwrap();
        }

        {
            let lua = Lua::new();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

            mgr.init_for_addon(&lua, "TestAddon", &["TestDB".to_string()], &[])
                .unwrap();

            let val1: String = lua.load("return TestDB.setting1").eval().unwrap();
            let val2: i64 = lua.load("return TestDB.setting2").eval().unwrap();

            assert_eq!(val1, "hello");
            assert_eq!(val2, 42);
        }
    }

    #[test]
    fn test_save_produces_lua_format() {
        let dir = tempdir().unwrap();
        let lua = Lua::new();
        let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

        mgr.init_for_addon(&lua, "TestAddon", &["TestDB".to_string()], &[])
            .unwrap();

        lua.load(r#"TestDB.name = "Haky"; TestDB.level = 70; TestDB.active = true"#)
            .exec()
            .unwrap();

        mgr.save_addon(&lua, "TestAddon").unwrap();

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
        let lua = Lua::new();
        let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

        mgr.init_for_addon(&lua, "TestAddon", &["TestDB".to_string()], &[])
            .unwrap();

        lua.load(
            r#"
            TestDB.nested = { a = 1, b = { c = "deep" } }
            TestDB.list = { 10, 20, 30 }
        "#,
        )
        .exec()
        .unwrap();

        mgr.save_addon(&lua, "TestAddon").unwrap();

        let lua2 = Lua::new();
        let mut mgr2 = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
        mgr2.init_for_addon(&lua2, "TestAddon", &["TestDB".to_string()], &[])
            .unwrap();

        let deep: String = lua2.load("return TestDB.nested.b.c").eval().unwrap();
        assert_eq!(deep, "deep");

        let second: i64 = lua2.load("return TestDB.list[2]").eval().unwrap();
        assert_eq!(second, 20);

        let len: i64 = lua2.load("return #TestDB.list").eval().unwrap();
        assert_eq!(len, 3);
    }

    #[test]
    fn test_save_string_escaping() {
        let dir = tempdir().unwrap();
        let lua = Lua::new();
        let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

        mgr.init_for_addon(&lua, "TestAddon", &["TestDB".to_string()], &[])
            .unwrap();

        lua.load(r#"TestDB.msg = "line1\nline2"; TestDB.path = "C:\\Users\\test""#)
            .exec()
            .unwrap();

        mgr.save_addon(&lua, "TestAddon").unwrap();

        let lua2 = Lua::new();
        let mut mgr2 = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
        mgr2.init_for_addon(&lua2, "TestAddon", &["TestDB".to_string()], &[])
            .unwrap();

        let msg: String = lua2.load("return TestDB.msg").eval().unwrap();
        assert_eq!(msg, "line1\nline2");

        let path: String = lua2.load("return TestDB.path").eval().unwrap();
        assert_eq!(path, "C:\\Users\\test");
    }

    #[test]
    fn test_per_character_variables() {
        let dir = tempdir().unwrap();

        {
            let lua = Lua::new();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
            mgr.set_character("Thrall", "Hyjal");

            mgr.init_for_addon(&lua, "TestAddon", &[], &["CharDB".to_string()])
                .unwrap();

            lua.load("CharDB.level = 70").exec().unwrap();
            mgr.save_addon(&lua, "TestAddon").unwrap();
        }

        {
            let lua = Lua::new();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
            mgr.set_character("Thrall", "Hyjal");

            mgr.init_for_addon(&lua, "TestAddon", &[], &["CharDB".to_string()])
                .unwrap();

            let level: i64 = lua.load("return CharDB.level").eval().unwrap();
            assert_eq!(level, 70);
        }

        {
            let lua = Lua::new();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
            mgr.set_character("Jaina", "Hyjal");

            mgr.init_for_addon(&lua, "TestAddon", &[], &["CharDB".to_string()])
                .unwrap();

            let level: Value = lua.load("return CharDB.level").eval().unwrap();
            assert!(level.is_nil());
        }
    }

    #[test]
    fn test_multiple_variables_per_addon() {
        let dir = tempdir().unwrap();

        {
            let lua = Lua::new();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

            mgr.init_for_addon(
                &lua,
                "Angleur",
                &[
                    "AngleurConfig".to_string(),
                    "AngleurMinimapButton".to_string(),
                ],
                &["AngleurCharacter".to_string()],
            )
            .unwrap();

            lua.load(
                r#"
                AngleurConfig.method = "oneKey"
                AngleurMinimapButton.hide = true
                AngleurCharacter.sleeping = false
            "#,
            )
            .exec()
            .unwrap();

            mgr.save_addon(&lua, "Angleur").unwrap();
        }

        {
            let lua = Lua::new();
            let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());

            mgr.init_for_addon(
                &lua,
                "Angleur",
                &[
                    "AngleurConfig".to_string(),
                    "AngleurMinimapButton".to_string(),
                ],
                &["AngleurCharacter".to_string()],
            )
            .unwrap();

            let method: String = lua.load("return AngleurConfig.method").eval().unwrap();
            assert_eq!(method, "oneKey");

            let hide: bool = lua.load("return AngleurMinimapButton.hide").eval().unwrap();
            assert!(hide);

            let sleeping: bool = lua.load("return AngleurCharacter.sleeping").eval().unwrap();
            assert!(!sleeping);
        }
    }

    #[test]
    fn test_serialize_format_matches_wow() {
        let lua = Lua::new();
        lua.load(
            r#"
            TestVar = {
                ["setting"] = "hello",
                ["items"] = { 10, 20, 30 },
            }
        "#,
        )
        .exec()
        .unwrap();

        let value: Value = lua.globals().get("TestVar").unwrap();
        let mut output = String::new();
        serialize_assignment(&mut output, "TestVar", &value);

        assert!(output.starts_with("TestVar = {"));
        assert!(output.contains("-- [1]"));
        assert!(output.contains("-- [2]"));
        assert!(output.contains("[\"setting\"] = \"hello\""));
    }
}
