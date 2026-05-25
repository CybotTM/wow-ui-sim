#[test]
fn edit_mode_cache_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "__wow_copy_edit_mode_value",
        "__wow_edit_mode_parse_account_cache",
        "__wow_edit_mode_active_layout_from_character_cache",
        "function C_EditMode.GetAccountSettings",
        "function C_EditMode.GetLayouts",
        "function C_EditMode.__LoadCache",
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} must live in the explicit temporary EditMode cache workaround boundary, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} must live in the explicit temporary EditMode cache workaround boundary, not shared bootstrap"
        );
    }
}
