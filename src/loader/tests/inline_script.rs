use super::super::addon::AddonContext;
use super::super::xml_file::load_xml_file;
use super::super::LoadTiming;
use crate::lua_api::WowLuaEnv;
use crate::xml::parse_xml_file;

#[test]
fn test_parse_wowless_test_xml() {
    let path = std::path::Path::new("Interface/AddOns/Wowless/test.xml");
    if !path.exists() {
        return; // Skip if Wowless addon not present
    }
    let result = parse_xml_file(path);
    assert!(result.is_ok(), "Wowless test.xml should parse: {:?}", result.err());
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
    let ctx = AddonContext {
        name: "TestAddon",
        table: addon_table,
        addon_root: &temp_dir,
        use_secure_env: false,
        taint: false,
    };
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

    let _ = std::fs::remove_dir_all(&temp_dir);
}
