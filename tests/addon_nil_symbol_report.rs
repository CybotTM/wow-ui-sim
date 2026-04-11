use std::io::Write;

use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn create_test_addon_with_missing_symbol_accesses() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let addon_dir = dir.path();

    let toc_path = addon_dir.join("TestNilSymbols.toc");
    let mut toc = std::fs::File::create(&toc_path).unwrap();
    writeln!(toc, "## Title: TestNilSymbols").unwrap();
    writeln!(toc, "TestNilSymbols.lua").unwrap();

    let lua_path = addon_dir.join("TestNilSymbols.lua");
    let mut lua = std::fs::File::create(&lua_path).unwrap();
    writeln!(
        lua,
        r#"
local _ = MissingGlobalSymbol
local _ = C_Container.MissingMethod
local _ = C_Container.MissingMethod
"#
    )
    .unwrap();

    dir
}

#[test]
fn load_addon_reports_missing_global_and_namespace_symbol_accesses() {
    let env = WowLuaEnv::new().unwrap();
    let dir = create_test_addon_with_missing_symbol_accesses();
    let toc_path = dir.path().join("TestNilSymbols.toc");

    let result = load_addon(&env.loader_env(), &toc_path).expect("addon load should succeed");

    assert!(
        result
            .warnings
            .contains(&"nil symbol access: _G.MissingGlobalSymbol (1x)".to_string()),
        "expected missing _G symbol warning, got {:?}",
        result.warnings
    );
    assert!(
        result
            .warnings
            .contains(&"nil symbol access: C_Container.MissingMethod (2x)".to_string()),
        "expected missing C_* symbol warning, got {:?}",
        result.warnings
    );
}
