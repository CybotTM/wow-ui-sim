#[test]
fn misc_global_frame_defaults_are_not_runtime_bootstrap_fallbacks() {
    let bootstrap = include_str!("../src/lua_api/env_init/runtime_surface_bootstrap.lua");
    let shared_bootstrap = include_str!("../src/lua_api/env_init/shared_bootstrap.lua");

    for needle in [
        "__wow_register_misc_global_frames",
        "__wow_make_named_frame",
        "__wow_seed_global_frame_path",
        "GameMenuFrame",
    ] {
        assert!(
            !bootstrap.contains(needle),
            "{needle} fallback must live in the explicit temporary misc global-frame workaround, not runtime bootstrap"
        );
        assert!(
            !shared_bootstrap.contains(needle),
            "{needle} fallback must live in the explicit temporary misc global-frame workaround, not shared bootstrap"
        );
    }
}
