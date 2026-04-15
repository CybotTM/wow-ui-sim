use super::super::LoadTiming;
use super::super::addon::AddonContext;
use super::super::lua_file::load_lua_file;
use super::super::xml_file::load_xml_file;
use crate::lua_api::WowLuaEnv;
use crate::xml::parse_xml_file;
use rilua::LuaApi;

#[test]
fn test_parse_wowless_test_xml() {
    let path = std::path::Path::new("Interface/AddOns/Wowless/test.xml");
    if !path.exists() {
        return; // Skip if Wowless addon not present
    }
    let result = parse_xml_file(path);
    assert!(
        result.is_ok(),
        "Wowless test.xml should parse: {:?}",
        result.err()
    );
}

#[test]
fn test_xml_inline_script_error_continues() {
    // In WoW, a Lua error inside a <Script> element does not abort the XML file.
    // Errors are caught by the error handler and processing continues.
    // This is how Wowless test.xml works: it sets WowlessXmlErrors = {} in the
    // first <Script>, then later elements may error, but the global persists.
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-inline-script-error");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test.xml");
    std::fs::write(
        &xml_path,
        r#"<Ui>
            <Script>
                ScriptErrorTestInit = "initialized"
            </Script>
            <Script>
                error("intentional error")
            </Script>
            <Script>
                ScriptErrorTestAfter = "still running"
            </Script>
        </Ui>"#,
    )
    .unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx =
        AddonContext::new(env.lua(), "TestAddon", addon_table, &temp_dir, false, false).unwrap();
    let before_errors = env.state().borrow().lua_errors.len();
    // Should not return an error — inline script errors are non-fatal
    let result = load_xml_file(
        &env.loader_env(),
        &xml_path,
        &ctx,
        &mut LoadTiming::default(),
    );
    assert!(
        result.is_ok(),
        "inline script error should not abort XML file: {:?}",
        result.err()
    );

    // First script should have run
    let init: String = env.eval("return ScriptErrorTestInit").unwrap();
    assert_eq!(init, "initialized");
    // Third script should also run despite second erroring
    let after: String = env.eval("return ScriptErrorTestAfter").unwrap();
    assert_eq!(after, "still running");
    let state = env.state().borrow();
    let new_errors = &state.lua_errors[before_errors..];
    assert!(
        new_errors
            .iter()
            .any(|msg| msg.contains("intentional error")),
        "inline XML script error should be collected in state.lua_errors: {new_errors:?}"
    );

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_load_lua_file_runtime_error_collects_lua_error() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test-load-lua-file-runtime-error");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let lua_path = temp_dir.join("test.lua");
    std::fs::write(&lua_path, r#"error("load lua failed")"#).unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx =
        AddonContext::new(env.lua(), "TestAddon", addon_table, &temp_dir, false, false).unwrap();

    let before_errors = env.state().borrow().lua_errors.len();
    let result = load_lua_file(
        &env.loader_env(),
        &lua_path,
        &ctx,
        &mut LoadTiming::default(),
    );
    assert!(result.is_err(), "runtime error should fail load_lua_file");

    let state = env.state().borrow();
    let new_errors = &state.lua_errors[before_errors..];
    assert!(
        new_errors.iter().any(|msg| msg.contains("load lua failed")),
        "load_lua_file runtime errors should be collected in state.lua_errors: {new_errors:?}"
    );

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_collect_lua_error_tracks_seen_message_counts() {
    let env = WowLuaEnv::new().unwrap();

    let first_seen = crate::lua_api::rilua_script_helpers::collect_lua_error(
        env.rilua().state(),
        "runtime error: repeated boom\nstack traceback:\n\t[C]: in function 'error'",
    );
    let second_seen = crate::lua_api::rilua_script_helpers::collect_lua_error(
        env.rilua().state(),
        "runtime error: repeated boom\nstack traceback:\n\t[C]: in function 'error'",
    );
    let third_seen = crate::lua_api::rilua_script_helpers::collect_lua_error(
        env.rilua().state(),
        "different boom",
    );

    let state = env.state().borrow();
    assert_eq!(state.lua_errors.len(), 3);
    assert_eq!(state.lua_error_counts.get("repeated boom"), Some(&2));
    assert_eq!(state.lua_error_counts.get("different boom"), Some(&1));
    assert!(first_seen, "first occurrence should be reported");
    assert!(!second_seen, "repeat occurrence should be suppressible");
    assert!(third_seen, "new message should be reported");
}

#[test]
fn test_suppressed_lua_error_summary_lines_report_repeat_counts() {
    let env = WowLuaEnv::new().unwrap();

    crate::lua_api::rilua_script_helpers::collect_lua_error(
        env.rilua().state(),
        "runtime error: repeated boom\nstack traceback:\n\t[C]: in function 'error'",
    );
    crate::lua_api::rilua_script_helpers::collect_lua_error(
        env.rilua().state(),
        "runtime error: repeated boom\nstack traceback:\n\t[C]: in function 'error'",
    );
    crate::lua_api::rilua_script_helpers::collect_lua_error(env.rilua().state(), "different boom");

    let state = env.state().borrow();
    let summary = crate::lua_errors::suppressed_error_summary_lines(&state);
    assert_eq!(summary.len(), 1);
    assert_eq!(
        summary[0],
        "Lua error suppressed 1 additional times: repeated boom"
    );
}
