#[test]
fn menu_helper_methods_are_not_runtime_bootstrap_fallbacks() {
    let runtime_bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "SetDefaultText",
        "SetSelectionTranslator",
        "SetSelectionText",
        "EnableRegenerateOnResponse",
        "GetSelectionText",
        "UpdateToMenuSelections",
        "SetDefaultCallback",
        "SetIsDefaultCallback",
        "SetUpdateCallback",
        "NotifyUpdate",
    ] {
        assert!(
            !runtime_bootstrap.contains(needle),
            "{needle} is Rust-owned and must not be redefined in runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} is Rust-owned and must not be redefined in shared bootstrap"
        );
    }
}
