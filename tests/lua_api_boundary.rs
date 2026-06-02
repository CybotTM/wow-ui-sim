#[test]
fn lua_api_globals_temporary_shims_module_is_removed() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/temporary_shims/mod.rs").exists(),
        "temporary Lua compatibility defaults should register directly from missing_surface or lua_api::workarounds, not through globals::temporary_shims"
    );
}
